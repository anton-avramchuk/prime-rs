//! Pure Button logic: from properties to classes and `data-p*` (based on `buttonstyle.ts` and `Button.dataP*` in PrimeNG).

use prime_core::{ClassNames, DataP, Size};
use std::borrow::Cow;

/// Button severity (`p-button-<severity>`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ButtonSeverity {
    /// `secondary`.
    Secondary,
    /// `success`.
    Success,
    /// `info`.
    Info,
    /// `warn`.
    Warn,
    /// `help`.
    Help,
    /// `danger`.
    Danger,
    /// `contrast`.
    Contrast,
}

impl ButtonSeverity {
    /// Value as in PrimeNG.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Secondary => "secondary",
            Self::Success => "success",
            Self::Info => "info",
            Self::Warn => "warn",
            Self::Help => "help",
            Self::Danger => "danger",
            Self::Contrast => "contrast",
        }
    }
}

/// Icon position relative to the label.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum IconPos {
    /// Left (default).
    #[default]
    Left,
    /// Right.
    Right,
    /// Top.
    Top,
    /// Bottom.
    Bottom,
}

impl IconPos {
    /// Value as in PrimeNG.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Right => "right",
            Self::Top => "top",
            Self::Bottom => "bottom",
        }
    }
}

/// Button variant (`variant`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ButtonVariant {
    /// With a border, no background.
    Outlined,
    /// Text.
    Text,
}

/// Snapshot of the properties that affect classes and `data-p*` (strings are borrowed, not copied).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ButtonState<'a> {
    /// Label (an empty string is the same as absent).
    pub label: Option<&'a str>,
    /// Whether there is an icon, loading icon or their template (`hasIcon` in PrimeNG).
    pub has_icon: bool,
    /// Badge value.
    pub badge: Option<&'a str>,
    /// Loading.
    pub loading: bool,
    /// Severity.
    pub severity: Option<ButtonSeverity>,
    /// Variant.
    pub variant: Option<ButtonVariant>,
    /// Text button.
    pub text: bool,
    /// Outlined button.
    pub outlined: bool,
    /// Plain text.
    pub plain: bool,
    /// Link.
    pub link: bool,
    /// With shadow.
    pub raised: bool,
    /// Rounded.
    pub rounded: bool,
    /// Size.
    pub size: Option<Size>,
    /// Full width.
    pub fluid: bool,
    /// Icon position.
    pub icon_pos: IconPos,
}

fn filled(s: Option<&str>) -> bool {
    s.is_some_and(|v| !v.is_empty())
}

impl ButtonState<'_> {
    /// Whether there is a label.
    pub fn has_label(&self) -> bool {
        filled(self.label)
    }

    /// Whether there is a badge.
    pub fn has_badge(&self) -> bool {
        filled(self.badge)
    }

    fn icon_only(&self) -> bool {
        self.has_icon && !self.has_label() && !self.has_badge()
    }

    fn vertical(&self) -> bool {
        matches!(self.icon_pos, IconPos::Top | IconPos::Bottom) && self.has_label()
    }

    fn is_text(&self) -> bool {
        self.text || self.variant == Some(ButtonVariant::Text)
    }

    fn is_outlined(&self) -> bool {
        self.outlined || self.variant == Some(ButtonVariant::Outlined)
    }
}

/// Adds the classes of the root `<button>` in the order of `buttonstyle.ts` (without `p-ripple`, `style_class` and `pt`).
pub fn write_root_classes(cn: &mut ClassNames, s: &ButtonState) {
    cn.add("p-button p-component")
        .add_if(s.icon_only(), "p-button-icon-only")
        .add_if(s.vertical(), "p-button-vertical")
        .add_if(s.loading, "p-button-loading")
        .add_if(s.link, "p-button-link");
    if let Some(sev) = s.severity {
        cn.add(match sev {
            ButtonSeverity::Secondary => "p-button-secondary",
            ButtonSeverity::Success => "p-button-success",
            ButtonSeverity::Info => "p-button-info",
            ButtonSeverity::Warn => "p-button-warn",
            ButtonSeverity::Help => "p-button-help",
            ButtonSeverity::Danger => "p-button-danger",
            ButtonSeverity::Contrast => "p-button-contrast",
        });
    }
    cn.add_if(s.raised, "p-button-raised")
        .add_if(s.rounded, "p-button-rounded")
        .add_if(s.is_text(), "p-button-text")
        .add_if(s.is_outlined(), "p-button-outlined")
        .add_if(s.size == Some(Size::Small), "p-button-sm")
        .add_if(s.size == Some(Size::Large), "p-button-lg")
        .add_if(s.plain, "p-button-plain")
        .add_if(s.fluid, "p-button-fluid");
}

/// Classes of the root `<button>` as a string.
pub fn root_classes(s: &ButtonState) -> Cow<'static, str> {
    let mut cn = ClassNames::new();
    write_root_classes(&mut cn, s);
    cn.finish_cow()
}

/// `data-p` value of the root: size, `icon-only`, `loading`, `fluid`, `rounded`, `raised`, `outlined`, `text`, `link`, `vertical`.
pub fn root_data_p(s: &ButtonState) -> Cow<'static, str> {
    let mut d = DataP::new();
    d.add_opt(s.size.map(Size::as_str))
        .add_if(s.icon_only(), "icon-only")
        .add_if(s.loading, "loading")
        .add_if(s.fluid, "fluid")
        .add_if(s.rounded, "rounded")
        .add_if(s.raised, "raised")
        .add_if(s.is_outlined(), "outlined")
        .add_if(s.is_text(), "text")
        .add_if(s.link, "link")
        .add_if(s.vertical(), "vertical");
    d.finish_cow()
}

/// Icon wrapper classes: `p-button-icon` and the position modifier (only when there is a label).
pub fn icon_classes(s: &ButtonState) -> Cow<'static, str> {
    let mut cn = ClassNames::new();
    cn.add("p-button-icon");
    if s.has_label() {
        cn.add(match s.icon_pos {
            IconPos::Left => "p-button-icon-left",
            IconPos::Right => "p-button-icon-right",
            IconPos::Top => "p-button-icon-top",
            IconPos::Bottom => "p-button-icon-bottom",
        });
    }
    cn.finish_cow()
}

/// Icon `data-p`: position and size.
pub fn icon_data_p(s: &ButtonState) -> Cow<'static, str> {
    let mut d = DataP::new();
    d.add(s.icon_pos.as_str()).add_opt(s.size.map(Size::as_str));
    d.finish_cow()
}

/// Label `data-p`: size and `icon-only`.
pub fn label_data_p(s: &ButtonState) -> Cow<'static, str> {
    let mut d = DataP::new();
    d.add_opt(s.size.map(Size::as_str)).add_if(s.icon_only(), "icon-only");
    d.finish_cow()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base() -> ButtonState<'static> {
        ButtonState::default()
    }

    #[test]
    fn root_classes_table() {
        use ButtonSeverity::*;
        let with_label = || ButtonState { label: Some("Save"), ..base() };
        let cases: Vec<(ButtonState<'_>, &str, &str)> = vec![
            (base(), "p-button p-component", ""),
            (ButtonState { has_icon: true, ..base() }, "p-button p-component p-button-icon-only", "icon-only"),
            (ButtonState { has_icon: true, badge: Some("3"), ..base() }, "p-button p-component", ""),
            (ButtonState { has_icon: true, ..with_label() }, "p-button p-component", ""),
            (ButtonState { has_icon: true, icon_pos: IconPos::Top, ..with_label() }, "p-button p-component p-button-vertical", "vertical"),
            (ButtonState { icon_pos: IconPos::Bottom, ..with_label() }, "p-button p-component p-button-vertical", "vertical"),
            (ButtonState { icon_pos: IconPos::Right, ..with_label() }, "p-button p-component", ""),
            (ButtonState { loading: true, ..base() }, "p-button p-component p-button-loading", "loading"),
            (ButtonState { link: true, ..base() }, "p-button p-component p-button-link", "link"),
            (ButtonState { severity: Some(Danger), ..base() }, "p-button p-component p-button-danger", ""),
            (ButtonState { severity: Some(Help), ..base() }, "p-button p-component p-button-help", ""),
            (
                ButtonState { raised: true, rounded: true, ..base() },
                "p-button p-component p-button-raised p-button-rounded",
                "rounded raised",
            ),
            (ButtonState { text: true, ..base() }, "p-button p-component p-button-text", "text"),
            (ButtonState { variant: Some(ButtonVariant::Text), ..base() }, "p-button p-component p-button-text", "text"),
            (ButtonState { outlined: true, ..base() }, "p-button p-component p-button-outlined", "outlined"),
            (ButtonState { variant: Some(ButtonVariant::Outlined), ..base() }, "p-button p-component p-button-outlined", "outlined"),
            (ButtonState { size: Some(Size::Small), ..base() }, "p-button p-component p-button-sm", "small"),
            (ButtonState { size: Some(Size::Large), ..base() }, "p-button p-component p-button-lg", "large"),
            (ButtonState { plain: true, ..base() }, "p-button p-component p-button-plain", ""),
            (ButtonState { fluid: true, ..base() }, "p-button p-component p-button-fluid", "fluid"),
            (
                ButtonState {
                    severity: Some(Success),
                    variant: Some(ButtonVariant::Outlined),
                    size: Some(Size::Small),
                    has_icon: true,
                    ..base()
                },
                "p-button p-component p-button-icon-only p-button-success p-button-outlined p-button-sm",
                "small icon-only outlined",
            ),
        ];
        for (s, class, data) in cases {
            assert_eq!(root_classes(&s), class, "{s:?}");
            assert_eq!(root_data_p(&s), data, "{s:?}");
        }
    }

    #[test]
    fn empty_label_and_badge_count_as_absent() {
        let s = ButtonState { label: Some(""), badge: Some(""), has_icon: true, ..base() };
        assert!(root_classes(&s).contains("p-button-icon-only"));
    }

    #[test]
    fn icon_classes_depend_on_label_and_position() {
        let no_label = base();
        assert_eq!(icon_classes(&no_label), "p-button-icon");
        for (pos, class) in [(IconPos::Left, "left"), (IconPos::Right, "right"), (IconPos::Top, "top"), (IconPos::Bottom, "bottom")] {
            let s = ButtonState { label: Some("x"), icon_pos: pos, ..base() };
            assert_eq!(icon_classes(&s), format!("p-button-icon p-button-icon-{class}"));
        }
    }

    #[test]
    fn icon_and_label_data_p() {
        let s = ButtonState { icon_pos: IconPos::Right, size: Some(Size::Large), has_icon: true, ..base() };
        assert_eq!(icon_data_p(&s), "right large");
        assert_eq!(label_data_p(&s), "large icon-only");
        assert_eq!(icon_data_p(&base()), "left");
    }
}
