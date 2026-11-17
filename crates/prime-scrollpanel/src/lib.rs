//! ScrollPanel — port of `p-scrollpanel` from PrimeNG 21.1.10.
//!
//! Markup and classes match PrimeNG: root `div.p-scrollpanel`, container `p-scrollpanel-content-container` with
//! `p-scrollpanel-content` inside and two bars `p-scrollpanel-bar-x` / `p-scrollpanel-bar-y` (`role="scrollbar"`).
//! Instead of the `<p-scrollpanel>` host, `<div class="p-scrollpanel …">` is rendered.
//!
//! Bar size and position are computed only in the browser and written straight to the DOM (see `docs/scrollpanel.md`);
//! SSR emits markup without touching the DOM. Properties are [`Prop`]/[`OptProp`]; if `style` and `style_class` are static,
//! the component renders ready-made values without closures and `RenderEffect`.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_scrollpanel::ScrollPanel;
//!
//! let html = view! { <ScrollPanel style="height:150px">"Text"</ScrollPanel> }.to_html();
//! assert!(html.contains("p-scrollpanel-content") && html.contains("p-scrollpanel-bar-x") && html.contains("p-scrollpanel-bar-y"));
//! ```

#[cfg(target_arch = "wasm32")]
mod client;

use leptos::either::Either;
use leptos::prelude::*;
use prime_rs_core::collapsible::CollapseIds;
use prime_rs_core::{ClassNames, OptProp, Pt, all_static};
use std::borrow::Cow;
use std::sync::Arc;

/// Default keyboard scroll step, pixels.
pub const DEFAULT_STEP: f64 = 5.0;

/// Minimum bar length, percent.
const MIN_BAR_PERCENT: f64 = 10.0;

/// Fraction of the visible part: `own / total`; with empty content it is 1 (no bar needed).
///
/// ```
/// use prime_scrollpanel::bar_ratio;
///
/// assert_eq!(bar_ratio(150.0, 300.0), 0.5);
/// assert_eq!(bar_ratio(150.0, 0.0), 1.0);
/// ```
pub fn bar_ratio(own: f64, total: f64) -> f64 {
    if total > 0.0 { own / total } else { 1.0 }
}

/// Bar length in percent: fraction × 100, but not less than 10.
///
/// ```
/// use prime_scrollpanel::bar_len;
///
/// assert_eq!(bar_len(0.5), 50.0);
/// assert_eq!(bar_len(0.02), 10.0);
/// ```
pub fn bar_len(ratio: f64) -> f64 {
    (ratio * 100.0).max(MIN_BAR_PERCENT)
}

/// Bar offset in percent: `scroll × (100 − len) / (total − own)`; with no scroll travel, 0.
///
/// ```
/// use prime_scrollpanel::bar_offset;
///
/// // content 400, area 100, bar 25%, scrolled half the travel (150 of 300)
/// assert_eq!(bar_offset(150.0, 25.0, 400.0, 100.0), 37.5);
/// assert_eq!(bar_offset(10.0, 25.0, 100.0, 100.0), 0.0);
/// ```
pub fn bar_offset(scroll: f64, len: f64, total: f64, own: f64) -> f64 {
    let travel = total - own;
    if travel > 0.0 { scroll * (100.0 - len) / travel } else { 0.0 }
}

/// Reaction to a key pressed on a bar.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KeyAction {
    /// Scroll: `horizontal` means the X axis, `delta` is the step sign (`1.0` or `-1.0`).
    Scroll {
        /// X axis (otherwise Y).
        horizontal: bool,
        /// Step sign.
        delta: f64,
    },
    /// Only cancel the browser default action.
    Block,
    /// Do nothing.
    Ignore,
}

/// Key table: the vertical axis listens to `ArrowUp`/`ArrowDown`, the horizontal to `ArrowLeft`/`ArrowRight`;
/// the other arrows are merely blocked.
///
/// ```
/// use prime_scrollpanel::{KeyAction, key_action};
///
/// assert_eq!(key_action(false, "ArrowDown"), KeyAction::Scroll { horizontal: false, delta: 1.0 });
/// assert_eq!(key_action(true, "ArrowLeft"), KeyAction::Scroll { horizontal: true, delta: -1.0 });
/// assert_eq!(key_action(false, "ArrowLeft"), KeyAction::Block);
/// assert_eq!(key_action(false, "Enter"), KeyAction::Ignore);
/// ```
pub fn key_action(horizontal: bool, code: &str) -> KeyAction {
    match (horizontal, code) {
        (false, "ArrowDown") => KeyAction::Scroll { horizontal: false, delta: 1.0 },
        (false, "ArrowUp") => KeyAction::Scroll { horizontal: false, delta: -1.0 },
        (true, "ArrowRight") => KeyAction::Scroll { horizontal: true, delta: 1.0 },
        (true, "ArrowLeft") => KeyAction::Scroll { horizontal: true, delta: -1.0 },
        (_, "ArrowLeft" | "ArrowRight" | "ArrowUp" | "ArrowDown") => KeyAction::Block,
        _ => KeyAction::Ignore,
    }
}

/// Clamps `scrollTop` to the range `0..=scroll_height − client_height`.
///
/// ```
/// use prime_scrollpanel::clamp_scroll;
///
/// assert_eq!(clamp_scroll(100000.0, 500.0, 150.0), 350.0);
/// assert_eq!(clamp_scroll(-5.0, 500.0, 150.0), 0.0);
/// ```
pub fn clamp_scroll(value: f64, scroll_size: f64, client_size: f64) -> f64 {
    value.min(scroll_size - client_size).max(0.0)
}

/// Pass-through of ScrollPanel parts.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ScrollPanelPt {
    /// Root element.
    pub root: Pt,
    /// Content container (`div.p-scrollpanel-content-container`).
    pub content_container: Pt,
    /// Content (`div.p-scrollpanel-content`).
    pub content: Pt,
    /// Horizontal bar (`div.p-scrollpanel-bar-x`).
    pub bar_x: Pt,
    /// Vertical bar (`div.p-scrollpanel-bar-y`).
    pub bar_y: Pt,
}

/// Handle for calling ScrollPanel methods from code (like `@ViewChild`): `refresh` and `scroll_top`.
///
/// Created before the component and passed via the `panel_ref` property; outside the browser and before mounting the methods do nothing.
#[derive(Clone, Copy)]
pub struct ScrollPanelRef {
    #[cfg(target_arch = "wasm32")]
    inner: StoredValue<Option<std::rc::Rc<client::Client>>, LocalStorage>,
}

impl ScrollPanelRef {
    /// Empty handle; the component fills it on mount.
    pub fn new() -> Self {
        Self {
            #[cfg(target_arch = "wasm32")]
            inner: StoredValue::new_local(None),
        }
    }

    /// Recalculates the position and size of the bars.
    pub fn refresh(&self) {
        #[cfg(target_arch = "wasm32")]
        if let Some(c) = self.inner.try_get_value().flatten() {
            c.move_bar();
        }
    }

    /// Scrolls the content to `value` pixels from the top (the value is clamped to the available travel).
    pub fn scroll_top(&self, value: f64) {
        #[cfg(target_arch = "wasm32")]
        if let Some(c) = self.inner.try_get_value().flatten() {
            c.scroll_top(value);
        }
        #[cfg(not(target_arch = "wasm32"))]
        let _ = value;
    }
}

impl Default for ScrollPanelRef {
    fn default() -> Self {
        Self::new()
    }
}

/// Panel with themable scrollbars.
///
/// `style` and `style_class` accept a value or a signal. Sizes (`width`, `height` or `max-height`) are set via `style`.
/// Content is `children` or the `content_template` slot (when the slot is set, `children` are not rendered).
#[component]
pub fn ScrollPanel(
    /// Keyboard arrow scroll step, pixels (default 5).
    #[prop(optional, into)]
    step: OptProp<f64>,
    /// Inline style of the root.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Extra class of the root.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Content slot (like the `#content` template).
    #[prop(optional, into)]
    content_template: Option<ViewFn>,
    /// Handle for `refresh` and `scroll_top`.
    #[prop(optional)]
    panel_ref: Option<ScrollPanelRef>,
    /// Pass-through.
    #[prop(optional)]
    pt: Option<ScrollPanelPt>,
    /// Content.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let pt = pt.unwrap_or_default();
    let is_static = all_static!(style, style_class);
    let props = Props { style, style_class };
    let content_id = CollapseIds::new(None).content;

    // `NodeRef`s are needed only by the client (measuring and listeners): on SSR there are none, and `()` is an empty attribute instead of `node_ref`.
    #[cfg(target_arch = "wasm32")]
    let (root_ref, content_ref, x_ref, y_ref) = {
        use leptos::html::Div;
        use leptos::tachys::html::node_ref::node_ref;
        let (root, content, x, y) = (NodeRef::<Div>::new(), NodeRef::<Div>::new(), NodeRef::<Div>::new(), NodeRef::<Div>::new());
        let step = step.clone();
        client::mount(root, content, x, y, std::rc::Rc::new(move || step.get_untracked().unwrap_or(DEFAULT_STEP)), panel_ref);
        (node_ref::<Div, _>(root), node_ref::<Div, _>(content), node_ref::<Div, _>(x), node_ref::<Div, _>(y))
    };
    #[cfg(not(target_arch = "wasm32"))]
    #[allow(clippy::let_unit_value)]
    let (root_ref, content_ref, x_ref, y_ref) = {
        let _ = (step, panel_ref);
        ((), (), (), ())
    };

    let content = match content_template {
        Some(slot) => Some(slot.run()),
        None => children.map(|c| c().into_any()),
    };
    let root_attrs = pt.root.attrs();
    let (container_class, container_attrs) =
        (pt.content_container.merge_class("p-scrollpanel-content-container"), pt.content_container.attrs());
    let (content_class, content_attrs) = (pt.content.merge_class("p-scrollpanel-content"), pt.content.attrs());
    let (bar_x_class, bar_x_style, bar_x_attrs) = bar_of(&pt.bar_x, "p-scrollpanel-bar p-scrollpanel-bar-x");
    let (bar_y_class, bar_y_style, bar_y_attrs) = bar_of(&pt.bar_y, "p-scrollpanel-bar p-scrollpanel-bar-y");
    let controls = content_id.clone();

    if is_static {
        let (class, style) = (class_of(&props, &pt), style_of(&props, &pt));
        Either::Left(scrollpanel_markup!(
            class,
            style,
            root_attrs,
            root_ref,
            container_class,
            container_attrs,
            content_ref,
            content_id,
            content_class,
            content_attrs,
            content,
            x_ref,
            bar_x_class,
            bar_x_style,
            bar_x_attrs,
            y_ref,
            bar_y_class,
            bar_y_style,
            bar_y_attrs,
            controls
        ))
    } else {
        let (props, pt) = (Arc::new(props), Arc::new(pt));
        let class = {
            let (props, pt) = (props.clone(), pt.clone());
            move || class_of(&props, &pt)
        };
        let style = move || style_of(&props, &pt);
        Either::Right(scrollpanel_markup!(
            class,
            style,
            root_attrs,
            root_ref,
            container_class,
            container_attrs,
            content_ref,
            content_id,
            content_class,
            content_attrs,
            content,
            x_ref,
            bar_x_class,
            bar_x_style,
            bar_x_attrs,
            y_ref,
            bar_y_class,
            bar_y_style,
            bar_y_attrs,
            controls
        ))
    }
}

/// ScrollPanel markup: one for the static and reactive branches (only the root `class` and `style` expressions differ).
macro_rules! scrollpanel_markup {
    (
        $class:expr, $style:expr, $root_attrs:expr, $root_ref:expr, $container_class:expr, $container_attrs:expr, $content_ref:expr,
        $content_id:expr, $content_class:expr, $content_attrs:expr, $content:expr, $x_ref:expr, $bar_x_class:expr, $bar_x_style:expr,
        $bar_x_attrs:expr, $y_ref:expr, $bar_y_class:expr, $bar_y_style:expr, $bar_y_attrs:expr, $controls:expr
    ) => {
        view! {
            <div class={$class} style={$style} {..$root_attrs} {..$root_ref}>
                <div class={$container_class} {..$container_attrs}>
                    <div {..$content_ref} id={$content_id} class={$content_class} {..$content_attrs}>
                        {$content}
                    </div>
                </div>
                <div
                    {..$x_ref}
                    class={$bar_x_class}
                    style={$bar_x_style}
                    tabindex="0"
                    role="scrollbar"
                    aria-orientation="horizontal"
                    aria-valuenow="0"
                    aria-controls={$controls.clone()}
                    data-pc-group-section="bar"
                    {..$bar_x_attrs}
                ></div>
                <div
                    {..$y_ref}
                    class={$bar_y_class}
                    style={$bar_y_style}
                    tabindex="0"
                    role="scrollbar"
                    aria-orientation="vertical"
                    aria-valuenow="0"
                    aria-controls={$controls}
                    data-pc-group-section="bar"
                    {..$bar_y_attrs}
                ></div>
            </div>
        }
    };
}
use scrollpanel_markup;

fn class_of(props: &Props, pt: &ScrollPanelPt) -> Cow<'static, str> {
    let mut cn = ClassNames::new();
    cn.add("p-scrollpanel p-component");
    props.style_class.with(|c| cn.add_user(c.as_deref()));
    cn.add_user(pt.root.extra_class());
    cn.finish_cow()
}

fn style_of(props: &Props, pt: &ScrollPanelPt) -> Option<String> {
    props.style.with(|st| (st.is_some() || pt.root.has_style()).then(|| pt.root.merge_style(st.as_deref().unwrap_or(""))))
}

/// Class, initial style and bar attributes; after that the bar `style` is driven by the client recalculation, not Leptos.
fn bar_of(pt: &Pt, base: &'static str) -> (Cow<'static, str>, Option<String>, Vec<leptos::attr::any_attribute::AnyAttribute>) {
    (pt.merge_class(base), pt.has_style().then(|| pt.merge_style("")), pt.attrs())
}

/// Instance properties.
struct Props {
    style: OptProp<String>,
    style_class: OptProp<String>,
}
