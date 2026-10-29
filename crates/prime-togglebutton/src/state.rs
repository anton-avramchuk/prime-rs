//! Чистая логика ToggleButton: классы и `data-p` (по `togglebuttonstyle.ts` и `ToggleButton.dataP` PrimeNG).

use prime_core::{ClassNames, DataP, Size};
use std::borrow::Cow;

/// Положение иконки относительно метки.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum IconPos {
    /// Слева от метки.
    #[default]
    Left,
    /// Справа от метки.
    Right,
}

impl IconPos {
    /// Класс оболочки иконки.
    pub fn class(self) -> &'static str {
        match self {
            Self::Left => "p-togglebutton-icon p-togglebutton-icon-left",
            Self::Right => "p-togglebutton-icon p-togglebutton-icon-right",
        }
    }
}

/// Снимок свойств, влияющих на классы и `data-p`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ToggleButtonState {
    /// Включена.
    pub checked: bool,
    /// Недоступна.
    pub disabled: bool,
    /// Недопустимое значение.
    pub invalid: bool,
    /// Размер.
    pub size: Option<Size>,
    /// На всю ширину.
    pub fluid: bool,
}

/// Добавляет классы корня в порядке `togglebuttonstyle.ts`.
pub fn write_classes(cn: &mut ClassNames, s: &ToggleButtonState) {
    cn.add("p-togglebutton p-component")
        .add_if(s.checked, "p-togglebutton-checked")
        .add_if(s.invalid, "p-invalid")
        .add_if(s.disabled, "p-disabled")
        .add_if(s.size == Some(Size::Small), "p-togglebutton-sm p-inputfield-sm")
        .add_if(s.size == Some(Size::Large), "p-togglebutton-lg p-inputfield-lg")
        .add_if(s.fluid, "p-togglebutton-fluid");
}

/// Значение `data-p`: `checked`, `invalid`, размер.
pub fn data_p(s: &ToggleButtonState) -> Cow<'static, str> {
    let mut d = DataP::new();
    d.add_if(s.checked, "checked").add_if(s.invalid, "invalid").add_opt(s.size.map(Size::as_str));
    d.finish_cow()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn classes(s: &ToggleButtonState) -> String {
        let mut cn = ClassNames::new();
        write_classes(&mut cn, s);
        cn.finish()
    }

    #[test]
    fn classes_and_data_p_table() {
        let base = ToggleButtonState::default();
        let cases = [
            (base, "p-togglebutton p-component", ""),
            (ToggleButtonState { checked: true, ..base }, "p-togglebutton p-component p-togglebutton-checked", "checked"),
            (ToggleButtonState { invalid: true, ..base }, "p-togglebutton p-component p-invalid", "invalid"),
            (ToggleButtonState { disabled: true, ..base }, "p-togglebutton p-component p-disabled", ""),
            (
                ToggleButtonState { size: Some(Size::Small), ..base },
                "p-togglebutton p-component p-togglebutton-sm p-inputfield-sm",
                "small",
            ),
            (
                ToggleButtonState { size: Some(Size::Large), ..base },
                "p-togglebutton p-component p-togglebutton-lg p-inputfield-lg",
                "large",
            ),
            (ToggleButtonState { fluid: true, ..base }, "p-togglebutton p-component p-togglebutton-fluid", ""),
            (
                ToggleButtonState { checked: true, disabled: true, invalid: true, size: Some(Size::Large), fluid: true },
                "p-togglebutton p-component p-togglebutton-checked p-invalid p-disabled p-togglebutton-lg p-inputfield-lg p-togglebutton-fluid",
                "checked invalid large",
            ),
        ];
        for (s, class, data) in cases {
            assert_eq!(classes(&s), class, "{s:?}");
            assert_eq!(data_p(&s), data, "{s:?}");
        }
    }

    #[test]
    fn icon_pos_classes() {
        assert_eq!(IconPos::default(), IconPos::Left);
        assert_eq!(IconPos::Left.class(), "p-togglebutton-icon p-togglebutton-icon-left");
        assert_eq!(IconPos::Right.class(), "p-togglebutton-icon p-togglebutton-icon-right");
    }
}
