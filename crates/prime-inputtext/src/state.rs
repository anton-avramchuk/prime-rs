//! Чистая логика InputText: классы и `data-p` (по `inputtextstyle.ts` и `InputText.dataP` PrimeNG).

use prime_core::{ClassNames, DataP, Size, Variant};

/// Снимок свойств, влияющих на классы и `data-p`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct InputTextState {
    /// Значение не пусто.
    pub filled: bool,
    /// Размер.
    pub size: Option<Size>,
    /// Недопустимое значение.
    pub invalid: bool,
    /// Итоговый вариант (свойство или `PrimeConfig.input_variant`).
    pub variant: Option<Variant>,
    /// На всю ширину контейнера.
    pub fluid: bool,
}

/// Добавляет классы корня в порядке `inputtextstyle.ts`.
pub fn write_classes(cn: &mut ClassNames, s: &InputTextState) {
    cn.add("p-inputtext p-component")
        .add_if(s.filled, "p-filled")
        .add_if(s.size == Some(Size::Small), "p-inputtext-sm")
        .add_if(s.size == Some(Size::Large), "p-inputtext-lg")
        .add_if(s.invalid, "p-invalid")
        .add_if(s.variant == Some(Variant::Filled), "p-variant-filled")
        .add_if(s.fluid, "p-inputtext-fluid");
}

/// Значение `data-p`: `invalid`, `fluid`, `filled` (при варианте `filled`), размер.
pub fn data_p(s: &InputTextState) -> String {
    let mut d = DataP::new();
    d.add_if(s.invalid, "invalid")
        .add_if(s.fluid, "fluid")
        .add_if(s.variant == Some(Variant::Filled), "filled")
        .add_opt(s.size.map(Size::as_str));
    d.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn classes(s: &InputTextState) -> String {
        let mut cn = ClassNames::new();
        write_classes(&mut cn, s);
        cn.finish()
    }

    #[test]
    fn classes_and_data_p_table() {
        let st = |filled, size, invalid, variant, fluid| InputTextState { filled, size, invalid, variant, fluid };
        let cases = [
            (InputTextState::default(), "p-inputtext p-component", ""),
            (st(true, None, false, None, false), "p-inputtext p-component p-filled", ""),
            (st(false, Some(Size::Small), false, None, false), "p-inputtext p-component p-inputtext-sm", "small"),
            (st(false, Some(Size::Large), false, None, false), "p-inputtext p-component p-inputtext-lg", "large"),
            (st(false, None, true, None, false), "p-inputtext p-component p-invalid", "invalid"),
            (st(false, None, false, Some(Variant::Filled), false), "p-inputtext p-component p-variant-filled", "filled"),
            (st(false, None, false, Some(Variant::Outlined), false), "p-inputtext p-component", ""),
            (st(false, None, false, None, true), "p-inputtext p-component p-inputtext-fluid", "fluid"),
            (
                st(true, Some(Size::Small), true, Some(Variant::Filled), true),
                "p-inputtext p-component p-filled p-inputtext-sm p-invalid p-variant-filled p-inputtext-fluid",
                "invalid fluid filled small",
            ),
        ];
        for (s, class, data) in cases {
            assert_eq!(classes(&s), class, "{s:?}");
            assert_eq!(data_p(&s), data, "{s:?}");
        }
    }
}
