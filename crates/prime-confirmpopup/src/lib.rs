//! ConfirmPopup: a port of `p-confirmpopup` from PrimeNG 21.1.10.
//!
//! A popup confirmation next to the element that triggered the action. The application sends a [`Confirmation`] to the shared
//! [`ConfirmationService`] (kept in the context), and the component shows the confirmations whose `key` matches its own:
//! a `div.p-confirmpopup[role=alertdialog]` panel at `target` with text, an icon and "No" and "Yes" buttons. The panel is rendered into
//! `body` and appears only in the browser.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_confirmation::provide_confirmation_service;
//! use prime_confirmpopup::ConfirmPopup;
//!
//! Owner::new().with(|| {
//!     provide_confirmation_service();
//!     let html = view! { <ConfirmPopup /> }.to_html();
//!     // while there is no confirmation, the panel is absent from the markup
//!     assert!(!html.contains("p-confirmpopup"));
//! });
//! ```

mod state;

use leptos::either::Either;
use leptos::prelude::*;
use prime_button::Button;
use prime_confirmation::{ConfirmButtonProps, ConfirmEvent, ConfirmFocus, Confirmation, use_confirmation_service};
use prime_dom::Portal;
use prime_rs_core::props::Size;
use prime_rs_core::{OptProp, Prop, Pt};
use std::sync::Arc;

pub use state::{accepts_key, arrow_left, button_label, focus_target, is_flipped, outside_click, root_classes};

#[cfg(target_arch = "wasm32")]
use prime_dom::{AlignMode, align_overlay};
#[cfg(target_arch = "wasm32")]
use prime_rs_core::{ZGroup, use_prime_config};
#[cfg(target_arch = "wasm32")]
use std::time::Duration;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

/// Pass-through parts of [`ConfirmPopup`].
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ConfirmPopupPt {
    /// Panel root.
    pub root: Pt,
    /// Content block.
    pub content: Pt,
    /// Icon.
    pub icon: Pt,
    /// Question text.
    pub message: Pt,
    /// Footer with the buttons.
    pub footer: Pt,
}

/// Context of the `headless` view.
#[derive(Clone)]
pub struct ConfirmPopupHeadless {
    /// The confirmation being shown.
    pub confirmation: Confirmation,
    /// Accept: calls `accept`, hides the panel and returns focus to the target.
    pub accept: Callback<()>,
    /// Reject: calls `reject`, hides the panel and returns focus to the target.
    pub reject: Callback<()>,
}

/// State shared by the panel handlers.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
struct Core {
    key: OptProp<String>,
    default_focus: Prop<ConfirmFocus>,
    auto_z_index: Prop<bool>,
    base_z_index: Prop<i32>,
    visible_prop: OptProp<bool>,
    /// The confirmation being shown (the panel is built from it).
    confirmation: StoredValue<Option<Confirmation>, LocalStorage>,
    /// Internal show state.
    visible: RwSignal<bool>,
    /// Retargeting: the panel is hidden for one tick and shown again; keep the confirmation.
    retargeting: StoredValue<bool>,
    panel: NodeRef<leptos::html::Div>,
    flipped: RwSignal<bool>,
    cleanups: StoredValue<Vec<Box<dyn FnOnce()>>, LocalStorage>,
    z_index: StoredValue<Option<i32>>,
    was_visible: StoredValue<bool>,
    on_show: Option<Callback<()>>,
    on_hide: Option<Callback<()>>,
}

impl Core {
    /// The panel is shown: the `visible` property, and if it is not set, the internal state.
    fn shown(&self) -> bool {
        self.visible_prop.get().unwrap_or_else(|| self.visible.get())
    }

    /// Window and layer subscriptions are released.
    fn release(&self) {
        for cleanup in self.cleanups.try_update_value(std::mem::take).unwrap_or_default() {
            cleanup();
        }
        if let Some(z) = self.z_index.try_update_value(Option::take).flatten() {
            prime_rs_core::zindex::revert(z);
        }
    }

    fn hide(&self) {
        self.visible.try_set(false);
    }

    /// Service event.
    fn on_event(self: &Arc<Self>, event: ConfirmEvent) {
        match event {
            ConfirmEvent::Close => self.hide(),
            ConfirmEvent::Confirm(confirmation) => {
                if !accepts_key(self.key.get_untracked().as_deref(), confirmation.key.as_deref()) {
                    return;
                }
                self.confirmation.set_value(Some(*confirmation));
                if self.visible.get_untracked() {
                    // The panel is already shown: hide and show again at the new target on the next tick.
                    self.retargeting.set_value(true);
                    self.visible.set(false);
                    let this = self.clone();
                    set_timeout(
                        move || {
                            if this.visible.is_disposed() {
                                return;
                            }
                            this.retargeting.set_value(false);
                            this.visible.set(true);
                        },
                        std::time::Duration::ZERO,
                    );
                } else {
                    self.visible.set(true);
                }
            }
        }
    }

    /// Accept or reject: callback, hide, focus on the target.
    fn answer(&self, accept: bool) {
        let confirmation = self.confirmation.get_value();
        if let Some(cb) = confirmation.as_ref().and_then(|c| if accept { c.accept } else { c.reject }) {
            cb.run(());
        }
        self.hide();
        self.focus_target_element();
    }

    fn focus_target_element(&self) {
        #[cfg(target_arch = "wasm32")]
        if let Some(target) = self.confirmation.get_value().and_then(|c| c.target) {
            let _ = target.focus();
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn is_inside(&self, event_target: Option<web_sys::EventTarget>) -> (bool, bool) {
        let node = event_target.and_then(|t| t.dyn_into::<web_sys::Node>().ok());
        let inside_panel = self.panel.get_untracked().is_some_and(|p| p.contains(node.as_ref()));
        let inside_target = self.confirmation.get_value().and_then(|c| c.target).is_some_and(|t| t.contains(node.as_ref()));
        (inside_panel, inside_target)
    }

    /// The panel has just been shown: layer, alignment, arrow, focus, subscriptions, `on_show`.
    #[cfg(target_arch = "wasm32")]
    fn opened(self: &Arc<Self>) {
        let Some(panel) = self.panel.get_untracked() else { return };
        let panel: web_sys::HtmlElement = panel.unchecked_into();
        let confirmation = self.confirmation.get_value().unwrap_or_default();
        if self.auto_z_index.get_untracked() {
            let base = use_prime_config().z_index.base(ZGroup::Overlay) + self.base_z_index.get_untracked();
            let z = prime_rs_core::zindex::generate(ZGroup::Overlay, base);
            self.z_index.set_value(Some(z));
            let _ = panel.style().set_property("z-index", &z.to_string());
        }
        if let Some(target) = confirmation.target.as_ref() {
            align_overlay(&panel, target, AlignMode::Absolute, false);
            let (container, anchor) = (panel.get_bounding_client_rect(), target.get_bounding_client_rect());
            let _ =
                panel.style().set_property("--p-confirmpopup-arrow-left", &format!("{}px", arrow_left(container.left(), anchor.left())));
            self.flipped.set(is_flipped(container.top(), anchor.top()));
        }

        let wanted = focus_target(confirmation.default_focus, self.default_focus.get_untracked());
        let selector = match wanted {
            ConfirmFocus::Accept => Some(".p-confirmpopup-accept-button"),
            ConfirmFocus::Reject => Some(".p-confirmpopup-reject-button"),
            ConfirmFocus::Close | ConfirmFocus::None => None,
        };
        if let Some(button) =
            selector.and_then(|s| panel.query_selector(s).ok().flatten()).and_then(|b| b.dyn_into::<web_sys::HtmlElement>().ok())
        {
            set_timeout(
                move || {
                    prime_dom::focus_no_scroll(&button);
                },
                Duration::from_millis(5),
            );
        }

        // Subscriptions go on the next tick: the click that called `confirm` is still bubbling to the window and would close the panel itself.
        let this = self.clone();
        let handle = leptos::leptos_dom::helpers::set_timeout_with_handle(
            move || {
                if this.visible.is_disposed() || !this.shown() {
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

    /// Outside click, `Escape`, window resize and scroll of a target ancestor.
    #[cfg(target_arch = "wasm32")]
    fn bind_window(self: &Arc<Self>) {
        let this = self.clone();
        let click = window_event_listener(leptos::ev::click, move |e: web_sys::MouseEvent| {
            let dismissable = this.confirmation.get_value().and_then(|c| c.dismissable_mask) != Some(false);
            if !dismissable || !this.shown() {
                return;
            }
            let (panel, target) = this.is_inside(e.target());
            if outside_click(panel, target) {
                this.hide();
            }
        });
        let this = self.clone();
        let escape = window_event_listener(leptos::ev::keydown, move |e: web_sys::KeyboardEvent| {
            if e.key() != "Escape" || !this.shown() {
                return;
            }
            if this.confirmation.get_value().and_then(|c| c.close_on_escape) != Some(false) {
                this.answer(false);
            }
        });
        let this = self.clone();
        let resize = window_event_listener(leptos::ev::resize, move |_| {
            let touch = web_sys::window().is_some_and(|w| w.navigator().max_touch_points() > 0);
            if !touch {
                this.hide();
            }
        });
        self.cleanups.update_value(|c| {
            c.push(Box::new(move || click.remove()));
            c.push(Box::new(move || escape.remove()));
            c.push(Box::new(move || resize.remove()));
        });

        // Scrolling a target ancestor (or page scrolling that moved the target) closes the panel.
        let this = self.clone();
        if let Some(target) = self.confirmation.get_value().and_then(|c| c.target)
            && let Some(unwatch) = prime_dom::watch_scroll(target, move || this.hide())
        {
            self.cleanups.update_value(|c| c.push(unwatch));
        }
    }

    /// The panel has just been hidden: subscriptions removed, confirmation forgotten (except for retargeting), `on_hide`.
    #[cfg(target_arch = "wasm32")]
    fn closed(&self) {
        self.release();
        self.flipped.set(false);
        if !self.retargeting.get_value() {
            self.confirmation.set_value(None);
        }
        if let Some(cb) = self.on_hide {
            cb.run(());
        }
    }
}

/// Popup confirmation next to the target element.
///
/// Properties accept a value or a signal. Views (`content`, `headless`, icons) are rebuilt on every show.
/// Requires a `ConfirmationService` in the context (`provide_confirmation_service`).
#[component]
pub fn ConfirmPopup(
    /// Key: the panel shows only confirmations with the same key.
    #[prop(optional, into)]
    key: OptProp<String>,
    /// What gets focus on show if the confirmation does not set its own (`Accept` by default).
    #[prop(optional, default = Prop::Static(ConfirmFocus::Accept), into)]
    default_focus: Prop<ConfirmFocus>,
    /// Assign `z-index` automatically.
    #[prop(optional, default = Prop::Static(true), into)]
    auto_z_index: Prop<bool>,
    /// Base `z-index` on top of the configuration value.
    #[prop(optional, into)]
    base_z_index: Prop<i32>,
    /// Controlled show: if set, it determines the show instead of the internal state.
    #[prop(optional, into)]
    visible: OptProp<bool>,
    /// Inline style of the panel.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Additional class of the panel.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Pass-through parts.
    #[prop(optional)]
    pt: Option<ConfirmPopupPt>,
    /// Custom content of the `p-confirmpopup-content` block.
    #[prop(optional)]
    content: Option<Callback<Confirmation, AnyView>>,
    /// Custom content of the whole panel.
    #[prop(optional)]
    headless: Option<Callback<ConfirmPopupHeadless, AnyView>>,
    /// Icon of the accept button.
    #[prop(optional, into)]
    accept_icon: Option<ViewFn>,
    /// Icon of the reject button.
    #[prop(optional, into)]
    reject_icon: Option<ViewFn>,
    /// Called after the panel is shown and aligned.
    #[prop(optional)]
    on_show: Option<Callback<()>>,
    /// Called after the panel is hidden.
    #[prop(optional)]
    on_hide: Option<Callback<()>>,
) -> impl IntoView {
    let pt = Arc::new(pt.unwrap_or_default());
    let core = Arc::new(Core {
        key,
        default_focus,
        auto_z_index,
        base_z_index,
        visible_prop: visible,
        confirmation: StoredValue::new_local(None),
        visible: RwSignal::new(false),
        retargeting: StoredValue::new(false),
        panel: NodeRef::new(),
        flipped: RwSignal::new(false),
        cleanups: StoredValue::new_local(Vec::new()),
        z_index: StoredValue::new(None),
        was_visible: StoredValue::new(false),
        on_show,
        on_hide,
    });

    let service = use_confirmation_service();
    let subscription = {
        let core = core.clone();
        service.subscribe(Callback::new(move |event| core.on_event(event)))
    };
    {
        let core = core.clone();
        on_cleanup(move || {
            service.unsubscribe(subscription);
            core.release();
        });
    }
    // Side effects of showing and hiding (position, listeners, focus) happen only in the browser.
    #[cfg(target_arch = "wasm32")]
    {
        let core = core.clone();
        Effect::new(move |_| {
            let shown = core.shown();
            // The panel enters the DOM after this effect, so we also watch the reference to it.
            // After retargeting the reference still holds the old, already removed panel: we need the one attached to the document.
            let mounted = core.panel.get().is_some_and(|p| web_sys::Node::is_connected(&p));
            let was = core.was_visible.get_value();
            if shown && mounted && !was {
                core.was_visible.set_value(true);
                core.opened();
            } else if !shown && was {
                core.was_visible.set_value(false);
                core.closed();
            }
        });
    }

    let parts = Arc::new(Parts { style, style_class, content, headless, accept_icon, reject_icon });
    move || {
        let (core, pt, parts) = (core.clone(), pt.clone(), parts.clone());
        core.shown().then(move || view! { <Portal>{build_panel(&core, &pt, &parts)}</Portal> })
    }
}

/// Properties needed only when rendering the panel.
struct Parts {
    style: OptProp<String>,
    style_class: OptProp<String>,
    content: Option<Callback<Confirmation, AnyView>>,
    headless: Option<Callback<ConfirmPopupHeadless, AnyView>>,
    accept_icon: Option<ViewFn>,
    reject_icon: Option<ViewFn>,
}

/// Footer button taking the confirmation properties into account.
fn footer_button(confirmation: &Confirmation, accept: bool, icon: Option<ViewFn>, core: &Arc<Core>) -> impl IntoView + use<> {
    let props: ConfirmButtonProps =
        if accept { confirmation.accept_button_props.clone() } else { confirmation.reject_button_props.clone() }.unwrap_or_default();
    let (label, default_key) =
        if accept { (confirmation.accept_label.clone(), "accept") } else { (confirmation.reject_label.clone(), "reject") };
    let default = prime_rs_core::use_prime_config().locale.text(default_key).to_owned();
    let label = button_label(props.label.as_deref(), label.as_deref(), &default);
    let extra = if accept { confirmation.accept_button_style_class.clone() } else { confirmation.reject_button_style_class.clone() };
    let base = if accept { "p-confirmpopup-accept-button" } else { "p-confirmpopup-reject-button" };
    let class = match extra.filter(|e| !e.is_empty()) {
        Some(extra) => format!("{base} {extra}"),
        None => base.to_owned(),
    };
    let icon = if accept { confirmation.accept_icon.clone() } else { confirmation.reject_icon.clone() }.or(icon);
    let core = core.clone();
    let on_click = Callback::new(move |_: web_sys::MouseEvent| core.answer(accept));
    let (size, severity, outlined, text) =
        (props.size.unwrap_or(Size::Small), props.severity, props.outlined.unwrap_or(false), props.text.unwrap_or(false));
    match icon {
        Some(icon) => Either::Left(view! {
            <Button label=label.clone() aria_label=label size=size severity=severity outlined=outlined text=text style_class=class icon=icon on_click=on_click />
        }),
        None => Either::Right(view! {
            <Button label=label.clone() aria_label=label size=size severity=severity outlined=outlined text=text style_class=class on_click=on_click />
        }),
    }
}

/// Panel with content and buttons.
fn build_panel(core: &Arc<Core>, pt: &Arc<ConfirmPopupPt>, parts: &Arc<Parts>) -> impl IntoView + use<> {
    let confirmation = core.confirmation.get_value().unwrap_or_default();
    let flipped = core.flipped;
    let class = {
        let (pt, parts) = (pt.clone(), parts.clone());
        move || {
            let extra = parts.style_class.with(|c| c.clone());
            let base = root_classes(flipped.get(), extra.as_deref());
            pt.root.merge_class(&base).into_owned()
        }
    };
    let style = {
        let (pt, parts) = (pt.clone(), parts.clone());
        move || {
            let own = parts.style.with(|s| s.clone()).unwrap_or_default();
            let merged = pt.root.merge_style(&own);
            (!merged.is_empty()).then_some(merged)
        }
    };
    let body = match parts.headless {
        Some(headless) => {
            let (accept, reject) = (
                {
                    let core = core.clone();
                    Callback::new(move |()| core.answer(true))
                },
                {
                    let core = core.clone();
                    Callback::new(move |()| core.answer(false))
                },
            );
            Either::Left(headless.run(ConfirmPopupHeadless { confirmation: confirmation.clone(), accept, reject }))
        }
        None => {
            let inner = match parts.content {
                Some(custom) => custom.run(confirmation.clone()).into_any(),
                None => {
                    let icon = confirmation.icon.clone().map(|icon| {
                        let class = pt.icon.merge_class("p-confirmpopup-icon").into_owned();
                        view! { <span class={class} {..pt.icon.attrs()}>{icon.run()}</span> }
                    });
                    let message = confirmation.message.clone().unwrap_or_default();
                    let message_class = pt.message.merge_class("p-confirmpopup-message").into_owned();
                    view! { {icon}<span class={message_class} {..pt.message.attrs()}>{message}</span> }.into_any()
                }
            };
            let reject = confirmation.reject_visible().then(|| footer_button(&confirmation, false, parts.reject_icon.clone(), core));
            let accept = confirmation.accept_visible().then(|| footer_button(&confirmation, true, parts.accept_icon.clone(), core));
            Either::Right(view! {
                <div class={pt.content.merge_class("p-confirmpopup-content").into_owned()} {..pt.content.attrs()}>{inner}</div>
                <div class={pt.footer.merge_class("p-confirmpopup-footer").into_owned()} {..pt.footer.attrs()}>
                    {reject}
                    {accept}
                </div>
            })
        }
    };
    let root_attrs = pt.root.attrs();
    view! {
        <div
            node_ref=core.panel
            class={class}
            style={style}
            role="alertdialog"
            data-p-confirmpopup-flipped={move || flipped.get().then_some("true")}
            {..root_attrs}
        >
            {body}
        </div>
    }
}
