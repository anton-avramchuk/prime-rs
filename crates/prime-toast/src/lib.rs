//! Toast and `MessageService` — port of `p-toast` and `MessageService` from PrimeNG 21.1.10.
//!
//! Notifications in a screen corner. The app sends messages to the shared [`MessageService`] (kept in context), and [`Toast`] components
//! show those addressed to their `key`. Messages close on their own via `life`, via the button and via `clear`; hovering pauses the
//! timer.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_toast::{Toast, ToastMessage, ToastSeverity, provide_message_service};
//!
//! Owner::new().with(|| {
//!     let service = provide_message_service();
//!     let html = view! { <Toast /> }.to_html();
//!     // no messages on the server: an empty container is rendered
//!     assert!(html.contains("p-toast") && !html.contains("p-toast-message"));
//!     service.add(ToastMessage::new(ToastSeverity::Success, "Saved", "Done"));
//! });
//! ```

mod message;
mod service;
mod state;

use leptos::prelude::*;
use prime_core::{OptProp, Prop, Pt, ZGroup, use_prime_config};
use prime_icons::{CheckIcon, ExclamationTriangleIcon, InfoCircleIcon, TimesCircleIcon, TimesIcon};
use std::sync::Arc;

pub use message::{ToastMessage, ToastSeverity};
pub use service::{MessageService, ServiceEvent, provide_message_service, use_message_service};
pub use state::{
    ToastPosition, breakpoints_css, can_add, contains_message, lifetime, matches_key, message_classes, position_style, root_classes,
};

#[cfg(target_arch = "wasm32")]
use leptos::leptos_dom::helpers::{TimeoutHandle, set_timeout_with_handle};
#[cfg(target_arch = "wasm32")]
use std::time::Duration;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

/// Fallback exit time: the animation lasts 250 ms, but without styles or with animation disabled `animationend` never fires.
#[cfg(target_arch = "wasm32")]
const LEAVE_FALLBACK: Duration = Duration::from_millis(600);

/// Pass-through parts of [`Toast`].
#[derive(Clone, Debug, Default)]
pub struct ToastPt {
    /// Container.
    pub root: Pt,
    /// Message.
    pub message: Pt,
    /// Message content.
    pub message_content: Pt,
    /// Message icon.
    pub message_icon: Pt,
    /// Text block.
    pub message_text: Pt,
    /// Message summary.
    pub summary: Pt,
    /// Message detail.
    pub detail: Pt,
    /// Close button.
    pub close_button: Pt,
    /// Close button icon.
    pub close_icon: Pt,
}

/// A displayed message.
#[derive(Clone)]
struct Item {
    key: u64,
    message: ToastMessage,
    /// False starts the exit animation.
    visible: RwSignal<bool>,
}

/// `Toast` state shared between handlers.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
struct Core {
    id: usize,
    key: Option<String>,
    life: Prop<u32>,
    auto_z_index: Prop<bool>,
    base_z_index: Prop<i32>,
    prevent_open_duplicates: Prop<bool>,
    prevent_duplicates: Prop<bool>,
    items: RwSignal<Vec<Item>>,
    archive: StoredValue<Vec<ToastMessage>>,
    next_key: StoredValue<u64>,
    z_index: StoredValue<Option<i32>>,
    root: NodeRef<leptos::html::Div>,
    on_close: Option<Callback<ToastMessage>>,
}

impl Core {
    fn on_event(&self, event: ServiceEvent) {
        match event {
            ServiceEvent::Add(messages) => {
                for message in messages {
                    let open: Vec<ToastMessage> = self.items.with_untracked(|items| items.iter().map(|i| i.message.clone()).collect());
                    let archive = self.archive.get_value();
                    if !can_add(
                        self.key.as_deref(),
                        &message,
                        &open,
                        &archive,
                        self.prevent_open_duplicates.get_untracked(),
                        self.prevent_duplicates.get_untracked(),
                    ) {
                        continue;
                    }
                    if self.prevent_duplicates.get_untracked() {
                        self.archive.update_value(|a| a.push(message.clone()));
                    }
                    let key = self.next_key.try_update_value(|n| {
                        *n += 1;
                        *n
                    });
                    let item = Item { key: key.unwrap_or_default(), message, visible: RwSignal::new(true) };
                    self.items.update(|items| items.push(item));
                    self.raise();
                }
            }
            ServiceEvent::Clear(key) => {
                if key.is_none() || matches_key(self.key.as_deref(), key.as_deref()) {
                    for item in self.items.get_untracked() {
                        item.visible.set(false);
                    }
                }
            }
        }
    }

    /// Assigns the container a `z-index` while there are messages.
    fn raise(&self) {
        if !self.auto_z_index.get_untracked() || self.z_index.get_value().is_some() {
            return;
        }
        let base = use_prime_config().z_index.base(ZGroup::Modal) + self.base_z_index.get_untracked();
        let z = prime_core::zindex::generate(ZGroup::Modal, base);
        self.z_index.set_value(Some(z));
        #[cfg(target_arch = "wasm32")]
        if let Some(root) = self.root.get_untracked() {
            let root: web_sys::HtmlElement = root.unchecked_into();
            let _ = root.style().set_property("z-index", &z.to_string());
        }
    }

    /// Releases the `z-index` once no messages are left.
    fn lower(&self) {
        if let Some(z) = self.z_index.try_update_value(Option::take).flatten() {
            prime_core::zindex::revert(z);
            #[cfg(target_arch = "wasm32")]
            if let Some(root) = self.root.get_untracked() {
                let root: web_sys::HtmlElement = root.unchecked_into();
                let _ = root.style().remove_property("z-index");
            }
        }
    }

    /// A message is gone (exit animation finished): it is removed from the list, `on_close` fires, the layer is released.
    fn remove(&self, key: u64) {
        let removed = self.items.try_update(|items| items.iter().position(|i| i.key == key).map(|index| items.remove(index)));
        if let Some(Some(item)) = removed {
            if let Some(cb) = self.on_close {
                cb.run(item.message);
            }
            if self.items.with_untracked(Vec::is_empty) {
                self.lower();
            }
        }
    }
}

/// Notification container; messages come from the [`MessageService`] in context.
///
/// Every property accepts a value or a signal. `key` sets the addressee: the component shows only messages with the same key.
#[component]
pub fn Toast(
    /// Addressee key; messages with another key are ignored.
    #[prop(optional, into)]
    key: Option<String>,
    /// Position on screen.
    #[prop(optional, into)]
    position: Prop<ToastPosition>,
    /// Default message display time, ms.
    #[prop(optional, default = Prop::Static(3000), into)]
    life: Prop<u32>,
    /// Assign `z-index` automatically.
    #[prop(optional, default = Prop::Static(true), into)]
    auto_z_index: Prop<bool>,
    /// Base `z-index` on top of the config value.
    #[prop(optional, into)]
    base_z_index: Prop<i32>,
    /// Do not add a message if an identical one is already shown.
    #[prop(optional, into)]
    prevent_open_duplicates: Prop<bool>,
    /// Show each message once (do not add one matching a previously added message).
    #[prop(optional, into)]
    prevent_duplicates: Prop<bool>,
    /// Container styles per screen width: pairs of "max width — CSS properties".
    #[prop(optional, into)]
    breakpoints: Vec<(String, Vec<(String, String)>)>,
    /// Extra container class.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Custom message content instead of the icon and texts (the close button stays).
    #[prop(optional, into)]
    message: Option<Callback<ToastMessage, AnyView>>,
    /// Replaces everything inside the message; receives the message and a function to close it.
    #[prop(optional, into)]
    headless: Option<Callback<(ToastMessage, Callback<()>), AnyView>>,
    /// Called when the message has closed and been removed.
    #[prop(optional, into)]
    on_close: Option<Callback<ToastMessage>>,
    /// Pass-through parts.
    #[prop(optional)]
    pt: Option<ToastPt>,
) -> impl IntoView {
    let pt = Arc::new(pt.unwrap_or_default());
    let core = Arc::new(Core {
        id: prime_core::ids::next_unique(),
        key,
        life,
        auto_z_index,
        base_z_index,
        prevent_open_duplicates,
        prevent_duplicates,
        items: RwSignal::new(Vec::new()),
        archive: StoredValue::new(Vec::new()),
        next_key: StoredValue::new(0),
        z_index: StoredValue::new(None),
        root: NodeRef::new(),
        on_close,
    });

    // Subscription to the service; dropped on unmount.
    let service = use_message_service();
    let subscription = {
        let core = core.clone();
        service.subscribe(Callback::new(move |event| core.on_event(event)))
    };
    {
        let core = core.clone();
        on_cleanup(move || {
            service.unsubscribe(subscription);
            core.lower();
        });
    }

    // Responsive rules in `head` — browser only.
    #[cfg(target_arch = "wasm32")]
    if !breakpoints.is_empty() {
        let attr = format!("data-toast-{}", core.id);
        let css = breakpoints_css(&attr, &breakpoints);
        let style_el: StoredValue<Option<web_sys::Element>, LocalStorage> = StoredValue::new_local(None);
        let core = core.clone();
        Effect::new(move |_| {
            let (Some(root), Some(doc)) = (core.root.get(), web_sys::window().and_then(|w| w.document())) else { return };
            if style_el.with_value(Option::is_some) {
                return;
            }
            let _ = root.set_attribute(&attr, "");
            if let (Some(head), Ok(style)) = (doc.head(), doc.create_element("style")) {
                style.set_text_content(Some(&css));
                let _ = head.append_child(&style);
                style_el.set_value(Some(style));
            }
        });
        on_cleanup(move || {
            if let Some(el) = style_el.try_update_value(Option::take).flatten() {
                el.remove();
            }
        });
    }
    #[cfg(not(target_arch = "wasm32"))]
    let _ = &breakpoints;

    let class = {
        let (pt, position) = (pt.clone(), position.clone());
        move || {
            let mut cn = root_classes(position.get(), None);
            style_class.with(|c| {
                if let Some(c) = c.as_deref().filter(|c| !c.is_empty()) {
                    cn.push(' ');
                    cn.push_str(c);
                }
            });
            pt.root.merge_class(&cn)
        }
    };
    let style = {
        let (pt, position) = (pt.clone(), position.clone());
        move || pt.root.merge_style_parts(&[&position_style(position.get())])
    };
    let data_p = move || position.get().as_str();

    let items = core.items;
    let root = core.root;
    let list_core = core.clone();
    let list_pt = pt.clone();
    view! {
        <div node_ref=root class={class} style={style} data-p={data_p} {..pt.root.attrs()}>
            <For
                each=move || items.get()
                key=|item| item.key
                children=move |item| build_item(&list_core, &list_pt, item, message, headless)
            />
        </div>
    }
}

/// Message lifetime timer and the closing flag.
#[derive(Clone, Copy)]
struct Life {
    visible: RwSignal<bool>,
    closing: StoredValue<bool>,
    #[cfg(target_arch = "wasm32")]
    timer: StoredValue<Option<TimeoutHandle>, LocalStorage>,
}

impl Life {
    fn new(visible: RwSignal<bool>) -> Self {
        Self {
            visible,
            closing: StoredValue::new(false),
            #[cfg(target_arch = "wasm32")]
            timer: StoredValue::new_local(None),
        }
    }

    /// (Re)starts the time-based close timer.
    fn start(&self, sticky: bool, message_life: Option<u32>, toast_life: u32) {
        #[cfg(target_arch = "wasm32")]
        if !sticky {
            self.clear();
            let visible = self.visible;
            let ms = lifetime(message_life, toast_life);
            if let Ok(handle) = set_timeout_with_handle(
                move || {
                    visible.try_set(false);
                },
                Duration::from_millis(u64::from(ms)),
            ) {
                self.timer.set_value(Some(handle));
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        let _ = (sticky, message_life, toast_life);
    }

    /// Stops the timer.
    fn clear(&self) {
        #[cfg(target_arch = "wasm32")]
        if let Some(handle) = self.timer.try_update_value(Option::take).flatten() {
            handle.clear();
        }
    }

    /// Starts closing: the timer is stopped, the exit animation is started.
    fn close(&self) {
        self.closing.set_value(true);
        self.clear();
        self.visible.try_set(false);
    }
}

/// Icon by severity or by the message CSS class.
fn message_icon(message: &ToastMessage, pt: &ToastPt) -> AnyView {
    if let Some(icon) = &message.icon {
        let class = pt.message_icon.merge_class(&format!("p-toast-message-icon pi {icon}"));
        return view! { <span class={class} {..pt.message_icon.attrs()}></span> }.into_any();
    }
    let class = pt.message_icon.merge_class("p-toast-message-icon");
    match message.severity {
        ToastSeverity::Success => view! { <CheckIcon class={class} /> }.into_any(),
        ToastSeverity::Warn => view! { <ExclamationTriangleIcon class={class} /> }.into_any(),
        ToastSeverity::Error => view! { <TimesCircleIcon class={class} /> }.into_any(),
        _ => view! { <InfoCircleIcon class={class} /> }.into_any(),
    }
}

/// Message close button.
fn close_button(message: &ToastMessage, pt: &ToastPt, life: Life) -> AnyView {
    let label = use_prime_config().locale.text("aria.close").to_owned();
    let icon = match &message.close_icon {
        Some(icon) => {
            let class = pt.close_icon.merge_class(&format!("p-toast-close-icon pi {icon}"));
            view! { <span class={class} {..pt.close_icon.attrs()}></span> }.into_any()
        }
        None => view! { <TimesIcon class={pt.close_icon.merge_class("p-toast-close-icon")} /> }.into_any(),
    };
    view! {
        <div>
            <button
                type="button"
                class={pt.close_button.merge_class("p-toast-close-button")}
                aria-label=label
                on:click=move |e: web_sys::MouseEvent| {
                    e.prevent_default();
                    life.close();
                }
                {..pt.close_button.attrs()}
            >
                {icon}
            </button>
        </div>
    }
    .into_any()
}

/// A single message: markup, lifetime timer, hover, closing and exit animation.
fn build_item(
    core: &Arc<Core>,
    pt: &Arc<ToastPt>,
    item: Item,
    template: Option<Callback<ToastMessage, AnyView>>,
    headless: Option<Callback<(ToastMessage, Callback<()>), AnyView>>,
) -> AnyView {
    let Item { key, message, visible } = item;
    let life = Life::new(visible);
    let (sticky, message_life, toast_life) = (message.sticky, message.life, core.life.clone());

    // Lifetime timer and exit — browser only.
    #[cfg(target_arch = "wasm32")]
    {
        life.start(sticky, message_life, toast_life.get_untracked());
        let core = core.clone();
        Effect::new(move |_| {
            if !visible.get() {
                life.closing.set_value(true);
                life.clear();
                let core = core.clone();
                // Fallback exit: if `animationend` never fires, the message leaves anyway.
                set_timeout(move || core.remove(key), LEAVE_FALLBACK);
            }
        });
        on_cleanup(move || life.clear());
    }

    let class = {
        let (pt, message) = (pt.clone(), message.clone());
        move || {
            let base = message_classes(&message.severity, message.style_class.as_deref());
            let animation = if visible.get() { "p-toast-message-enter-active" } else { "p-toast-message-leave-active" };
            pt.message.merge_class(&format!("{base} {animation}"))
        }
    };
    let close_fn = Callback::new(move |()| life.close());

    let body = match headless {
        Some(headless) => headless.run((message.clone(), close_fn)),
        None => {
            let extra = message.content_style_class.as_ref().map(|c| format!(" {c}")).unwrap_or_default();
            let content_class = pt.message_content.merge_class(&format!("p-toast-message-content{extra}"));
            let inner = match template {
                Some(template) => template.run(message.clone()),
                None => view! {
                    {message_icon(&message, pt)}
                    <div class={pt.message_text.merge_class("p-toast-message-text")} {..pt.message_text.attrs()}>
                        <div class={pt.summary.merge_class("p-toast-summary")} {..pt.summary.attrs()}>{message.summary.clone()}</div>
                        <div class={pt.detail.merge_class("p-toast-detail")} {..pt.detail.attrs()}>{message.detail.clone()}</div>
                    </div>
                }
                .into_any(),
            };
            let button = message.closable.then(|| close_button(&message, pt, life));
            view! {
                <div class={content_class} {..pt.message_content.attrs()}>
                    {inner}
                    {button}
                </div>
            }
            .into_any()
        }
    };

    let end_core = core.clone();
    view! {
        <div
            id={message.id.clone()}
            class={class}
            role="alert"
            aria-live="assertive"
            aria-atomic="true"
            data-p={message.severity.as_str().to_owned()}
            on:mouseenter=move |_| life.clear()
            on:mouseleave=move |_| {
                if !life.closing.get_value() {
                    life.start(sticky, message_life, toast_life.get_untracked());
                }
            }
            on:animationend=move |e: web_sys::AnimationEvent| animation_end(&end_core, key, visible, &e)
            {..pt.message.attrs()}
        >
            {body}
        </div>
    }
    .into_any()
}

/// `animationend` of the message itself (not nested elements) removes it after exit has started.
fn animation_end(core: &Arc<Core>, key: u64, visible: RwSignal<bool>, event: &web_sys::AnimationEvent) {
    let own = event.target().is_some_and(|t| event.current_target().is_some_and(|c| t == c));
    if own && !visible.get_untracked() {
        core.remove(key);
    }
}
