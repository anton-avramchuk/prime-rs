//! Closing popup panels on scroll.
//!
//! `scroll` does not bubble, so the panel listens to it in the capture phase on the window. Scrolling an ancestor of the target always closes the panel, and
//! scrolling the document itself only if the target has actually moved in the window: otherwise page shifts the user did not make (inserting the panel into the flow,
//! scrolling to a focused element) would cause false triggers.

/// Movement tolerance of the target, px.
pub const MOVE_TOLERANCE: f64 = 1.0;

/// Whether the target has moved relative to the `origin` position (top, left) at opening.
pub fn target_moved(origin: (f64, f64), now: (f64, f64)) -> bool {
    (now.0 - origin.0).abs() > MOVE_TOLERANCE || (now.1 - origin.1).abs() > MOVE_TOLERANCE
}

/// Whether to hide the panel: the document is scrolled (hide only if the target has moved) or another element (hide if it contains the target).
pub fn scroll_hides(scrolled_is_document: bool, scrolled_contains_target: bool, target_moved: bool) -> bool {
    if scrolled_is_document { target_moved } else { scrolled_contains_target }
}

/// Focus without scrolling the page: otherwise the browser scrolls to the element and shifts the target, and the popup panel would close.
#[cfg(target_arch = "wasm32")]
pub fn focus_no_scroll(el: &web_sys::HtmlElement) {
    let options = web_sys::FocusOptions::new();
    options.set_prevent_scroll(true);
    let _ = el.focus_with_options(&options);
}

/// Focus without scrolling (does nothing outside the browser).
#[cfg(not(target_arch = "wasm32"))]
pub fn focus_no_scroll(_el: &()) {}

/// Scroll subscription: calls `on_hide` when the panel at `target` should close. Returns an unsubscribe function; outside the
/// browser it does nothing and returns `None`.
#[cfg(target_arch = "wasm32")]
pub fn watch_scroll(target: web_sys::HtmlElement, on_hide: impl Fn() + 'static) -> Option<Box<dyn FnOnce()>> {
    use wasm_bindgen::JsCast;
    let window = web_sys::window()?;
    let rect = target.get_bounding_client_rect();
    let origin = (rect.top(), rect.left());
    let closure = wasm_bindgen::closure::Closure::<dyn FnMut(web_sys::Event)>::new(move |e: web_sys::Event| {
        let scrolled = e.target().and_then(|t| t.dyn_into::<web_sys::Node>().ok());
        let is_document = scrolled.as_ref().is_some_and(|n| n.node_type() == web_sys::Node::DOCUMENT_NODE);
        let contains = !is_document && scrolled.as_ref().is_some_and(|s| s.contains(Some(&target)));
        let now = target.get_bounding_client_rect();
        if scroll_hides(is_document, contains, target_moved(origin, (now.top(), now.left()))) {
            on_hide();
        }
    });
    window.add_event_listener_with_callback_and_bool("scroll", closure.as_ref().unchecked_ref(), true).ok()?;
    Some(Box::new(move || {
        let _ = window.remove_event_listener_with_callback_and_bool("scroll", closure.as_ref().unchecked_ref(), true);
    }))
}

/// Scroll subscription (not available outside the browser).
#[cfg(not(target_arch = "wasm32"))]
pub fn watch_scroll(_target: (), _on_hide: impl Fn() + 'static) -> Option<Box<dyn FnOnce()>> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn target_moved_has_a_tolerance() {
        assert!(!target_moved((10.0, 20.0), (10.0, 20.0)));
        assert!(!target_moved((10.0, 20.0), (10.9, 19.1)));
        assert!(target_moved((10.0, 20.0), (12.0, 20.0)));
        assert!(target_moved((10.0, 20.0), (10.0, 18.5)));
    }

    #[test]
    fn scroll_hides_table() {
        // (document, contains the target, the target moved) → hide
        let table = [
            (true, false, false, false),
            (true, false, true, true),
            (true, true, false, false),
            (false, true, false, true),
            (false, true, true, true),
            (false, false, true, false),
            (false, false, false, false),
        ];
        for (doc, contains, moved, want) in table {
            assert_eq!(scroll_hides(doc, contains, moved), want, "{doc} {contains} {moved}");
        }
    }
}
