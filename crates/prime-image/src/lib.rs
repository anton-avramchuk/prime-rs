//! Image — port of `p-image` from PrimeNG 21.1.10.
//!
//! Root `div.p-image.p-component` with a regular `<img>`. With `preview`, a button
//! `button.p-image-preview-mask` (an eye icon on hover) lies over the image; a click on it opens a modal preview in a `Portal`:
//! the mask `div.p-image-mask.p-overlay-mask[role=dialog]`, an action toolbar (rotation by ±90°, zoom in steps of 0.1, close) and
//! the enlarged image `img.p-image-original`. The preview is closed by the button, a click on the mask and the `Escape` key; it blocks
//! page scrolling, keeps focus inside and takes a layer of the `Modal` group. The preview is rendered only in the browser.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_image::Image;
//!
//! let html = Owner::new().with(|| view! { <Image src="a.jpg" alt="Photo" width="250" preview=true /> }.to_html());
//! assert!(html.contains("p-image-preview-mask") && html.contains("src=\"a.jpg\"") && html.contains("width=\"250\""));
//! ```

mod state;

use leptos::prelude::*;
use prime_dom::Portal;
use prime_icons::{EyeIcon, RefreshIcon, SearchMinusIcon, SearchPlusIcon, TimesIcon, UndoIcon};
use prime_rs_core::{OptProp, Prop, Pt};
use std::sync::Arc;

pub use state::{
    ActionKind, ImageLoading, ZOOM_DEFAULT, ZOOM_MAX, ZOOM_MIN, ZOOM_STEP, action_classes, dimension_style, mask_should_close,
    preview_style, root_classes, transform, zoom_in, zoom_in_disabled, zoom_out, zoom_out_disabled,
};

#[cfg(target_arch = "wasm32")]
use leptos::leptos_dom::helpers::{TimeoutHandle, set_timeout_with_handle};
#[cfg(target_arch = "wasm32")]
use prime_dom::{FocusTrap, block_body_scroll, unblock_body_scroll};
#[cfg(target_arch = "wasm32")]
use prime_rs_core::{ZGroup, use_prime_config};
#[cfg(target_arch = "wasm32")]
use std::time::Duration;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

/// Fallback exit time: without styles or with animation disabled, `animationend` will never arrive.
#[cfg(target_arch = "wasm32")]
const LEAVE_FALLBACK: Duration = Duration::from_millis(400);
/// Focus delay after showing and after `Escape` (as in PrimeNG).
#[cfg(target_arch = "wasm32")]
const FOCUS_DELAY: Duration = Duration::from_millis(25);

/// Context of the `preview_content` view.
#[derive(Clone)]
pub struct ImagePreviewContext {
    /// Class of the enlarged image (`p-image-original` and the animation state).
    pub class: String,
    /// Style with `transform` (rotation and scale); it changes on toolbar actions.
    pub style: Signal<String>,
    /// Called on a click on the image: such a click does not close the preview.
    pub preview_callback: Callback<()>,
}

/// Pass-through of the Image parts.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ImagePt {
    /// Root element.
    pub root: Pt,
    /// Main image.
    pub image: Pt,
    /// Preview button.
    pub preview_mask: Pt,
    /// Preview button icon (class only).
    pub preview_icon: Pt,
    /// Preview mask.
    pub mask: Pt,
    /// Action toolbar.
    pub toolbar: Pt,
    /// Rotate right button.
    pub rotate_right_button: Pt,
    /// Rotate left button.
    pub rotate_left_button: Pt,
    /// Zoom out button.
    pub zoom_out_button: Pt,
    /// Zoom in button.
    pub zoom_in_button: Pt,
    /// Close button.
    pub close_button: Pt,
    /// Enlarged image.
    pub original: Pt,
}

/// State shared by the preview handlers.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
struct Core {
    /// Preview requested.
    open: RwSignal<bool>,
    /// The mask is in the DOM (it stays in the DOM while the exit animation plays).
    rendered: RwSignal<bool>,
    /// The exit animation is running.
    leaving: RwSignal<bool>,
    rotate: RwSignal<i32>,
    scale: RwSignal<f64>,
    /// The last click came from the enlarged image: it does not close the mask.
    preview_click: StoredValue<bool>,
    /// The layer assigned to the mask.
    z_index: RwSignal<Option<i32>>,
    scroll_blocked: StoredValue<bool>,
    was_open: StoredValue<bool>,
    mask: NodeRef<leptos::html::Div>,
    close_button: NodeRef<leptos::html::Button>,
    preview_button: NodeRef<leptos::html::Button>,
    on_show: Option<Callback<()>>,
    on_hide: Option<Callback<()>>,
    #[cfg(target_arch = "wasm32")]
    trap: StoredValue<Option<FocusTrap>, LocalStorage>,
    #[cfg(target_arch = "wasm32")]
    escape: StoredValue<Option<Box<dyn FnOnce()>>, LocalStorage>,
    #[cfg(target_arch = "wasm32")]
    timer: StoredValue<Option<TimeoutHandle>, LocalStorage>,
}

impl Core {
    /// Removes the `Escape` listener and the focus trap.
    fn release_input(&self) {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(remove) = self.escape.try_update_value(Option::take).flatten() {
                remove();
            }
            self.trap.try_update_value(Option::take);
        }
    }

    /// Releases the layer and the scroll lock.
    fn release_layer(&self) {
        if let Some(z) = self.z_index.try_update(Option::take).flatten() {
            prime_rs_core::zindex::revert(z);
        }
        if self.scroll_blocked.try_update_value(std::mem::take).unwrap_or(false) {
            #[cfg(target_arch = "wasm32")]
            unblock_body_scroll();
        }
    }

    /// Full release when the component is unmounted.
    fn teardown(&self) {
        self.release_input();
        self.release_layer();
        #[cfg(target_arch = "wasm32")]
        if let Some(handle) = self.timer.try_update_value(Option::take).flatten() {
            handle.clear();
        }
    }

    /// The preview was requested to close.
    fn close(&self) {
        self.open.set(false);
    }

    /// Focus the preview button (after `Escape`).
    #[cfg(target_arch = "wasm32")]
    fn focus_preview_button(self: &Arc<Self>) {
        let this = self.clone();
        leptos::prelude::set_timeout(
            move || {
                if let Some(button) = this.preview_button.get_untracked() {
                    let _ = button.focus();
                }
            },
            FOCUS_DELAY,
        );
    }

    /// The mask has just entered the DOM: layer, scroll lock, focus trap, `Escape`, focus, `on_show`.
    ///
    /// Reopening during exit calls this again while the layer and lock are still held: we do not take them a second time.
    #[cfg(target_arch = "wasm32")]
    fn opened(self: &Arc<Self>) {
        if self.z_index.get_untracked().is_none() {
            let base = use_prime_config().z_index.base(ZGroup::Modal);
            self.z_index.set(Some(prime_rs_core::zindex::generate(ZGroup::Modal, base)));
        }
        if !self.scroll_blocked.get_value() {
            block_body_scroll();
            self.scroll_blocked.set_value(true);
        }
        if let Some(mask) = self.mask.get_untracked() {
            let mask: web_sys::HtmlElement = mask.unchecked_into();
            self.trap.set_value(FocusTrap::attach(&mask));
        }
        let this = self.clone();
        let handle = window_event_listener(leptos::ev::keydown, move |e: web_sys::KeyboardEvent| {
            if e.key() != "Escape" || !this.open.get_untracked() {
                return;
            }
            // Another layer (for example, a dialog) may lie over the preview: only the topmost one is closed.
            let own = this.z_index.get_untracked().unwrap_or(0);
            if own >= prime_rs_core::zindex::current() {
                e.prevent_default();
                this.close();
                this.focus_preview_button();
            }
        });
        self.escape.set_value(Some(Box::new(move || handle.remove())));
        let this = self.clone();
        leptos::prelude::set_timeout(
            move || {
                if let Some(button) = this.close_button.get_untracked() {
                    let _ = button.focus();
                }
            },
            FOCUS_DELAY,
        );
        if let Some(cb) = self.on_show {
            cb.run(());
        }
    }

    /// The preview is closed: the exit animation plays.
    #[cfg(target_arch = "wasm32")]
    fn begin_leave(self: &Arc<Self>) {
        self.release_input();
        self.leaving.set(true);
        let this = self.clone();
        if let Ok(handle) = set_timeout_with_handle(move || this.finish_leave(), LEAVE_FALLBACK) {
            self.timer.set_value(Some(handle));
        }
    }

    /// The exit animation has finished: the mask is removed from the DOM, the layer and lock are released, rotation and scale are reset.
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
        self.rotate.set(0);
        self.scale.set(ZOOM_DEFAULT);
        if let Some(cb) = self.on_hide {
            cb.run(());
        }
    }
}

/// An image with an optional preview in a modal layer.
///
/// Properties accept a value or a signal. Views (`image`, `indicator`, `preview_content`, toolbar icons) are rebuilt
/// on every show.
#[component]
pub fn Image(
    /// Image address.
    #[prop(optional, into)]
    src: OptProp<String>,
    /// Image `srcset`.
    #[prop(optional, into)]
    srcset: OptProp<String>,
    /// Image `sizes`.
    #[prop(optional, into)]
    sizes: OptProp<String>,
    /// Image address in the preview (`src` by default).
    #[prop(optional, into)]
    preview_image_src: OptProp<String>,
    /// Image `srcset` in the preview.
    #[prop(optional, into)]
    preview_image_srcset: OptProp<String>,
    /// Image `sizes` in the preview.
    #[prop(optional, into)]
    preview_image_sizes: OptProp<String>,
    /// The `alt` attribute.
    #[prop(optional, into)]
    alt: OptProp<String>,
    /// The `width` attribute (a number is in pixels; it also sets the preview button size).
    #[prop(optional, into)]
    width: OptProp<String>,
    /// The `height` attribute.
    #[prop(optional, into)]
    height: OptProp<String>,
    /// The `loading` attribute.
    #[prop(optional, into)]
    loading: OptProp<ImageLoading>,
    /// Enables the preview.
    #[prop(optional, into)]
    preview: Prop<bool>,
    /// Image class.
    #[prop(optional, into)]
    image_class: OptProp<String>,
    /// Inline style of the image.
    #[prop(optional, into)]
    image_style: OptProp<String>,
    /// Additional class of the root.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Inline style of the root.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Pass-through of the parts.
    #[prop(optional)]
    pt: Option<ImagePt>,
    /// Custom image instead of `img`; receives the load error callback.
    #[prop(optional)]
    image: Option<Callback<Callback<web_sys::Event>, AnyView>>,
    /// Custom preview button icon instead of the eye.
    #[prop(optional, into)]
    indicator: Option<ViewFn>,
    /// Custom enlarged image instead of `img.p-image-original`.
    #[prop(optional)]
    preview_content: Option<Callback<ImagePreviewContext, AnyView>>,
    /// Rotate right icon.
    #[prop(optional, into)]
    rotate_right_icon: Option<ViewFn>,
    /// Rotate left icon.
    #[prop(optional, into)]
    rotate_left_icon: Option<ViewFn>,
    /// Zoom out icon.
    #[prop(optional, into)]
    zoom_out_icon: Option<ViewFn>,
    /// Zoom in icon.
    #[prop(optional, into)]
    zoom_in_icon: Option<ViewFn>,
    /// Close icon.
    #[prop(optional, into)]
    close_icon: Option<ViewFn>,
    /// The preview is shown.
    #[prop(optional)]
    on_show: Option<Callback<()>>,
    /// The preview is hidden (after the exit animation).
    #[prop(optional)]
    on_hide: Option<Callback<()>>,
    /// Main image load error.
    #[prop(optional)]
    on_image_error: Option<Callback<web_sys::Event>>,
) -> impl IntoView {
    let pt = Arc::new(pt.unwrap_or_default());
    let core = Arc::new(Core {
        open: RwSignal::new(false),
        rendered: RwSignal::new(false),
        leaving: RwSignal::new(false),
        rotate: RwSignal::new(0),
        scale: RwSignal::new(ZOOM_DEFAULT),
        preview_click: StoredValue::new(false),
        z_index: RwSignal::new(None),
        scroll_blocked: StoredValue::new(false),
        was_open: StoredValue::new(false),
        mask: NodeRef::new(),
        close_button: NodeRef::new(),
        preview_button: NodeRef::new(),
        on_show,
        on_hide,
        #[cfg(target_arch = "wasm32")]
        trap: StoredValue::new_local(None),
        #[cfg(target_arch = "wasm32")]
        escape: StoredValue::new_local(None),
        #[cfg(target_arch = "wasm32")]
        timer: StoredValue::new_local(None),
    });
    {
        let core = core.clone();
        on_cleanup(move || core.teardown());
    }
    // Showing, exit and side effects (layer, scrolling, focus trap, `Escape`) happen only in the browser.
    #[cfg(target_arch = "wasm32")]
    {
        let core = core.clone();
        Effect::new(move |_| {
            let shown = core.open.get();
            // The mask enters the DOM after this effect, so we also track the reference to it.
            let mounted = core.mask.get().is_some();
            let was_open = core.was_open.get_value();
            if shown {
                if let Some(handle) = core.timer.try_update_value(Option::take).flatten() {
                    handle.clear();
                }
                // Reopening during exit: the mask is still in the DOM, so we return it to the "open" state.
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
                // Closed before the mask appeared in the DOM: nothing to wait for.
                core.rendered.set(false);
            }
        });
    }

    let on_error = move |e: web_sys::Event| {
        if let Some(cb) = on_image_error {
            cb.run(e);
        }
    };
    let preview_root = preview.clone();
    let root_class = {
        let pt = pt.clone();
        move || {
            let extra = style_class.with(|c| c.clone());
            let base = root_classes(preview_root.get(), extra.as_deref());
            pt.root.merge_class(&base).into_owned()
        }
    };
    let root_style = {
        let pt = pt.clone();
        move || {
            let merged = style.with(|s| pt.root.merge_style(s.as_deref().unwrap_or_default()));
            (!merged.is_empty()).then_some(merged)
        }
    };
    let img_class = {
        let pt = pt.clone();
        move || {
            let merged = image_class.with(|c| pt.image.merge_class(c.as_deref().unwrap_or_default()));
            (!merged.is_empty()).then(|| merged.into_owned())
        }
    };
    let img_style = {
        let pt = pt.clone();
        move || {
            let merged = image_style.with(|s| pt.image.merge_style(s.as_deref().unwrap_or_default()));
            (!merged.is_empty()).then_some(merged)
        }
    };
    let (width_img, height_img) = (width.clone(), height.clone());
    let (src_img, srcset_img, sizes_img) = (src.clone(), srcset.clone(), sizes.clone());
    let image_view = match image {
        Some(custom) => custom.run(Callback::new(on_error)).into_any(),
        None => {
            let attrs = pt.image.attrs();
            view! {
                <img
                    src={move || src_img.get()}
                    srcset={move || srcset_img.get()}
                    sizes={move || sizes_img.get()}
                    alt={move || alt.get()}
                    width={move || width_img.get()}
                    height={move || height_img.get()}
                    loading={move || loading.get().map(ImageLoading::as_str)}
                    class={img_class}
                    style={img_style}
                    on:error={move |e: web_sys::ErrorEvent| on_error(e.into())}
                    {..attrs}
                />
            }
            .into_any()
        }
    };

    let parts = Arc::new(Parts {
        src,
        srcset,
        sizes,
        preview_image_src,
        preview_image_srcset,
        preview_image_sizes,
        preview_content,
        rotate_right_icon,
        rotate_left_icon,
        zoom_out_icon,
        zoom_in_icon,
        close_icon,
    });

    let preview_button = {
        let (core, pt) = (core.clone(), pt.clone());
        move || {
            preview.get().then(|| {
                let icon = match indicator.clone() {
                    Some(custom) => custom.run(),
                    None => {
                        let class = pt.preview_icon.merge_class("p-image-preview-icon").into_owned();
                        view! { <EyeIcon class=class /> }.into_any()
                    }
                };
                let label = use_prime_config_text("aria.zoomImage");
                let (w, h) = (width.clone(), height.clone());
                let size = move || w.with(|w| h.with(|h| dimension_style(w.as_deref(), h.as_deref())));
                let open = {
                    let core = core.clone();
                    move |_| {
                        core.open.set(true);
                        #[cfg(not(target_arch = "wasm32"))]
                        let _ = &core;
                    }
                };
                let attrs = pt.preview_mask.attrs();
                let class = pt.preview_mask.merge_class("p-image-preview-mask").into_owned();
                view! {
                    <button type="button" class=class aria-label=label style=size node_ref=core.preview_button on:click={open} {..attrs}>
                        {icon}
                    </button>
                }
            })
        }
    };

    let preview_dialog = {
        let (core, pt, parts) = (core.clone(), pt.clone(), parts.clone());
        move || {
            let (core, pt, parts) = (core.clone(), pt.clone(), parts.clone());
            core.rendered.get().then(move || view! { <Portal>{build_preview(&core, &pt, &parts)}</Portal> })
        }
    };

    let root_attrs = pt.root.attrs();
    view! {
        <div class={root_class} style={root_style} {..root_attrs}>
            {image_view}
            {preview_button}
            {preview_dialog}
        </div>
    }
}

/// Translation for the label: in SSR it is taken from the configuration the same way as in the browser.
fn use_prime_config_text(key: &str) -> String {
    prime_rs_core::use_prime_config().locale.text(key).to_owned()
}

/// Properties needed only when rendering the preview.
struct Parts {
    src: OptProp<String>,
    srcset: OptProp<String>,
    sizes: OptProp<String>,
    preview_image_src: OptProp<String>,
    preview_image_srcset: OptProp<String>,
    preview_image_sizes: OptProp<String>,
    preview_content: Option<Callback<ImagePreviewContext, AnyView>>,
    rotate_right_icon: Option<ViewFn>,
    rotate_left_icon: Option<ViewFn>,
    zoom_out_icon: Option<ViewFn>,
    zoom_in_icon: Option<ViewFn>,
    close_icon: Option<ViewFn>,
}

/// Preview mask with the action toolbar and the enlarged image.
fn build_preview(core: &Arc<Core>, pt: &Arc<ImagePt>, parts: &Arc<Parts>) -> impl IntoView + use<> {
    let (leaving, z_index, rotate, scale) = (core.leaving, core.z_index, core.rotate, core.scale);
    let mask_class = {
        let pt = pt.clone();
        move || {
            let state = if leaving.get() { "p-overlay-mask-leave-active" } else { "p-overlay-mask-enter-active" };
            pt.mask.merge_class(&format!("p-image-mask p-overlay-mask {state}")).into_owned()
        }
    };
    let mask_style = {
        let pt = pt.clone();
        move || {
            let z = z_index.get().map(|z| format!("z-index: {z}")).unwrap_or_default();
            let merged = pt.mask.merge_style(&z);
            (!merged.is_empty()).then_some(merged)
        }
    };
    let on_mask_click = {
        let core = core.clone();
        move |_| {
            if mask_should_close(core.preview_click.get_value()) {
                core.close();
            }
            core.preview_click.set_value(false);
        }
    };
    let on_animation_end = {
        let core = core.clone();
        move |e: web_sys::AnimationEvent| animation_end(&core, &e)
    };

    let zoom_out_off = Signal::derive(move || zoom_out_disabled(scale.get()));
    let zoom_in_off = Signal::derive(move || zoom_in_disabled(scale.get()));
    let never = Signal::stored(false);
    let tool =
        |kind, off, key: &'static str, icon: ViewFn, pt: &Pt, act: Callback<()>, node| action_button(kind, off, key, icon, pt, act, node);
    let buttons = view! {
        {tool(
            ActionKind::RotateRight,
            never,
            "aria.rotateRight",
            parts.rotate_right_icon.clone().unwrap_or_else(|| ViewFn::from(|| view! { <RefreshIcon /> })),
            &pt.rotate_right_button,
            Callback::new(move |()| rotate.update(|r| *r += 90)),
            None,
        )}
        {tool(
            ActionKind::RotateLeft,
            never,
            "aria.rotateLeft",
            parts.rotate_left_icon.clone().unwrap_or_else(|| ViewFn::from(|| view! { <UndoIcon /> })),
            &pt.rotate_left_button,
            Callback::new(move |()| rotate.update(|r| *r -= 90)),
            None,
        )}
        {tool(
            ActionKind::ZoomOut,
            zoom_out_off,
            "aria.zoomOut",
            parts.zoom_out_icon.clone().unwrap_or_else(|| ViewFn::from(|| view! { <SearchMinusIcon /> })),
            &pt.zoom_out_button,
            Callback::new(move |()| scale.update(|s| *s = zoom_out(*s))),
            None,
        )}
        {tool(
            ActionKind::ZoomIn,
            zoom_in_off,
            "aria.zoomIn",
            parts.zoom_in_icon.clone().unwrap_or_else(|| ViewFn::from(|| view! { <SearchPlusIcon /> })),
            &pt.zoom_in_button,
            Callback::new(move |()| scale.update(|s| *s = zoom_in(*s))),
            None,
        )}
        {tool(
            ActionKind::Close,
            never,
            "aria.close",
            parts.close_icon.clone().unwrap_or_else(|| ViewFn::from(|| view! { <TimesIcon /> })),
            &pt.close_button,
            {
                let core = core.clone();
                Callback::new(move |()| core.close())
            },
            Some(core.close_button),
        )}
    };

    let toolbar_attrs = pt.toolbar.attrs();
    let toolbar_class = pt.toolbar.merge_class("p-image-toolbar").into_owned();
    let toolbar_style = pt.toolbar.has_style().then(|| pt.toolbar.merge_style(""));

    let original = build_original(core, pt, parts);
    let mask_attrs = pt.mask.attrs();
    view! {
        <div
            node_ref=core.mask
            class=mask_class
            style=mask_style
            role="dialog"
            aria-modal="true"
            on:click=on_mask_click
            on:animationend={on_animation_end}
            {..mask_attrs}
        >
            <div class=toolbar_class style=toolbar_style on:click=|e: web_sys::MouseEvent| e.stop_propagation() {..toolbar_attrs}>
                {buttons}
            </div>
            {original}
        </div>
    }
}

/// Enlarged image (or the `preview_content` view).
fn build_original(core: &Arc<Core>, pt: &Arc<ImagePt>, parts: &Arc<Parts>) -> impl IntoView + use<> {
    let (leaving, rotate, scale) = (core.leaving, core.rotate, core.scale);
    let state = move || if leaving.get() { "p-image-original-leave-active" } else { "p-image-original-enter-active" };
    let style = Signal::derive(move || preview_style(rotate.get(), scale.get()));
    let mark = {
        let core = core.clone();
        Callback::new(move |()| core.preview_click.set_value(true))
    };
    if let Some(custom) = parts.preview_content {
        let class = pt.original.merge_class(&format!("p-image-original {}", state())).into_owned();
        return custom.run(ImagePreviewContext { class, style, preview_callback: mark }).into_any();
    }
    let class = {
        let pt = pt.clone();
        move || pt.original.merge_class(&format!("p-image-original {}", state())).into_owned()
    };
    let p = parts.clone();
    let (src, srcset, sizes) = (
        move || p.preview_image_src.get().or_else(|| p.src.get()),
        {
            let p = parts.clone();
            move || p.preview_image_srcset.get().or_else(|| p.srcset.get())
        },
        {
            let p = parts.clone();
            move || p.preview_image_sizes.get().or_else(|| p.sizes.get())
        },
    );
    let attrs = pt.original.attrs();
    let style_attr = {
        let pt = pt.clone();
        move || pt.original.merge_style(&style.get())
    };
    view! { <img src=src srcset=srcset sizes=sizes class=class style=style_attr on:click=move |_| mark.run(()) {..attrs} /> }.into_any()
}

/// Toolbar action button.
fn action_button(
    kind: ActionKind,
    disabled: Signal<bool>,
    label_key: &'static str,
    icon: ViewFn,
    pt: &Pt,
    act: Callback<()>,
    node: Option<NodeRef<leptos::html::Button>>,
) -> impl IntoView + use<> {
    let label = use_prime_config_text(label_key);
    let base = pt.clone();
    let class = {
        let base = base.clone();
        move || base.merge_class(action_classes(kind, disabled.get())).into_owned()
    };
    let attrs = pt.attrs();
    let node = node.unwrap_or_default();
    view! {
        <button type="button" node_ref=node class=class aria-label=label disabled=move || disabled.get() on:click=move |_| act.run(()) {..attrs}>
            {icon.run()}
        </button>
    }
}

/// `animationend` of the mask itself (not of nested elements) finishes the exit.
fn animation_end(core: &Arc<Core>, e: &web_sys::AnimationEvent) {
    #[cfg(target_arch = "wasm32")]
    {
        let own = e.target().zip(e.current_target()).is_some_and(|(t, c)| t == c);
        if own && core.leaving.get_untracked() {
            core.finish_leave();
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    let _ = (core, e);
}
