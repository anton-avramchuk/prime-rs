//! Popover — port of `p-popover` from PrimeNG 21.1.10.
//!
//! A floating panel with arbitrary content next to a trigger element. The panel is controlled by [`PopoverRef`]
//! (the counterpart of `op.toggle($event)` in PrimeNG): create it in advance, pass it to [`Popover`] and call it from handlers.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_popover::{Popover, PopoverRef};
//!
//! let html = view! {
//!     <Popover popover_ref=PopoverRef::new()>
//!         <p>"Body"</p>
//!     </Popover>
//! }
//! .to_html();
//! // while the panel is not shown, it is not in the markup
//! assert!(!html.contains("p-popover") && !html.contains("Body"));
//! ```

mod state;

use leptos::prelude::*;
use prime_core::{OptProp, Prop, Pt};
use std::sync::Arc;
use wasm_bindgen::JsCast;

pub use state::{Retarget, arrow_left, is_flipped, outside_click, retarget, root_classes};

#[cfg(target_arch = "wasm32")]
use leptos::leptos_dom::helpers::set_timeout_with_handle;
#[cfg(target_arch = "wasm32")]
use prime_core::{ZGroup, use_prime_config};
#[cfg(target_arch = "wasm32")]
use prime_dom::dom::{AlignMode, align_overlay};
#[cfg(target_arch = "wasm32")]
use std::time::Duration;

/// Pass-through of the [`Popover`] parts.
#[derive(Clone, Debug, Default)]
pub struct PopoverPt {
    /// Panel root.
    pub root: Pt,
    /// Content.
    pub content: Pt,
}

/// Panel control object: `toggle`/`show`/`hide` from an event and a target.
#[derive(Clone, Copy)]
pub struct PopoverRef {
    visible: RwSignal<bool>,
    target: StoredValue<Option<web_sys::HtmlElement>, LocalStorage>,
    pending: StoredValue<Option<web_sys::HtmlElement>, LocalStorage>,
}

impl Default for PopoverRef {
    fn default() -> Self {
        Self::new()
    }
}

/// Default target: the event's `currentTarget`, or `target` if there is none.
fn event_target(event: &web_sys::Event) -> Option<web_sys::HtmlElement> {
    event
        .current_target()
        .and_then(|t| t.dyn_into::<web_sys::HtmlElement>().ok())
        .or_else(|| event.target().and_then(|t| t.dyn_into::<web_sys::HtmlElement>().ok()))
}

impl PopoverRef {
    /// Creates a hidden panel. Call inside a component (the signal owner).
    pub fn new() -> Self {
        Self { visible: RwSignal::new(false), target: StoredValue::new_local(None), pending: StoredValue::new_local(None) }
    }

    /// The panel is currently shown (subscribes).
    pub fn visible(&self) -> bool {
        self.visible.get()
    }

    /// Shows the panel at the event's `currentTarget`.
    pub fn show(&self, event: &web_sys::Event) {
        if let Some(target) = event_target(event) {
            self.show_target(target);
        }
    }

    /// Shows the panel at an explicit target.
    pub fn show_at(&self, target: &web_sys::HtmlElement) {
        self.show_target(target.clone());
    }

    /// Toggles the panel; the target is the event's `currentTarget`.
    pub fn toggle(&self, event: &web_sys::Event) {
        if let Some(target) = event_target(event) {
            self.toggle_target(target);
        }
    }

    /// Toggles the panel, aligning it to an explicit target.
    pub fn toggle_at(&self, target: &web_sys::HtmlElement) {
        self.toggle_target(target.clone());
    }

    /// Hides the panel.
    pub fn hide(&self) {
        self.pending.set_value(None);
        self.visible.try_set(false);
    }

    fn show_target(&self, target: web_sys::HtmlElement) {
        self.target.set_value(Some(target));
        self.visible.set(true);
    }

    fn toggle_target(&self, target: web_sys::HtmlElement) {
        let visible = self.visible.get_untracked();
        let same = self.target.with_value(|t| t.as_ref().is_some_and(|t| t.is_same_node(Some(&target))));
        match retarget(visible, same) {
            Retarget::Show => self.show_target(target),
            Retarget::Hide => self.hide(),
            Retarget::Switch => {
                // Hide at the old target and show at the new one on the next tick: otherwise the panel is not recreated.
                self.visible.set(false);
                self.pending.set_value(Some(target));
                let this = *self;
                set_timeout(
                    move || {
                        if this.visible.is_disposed() {
                            return;
                        }
                        if let Some(target) = this.pending.try_update_value(Option::take).flatten() {
                            this.show_target(target);
                        }
                    },
                    std::time::Duration::ZERO,
                );
            }
        }
    }
}

/// State shared by the window handlers.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
struct Core {
    r: PopoverRef,
    panel: NodeRef<leptos::html::Div>,
    dismissable: Prop<bool>,
    focus_on_show: Prop<bool>,
    auto_z_index: Prop<bool>,
    base_z_index: Prop<i32>,
    flipped: RwSignal<bool>,
    cleanups: StoredValue<Vec<Box<dyn FnOnce()>>, LocalStorage>,
    z_index: StoredValue<Option<i32>>,
    was_visible: StoredValue<bool>,
    on_show: Option<Callback<()>>,
    on_hide: Option<Callback<()>>,
}

impl Core {
    /// Removes the window subscriptions and releases the z-index.
    fn release(&self) {
        for cleanup in self.cleanups.try_update_value(std::mem::take).unwrap_or_default() {
            cleanup();
        }
        if let Some(z) = self.z_index.try_update_value(Option::take).flatten() {
            prime_core::zindex::revert(z);
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn is_inside(&self, event_target: Option<web_sys::EventTarget>) -> (bool, bool) {
        let node = event_target.and_then(|t| t.dyn_into::<web_sys::Node>().ok());
        let inside_panel = self.panel.get_untracked().is_some_and(|p| p.contains(node.as_ref()));
        let inside_target = self.r.target.with_value(|t| t.as_ref().is_some_and(|t| t.contains(node.as_ref())));
        (inside_panel, inside_target)
    }

    /// The panel has just been shown: z-index, alignment, arrow, focus, subscriptions, `on_show`.
    #[cfg(target_arch = "wasm32")]
    fn opened(self: &Arc<Self>) {
        let (Some(panel), Some(target)) = (self.panel.get_untracked(), self.r.target.get_value()) else { return };
        let panel: web_sys::HtmlElement = panel.unchecked_into();
        if self.auto_z_index.get_untracked() {
            let base = use_prime_config().z_index.base(ZGroup::Overlay) + self.base_z_index.get_untracked();
            let z = prime_core::zindex::generate(ZGroup::Overlay, base);
            self.z_index.set_value(Some(z));
            let _ = panel.style().set_property("z-index", &z.to_string());
        }
        align_overlay(&panel, &target, AlignMode::Relative, false);

        let (container, anchor) = (panel.get_bounding_client_rect(), target.get_bounding_client_rect());
        let radius = web_sys::window()
            .and_then(|w| w.get_computed_style(&panel).ok().flatten())
            .and_then(|s| s.get_property_value("border-radius").ok())
            .and_then(|v| v.trim_end_matches("px").trim().parse::<f64>().ok())
            .unwrap_or(0.0);
        let _ = panel.style().set_property("--p-popover-arrow-left", &format!("{}px", arrow_left(container.left(), anchor.left(), radius)));
        self.flipped.set(is_flipped(container.top(), anchor.top()));

        if self.focus_on_show.get_untracked()
            && let Ok(Some(focusable)) = panel.query_selector("[autofocus]")
            && let Ok(focusable) = focusable.dyn_into::<web_sys::HtmlElement>()
        {
            set_timeout(
                move || {
                    prime_dom::focus_no_scroll(&focusable);
                },
                Duration::from_millis(5),
            );
        }

        // Subscriptions on the next tick: the click that opened the panel is still bubbling to the window and would close it itself.
        let this = self.clone();
        let handle = set_timeout_with_handle(
            move || {
                if this.r.visible.is_disposed() || !this.r.visible.get_untracked() {
                    return;
                }
                this.bind_window();
            },
            Duration::ZERO,
        );
        if let Ok(handle) = handle {
            self.cleanups.update_value(|c| c.push(Box::new(move || handle.clear())));
        }
        if let Some(cb) = self.on_show {
            cb.run(());
        }
    }

    /// A click outside, Escape, a window resize and a scroll of the target's ancestor hide the panel.
    #[cfg(target_arch = "wasm32")]
    fn bind_window(self: &Arc<Self>) {
        let this = self.clone();
        let click = window_event_listener(leptos::ev::click, move |e: web_sys::MouseEvent| {
            if !this.dismissable.get_untracked() || !this.r.visible.get_untracked() {
                return;
            }
            let (panel, target) = this.is_inside(e.target());
            if outside_click(panel, target) {
                this.r.hide();
            }
        });
        let this = self.clone();
        let escape = window_event_listener(leptos::ev::keydown, move |e: web_sys::KeyboardEvent| {
            if e.key() == "Escape" {
                this.r.hide();
            }
        });
        let this = self.clone();
        let resize = window_event_listener(leptos::ev::resize, move |_| {
            let touch = web_sys::window().is_some_and(|w| w.navigator().max_touch_points() > 0);
            if !touch {
                this.r.hide();
            }
        });
        self.cleanups.update_value(|c| {
            c.push(Box::new(move || click.remove()));
            c.push(Box::new(move || escape.remove()));
            c.push(Box::new(move || resize.remove()));
        });

        // A scroll of the target's ancestor (or a page scroll that moved the target) closes the panel.
        let this = self.clone();
        if let Some(target) = self.r.target.get_value()
            && let Some(unwatch) = prime_dom::watch_scroll(target, move || this.r.hide())
        {
            self.cleanups.update_value(|c| c.push(unwatch));
        }
    }

    /// The panel has just been hidden: subscriptions removed, target forgotten, `on_hide`.
    #[cfg(target_arch = "wasm32")]
    fn closed(&self) {
        self.release();
        self.flipped.set(false);
        if self.r.pending.with_value(Option::is_none) {
            self.r.target.set_value(None);
        }
        if let Some(cb) = self.on_hide {
            cb.run(());
        }
    }
}

/// A floating panel bound to a trigger element.
///
/// Every property accepts a value or a signal. The panel is rendered in its own place in the markup (not in `body`),
/// so for alignment the parent must be `position: relative`, not `overflow: hidden`.
#[component]
pub fn Popover(
    /// Control object: used to show, hide and toggle the panel.
    popover_ref: PopoverRef,
    /// Close the panel on a click outside it and outside the target.
    #[prop(optional, default = Prop::Static(true), into)]
    dismissable: Prop<bool>,
    /// On show, focus the first element with `autofocus`.
    #[prop(optional, default = Prop::Static(true), into)]
    focus_on_show: Prop<bool>,
    /// Assign `z-index` automatically.
    #[prop(optional, default = Prop::Static(true), into)]
    auto_z_index: Prop<bool>,
    /// Base `z-index` on top of the config value.
    #[prop(optional, into)]
    base_z_index: Prop<i32>,
    /// `aria-label` attribute of the panel.
    #[prop(optional, into)]
    aria_label: OptProp<String>,
    /// `aria-labelledby` attribute of the panel.
    #[prop(optional, into)]
    aria_labelledby: OptProp<String>,
    /// `id` attribute of the panel.
    #[prop(optional, into)]
    id: OptProp<String>,
    /// Inline style of the panel.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Extra class of the panel.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Called after the panel is shown and aligned.
    #[prop(optional, into)]
    on_show: Option<Callback<()>>,
    /// Called after the panel is hidden.
    #[prop(optional, into)]
    on_hide: Option<Callback<()>>,
    /// Pass-through of parts.
    #[prop(optional)]
    pt: Option<PopoverPt>,
    /// Panel content.
    children: ChildrenFn,
) -> impl IntoView {
    let pt = Arc::new(pt.unwrap_or_default());
    let flipped = RwSignal::new(false);
    let core = Arc::new(Core {
        r: popover_ref,
        panel: NodeRef::new(),
        dismissable,
        focus_on_show,
        auto_z_index,
        base_z_index,
        flipped,
        cleanups: StoredValue::new_local(Vec::new()),
        z_index: StoredValue::new(None),
        was_visible: StoredValue::new(false),
        on_show,
        on_hide,
    });
    {
        let core = core.clone();
        on_cleanup(move || core.release());
    }
    // Side effects of showing and hiding (position, listeners, focus) — browser only.
    #[cfg(target_arch = "wasm32")]
    {
        let core = core.clone();
        Effect::new(move |_| {
            let visible = core.r.visible.get();
            // The panel lands in the DOM after this effect, so we also watch the reference to it.
            let mounted = core.panel.get().is_some();
            let was = core.was_visible.get_value();
            if visible && mounted && !was {
                core.was_visible.set_value(true);
                core.opened();
            } else if !visible && was {
                core.was_visible.set_value(false);
                core.closed();
            }
        });
    }

    let panel = core.panel;
    let class = {
        let pt = pt.clone();
        Signal::derive(move || {
            let mut cn = String::from(root_classes(flipped.get()));
            style_class.with(|c| {
                if let Some(c) = c.as_deref().filter(|c| !c.is_empty()) {
                    cn.push(' ');
                    cn.push_str(c);
                }
            });
            pt.root.merge_class(&cn)
        })
    };
    let root_style = {
        let pt = pt.clone();
        Signal::derive(move || {
            let user = style.get().unwrap_or_default();
            let merged = pt.root.merge_style_parts(&[&user]);
            (!merged.is_empty()).then_some(merged)
        })
    };
    let visible = popover_ref.visible;
    move || {
        visible.get().then(|| {
            let (id, aria_label, aria_labelledby) = (id.clone(), aria_label.clone(), aria_labelledby.clone());
            let (class, root_style, pt, children) = (class, root_style, pt.clone(), children.clone());
            view! {
                <div
                    node_ref=panel
                    class={class}
                    style={root_style}
                    id={move || id.get()}
                    role="dialog"
                    aria-modal="true"
                    aria-label={move || aria_label.get()}
                    aria-labelledby={move || aria_labelledby.get()}
                    data-p-popover-flipped={move || flipped.get().then_some("true")}
                    {..pt.root.attrs()}
                >
                    <div class={pt.content.merge_class("p-popover-content")} {..pt.content.attrs()}>
                        {children()}
                    </div>
                </div>
            }
        })
    }
}
