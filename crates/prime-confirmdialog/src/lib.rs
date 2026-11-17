//! ConfirmDialog — port of PrimeNG 21.1.10 `p-confirmdialog`.
//!
//! A modal confirmation window on top of [`Dialog`]. The app sends a [`Confirmation`] to the shared [`ConfirmationService`] (it lives in
//! the context), and the component shows the confirmations whose `key` matches its own: the header, icon and question text and
//! the "No" and "Yes" buttons. The close icon, `Escape` and a click on the mask count as a rejection. The window appears only in the browser.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_confirmation::provide_confirmation_service;
//! use prime_confirmdialog::ConfirmDialog;
//!
//! Owner::new().with(|| {
//!     provide_confirmation_service();
//!     let html = view! { <ConfirmDialog /> }.to_html();
//!     // while there is no confirmation, there is no window in the markup
//!     assert!(!html.contains("p-dialog"));
//! });
//! ```

mod state;

use leptos::prelude::*;
use prime_button::Button;
use prime_confirmation::{ConfirmButtonProps, ConfirmEvent, ConfirmEventType, ConfirmFocus, Confirmation, use_confirmation_service};
use prime_core::{OptProp, Prop, Pt};
use prime_dialog::{Dialog, DialogPosition, DialogPt};
use std::sync::Arc;

pub use state::{button_class, button_label, pick, position_of, root_class};

/// Pass-through parts of [`ConfirmDialog`].
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ConfirmDialogPt {
    /// Window (`p-dialog`).
    pub root: Pt,
    /// Mask.
    pub mask: Pt,
    /// Header.
    pub header: Pt,
    /// Window content.
    pub content: Pt,
    /// Footer.
    pub footer: Pt,
    /// Icon.
    pub icon: Pt,
    /// Question text.
    pub message: Pt,
}

/// Context of the `headless` view.
#[derive(Clone)]
pub struct ConfirmDialogHeadless {
    /// The confirmation being shown.
    pub confirmation: Confirmation,
    /// Accept: calls `accept` and hides the window.
    pub accept: Callback<()>,
    /// Reject: calls `reject` and hides the window.
    pub reject: Callback<()>,
}

/// State shared by the window handlers.
struct Core {
    key: OptProp<String>,
    visible: RwSignal<bool>,
    /// The confirmation being shown (the content is built from it).
    confirmation: StoredValue<Option<Confirmation>, LocalStorage>,
    /// Grows on every confirmation replacement: the window content follows it.
    generation: RwSignal<u32>,
    /// The window was closed programmatically (buttons, `Close`): distinguishes it from the close icon, `Escape` and the mask.
    answered: StoredValue<bool>,
    on_hide: Option<Callback<Option<ConfirmEventType>>>,
}

impl Core {
    fn current(&self) -> Confirmation {
        self.confirmation.get_value().unwrap_or_default()
    }

    /// Service event.
    fn on_event(&self, event: ConfirmEvent) {
        match event {
            ConfirmEvent::Close => self.hide(None),
            ConfirmEvent::Confirm(confirmation) => {
                if self.key.get_untracked() != confirmation.key {
                    return;
                }
                self.confirmation.set_value(Some(*confirmation));
                self.generation.update(|g| *g += 1);
                self.visible.set(true);
            }
        }
    }

    /// Hides the window programmatically and reports the close type.
    fn hide(&self, kind: Option<ConfirmEventType>) {
        if !self.visible.get_untracked() {
            return;
        }
        self.answered.set_value(true);
        if let Some(cb) = self.on_hide {
            cb.run(kind);
        }
        self.visible.set(false);
    }

    /// Accept or reject by a button.
    fn answer(&self, accept: bool) {
        let confirmation = self.current();
        if let Some(cb) = if accept { confirmation.accept } else { confirmation.reject } {
            cb.run(());
        }
        self.hide(Some(if accept { ConfirmEventType::Accept } else { ConfirmEventType::Reject }));
    }

    /// The window was closed by the close icon, `Escape` or the mask: a rejection with the `Cancel` type.
    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    fn cancelled(&self) {
        if self.answered.try_update_value(std::mem::take).unwrap_or(false) {
            return;
        }
        if let Some(cb) = self.current().reject {
            cb.run(());
        }
        if let Some(cb) = self.on_hide {
            cb.run(Some(ConfirmEventType::Cancel));
        }
    }
}

/// Component default props that a confirmation can override.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
struct Defaults {
    header: OptProp<String>,
    message: OptProp<String>,
    icon: Option<ViewFn>,
    accept_label: OptProp<String>,
    reject_label: OptProp<String>,
    accept_icon: Option<ViewFn>,
    reject_icon: Option<ViewFn>,
    accept_visible: Prop<bool>,
    reject_visible: Prop<bool>,
    accept_button_style_class: OptProp<String>,
    reject_button_style_class: OptProp<String>,
    accept_aria_label: OptProp<String>,
    reject_aria_label: OptProp<String>,
    default_focus: Prop<ConfirmFocus>,
}

/// Views that are built on show.
struct Views {
    header_content: Option<ViewFn>,
    footer: Option<ViewFn>,
    message_content: Option<Callback<Confirmation, AnyView>>,
    icon_content: Option<ViewFn>,
    headless: Option<Callback<ConfirmDialogHeadless, AnyView>>,
}

/// Pick of a boolean confirmation field.
type BoolField = Arc<dyn Fn(&Confirmation) -> Option<bool> + Send + Sync>;

/// A confirmation window on top of `Dialog`.
///
/// Props accept a value or a signal; a confirmation overrides the same-named component values. Needs a
/// `ConfirmationService` in the context (`provide_confirmation_service`).
#[component]
pub fn ConfirmDialog(
    /// Key: the window shows only confirmations with the same key.
    #[prop(optional, into)]
    key: OptProp<String>,
    /// Default header.
    #[prop(optional, into)]
    header: OptProp<String>,
    /// Default question text.
    #[prop(optional, into)]
    message: OptProp<String>,
    /// Default icon.
    #[prop(optional, into)]
    icon: Option<ViewFn>,
    /// Accept button label.
    #[prop(optional, into)]
    accept_label: OptProp<String>,
    /// Reject button label.
    #[prop(optional, into)]
    reject_label: OptProp<String>,
    /// Accept button icon.
    #[prop(optional, into)]
    accept_icon: Option<ViewFn>,
    /// Reject button icon.
    #[prop(optional, into)]
    reject_icon: Option<ViewFn>,
    /// Whether to show the accept button.
    #[prop(optional, default = Prop::Static(true), into)]
    accept_visible: Prop<bool>,
    /// Whether to show the reject button.
    #[prop(optional, default = Prop::Static(true), into)]
    reject_visible: Prop<bool>,
    /// Extra class of the accept button.
    #[prop(optional, into)]
    accept_button_style_class: OptProp<String>,
    /// Extra class of the reject button.
    #[prop(optional, into)]
    reject_button_style_class: OptProp<String>,
    /// `aria-label` of the accept button.
    #[prop(optional, into)]
    accept_aria_label: OptProp<String>,
    /// `aria-label` of the reject button.
    #[prop(optional, into)]
    reject_aria_label: OptProp<String>,
    /// `aria-label` of the window close button.
    #[prop(optional, into)]
    close_aria_label: OptProp<String>,
    /// Whether to close the window with the `Escape` key.
    #[prop(optional, default = Prop::Static(true), into)]
    close_on_escape: Prop<bool>,
    /// Whether to close the window with a click on the mask.
    #[prop(optional, into)]
    dismissable_mask: Prop<bool>,
    /// Whether to block page scrolling.
    #[prop(optional, default = Prop::Static(true), into)]
    block_scroll: Prop<bool>,
    /// Whether there is a close button.
    #[prop(optional, default = Prop::Static(true), into)]
    closable: Prop<bool>,
    /// Whether the window is modal.
    #[prop(optional, default = Prop::Static(true), into)]
    modal: Prop<bool>,
    /// Window position.
    #[prop(optional, into)]
    position: Prop<DialogPosition>,
    /// What gets focus on show (`Accept` by default).
    #[prop(optional, default = Prop::Static(ConfirmFocus::Accept), into)]
    default_focus: Prop<ConfirmFocus>,
    /// Whether the window can be dragged.
    #[prop(optional, default = Prop::Static(true), into)]
    draggable: Prop<bool>,
    /// Whether to keep focus inside the window.
    #[prop(optional, default = Prop::Static(true), into)]
    focus_trap: Prop<bool>,
    /// Automatically assign `z-index`.
    #[prop(optional, default = Prop::Static(true), into)]
    auto_z_index: Prop<bool>,
    /// Base `z-index` on top of the configuration value.
    #[prop(optional, into)]
    base_z_index: Prop<i32>,
    /// Window width on narrow screens: "limit → width" pairs.
    #[prop(optional, into)]
    breakpoints: Vec<(String, String)>,
    /// Inline style of the window.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Extra class of the window.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Extra class of the mask.
    #[prop(optional, into)]
    mask_style_class: OptProp<String>,
    /// Pass-through of parts.
    #[prop(optional)]
    pt: Option<ConfirmDialogPt>,
    /// Custom header content.
    #[prop(optional, into)]
    header_content: Option<ViewFn>,
    /// Custom footer instead of the buttons.
    #[prop(optional, into)]
    footer: Option<ViewFn>,
    /// Custom content instead of the icon and question text.
    #[prop(optional)]
    message_content: Option<Callback<Confirmation, AnyView>>,
    /// Custom icon instead of the standard one.
    #[prop(optional, into)]
    icon_content: Option<ViewFn>,
    /// Custom content of the whole window.
    #[prop(optional)]
    headless: Option<Callback<ConfirmDialogHeadless, AnyView>>,
    /// The window is hidden; the type is how it was closed (`None` — by the service).
    #[prop(optional)]
    on_hide: Option<Callback<Option<ConfirmEventType>>>,
) -> impl IntoView {
    let pt = Arc::new(pt.unwrap_or_default());
    let core = Arc::new(Core {
        key,
        visible: RwSignal::new(false),
        confirmation: StoredValue::new_local(None),
        generation: RwSignal::new(0),
        answered: StoredValue::new(false),
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
            core.visible.try_set(false);
        });
    }
    // The window was closed not by us (close icon, `Escape`, mask): this is a rejection. Effects are needed only in the browser.
    #[cfg(target_arch = "wasm32")]
    {
        let core = core.clone();
        Effect::new(move |was: Option<bool>| {
            let now = core.visible.get();
            if was == Some(true) && !now {
                core.cancelled();
            }
            now
        });
    }

    let defaults = Arc::new(Defaults {
        header,
        message,
        icon,
        accept_label,
        reject_label,
        accept_icon,
        reject_icon,
        accept_visible,
        reject_visible,
        accept_button_style_class,
        reject_button_style_class,
        accept_aria_label,
        reject_aria_label,
        default_focus,
    });
    let views = Arc::new(Views { header_content, footer, message_content, icon_content, headless });
    let generation = core.generation;
    let derived = |f: BoolField, base: Prop<bool>| {
        let core = core.clone();
        Signal::derive(move || {
            generation.track();
            pick(f(&core.current()), base.get())
        })
    };
    let modal_sig = derived(Arc::new(|c| c.modal), modal);
    let closable_sig = derived(Arc::new(|c| c.closable), closable);
    let block_scroll_sig = derived(Arc::new(|c| c.block_scroll), block_scroll);
    let escape_sig = derived(Arc::new(|c| c.close_on_escape), close_on_escape);
    let mask_sig = derived(Arc::new(|c| c.dismissable_mask), dismissable_mask);
    let position_sig = {
        let core = core.clone();
        Signal::derive(move || {
            generation.track();
            core.current().position.as_deref().map_or_else(|| position.get(), position_of)
        })
    };
    let header_sig = {
        let (core, defaults) = (core.clone(), defaults.clone());
        Signal::derive(move || {
            generation.track();
            core.current().header.or_else(|| defaults.header.get())
        })
    };
    let class_sig = Signal::derive(move || Some(root_class(style_class.get().as_deref())));

    let footer_view = match views.footer.clone() {
        Some(custom) => custom,
        None => {
            let (core, defaults, pt) = (core.clone(), defaults.clone(), pt.clone());
            ViewFn::from(move || {
                let (core, defaults, pt) = (core.clone(), defaults.clone(), pt.clone());
                view! { {move || footer_buttons(&core, &defaults, &pt)} }
            })
        }
    };
    let headless_view = views.headless.map(|headless| {
        let core = core.clone();
        ViewFn::from(move || {
            let core = core.clone();
            view! {
                {move || {
                    generation.track();
                    let (accept, reject) = ({
                        let core = core.clone();
                        Callback::new(move |()| core.answer(true))
                    }, {
                        let core = core.clone();
                        Callback::new(move |()| core.answer(false))
                    });
                    headless.run(ConfirmDialogHeadless { confirmation: core.current(), accept, reject })
                }}
            }
        })
    });

    let on_show = {
        let (core, defaults) = (core.clone(), defaults.clone());
        Callback::new(move |()| focus_default(&core, &defaults))
    };
    let on_dialog_hide = {
        let core = core.clone();
        Callback::new(move |()| {
            // The window left the DOM: the confirmation is forgotten unless a new one arrived in the meantime.
            if !core.visible.get_untracked() {
                core.confirmation.set_value(None);
            }
        })
    };
    let dialog_pt = DialogPt {
        root: pt.root.clone(),
        mask: pt.mask.clone(),
        header: pt.header.clone(),
        content: pt.content.clone(),
        footer: pt.footer.clone(),
        ..DialogPt::default()
    };
    let body = {
        let (core, defaults, views, pt) = (core.clone(), defaults.clone(), views.clone(), pt.clone());
        move || {
            let (core, defaults, views, pt) = (core.clone(), defaults.clone(), views.clone(), pt.clone());
            view! { {move || message_body(&core, &defaults, &views, &pt)} }
        }
    };
    let visible = core.visible;
    // `header_content` and `headless` of `Dialog` are optional views: we pass them only when they are set.
    macro_rules! dialog {
        ($($extra:tt)*) => {
            view! {
                <Dialog
                    visible=visible
                    header=header_sig
                    role="alertdialog"
                    modal=modal_sig
                    closable=closable_sig
                    block_scroll=block_scroll_sig
                    close_on_escape=escape_sig
                    dismissable_mask=mask_sig
                    position=position_sig
                    draggable=draggable
                    resizable=false
                    focus_on_show=false
                    focus_trap=focus_trap
                    auto_z_index=auto_z_index
                    base_z_index=base_z_index
                    breakpoints=breakpoints
                    close_aria_label=close_aria_label
                    style=style
                    style_class=class_sig
                    mask_style_class=mask_style_class
                    footer=footer_view
                    on_show=on_show
                    on_hide=on_dialog_hide
                    pt=dialog_pt
                    $($extra)*
                >
                    {body.clone()}
                </Dialog>
            }
            .into_any()
        };
    }
    match (views.header_content.clone(), headless_view) {
        (Some(header_content), Some(headless)) => dialog!(header_content=header_content headless=headless),
        (Some(header_content), None) => dialog!(header_content = header_content),
        (None, Some(headless)) => dialog!(headless = headless),
        (None, None) => dialog!(),
    }
}

/// Window body: the icon and question text (or custom views).
fn message_body(core: &Arc<Core>, defaults: &Arc<Defaults>, views: &Arc<Views>, pt: &Arc<ConfirmDialogPt>) -> AnyView {
    core.generation.track();
    let confirmation = core.current();
    if let Some(custom) = views.message_content {
        return custom.run(confirmation);
    }
    let icon = match views.icon_content.clone() {
        Some(custom) => Some(custom.run()),
        None => confirmation.icon.clone().or_else(|| defaults.icon.clone()).map(|icon| {
            let class = pt.icon.merge_class("p-confirmdialog-icon").into_owned();
            view! { <span class={class} {..pt.icon.attrs()}>{icon.run()}</span> }.into_any()
        }),
    };
    let text = confirmation.message.clone().or_else(|| defaults.message.get()).unwrap_or_default();
    let class = pt.message.merge_class("p-confirmdialog-message").into_owned();
    view! {
        {icon}
        <span class={class} {..pt.message.attrs()}>{text}</span>
    }
    .into_any()
}

/// Footer buttons: reject, then accept.
fn footer_buttons(core: &Arc<Core>, defaults: &Arc<Defaults>, _pt: &Arc<ConfirmDialogPt>) -> AnyView {
    core.generation.track();
    let confirmation = core.current();
    let reject =
        pick(confirmation.reject_visible, defaults.reject_visible.get()).then(|| footer_button(core, defaults, &confirmation, false));
    let accept =
        pick(confirmation.accept_visible, defaults.accept_visible.get()).then(|| footer_button(core, defaults, &confirmation, true));
    view! { {reject}{accept} }.into_any()
}

/// A single footer button taking the confirmation and component props into account.
fn footer_button(core: &Arc<Core>, defaults: &Arc<Defaults>, confirmation: &Confirmation, accept: bool) -> AnyView {
    let props: ConfirmButtonProps =
        if accept { confirmation.accept_button_props.clone() } else { confirmation.reject_button_props.clone() }.unwrap_or_default();
    let (own_label, comp_label, own_class, comp_class, own_icon, comp_icon, aria) = if accept {
        (
            confirmation.accept_label.clone(),
            defaults.accept_label.get(),
            confirmation.accept_button_style_class.clone(),
            defaults.accept_button_style_class.get(),
            confirmation.accept_icon.clone(),
            defaults.accept_icon.clone(),
            defaults.accept_aria_label.get(),
        )
    } else {
        (
            confirmation.reject_label.clone(),
            defaults.reject_label.get(),
            confirmation.reject_button_style_class.clone(),
            defaults.reject_button_style_class.get(),
            confirmation.reject_icon.clone(),
            defaults.reject_icon.clone(),
            defaults.reject_aria_label.get(),
        )
    };
    let default = prime_core::use_prime_config().locale.text(if accept { "accept" } else { "reject" }).to_owned();
    let label = button_label(own_label.or(comp_label).as_deref(), props.label.as_deref(), &default);
    let base = if accept { "p-confirmdialog-accept-button" } else { "p-confirmdialog-reject-button" };
    let class = button_class(base, own_class.or(comp_class).as_deref());
    let icon = own_icon.or(comp_icon);
    let aria_label = aria.unwrap_or_else(|| label.clone());
    let core = core.clone();
    let on_click = Callback::new(move |_: web_sys::MouseEvent| core.answer(accept));
    let (severity, outlined, text, size) = (props.severity, props.outlined.unwrap_or(false), props.text.unwrap_or(false), props.size);
    match (icon, size) {
        (Some(icon), Some(size)) => view! {
            <Button label=label aria_label=aria_label size=size severity=severity outlined=outlined text=text style_class=class icon=icon on_click=on_click />
        }
        .into_any(),
        (Some(icon), None) => view! {
            <Button label=label aria_label=aria_label severity=severity outlined=outlined text=text style_class=class icon=icon on_click=on_click />
        }
        .into_any(),
        (None, Some(size)) => view! {
            <Button label=label aria_label=aria_label size=size severity=severity outlined=outlined text=text style_class=class on_click=on_click />
        }
        .into_any(),
        (None, None) => view! {
            <Button label=label aria_label=aria_label severity=severity outlined=outlined text=text style_class=class on_click=on_click />
        }
        .into_any(),
    }
}

/// The window is shown: focus the needed button.
fn focus_default(core: &Arc<Core>, defaults: &Arc<Defaults>) {
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::JsCast;
        let wanted = pick(core.current().default_focus, defaults.default_focus.get_untracked());
        let selector = match wanted {
            ConfirmFocus::Accept => ".p-confirmdialog .p-confirmdialog-accept-button",
            ConfirmFocus::Reject => ".p-confirmdialog .p-confirmdialog-reject-button",
            ConfirmFocus::Close => ".p-confirmdialog .p-dialog-close-button",
            ConfirmFocus::None => return,
        };
        // The footer and header buttons get into the DOM a bit later than `on_show`: we look for them on the next tick.
        set_timeout(
            move || {
                let target = web_sys::window()
                    .and_then(|w| w.document())
                    .and_then(|d| d.query_selector(selector).ok().flatten())
                    .and_then(|e| e.dyn_into::<web_sys::HtmlElement>().ok());
                if let Some(target) = target {
                    let _ = target.focus();
                }
            },
            std::time::Duration::from_millis(20),
        );
    }
    #[cfg(not(target_arch = "wasm32"))]
    let _ = (core, defaults);
}
