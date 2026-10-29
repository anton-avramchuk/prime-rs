//! Форматирование и разбор чисел (`formatValue` и `parseValue` PrimeNG на таблице локалей вместо `Intl.NumberFormat`).

use crate::locale::{CURRENCY_GAP, Currency, DEFAULT_LOCALE, Grouping, LocaleData, RAW_LOCALE, resolve_locale};

/// Режим форматирования.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum InputNumberMode {
    /// Десятичное число.
    #[default]
    Decimal,
    /// Валюта.
    Currency,
}

/// Отображение валюты.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum CurrencyDisplay {
    /// Символ (`$`, `€`).
    #[default]
    Symbol,
    /// Код (`USD`).
    Code,
}

/// Параметры форматирования (свойства компонента, влияющие на текст).
#[derive(Clone, Debug, PartialEq)]
pub struct FormatOptions {
    /// Тег локали.
    pub locale: String,
    /// Режим.
    pub mode: InputNumberMode,
    /// Код валюты (режим `Currency`).
    pub currency: String,
    /// Отображение валюты.
    pub currency_display: CurrencyDisplay,
    /// Группировать разряды.
    pub use_grouping: bool,
    /// Минимум знаков после запятой.
    pub min_fraction_digits: Option<u32>,
    /// Максимум знаков после запятой.
    pub max_fraction_digits: Option<u32>,
    /// Текст перед числом.
    pub prefix: String,
    /// Текст после числа.
    pub suffix: String,
    /// Форматировать (иначе число выводится как есть).
    pub format: bool,
}

impl Default for FormatOptions {
    fn default() -> Self {
        Self {
            locale: String::new(),
            mode: InputNumberMode::Decimal,
            currency: String::new(),
            currency_display: CurrencyDisplay::Symbol,
            use_grouping: true,
            min_fraction_digits: None,
            max_fraction_digits: None,
            prefix: String::new(),
            suffix: String::new(),
            format: true,
        }
    }
}

/// Результат разбора текста.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Parsed {
    /// Нет числа.
    Empty,
    /// Введён только минус.
    Minus,
    /// Число.
    Number(f64),
}

impl Parsed {
    /// Число, если оно есть.
    pub fn value(self) -> Option<f64> {
        match self {
            Self::Number(v) => Some(v),
            _ => None,
        }
    }
}

/// Цифры числа, разобранного из текста: знак, целая и дробная части.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Parts {
    /// Есть знак минуса.
    pub neg: bool,
    /// Цифры целой части без групп (может быть пустой).
    pub int: String,
    /// Цифры дробной части; `Some` — десятичный разделитель набран.
    pub frac: Option<String>,
}

impl Parts {
    /// Значение: пустая целая часть считается нулём; без цифр и разделителя числа нет.
    pub fn parsed(&self) -> Parsed {
        let has_digits = !self.int.is_empty() || self.frac.as_ref().is_some_and(|f| !f.is_empty());
        if !has_digits {
            return if self.neg { Parsed::Minus } else { Parsed::Empty };
        }
        let int = if self.int.is_empty() { "0" } else { self.int.as_str() };
        let text = match &self.frac {
            Some(f) if !f.is_empty() => format!("{int}.{f}"),
            _ => int.to_owned(),
        };
        match text.parse::<f64>() {
            Ok(v) => Parsed::Number(if self.neg { -v } else { v }),
            Err(_) => Parsed::Empty,
        }
    }
}

/// Готовый форматтер для набора параметров.
#[derive(Clone, Debug)]
pub struct NumberFormat {
    locale: LocaleData,
    currency: Option<Currency>,
    show_code: bool,
    use_grouping: bool,
    min_frac: usize,
    max_frac: usize,
    explicit_max: Option<u32>,
    user_prefix: String,
    user_suffix: String,
    cur_prefix: String,
    cur_suffix: String,
    format: bool,
}

/// Ограничение числа знаков после запятой (как `validateFractionDigits` PrimeNG, но не более 20).
fn clamp_digits(v: Option<u32>) -> Option<u32> {
    v.map(|d| d.min(20))
}

impl NumberFormat {
    /// Строит форматтер: выбирает локаль и валюту, определяет число знаков и обрамление.
    pub fn new(options: &FormatOptions) -> Self {
        let formatted = options.format;
        let locale = if formatted { *resolve_locale(&options.locale) } else { RAW_LOCALE };
        let currency = (formatted && options.mode == InputNumberMode::Currency && !options.currency.trim().is_empty())
            .then(|| Currency::lookup(&options.currency, &locale));
        let (default_min, default_max) = match &currency {
            Some(c) => (c.digits, c.digits),
            None => (0, 3),
        };
        let (min, max) = (clamp_digits(options.min_fraction_digits), clamp_digits(options.max_fraction_digits));
        // правила Intl: заданная граница сдвигает вторую; при `min > max` минимум приравнивается к максимуму
        let (min_frac, max_frac) = match (min, max) {
            (None, None) => (default_min, default_max),
            (Some(a), None) => (a, default_max.max(a)),
            (None, Some(b)) => (default_min.min(b), b),
            (Some(a), Some(b)) => (a.min(b), b),
        };
        let show_code = options.currency_display == CurrencyDisplay::Code || currency.as_ref().is_some_and(|c| !c.is_known());
        let (mut cur_prefix, mut cur_suffix) = (String::new(), String::new());
        if let Some(c) = &currency {
            let label = if show_code { c.code.as_str() } else { c.symbol.as_str() };
            if locale.currency_suffix {
                cur_suffix = format!("{CURRENCY_GAP}{label}");
            } else if show_code {
                cur_prefix = format!("{label}{CURRENCY_GAP}");
            } else {
                cur_prefix = label.to_owned();
            }
        }
        Self {
            locale,
            currency,
            show_code,
            use_grouping: options.use_grouping,
            min_frac: min_frac as usize,
            max_frac: max_frac as usize,
            explicit_max: options.max_fraction_digits,
            user_prefix: if formatted { options.prefix.clone() } else { String::new() },
            user_suffix: if formatted { options.suffix.clone() } else { String::new() },
            cur_prefix,
            cur_suffix,
            format: formatted,
        }
    }

    /// Десятичный разделитель.
    pub fn decimal(&self) -> char {
        self.locale.decimal
    }

    /// Разделитель групп.
    pub fn group(&self) -> char {
        self.locale.group
    }

    /// Знак минуса локали.
    pub fn minus(&self) -> char {
        self.locale.minus
    }

    /// Минимум знаков после запятой.
    pub fn min_frac(&self) -> usize {
        self.min_frac
    }

    /// Максимум знаков после запятой.
    pub fn max_frac(&self) -> usize {
        self.max_frac
    }

    /// Можно ли набрать десятичный разделитель в числе без него: в PrimeNG — только при заданном свойстве `maxFractionDigits`.
    pub fn typing_decimal_allowed(&self) -> bool {
        self.explicit_max.is_some_and(|m| m > 0) && self.max_frac > 0
    }

    /// Выводится ли символ валюты кодом.
    pub fn shows_code(&self) -> bool {
        self.show_code && self.currency.is_some()
    }

    /// Текст перед числом (пользовательский префикс и часть валюты), без знака.
    pub fn prefix(&self) -> String {
        format!("{}{}", self.user_prefix, self.cur_prefix)
    }

    /// Текст после числа (часть валюты и пользовательский суффикс).
    pub fn suffix(&self) -> String {
        format!("{}{}", self.cur_suffix, self.user_suffix)
    }

    /// Собирает текст из знака, цифр целой части и (необязательной) дроби без округления и добавления нулей.
    pub fn compose(&self, neg: bool, int: &str, frac: Option<&str>) -> String {
        if !self.format {
            let mut out = String::new();
            if neg {
                out.push('-');
            }
            out.push_str(int);
            if let Some(f) = frac {
                out.push('.');
                out.push_str(f);
            }
            return out;
        }
        let mut out = String::with_capacity(int.len() + 12);
        out.push_str(&self.user_prefix);
        if neg {
            out.push(self.locale.minus);
        }
        out.push_str(&self.cur_prefix);
        out.push_str(&group_int(int, &self.locale, self.use_grouping));
        if let Some(f) = frac {
            out.push(self.locale.decimal);
            out.push_str(f);
        }
        out.push_str(&self.cur_suffix);
        out.push_str(&self.user_suffix);
        out
    }

    /// Форматирует число: округление «от нуля» по кратчайшему десятичному представлению, число знаков по правилам формата.
    pub fn format(&self, value: f64) -> String {
        if !value.is_finite() {
            return String::new();
        }
        if !self.format {
            return value.to_string();
        }
        let shortest = value.abs().to_string();
        let (int, frac) = shortest.split_once('.').unwrap_or((shortest.as_str(), ""));
        let (int, mut frac) = round_digits(int, frac, self.max_frac);
        while frac.len() > self.min_frac && frac.ends_with('0') {
            frac.pop();
        }
        while frac.len() < self.min_frac {
            frac.push('0');
        }
        let zero = int.bytes().all(|b| b == b'0') && frac.bytes().all(|b| b == b'0');
        self.compose(value < 0.0 && !zero, &int, (!frac.is_empty()).then_some(frac.as_str()))
    }

    /// Разбирает текст на знак и цифры; `None` — в тексте есть посторонние символы или лишний десятичный разделитель.
    pub fn decompose(&self, text: &str) -> Option<Parts> {
        let mut s = text.to_owned();
        for affix in [self.user_prefix.as_str(), self.cur_prefix.as_str(), self.cur_suffix.as_str(), self.user_suffix.as_str()] {
            if !affix.is_empty() {
                s = s.replacen(affix, "", 1);
            }
        }
        let mut parts = Parts::default();
        for c in s.chars() {
            if c.is_ascii_digit() {
                match &mut parts.frac {
                    Some(f) => f.push(c),
                    None => parts.int.push(c),
                }
            } else if c == self.locale.decimal {
                if parts.frac.is_some() {
                    return None;
                }
                parts.frac = Some(String::new());
            } else if c == self.locale.minus || c == '-' {
                parts.neg = true;
            } else if c == self.locale.group || c.is_whitespace() {
                // группы и пробелы не несут значения
            } else {
                return None;
            }
        }
        Some(parts)
    }

    /// Разбирает текст в число (`parseValue`): пусто, только минус или число; недопустимый текст — пусто.
    pub fn parse(&self, text: &str) -> Parsed {
        self.decompose(text).map_or(Parsed::Empty, |p| p.parsed())
    }

    /// Границы числовой части текста: от первой цифры или разделителя до конца последней.
    pub fn core_bounds(&self, text: &str) -> (usize, usize) {
        let chars: Vec<char> = text.chars().collect();
        let is_sig = |c: &char| c.is_ascii_digit() || *c == self.locale.decimal;
        let start = chars.iter().position(is_sig).unwrap_or(chars.len());
        let end = chars.iter().rposition(is_sig).map_or(start, |i| i + 1);
        (start, end)
    }
}

/// Округляет дробь до `max` знаков «от нуля» (по десятичной записи); возвращает целую и дробную части.
fn round_digits(int: &str, frac: &str, max: usize) -> (String, String) {
    if frac.len() <= max {
        return (int.to_owned(), frac.to_owned());
    }
    let round_up = frac.as_bytes()[max] >= b'5';
    let mut digits: Vec<u8> = int.bytes().chain(frac.bytes().take(max)).collect();
    if round_up {
        let mut i = digits.len();
        loop {
            if i == 0 {
                digits.insert(0, b'1');
                break;
            }
            i -= 1;
            if digits[i] == b'9' {
                digits[i] = b'0';
            } else {
                digits[i] += 1;
                break;
            }
        }
    }
    let int_len = digits.len() - max;
    let text = String::from_utf8(digits).unwrap_or_default();
    (text[..int_len].to_owned(), text[int_len..].to_owned())
}

/// Группирует цифры целой части.
fn group_int(int: &str, locale: &LocaleData, enabled: bool) -> String {
    if !enabled || int.len() <= 3 {
        return int.to_owned();
    }
    let (head, tail) = int.split_at(int.len() - 3);
    let size = match locale.grouping {
        Grouping::Standard => 3,
        Grouping::Indian => 2,
    };
    let mut groups: Vec<&str> = Vec::new();
    let mut rest = head;
    while rest.len() > size {
        let (left, right) = rest.split_at(rest.len() - size);
        groups.push(right);
        rest = left;
    }
    groups.push(rest);
    groups.reverse();
    let sep = locale.group.to_string();
    format!("{}{sep}{tail}", groups.join(&sep))
}

/// Форматтер для `en-US` без параметров (удобство для тестов и значений по умолчанию).
pub fn default_format() -> NumberFormat {
    NumberFormat::new(&FormatOptions { locale: DEFAULT_LOCALE.tag.to_owned(), ..FormatOptions::default() })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn nf(locale: &str) -> FormatOptions {
        FormatOptions { locale: locale.to_owned(), ..FormatOptions::default() }
    }

    fn fmt(options: &FormatOptions, v: f64) -> String {
        NumberFormat::new(options).format(v)
    }

    #[test]
    fn decimal_en_us_table() {
        let plain = nf("en-US");
        let two = FormatOptions { min_fraction_digits: Some(2), ..nf("en-US") };
        let max2 = FormatOptions { max_fraction_digits: Some(2), ..nf("en-US") };
        let max0 = FormatOptions { max_fraction_digits: Some(0), ..nf("en-US") };
        let cases = [
            (&plain, 0.0, "0"),
            (&plain, 1234567.891, "1,234,567.891"),
            (&plain, -1234.5, "-1,234.5"),
            (&plain, 0.1 + 0.2, "0.3"),
            (&plain, 999.9996, "1,000"),
            (&plain, 1e21, "1,000,000,000,000,000,000,000"),
            (&plain, f64::NAN, ""),
            (&two, 1234.5, "1,234.50"),
            (&two, 5.0, "5.00"),
            (&max2, 1.005, "1.01"),
            (&max2, 1.004, "1"),
            (&max0, 0.5, "1"),
            (&max0, -0.5, "-1"),
            (&max0, 2.5, "3"),
            (&max0, -0.4, "0"),
        ];
        for (options, value, expected) in cases {
            assert_eq!(fmt(options, value), expected, "{value}");
        }
    }

    #[test]
    fn fraction_digit_rules_follow_intl() {
        let o = |min, max| FormatOptions { min_fraction_digits: min, max_fraction_digits: max, ..nf("en-US") };
        // min > max: минимум приравнивается к максимуму
        assert_eq!(fmt(&o(Some(4), Some(2)), 1.0), "1.00");
        // заданный минимум сдвигает максимум по умолчанию (3)
        assert_eq!(fmt(&o(Some(5), None), 1.0), "1.00000");
        // заданный максимум меньше минимума по умолчанию валюты — минимум уменьшается
        let cur = FormatOptions { mode: InputNumberMode::Currency, currency: "USD".into(), max_fraction_digits: Some(0), ..nf("en-US") };
        assert_eq!(fmt(&cur, 1234.5), "$1,235");
        assert_eq!(fmt(&o(None, Some(25)), 0.5), "0.5");
    }

    #[test]
    fn locales_table() {
        let two = |tag: &str| FormatOptions { min_fraction_digits: Some(2), ..nf(tag) };
        let cases = [
            ("de-DE", 1234.5, "1.234,50"),
            ("fr-FR", 1234.5, "1\u{202f}234,50"),
            ("ru-RU", 1234.5, "1\u{a0}234,50"),
            ("ru-RU", -1234.5, "\u{2212}1\u{a0}234,50"),
            ("en-IN", 1234567.891, "12,34,567.891"),
            ("en-IN", 123456.0, "1,23,456.00"),
            ("en-IN", 12345.0, "12,345.00"),
            ("en-IN", 1234.0, "1,234.00"),
            ("en-IN", 100.0, "100.00"),
            ("ja-JP", 1234.5, "1,234.50"),
            ("xx-YY", 1234.5, "1,234.50"),
            ("", 1234.5, "1,234.50"),
        ];
        for (tag, value, expected) in cases {
            assert_eq!(fmt(&two(tag), value), expected, "{tag} {value}");
        }
        assert_eq!(fmt(&nf("en-IN"), 1234567.891), "12,34,567.891");
    }

    #[test]
    fn grouping_can_be_disabled() {
        let o = FormatOptions { use_grouping: false, ..nf("en-US") };
        assert_eq!(fmt(&o, 1234567.0), "1234567");
    }

    #[test]
    fn prefix_and_suffix() {
        let o = FormatOptions { prefix: "Expires in ".into(), suffix: " days".into(), ..nf("en-US") };
        assert_eq!(fmt(&o, 7.0), "Expires in 7 days");
        assert_eq!(fmt(&o, -7.0), "Expires in -7 days");
        assert_eq!(fmt(&FormatOptions { suffix: " mi".into(), ..nf("en-US") }, 1234.0), "1,234 mi");
    }

    #[test]
    fn currency_table() {
        let cur = |tag: &str, code: &str, display| FormatOptions {
            mode: InputNumberMode::Currency,
            currency: code.into(),
            currency_display: display,
            ..nf(tag)
        };
        let sym = CurrencyDisplay::Symbol;
        let cases = [
            (cur("en-US", "USD", sym), 1234.5, "$1,234.50"),
            (cur("en-US", "USD", sym), -1234.5, "-$1,234.50"),
            (cur("en-US", "USD", sym), 0.0, "$0.00"),
            (cur("de-DE", "EUR", sym), 1234.5, "1.234,50\u{a0}€"),
            (cur("de-DE", "EUR", sym), -1234.5, "-1.234,50\u{a0}€"),
            (cur("fr-FR", "EUR", sym), 1234.5, "1\u{202f}234,50\u{a0}€"),
            (cur("ru-RU", "RUB", sym), 1234.5, "1\u{a0}234,50\u{a0}₽"),
            (cur("en-IN", "INR", CurrencyDisplay::Code), 1234.0, "INR\u{a0}1,234.00"),
            (cur("en-IN", "INR", sym), 1234567.0, "₹12,34,567.00"),
            (cur("en-US", "JPY", sym), 1234.5, "¥1,235"),
            (cur("ja-JP", "JPY", sym), 1234.5, "\u{ffe5}1,235"),
            (cur("en-GB", "GBP", sym), 1234.5, "£1,234.50"),
            (cur("en-GB", "USD", sym), 1.0, "US$1.00"),
            (cur("de-DE", "EUR", CurrencyDisplay::Code), 1.0, "1,00\u{a0}EUR"),
            (cur("en-US", "XYZ", sym), 1234.5, "XYZ\u{a0}1,234.50"),
            (cur("en-US", "", sym), 1234.5, "1,234.5"),
        ];
        for (options, value, expected) in cases {
            assert_eq!(fmt(&options, value), expected, "{options:?} {value}");
        }
    }

    #[test]
    fn unformatted_output_is_plain() {
        let o = FormatOptions { format: false, locale: "de-DE".into(), prefix: "x".into(), ..FormatOptions::default() };
        assert_eq!(fmt(&o, 1234.5), "1234.5");
        assert_eq!(NumberFormat::new(&o).parse("1234.5"), Parsed::Number(1234.5));
        assert_eq!(NumberFormat::new(&o).compose(true, "12", Some("5")), "-12.5");
    }

    #[test]
    fn parse_table() {
        let usd = FormatOptions { mode: InputNumberMode::Currency, currency: "USD".into(), ..nf("en-US") };
        let eur = FormatOptions { mode: InputNumberMode::Currency, currency: "EUR".into(), ..nf("de-DE") };
        let affixed = FormatOptions { prefix: "Expires in ".into(), suffix: " days".into(), ..nf("en-US") };
        let cases: [(&FormatOptions, &str, Parsed); 14] = [
            (&nf("en-US"), "", Parsed::Empty),
            (&nf("en-US"), "-", Parsed::Minus),
            (&nf("en-US"), "1,234.5", Parsed::Number(1234.5)),
            (&nf("en-US"), "-1,234.5", Parsed::Number(-1234.5)),
            (&nf("en-US"), "1.", Parsed::Number(1.0)),
            (&nf("en-US"), ".5", Parsed::Number(0.5)),
            (&nf("en-US"), ".", Parsed::Empty),
            (&nf("en-US"), "abc", Parsed::Empty),
            (&nf("en-US"), "1.2.3", Parsed::Empty),
            (&nf("de-DE"), "1.234,5", Parsed::Number(1234.5)),
            (&nf("ru-RU"), "\u{2212}1\u{a0}234,5", Parsed::Number(-1234.5)),
            (&usd, "-$1,234.50", Parsed::Number(-1234.5)),
            (&eur, "1.234,50\u{a0}€", Parsed::Number(1234.5)),
            (&affixed, "Expires in 7 days", Parsed::Number(7.0)),
        ];
        for (options, text, expected) in cases {
            assert_eq!(NumberFormat::new(options).parse(text), expected, "{text:?}");
        }
    }

    #[test]
    fn parse_inverts_format() {
        let values = [0.0, 1.0, -1.0, 12.0, 1234.5, -1234.5, 1234567.891, 0.001, 100.0, 99999.999];
        let modes = [InputNumberMode::Decimal, InputNumberMode::Currency];
        for tag in ["en-US", "en-GB", "en-IN", "de-DE", "fr-FR", "ja-JP", "ru-RU"] {
            for mode in modes {
                for currency in ["USD", "EUR", "JPY", "INR", "RUB", "XYZ"] {
                    let options = FormatOptions { mode, currency: currency.into(), max_fraction_digits: Some(3), ..nf(tag) };
                    let f = NumberFormat::new(&options);
                    for v in values {
                        let parsed = f.parse(&f.format(v));
                        assert_eq!(parsed.value(), Some(v), "{tag} {mode:?} {currency} {v}: {:?}", f.format(v));
                    }
                }
            }
        }
    }

    #[test]
    fn decompose_keeps_typed_fraction_and_rejects_garbage() {
        let f = default_format();
        assert_eq!(f.decompose("1,234.50"), Some(Parts { neg: false, int: "1234".into(), frac: Some("50".into()) }));
        assert_eq!(f.decompose("-0."), Some(Parts { neg: true, int: "0".into(), frac: Some(String::new()) }));
        assert_eq!(f.decompose("1x"), None);
        assert_eq!(f.decompose("1..2"), None);
    }

    #[test]
    fn core_bounds_skip_affixes() {
        let o = FormatOptions { mode: InputNumberMode::Currency, currency: "USD".into(), ..nf("en-US") };
        let f = NumberFormat::new(&o);
        assert_eq!(f.core_bounds("-$1,234.50"), (2, 10));
        let d = NumberFormat::new(&FormatOptions { suffix: " mi".into(), ..nf("en-US") });
        assert_eq!(d.core_bounds("12 mi"), (0, 2));
        assert_eq!(d.core_bounds(""), (0, 0));
    }

    #[test]
    fn typing_decimal_follows_primeng_quirk() {
        assert!(!default_format().typing_decimal_allowed());
        assert!(NumberFormat::new(&FormatOptions { max_fraction_digits: Some(2), ..nf("en-US") }).typing_decimal_allowed());
        assert!(!NumberFormat::new(&FormatOptions { max_fraction_digits: Some(0), ..nf("en-US") }).typing_decimal_allowed());
    }
}
