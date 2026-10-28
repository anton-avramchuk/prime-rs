//! Чистая логика RadioButton: классы и `data-p` (по `radiobuttonstyle.ts` и `RadioButton.dataP` PrimeNG).

use prime_core::{ClassNames, DataP, Size, Variant};
use std::borrow::Cow;

/// Снимок свойств, влияющих на классы и `data-p`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RadioButtonState {
    /// Отмечен (с учётом `indeterminate`).
    pub checked: bool,
    /// Недоступен.
    pub disabled: bool,
    /// Недопустимое значение.
    pub invalid: bool,
    /// Итоговый вариант (свойство или `PrimeConfig.input_variant`).
    pub variant: Option<Variant>,
    /// Размер.
    pub size: Option<Size>,
}

/// Добавляет классы корня в порядке `radiobuttonstyle.ts`.
pub fn write_classes(cn: &mut ClassNames, s: &RadioButtonState) {
    cn.add("p-radiobutton p-component")
        .add_if(s.checked, "p-radiobutton-checked")
        .add_if(s.disabled, "p-disabled")
        .add_if(s.invalid, "p-invalid")
        .add_if(s.variant == Some(Variant::Filled), "p-variant-filled")
        .add_if(s.size == Some(Size::Small), "p-radiobutton-sm p-inputfield-sm")
        .add_if(s.size == Some(Size::Large), "p-radiobutton-lg p-inputfield-lg");
}

/// Значение `data-p`: `invalid`, `checked`, `disabled`, `filled` (при варианте `filled`), размер.
pub fn data_p(s: &RadioButtonState) -> Cow<'static, str> {
    let mut d = DataP::new();
    d.add_if(s.invalid, "invalid")
        .add_if(s.checked, "checked")
        .add_if(s.disabled, "disabled")
        .add_if(s.variant == Some(Variant::Filled), "filled")
        .add_opt(s.size.map(Size::as_str));
    d.finish_cow()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn classes(s: &RadioButtonState) -> String {
        let mut cn = ClassNames::new();
        write_classes(&mut cn, s);
        cn.finish()
    }

    #[test]
    fn classes_and_data_p_table() {
        let st = |checked, disabled, invalid, variant, size| RadioButtonState { checked, disabled, invalid, variant, size };
        let cases = [
            (RadioButtonState::default(), "p-radiobutton p-component", ""),
            (st(true, false, false, None, None), "p-radiobutton p-component p-radiobutton-checked", "checked"),
            (st(false, true, false, None, None), "p-radiobutton p-component p-disabled", "disabled"),
            (st(false, false, true, None, None), "p-radiobutton p-component p-invalid", "invalid"),
            (st(false, false, false, Some(Variant::Filled), None), "p-radiobutton p-component p-variant-filled", "filled"),
            (st(false, false, false, Some(Variant::Outlined), None), "p-radiobutton p-component", ""),
            (st(false, false, false, None, Some(Size::Small)), "p-radiobutton p-component p-radiobutton-sm p-inputfield-sm", "small"),
            (st(false, false, false, None, Some(Size::Large)), "p-radiobutton p-component p-radiobutton-lg p-inputfield-lg", "large"),
            (
                st(true, true, true, Some(Variant::Filled), Some(Size::Large)),
                "p-radiobutton p-component p-radiobutton-checked p-disabled p-invalid p-variant-filled p-radiobutton-lg p-inputfield-lg",
                "invalid checked disabled filled large",
            ),
        ];
        for (s, class, data) in cases {
            assert_eq!(classes(&s), class, "{s:?}");
            assert_eq!(data_p(&s), data, "{s:?}");
        }
    }
}
