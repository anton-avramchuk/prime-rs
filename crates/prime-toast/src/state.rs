//! Pure Toast logic: position, classes, duplicates, lifetime, responsive rules (after PrimeNG `toast.ts` and `toaststyle.ts`).

use crate::message::{ToastMessage, ToastSeverity};

/// Container position on screen.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ToastPosition {
    /// Top right (default).
    #[default]
    TopRight,
    /// Top left.
    TopLeft,
    /// Top center.
    TopCenter,
    /// Bottom left.
    BottomLeft,
    /// Bottom center.
    BottomCenter,
    /// Bottom right.
    BottomRight,
    /// Screen center.
    Center,
}

impl ToastPosition {
    /// Value, as in PrimeNG (`top-right`, …).
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TopRight => "top-right",
            Self::TopLeft => "top-left",
            Self::TopCenter => "top-center",
            Self::BottomLeft => "bottom-left",
            Self::BottomCenter => "bottom-center",
            Self::BottomRight => "bottom-right",
            Self::Center => "center",
        }
    }

    fn is_top(self) -> bool {
        matches!(self, Self::TopRight | Self::TopLeft | Self::TopCenter)
    }

    fn is_bottom(self) -> bool {
        matches!(self, Self::BottomLeft | Self::BottomCenter | Self::BottomRight)
    }

    fn is_right(self) -> bool {
        matches!(self, Self::TopRight | Self::BottomRight)
    }

    fn is_left(self) -> bool {
        matches!(self, Self::TopLeft | Self::BottomLeft)
    }

    fn is_centered_horizontally(self) -> bool {
        matches!(self, Self::Center | Self::TopCenter | Self::BottomCenter)
    }
}

/// Container inline style: `position: fixed` and offsets by position (20px at edges, 50% at center).
pub fn position_style(position: ToastPosition) -> String {
    let mut css = String::from("position: fixed");
    if position.is_top() {
        css.push_str("; top: 20px");
    } else if position == ToastPosition::Center {
        css.push_str("; top: 50%");
    }
    if position.is_right() {
        css.push_str("; right: 20px");
    }
    if position.is_bottom() {
        css.push_str("; bottom: 20px");
    }
    if position.is_left() {
        css.push_str("; left: 20px");
    } else if position.is_centered_horizontally() {
        css.push_str("; left: 50%");
    }
    css
}

/// Container classes: `p-toast p-component p-toast-<position>` and a custom one.
pub fn root_classes(position: ToastPosition, style_class: Option<&str>) -> String {
    let mut cn = format!("p-toast p-component p-toast-{}", position.as_str());
    if let Some(extra) = style_class.filter(|c| !c.is_empty()) {
        cn.push(' ');
        cn.push_str(extra);
    }
    cn
}

/// Message classes: `p-toast-message`, severity class (except custom) and a custom one.
pub fn message_classes(severity: &ToastSeverity, style_class: Option<&str>) -> String {
    let mut cn = String::from("p-toast-message");
    if !matches!(severity, ToastSeverity::Custom(_)) {
        cn.push_str(" p-toast-message-");
        cn.push_str(severity.as_str());
    }
    if let Some(extra) = style_class.filter(|c| !c.is_empty()) {
        cn.push(' ');
        cn.push_str(extra);
    }
    cn
}

/// Whether the message is addressed to this `Toast`: keys are equal (both `None` are equal too).
pub fn matches_key(toast_key: Option<&str>, message_key: Option<&str>) -> bool {
    toast_key == message_key
}

/// The collection has a message with the same `summary`, `detail` and severity.
pub fn contains_message(collection: &[ToastMessage], message: &ToastMessage) -> bool {
    collection.iter().any(|m| m.summary == message.summary && m.detail == message.detail && m.severity == message.severity)
}

/// Whether a message can be added to a `Toast`: key first, then duplicate prevention among shown and among all added.
pub fn can_add(
    toast_key: Option<&str>,
    message: &ToastMessage,
    open: &[ToastMessage],
    archive: &[ToastMessage],
    prevent_open_duplicates: bool,
    prevent_duplicates: bool,
) -> bool {
    if !matches_key(toast_key, message.key.as_deref()) {
        return false;
    }
    if prevent_open_duplicates && contains_message(open, message) {
        return false;
    }
    !(prevent_duplicates && contains_message(archive, message))
}

/// Display time, ms: from the message, else from `Toast`, else 3000; zero values count as unset.
pub fn lifetime(message_life: Option<u32>, toast_life: u32) -> u32 {
    message_life.filter(|l| *l > 0).unwrap_or(if toast_life > 0 { toast_life } else { 3000 })
}

/// `@media` rules for the container: `attr` is the container attribute name, styles of each breakpoint carry `!important`.
pub fn breakpoints_css(attr: &str, breakpoints: &[(String, Vec<(String, String)>)]) -> String {
    breakpoints
        .iter()
        .map(|(bp, styles)| {
            let body: String = styles.iter().map(|(prop, value)| format!("{prop}:{value} !important;")).collect();
            format!("@media screen and (max-width: {bp}) {{ .p-toast[{attr}] {{ {body} }} }}\n")
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::ToastPosition::*;
    use super::*;

    fn msg(summary: &str, detail: &str, severity: ToastSeverity) -> ToastMessage {
        ToastMessage::new(severity, summary, detail)
    }

    #[test]
    fn position_style_table() {
        // (position, expected parts, missing parts)
        let cases: [(ToastPosition, &[&str], &[&str]); 7] = [
            (TopRight, &["top: 20px", "right: 20px"], &["bottom", "left"]),
            (TopLeft, &["top: 20px", "left: 20px"], &["bottom", "right"]),
            (TopCenter, &["top: 20px", "left: 50%"], &["bottom", "right"]),
            (BottomLeft, &["bottom: 20px", "left: 20px"], &["top", "right"]),
            (BottomCenter, &["bottom: 20px", "left: 50%"], &["top", "right"]),
            (BottomRight, &["bottom: 20px", "right: 20px"], &["top", "left"]),
            (Center, &["top: 50%", "left: 50%"], &["bottom", "right"]),
        ];
        for (p, has, hasnt) in cases {
            let css = position_style(p);
            assert!(css.starts_with("position: fixed"), "{p:?}: {css}");
            for part in has {
                assert!(css.contains(part), "{p:?}: no {part} in {css}");
            }
            for part in hasnt {
                assert!(!css.contains(part), "{p:?}: extra {part} in {css}");
            }
        }
    }

    #[test]
    fn class_tables() {
        assert_eq!(root_classes(TopRight, None), "p-toast p-component p-toast-top-right");
        assert_eq!(root_classes(BottomCenter, Some("x y")), "p-toast p-component p-toast-bottom-center x y");
        assert_eq!(root_classes(Center, Some("")), "p-toast p-component p-toast-center");
        assert_eq!(message_classes(&ToastSeverity::Success, None), "p-toast-message p-toast-message-success");
        assert_eq!(message_classes(&ToastSeverity::Info, Some("blur")), "p-toast-message p-toast-message-info blur");
        for (severity, name) in [
            (ToastSeverity::Warn, "warn"),
            (ToastSeverity::Error, "error"),
            (ToastSeverity::Secondary, "secondary"),
            (ToastSeverity::Contrast, "contrast"),
        ] {
            assert_eq!(message_classes(&severity, None), format!("p-toast-message p-toast-message-{name}"));
        }
        assert_eq!(message_classes(&ToastSeverity::Custom("custom".into()), None), "p-toast-message", "custom severity has no class");
    }

    #[test]
    fn key_matching() {
        assert!(matches_key(None, None));
        assert!(matches_key(Some("a"), Some("a")));
        assert!(!matches_key(Some("a"), Some("b")));
        assert!(!matches_key(Some("a"), None));
        assert!(!matches_key(None, Some("a")));
    }

    #[test]
    fn duplicates_table() {
        let a = msg("S", "D", ToastSeverity::Info);
        let same = msg("S", "D", ToastSeverity::Info);
        let other_detail = msg("S", "other", ToastSeverity::Info);
        let other_severity = msg("S", "D", ToastSeverity::Error);
        assert!(contains_message(std::slice::from_ref(&a), &same));
        assert!(!contains_message(std::slice::from_ref(&a), &other_detail));
        assert!(!contains_message(std::slice::from_ref(&a), &other_severity));
        assert!(!contains_message(&[], &a));
    }

    #[test]
    fn can_add_table() {
        let m = msg("S", "D", ToastSeverity::Info);
        let keyed = msg("S", "D", ToastSeverity::Info).key("k");
        let open = [m.clone()];
        let archive = [m.clone()];
        // (Toast key, message, open, archive, prevent_open, prevent_all, expectation)
        type Case<'a> = (Option<&'a str>, &'a ToastMessage, &'a [ToastMessage], &'a [ToastMessage], bool, bool, bool);
        let cases: [Case; 8] = [
            (None, &m, &[], &[], false, false, true),
            (Some("k"), &m, &[], &[], false, false, false),
            (Some("k"), &keyed, &[], &[], false, false, true),
            (None, &m, &open, &[], false, false, true),
            (None, &m, &open, &[], true, false, false),
            (None, &m, &[], &archive, false, false, true),
            (None, &m, &[], &archive, false, true, false),
            (Some("k"), &keyed, &open, &archive, true, true, false),
        ];
        // last case: key matches, but a message with the same texts is already among the shown ones (key is ignored in the comparison)
        for (i, (key, message, open, archive, po, pd, want)) in cases.into_iter().enumerate() {
            assert_eq!(can_add(key, message, open, archive, po, pd), want, "case {i}");
        }
    }

    #[test]
    fn lifetime_table() {
        assert_eq!(lifetime(None, 3000), 3000);
        assert_eq!(lifetime(Some(500), 3000), 500);
        assert_eq!(lifetime(Some(0), 4000), 4000, "zero on the message — unset");
        assert_eq!(lifetime(None, 0), 3000, "zero on Toast — default value");
        assert_eq!(lifetime(Some(0), 0), 3000);
    }

    #[test]
    fn breakpoints_css_table() {
        assert_eq!(breakpoints_css("data-toast-1", &[]), "");
        let css =
            breakpoints_css("data-toast-7", &[("920px".into(), vec![("width".into(), "50%".into()), ("right".into(), "auto".into())])]);
        assert_eq!(
            css,
            "@media screen and (max-width: 920px) { .p-toast[data-toast-7] { width:50% !important;right:auto !important; } }\n"
        );
    }
}
