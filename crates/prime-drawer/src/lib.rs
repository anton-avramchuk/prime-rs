//! Drawer - port of PrimeNG 21.1.10 `p-drawer`.
//!
//! A panel at the screen edge over the page: mask, enter and leave animations, closing by mask, button and `Escape`,
//! page scroll blocking and a focus trap. The panel and mask are rendered into `body`.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_drawer::{Drawer, DrawerPosition};
//!
//! let visible = RwSignal::new(false);
//! let html = view! {
//!     <Drawer visible=visible header="Left Drawer" position=DrawerPosition::Left>
//!         <p>"Body"</p>
//!     </Drawer>
//! }
//! .to_html();
//! // while the panel is not shown, it is absent from the markup
//! assert!(!html.contains("p-drawer") && !html.contains("Body"));
//! ```
#![recursion_limit = "512"]

mod state;

use leptos::prelude::*;
use prime_button::{Button, ButtonPt, ButtonSeverity};
use prime_core::{OptProp, Prop, Pt};
use prime_dom::Portal;
use prime_icons::TimesIcon;
use std::sync::Arc;

pub use state::{DrawerPosition, animation_class, data_p, is_topmost, mask_classes, mask_z, root_classes};

#[cfg(target_arch = "wasm32")]
use leptos::leptos_dom::helpers::{TimeoutHandle, set_timeout_with_handle};
#[cfg(target_arch = "wasm32")]
use prime_core::{ZGroup, use_prime_config};
#[cfg(target_arch = "wasm32")]
use prime_dom::{FocusTrap, block_body_scroll, unblock_body_scroll};
#[cfg(target_arch = "wasm32")]
use std::time::Duration;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

/// Fallback leave time: the animation lasts 500 ms, but without styles or with animation disabled `animationend` never arrives.
#[cfg(target_arch = "wasm32")]
const LEAVE_FALLBACK: Duration = Duration::from_millis(700);

/// Pass-through for [`Drawer`] parts.
#[derive(Clone, Debug, Default)]
pub struct DrawerPt {
    /// Panel root.
    pub root: Pt,
    /// Mask.
    pub mask: Pt,
    /// Panel header.
    pub header: Pt,
    /// Header title.
    pub title: Pt,
    /// Close button.
    pub close_button: Pt,
    /// Content.
    pub content: Pt,
    /// Footer.
    pub footer: Pt,
}

/// State shared by the panel handlers.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
struct Core {
    visible: RwSignal<bool>,
    /// Panel in the DOM (while the leave animation plays it is still in the DOM).
    rendered: RwSignal<bool>,
    /// The leave animation is playing.
    leaving: RwSignal<bool>,
    panel: NodeRef<leptos::html::Div>,
    mask: NodeRef<leptos::html::Div>,
    position: Prop<DrawerPosition>,
    full_screen: Prop<bool>,
    modal: Prop<bool>,
    dismissible: Prop<bool>,
    close_on_escape: Prop<bool>,
    block_scroll: Prop<bool>,
    auto_z_index: Prop<bool>,
    base_z_index: Prop<i32>,
    cleanups: StoredValue<Vec<Box<dyn FnOnce()>>, LocalStorage>,
    z_index: StoredValue<Option<i32>>,
    scroll_blocked: StoredValue<bool>,
    was_open: StoredValue<bool>,
    on_show: Option<Callback<()>>,
    on_hide: Option<Callback<()>>,
    #[cfg(target_arch = "wasm32")]
    trap: StoredValue<Option<FocusTrap>, LocalStorage>,
    #[cfg(target_arch = "wasm32")]
    timer: StoredValue<Option<TimeoutHandle>, LocalStorage>,
}

impl Core {
    /// Removes the `Escape` listener and the focus trap.
    fn release_input(&self) {
        for cleanup in self.cleanups.try_update_value(std::mem::take).unwrap_or_default() {
            cleanup();
        }
        #[cfg(target_arch = "wasm32")]
        {
            self.trap.try_update_value(|t| t.take());
        }
    }

    /// Releases the layer and the scroll lock.
    fn release_layer(&self) {
        if let Some(z) = self.z_index.try_update_value(Option::take).flatten() {
            prime_core::zindex::revert(z);
        }
        if self.scroll_blocked.try_update_value(std::mem::take).unwrap_or(false) {
            #[cfg(target_arch = "wasm32")]
            unblock_body_scroll();
        }
    }

    /// Full release when the component unmounts.
    fn teardown(&self) {
        self.release_input();
        self.release_layer();
        #[cfg(target_arch = "wasm32")]
        if let Some(handle) = self.timer.try_update_value(Option::take).flatten() {
            handle.clear();
        }
    }

    /// The panel has just entered the DOM: layer, focus trap, scroll lock, Escape, `on_show`.
    #[cfg(target_arch = "wasm32")]
    fn opened(self: &Arc<Self>) {
        let Some(panel) = self.panel.get_untracked() else { return };
        let panel: web_sys::HtmlElement = panel.unchecked_into();
        if self.auto_z_index.get_untracked() {
            let base = use_prime_config().z_index.base(ZGroup::Modal) + self.base_z_index.get_untracked();
            let z = prime_core::zindex::generate(ZGroup::Modal, base);
            self.z_index.set_value(Some(z));
            let _ = panel.style().set_property("z-index", &z.to_string());
            if let Some(mask) = self.mask.get_untracked() {
                let mask: web_sys::HtmlElement = mask.unchecked_into();
                let _ = mask.style().set_property("z-index", &mask_z(z).to_string());
            }
        }
        if self.modal.get_untracked() && self.block_scroll.get_untracked() {
            block_body_scroll();
            self.scroll_blocked.set_value(true);
        }
        self.trap.set_value(FocusTrap::attach(&panel));
        if self.close_on_escape.get_untracked() {
            let this = self.clone();
            let handle = window_event_listener(leptos::ev::keydown, move |e: web_sys::KeyboardEvent| {
                if e.key() != "Escape" || !this.visible.get_untracked() {
                    return;
                }
                let own = this.z_index.get_value().unwrap_or(0);
                if is_topmost(own, prime_core::zindex::current()) {
                    this.visible.set(false);
                }
            });
            self.cleanups.update_value(|c| c.push(Box::new(move || handle.remove())));
        }
        if let Some(cb) = self.on_show {
            cb.run(());
        }
    }

    /// Closing has started: listeners removed, the leave animation plays, `on_hide`.
    #[cfg(target_arch = "wasm32")]
    fn begin_leave(self: &Arc<Self>) {
        self.release_input();
        self.leaving.set(true);
        let this = self.clone();
        let handle = set_timeout_with_handle(move || this.finish_leave(), LEAVE_FALLBACK);
        if let Ok(handle) = handle {
            self.timer.set_value(Some(handle));
        }
        if let Some(cb) = self.on_hide {
            cb.run(());
        }
    }

    /// The leave animation has finished: the panel is removed from the DOM, the layer and lock are released.
    #[cfg(target_arch = "wasm32")]
    fn finish_leave(&self) {
        if self.rendered.is_disposed() || !self.leaving.get_untracked() {
            return;
        }
        if let Some(handle) = self.timer.try_update_value(Option::take).flatten() {
            handle.clear();
        }
        self.release_layer();
        self.leaving.set(false);
        self.rendered.set(false);
    }
}

/// Panel at the screen edge with a mask, animation and a focus trap.
///
/// Each property accepts a value or a signal; `visible` is a two-way signal. The panel and mask are rendered
/// into `body` and are not emitted by the server before hydration.
#[component]
pub fn Drawer(
    /// Two-way visibility signal; without it the state is kept internally (hidden by default).
    #[prop(optional)]
    visible: Option<RwSignal<bool>>,
    /// Screen edge.
    #[prop(optional, into)]
    position: Prop<DrawerPosition>,
    /// Stretch the panel to the full screen.
    #[prop(optional, into)]
    full_screen: Prop<bool>,
    /// Show a mask behind the panel.
    #[prop(optional, default = Prop::Static(true), into)]
    modal: Prop<bool>,
    /// Close the panel by clicking the mask.
    #[prop(optional, default = Prop::Static(true), into)]
    dismissible: Prop<bool>,
    /// Close the panel with the `Escape` key.
    #[prop(optional, default = Prop::Static(true), into)]
    close_on_escape: Prop<bool>,
    /// Show a close button in the header.
    #[prop(optional, default = Prop::Static(true), into)]
    closable: Prop<bool>,
    /// Block page scrolling while the panel is shown.
    #[prop(optional, into)]
    block_scroll: Prop<bool>,
    /// Assign `z-index` automatically.
    #[prop(optional, default = Prop::Static(true), into)]
    auto_z_index: Prop<bool>,
    /// Base `z-index` on top of the config value.
    #[prop(optional, into)]
    base_z_index: Prop<i32>,
    /// Header title.
    #[prop(optional, into)]
    header: OptProp<String>,
    /// `aria-label` attribute of the close button.
    #[prop(optional, into)]
    aria_close_label: OptProp<String>,
    /// Inline style of the mask.
    #[prop(optional, into)]
    mask_style: OptProp<String>,
    /// Inline style of the panel.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Extra class of the panel.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Close button icon instead of `TimesIcon`.
    #[prop(optional, into)]
    close_icon: Option<ViewFn>,
    /// Custom header content (rendered before the title).
    #[prop(optional, into)]
    header_content: Option<ViewFn>,
    /// Panel footer.
    #[prop(optional, into)]
    footer: Option<ViewFn>,
    /// Replaces the entire panel content (header, content, footer).
    #[prop(optional, into)]
    headless: Option<ViewFn>,
    /// Called after the panel is shown.
    #[prop(optional, into)]
    on_show: Option<Callback<()>>,
    /// Called when the panel starts closing.
    #[prop(optional, into)]
    on_hide: Option<Callback<()>>,
    /// Pass-through for parts.
    #[prop(optional)]
    pt: Option<DrawerPt>,
    /// Panel content.
    children: ChildrenFn,
) -> impl IntoView {
    let pt = Arc::new(pt.unwrap_or_default());
    let visible = visible.unwrap_or_default();
    let core = Arc::new(Core {
        visible,
        rendered: RwSignal::new(false),
        leaving: RwSignal::new(false),
        panel: NodeRef::new(),
        mask: NodeRef::new(),
        position,
        full_screen,
        modal,
        dismissible,
        close_on_escape,
        block_scroll,
        auto_z_index,
        base_z_index,
        cleanups: StoredValue::new_local(Vec::new()),
        z_index: StoredValue::new(None),
        scroll_blocked: StoredValue::new(false),
        was_open: StoredValue::new(false),
        on_show,
        on_hide,
        #[cfg(target_arch = "wasm32")]
        trap: StoredValue::new_local(None),
        #[cfg(target_arch = "wasm32")]
        timer: StoredValue::new_local(None),
    });
    {
        let core = core.clone();
        on_cleanup(move || core.teardown());
    }
    // Showing, closing and side effects (layer, focus trap, Escape) happen only in the browser.
    #[cfg(target_arch = "wasm32")]
    {
        let core = core.clone();
        Effect::new(move |_| {
            let shown = core.visible.get();
            // The panel enters the DOM after this effect, so we also track the reference to it.
            let mounted = core.panel.get().is_some();
            let was_open = core.was_open.get_value();
            if shown {
                if let Some(handle) = core.timer.try_update_value(Option::take).flatten() {
                    handle.clear();
                }
                // Shown again during the leave animation: the panel is still in the DOM, return it to the "open" state.
                if core.leaving.get_untracked() {
                    core.leaving.set(false);
                }
                if !core.rendered.get_untracked() {
                    core.rendered.set(true);
                }
                if mounted && !was_open {
                    core.was_open.set_value(true);
                    core.opened();
                }
            } else if was_open {
                core.was_open.set_value(false);
                core.begin_leave();
            } else if core.rendered.get_untracked() && !core.leaving.get_untracked() {
                // Closed before the panel appeared in the DOM: nothing to wait for.
                core.rendered.set(false);
            }
        });
    }

    let parts = Arc::new(Parts {
        closable,
        header,
        aria_close_label,
        mask_style,
        style,
        style_class,
        close_icon,
        header_content,
        footer,
        headless,
        children,
    });
    // The content is rebuilt inside `Portal` on every show: ready-made views are not `Send + Sync`.
    move || {
        let (core, pt, parts) = (core.clone(), pt.clone(), parts.clone());
        core.rendered.get().then(move || view! { <Portal>{build_panel(&core, &pt, &parts)}</Portal> })
    }
}

/// Properties needed only when rendering the panel.
struct Parts {
    closable: Prop<bool>,
    header: OptProp<String>,
    aria_close_label: OptProp<String>,
    mask_style: OptProp<String>,
    style: OptProp<String>,
    style_class: OptProp<String>,
    close_icon: Option<ViewFn>,
    header_content: Option<ViewFn>,
    footer: Option<ViewFn>,
    headless: Option<ViewFn>,
    children: ChildrenFn,
}

/// Mask and panel.
fn build_panel(core: &Arc<Core>, pt: &Arc<DrawerPt>, parts: &Arc<Parts>) -> AnyView {
    let (panel, mask_ref, leaving) = (core.panel, core.mask, core.leaving);

    let mask = core.modal.get_untracked().then(|| {
        let (full, pt) = (core.full_screen.clone(), pt.clone());
        let (dismissible, visible) = (core.dismissible.clone(), core.visible);
        let (position, modal) = (core.position.clone(), core.modal.clone());
        let class_pt = pt.clone();
        view! {
            <div
                node_ref=mask_ref
                class={move || class_pt.mask.merge_class(&mask_classes(full.get(), leaving.get()))}
                style={parts.mask_style.get()}
                data-p={move || data_p(position.get(), true, modal.get())}
                on:click=move |_| {
                    if dismissible.get_untracked() {
                        visible.set(false);
                    }
                }
                {..pt.mask.attrs()}
            ></div>
        }
    });

    let class = {
        let (pt, position, full_screen, style_class) =
            (pt.clone(), core.position.clone(), core.full_screen.clone(), parts.style_class.clone());
        move || {
            let (position, full) = (position.get(), full_screen.get());
            let mut cn = root_classes(position, full, !leaving.get());
            cn.push(' ');
            cn.push_str(&animation_class(position, full, leaving.get()));
            style_class.with(|c| {
                if let Some(c) = c.as_deref().filter(|c| !c.is_empty()) {
                    cn.push(' ');
                    cn.push_str(c);
                }
            });
            pt.root.merge_class(&cn)
        }
    };
    let root_style = {
        let user = parts.style.get().unwrap_or_default();
        let merged = pt.root.merge_style_parts(&[&user]);
        (!merged.is_empty()).then_some(merged)
    };

    let content = if let Some(headless) = parts.headless.clone() {
        headless.run().into_any()
    } else {
        let close_button = parts.closable.get_untracked().then(|| {
            let icon = parts.close_icon.clone().unwrap_or_else(|| ViewFn::from(|| view! { <TimesIcon /> }));
            let label = parts.aria_close_label.get().unwrap_or_else(|| prime_core::use_prime_config().locale.text("aria.close").to_owned());
            let visible = core.visible;
            view! {
                <Button
                    button_type="button"
                    text=true
                    rounded=true
                    severity=ButtonSeverity::Secondary
                    icon=icon
                    aria_label=Some(label)
                    style_class="p-drawer-close-button"
                    on_click=Callback::new(move |e: web_sys::MouseEvent| {
                        e.prevent_default();
                        visible.set(false);
                    })
                    pt=ButtonPt { root: pt.close_button.clone(), ..Default::default() }
                />
            }
        });
        let title = parts
            .header
            .get()
            .map(|title| view! { <div class={pt.title.merge_class("p-drawer-title")} {..pt.title.attrs()}>{title}</div> });
        let footer = parts.footer.clone().map(|footer| {
            view! {
                <div class={pt.footer.merge_class("p-drawer-footer")} data-pc-section="footer" {..pt.footer.attrs()}>
                    {footer.run()}
                </div>
            }
        });
        view! {
            <div class={pt.header.merge_class("p-drawer-header")} data-pc-section="header" {..pt.header.attrs()}>
                {parts.header_content.clone().map(|h| h.run())}
                {title}
                {close_button}
            </div>
            <div class={pt.content.merge_class("p-drawer-content")} data-pc-section="content" {..pt.content.attrs()}>
                {(parts.children)()}
            </div>
            {footer}
        }
        .into_any()
    };

    let (data_core, open_core, end_core) = (core.clone(), core.clone(), core.clone());
    view! {
        {mask}
        <div
            node_ref=panel
            class={class}
            style={root_style}
            role="complementary"
            data-p={move || data_p(data_core.position.get(), !data_core.leaving.get(), data_core.modal.get())}
            data-p-open={move || (!open_core.leaving.get()).then_some("true")}
            on:animationend=move |e: web_sys::AnimationEvent| animation_end(&end_core, &e)
            {..pt.root.attrs()}
        >
            {content}
        </div>
    }
    .into_any()
}

/// `animationend` of the panel root (not of nested elements) completes the leave.
fn animation_end(core: &Arc<Core>, event: &web_sys::AnimationEvent) {
    #[cfg(target_arch = "wasm32")]
    {
        let own = event.target().is_some_and(|t| event.current_target().is_some_and(|c| t == c));
        if own && core.leaving.get_untracked() {
            core.finish_leave();
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    let _ = (core, event);
}
