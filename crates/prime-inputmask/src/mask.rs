//! Движок маски InputMask — чистый порт логики `inputmask.ts` PrimeNG 21.1.10 (`initMask`, `checkVal`, `shiftL`/`shiftR`,
//! `onKeyPress`, удаление, фокус, потеря фокуса, `getUnmaskedValue`) без DOM: методы принимают и возвращают текст и позиции каретки.
//!
//! Шаблоны маски: `9` — цифра, `a` — буква (по предикату), `*` — буква или цифра, `?` — начало необязательной части, остальные
//! символы — постоянные.

use std::sync::Arc;

/// Предикат «буква» для шаблона `a` и `*`.
pub type CharTest = Arc<dyn Fn(char) -> bool + Send + Sync>;

/// Предикат по умолчанию: ASCII-буквы (`[A-Za-z]`).
pub fn ascii_letters() -> CharTest {
    Arc::new(|c: char| c.is_ascii_alphabetic())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Kind {
    Digit,
    Alpha,
    AlphaNum,
}

/// Результат ввода символа ([`Mask::key_press`]).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Typed {
    /// Новый текст поля, если буфер записывался.
    pub text: Option<String>,
    /// Новая позиция каретки, если она менялась.
    pub caret: Option<usize>,
    /// Символ принят.
    pub accepted: bool,
    /// После ввода обязательная часть заполнена.
    pub completed: bool,
}

/// Результат удаления ([`Mask::delete`]).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Deleted {
    /// Новый текст поля, если буфер записывался.
    pub text: Option<String>,
    /// Новая позиция каретки, если она менялась.
    pub caret: Option<usize>,
}

/// Результат фокуса ([`Mask::focus`]): текст и выделение.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Focused {
    /// Текст поля после проверки значения (без записи буфера).
    pub text: String,
    /// Текст поля через 10 мс: записанный буфер.
    pub buffer_text: String,
    /// Начало выделения.
    pub begin: usize,
    /// Конец выделения.
    pub end: usize,
}

/// Состояние маски: буфер и таблицы позиций.
#[derive(Clone)]
pub struct Mask {
    tests: Vec<Option<Kind>>,
    len: usize,
    partial_position: usize,
    first_non_mask: Option<usize>,
    last_required: Option<usize>,
    buffer: Vec<char>,
    default_buffer: String,
    slot: Vec<char>,
    auto_clear: bool,
    keep_buffer: bool,
    alpha: CharTest,
}

impl Mask {
    /// Строит маску (`initMask`). Пустая маска даёт движок без позиций ввода.
    pub fn new(mask: &str, slot_char: &str, auto_clear: bool, keep_buffer: bool, alpha: CharTest) -> Self {
        let tokens: Vec<char> = mask.chars().collect();
        let slot: Vec<char> = if slot_char.is_empty() { vec!['_'] } else { slot_char.chars().collect() };
        let mut m = Self {
            tests: Vec::new(),
            len: tokens.len(),
            partial_position: tokens.len(),
            first_non_mask: None,
            last_required: None,
            buffer: Vec::new(),
            default_buffer: String::new(),
            slot,
            auto_clear,
            keep_buffer,
            alpha,
        };
        let kind_of = |c: char| match c {
            '9' => Some(Kind::Digit),
            'a' => Some(Kind::Alpha),
            '*' => Some(Kind::AlphaNum),
            _ => None,
        };
        for (i, &c) in tokens.iter().enumerate() {
            if c == '?' {
                m.len -= 1;
                m.partial_position = i;
            } else if let Some(kind) = kind_of(c) {
                m.tests.push(Some(kind));
                if m.first_non_mask.is_none() {
                    m.first_non_mask = Some(m.tests.len() - 1);
                }
                if i < m.partial_position {
                    m.last_required = Some(m.tests.len() - 1);
                }
            } else {
                m.tests.push(None);
            }
        }
        for (i, &c) in tokens.iter().enumerate() {
            if c != '?' {
                m.buffer.push(if kind_of(c).is_some() { m.placeholder(i) } else { c });
            }
        }
        m.default_buffer = m.buffer.iter().collect();
        m
    }

    /// Буфер как строка (`buffer.join('')`).
    pub fn buffer_text(&self) -> String {
        self.buffer.iter().collect()
    }

    /// Пустой буфер маски (постоянные символы и заполнители).
    pub fn default_buffer(&self) -> &str {
        &self.default_buffer
    }

    /// Число позиций (без `?`).
    pub fn len(&self) -> usize {
        self.len
    }

    /// Нет позиций.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn placeholder(&self, i: usize) -> char {
        self.slot.get(i).or_else(|| self.slot.first()).copied().unwrap_or('_')
    }

    fn matches(&self, kind: Kind, c: char) -> bool {
        match kind {
            Kind::Digit => c.is_ascii_digit(),
            Kind::Alpha => (self.alpha)(c),
            Kind::AlphaNum => (self.alpha)(c) || c.is_ascii_digit(),
        }
    }

    fn test(&self, i: usize, c: char) -> bool {
        self.tests.get(i).copied().flatten().is_some_and(|k| self.matches(k, c))
    }

    fn has_test(&self, i: isize) -> bool {
        i >= 0 && self.tests.get(i as usize).copied().flatten().is_some()
    }

    /// Первая позиция ввода после `pos`; `len`, если её нет.
    pub fn seek_next(&self, pos: isize) -> usize {
        let mut p = pos;
        loop {
            p += 1;
            if p >= self.len as isize || self.has_test(p) {
                return p.max(0) as usize;
            }
        }
    }

    /// Последняя позиция ввода перед `pos`; `-1`, если её нет.
    pub fn seek_prev(&self, pos: isize) -> isize {
        let mut p = pos;
        loop {
            p -= 1;
            if p < 0 || self.has_test(p) {
                return p;
            }
        }
    }

    /// Все ли обязательные позиции заполнены (`isCompleted`).
    pub fn is_completed(&self) -> bool {
        let (Some(first), Some(last)) = (self.first_non_mask, self.last_required) else { return true };
        (first..=last).all(|i| !(self.tests.get(i).copied().flatten().is_some() && self.buffer.get(i) == Some(&self.placeholder(i))))
    }

    /// Введённые символы без постоянных и заполнителей (`getUnmaskedValue`).
    pub fn unmasked(&self) -> String {
        self.buffer
            .iter()
            .enumerate()
            .filter(|(i, c)| self.tests.get(*i).copied().flatten().is_some() && **c != self.placeholder(*i))
            .map(|(_, c)| *c)
            .collect()
    }

    fn clear_buffer(&mut self, start: isize, end: usize) {
        if self.keep_buffer {
            return;
        }
        let mut i = start;
        while i < end as isize && i < self.len as isize {
            if i >= 0 && self.tests.get(i as usize).copied().flatten().is_some() {
                self.buffer[i as usize] = self.placeholder(i as usize);
            }
            i += 1;
        }
    }

    fn shift_l(&mut self, begin: isize, end: isize) -> Option<usize> {
        if begin < 0 {
            return None;
        }
        let mut j = self.seek_next(end);
        let mut i = begin as usize;
        while i < self.len {
            if self.tests.get(i).copied().flatten().is_some() {
                if j < self.len && self.test(i, self.buffer[j]) {
                    self.buffer[i] = self.buffer[j];
                    self.buffer[j] = self.placeholder(j);
                } else {
                    break;
                }
                j = self.seek_next(j as isize);
            }
            i += 1;
        }
        Some(self.first_non_mask.unwrap_or(0).max(begin as usize))
    }

    fn shift_r(&mut self, pos: usize) {
        let mut c = self.placeholder(pos);
        let mut i = pos;
        while i < self.len {
            if self.tests.get(i).copied().flatten().is_some() {
                let j = self.seek_next(i as isize);
                let t = self.buffer[i];
                self.buffer[i] = c;
                if j < self.len && self.test(j, t) {
                    c = t;
                } else {
                    break;
                }
            }
            i += 1;
        }
    }

    /// `checkVal`: раскладывает символы `text` по позициям маски. Возвращает новый текст поля и позицию каретки.
    pub fn check_val(&mut self, text: &str, allow: bool) -> (String, usize) {
        let chars: Vec<char> = text.chars().collect();
        let mut last_match: isize = -1;
        let mut pos = 0usize;
        let mut i = 0usize;
        while i < self.len {
            if self.tests.get(i).copied().flatten().is_some() {
                self.buffer[i] = self.placeholder(i);
                let mut exhausted = true;
                while pos < chars.len() {
                    let c = chars[pos];
                    pos += 1;
                    if self.test(i, c) {
                        if !self.keep_buffer {
                            self.buffer[i] = c;
                        }
                        last_match = i as isize;
                        exhausted = false;
                        break;
                    }
                }
                if exhausted {
                    self.clear_buffer(i as isize + 1, self.len);
                    break;
                }
            } else {
                if chars.get(pos) == Some(&self.buffer[i]) {
                    pos += 1;
                }
                if i < self.partial_position {
                    last_match = i as isize;
                }
            }
            i += 1;
        }
        let new_text = if allow {
            self.buffer_text()
        } else if last_match + 1 < self.partial_position as isize {
            if self.auto_clear || self.buffer_text() == self.default_buffer {
                self.clear_buffer(0, self.len);
                String::new()
            } else {
                self.buffer_text()
            }
        } else {
            self.buffer_text().chars().take((last_match + 1) as usize).collect()
        };
        (new_text, i)
    }

    /// Ввод символа в позиции каретки `[begin, end)` (`onKeyPress`).
    pub fn key_press(&mut self, ch: char, begin: usize, end: usize) -> Typed {
        let mut out = Typed { text: None, caret: None, accepted: false, completed: false };
        if end > begin {
            self.clear_buffer(begin as isize, end);
            if let Some(caret) = self.shift_l(begin as isize, end as isize - 1) {
                out.text = Some(self.buffer_text());
                out.caret = Some(caret);
            }
        }
        let p = self.seek_next(begin as isize - 1);
        if p < self.len && self.test(p, ch) {
            self.shift_r(p);
            self.buffer[p] = ch;
            out.text = Some(self.buffer_text());
            out.caret = Some(self.seek_next(p as isize));
            out.accepted = true;
            if self.last_required.is_some_and(|last| begin <= last) {
                out.completed = self.is_completed();
            }
        }
        out
    }

    /// `Backspace` (`forward = false`) или `Delete` (`forward = true`) в позиции каретки `[begin, end)`.
    pub fn delete(&mut self, begin: usize, end: usize, forward: bool) -> Deleted {
        let (mut b, mut e) = (begin as isize, end as isize);
        if e - b == 0 {
            if forward {
                e = self.seek_next(b - 1) as isize;
                b = e;
                e = self.seek_next(e) as isize;
            } else {
                b = self.seek_prev(b);
            }
        }
        self.clear_buffer(b, e.max(0) as usize);
        let shift_end = if self.keep_buffer { e - 2 } else { e - 1 };
        match self.shift_l(b, shift_end) {
            Some(caret) => Deleted { text: Some(self.buffer_text()), caret: Some(caret) },
            None => Deleted { text: None, caret: None },
        }
    }

    /// Фокус: проверка значения, затем (через 10 мс) запись буфера и выделение — вся маска, если она пуста, иначе каретка.
    pub fn focus(&mut self, text: &str) -> Focused {
        let (checked, pos) = self.check_val(text, false);
        let full = self.len + usize::from(false);
        let (begin, end) = if pos == full { (0, pos) } else { (pos, pos) };
        Focused { text: checked, buffer_text: self.buffer_text(), begin, end }
    }

    /// Потеря фокуса: итоговый текст поля.
    pub fn blur(&mut self, text: &str) -> String {
        if self.keep_buffer { text.to_owned() } else { self.check_val(text, false).0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mask(m: &str) -> Mask {
        Mask::new(m, "_", true, false, ascii_letters())
    }

    /// Печатает строку подряд с позиции после фокуса; возвращает итоговый текст и каретку.
    fn type_all(m: &mut Mask, s: &str) -> (String, usize) {
        let mut text = m.focus("").buffer_text;
        let mut caret = m.focus("").begin;
        for ch in s.chars() {
            let t = m.key_press(ch, caret, caret);
            if let Some(x) = t.text {
                text = x;
            }
            if let Some(c) = t.caret {
                caret = c;
            }
        }
        (text, caret)
    }

    #[test]
    fn buffers_and_positions() {
        let m = mask("(999) 999-9999");
        assert_eq!(m.default_buffer(), "(___) ___-____");
        assert_eq!(m.len(), 14);
        let m = mask("99-aaa?9");
        assert_eq!(m.default_buffer(), "__-___9".replace('9', "_"), "необязательный `?` не входит в буфер");
        assert_eq!(m.len(), 7);
        let m = Mask::new("99-99", "#", true, false, ascii_letters());
        assert_eq!(m.default_buffer(), "##-##");
        let m = Mask::new("9999", "ab", true, false, ascii_letters());
        assert_eq!(m.default_buffer(), "abaa", "заполнитель — символ по индексу маски, иначе первый");
        assert!(mask("").is_empty());
    }

    #[test]
    fn typing_fills_and_skips_constants() {
        let mut m = mask("99-99");
        let (text, caret) = type_all(&mut m, "1234");
        assert_eq!((text.as_str(), caret), ("12-34", 5));
        assert!(m.is_completed());
        assert_eq!(m.unmasked(), "1234");
        let mut m = mask("(999) 999-9999");
        assert_eq!(type_all(&mut m, "5551234567").0, "(555) 123-4567");
    }

    #[test]
    fn invalid_characters_are_ignored() {
        let mut m = mask("99-99");
        let t = m.key_press('x', 0, 0);
        assert_eq!(t, Typed { text: None, caret: None, accepted: false, completed: false });
        assert_eq!(m.buffer_text(), "__-__");
        let mut m = mask("aa-99");
        assert!(m.key_press('7', 0, 0).text.is_none() && m.key_press('b', 0, 0).accepted);
        let mut m = mask("**");
        assert!(m.key_press('7', 0, 0).accepted && m.key_press('z', 1, 1).accepted);
        assert_eq!(m.buffer_text(), "7z");
        assert!(m.key_press('!', 2, 2).text.is_none(), "за пределами маски ничего нет");
    }

    #[test]
    fn insertion_shifts_existing_characters_right() {
        let mut m = mask("9999");
        type_all(&mut m, "123");
        assert_eq!(m.buffer_text(), "123_");
        let t = m.key_press('9', 1, 1);
        assert_eq!((t.text.as_deref(), t.caret), (Some("1923"), Some(2)));
        let t = m.key_press('8', 0, 0);
        assert_eq!(t.text.as_deref(), Some("8192"), "последний символ вытолкнут, как в PrimeNG");
    }

    #[test]
    fn selection_is_replaced() {
        let mut m = mask("9999");
        type_all(&mut m, "1234");
        let t = m.key_press('5', 1, 3);
        assert_eq!(t.text.as_deref(), Some("154_"), "выделение удалено, остаток сдвинут влево, затем вправо под вставляемый символ");
    }

    #[test]
    fn backspace_and_delete_shift_left() {
        let mut m = mask("99-99");
        type_all(&mut m, "1234");
        let d = m.delete(4, 4, false);
        assert_eq!((d.text.as_deref(), d.caret), (Some("12-4_"), Some(3)), "Backspace удаляет символ слева от каретки, остаток сдвигается");
        let d = m.delete(1, 1, true);
        assert_eq!(d.text.as_deref(), Some("14-__"), "Delete удаляет символ справа и сдвигает остаток");
        let d = m.delete(0, 5, false);
        assert_eq!(d.text.as_deref(), Some("__-__"), "выделение удаляется целиком");
        let d = m.delete(0, 0, false);
        assert_eq!(d, Deleted { text: None, caret: None }, "слева от начала ничего нет");
    }

    #[test]
    fn optional_part_and_completion() {
        let mut m = mask("99-9?99");
        assert_eq!(m.default_buffer(), "__-___");
        assert_eq!(m.len(), 6);
        type_all(&mut m, "123");
        assert!(m.is_completed(), "обязательная часть заполнена");
        let t = m.key_press('4', 5, 5);
        assert!(t.accepted);
        let mut m = mask("99-99");
        let a = m.key_press('1', 0, 0);
        assert!(!a.completed);
        for (i, ch) in "234".chars().enumerate() {
            m.key_press(ch, [1, 3, 4][i], [1, 3, 4][i]);
        }
        let last = m.key_press('4', 4, 4);
        assert!(last.completed || m.is_completed());
    }

    #[test]
    fn check_val_distributes_pasted_text() {
        let mut m = mask("(999) 999-9999");
        let (text, pos) = m.check_val("555-123-4567", true);
        assert_eq!(text, "(555) 123-4567");
        assert_eq!(pos, 14);
        let (text, _) = m.check_val("a5b5", true);
        assert_eq!(text, "(55_) ___-____", "лишние символы пропускаются");
    }

    #[test]
    fn blur_clears_incomplete_values() {
        let mut m = mask("99-99");
        assert_eq!(m.check_val("12", false).0, "", "неполное значение с auto_clear очищается");
        let mut m = mask("99-99");
        assert_eq!(m.check_val("1234", false).0, "12-34");
        let mut m = Mask::new("99-99", "_", false, false, ascii_letters());
        assert_eq!(m.check_val("12", false).0, "12-__", "без auto_clear остаётся");
        assert_eq!(m.blur("12-__"), "12-__");
        let mut m = Mask::new("99-99", "_", true, true, ascii_letters());
        assert_eq!(m.blur("12-__"), "12-__", "keep_buffer не пересчитывает значение");
    }

    #[test]
    fn focus_selects_everything_in_a_full_mask() {
        let mut m = mask("99-99");
        let f = m.focus("");
        assert_eq!((f.text.as_str(), f.buffer_text.as_str(), f.begin, f.end), ("", "__-__", 0, 0), "пустая маска — каретка в начале");
        let f = m.focus("12-34");
        assert_eq!((f.buffer_text.as_str(), f.begin, f.end), ("12-34", 0, 5), "полная маска выделяется целиком");
        let mut m = Mask::new("99-99", "_", false, false, ascii_letters());
        let f = m.focus("12-3");
        assert_eq!((f.buffer_text.as_str(), f.begin, f.end), ("12-3_", 4, 4), "каретка на первой свободной позиции");
        let mut m = mask("99-99");
        let f = m.focus("12-3");
        assert_eq!(f.buffer_text, "__-__", "неполное значение при auto_clear очищается уже при фокусе (как в PrimeNG)");
    }

    #[test]
    fn unmasked_ignores_placeholders_and_constants() {
        let mut m = mask("(999) 99a");
        type_all(&mut m, "12");
        assert_eq!(m.unmasked(), "12");
        assert_eq!(mask("999").unmasked(), "");
    }

    #[test]
    fn custom_letter_pattern() {
        let digits_or_x: CharTest = Arc::new(|c| c == 'x' || c == 'y');
        let mut m = Mask::new("aa", "_", true, false, digits_or_x);
        assert!(m.key_press('x', 0, 0).accepted && !m.key_press('b', 1, 1).accepted);
    }
}
