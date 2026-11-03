//! Чистая логика Image: классы, границы и шаги масштаба, `transform` (по `image.ts` и `imagestyle.ts` PrimeNG).

use prime_core::ClassNames;
use std::borrow::Cow;

/// Шаг масштаба.
pub const ZOOM_STEP: f64 = 0.1;
/// Наименьший масштаб.
pub const ZOOM_MIN: f64 = 0.5;
/// Наибольший масштаб.
pub const ZOOM_MAX: f64 = 1.5;
/// Масштаб по умолчанию.
pub const ZOOM_DEFAULT: f64 = 1.0;
/// Допуск сравнения дробных масштабов.
const EPS: f64 = 1e-9;

/// Значение `loading` картинки.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImageLoading {
    /// Отложенная загрузка.
    Lazy,
    /// Немедленная загрузка.
    Eager,
}

impl ImageLoading {
    /// Значение атрибута.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Lazy => "lazy",
            Self::Eager => "eager",
        }
    }
}

/// Кнопка панели действий.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActionKind {
    /// Поворот вправо.
    RotateRight,
    /// Поворот влево.
    RotateLeft,
    /// Уменьшение.
    ZoomOut,
    /// Увеличение.
    ZoomIn,
    /// Закрытие.
    Close,
}

/// Классы корня: `p-image p-component`, `p-image-preview` и пользовательский класс.
pub fn root_classes(preview: bool, extra: Option<&str>) -> Cow<'static, str> {
    let mut cn = ClassNames::new();
    cn.add("p-image p-component").add_if(preview, "p-image-preview").add_user(extra);
    cn.finish_cow()
}

/// Классы кнопки панели; у уменьшения и увеличения — `p-disabled`, когда шаг невозможен.
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

/// Нельзя ли уменьшить: шаг не оставляет масштаб выше наименьшего (`scale - step <= min`).
pub fn zoom_out_disabled(scale: f64) -> bool {
    scale - ZOOM_STEP <= ZOOM_MIN + EPS
}

/// Нельзя ли увеличить: шаг не оставляет масштаб ниже наибольшего (`scale + step >= max`).
pub fn zoom_in_disabled(scale: f64) -> bool {
    scale + ZOOM_STEP >= ZOOM_MAX - EPS
}

/// Масштаб после шага вверх (округлён до сотых).
pub fn zoom_in(scale: f64) -> f64 {
    round2(scale + ZOOM_STEP)
}

/// Масштаб после шага вниз (округлён до сотых).
pub fn zoom_out(scale: f64) -> f64 {
    round2(scale - ZOOM_STEP)
}

fn round2(v: f64) -> f64 {
    (v * 100.0).round() / 100.0
}

/// Значение `transform` увеличенной картинки.
pub fn transform(rotate: i32, scale: f64) -> String {
    format!("rotate({rotate}deg) scale({scale})")
}

/// Стиль `transform` увеличенной картинки.
pub fn preview_style(rotate: i32, scale: f64) -> String {
    format!("transform: {}", transform(rotate, scale))
}

/// Размер кнопки просмотра по `width` и `height` картинки: числа получают `px` (в PrimeNG — `height + 'px'`), прочее
/// (`50%`, `10rem`) берётся как есть; без размеров стиля нет.
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

/// Закрывает ли клик по маске просмотр (не после действия панели или клика по картинке).
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
        assert!(!zoom_in_disabled(1.3) && zoom_in_disabled(1.4), "из 1.4 шаг даст 1.5");
        assert!(!zoom_out_disabled(0.7) && zoom_out_disabled(0.6), "из 0.6 шаг даст 0.5");
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
