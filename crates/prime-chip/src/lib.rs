//! Chip — port of `p-chip` from PrimeNG 21.1.10.
//!
//! Markup, classes and `data-p` match PrimeNG; instead of the host element `<p-chip>` a `<div class="p-chip …">` is rendered.
//! A removable Chip is hidden (`display: none`) on a click on the remove element and on the Enter and Backspace keys, and calls `on_remove`.
//!
//! Properties are [`Prop`]/[`OptProp`]; if all of them are static and the Chip is not removable, the component renders ready values
//! without closures and `RenderEffect` (a removable one needs internal visibility state, so it takes the reactive path).
//!
//! ```
//! use leptos::prelude::*;
//! use prime_chip::Chip;
//!
//! let html = view! { <Chip label="Amy Elsner" removable=true /> }.to_html();
//! assert!(html.contains("p-chip-label") && html.contains("p-chip-remove-icon") && html.contains(r#"role="button""#));
//! ```

mod state;

use leptos::either::Either;
use leptos::prelude::*;
use prime_icons::TimesCircleIcon;
use prime_rs_core::{ClassNames, OptProp, Prop, Pt, all_static, use_prime_config};
use std::borrow::Cow;
use std::sync::Arc;

pub use state::{ChipState, data_p, is_remove_key, write_classes};

/// Pass-through for the parts of Chip.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ChipPt {
    /// Root element.
    pub root: Pt,
    /// Image (`img.p-chip-image`).
    pub image: Pt,
    /// Icon (`span.p-chip-icon`).
    pub icon: Pt,
    /// Label (`div.p-chip-label`).
    pub label: Pt,
    /// Remove element (`span.p-chip-remove-icon`).
    pub remove_icon: Pt,
}

/// Chip: label, icon or image, removal.
///
/// Each property accepts a value or a signal. `children` are rendered first, like projected content in PrimeNG.
#[component]
pub fn Chip(
    /// Label text.
    #[prop(optional, into)]
    label: OptProp<String>,
    /// Icon; rendered in `span.p-chip-icon` if there is no `image`.
    #[prop(optional, into)]
    icon: Option<ViewFn>,
    /// Image URL; replaces the icon.
    #[prop(optional, into)]
    image: OptProp<String>,
    /// Image alternative text.
    #[prop(optional, into)]
    alt: OptProp<String>,
    /// Unavailable (`p-disabled`, `tabindex=-1` on the remove element).
    #[prop(optional, into)]
    disabled: Prop<bool>,
    /// Whether to show the remove element.
    #[prop(optional, into)]
    removable: Prop<bool>,
    /// Custom remove icon (default `TimesCircleIcon`).
    #[prop(optional, into)]
    remove_icon: Option<ViewFn>,
    /// Inline style.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Extra class.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Called on removal (click, Enter, Backspace) with the original event.
    #[prop(optional, into)]
    on_remove: Option<Callback<web_sys::Event>>,
    /// Called when the image fails to load.
    #[prop(optional, into)]
    on_image_error: Option<Callback<web_sys::Event>>,
    /// Pass-through.
    #[prop(optional)]
    pt: Option<ChipPt>,
    /// Content before the image, label and remove element.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let pt = pt.unwrap_or_default();
    let is_static = all_static!(label, image, alt, disabled, removable, style, style_class) && !removable.get_untracked();
    let props = Props { label, image, alt, disabled, removable, style, style_class, icon, remove_icon };
    let content = children.map(|c| c());
    let attrs = pt.root.attrs();

    if is_static {
        // Static path: values and blocks are computed once, there are no closures and no `RenderEffect`.
        let (class, style, data) = (class_of(&props, &pt), style_of(&props, &pt, true), data_of(&props));
        let (media, label) = (media_of(&props, &pt, on_image_error), label_of(&props, &pt));
        let remove: Option<()> = None;
        Either::Left(chip_markup!(class, style, data, attrs, content, media, label, remove))
    } else {
        let (props, pt) = (Arc::new(props), Arc::new(pt));
        let visible = RwSignal::new(true);
        let class = {
            let (props, pt) = (props.clone(), pt.clone());
            move || class_of(&props, &pt)
        };
        let style = {
            let (props, pt) = (props.clone(), pt.clone());
            move || style_of(&props, &pt, visible.get())
        };
        let data = {
            let props = props.clone();
            move || data_of(&props)
        };
        let media = {
            let (props, pt) = (props.clone(), pt.clone());
            move || media_of(&props, &pt, on_image_error)
        };
        let label = {
            let (props, pt) = (props.clone(), pt.clone());
            move || label_of(&props, &pt)
        };
        let remove = move || props.removable.get().then(|| remove_of(&props, &pt, visible, on_remove));
        Either::Right(chip_markup!(class, style, data, attrs, content, media, label, remove))
    }
}

/// Chip markup: one for the static and reactive branches (only the attribute and block expressions differ).
macro_rules! chip_markup {
    ($class:expr, $style:expr, $data:expr, $attrs:expr, $content:expr, $media:expr, $label:expr, $remove:expr) => {
        view! {
            <div class={$class} style={$style} data-p={$data} {..$attrs}>
                {$content}
                {$media}
                {$label}
                {$remove}
            </div>
        }
    };
}
use chip_markup;

fn class_of(props: &Props, pt: &ChipPt) -> Cow<'static, str> {
    let mut cn = ClassNames::new();
    write_classes(&mut cn, &ChipState { disabled: props.disabled.get(), removable: props.removable.get() });
    props.style_class.with(|c| cn.add_user(c.as_deref()));
    cn.add_user(pt.root.extra_class());
    cn.finish_cow()
}

fn data_of(props: &Props) -> Cow<'static, str> {
    data_p(&ChipState { disabled: props.disabled.get(), removable: props.removable.get() })
}

/// Inline style of the root: `display: none` after removal, then `style` and the style from `pt`.
fn style_of(props: &Props, pt: &ChipPt, visible: bool) -> Option<String> {
    props.style.with(|st| {
        if st.is_none() && visible && !pt.root.has_style() {
            return None;
        }
        let merged = pt.root.merge_style_parts(&[if visible { "" } else { "display: none" }, st.as_deref().unwrap_or("")]);
        (!merged.is_empty()).then_some(merged)
    })
}

/// Image, or the icon if there is no image.
fn media_of(props: &Props, pt: &ChipPt, on_image_error: Option<Callback<web_sys::Event>>) -> Option<impl IntoView + use<>> {
    if let Some(src) = props.image.get().filter(|s| !s.is_empty()) {
        let alt = props.alt.get();
        Some(Either::Left(view! {
            <img
                class=pt.image.merge_class("p-chip-image")
                src=src
                alt=alt
                on:error=move |e| {
                    if let Some(cb) = on_image_error {
                        cb.run(e.into());
                    }
                }
                {..pt.image.attrs()}
            />
        }))
    } else {
        props
            .icon
            .clone()
            .map(|icon| Either::Right(view! { <span class=pt.icon.merge_class("p-chip-icon") {..pt.icon.attrs()}>{icon.run()}</span> }))
    }
}

/// Label (`None` if empty).
fn label_of(props: &Props, pt: &ChipPt) -> Option<impl IntoView + use<>> {
    props
        .label
        .get()
        .filter(|s| !s.is_empty())
        .map(|label| view! { <div class=pt.label.merge_class("p-chip-label") {..pt.label.attrs()}>{label}</div> })
}

/// Remove element: `role="button"`, label from the locale, click and the Enter and Backspace keys.
fn remove_of(props: &Props, pt: &ChipPt, visible: RwSignal<bool>, on_remove: Option<Callback<web_sys::Event>>) -> impl IntoView + use<> {
    let aria = use_prime_config().locale.text("aria.removeLabel").to_owned();
    let tabindex = if props.disabled.get() { -1 } else { 0 };
    let close = move |e: web_sys::Event| {
        visible.set(false);
        if let Some(cb) = on_remove {
            cb.run(e);
        }
    };
    let click = move |e: web_sys::MouseEvent| close(e.into());
    let keydown = move |e: web_sys::KeyboardEvent| {
        if is_remove_key(&e.key()) {
            close(e.into());
        }
    };
    let inner = match props.remove_icon.clone() {
        Some(icon) => Either::Left(icon.run()),
        None => Either::Right(view! { <TimesCircleIcon /> }),
    };
    view! {
        <span
            class=pt.remove_icon.merge_class("p-chip-remove-icon")
            role="button"
            tabindex=tabindex
            aria-label=aria
            on:click={click}
            on:keydown={keydown}
            {..pt.remove_icon.attrs()}
        >
            {inner}
        </span>
    }
}

/// Instance properties.
struct Props {
    label: OptProp<String>,
    image: OptProp<String>,
    alt: OptProp<String>,
    disabled: Prop<bool>,
    removable: Prop<bool>,
    style: OptProp<String>,
    style_class: OptProp<String>,
    icon: Option<ViewFn>,
    remove_icon: Option<ViewFn>,
}
