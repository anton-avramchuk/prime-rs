//! Чистая логика Button: из свойств в классы и `data-p*` (по `buttonstyle.ts` и `Button.dataP*` PrimeNG).

use prime_core::{ClassNames, DataP, Size};

/// Серьёзность кнопки (`p-button-<severity>`).
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
    /// Значение, как в PrimeNG.
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

/// Положение иконки относительно метки.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum IconPos {
    /// Слева (по умолчанию).
    #[default]
    Left,
    /// Справа.
    Right,
    /// Сверху.
    Top,
    /// Снизу.
    Bottom,
}

impl IconPos {
    /// Значение, как в PrimeNG.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Right => "right",
            Self::Top => "top",
            Self::Bottom => "bottom",
        }
    }
}

/// Вид кнопки (`variant`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ButtonVariant {
    /// С рамкой, без фона.
    Outlined,
    /// Текстовая.
    Text,
}

/// Снимок свойств, влияющих на классы и `data-p*`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ButtonState {
    /// Метка (пустая строка равна отсутствию).
    pub label: Option<String>,
    /// Есть ли иконка, иконка загрузки или их шаблон (`hasIcon` в PrimeNG).
    pub has_icon: bool,
    /// Значение бейджа.
    pub badge: Option<String>,
    /// Идёт загрузка.
    pub loading: bool,
    /// Серьёзность.
    pub severity: Option<ButtonSeverity>,
    /// Вид.
    pub variant: Option<ButtonVariant>,
    /// Текстовая кнопка.
    pub text: bool,
    /// Кнопка с рамкой.
    pub outlined: bool,
    /// Простой текст.
    pub plain: bool,
    /// Ссылка.
    pub link: bool,
    /// С тенью.
    pub raised: bool,
    /// Скруглённая.
    pub rounded: bool,
    /// Размер.
    pub size: Option<Size>,
    /// На всю ширину.
    pub fluid: bool,
    /// Положение иконки.
    pub icon_pos: IconPos,
}

fn filled(s: &Option<String>) -> bool {
    s.as_deref().is_some_and(|v| !v.is_empty())
}

impl ButtonState {
    /// Есть ли метка.
    pub fn has_label(&self) -> bool {
        filled(&self.label)
    }

    /// Есть ли бейдж.
    pub fn has_badge(&self) -> bool {
        filled(&self.badge)
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

/// Классы корневого `<button>` в порядке `buttonstyle.ts` (без `p-ripple`, `style_class` и `pt`).
pub fn root_classes(s: &ButtonState) -> String {
    let mut cn = ClassNames::new();
    cn.add("p-button p-component")
        .add_if(s.icon_only(), "p-button-icon-only")
        .add_if(s.vertical(), "p-button-vertical")
        .add_if(s.loading, "p-button-loading")
        .add_if(s.link, "p-button-link")
        .add_if(s.severity.is_some(), &s.severity.map(|v| format!("p-button-{}", v.as_str())).unwrap_or_default())
        .add_if(s.raised, "p-button-raised")
        .add_if(s.rounded, "p-button-rounded")
        .add_if(s.is_text(), "p-button-text")
        .add_if(s.is_outlined(), "p-button-outlined")
        .add_if(s.size == Some(Size::Small), "p-button-sm")
        .add_if(s.size == Some(Size::Large), "p-button-lg")
        .add_if(s.plain, "p-button-plain")
        .add_if(s.fluid, "p-button-fluid");
    cn.finish()
}

/// Значение `data-p` корня: размер, `icon-only`, `loading`, `fluid`, `rounded`, `raised`, `outlined`, `text`, `link`, `vertical`.
pub fn root_data_p(s: &ButtonState) -> String {
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
    d.finish()
}

/// Классы оболочки иконки: `p-button-icon` и модификатор положения (только при наличии метки).
pub fn icon_classes(s: &ButtonState) -> String {
    let mut cn = ClassNames::new();
    cn.add("p-button-icon").add_if(s.has_label(), &format!("p-button-icon-{}", s.icon_pos.as_str()));
    cn.finish()
}

/// `data-p` иконки: положение и размер.
pub fn icon_data_p(s: &ButtonState) -> String {
    let mut d = DataP::new();
    d.add(s.icon_pos.as_str()).add_opt(s.size.map(Size::as_str));
    d.finish()
}

/// `data-p` метки: размер и `icon-only`.
pub fn label_data_p(s: &ButtonState) -> String {
    let mut d = DataP::new();
    d.add_opt(s.size.map(Size::as_str)).add_if(s.icon_only(), "icon-only");
    d.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base() -> ButtonState {
        ButtonState::default()
    }

    #[test]
    fn root_classes_table() {
        use ButtonSeverity::*;
        let with_label = || ButtonState { label: Some("Save".into()), ..base() };
        let cases: Vec<(ButtonState, &str, &str)> = vec![
            (base(), "p-button p-component", ""),
            (ButtonState { has_icon: true, ..base() }, "p-button p-component p-button-icon-only", "icon-only"),
            (ButtonState { has_icon: true, badge: Some("3".into()), ..base() }, "p-button p-component", ""),
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
        let s = ButtonState { label: Some(String::new()), badge: Some(String::new()), has_icon: true, ..base() };
        assert!(root_classes(&s).contains("p-button-icon-only"));
    }

    #[test]
    fn icon_classes_depend_on_label_and_position() {
        let no_label = base();
        assert_eq!(icon_classes(&no_label), "p-button-icon");
        for (pos, class) in [(IconPos::Left, "left"), (IconPos::Right, "right"), (IconPos::Top, "top"), (IconPos::Bottom, "bottom")] {
            let s = ButtonState { label: Some("x".into()), icon_pos: pos, ..base() };
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
