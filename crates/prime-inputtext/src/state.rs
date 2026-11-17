//! Pure InputText logic: classes and `data-p` (after PrimeNG `inputtextstyle.ts` and `InputText.dataP`).

use prime_rs_core::{ClassNames, DataP, Size, Variant};
use std::borrow::Cow;

/// Snapshot of the properties that affect classes and `data-p`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct InputTextState {
    /// The value is not empty.
    pub filled: bool,
    /// Size.
    pub size: Option<Size>,
    /// Invalid value.
    pub invalid: bool,
    /// Resulting variant (the property or `PrimeConfig.input_variant`).
    pub variant: Option<Variant>,
    /// Full container width.
    pub fluid: bool,
}

/// Adds the root classes in the order of `inputtextstyle.ts`.
pub fn write_classes(cn: &mut ClassNames, s: &InputTextState) {
    cn.add("p-inputtext p-component")
        .add_if(s.filled, "p-filled")
        .add_if(s.size == Some(Size::Small), "p-inputtext-sm")
        .add_if(s.size == Some(Size::Large), "p-inputtext-lg")
        .add_if(s.invalid, "p-invalid")
        .add_if(s.variant == Some(Variant::Filled), "p-variant-filled")
        .add_if(s.fluid, "p-inputtext-fluid");
}

/// `data-p` value: `invalid`, `fluid`, `filled` (with the `filled` variant), size.
pub fn data_p(s: &InputTextState) -> Cow<'static, str> {
    let mut d = DataP::new();
    d.add_if(s.invalid, "invalid")
        .add_if(s.fluid, "fluid")
        .add_if(s.variant == Some(Variant::Filled), "filled")
        .add_opt(s.size.map(Size::as_str));
    d.finish_cow()
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
