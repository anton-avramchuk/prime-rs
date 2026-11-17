//! Pure Textarea logic: classes (based on PrimeNG `textareastyle.ts`) and height calculation for auto-resize.

use prime_core::{ClassNames, Size, Variant};

/// Snapshot of the properties that affect classes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TextareaState {
    /// The value is not empty.
    pub filled: bool,
    /// Automatic height adjustment is enabled.
    pub auto_resize: bool,
    /// Effective variant (the property or `PrimeConfig.input_variant`).
    pub variant: Option<Variant>,
    /// Full width of the container.
    pub fluid: bool,
    /// Size.
    pub size: Option<Size>,
    /// Invalid value.
    pub invalid: bool,
}

/// Adds the root classes in the order of `textareastyle.ts`.
pub fn write_classes(cn: &mut ClassNames, s: &TextareaState) {
    cn.add("p-textarea p-component")
        .add_if(s.filled, "p-filled")
        .add_if(s.auto_resize, "p-textarea-resizable")
        .add_if(s.variant == Some(Variant::Filled), "p-variant-filled")
        .add_if(s.fluid, "p-textarea-fluid")
        .add_if(s.size == Some(Size::Small), "p-inputfield-sm p-textarea-sm")
        .add_if(s.size == Some(Size::Large), "p-textarea-lg p-inputfield-lg")
        .add_if(s.invalid, "p-invalid");
}

/// Result of the automatic height adjustment.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Fit {
    /// Height in pixels.
    pub height: f64,
    /// Vertical scrolling (otherwise `overflow: hidden`).
    pub scroll: bool,
}

/// Height for auto-resize: `scroll_height`, but no more than `max_height` (if set); when the limit is reached, scrolling is enabled.
pub fn fit_height(scroll_height: f64, max_height: Option<f64>) -> Fit {
    match max_height {
        Some(max) if scroll_height >= max => Fit { height: max, scroll: true },
        _ => Fit { height: scroll_height, scroll: false },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn classes(s: &TextareaState) -> String {
        let mut cn = ClassNames::new();
        write_classes(&mut cn, s);
        cn.finish()
    }

    #[test]
    fn classes_table() {
        let cases = [
            (TextareaState::default(), "p-textarea p-component"),
            (TextareaState { filled: true, ..Default::default() }, "p-textarea p-component p-filled"),
            (TextareaState { auto_resize: true, ..Default::default() }, "p-textarea p-component p-textarea-resizable"),
            (TextareaState { variant: Some(Variant::Filled), ..Default::default() }, "p-textarea p-component p-variant-filled"),
            (TextareaState { variant: Some(Variant::Outlined), ..Default::default() }, "p-textarea p-component"),
            (TextareaState { fluid: true, ..Default::default() }, "p-textarea p-component p-textarea-fluid"),
            (TextareaState { size: Some(Size::Small), ..Default::default() }, "p-textarea p-component p-inputfield-sm p-textarea-sm"),
            (TextareaState { size: Some(Size::Large), ..Default::default() }, "p-textarea p-component p-textarea-lg p-inputfield-lg"),
            (TextareaState { invalid: true, ..Default::default() }, "p-textarea p-component p-invalid"),
            (
                TextareaState {
                    filled: true,
                    auto_resize: true,
                    variant: Some(Variant::Filled),
                    fluid: true,
                    size: Some(Size::Large),
                    invalid: true,
                },
                "p-textarea p-component p-filled p-textarea-resizable p-variant-filled p-textarea-fluid p-textarea-lg p-inputfield-lg p-invalid",
            ),
        ];
        for (s, class) in cases {
            assert_eq!(classes(&s), class, "{s:?}");
        }
    }

    #[test]
    fn fit_height_follows_content_until_max() {
        assert_eq!(fit_height(80.0, None), Fit { height: 80.0, scroll: false });
        assert_eq!(fit_height(80.0, Some(200.0)), Fit { height: 80.0, scroll: false });
        assert_eq!(fit_height(200.0, Some(200.0)), Fit { height: 200.0, scroll: true });
        assert_eq!(fit_height(500.0, Some(200.0)), Fit { height: 200.0, scroll: true });
    }
}
