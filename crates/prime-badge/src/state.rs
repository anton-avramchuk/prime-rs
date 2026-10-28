//! Чистая логика Badge: из свойств в классы и `data-p` (по `badgestyle.ts` и `Badge.dataP` PrimeNG).

use prime_core::{ClassNames, DataP};
use std::borrow::Cow;

/// Серьёзность бейджа.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BadgeSeverity {
    /// `secondary`.
    Secondary,
    /// `info`.
    Info,
    /// `success`.
    Success,
    /// `warn`.
    Warn,
    /// `danger`.
    Danger,
    /// `contrast`.
    Contrast,
}

impl BadgeSeverity {
    /// Значение, как в PrimeNG.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Secondary => "secondary",
            Self::Info => "info",
            Self::Success => "success",
            Self::Warn => "warn",
            Self::Danger => "danger",
            Self::Contrast => "contrast",
        }
    }
}

/// Размер бейджа.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BadgeSize {
    /// `small`.
    Small,
    /// `large`.
    Large,
    /// `xlarge`.
    XLarge,
}

impl BadgeSize {
    /// Значение, как в PrimeNG.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Large => "large",
            Self::XLarge => "xlarge",
        }
    }
}

/// Снимок свойств, влияющих на классы и `data-p` (значение заимствовано, без копирования строки).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BadgeState<'a> {
    /// Значение.
    pub value: Option<&'a str>,
    /// Серьёзность.
    pub severity: Option<BadgeSeverity>,
    /// Размер.
    pub size: Option<BadgeSize>,
    /// Скрыт ли бейдж.
    pub disabled: bool,
}

fn is_circle(value: Option<&str>) -> bool {
    value.is_some_and(|v| v.chars().count() == 1)
}

/// Добавляет классы корня в порядке `badgestyle.ts`.
pub fn write_classes(cn: &mut ClassNames, s: &BadgeState) {
    cn.add("p-badge p-component")
        .add_if(is_circle(s.value), "p-badge-circle")
        .add_if(s.value.is_none_or(str::is_empty), "p-badge-dot")
        .add_if(s.size == Some(BadgeSize::Small), "p-badge-sm")
        .add_if(s.size == Some(BadgeSize::Large), "p-badge-lg")
        .add_if(s.size == Some(BadgeSize::XLarge), "p-badge-xl");
    if let Some(sev) = s.severity {
        cn.add(match sev {
            BadgeSeverity::Info => "p-badge-info",
            BadgeSeverity::Success => "p-badge-success",
            BadgeSeverity::Warn => "p-badge-warn",
            BadgeSeverity::Danger => "p-badge-danger",
            BadgeSeverity::Secondary => "p-badge-secondary",
            BadgeSeverity::Contrast => "p-badge-contrast",
        });
    }
}

/// Классы корня строкой.
pub fn classes(s: &BadgeState) -> Cow<'static, str> {
    let mut cn = ClassNames::new();
    write_classes(&mut cn, s);
    cn.finish_cow()
}

/// Значение `data-p`: `circle`, `empty`, `disabled`, серьёзность, размер.
pub fn data_p(s: &BadgeState) -> Cow<'static, str> {
    let mut d = DataP::new();
    d.add_if(is_circle(s.value), "circle")
        .add_if(s.value.is_none(), "empty")
        .add_if(s.disabled, "disabled")
        .add_opt(s.severity.map(BadgeSeverity::as_str))
        .add_opt(s.size.map(BadgeSize::as_str));
    d.finish_cow()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn state(value: Option<&str>, severity: Option<BadgeSeverity>, size: Option<BadgeSize>, disabled: bool) -> BadgeState<'_> {
        BadgeState { value, severity, size, disabled }
    }

    #[test]
    fn classes_and_data_p_table() {
        use BadgeSeverity::*;
        use BadgeSize::*;
        let cases: Vec<(BadgeState<'_>, &str, &str)> = vec![
            (state(Some("New"), None, None, false), "p-badge p-component", ""),
            (state(Some("5"), None, None, false), "p-badge p-component p-badge-circle", "circle"),
            (state(None, None, None, false), "p-badge p-component p-badge-dot", "empty"),
            (state(Some(""), None, None, false), "p-badge p-component p-badge-dot", ""),
            (state(Some("12"), Some(Danger), Some(XLarge), false), "p-badge p-component p-badge-xl p-badge-danger", "danger xlarge"),
            (
                state(Some("1"), Some(Info), Some(Small), true),
                "p-badge p-component p-badge-circle p-badge-sm p-badge-info",
                "circle disabled info small",
            ),
            (
                state(Some("x"), Some(Contrast), Some(Large), false),
                "p-badge p-component p-badge-circle p-badge-lg p-badge-contrast",
                "circle contrast large",
            ),
            (state(Some("a"), Some(Secondary), None, false), "p-badge p-component p-badge-circle p-badge-secondary", "circle secondary"),
            (state(Some("ab"), Some(Success), None, false), "p-badge p-component p-badge-success", "success"),
            (state(Some("ab"), Some(Warn), None, false), "p-badge p-component p-badge-warn", "warn"),
        ];
        for (s, class, data) in cases {
            assert_eq!(classes(&s), class, "{s:?}");
            assert_eq!(data_p(&s), data, "{s:?}");
        }
    }

    #[test]
    fn circle_counts_characters_not_bytes() {
        assert!(is_circle(Some("я")));
        assert!(!is_circle(Some("яя")));
    }
}
