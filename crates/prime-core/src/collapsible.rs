//! Shared logic of a collapsible region (Panel, Fieldset): state, identifiers, ARIA attributes and the expand animation.
//!
//! The animation is driven by the theme classes `p-collapsible-enter-active` and `p-collapsible-leave-active` (`grid-template-rows` `0fr` ↔ `1fr`).
//! The final hiding is the inline `display: none` set by [`CollapseState::initial_style`] on the first render
//! and then (only in the browser) by [`bind_region`].

use crate::ids::next_unique;
use leptos::prelude::*;

/// "Collapsed" state: an external two-way signal or its own.
#[derive(Clone, Copy, Debug)]
pub struct CollapseState {
    signal: RwSignal<bool>,
}

impl CollapseState {
    /// Takes the external signal, and without it creates its own with the initial value `initial`.
    ///
    /// ```
    /// use leptos::prelude::*;
    /// use prime_core::collapsible::CollapseState;
    ///
    /// let own = CollapseState::new(None, true);
    /// assert!(own.get_untracked());
    /// assert!(!own.toggle());
    ///
    /// let external = RwSignal::new(false);
    /// let state = CollapseState::new(Some(external), true);
    /// assert!(state.toggle() && external.get_untracked());
    /// ```
    pub fn new(external: Option<RwSignal<bool>>, initial: bool) -> Self {
        Self { signal: external.unwrap_or_else(|| RwSignal::new(initial)) }
    }

    /// Current value with subscription.
    pub fn get(&self) -> bool {
        self.signal.get()
    }

    /// Current value without subscription.
    pub fn get_untracked(&self) -> bool {
        self.signal.get_untracked()
    }

    /// Inverts the state and returns the new `collapsed` value.
    pub fn toggle(&self) -> bool {
        let now = !self.signal.get_untracked();
        self.signal.set(now);
        now
    }

    /// Inline style of the region on the first render: a collapsed region is hidden at once, without animation.
    pub fn initial_style(&self) -> Option<&'static str> {
        hidden_style(self.get_untracked())
    }
}

/// Style of a hidden region.
pub fn hidden_style(collapsed: bool) -> Option<&'static str> {
    collapsed.then_some("display: none")
}

/// Identifiers of the parts: header, content region and header row.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CollapseIds {
    /// Base: the root `id`.
    pub root: String,
    /// `{id}_header`: the header, pointed to by `aria-labelledby`.
    pub header: String,
    /// `{id}_content`: the content region, pointed to by `aria-controls`.
    pub content: String,
    /// `{id}-titlebar`: the header row.
    pub titlebar: String,
}

impl CollapseIds {
    /// Identifiers derived from `id`. Without `id` the number comes from the shared hydration context (the same on the server and in the browser),
    /// and outside it from the global counter.
    ///
    /// ```
    /// use prime_core::collapsible::CollapseIds;
    ///
    /// let ids = CollapseIds::new(Some("p1"));
    /// assert_eq!((ids.header.as_str(), ids.content.as_str(), ids.titlebar.as_str()), ("p1_header", "p1_content", "p1-titlebar"));
    /// ```
    pub fn new(id: Option<&str>) -> Self {
        let root = match id {
            Some(id) => id.to_owned(),
            None => format!("pn_id_{}", next_unique()),
        };
        Self { header: format!("{root}_header"), content: format!("{root}_content"), titlebar: format!("{root}-titlebar"), root }
    }
}

/// `aria-expanded` of the toggle.
pub fn aria_expanded(collapsed: bool) -> &'static str {
    if collapsed { "false" } else { "true" }
}

/// `aria-hidden` of the region.
pub fn aria_hidden(collapsed: bool) -> &'static str {
    if collapsed { "true" } else { "false" }
}

/// `tabindex` of the region itself: `-1` only when it is collapsed.
pub fn region_tabindex(collapsed: bool) -> Option<&'static str> {
    collapsed.then_some("-1")
}

/// Animation class for expanding or collapsing.
pub fn animation_class(expanding: bool) -> &'static str {
    if expanding { "p-collapsible-enter-active" } else { "p-collapsible-leave-active" }
}

/// Binds the region to the state (browser only): `tabindex` of focusable descendants and animation on state change.
/// The first run animates nothing. `on_done` is called with the new `collapsed` after the animation ends.
#[cfg(target_arch = "wasm32")]
pub fn bind_region(region: NodeRef<leptos::html::Div>, state: CollapseState, on_done: Option<Callback<bool>>) {
    bind_region_with(region, move || state.get(), on_done);
}

/// Same as [`bind_region`], but the "collapsed" state is computed by a function (for example, from the list of active values of a group).
/// The function reads signals inside an `Effect`, so a change of any of them restarts the binding.
#[cfg(target_arch = "wasm32")]
pub fn bind_region_with(region: NodeRef<leptos::html::Div>, collapsed: impl Fn() -> bool + 'static, on_done: Option<Callback<bool>>) {
    Effect::new(move |prev: Option<bool>| {
        let collapsed = collapsed();
        let Some(el) = region.get() else { return collapsed };
        let el: web_sys::HtmlElement = (*el).clone();
        match prev {
            None => {
                if collapsed {
                    set_descendants_tabindex(&el, true);
                }
            }
            Some(p) if p == collapsed => {}
            Some(_) => {
                set_descendants_tabindex(&el, collapsed);
                animate(&el, !collapsed, move || {
                    if let Some(cb) = on_done {
                        cb.run(collapsed);
                    }
                });
            }
        }
        collapsed
    });
}

/// Sets or removes `tabindex="-1"` on focusable descendants of the region.
#[cfg(target_arch = "wasm32")]
fn set_descendants_tabindex(region: &web_sys::HtmlElement, collapsed: bool) {
    let Ok(list) = region.query_selector_all("input, button, select, a, textarea, [tabindex]") else { return };
    for i in 0..list.length() {
        let Some(node) = list.item(i) else { continue };
        let el: web_sys::Element = leptos::wasm_bindgen::JsCast::unchecked_into(node);
        let _ = if collapsed { el.set_attribute("tabindex", "-1") } else { el.remove_attribute("tabindex") };
    }
}

/// Starts the region animation and calls `done` once on `animationend`; if there is no event (disabled animations,
/// hidden element), finishes via a fallback timer.
#[cfg(target_arch = "wasm32")]
pub fn animate(el: &web_sys::HtmlElement, expanding: bool, done: impl FnOnce() + 'static) {
    use leptos::wasm_bindgen::{JsCast, closure::Closure};
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::time::Duration;

    // Each run gets its own number: a stale run (fast repeated toggling) changes nothing on completion,
    // otherwise its fallback timer would hide an already expanded region.
    thread_local! {
        static RUN: std::cell::Cell<u32> = const { std::cell::Cell::new(0) };
    }
    let run = RUN.with(|c| {
        c.set(c.get().wrapping_add(1));
        c.get().to_string()
    });
    let _ = el.set_attribute("data-p-collapse-run", &run);
    let class = animation_class(expanding);
    let _ = el.class_list().remove_2(animation_class(true), animation_class(false));
    let _ = el.style().remove_property("display");
    let _ = el.class_list().add_1(class);

    type Done = Rc<RefCell<Option<Box<dyn FnOnce()>>>>;
    let done: Done = Rc::new(RefCell::new(Some(Box::new(done))));
    let finish: Rc<dyn Fn()> = {
        let (el, run) = (el.clone(), run.clone());
        Rc::new(move || {
            let Some(done) = done.borrow_mut().take() else { return };
            if el.get_attribute("data-p-collapse-run").as_deref() != Some(run.as_str()) {
                return;
            }
            let _ = el.class_list().remove_1(class);
            if !expanding {
                let _ = el.style().set_property("display", "none");
            }
            done();
        })
    };
    let on_end = {
        let finish = finish.clone();
        Closure::once_into_js(move || finish())
    };
    let _ = el.add_event_listener_with_callback("animationend", on_end.unchecked_ref());
    set_timeout(move || finish(), Duration::from_millis(400));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attributes_follow_state() {
        assert_eq!((aria_expanded(false), aria_hidden(false), region_tabindex(false)), ("true", "false", None));
        assert_eq!((aria_expanded(true), aria_hidden(true), region_tabindex(true)), ("false", "true", Some("-1")));
        assert_eq!(hidden_style(true), Some("display: none"));
        assert_eq!(hidden_style(false), None);
        assert_eq!(animation_class(true), "p-collapsible-enter-active");
        assert_eq!(animation_class(false), "p-collapsible-leave-active");
    }

    #[test]
    fn ids_are_derived_and_unique() {
        let a = CollapseIds::new(Some("x"));
        assert_eq!(
            (a.root.as_str(), a.header.as_str(), a.content.as_str(), a.titlebar.as_str()),
            ("x", "x_header", "x_content", "x-titlebar")
        );
        let (b, c) = (CollapseIds::new(None), CollapseIds::new(None));
        assert_ne!(b.root, c.root);
        assert!(b.root.starts_with("pn_id_"));
    }

    #[test]
    fn state_toggles_own_and_external() {
        let owner = Owner::new();
        owner.with(|| {
            let own = CollapseState::new(None, true);
            assert_eq!((own.get_untracked(), own.toggle(), own.get_untracked()), (true, false, false));
            let ext = RwSignal::new(false);
            let s = CollapseState::new(Some(ext), true);
            assert!(!s.get_untracked(), "the external signal takes precedence over the initial value");
            assert!(s.toggle() && ext.get_untracked());
            assert_eq!(s.initial_style(), Some("display: none"));
        });
    }
}
