//! Чистая логика InputNumber: раскладка кнопок, классы и `data-p` (по `inputnumberstyle.ts` и `InputNumber.dataP` PrimeNG).

use prime_core::{ClassNames, DataP, Size};
use std::borrow::Cow;

/// Раскладка кнопок «больше/меньше».
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ButtonLayout {
    /// Две кнопки в столбце справа от поля.
    #[default]
    Stacked,
    /// Кнопки по краям поля.
    Horizontal,
    /// Кнопки сверху и снизу.
    Vertical,
}

impl ButtonLayout {
    /// Строковое значение, как в PrimeNG.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Stacked => "stacked",
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

/// Снимок свойств, влияющих на классы и `data-p` корня.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct InputNumberState {
    /// Поле не пусто.
    pub filled: bool,
    /// `allow_empty = false`: поле считается заполненным всегда.
    pub always_filled: bool,
    /// Поле в фокусе.
    pub focused: bool,
    /// Кнопки показаны.
    pub show_buttons: bool,
    /// Раскладка кнопок.
    pub layout: ButtonLayout,
    /// На всю ширину.
    pub fluid: bool,
    /// Недопустимое значение.
    pub invalid: bool,
    /// Недоступно.
    pub disabled: bool,
    /// Вариант `filled`.
    pub variant_filled: bool,
    /// Размер.
    pub size: Option<Size>,
}

/// Добавляет классы корня в порядке `inputnumberstyle.ts`.
pub fn write_classes(cn: &mut ClassNames, s: &InputNumberState) {
    cn.add("p-inputnumber p-component p-inputwrapper")
        .add_if(s.filled || s.always_filled, "p-inputwrapper-filled")
        .add_if(s.focused, "p-inputwrapper-focus")
        .add_if(s.show_buttons && s.layout == ButtonLayout::Stacked, "p-inputnumber-stacked")
        .add_if(s.show_buttons && s.layout == ButtonLayout::Horizontal, "p-inputnumber-horizontal")
        .add_if(s.show_buttons && s.layout == ButtonLayout::Vertical, "p-inputnumber-vertical")
        .add_if(s.fluid, "p-inputnumber-fluid")
        .add_if(s.invalid, "p-invalid");
}

/// Значение `data-p`: `invalid`, `disabled`, `focus`, `fluid`, `filled`, `empty`, размер, раскладка.
pub fn data_p(s: &InputNumberState) -> Cow<'static, str> {
    let mut d = DataP::new();
    d.add_if(s.invalid, "invalid")
        .add_if(s.disabled, "disabled")
        .add_if(s.focused, "focus")
        .add_if(s.fluid, "fluid")
        .add_if(s.variant_filled, "filled")
        .add_if(!s.filled, "empty")
        .add_opt(s.size.map(Size::as_str));
    if s.show_buttons {
        d.add(s.layout.as_str());
    }
    d.finish_cow()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn classes(s: &InputNumberState) -> String {
        let mut cn = ClassNames::new();
        write_classes(&mut cn, s);
        cn.finish()
    }

    #[test]
    fn classes_and_data_p_table() {
        let base = InputNumberState::default();
        let cases = [
            (base, "p-inputnumber p-component p-inputwrapper", "empty"),
            (InputNumberState { filled: true, ..base }, "p-inputnumber p-component p-inputwrapper p-inputwrapper-filled", ""),
            (InputNumberState { always_filled: true, ..base }, "p-inputnumber p-component p-inputwrapper p-inputwrapper-filled", "empty"),
            (
                InputNumberState { focused: true, filled: true, ..base },
                "p-inputnumber p-component p-inputwrapper p-inputwrapper-filled p-inputwrapper-focus",
                "focus",
            ),
            (
                InputNumberState { show_buttons: true, filled: true, ..base },
                "p-inputnumber p-component p-inputwrapper p-inputwrapper-filled p-inputnumber-stacked",
                "stacked",
            ),
            (
                InputNumberState { show_buttons: true, layout: ButtonLayout::Horizontal, ..base },
                "p-inputnumber p-component p-inputwrapper p-inputnumber-horizontal",
                "empty horizontal",
            ),
            (
                InputNumberState { show_buttons: true, layout: ButtonLayout::Vertical, ..base },
                "p-inputnumber p-component p-inputwrapper p-inputnumber-vertical",
                "empty vertical",
            ),
            // раскладка без кнопок не влияет
            (InputNumberState { layout: ButtonLayout::Vertical, ..base }, "p-inputnumber p-component p-inputwrapper", "empty"),
            (InputNumberState { fluid: true, ..base }, "p-inputnumber p-component p-inputwrapper p-inputnumber-fluid", "fluid empty"),
            (InputNumberState { invalid: true, ..base }, "p-inputnumber p-component p-inputwrapper p-invalid", "invalid empty"),
            (InputNumberState { disabled: true, ..base }, "p-inputnumber p-component p-inputwrapper", "disabled empty"),
            (InputNumberState { variant_filled: true, ..base }, "p-inputnumber p-component p-inputwrapper", "filled empty"),
            (InputNumberState { size: Some(Size::Small), ..base }, "p-inputnumber p-component p-inputwrapper", "empty small"),
            (
                InputNumberState {
                    filled: true,
                    focused: true,
                    show_buttons: true,
                    layout: ButtonLayout::Horizontal,
                    fluid: true,
                    invalid: true,
                    disabled: true,
                    variant_filled: true,
                    size: Some(Size::Large),
                    always_filled: false,
                },
                "p-inputnumber p-component p-inputwrapper p-inputwrapper-filled p-inputwrapper-focus p-inputnumber-horizontal p-inputnumber-fluid p-invalid",
                "invalid disabled focus fluid filled large horizontal",
            ),
        ];
        for (s, class, data) in cases {
            assert_eq!(classes(&s), class, "{s:?}");
            assert_eq!(data_p(&s), data, "{s:?}");
        }
    }

    #[test]
    fn layout_names() {
        assert_eq!(ButtonLayout::default(), ButtonLayout::Stacked);
        assert_eq!(ButtonLayout::Stacked.as_str(), "stacked");
        assert_eq!(ButtonLayout::Horizontal.as_str(), "horizontal");
        assert_eq!(ButtonLayout::Vertical.as_str(), "vertical");
    }
}
