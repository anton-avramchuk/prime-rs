//! Skeleton — port of `p-skeleton` from PrimeNG 21.1.10.
//!
//! Loading placeholder: `div.p-skeleton.p-component[aria-hidden=true]` (instead of the `<p-skeleton>` host); shape, size and rounding are set
//! by properties and end up in the inline style, as in PrimeNG.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_skeleton::{Skeleton, SkeletonShape};
//!
//! let html = view! { <Skeleton shape=SkeletonShape::Circle size="4rem" /> }.to_html();
//! assert!(html.contains("p-skeleton-circle") && html.contains("width: 4rem; height: 4rem;"));
//! ```

mod state;

use leptos::either::Either;
use leptos::prelude::*;
use prime_rs_core::{ClassNames, OptProp, Prop, Pt, all_static};
use std::borrow::Cow;
use std::sync::Arc;

pub use state::{SkeletonAnimation, SkeletonShape, container_style, data_p, write_classes};

/// Loading placeholder.
///
/// Each property accepts a value or a signal.
#[component]
pub fn Skeleton(
    /// Shape (rectangle by default).
    #[prop(optional, into)]
    shape: Prop<SkeletonShape>,
    /// Animation (wave by default).
    #[prop(optional, into)]
    animation: Prop<SkeletonAnimation>,
    /// Rounding (CSS `border-radius`).
    #[prop(optional, into)]
    border_radius: OptProp<String>,
    /// Size: width and height at once; takes priority over `width` and `height`.
    #[prop(optional, into)]
    size: OptProp<String>,
    /// Width (`100%` by default).
    #[prop(optional, into)]
    width: OptProp<String>,
    /// Height (`1rem` by default).
    #[prop(optional, into)]
    height: OptProp<String>,
    /// Extra class of the root.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Pass-through of the root.
    #[prop(optional)]
    pt: Option<Pt>,
) -> impl IntoView {
    let pt = pt.unwrap_or_default();
    let attrs = pt.attrs();
    let is_static = all_static!(shape, animation, border_radius, size, width, height, style_class);
    let props = Props { shape, animation, border_radius, size, width, height, style_class };

    if is_static {
        let (class, style, data) = (class_of(&props, &pt), style_of(&props, &pt), data_p(props.shape.get()));
        Either::Left(skeleton_markup!(class, style, data, attrs))
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
        let data = move || data_p(props.shape.get());
        Either::Right(skeleton_markup!(class, style, data, attrs))
    }
}

/// Skeleton markup: one for the static and reactive branches (only the attribute expressions change).
macro_rules! skeleton_markup {
    ($class:expr, $style:expr, $data:expr, $attrs:expr) => {
        view! { <div aria-hidden="true" class={$class} style={$style} data-p={$data} {..$attrs}></div> }
    };
}
use skeleton_markup;

fn class_of(props: &Props, pt: &Pt) -> Cow<'static, str> {
    let mut cn = ClassNames::new();
    write_classes(&mut cn, props.shape.get(), props.animation.get());
    props.style_class.with(|c| cn.add_user(c.as_deref()));
    cn.add_user(pt.extra_class());
    cn.finish_cow()
}

fn style_of(props: &Props, pt: &Pt) -> String {
    props.size.with(|size| {
        props.width.with(|width| {
            props.height.with(|height| {
                props.border_radius.with(|radius| {
                    let own = container_style(size.as_deref(), width.as_deref(), height.as_deref(), radius.as_deref());
                    if pt.has_style() { pt.merge_style(&own) } else { own }
                })
            })
        })
    })
}

/// Instance properties.
struct Props {
    shape: Prop<SkeletonShape>,
    animation: Prop<SkeletonAnimation>,
    border_radius: OptProp<String>,
    size: OptProp<String>,
    width: OptProp<String>,
    height: OptProp<String>,
    style_class: OptProp<String>,
}
