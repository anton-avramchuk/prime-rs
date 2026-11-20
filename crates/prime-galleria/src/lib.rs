//! Galleria: a port of `p-galleria` from PrimeNG 21.1.10.
//!
//! Image gallery: a large frame, scrollable thumbnails, indicators, captions, automatic frame advance
//! and fullscreen mode (a mask in `body`). The thumbnail window is computed by the pure [`model`].
//!
//! ```
//! use leptos::prelude::*;
//! use prime_galleria::{Galleria, GalleriaImage};
//!
//! let images = vec![GalleriaImage::new("a.jpg").thumbnail("a-small.jpg").alt("A")];
//! let html = view! { <Galleria value=images /> }.to_html();
//! assert!(html.contains("p-galleria-item") && html.contains("a.jpg"));
//! ```

#![recursion_limit = "512"]

pub mod model;

use leptos::prelude::*;
pub use model::{GalleriaPosition, GalleriaResponsive};
use model::{
    effective_num_visible, item_backward_disabled, item_forward_disabled, next_index, prev_index, root_classes, shifted_items,
    thumb_backward_disabled, thumb_forward_disabled, thumbnail_item_classes, thumbnails_transform,
};
use prime_dom::Portal;
use prime_icons::{ChevronDownIcon, ChevronLeftIcon, ChevronRightIcon, ChevronUpIcon, TimesIcon};
use prime_rs_core::{OptProp, Prop, Pt};
use std::sync::Arc;

#[cfg(target_arch = "wasm32")]
use leptos::leptos_dom::helpers::WindowListenerHandle;
#[cfg(target_arch = "wasm32")]
use leptos::leptos_dom::helpers::{IntervalHandle, TimeoutHandle, set_interval_with_handle, set_timeout_with_handle};
#[cfg(target_arch = "wasm32")]
use prime_dom::{FocusTrap, block_body_scroll, unblock_body_scroll};
#[cfg(target_arch = "wasm32")]
use prime_rs_core::{ZGroup, use_prime_config};
#[cfg(target_arch = "wasm32")]
use std::time::Duration;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

/// Fallback leave time: the animation takes 300 ms, but without styles `animationend` will not arrive.
#[cfg(target_arch = "wasm32")]
const LEAVE_FALLBACK: Duration = Duration::from_millis(700);

/// Gallery image.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GalleriaImage {
    /// Large image URL.
    pub item_image_src: String,
    /// Thumbnail URL (the large image by default).
    pub thumbnail_image_src: Option<String>,
    /// `alt` attribute.
    pub alt: String,
    /// Title (for the caption).
    pub title: String,
}

impl GalleriaImage {
    /// Image with the URL of the large frame.
    pub fn new(item_image_src: impl Into<String>) -> Self {
        Self { item_image_src: item_image_src.into(), ..Self::default() }
    }

    /// Thumbnail URL.
    pub fn thumbnail(mut self, src: impl Into<String>) -> Self {
        self.thumbnail_image_src = Some(src.into());
        self
    }

    /// `alt` text.
    pub fn alt(mut self, alt: impl Into<String>) -> Self {
        self.alt = alt.into();
        self
    }

    /// Title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }
}

/// Pass-through parts of [`Galleria`].
#[derive(Clone, Debug, Default)]
pub struct GalleriaPt {
    /// Root.
    pub root: Pt,
    /// Fullscreen mode mask.
    pub mask: Pt,
    /// Close button.
    pub close_button: Pt,
    /// Header.
    pub header: Pt,
    /// Content.
    pub content: Pt,
    /// Footer.
    pub footer: Pt,
    /// Frames container.
    pub items_container: Pt,
    /// Wrapper of the frame and buttons.
    pub items: Pt,
    /// Frame "backward" button.
    pub prev_button: Pt,
    /// Frame.
    pub item: Pt,
    /// Frame "forward" button.
    pub next_button: Pt,
    /// Caption.
    pub caption: Pt,
    /// Indicators list.
    pub indicator_list: Pt,
    /// Indicator.
    pub indicator: Pt,
    /// Indicator button.
    pub indicator_button: Pt,
    /// Thumbnails.
    pub thumbnails: Pt,
    /// Thumbnails content.
    pub thumbnail_content: Pt,
    /// Thumbnails "backward" button.
    pub thumbnail_prev_button: Pt,
    /// Thumbnails window.
    pub thumbnails_viewport: Pt,
    /// Thumbnails strip.
    pub thumbnail_items: Pt,
    /// Strip item.
    pub thumbnail_item: Pt,
    /// Thumbnail.
    pub thumbnail: Pt,
    /// Thumbnails "forward" button.
    pub thumbnail_next_button: Pt,
}

/// State shared by the handlers.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
struct Core {
    value: Prop<Vec<GalleriaImage>>,
    active: RwSignal<usize>,
    visible: RwSignal<bool>,
    rendered: RwSignal<bool>,
    leaving: RwSignal<bool>,
    slide_show_active: RwSignal<bool>,
    num_visible: Prop<usize>,
    /// Number of visible thumbnails taking responsive options into account.
    num_visible_now: RwSignal<usize>,
    responsive_options: Vec<GalleriaResponsive>,
    items_hidden: RwSignal<bool>,
    transition_on: RwSignal<bool>,
    /// The thumbnail reachable by Tab (after arrow keys); `None` means the active one.
    tab_index_override: RwSignal<Option<usize>>,
    full_screen: bool,
    show_item_navigators: Prop<bool>,
    show_thumbnail_navigators: Prop<bool>,
    show_item_navigators_on_hover: Prop<bool>,
    change_item_on_indicator_hover: Prop<bool>,
    circular: Prop<bool>,
    auto_play: Prop<bool>,
    should_stop_autoplay_by_click: Prop<bool>,
    transition_interval: Prop<u32>,
    show_thumbnails: Prop<bool>,
    thumbnails_position: Prop<GalleriaPosition>,
    vertical_height: Prop<String>,
    show_indicators: Prop<bool>,
    show_indicators_on_item: Prop<bool>,
    indicators_position: Prop<GalleriaPosition>,
    base_z_index: Prop<i32>,
    id: String,
    mask_class: OptProp<String>,
    container_class: OptProp<String>,
    container_style: OptProp<String>,
    style: OptProp<String>,
    style_class: OptProp<String>,
    item: Option<Callback<GalleriaImage, AnyView>>,
    thumbnail: Option<Callback<GalleriaImage, AnyView>>,
    caption: Option<Callback<GalleriaImage, AnyView>>,
    indicator: Option<Callback<usize, AnyView>>,
    header: Option<ViewFn>,
    footer: Option<ViewFn>,
    mask: NodeRef<leptos::html::Div>,
    panel: NodeRef<leptos::html::Div>,
    thumb_items: NodeRef<leptos::html::Div>,
    cleanups: StoredValue<Vec<Box<dyn FnOnce()>>, LocalStorage>,
    z_index: StoredValue<Option<i32>>,
    scroll_blocked: StoredValue<bool>,
    was_open: StoredValue<bool>,
    #[cfg(target_arch = "wasm32")]
    trap: StoredValue<Option<FocusTrap>, LocalStorage>,
    #[cfg(target_arch = "wasm32")]
    timer: StoredValue<Option<TimeoutHandle>, LocalStorage>,
    #[cfg(target_arch = "wasm32")]
    interval: StoredValue<Option<IntervalHandle>, LocalStorage>,
    #[cfg(target_arch = "wasm32")]
    resize: StoredValue<Option<WindowListenerHandle>, LocalStorage>,
}

impl Core {
    fn len(&self) -> usize {
        self.value.with(Vec::len)
    }

    fn nv(&self) -> usize {
        effective_num_visible(self.num_visible_now.get(), self.len())
    }

    fn shifted(&self) -> i32 {
        shifted_items(self.len(), self.nv(), self.active.get())
    }

    fn vertical(&self) -> bool {
        self.show_thumbnails.get() && self.thumbnails_position.get().is_vertical()
    }

    /// Changes the active index: the thumbnail strip is animated, hidden thumbnails become visible again.
    fn set_active(&self, index: usize) {
        if self.active.get_untracked() != index {
            self.active.set(index);
            self.items_hidden.set(false);
            self.transition_on.set(true);
        }
        self.tab_index_override.set(None);
    }

    /// Stops auto-advance (`stopSlideShow`): with `auto_play` it has no effect without `should_stop_autoplay_by_click`.
    fn stop_slide_show(&self) {
        if self.auto_play.get_untracked() && !self.should_stop_autoplay_by_click.get_untracked() {
            return;
        }
        self.halt_slide_show();
    }

    fn halt_slide_show(&self) {
        #[cfg(target_arch = "wasm32")]
        if let Some(handle) = self.interval.try_update_value(Option::take).flatten() {
            handle.clear();
        }
        self.slide_show_active.try_set(false);
    }

    #[cfg(target_arch = "wasm32")]
    fn start_slide_show(self: &Arc<Self>) {
        self.halt_slide_show();
        let this = self.clone();
        let handle = set_interval_with_handle(
            move || {
                if this.active.is_disposed() {
                    return;
                }
                let next = next_index(this.active.get_untracked(), this.len(), this.circular.get_untracked());
                this.set_active(next);
            },
            Duration::from_millis(u64::from(self.transition_interval.get_untracked())),
        );
        if let Ok(handle) = handle {
            self.interval.set_value(Some(handle));
            self.slide_show_active.set(true);
        }
    }

    fn step_item(&self, forward: bool, e: &web_sys::MouseEvent) {
        self.stop_slide_show();
        let (active, len, circular) = (self.active.get_untracked(), self.len(), self.circular.get_untracked());
        self.set_active(if forward { next_index(active, len, circular) } else { prev_index(active, len, circular) });
        if e.cancelable() {
            e.stop_propagation();
            e.prevent_default();
        }
    }

    fn step_thumbnails(&self, forward: bool, e: &web_sys::MouseEvent) {
        self.stop_slide_show();
        let (active, len, circular) = (self.active.get_untracked(), self.len(), self.circular.get_untracked());
        self.set_active(if forward { next_index(active, len, circular) } else { prev_index(active, len, circular) });
        if e.cancelable() {
            e.prevent_default();
        }
    }

    fn select(&self, index: usize) {
        self.stop_slide_show();
        if index < self.len() {
            self.set_active(index);
        }
    }

    /// Thumbnail keys: `Enter`/`Space` select, arrows, `Home`, `End` move focus, `Tab` returns to the active one.
    fn thumb_keydown(&self, index: usize, e: &web_sys::KeyboardEvent) {
        let len = self.len();
        let focus_to = |target: usize| {
            self.tab_index_override.set(Some(target));
            self.focus_thumbnail(target);
        };
        match e.code().as_str() {
            "Enter" | "Space" => {
                self.select(index);
                e.prevent_default();
            }
            "ArrowRight" => focus_to((index + 1).min(len.saturating_sub(1))),
            "ArrowLeft" => focus_to(index.saturating_sub(1)),
            "Home" => {
                focus_to(0);
                e.prevent_default();
            }
            "End" => {
                focus_to(len.saturating_sub(1));
                e.prevent_default();
            }
            "ArrowUp" | "ArrowDown" => e.prevent_default(),
            "Tab" => self.tab_index_override.set(None),
            _ => {}
        }
    }

    fn focus_thumbnail(&self, index: usize) {
        #[cfg(target_arch = "wasm32")]
        if let Some(items) = self.thumb_items.get_untracked()
            && let Some(item) = items.children().item(index as u32)
            && let Some(inner) = item.first_element_child()
            && let Ok(inner) = inner.dyn_into::<web_sys::HtmlElement>()
        {
            let _ = inner.set_attribute("tabindex", "0");
            let _ = inner.focus();
        }
        #[cfg(not(target_arch = "wasm32"))]
        let _ = index;
    }

    /// Removes listeners, the focus trap, the layer, the scroll lock, auto-advance and timers.
    fn teardown(&self) {
        for cleanup in self.cleanups.try_update_value(std::mem::take).unwrap_or_default() {
            cleanup();
        }
        self.release_layer();
        #[cfg(target_arch = "wasm32")]
        {
            self.trap.try_update_value(|t| t.take());
            if let Some(handle) = self.timer.try_update_value(Option::take).flatten() {
                handle.clear();
            }
            if let Some(handle) = self.interval.try_update_value(Option::take).flatten() {
                handle.clear();
            }
            if let Some(handle) = self.resize.try_update_value(Option::take).flatten() {
                handle.remove();
            }
        }
    }

    fn release_layer(&self) {
        if let Some(z) = self.z_index.try_update_value(Option::take).flatten() {
            prime_rs_core::zindex::revert(z);
        }
        if self.scroll_blocked.try_update_value(std::mem::take).unwrap_or(false) {
            #[cfg(target_arch = "wasm32")]
            unblock_body_scroll();
        }
    }

    /// The mask has just entered the DOM: layer, scroll lock, focus trap, focus on the close button.
    #[cfg(target_arch = "wasm32")]
    fn opened(self: &Arc<Self>) {
        let (Some(panel), Some(mask)) = (self.panel.get_untracked(), self.mask.get_untracked()) else { return };
        let (panel, mask): (web_sys::HtmlElement, web_sys::HtmlElement) = (panel.unchecked_into(), mask.unchecked_into());
        let base = use_prime_config().z_index.base(ZGroup::Modal) + self.base_z_index.get_untracked();
        let z = prime_rs_core::zindex::generate(ZGroup::Modal, base);
        self.z_index.set_value(Some(z));
        let _ = mask.style().set_property("z-index", &z.to_string());
        block_body_scroll();
        self.scroll_blocked.set_value(true);
        self.trap.set_value(FocusTrap::attach(&panel));
        let close = panel.query_selector("[data-pc-section=\"closebutton\"]").ok().flatten();
        if let Some(close) = close.and_then(|c| c.dyn_into::<web_sys::HtmlElement>().ok()) {
            set_timeout_with_handle(
                move || {
                    prime_dom::focus_no_scroll(&close);
                },
                Duration::from_millis(25),
            )
            .ok();
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn begin_leave(self: &Arc<Self>) {
        self.trap.try_update_value(|t| t.take());
        self.leaving.set(true);
        let this = self.clone();
        if let Ok(handle) = set_timeout_with_handle(move || this.finish_leave(), LEAVE_FALLBACK) {
            self.timer.set_value(Some(handle));
        }
    }

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

/// Image gallery.
///
/// Every property accepts a value or a signal; `active_index` and `visible` are two-way signals. In
/// `full_screen` mode the mask is rendered into `body` and is not sent by the server before hydration.
#[component]
pub fn Galleria(
    /// Images.
    #[prop(optional, into)]
    value: Prop<Vec<GalleriaImage>>,
    /// Two-way signal of the active frame index.
    #[prop(optional)]
    active_index: Option<RwSignal<usize>>,
    /// Two-way visibility signal (`full_screen` only).
    #[prop(optional)]
    visible: Option<RwSignal<bool>>,
    /// Fullscreen mode: the gallery in a mask over the page.
    #[prop(optional)]
    full_screen: bool,
    /// Number of visible thumbnails.
    #[prop(optional, default = Prop::Static(3), into)]
    num_visible: Prop<usize>,
    /// Responsive options for the number of visible thumbnails.
    #[prop(optional, into)]
    responsive_options: Vec<GalleriaResponsive>,
    /// Frame navigation buttons.
    #[prop(optional, into)]
    show_item_navigators: Prop<bool>,
    /// Thumbnail page buttons.
    #[prop(optional, default = Prop::Static(true), into)]
    show_thumbnail_navigators: Prop<bool>,
    /// Frame navigation buttons only on hover.
    #[prop(optional, into)]
    show_item_navigators_on_hover: Prop<bool>,
    /// Change the frame when hovering over an indicator.
    #[prop(optional, into)]
    change_item_on_indicator_hover: Prop<bool>,
    /// Wrap navigation around.
    #[prop(optional, into)]
    circular: Prop<bool>,
    /// Automatic frame advance.
    #[prop(optional, into)]
    auto_play: Prop<bool>,
    /// Stop auto-advance on user action.
    #[prop(optional, default = Prop::Static(true), into)]
    should_stop_autoplay_by_click: Prop<bool>,
    /// Auto-advance interval, ms.
    #[prop(optional, default = Prop::Static(4000), into)]
    transition_interval: Prop<u32>,
    /// Show thumbnails.
    #[prop(optional, default = Prop::Static(true), into)]
    show_thumbnails: Prop<bool>,
    /// Thumbnail position.
    #[prop(optional, into)]
    thumbnails_position: Prop<GalleriaPosition>,
    /// Height of the vertical thumbnails window.
    #[prop(optional, default = Prop::Static("300px".to_owned()), into)]
    vertical_thumbnail_view_port_height: Prop<String>,
    /// Show indicators.
    #[prop(optional, into)]
    show_indicators: Prop<bool>,
    /// Indicators over the frame.
    #[prop(optional, into)]
    show_indicators_on_item: Prop<bool>,
    /// Indicator position.
    #[prop(optional, into)]
    indicators_position: Prop<GalleriaPosition>,
    /// Base `z-index` of the mask on top of the configuration value.
    #[prop(optional, into)]
    base_z_index: Prop<i32>,
    /// `id` attribute of the root.
    #[prop(optional, into)]
    id: Option<String>,
    /// Additional class of the mask (fullscreen mode).
    #[prop(optional, into)]
    mask_class: OptProp<String>,
    /// Additional class of the root in fullscreen mode.
    #[prop(optional, into)]
    container_class: OptProp<String>,
    /// Inline style of the root in fullscreen mode.
    #[prop(optional, into)]
    container_style: OptProp<String>,
    /// Inline style of the root.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Additional class of the root.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Custom frame view.
    #[prop(optional, into)]
    item: Option<Callback<GalleriaImage, AnyView>>,
    /// Custom thumbnail view.
    #[prop(optional, into)]
    thumbnail: Option<Callback<GalleriaImage, AnyView>>,
    /// Frame caption.
    #[prop(optional, into)]
    caption: Option<Callback<GalleriaImage, AnyView>>,
    /// Custom indicator view (by index).
    #[prop(optional, into)]
    indicator: Option<Callback<usize, AnyView>>,
    /// Gallery header.
    #[prop(optional, into)]
    header: Option<ViewFn>,
    /// Gallery footer.
    #[prop(optional, into)]
    footer: Option<ViewFn>,
    /// Pass-through parts.
    #[prop(optional)]
    pt: Option<GalleriaPt>,
) -> impl IntoView {
    let pt = Arc::new(pt.unwrap_or_default());
    let id = id.unwrap_or_else(|| format!("pn_id_{}", prime_rs_core::ids::next_unique()));
    let base_nv = num_visible.get_untracked();
    let core = Arc::new(Core {
        value,
        active: active_index.unwrap_or_default(),
        visible: visible.unwrap_or_default(),
        rendered: RwSignal::new(false),
        leaving: RwSignal::new(false),
        slide_show_active: RwSignal::new(true),
        num_visible,
        num_visible_now: RwSignal::new(base_nv),
        responsive_options,
        items_hidden: RwSignal::new(false),
        transition_on: RwSignal::new(false),
        tab_index_override: RwSignal::new(None),
        full_screen,
        show_item_navigators,
        show_thumbnail_navigators,
        show_item_navigators_on_hover,
        change_item_on_indicator_hover,
        circular,
        auto_play,
        should_stop_autoplay_by_click,
        transition_interval,
        show_thumbnails,
        thumbnails_position,
        vertical_height: vertical_thumbnail_view_port_height,
        show_indicators,
        show_indicators_on_item,
        indicators_position,
        base_z_index,
        id,
        mask_class,
        container_class,
        container_style,
        style,
        style_class,
        item,
        thumbnail,
        caption,
        indicator,
        header,
        footer,
        mask: NodeRef::new(),
        panel: NodeRef::new(),
        thumb_items: NodeRef::new(),
        cleanups: StoredValue::new_local(Vec::new()),
        z_index: StoredValue::new(None),
        scroll_blocked: StoredValue::new(false),
        was_open: StoredValue::new(false),
        #[cfg(target_arch = "wasm32")]
        trap: StoredValue::new_local(None),
        #[cfg(target_arch = "wasm32")]
        timer: StoredValue::new_local(None),
        #[cfg(target_arch = "wasm32")]
        interval: StoredValue::new_local(None),
        #[cfg(target_arch = "wasm32")]
        resize: StoredValue::new_local(None),
    });
    {
        let core = core.clone();
        on_cleanup(move || core.teardown());
    }

    // Auto-advance, responsive options, showing and closing the mask happen only in the browser.
    #[cfg(target_arch = "wasm32")]
    {
        let c = core.clone();
        Effect::new(move |_| {
            if c.auto_play.get() {
                c.start_slide_show();
            } else {
                c.halt_slide_show();
            }
        });
        let c = core.clone();
        Effect::new(move |_| {
            // the base visible count changed: recompute taking the window width into account
            c.num_visible.get();
            update_responsive(&c);
        });
        if !core.responsive_options.is_empty() {
            let c = core.clone();
            let handle = window_event_listener(leptos::ev::resize, move |_| update_responsive(&c));
            core.resize.set_value(Some(handle));
        }
        if core.full_screen {
            let c = core.clone();
            Effect::new(move |_| {
                let shown = c.visible.get();
                let mounted = c.panel.get().is_some();
                let was_open = c.was_open.get_value();
                if shown {
                    if let Some(handle) = c.timer.try_update_value(Option::take).flatten() {
                        handle.clear();
                    }
                    if c.leaving.get_untracked() {
                        c.leaving.set(false);
                    }
                    if !c.rendered.get_untracked() {
                        c.rendered.set(true);
                    }
                    if mounted && !was_open {
                        c.was_open.set_value(true);
                        c.opened();
                    }
                } else if was_open {
                    c.was_open.set_value(false);
                    c.begin_leave();
                } else if c.rendered.get_untracked() && !c.leaving.get_untracked() {
                    c.rendered.set(false);
                }
            });
        }
    }

    if full_screen {
        let gate = move || {
            let (core, pt) = (core.clone(), pt.clone());
            core.rendered.get().then(move || view! { <Portal>{mask_view(&core, &pt)}</Portal> })
        };
        gate.into_any()
    } else {
        content_view(&core, &pt).into_any()
    }
}

#[cfg(target_arch = "wasm32")]
fn update_responsive(core: &Arc<Core>) {
    let base = core.num_visible.get_untracked();
    let next = match web_sys::window().and_then(|w| w.inner_width().ok()).and_then(|w| w.as_f64()) {
        Some(width) if !core.responsive_options.is_empty() => model::responsive_num_visible(&core.responsive_options, width as u32, base),
        _ => base,
    };
    if core.num_visible_now.get_untracked() != next {
        core.num_visible_now.set(next);
    }
}

/// Fullscreen mode mask with the gallery.
fn mask_view(core: &Arc<Core>, pt: &Arc<GalleriaPt>) -> impl IntoView + use<> {
    let (mask, leaving) = (core.mask, core.leaving);
    let class = {
        let (pt, core) = (pt.clone(), core.clone());
        move || {
            let mut cn = String::from("p-galleria-mask p-overlay-mask ");
            cn.push_str(if leaving.get() { "p-overlay-mask-leave-active" } else { "p-overlay-mask-enter-active" });
            core.mask_class.with(|c| {
                if let Some(c) = c.as_deref().filter(|c| !c.is_empty()) {
                    cn.push(' ');
                    cn.push_str(c);
                }
            });
            pt.mask.merge_class(&cn)
        }
    };
    let (click_core, end_core) = (core.clone(), core.clone());
    view! {
        <div
            node_ref=mask
            class={class}
            role="dialog"
            aria-modal="true"
            on:click=move |e: web_sys::MouseEvent| {
                if e.target() == e.current_target() {
                    click_core.visible.set(false);
                }
            }
            on:animationend=move |e: web_sys::AnimationEvent| {
                #[cfg(target_arch = "wasm32")]
                {
                    // the animation of the gallery root (not nested elements) finishes the leave
                    let own = e.target().is_some_and(|t| t.dyn_ref::<web_sys::Element>().is_some_and(|el| el.class_list().contains("p-galleria")));
                    if own && end_core.leaving.get_untracked() {
                        end_core.finish_leave();
                    }
                }
                #[cfg(not(target_arch = "wasm32"))]
                let _ = (&e, &end_core);
            }
            {..pt.mask.attrs()}
        >
            {content_view(core, pt)}
        </div>
    }
}

/// Gallery root with content (`GalleriaContent`).
fn content_view(core: &Arc<Core>, pt: &Arc<GalleriaPt>) -> impl IntoView + use<> {
    let panel = core.panel;
    let class = {
        let (core, pt) = (core.clone(), pt.clone());
        move || {
            let mut cn = root_classes(
                core.full_screen,
                core.show_indicators_on_item.get(),
                core.show_item_navigators_on_hover.get(),
                core.show_thumbnails.get().then(|| core.thumbnails_position.get()),
                core.show_indicators.get().then(|| core.indicators_position.get()),
            );
            if core.full_screen {
                cn.push_str(if core.leaving.get() { " p-galleria-leave-active" } else { " p-galleria-enter-active" });
                core.container_class.with(|c| push_class(&mut cn, c.as_deref()));
            } else {
                core.style_class.with(|c| push_class(&mut cn, c.as_deref()));
            }
            pt.root.merge_class(&cn)
        }
    };
    let root_style = {
        let (core, pt) = (core.clone(), pt.clone());
        move || {
            let user = if core.full_screen { core.container_style.get() } else { core.style.get().or_else(|| core.container_style.get()) }
                .unwrap_or_default();
            let merged = pt.root.merge_style_parts(&[&user]);
            (!merged.is_empty()).then_some(merged)
        }
    };
    let body = {
        let (core, pt) = (core.clone(), pt.clone());
        move || (core.len() > 0).then(|| body_view(&core, &pt))
    };
    let id = core.id.clone();
    view! {
        <div node_ref=panel id={id} role="region" class={class} style={root_style} {..pt.root.attrs()}>
            {body}
        </div>
    }
}

fn push_class(cn: &mut String, extra: Option<&str>) {
    if let Some(extra) = extra.filter(|c| !c.is_empty()) {
        cn.push(' ');
        cn.push_str(extra);
    }
}

/// Close button, header, content and footer.
fn body_view(core: &Arc<Core>, pt: &Arc<GalleriaPt>) -> impl IntoView + use<> {
    let close = core.full_screen.then(|| {
        let visible = core.visible;
        let label = prime_rs_core::use_prime_config().locale.text("aria.close").to_owned();
        view! {
            <button
                type="button"
                class={pt.close_button.merge_class("p-galleria-close-button")}
                data-pc-section="closebutton"
                aria-label={label}
                on:click=move |_| visible.set(false)
                {..pt.close_button.attrs()}
            >
                <TimesIcon class="p-galleria-close-icon" />
            </button>
        }
    });
    let header = core
        .header
        .clone()
        .map(|h| view! { <div class={pt.header.merge_class("p-galleria-header")} {..pt.header.attrs()}>{h.run()}</div> });
    let footer = core
        .footer
        .clone()
        .map(|f| view! { <div class={pt.footer.merge_class("p-galleria-footer")} {..pt.footer.attrs()}>{f.run()}</div> });
    let live = if core.auto_play.get_untracked() { "polite" } else { "off" };
    let thumbnails = {
        let (core, pt) = (core.clone(), pt.clone());
        move || core.show_thumbnails.get().then(|| thumbnails_view(&core, &pt))
    };
    view! {
        {close}
        {header}
        <div class={pt.content.merge_class("p-galleria-content")} aria-live={live} {..pt.content.attrs()}>
            {items_container_view(core, pt)}
            {thumbnails}
        </div>
        {footer}
    }
}

/// Frame, buttons, caption and indicators (`GalleriaItem`).
fn items_container_view(core: &Arc<Core>, pt: &Arc<GalleriaPt>) -> impl IntoView + use<> {
    let nav = |forward: bool| {
        let core = core.clone();
        let pt = pt.clone();
        move || {
            core.show_item_navigators.get().then(|| {
                let (cls_core, click_core) = (core.clone(), core.clone());
                let (base, button_pt) = if forward {
                    ("p-galleria-next-button p-galleria-nav-button", pt.next_button.clone())
                } else {
                    ("p-galleria-prev-button p-galleria-nav-button", pt.prev_button.clone())
                };
                let class_pt = button_pt.clone();
                let class = move || {
                    let (active, len, circular) = (cls_core.active.get(), cls_core.len(), cls_core.circular.get());
                    let disabled =
                        if forward { item_forward_disabled(active, len, circular) } else { item_backward_disabled(active, circular) };
                    class_pt.merge_class(&if disabled { format!("{base} p-disabled") } else { base.to_owned() }).into_owned()
                };
                let icon = if forward {
                    view! { <ChevronRightIcon class="p-galleria-next-icon" /> }.into_any()
                } else {
                    view! { <ChevronLeftIcon class="p-galleria-prev-icon" /> }.into_any()
                };
                view! {
                    <button
                        type="button"
                        role="navigation"
                        class={class}
                        data-pc-group-section="itemnavigator"
                        on:click=move |e: web_sys::MouseEvent| click_core.step_item(forward, &e)
                        {..button_pt.attrs()}
                    >
                        {icon}
                    </button>
                }
            })
        }
    };
    let item = {
        let (core, pt) = (core.clone(), pt.clone());
        move || {
            let active = core.active.get();
            let image = core.value.with(|v| v.get(active).cloned())?;
            let label =
                prime_rs_core::use_prime_config().locale.text("aria.slideNumber").replace("{slideNumber}", &(active + 1).to_string());
            let roledescription = prime_rs_core::use_prime_config().locale.text("aria.slide").to_owned();
            let inner = match core.item {
                Some(render) => render.run(image.clone()),
                None => view! { <img src={image.item_image_src.clone()} alt={image.alt.clone()} style="width: 100%; display: block" /> }
                    .into_any(),
            };
            let caption = core.caption.map(|render| {
                view! { <div class={pt.caption.merge_class("p-galleria-caption")} {..pt.caption.attrs()}>{render.run(image.clone())}</div> }
            });
            Some(view! {
                <div
                    id={format!("{}_item_{active}", core.id)}
                    role="group"
                    class={pt.item.merge_class("p-galleria-item")}
                    aria-label={label}
                    aria-roledescription={roledescription}
                    {..pt.item.attrs()}
                >
                    {inner}
                </div>
                {caption}
            })
        }
    };
    let indicators = {
        let (core, pt) = (core.clone(), pt.clone());
        move || core.show_indicators.get().then(|| indicators_view(&core, &pt))
    };
    view! {
        <div class={pt.items_container.merge_class("p-galleria-items-container")} {..pt.items_container.attrs()}>
            <div class={pt.items.merge_class("p-galleria-items")} {..pt.items.attrs()}>
                {nav(false)}
                {item}
                {nav(true)}
            </div>
            {indicators}
        </div>
    }
}

fn indicators_view(core: &Arc<Core>, pt: &Arc<GalleriaPt>) -> impl IntoView + use<> {
    let items = {
        let (core, pt) = (core.clone(), pt.clone());
        move || {
            (0..core.len())
                .map(|index| {
                    let (c_class, c_click, c_hover, c_key, c_sel, c_aria) =
                        (core.clone(), core.clone(), core.clone(), core.clone(), core.clone(), core.clone());
                    let pt = pt.clone();
                    let label = prime_rs_core::use_prime_config().locale.text("aria.pageLabel").replace("{page}", &(index + 1).to_string());
                    let class = {
                        let pt = pt.clone();
                        move || {
                            let base = if c_class.active.get() == index { "p-galleria-indicator p-galleria-indicator-active" } else { "p-galleria-indicator" };
                            pt.indicator.merge_class(base)
                        }
                    };
                    let inner = match core.indicator {
                        Some(render) => render.run(index),
                        None => view! { <button type="button" tabindex="-1" class={pt.indicator_button.merge_class("p-galleria-indicator-button")} {..pt.indicator_button.attrs()}></button> }
                            .into_any(),
                    };
                    view! {
                        <li
                            tabindex="0"
                            class={class}
                            aria-label={label}
                            aria-selected={move || (c_sel.active.get() == index).to_string()}
                            aria-controls={format!("{}_item_{index}", core.id)}
                            data-p-active={move || (c_aria.active.get() == index).to_string()}
                            on:click=move |_| c_click.select(index)
                            on:mouseenter=move |_| {
                                if c_hover.change_item_on_indicator_hover.get_untracked() {
                                    c_hover.select(index);
                                }
                            }
                            on:keydown=move |e: web_sys::KeyboardEvent| match e.code().as_str() {
                                "Enter" | "Space" => {
                                    c_key.select(index);
                                    e.prevent_default();
                                }
                                "ArrowDown" | "ArrowUp" => e.prevent_default(),
                                _ => {}
                            }
                            {..pt.indicator.attrs()}
                        >
                            {inner}
                        </li>
                    }
                })
                .collect_view()
        }
    };
    view! { <ul class={pt.indicator_list.merge_class("p-galleria-indicator-list")} {..pt.indicator_list.attrs()}>{items}</ul> }
}

/// Thumbnail strip (`GalleriaThumbnails`).
fn thumbnails_view(core: &Arc<Core>, pt: &Arc<GalleriaPt>) -> impl IntoView + use<> {
    let nav = |forward: bool| {
        let (core, pt) = (core.clone(), pt.clone());
        move || {
            core.show_thumbnail_navigators.get().then(|| {
                let (cls_core, click_core) = (core.clone(), core.clone());
                let (base, button_pt, label_key) = if forward {
                    (
                        "p-galleria-thumbnail-next-button p-galleria-thumbnail-nav-button",
                        pt.thumbnail_next_button.clone(),
                        "aria.nextPageLabel",
                    )
                } else {
                    (
                        "p-galleria-thumbnail-prev-button p-galleria-thumbnail-nav-button",
                        pt.thumbnail_prev_button.clone(),
                        "aria.prevPageLabel",
                    )
                };
                let class_pt = button_pt.clone();
                let class = move || {
                    let (active, len, nv, circular) = (cls_core.active.get(), cls_core.len(), cls_core.nv(), cls_core.circular.get());
                    let disabled = if forward {
                        thumb_forward_disabled(active, len, nv, circular)
                    } else {
                        thumb_backward_disabled(active, len, nv, circular)
                    };
                    class_pt.merge_class(&if disabled { format!("{base} p-disabled") } else { base.to_owned() }).into_owned()
                };
                let icon_core = core.clone();
                let icon = move || match (forward, icon_core.vertical()) {
                    (false, false) => view! { <ChevronLeftIcon class="p-galleria-thumbnail-prev-icon" /> }.into_any(),
                    (false, true) => view! { <ChevronUpIcon class="p-galleria-thumbnail-prev-icon" /> }.into_any(),
                    (true, false) => view! { <ChevronRightIcon class="p-galleria-thumbnail-next-icon" /> }.into_any(),
                    (true, true) => view! { <ChevronDownIcon class="p-galleria-thumbnail-next-icon" /> }.into_any(),
                };
                view! {
                    <button
                        type="button"
                        class={class}
                        aria-label={prime_rs_core::use_prime_config().locale.text(label_key).to_owned()}
                        data-pc-group-section="thumbnailnavigator"
                        on:click=move |e: web_sys::MouseEvent| click_core.step_thumbnails(forward, &e)
                        {..button_pt.attrs()}
                    >
                        {icon}
                    </button>
                }
            })
        }
    };
    let viewport_style = {
        let core = core.clone();
        move || core.vertical().then(|| format!("height: {}", core.vertical_height.get()))
    };
    let items_class = {
        let (core, pt) = (core.clone(), pt.clone());
        move || {
            let base = if core.items_hidden.get() { "p-galleria-thumbnail-items p-items-hidden" } else { "p-galleria-thumbnail-items" };
            pt.thumbnail_items.merge_class(base)
        }
    };
    let items_style = {
        let core = core.clone();
        move || {
            let mut style = format!("transform: {}", thumbnails_transform(core.shifted(), core.nv(), core.vertical()));
            if core.transition_on.get() {
                style.push_str("; transition: transform 500ms ease 0s");
            }
            style
        }
    };
    let transition_core = core.clone();
    let thumbs = {
        let (core, pt) = (core.clone(), pt.clone());
        move || (0..core.len()).map(|index| thumbnail_view(&core, &pt, index)).collect_view()
    };
    let items_ref = core.thumb_items;
    view! {
        <div class={pt.thumbnails.merge_class("p-galleria-thumbnails")} {..pt.thumbnails.attrs()}>
            <div class={pt.thumbnail_content.merge_class("p-galleria-thumbnails-content")} {..pt.thumbnail_content.attrs()}>
                {nav(false)}
                <div class={pt.thumbnails_viewport.merge_class("p-galleria-thumbnails-viewport")} style={viewport_style} {..pt.thumbnails_viewport.attrs()}>
                    <div
                        node_ref=items_ref
                        class={items_class}
                        style={items_style}
                        role="tablist"
                        on:transitionend=move |_| {
                            transition_core.items_hidden.set(true);
                            transition_core.transition_on.set(false);
                        }
                        {..pt.thumbnail_items.attrs()}
                    >
                        {thumbs}
                    </div>
                </div>
                {nav(true)}
            </div>
        </div>
    }
}

fn thumbnail_view(core: &Arc<Core>, pt: &Arc<GalleriaPt>, index: usize) -> impl IntoView + use<> {
    let (c_class, c_style, c_sel, c_active, c_key, c_click, c_tab, c_current, c_label) =
        (core.clone(), core.clone(), core.clone(), core.clone(), core.clone(), core.clone(), core.clone(), core.clone(), core.clone());
    let _ = c_label;
    let image = core.value.with(|v| v.get(index).cloned()).unwrap_or_default();
    let inner = match core.thumbnail {
        Some(render) => render.run(image),
        None => {
            let src = image.thumbnail_image_src.clone().unwrap_or_else(|| image.item_image_src.clone());
            view! { <img src={src} alt={image.alt.clone()} style="display: block" /> }.into_any()
        }
    };
    let label = prime_rs_core::use_prime_config().locale.text("aria.pageLabel").replace("{page}", &(index + 1).to_string());
    let class = {
        let pt = pt.clone();
        move || {
            pt.thumbnail_item
                .merge_class(&thumbnail_item_classes(index, c_class.active.get(), c_class.shifted(), c_class.nv()))
                .into_owned()
        }
    };
    let style = move || format!("flex: 1 0 {}%", 100.0 / c_style.nv().max(1) as f64);
    view! {
        <div
            class={class}
            style={style}
            aria-selected={move || (c_sel.active.get() == index).to_string()}
            aria-controls={format!("{}_item_{index}", core.id)}
            data-p-active={move || (c_active.active.get() == index).to_string()}
            on:keydown=move |e: web_sys::KeyboardEvent| c_key.thumb_keydown(index, &e)
            {..pt.thumbnail_item.attrs()}
        >
            <div
                class={pt.thumbnail.merge_class("p-galleria-thumbnail")}
                tabindex={move || if c_tab.tab_index_override.get().unwrap_or_else(|| c_tab.active.get()) == index { "0" } else { "-1" }}
                aria-current={move || (c_current.active.get() == index).then_some("page")}
                aria-label={label}
                on:click=move |_| c_click.select(index)
                {..pt.thumbnail.attrs()}
            >
                {inner}
            </div>
        </div>
    }
}
