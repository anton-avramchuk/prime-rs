//! Pure Tag logic: from properties to classes and `data-p` (after PrimeNG `tagstyle.ts` and `Tag.dataP`).

use prime_rs_core::{ClassNames, DataP};
use std::borrow::Cow;

/// Tag severity (`p-tag-<severity>`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TagSeverity {
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

impl TagSeverity {
    /// Value, as in PrimeNG.
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

    fn class(self) -> &'static str {
        match self {
            Self::Secondary => "p-tag-secondary",
            Self::Info => "p-tag-info",
            Self::Success => "p-tag-success",
            Self::Warn => "p-tag-warn",
            Self::Danger => "p-tag-danger",
            Self::Contrast => "p-tag-contrast",
        }
    }
}

/// Snapshot of the properties that affect classes and `data-p`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TagState {
    /// Severity.
    pub severity: Option<TagSeverity>,
    /// Rounded edges.
    pub rounded: bool,
}

/// Adds the root classes in the order of `tagstyle.ts`.
pub fn write_classes(cn: &mut ClassNames, s: &TagState) {
    cn.add("p-tag p-component");
    if let Some(sev) = s.severity {
        cn.add(sev.class());
    }
    cn.add_if(s.rounded, "p-tag-rounded");
}

/// `data-p` value: `rounded`, severity.
pub fn data_p(s: &TagState) -> Cow<'static, str> {
    let mut d = DataP::new();
    d.add_if(s.rounded, "rounded").add_opt(s.severity.map(TagSeverity::as_str));
    d.finish_cow()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn classes(s: &TagState) -> String {
        let mut cn = ClassNames::new();
        write_classes(&mut cn, s);
        cn.finish()
    }

    #[test]
    fn classes_and_data_p_table() {
        use TagSeverity::*;
        let cases = [
            (TagState::default(), "p-tag p-component", ""),
            (TagState { severity: Some(Info), rounded: false }, "p-tag p-component p-tag-info", "info"),
            (TagState { severity: Some(Success), rounded: false }, "p-tag p-component p-tag-success", "success"),
            (TagState { severity: Some(Warn), rounded: false }, "p-tag p-component p-tag-warn", "warn"),
            (TagState { severity: Some(Danger), rounded: false }, "p-tag p-component p-tag-danger", "danger"),
            (TagState { severity: Some(Secondary), rounded: false }, "p-tag p-component p-tag-secondary", "secondary"),
            (TagState { severity: Some(Contrast), rounded: false }, "p-tag p-component p-tag-contrast", "contrast"),
            (TagState { severity: None, rounded: true }, "p-tag p-component p-tag-rounded", "rounded"),
            (TagState { severity: Some(Success), rounded: true }, "p-tag p-component p-tag-success p-tag-rounded", "rounded success"),
        ];
        for (s, class, data) in cases {
            assert_eq!(classes(&s), class, "{s:?}");
            assert_eq!(data_p(&s), data, "{s:?}");
        }
    }
}
