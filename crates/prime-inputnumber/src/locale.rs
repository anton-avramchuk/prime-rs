//! Малая таблица локалей и валют для форматирования чисел (вместо `Intl.NumberFormat` PrimeNG).
//!
//! Данные одинаковы при SSR и в браузере. Поддерживаются локали `en-US`, `en-GB`, `en-IN`, `de-DE`, `fr-FR`, `ja-JP`, `ru-RU`;
//! неизвестная локаль заменяется на `en-US`.

/// Неразрывный пробел.
const NBSP: char = '\u{a0}';
/// Узкий неразрывный пробел.
const NNBSP: char = '\u{202f}';

/// Группировка разрядов.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Grouping {
    /// Группы по три цифры.
    Standard,
    /// Последние три цифры, затем группы по две (`12,34,567`).
    Indian,
}

/// Данные локали.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LocaleData {
    /// Тег локали.
    pub tag: &'static str,
    /// Десятичный разделитель.
    pub decimal: char,
    /// Разделитель групп.
    pub group: char,
    /// Знак минуса.
    pub minus: char,
    /// Группировка.
    pub grouping: Grouping,
    /// Символ валюты стоит после числа (иначе перед ним).
    pub currency_suffix: bool,
}

const LOCALES: [LocaleData; 7] = [
    LocaleData { tag: "en-US", decimal: '.', group: ',', minus: '-', grouping: Grouping::Standard, currency_suffix: false },
    LocaleData { tag: "en-GB", decimal: '.', group: ',', minus: '-', grouping: Grouping::Standard, currency_suffix: false },
    LocaleData { tag: "en-IN", decimal: '.', group: ',', minus: '-', grouping: Grouping::Indian, currency_suffix: false },
    LocaleData { tag: "de-DE", decimal: ',', group: '.', minus: '-', grouping: Grouping::Standard, currency_suffix: true },
    LocaleData { tag: "fr-FR", decimal: ',', group: NNBSP, minus: '-', grouping: Grouping::Standard, currency_suffix: true },
    LocaleData { tag: "ja-JP", decimal: '.', group: ',', minus: '-', grouping: Grouping::Standard, currency_suffix: false },
    LocaleData { tag: "ru-RU", decimal: ',', group: NBSP, minus: '\u{2212}', grouping: Grouping::Standard, currency_suffix: true },
];

/// Локаль по умолчанию.
pub const DEFAULT_LOCALE: &LocaleData = &LOCALES[0];

/// Локаль для вывода без форматирования (`format = false`): `.` и без групп.
pub const RAW_LOCALE: LocaleData =
    LocaleData { tag: "raw", decimal: '.', group: '\0', minus: '-', grouping: Grouping::Standard, currency_suffix: false };

/// Выбирает локаль по тегу: точное совпадение, затем по языку, иначе `en-US`.
///
/// ```
/// use prime_inputnumber::resolve_locale;
///
/// assert_eq!(resolve_locale("de-DE").decimal, ',');
/// assert_eq!(resolve_locale("de").tag, "de-DE");
/// assert_eq!(resolve_locale("jp-JP").tag, "en-US");
/// ```
pub fn resolve_locale(tag: &str) -> &'static LocaleData {
    let tag = tag.trim().replace('_', "-");
    if let Some(found) = LOCALES.iter().find(|l| l.tag.eq_ignore_ascii_case(&tag)) {
        return found;
    }
    let language = tag.split('-').next().unwrap_or_default();
    match language.to_ascii_lowercase().as_str() {
        "de" => &LOCALES[3],
        "fr" => &LOCALES[4],
        "ja" => &LOCALES[5],
        "ru" => &LOCALES[6],
        _ => DEFAULT_LOCALE,
    }
}

/// Данные валюты.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Currency {
    /// Код ISO 4217 в верхнем регистре.
    pub code: String,
    /// Символ в данной локали (для неизвестных валют — код).
    pub symbol: String,
    /// Число знаков после запятой по умолчанию.
    pub digits: u32,
}

impl Currency {
    /// Данные валюты для локали; неизвестная валюта показывается кодом и имеет два знака.
    ///
    /// ```
    /// use prime_inputnumber::{Currency, resolve_locale};
    ///
    /// assert_eq!(Currency::lookup("usd", resolve_locale("en-US")).symbol, "$");
    /// assert_eq!(Currency::lookup("JPY", resolve_locale("en-US")).digits, 0);
    /// assert_eq!(Currency::lookup("XYZ", resolve_locale("en-US")).symbol, "XYZ");
    /// ```
    pub fn lookup(code: &str, locale: &LocaleData) -> Self {
        let code = code.trim().to_ascii_uppercase();
        let (symbol, digits) = match (code.as_str(), locale.tag) {
            ("USD", "en-GB") => ("US$", 2),
            ("USD", "fr-FR") => ("$US", 2),
            ("USD", _) => ("$", 2),
            ("EUR", _) => ("€", 2),
            ("GBP", _) => ("£", 2),
            ("JPY", "ja-JP") => ("\u{ffe5}", 0),
            ("JPY", _) => ("¥", 0),
            ("INR", _) => ("₹", 2),
            ("RUB", _) => ("₽", 2),
            _ => (code.as_str(), 2),
        };
        Self { symbol: symbol.to_owned(), digits, code }
    }

    /// Известна ли валюта таблице (иначе символом служит код).
    pub fn is_known(&self) -> bool {
        self.symbol != self.code
    }
}

/// Неразрывный пробел между кодом (или символом) валюты и числом.
pub const CURRENCY_GAP: char = NBSP;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_exact_language_and_fallback() {
        for (tag, expected) in [
            ("en-US", "en-US"),
            ("EN-us", "en-US"),
            ("en_IN", "en-IN"),
            ("en", "en-US"),
            ("en-AU", "en-US"),
            ("de", "de-DE"),
            ("de-AT", "de-DE"),
            ("fr-CA", "fr-FR"),
            ("ja", "ja-JP"),
            ("ru-RU", "ru-RU"),
            ("jp-JP", "en-US"),
            ("", "en-US"),
            ("xx-YY", "en-US"),
        ] {
            assert_eq!(resolve_locale(tag).tag, expected, "{tag}");
        }
    }

    #[test]
    fn currency_lookup_table() {
        let (us, gb, fr, ja) = (resolve_locale("en-US"), resolve_locale("en-GB"), resolve_locale("fr-FR"), resolve_locale("ja-JP"));
        for (code, locale, symbol, digits) in [
            ("USD", us, "$", 2),
            ("usd", us, "$", 2),
            ("USD", gb, "US$", 2),
            ("USD", fr, "$US", 2),
            ("EUR", us, "€", 2),
            ("GBP", us, "£", 2),
            ("JPY", us, "¥", 0),
            ("JPY", ja, "\u{ffe5}", 0),
            ("INR", us, "₹", 2),
            ("RUB", us, "₽", 2),
            ("XYZ", us, "XYZ", 2),
        ] {
            let c = Currency::lookup(code, locale);
            assert_eq!((c.symbol.as_str(), c.digits), (symbol, digits), "{code} {}", locale.tag);
        }
        assert!(Currency::lookup("USD", us).is_known() && !Currency::lookup("XYZ", us).is_known());
    }
}
