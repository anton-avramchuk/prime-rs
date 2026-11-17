//! Pure Image logic: classes, zoom bounds and steps, `transform` (after PrimeNG's `image.ts` and `imagestyle.ts`).

use prime_core::ClassNames;
use std::borrow::Cow;

/// Zoom step.
pub const ZOOM_STEP: f64 = 0.1;
/// Minimum zoom.
pub const ZOOM_MIN: f64 = 0.5;
/// Maximum zoom.
pub const ZOOM_MAX: f64 = 1.5;
/// Default zoom.
pub const ZOOM_DEFAULT: f64 = 1.0;
/// Tolerance for comparing fractional zooms.
const EPS: f64 = 1e-9;

/// Value of the image's `loading`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImageLoading {
    /// Lazy loading.
    Lazy,
    /// Eager loading.
    Eager,
}

impl ImageLoading {
    /// Attribute value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Lazy => "lazy",
            Self::Eager => "eager",
        }
    }
}

/// Toolbar action button.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActionKind {
    /// Rotate right.
    RotateRight,
    /// Rotate left.
    RotateLeft,
    /// Zoom out.
    ZoomOut,
    /// Zoom in.
    ZoomIn,
    /// Close.
    Close,
}

/// Root classes: `p-image p-component`, `p-image-preview` and the user class.
pub fn root_classes(preview: bool, extra: Option<&str>) -> Cow<'static, str> {
    let mut cn = ClassNames::new();
    cn.add("p-image p-component").add_if(preview, "p-image-preview").add_user(extra);
    cn.finish_cow()
}

/// Toolbar button classes; zoom out and zoom in get `p-disabled` when the step is impossible.
pub fn action_classes(kind: ActionKind, disabled: bool) -> &'static str {
    match (kind, disabled) {
        (ActionKind::RotateRight, _) => "p-image-action p-image-rotate-right-button",
        (ActionKind::RotateLeft, _) => "p-image-action p-image-rotate-left-button",
        (ActionKind::ZoomOut, false) => "p-image-action p-image-zoom-out-button",
        (ActionKind::ZoomOut, true) => "p-image-action p-image-zoom-out-button p-disabled",
        (ActionKind::ZoomIn, false) => "p-image-action p-image-zoom-in-button",
        (ActionKind::ZoomIn, true) => "p-image-action p-image-zoom-in-button p-disabled",
        (ActionKind::Close, _) => "p-image-action p-image-close-button",
    }
}

/// Whether zooming out is impossible: the step does not leave the zoom above the minimum (`scale - step <= min`).
pub fn zoom_out_disabled(scale: f64) -> bool {
    scale - ZOOM_STEP <= ZOOM_MIN + EPS
}

/// Whether zooming in is impossible: the step does not leave the zoom below the maximum (`scale + step >= max`).
pub fn zoom_in_disabled(scale: f64) -> bool {
    scale + ZOOM_STEP >= ZOOM_MAX - EPS
}

/// Zoom after a step up (rounded to hundredths).
pub fn zoom_in(scale: f64) -> f64 {
    round2(scale + ZOOM_STEP)
}

/// Zoom after a step down (rounded to hundredths).
pub fn zoom_out(scale: f64) -> f64 {
    round2(scale - ZOOM_STEP)
}

fn round2(v: f64) -> f64 {
    (v * 100.0).round() / 100.0
}

/// The `transform` value of the enlarged image.
pub fn transform(rotate: i32, scale: f64) -> String {
    format!("rotate({rotate}deg) scale({scale})")
}

/// The `transform` style of the enlarged image.
pub fn preview_style(rotate: i32, scale: f64) -> String {
    format!("transform: {}", transform(rotate, scale))
}

/// Preview button size from the image's `width` and `height`: numbers get `px` (in PrimeNG, `height + 'px'`), anything else
/// (`50%`, `10rem`) is taken as is; without sizes there is no style.
pub fn dimension_style(width: Option<&str>, height: Option<&str>) -> Option<String> {
    let css = |v: &str| {
        let v = v.trim();
        if v.parse::<f64>().is_ok() { format!("{v}px") } else { v.to_owned() }
    };
    let mut parts = Vec::new();
    if let Some(h) = height.filter(|h| !h.trim().is_empty()) {
        parts.push(format!("height: {}", css(h)));
    }
    if let Some(w) = width.filter(|w| !w.trim().is_empty()) {
        parts.push(format!("width: {}", css(w)));
    }
    (!parts.is_empty()).then(|| parts.join("; "))
}

/// Whether a click on the mask closes the preview (not after a toolbar action or a click on the image).
pub fn mask_should_close(preview_click: bool) -> bool {
    !preview_click
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn root_classes_table() {
        assert_eq!(root_classes(false, None), "p-image p-component");
        assert_eq!(root_classes(true, None), "p-image p-component p-image-preview");
        assert_eq!(root_classes(true, Some("mine")), "p-image p-component p-image-preview mine");
    }

    #[test]
    fn action_classes_table() {
        assert_eq!(action_classes(ActionKind::RotateRight, false), "p-image-action p-image-rotate-right-button");
        assert_eq!(action_classes(ActionKind::RotateLeft, true), "p-image-action p-image-rotate-left-button");
        assert_eq!(action_classes(ActionKind::ZoomOut, true), "p-image-action p-image-zoom-out-button p-disabled");
        assert_eq!(action_classes(ActionKind::ZoomIn, false), "p-image-action p-image-zoom-in-button");
        assert_eq!(action_classes(ActionKind::Close, true), "p-image-action p-image-close-button");
    }

    #[test]
    fn zoom_bounds() {
        assert!(!zoom_in_disabled(1.0) && !zoom_out_disabled(1.0));
        assert!(!zoom_in_disabled(1.3) && zoom_in_disabled(1.4), "from 1.4 the step gives 1.5");
        assert!(!zoom_out_disabled(0.7) && zoom_out_disabled(0.6), "from 0.6 the step gives 0.5");
        assert!(zoom_in_disabled(1.5) && zoom_out_disabled(0.5));
    }

    #[test]
    fn zoom_walks_without_float_drift() {
        let mut s = ZOOM_DEFAULT;
        for _ in 0..4 {
            assert!(!zoom_in_disabled(s));
            s = zoom_in(s);
        }
        assert_eq!(s, 1.4);
        assert!(zoom_in_disabled(s));
        let mut s = ZOOM_DEFAULT;
        for _ in 0..4 {
            assert!(!zoom_out_disabled(s));
            s = zoom_out(s);
        }
        assert_eq!(s, 0.6);
        assert!(zoom_out_disabled(s));
    }

    #[test]
    fn dimension_style_table() {
        assert_eq!(dimension_style(Some("250"), None).as_deref(), Some("width: 250px"));
        assert_eq!(dimension_style(Some("250"), Some("100")).as_deref(), Some("height: 100px; width: 250px"));
        assert_eq!(dimension_style(Some("50%"), Some("10rem")).as_deref(), Some("height: 10rem; width: 50%"));
        assert_eq!(dimension_style(None, None), None);
        assert_eq!(dimension_style(Some(" "), Some("")), None);
    }

    #[test]
    fn transform_and_mask() {
        assert_eq!(transform(90, 1.1), "rotate(90deg) scale(1.1)");
        assert_eq!(preview_style(-180, 1.0), "transform: rotate(-180deg) scale(1)");
        assert!(mask_should_close(false) && !mask_should_close(true));
        assert_eq!(ImageLoading::Lazy.as_str(), "lazy");
        assert_eq!(ImageLoading::Eager.as_str(), "eager");
    }
}
