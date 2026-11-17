//! Focus trap (port of the `pFocusTrap` directive) and the search for focusable elements.

use crate::dom::document;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{Element, FocusEvent, HtmlElement};

const EXCLUDE: &str =
    r#":not([tabindex="-1"]):not([disabled]):not([hidden]):not(.p-hidden-focusable):not([data-p-hidden-focusable="true"])"#;
const FOCUSABLE: [&str; 7] = [
    "button",
    r#"input:not([type="hidden"])"#,
    "select",
    "textarea",
    "a[href]",
    "[tabindex]",
    r#"[contenteditable]:not([contenteditable="false"])"#,
];

fn selector() -> String {
    FOCUSABLE.iter().map(|s| format!("{s}{EXCLUDE}")).collect::<Vec<_>>().join(", ")
}

fn focusables(container: &Element) -> Vec<HtmlElement> {
    let Ok(list) = container.query_selector_all(&selector()) else { return Vec::new() };
    (0..list.length()).filter_map(|i| list.item(i)).filter_map(|n| n.dyn_into::<HtmlElement>().ok()).collect()
}

/// First focusable element inside the container.
pub fn first_focusable(container: &Element) -> Option<HtmlElement> {
    focusables(container).into_iter().next()
}

/// Last focusable element inside the container.
pub fn last_focusable(container: &Element) -> Option<HtmlElement> {
    focusables(container).pop()
}

fn sentinel(doc: &web_sys::Document, section: &str) -> Option<HtmlElement> {
    let el = doc.create_element("span").ok()?.dyn_into::<HtmlElement>().ok()?;
    for (k, v) in [
        ("class", "p-hidden-accessible p-hidden-focusable"),
        ("tabindex", "0"),
        ("role", "presentation"),
        ("aria-hidden", "true"),
        ("data-p-hidden-accessible", "true"),
        ("data-p-hidden-focusable", "true"),
        ("data-pc-section", section),
    ] {
        el.set_attribute(k, v).ok()?;
    }
    Some(el)
}

fn focus_el(el: Option<HtmlElement>) {
    if let Some(el) = el {
        let _ = el.focus();
    }
}

/// Active focus trap: Tab on the last element moves the focus to the first one, and vice versa.
///
/// On `Drop` (or [`FocusTrap::detach`]) the service elements and handlers are removed.
pub struct FocusTrap {
    first: HtmlElement,
    last: HtmlElement,
    _on_first: Closure<dyn FnMut(FocusEvent)>,
    _on_last: Closure<dyn FnMut(FocusEvent)>,
}

impl FocusTrap {
    /// Enables the trap in `container`. Outside the browser returns `None`.
    pub fn attach(container: &HtmlElement) -> Option<Self> {
        let doc = document()?;
        let first = sentinel(&doc, "firstfocusableelement")?;
        let last = sentinel(&doc, "lastfocusableelement")?;

        let on_first = {
            let (container, last) = (container.clone(), last.clone());
            Closure::<dyn FnMut(FocusEvent)>::new(move |e: FocusEvent| {
                let related = e.related_target().and_then(|t| t.dyn_into::<web_sys::Node>().ok());
                let from_last = related.as_ref().is_some_and(|r| r.is_same_node(Some(&last)));
                let outside = !related.as_ref().is_some_and(|r| container.contains(Some(r)));
                if from_last || outside { focus_el(first_focusable(&container)) } else { focus_el(Some(last.clone())) }
            })
        };
        let on_last = {
            let (container, first) = (container.clone(), first.clone());
            Closure::<dyn FnMut(FocusEvent)>::new(move |e: FocusEvent| {
                let related = e.related_target().and_then(|t| t.dyn_into::<web_sys::Node>().ok());
                let from_first = related.as_ref().is_some_and(|r| r.is_same_node(Some(&first)));
                let outside = !related.as_ref().is_some_and(|r| container.contains(Some(r)));
                if from_first || outside { focus_el(last_focusable(&container)) } else { focus_el(Some(first.clone())) }
            })
        };

        first.add_event_listener_with_callback("focus", on_first.as_ref().unchecked_ref()).ok()?;
        last.add_event_listener_with_callback("focus", on_last.as_ref().unchecked_ref()).ok()?;
        container.prepend_with_node_1(&first).ok()?;
        container.append_with_node_1(&last).ok()?;
        Some(Self { first, last, _on_first: on_first, _on_last: on_last })
    }

    /// First service element (for tests and debugging).
    pub fn first_sentinel(&self) -> &HtmlElement {
        &self.first
    }

    /// Last service element (for tests and debugging).
    pub fn last_sentinel(&self) -> &HtmlElement {
        &self.last
    }

    /// Disables the trap.
    pub fn detach(self) {}
}

impl Drop for FocusTrap {
    fn drop(&mut self) {
        self.first.remove();
        self.last.remove();
    }
}
