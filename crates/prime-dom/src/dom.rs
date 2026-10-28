//! Обёртки над DOM для оверлеев. На сервере и вне браузера все функции ничего не делают.

use crate::position::{Input, Placement, Point, Size, absolute_position, relative_position};
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlElement, Window};

/// Окно браузера; `None` на сервере (SSR) и в нативных тестах, где вызов web-sys привёл бы к панике.
pub fn window() -> Option<Window> {
    if cfg!(target_arch = "wasm32") { web_sys::window() } else { None }
}

/// Документ браузера; `None` вне браузера.
pub fn document() -> Option<Document> {
    window().and_then(|w| w.document())
}

/// Размер видимой области окна (как `DomHandler.getViewport`).
pub fn viewport() -> Size {
    let Some(win) = window() else { return Size::default() };
    let inner = |v: Result<wasm_bindgen::JsValue, _>| v.ok().and_then(|v| v.as_f64()).unwrap_or(0.0);
    let (mut width, mut height) = (inner(win.inner_width()), inner(win.inner_height()));
    if let Some(el) = win.document().and_then(|d| d.document_element()) {
        if width == 0.0 {
            width = f64::from(el.client_width());
        }
        if height == 0.0 {
            height = f64::from(el.client_height());
        }
    }
    Size { width, height }
}

/// Прокрутка окна.
pub fn scroll() -> Point {
    let Some(win) = window() else { return Point::default() };
    Point { top: win.page_y_offset().unwrap_or(0.0), left: win.page_x_offset().unwrap_or(0.0) }
}

/// Размер элемента; для скрытого (`offsetParent == null`) временно показывает его вне экрана и измеряет.
pub fn element_size(el: &HtmlElement) -> Size {
    if el.offset_parent().is_some() {
        return Size { width: f64::from(el.offset_width()), height: f64::from(el.offset_height()) };
    }
    let style = el.style();
    let (visibility, display) =
        (style.get_property_value("visibility").unwrap_or_default(), style.get_property_value("display").unwrap_or_default());
    let _ = style.set_property("visibility", "hidden");
    let _ = style.set_property("display", "block");
    let size = Size { width: f64::from(el.offset_width()), height: f64::from(el.offset_height()) };
    let _ = style.set_property("display", &display);
    let _ = style.set_property("visibility", &visibility);
    size
}

/// Режим позиционирования.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlignMode {
    /// Оверлей лежит внутри компонента (`appendTo = 'self'`).
    Relative,
    /// Оверлей добавлен в `body` (`appendTo = 'body'`).
    Absolute,
}

fn closest_relative(el: &HtmlElement) -> Option<web_sys::Element> {
    let win = window()?;
    let mut cur = el.parent_element();
    while let Some(e) = cur {
        let position = win.get_computed_style(&e).ok().flatten().and_then(|s| s.get_property_value("position").ok());
        if position.as_deref() == Some("relative") {
            return Some(e);
        }
        cur = e.parent_element();
    }
    None
}

fn measure(overlay: &HtmlElement, target: &HtmlElement) -> Input {
    let rect = target.get_bounding_client_rect();
    Input {
        target_offset: Point { top: rect.top(), left: rect.left() },
        target: Size { width: f64::from(target.offset_width()), height: f64::from(target.offset_height()) },
        overlay: element_size(overlay),
        viewport: viewport(),
        scroll: scroll(),
    }
}

/// Позиционирует `overlay` относительно `target` и записывает `top`, `left`, `transform-origin` в его стиль.
///
/// Возвращает рассчитанную позицию; вне браузера ничего не делает и возвращает `None`.
pub fn align_overlay(overlay: &HtmlElement, target: &HtmlElement, mode: AlignMode, calculate_min_width: bool) -> Option<Placement> {
    window()?;
    let style = overlay.style();
    if calculate_min_width {
        let _ = style.set_property("min-width", &format!("{}px", target.offset_width()));
    }
    let input = measure(overlay, target);
    let placement = match mode {
        AlignMode::Absolute => absolute_position(&input),
        AlignMode::Relative => {
            let offset = closest_relative(overlay).map(|e| {
                let r = e.get_bounding_client_rect();
                Point { top: r.top(), left: r.left() }
            });
            relative_position(&input, offset)
        }
    };
    let _ = style.set_property("top", &format!("{}px", placement.top));
    let _ = style.set_property("left", &format!("{}px", placement.left));
    let _ = style.set_property("transform-origin", placement.origin.as_str());
    Some(placement)
}

/// Приводит `Element` к `HtmlElement`.
pub fn as_html(el: &web_sys::Element) -> Option<HtmlElement> {
    el.clone().dyn_into::<HtmlElement>().ok()
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod ssr_tests {
    use super::*;

    #[test]
    fn no_browser_means_no_panic_and_neutral_values() {
        assert!(window().is_none());
        assert!(document().is_none());
        assert_eq!(viewport(), Size::default());
        assert_eq!(scroll(), Point::default());
    }
}
