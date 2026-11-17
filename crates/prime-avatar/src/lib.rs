//! Avatar: a port of `p-avatar` from PrimeNG 21.1.10.
//!
//! Markup, classes and `data-p` match PrimeNG; instead of the host element `<p-avatar>` a `<div class="p-avatar …">` is rendered.
//! Content: `children`, then the label, otherwise the icon, otherwise the image.
//!
//! Properties are [`Prop`]/[`OptProp`]; if all of them are static, the component renders ready-made values without closures and `RenderEffect`.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_avatar::{Avatar, AvatarShape, AvatarSize};
//!
//! let html = view! { <Avatar label="P" shape=AvatarShape::Circle size=AvatarSize::XLarge /> }.to_html();
//! assert!(html.contains("p-avatar-circle") && html.contains("p-avatar-xl") && html.contains(r#"data-p="circle xlarge""#));
//! ```

mod state;

use leptos::either::Either;
use leptos::prelude::*;
use prime_rs_core::{ClassNames, OptProp, Prop, Pt, all_static};
use std::borrow::Cow;
use std::sync::Arc;

pub use state::{AvatarShape, AvatarSize, AvatarState, data_p, write_classes};

/// Pass-through parts of Avatar.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AvatarPt {
    /// Root element.
    pub root: Pt,
    /// Label (`span.p-avatar-label`).
    pub label: Pt,
    /// Icon (`span.p-avatar-icon`).
    pub icon: Pt,
    /// Image (`img`).
    pub image: Pt,
}

/// Avatar: a label, an icon or an image.
///
/// Every property accepts a value or a signal. Block priority: label, icon, image.
#[component]
pub fn Avatar(
    /// Label text.
    #[prop(optional, into)]
    label: OptProp<String>,
    /// Icon; rendered in `span.p-avatar-icon` when there is no label.
    #[prop(optional, into)]
    icon: Option<ViewFn>,
    /// Image URL; rendered when there is neither a label nor an icon.
    #[prop(optional, into)]
    image: OptProp<String>,
    /// Size (`normal` by default).
    #[prop(optional, into)]
    size: Prop<AvatarSize>,
    /// Shape (`square` by default).
    #[prop(optional, into)]
    shape: Prop<AvatarShape>,
    /// Inline style.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Additional class.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// `aria-label` attribute of the root and the image.
    #[prop(optional, into)]
    aria_label: OptProp<String>,
    /// `aria-labelledby` attribute of the root.
    #[prop(optional, into)]
    aria_labelledby: OptProp<String>,
    /// Called when the image fails to load.
    #[prop(optional, into)]
    on_image_error: Option<Callback<web_sys::Event>>,
    /// Pass-through.
    #[prop(optional)]
    pt: Option<AvatarPt>,
    /// Content before the label, icon and image.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let pt = pt.unwrap_or_default();
    let is_static = all_static!(label, image, size, shape, style, style_class, aria_label, aria_labelledby);
    let props = Props { label, image, size, shape, style, style_class, aria_label, aria_labelledby, icon };
    let content = children.map(|c| c());
    let attrs = pt.root.attrs();

    if is_static {
        let (class, style, data) = (class_of(&props, &pt), style_of(&props, &pt), data_of(&props));
        let (aria, labelledby) = (props.aria_label.get(), props.aria_labelledby.get());
        let block = block_of(&props, &pt, on_image_error);
        Either::Left(avatar_markup!(class, style, data, aria, labelledby, attrs, content, block))
    } else {
        let (props, pt) = (Arc::new(props), Arc::new(pt));
        let class = {
            let (props, pt) = (props.clone(), pt.clone());
            move || class_of(&props, &pt)
        };
        let style = {
            let (props, pt) = (props.clone(), pt.clone());
            move || style_of(&props, &pt)
        };
        let data = {
            let props = props.clone();
            move || data_of(&props)
        };
        let aria = {
            let props = props.clone();
            move || props.aria_label.get()
        };
        let labelledby = {
            let props = props.clone();
            move || props.aria_labelledby.get()
        };
        let block = move || block_of(&props, &pt, on_image_error);
        Either::Right(avatar_markup!(class, style, data, aria, labelledby, attrs, content, block))
    }
}

/// Avatar markup: shared by the static and reactive branches (only the attribute and block expressions differ).
macro_rules! avatar_markup {
    ($class:expr, $style:expr, $data:expr, $aria:expr, $labelledby:expr, $attrs:expr, $content:expr, $block:expr) => {
        view! {
            <div class={$class} style={$style} aria-label={$aria} aria-labelledby={$labelledby} data-p={$data} {..$attrs}>
                {$content}
                {$block}
            </div>
        }
    };
}
use avatar_markup;

fn state_of(props: &Props) -> AvatarState {
    AvatarState { has_image: props.image.with(Option::is_some), shape: props.shape.get(), size: props.size.get() }
}

fn class_of(props: &Props, pt: &AvatarPt) -> Cow<'static, str> {
    let mut cn = ClassNames::new();
    write_classes(&mut cn, &state_of(props));
    props.style_class.with(|c| cn.add_user(c.as_deref()));
    cn.add_user(pt.root.extra_class());
    cn.finish_cow()
}

fn style_of(props: &Props, pt: &AvatarPt) -> Option<String> {
    props.style.with(|st| (st.is_some() || pt.root.has_style()).then(|| pt.root.merge_style(st.as_deref().unwrap_or(""))))
}

fn data_of(props: &Props) -> Cow<'static, str> {
    data_p(&state_of(props))
}

/// One of the blocks: label, otherwise icon, otherwise image.
fn block_of(props: &Props, pt: &AvatarPt, on_image_error: Option<Callback<web_sys::Event>>) -> Option<AnyView> {
    let data = data_of(props);
    if let Some(label) = props.label.get().filter(|s| !s.is_empty()) {
        return Some(
            view! { <span class=pt.label.merge_class("p-avatar-label") data-p={data} {..pt.label.attrs()}>{label}</span> }.into_any(),
        );
    }
    if let Some(icon) = props.icon.clone() {
        return Some(
            view! { <span class=pt.icon.merge_class("p-avatar-icon") data-p={data} {..pt.icon.attrs()}>{icon.run()}</span> }.into_any(),
        );
    }
    let src = props.image.get()?;
    let aria = props.aria_label.get();
    let class = pt.image.extra_class().map(str::to_owned);
    Some(
        view! {
            <img
                src=src
                aria-label=aria
                data-p={data}
                class=class
                on:error=move |e| {
                    if let Some(cb) = on_image_error {
                        cb.run(e.into());
                    }
                }
                {..pt.image.attrs()}
            />
        }
        .into_any(),
    )
}

/// Instance properties.
struct Props {
    label: OptProp<String>,
    image: OptProp<String>,
    size: Prop<AvatarSize>,
    shape: Prop<AvatarShape>,
    style: OptProp<String>,
    style_class: OptProp<String>,
    aria_label: OptProp<String>,
    aria_labelledby: OptProp<String>,
    icon: Option<ViewFn>,
}
