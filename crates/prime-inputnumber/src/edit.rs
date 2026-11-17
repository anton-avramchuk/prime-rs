//! Pure field text editing operations (PrimeNG's `insert`, `deleteRange`, `onKeyDown`, `initCursor`), without the DOM.
//!
//! The text is parsed into a model (sign and digits with an optional decimal separator), edited and reassembled per the format;
//! the caret is carried over by the number of significant characters (digits and the separator) to its left.

use crate::format::{InputNumberMode, NumberFormat, Parsed};

/// Editing context: format, bounds and modes.
#[derive(Clone, Copy, Debug)]
pub struct EditCtx<'a> {
    /// Formatter.
    pub nf: &'a NumberFormat,
    /// Lower bound.
    pub min: Option<f64>,
    /// Upper bound.
    pub max: Option<f64>,
    /// Whether an empty value is allowed (otherwise emptiness is replaced by zero).
    pub allow_empty: bool,
    /// Formatting mode.
    pub mode: InputNumberMode,
}

/// Result of an edit.
#[derive(Clone, Debug, PartialEq)]
pub struct Edit {
    /// New field text.
    pub text: String,
    /// New caret position (in characters).
    pub caret: usize,
    /// Value parsed from the new text.
    pub value: Parsed,
}

/// Editing model: sign and digits with the `.` marker for the decimal separator.
#[derive(Clone, Debug)]
struct Model {
    neg: bool,
    /// Digits and at most one `.`.
    flat: Vec<char>,
}

const DOT: char = '.';

impl Model {
    fn from_text(nf: &NumberFormat, text: &str) -> Option<Self> {
        let parts = nf.decompose(text)?;
        let mut flat: Vec<char> = parts.int.chars().collect();
        if let Some(frac) = &parts.frac {
            flat.push(DOT);
            flat.extend(frac.chars());
        }
        Some(Self { neg: parts.neg, flat })
    }

    fn dot(&self) -> Option<usize> {
        self.flat.iter().position(|&c| c == DOT)
    }

    fn int_len(&self) -> usize {
        self.dot().unwrap_or(self.flat.len())
    }

    fn frac_len(&self) -> usize {
        self.dot().map_or(0, |d| self.flat.len() - d - 1)
    }

    fn remove_range(&mut self, start: usize, end: usize) {
        let end = end.min(self.flat.len());
        if start < end {
            self.flat.drain(start..end);
        }
    }
}

/// Number of significant characters (digits and the decimal separator) in `text[..caret]` within the numeric part.
fn flat_before(nf: &NumberFormat, text: &str, caret: usize) -> usize {
    let (start, _) = nf.core_bounds(text);
    text.chars().enumerate().filter(|(i, c)| *i >= start && *i < caret && (c.is_ascii_digit() || *c == nf.decimal())).count()
}

/// Caret position right after `flat` significant characters (at 0, the start of the numeric part).
fn caret_after(nf: &NumberFormat, text: &str, flat: usize) -> usize {
    let (start, end) = nf.core_bounds(text);
    if flat == 0 {
        return start;
    }
    let mut seen = 0;
    for (i, c) in text.chars().enumerate().skip(start) {
        if c.is_ascii_digit() || c == nf.decimal() {
            seen += 1;
            if seen == flat {
                return i + 1;
            }
        }
    }
    end
}

/// Index of the minus in the text, if present.
fn minus_index(nf: &NumberFormat, text: &str) -> Option<usize> {
    text.chars().position(|c| c == nf.minus() || c == '-')
}

/// Assembles text from the model and places the caret after `caret_flat` significant characters.
fn render(ctx: &EditCtx, mut model: Model, caret_flat: usize) -> Edit {
    let nf = ctx.nf;
    let mut int: Vec<char> = model.flat[..model.int_len()].to_vec();
    let mut frac: Option<Vec<char>> = model.dot().map(|d| model.flat[d + 1..].to_vec());
    // leading zeros: "05" -> "5", "00" -> "0", ".5" -> "0.5"
    let lead = int.iter().take_while(|&&c| c == '0').count();
    let mut caret_flat = caret_flat;
    if lead > 0 {
        let keep_zero = lead == int.len();
        let removed = if keep_zero { lead - 1 } else { lead };
        int.drain(..removed);
        caret_flat = caret_flat.saturating_sub(removed);
    } else if int.is_empty() && frac.is_some() {
        int.push('0');
        caret_flat += 1;
    }
    if nf.max_frac() == 0 {
        frac = None;
    }
    if let Some(f) = &mut frac {
        f.truncate(nf.max_frac());
    }
    model.flat = Vec::new();
    let (text, value) = if int.is_empty() && frac.is_none() {
        if ctx.allow_empty {
            let neg = model.neg;
            (if neg { "-".to_owned() } else { String::new() }, if neg { Parsed::Minus } else { Parsed::Empty })
        } else {
            (nf.format(0.0), Parsed::Number(0.0))
        }
    } else {
        let int_text: String = int.iter().collect();
        let frac_text: Option<String> = match frac {
            Some(f) => Some(f.iter().collect()),
            None if nf.min_frac() > 0 => Some("0".repeat(nf.min_frac())),
            None => None,
        };
        let text = nf.compose(model.neg, &int_text, frac_text.as_deref());
        let value = nf.parse(&text);
        (text, value)
    };
    let caret_flat =
        if !ctx.allow_empty && matches!(value, Parsed::Number(v) if v == 0.0) && text.chars().all(|c| !c.is_ascii_digit() || c == '0') {
            caret_flat.max(1)
        } else {
            caret_flat
        };
    let caret = caret_after(nf, &text, caret_flat);
    Edit { text, caret, value }
}

/// Whether a minus is allowed for the given lower bound.
fn allows_minus(ctx: &EditCtx) -> bool {
    ctx.min.is_none_or(|m| m < 0.0)
}

/// Enters one character in place of the selection `sel`; `None` means the character was rejected.
pub fn insert_char(ctx: &EditCtx, text: &str, sel: (usize, usize), c: char) -> Option<Edit> {
    let nf = ctx.nf;
    let mut model = Model::from_text(nf, text)?;
    let (fs, fe) = (flat_before(nf, text, sel.0), flat_before(nf, text, sel.1));
    let selection_has_minus = minus_index(nf, text).is_some_and(|i| i >= sel.0 && i < sel.1);
    if c == nf.minus() || c == '-' {
        if !allows_minus(ctx) || fs != 0 {
            return None;
        }
        model.remove_range(fs, fe);
        if selection_has_minus {
            model.neg = false;
        }
        model.neg = true;
        return Some(render(ctx, model, 0));
    }
    if c == nf.decimal() {
        let dot = model.dot();
        match dot {
            Some(d) if fs == d && fe == fs => return Some(render(ctx, model, d + 1)),
            Some(d) if fs <= d && d < fe => {}
            Some(_) => return None,
            None if nf.typing_decimal_allowed() => {}
            None => return None,
        }
        model.remove_range(fs, fe);
        model.flat.insert(fs.min(model.flat.len()), DOT);
        return Some(render(ctx, model, fs + 1));
    }
    if !c.is_ascii_digit() {
        return None;
    }
    let collapsed = fs == fe;
    model.remove_range(fs, fe);
    if let Some(d) = model.dot()
        && fs > d
    {
        let idx = fs - d - 1;
        if idx >= nf.max_frac() {
            return None;
        }
        if collapsed && fs < model.flat.len() {
            // in the fraction part a digit replaces the next one
            model.flat[fs] = c;
        } else {
            model.flat.insert(fs, c);
        }
    } else {
        model.flat.insert(fs.min(model.flat.len()), c);
    }
    Some(render(ctx, model, fs + 1))
}

/// Enters a string (several characters typed in a row): edits are applied character by character.
pub fn insert_text(ctx: &EditCtx, text: &str, sel: (usize, usize), data: &str) -> Option<Edit> {
    let mut current: Option<Edit> = None;
    let (mut text_now, mut sel_now) = (text.to_owned(), sel);
    for c in data.chars() {
        match insert_char(ctx, &text_now, sel_now, c) {
            Some(edit) => {
                sel_now = (edit.caret, edit.caret);
                text_now = edit.text.clone();
                current = Some(edit);
            }
            None => continue,
        }
    }
    current
}

/// Deletes the selected range.
pub fn delete_range(ctx: &EditCtx, text: &str, sel: (usize, usize)) -> Option<Edit> {
    let nf = ctx.nf;
    let mut model = Model::from_text(nf, text)?;
    let (fs, fe) = (flat_before(nf, text, sel.0), flat_before(nf, text, sel.1));
    let (core_start, core_end) = nf.core_bounds(text);
    if minus_index(nf, text).is_some_and(|i| i >= sel.0 && i < sel.1) || (sel.0 <= core_start && sel.1 >= core_end) {
        model.neg = false;
    }
    model.remove_range(fs, fe);
    Some(render(ctx, model, fs))
}

/// Deletes a fraction digit: with a zero or by shifting (in decimal mode when the digit count exceeds the minimum).
fn zero_or_remove_frac(ctx: &EditCtx, model: &mut Model, flat_idx: usize) -> bool {
    let shift = ctx.mode == InputNumberMode::Decimal && ctx.nf.min_frac() < model.frac_len();
    if shift {
        model.flat.remove(flat_idx);
    } else {
        model.flat[flat_idx] = '0';
    }
    shift
}

/// Deletes an integer digit; a lone digit is replaced by zero, and a result not greater than zero becomes empty.
fn remove_int_digit(model: &mut Model, flat_idx: usize) {
    if model.int_len() == 1 {
        model.flat[flat_idx] = '0';
        let positive = model.flat.iter().any(|&c| c.is_ascii_digit() && c != '0');
        if !positive {
            model.flat.clear();
            model.neg = false;
        }
    } else {
        model.flat.remove(flat_idx);
    }
}

/// Backspace with a collapsed selection; `None` means the action does not change the text.
pub fn delete_back(ctx: &EditCtx, text: &str, sel: (usize, usize)) -> Option<Edit> {
    if sel.0 != sel.1 {
        return delete_range(ctx, text, sel);
    }
    let nf = ctx.nf;
    let chars: Vec<char> = text.chars().collect();
    let caret = sel.0;
    if caret == 0 || caret > chars.len() {
        return None;
    }
    let prev = chars[caret - 1];
    let mut model = Model::from_text(nf, text)?;
    let f = flat_before(nf, text, caret);
    if prev.is_ascii_digit() && f > 0 {
        let idx = f - 1;
        match model.dot() {
            Some(d) if idx > d => {
                zero_or_remove_frac(ctx, &mut model, idx);
                return Some(render(ctx, model, idx));
            }
            _ => remove_int_digit(&mut model, idx),
        }
        return Some(render(ctx, model, idx));
    }
    if prev == nf.group() && f > 0 {
        model.flat.remove(f - 1);
        return Some(render(ctx, model, f - 1));
    }
    if prev == nf.decimal() && f > 0 {
        if model.frac_len() > 0 {
            return Some(render(ctx, model, f - 1));
        }
        model.flat.remove(f - 1);
        return Some(render(ctx, model, f - 1));
    }
    if prev == nf.minus() || prev == '-' {
        model.neg = false;
        return Some(render(ctx, model, f));
    }
    None
}

/// Delete with a collapsed selection; `None` means the action does not change the text.
pub fn delete_forward(ctx: &EditCtx, text: &str, sel: (usize, usize)) -> Option<Edit> {
    if sel.0 != sel.1 {
        return delete_range(ctx, text, sel);
    }
    let nf = ctx.nf;
    let chars: Vec<char> = text.chars().collect();
    let caret = sel.0;
    let next = *chars.get(caret)?;
    let mut model = Model::from_text(nf, text)?;
    let f = flat_before(nf, text, caret);
    if next.is_ascii_digit() {
        match model.dot() {
            Some(d) if f > d => {
                let shifted = zero_or_remove_frac(ctx, &mut model, f);
                return Some(render(ctx, model, if shifted { f } else { f + 1 }));
            }
            _ => remove_int_digit(&mut model, f),
        }
        return Some(render(ctx, model, f));
    }
    if next == nf.group() {
        if f < model.flat.len() {
            model.flat.remove(f);
        }
        return Some(render(ctx, model, f));
    }
    if next == nf.decimal() {
        if model.frac_len() > 0 {
            return Some(render(ctx, model, f + 1));
        }
        model.flat.remove(f);
        return Some(render(ctx, model, f));
    }
    if next == nf.minus() || next == '-' {
        model.neg = false;
        return Some(render(ctx, model, f));
    }
    None
}

/// Paste from the clipboard: the pasted text is parsed as a number (a fraction is allowed regardless of `max_fraction_digits`).
pub fn paste(ctx: &EditCtx, text: &str, sel: (usize, usize), pasted: &str) -> Option<Edit> {
    let nf = ctx.nf;
    let parts = nf.decompose(pasted.trim())?;
    if parts.int.is_empty() && parts.frac.as_ref().is_none_or(String::is_empty) {
        return None;
    }
    let mut model = Model::from_text(nf, text)?;
    let (fs, fe) = (flat_before(nf, text, sel.0), flat_before(nf, text, sel.1));
    let (core_start, core_end) = nf.core_bounds(text);
    let whole = text.is_empty() || (sel.0 <= core_start && sel.1 >= core_end);
    if whole {
        model.flat.clear();
        model.neg = false;
    } else {
        model.remove_range(fs, fe);
    }
    let mut at = if whole { 0 } else { fs };
    if parts.neg && allows_minus(ctx) && at == 0 {
        model.neg = true;
    }
    let mut incoming: Vec<char> = parts.int.chars().collect();
    if let Some(frac) = &parts.frac {
        incoming.push(DOT);
        incoming.extend(frac.chars());
    }
    for c in incoming {
        if c == DOT {
            if model.dot().is_none() && nf.max_frac() > 0 {
                model.flat.insert(at.min(model.flat.len()), DOT);
                at += 1;
            }
            continue;
        }
        if model.dot().is_some_and(|d| at > d) && model.frac_len() >= nf.max_frac() {
            continue;
        }
        model.flat.insert(at.min(model.flat.len()), c);
        at += 1;
    }
    Some(render(ctx, model, at))
}

/// Clamps the value to the bounds.
pub fn validate(value: f64, min: Option<f64>, max: Option<f64>) -> f64 {
    let v = min.map_or(value, |m| value.max(m));
    max.map_or(v, |m| v.min(m))
}

/// Commit of the text (`Enter`, `Tab`, focus loss): the number is clamped to the bounds and formatted again.
pub fn commit(ctx: &EditCtx, text: &str) -> (Option<f64>, String) {
    let nf = ctx.nf;
    match nf.parse(text) {
        Parsed::Number(v) => {
            let v = validate(v, ctx.min, ctx.max);
            (Some(v), nf.format(v))
        }
        _ if !ctx.allow_empty => {
            let v = validate(0.0, ctx.min, ctx.max);
            (Some(v), nf.format(v))
        }
        _ => (None, String::new()),
    }
}

/// Step up or down from the value in the field (empty is 0); the result is clamped to the bounds, the text is formatted.
pub fn spin(ctx: &EditCtx, text: &str, dir: f64, step: f64) -> (f64, String) {
    let current = ctx.nf.parse(text).value().unwrap_or(0.0);
    // remove binary arithmetic noise (0.1 + 0.2)
    let raw = ((current + dir * step) * 1e12).round() / 1e12;
    let v = validate(raw, ctx.min, ctx.max);
    (v, ctx.nf.format(v))
}

/// Brings the caret inside the numeric part (`initCursor`): it does not enter the prefix or suffix.
pub fn clamp_caret(nf: &NumberFormat, text: &str, caret: usize) -> usize {
    let (start, end) = nf.core_bounds(text);
    caret.clamp(start, end)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::format::{FormatOptions, default_format};

    fn ctx_with<'a>(nf: &'a NumberFormat, mode: InputNumberMode) -> EditCtx<'a> {
        EditCtx { nf, min: None, max: None, allow_empty: true, mode }
    }

    fn nf_of(options: FormatOptions) -> NumberFormat {
        NumberFormat::new(&options)
    }

    fn usd() -> NumberFormat {
        nf_of(FormatOptions { mode: InputNumberMode::Currency, currency: "USD".into(), locale: "en-US".into(), ..FormatOptions::default() })
    }

    fn frac2() -> NumberFormat {
        nf_of(FormatOptions { max_fraction_digits: Some(2), locale: "en-US".into(), ..FormatOptions::default() })
    }

    /// Applies a sequence of characters at the end of the field; returns the text and the caret.
    fn typed(ctx: &EditCtx, chars: &str) -> (String, usize) {
        let (mut text, mut caret) = (String::new(), 0);
        for c in chars.chars() {
            if let Some(edit) = insert_char(ctx, &text, (caret, caret), c) {
                text = edit.text;
                caret = edit.caret;
            }
        }
        (text, caret)
    }

    #[test]
    fn typing_digits_groups_and_keeps_caret_at_the_end() {
        let nf = default_format();
        let ctx = ctx_with(&nf, InputNumberMode::Decimal);
        assert_eq!(typed(&ctx, "1234"), ("1,234".to_owned(), 5));
        assert_eq!(typed(&ctx, "1234567"), ("1,234,567".to_owned(), 9));
        assert_eq!(typed(&ctx, "05"), ("5".to_owned(), 1));
        assert_eq!(typed(&ctx, "0"), ("0".to_owned(), 1));
        assert_eq!(typed(&ctx, "00"), ("0".to_owned(), 1));
        assert_eq!(typed(&ctx, "1a2b"), ("12".to_owned(), 2));
    }

    #[test]
    fn insert_in_the_middle_keeps_the_caret_after_the_inserted_digit() {
        let nf = default_format();
        let ctx = ctx_with(&nf, InputNumberMode::Decimal);
        // "1,234": inserting "9" after "1"
        let e = insert_char(&ctx, "1,234", (1, 1), '9').unwrap();
        assert_eq!((e.text.as_str(), e.caret), ("19,234", 2));
        // insertion at the start
        let e = insert_char(&ctx, "234", (0, 0), '1').unwrap();
        assert_eq!((e.text.as_str(), e.caret), ("1,234", 1));
        // replacing a selection
        let e = insert_char(&ctx, "1,234", (0, 5), '7').unwrap();
        assert_eq!((e.text.as_str(), e.caret), ("7", 1));
    }

    #[test]
    fn decimal_typing_needs_an_explicit_max_fraction_like_primeng() {
        let plain = default_format();
        let ctx = ctx_with(&plain, InputNumberMode::Decimal);
        assert_eq!(typed(&ctx, "1.5").0, "15");
        let nf = frac2();
        let ctx = ctx_with(&nf, InputNumberMode::Decimal);
        assert_eq!(typed(&ctx, "1."), ("1.".to_owned(), 2));
        assert_eq!(typed(&ctx, "1.5"), ("1.5".to_owned(), 3));
        assert_eq!(typed(&ctx, "1.25"), ("1.25".to_owned(), 4));
        // the third fraction digit is not entered
        assert_eq!(typed(&ctx, "1.259"), ("1.25".to_owned(), 4));
        // the second separator is ignored
        assert_eq!(typed(&ctx, "1.2.").0, "1.2");
        // "." in an empty field gives "0."
        assert_eq!(typed(&ctx, "."), ("0.".to_owned(), 2));
        assert_eq!(typed(&ctx, ".5"), ("0.5".to_owned(), 3));
    }

    #[test]
    fn fraction_digits_overtype_the_next_digit() {
        let nf = frac2();
        let ctx = ctx_with(&nf, InputNumberMode::Decimal);
        let e = insert_char(&ctx, "1.50", (2, 2), '7').unwrap();
        assert_eq!((e.text.as_str(), e.caret), ("1.70", 3));
        let e = insert_char(&ctx, "1.50", (3, 3), '7').unwrap();
        assert_eq!((e.text.as_str(), e.caret), ("1.57", 4));
        assert!(insert_char(&ctx, "1.57", (4, 4), '1').is_none(), "beyond the fraction limit");
    }

    #[test]
    fn decimal_char_moves_over_an_existing_separator() {
        let nf = nf_of(FormatOptions { min_fraction_digits: Some(2), locale: "en-US".into(), ..FormatOptions::default() });
        let ctx = ctx_with(&nf, InputNumberMode::Decimal);
        let e = insert_char(&ctx, "12.00", (2, 2), '.').unwrap();
        assert_eq!((e.text.as_str(), e.caret), ("12.00", 3));
        // a selection covering the separator is replaced by it
        let e = insert_char(&ctx, "1,234.00", (1, 6), '.').unwrap();
        assert_eq!(e.value, Parsed::Number(1.0));
    }

    #[test]
    fn min_fraction_digits_pad_while_typing() {
        let nf = nf_of(FormatOptions { min_fraction_digits: Some(2), locale: "en-US".into(), ..FormatOptions::default() });
        let ctx = ctx_with(&nf, InputNumberMode::Decimal);
        assert_eq!(typed(&ctx, "1"), ("1.00".to_owned(), 1));
        assert_eq!(typed(&ctx, "12"), ("12.00".to_owned(), 2));
        assert_eq!(typed(&ctx, "1234"), ("1,234.00".to_owned(), 5));
    }

    #[test]
    fn currency_typing_and_caret_around_the_prefix() {
        let nf = usd();
        let ctx = ctx_with(&nf, InputNumberMode::Currency);
        assert_eq!(typed(&ctx, "5"), ("$5.00".to_owned(), 2));
        assert_eq!(typed(&ctx, "1234"), ("$1,234.00".to_owned(), 6));
        // caret before the number (after "$"): insertion at the start
        let e = insert_char(&ctx, "$5.00", (1, 1), '1').unwrap();
        assert_eq!((e.text.as_str(), e.caret), ("$15.00", 2));
    }

    #[test]
    fn minus_rules() {
        let nf = default_format();
        let ctx = ctx_with(&nf, InputNumberMode::Decimal);
        assert_eq!(typed(&ctx, "-5"), ("-5".to_owned(), 2));
        assert_eq!(typed(&ctx, "-").0, "-");
        // a minus not at the start of the number is ignored
        assert!(insert_char(&ctx, "12", (2, 2), '-').is_none());
        // a repeated minus does not change the number
        let e = insert_char(&ctx, "-12", (0, 0), '-').unwrap();
        assert_eq!(e.text, "-12");
        // lower bound not below zero: minus is forbidden
        let non_negative = EditCtx { min: Some(0.0), ..ctx };
        assert!(insert_char(&non_negative, "", (0, 0), '-').is_none());
        // negative currency
        let nf = usd();
        let ctx = ctx_with(&nf, InputNumberMode::Currency);
        assert_eq!(typed(&ctx, "-5").0, "-$5.00");
        // ru-RU: the ASCII minus is accepted and shown as the locale's sign
        let ru = nf_of(FormatOptions { locale: "ru-RU".into(), ..FormatOptions::default() });
        let ctx = ctx_with(&ru, InputNumberMode::Decimal);
        assert_eq!(typed(&ctx, "-5").0, "\u{2212}5");
    }

    #[test]
    fn backspace_over_digits_groups_and_decimal() {
        let nf = default_format();
        let ctx = ctx_with(&nf, InputNumberMode::Decimal);
        // ordinary digit
        let e = delete_back(&ctx, "1,234", (5, 5)).unwrap();
        assert_eq!((e.text.as_str(), e.caret), ("123", 3));
        // right after the comma the digit before it is deleted
        let e = delete_back(&ctx, "1,234", (2, 2)).unwrap();
        assert_eq!((e.text.as_str(), e.caret), ("234", 0));
        // a lone digit: empty
        let e = delete_back(&ctx, "5", (1, 1)).unwrap();
        assert_eq!((e.text.as_str(), e.value), ("", Parsed::Empty));
        // no digits to the left
        assert!(delete_back(&ctx, "5", (0, 0)).is_none());
    }

    #[test]
    fn backspace_in_fraction_zeroes_or_shifts_by_mode() {
        let nf = frac2();
        let ctx = ctx_with(&nf, InputNumberMode::Decimal);
        // decimal mode, digit count above the minimum (0): the digit is removed
        let e = delete_back(&ctx, "1.25", (4, 4)).unwrap();
        assert_eq!((e.text.as_str(), e.caret), ("1.2", 3));
        // with a non-empty fraction the separator only moves the caret
        let e = delete_back(&ctx, "1.25", (2, 2)).unwrap();
        assert_eq!((e.text.as_str(), e.caret), ("1.25", 1));
        // a separator without a fraction is deleted
        let e = delete_back(&ctx, "1.", (2, 2)).unwrap();
        assert_eq!((e.text.as_str(), e.caret), ("1", 1));
        // currency: the fraction digit is replaced by zero
        let cur = usd();
        let ctx = ctx_with(&cur, InputNumberMode::Currency);
        let e = delete_back(&ctx, "$12.34", (6, 6)).unwrap();
        assert_eq!((e.text.as_str(), e.caret), ("$12.30", 5));
        // the prefix is not deleted
        assert!(delete_back(&ctx, "$12.34", (1, 1)).is_none());
    }

    #[test]
    fn deleting_the_only_integer_digit_keeps_a_positive_fraction() {
        let nf = frac2();
        let ctx = ctx_with(&nf, InputNumberMode::Decimal);
        let e = delete_back(&ctx, "5.5", (1, 1)).unwrap();
        assert_eq!((e.text.as_str(), e.value), ("0.5", Parsed::Number(0.5)));
        let e = delete_back(&ctx, "-5", (2, 2)).unwrap();
        assert_eq!((e.text.as_str(), e.value), ("", Parsed::Empty), "zero and below is empty, so is a minus");
    }

    #[test]
    fn delete_forward_cases() {
        let nf = default_format();
        let ctx = ctx_with(&nf, InputNumberMode::Decimal);
        let e = delete_forward(&ctx, "1,234", (0, 0)).unwrap();
        assert_eq!((e.text.as_str(), e.caret), ("234", 0));
        // before the comma the digit after it is deleted
        let e = delete_forward(&ctx, "1,234", (1, 1)).unwrap();
        assert_eq!((e.text.as_str(), e.caret), ("134", 1));
        assert!(delete_forward(&ctx, "1,234", (5, 5)).is_none());
        let nf = frac2();
        let ctx = ctx_with(&nf, InputNumberMode::Decimal);
        let e = delete_forward(&ctx, "1.25", (2, 2)).unwrap();
        assert_eq!(e.text, "1.5");
        let e = delete_forward(&ctx, "1.25", (1, 1)).unwrap();
        assert_eq!((e.text.as_str(), e.caret), ("1.25", 2));
    }

    #[test]
    fn range_deletion() {
        let nf = default_format();
        let ctx = ctx_with(&nf, InputNumberMode::Decimal);
        let e = delete_range(&ctx, "1,234,567", (1, 5)).unwrap();
        assert_eq!((e.text.as_str(), e.caret), ("1,567", 1));
        let e = delete_back(&ctx, "1,234", (0, 5)).unwrap();
        assert_eq!((e.text.as_str(), e.value), ("", Parsed::Empty));
        // currency: everything selected: empty without the prefix
        let cur = usd();
        let ctx = ctx_with(&cur, InputNumberMode::Currency);
        let e = delete_range(&ctx, "$12.00", (0, 6)).unwrap();
        assert_eq!(e.text, "");
    }

    #[test]
    fn allow_empty_false_turns_emptiness_into_zero() {
        let nf = default_format();
        let ctx = EditCtx { allow_empty: false, ..ctx_with(&nf, InputNumberMode::Decimal) };
        let e = delete_back(&ctx, "5", (1, 1)).unwrap();
        assert_eq!((e.text.as_str(), e.value), ("0", Parsed::Number(0.0)));
        assert_eq!(commit(&ctx, ""), (Some(0.0), "0".to_owned()));
        assert_eq!(commit(&ctx, "-"), (Some(0.0), "0".to_owned()));
    }

    #[test]
    fn paste_cases() {
        let nf = default_format();
        let ctx = ctx_with(&nf, InputNumberMode::Decimal);
        let e = paste(&ctx, "", (0, 0), "1,234.5").unwrap();
        assert_eq!((e.text.as_str(), e.value), ("1,234.5", Parsed::Number(1234.5)));
        let e = paste(&ctx, "12", (1, 1), "99").unwrap();
        assert_eq!((e.text.as_str(), e.caret), ("1,992", 4));
        let e = paste(&ctx, "1,234", (0, 5), "-7").unwrap();
        assert_eq!(e.value, Parsed::Number(-7.0));
        assert!(paste(&ctx, "12", (0, 0), "abc").is_none());
        assert!(paste(&ctx, "12", (0, 0), "").is_none());
        let non_negative = EditCtx { min: Some(0.0), ..ctx };
        assert_eq!(paste(&non_negative, "", (0, 0), "-7").unwrap().value, Parsed::Number(7.0));
        // the fraction is truncated to the format maximum
        let two = frac2();
        let ctx = ctx_with(&two, InputNumberMode::Decimal);
        assert_eq!(paste(&ctx, "", (0, 0), "1.2345").unwrap().text, "1.23");
    }

    #[test]
    fn commit_validates_formats_and_empties() {
        let nf = nf_of(FormatOptions { min_fraction_digits: Some(2), locale: "en-US".into(), ..FormatOptions::default() });
        let ctx = EditCtx { min: Some(0.0), max: Some(100.0), ..ctx_with(&nf, InputNumberMode::Decimal) };
        assert_eq!(commit(&ctx, "250"), (Some(100.0), "100.00".to_owned()));
        assert_eq!(commit(&ctx, "-5"), (Some(0.0), "0.00".to_owned()));
        assert_eq!(commit(&ctx, "12.5"), (Some(12.5), "12.50".to_owned()));
        assert_eq!(commit(&ctx, ""), (None, String::new()));
        assert_eq!(commit(&ctx, "-"), (None, String::new()));
    }

    #[test]
    fn spin_steps_clamps_and_rounds_noise() {
        let nf = default_format();
        let ctx = ctx_with(&nf, InputNumberMode::Decimal);
        assert_eq!(spin(&ctx, "5", 1.0, 1.0), (6.0, "6".to_owned()));
        assert_eq!(spin(&ctx, "", 1.0, 1.0), (1.0, "1".to_owned()));
        assert_eq!(spin(&ctx, "-", -1.0, 1.0), (-1.0, "-1".to_owned()));
        assert_eq!(spin(&ctx, "0.1", 1.0, 0.2).0, 0.3);
        let bounded = EditCtx { max: Some(1.0), ..ctx };
        assert_eq!(spin(&bounded, "0.9", 1.0, 0.25).0, 1.0);
        let lower = EditCtx { min: Some(0.0), ..ctx };
        assert_eq!(spin(&lower, "0", -1.0, 1.0).0, 0.0);
    }

    #[test]
    fn caret_is_clamped_into_the_number() {
        let nf = usd();
        assert_eq!(clamp_caret(&nf, "$1,234.50", 0), 1);
        assert_eq!(clamp_caret(&nf, "$1,234.50", 5), 5);
        let d = nf_of(FormatOptions { suffix: " mi".into(), locale: "en-US".into(), ..FormatOptions::default() });
        assert_eq!(clamp_caret(&d, "12 mi", 5), 2);
        assert_eq!(clamp_caret(&d, "", 0), 0);
    }

    #[test]
    fn typing_with_prefix_and_suffix() {
        let nf = nf_of(FormatOptions {
            prefix: "Expires in ".into(),
            suffix: " days".into(),
            locale: "en-US".into(),
            ..FormatOptions::default()
        });
        let ctx = ctx_with(&nf, InputNumberMode::Decimal);
        assert_eq!(typed(&ctx, "42"), ("Expires in 42 days".to_owned(), 13));
        // Backspace does not touch the suffix
        let e = delete_back(&ctx, "Expires in 42 days", (13, 13)).unwrap();
        assert_eq!(e.text, "Expires in 4 days");
        assert!(delete_back(&ctx, "Expires in 42 days", (11, 11)).is_none());
    }

    #[test]
    fn german_typing_uses_the_comma() {
        let nf = nf_of(FormatOptions { max_fraction_digits: Some(2), locale: "de-DE".into(), ..FormatOptions::default() });
        let ctx = ctx_with(&nf, InputNumberMode::Decimal);
        assert_eq!(typed(&ctx, "1234,5").0, "1.234,5");
        // the dot in the German locale is not a decimal separator
        assert_eq!(typed(&ctx, "1.5").0, "15");
    }
}
