//! ProgressSpinner — port of `p-progressspinner` from PrimeNG 21.1.10.
//!
//! Markup and classes match PrimeNG: root `div.p-progressspinner` (`role="progressbar"`, `aria-busy="true"`) with
//! `svg.p-progressspinner-spin` and `circle.p-progressspinner-circle` inside. Instead of the host `<p-progressspinner>`
//! a `<div class="p-progressspinner">` is rendered.
//!
//! Properties are [`Prop`]/[`OptProp`]; if all of them are static, the component renders ready values without closures and `RenderEffect`.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_progressspinner::ProgressSpinner;
//!
//! let html = view! { <ProgressSpinner aria_label="loading" stroke_width="8" /> }.to_html();
//! assert!(html.contains(r#"role="progressbar""#) && html.contains(r#"stroke-width="8""#) && html.contains(r#"aria-label="loading""#));
//! ```

use leptos::either::Either;
use leptos::prelude::*;
use prime_core::{ClassNames, OptProp, Pt, all_static};
use std::borrow::Cow;
use std::sync::Arc;

/// Default line thickness.
const DEFAULT_STROKE_WIDTH: &str = "2";
/// Default fill.
const DEFAULT_FILL: &str = "none";
/// Default rotation duration.
const DEFAULT_DURATION: &str = "2s";

/// Pass-through for the parts of ProgressSpinner.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ProgressSpinnerPt {
    /// Root element.
    pub root: Pt,
    /// Rotating `svg` (`.p-progressspinner-spin`).
    pub spin: Pt,
    /// Circle (`.p-progressspinner-circle`).
    pub circle: Pt,
}

/// Loading indicator.
///
/// Properties accept a value or a signal.
#[component]
pub fn ProgressSpinner(
    /// Circle line thickness (default `2`).
    #[prop(optional, into)]
    stroke_width: OptProp<String>,
    /// Circle fill (default `none`).
    #[prop(optional, into)]
    fill: OptProp<String>,
    /// Rotation duration (default `2s`).
    #[prop(optional, into)]
    animation_duration: OptProp<String>,
    /// `aria-label` attribute.
    #[prop(optional, into)]
    aria_label: OptProp<String>,
    /// Inline style of the root.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Extra class of the root.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Pass-through.
    #[prop(optional)]
    pt: Option<ProgressSpinnerPt>,
) -> impl IntoView {
    let pt = pt.unwrap_or_default();
    let is_static = all_static!(stroke_width, fill, animation_duration, aria_label, style, style_class);
    let props = Props { stroke_width, fill, animation_duration, aria_label, style, style_class };
    let (root_attrs, spin_attrs, circle_attrs) = (pt.root.attrs(), pt.spin.attrs(), pt.circle.attrs());
    let spin_class = pt.spin.merge_class("p-progressspinner-spin");
    let circle_class = pt.circle.merge_class("p-progressspinner-circle");
    let circle_style = pt.circle.has_style().then(|| pt.circle.merge_style(""));

    if is_static {
        let (class, style, aria) = (class_of(&props, &pt), style_of(&props, &pt), props.aria_label.get());
        let (spin_style, stroke, fill) = (spin_style_of(&props, &pt), stroke_of(&props), fill_of(&props));
        Either::Left(spinner_markup!(
            class,
            style,
            aria,
            root_attrs,
            spin_class,
            spin_style,
            spin_attrs,
            circle_class,
            circle_style,
            circle_attrs,
            fill,
            stroke
        ))
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
        let aria = {
            let props = props.clone();
            move || props.aria_label.get()
        };
        let spin_style = {
            let (props, pt) = (props.clone(), pt);
            move || spin_style_of(&props, &pt)
        };
        let stroke = {
            let props = props.clone();
            move || stroke_of(&props)
        };
        let fill = move || fill_of(&props);
        Either::Right(spinner_markup!(
            class,
            style,
            aria,
            root_attrs,
            spin_class,
            spin_style,
            spin_attrs,
            circle_class,
            circle_style,
            circle_attrs,
            fill,
            stroke
        ))
    }
}

/// ProgressSpinner markup: one for the static and reactive branches (only the attribute expressions differ).
macro_rules! spinner_markup {
    (
        $class:expr, $style:expr, $aria:expr, $root_attrs:expr, $spin_class:expr, $spin_style:expr, $spin_attrs:expr,
        $circle_class:expr, $circle_style:expr, $circle_attrs:expr, $fill:expr, $stroke:expr
    ) => {
        view! {
            <div class={$class} style={$style} role="progressbar" aria-busy="true" aria-label={$aria} {..$root_attrs}>
                <svg class={$spin_class} style={$spin_style} viewBox="25 25 50 50" {..$spin_attrs}>
                    <circle
                        class={$circle_class}
                        style={$circle_style}
                        cx="50"
                        cy="50"
                        r="20"
                        fill={$fill}
                        stroke-width={$stroke}
                        stroke-miterlimit="10"
                        {..$circle_attrs}
                    ></circle>
                </svg>
            </div>
        }
    };
}
use spinner_markup;

fn class_of(props: &Props, pt: &ProgressSpinnerPt) -> Cow<'static, str> {
    let mut cn = ClassNames::new();
    cn.add("p-progressspinner");
    props.style_class.with(|c| cn.add_user(c.as_deref()));
    cn.add_user(pt.root.extra_class());
    cn.finish_cow()
}

fn style_of(props: &Props, pt: &ProgressSpinnerPt) -> Option<String> {
    props.style.with(|st| (st.is_some() || pt.root.has_style()).then(|| pt.root.merge_style(st.as_deref().unwrap_or(""))))
}

fn spin_style_of(props: &Props, pt: &ProgressSpinnerPt) -> String {
    let duration = props.animation_duration.get();
    pt.spin.merge_style(&format!("animation-duration: {};", duration.as_deref().unwrap_or(DEFAULT_DURATION)))
}

fn stroke_of(props: &Props) -> String {
    props.stroke_width.get().unwrap_or_else(|| DEFAULT_STROKE_WIDTH.to_owned())
}

fn fill_of(props: &Props) -> String {
    props.fill.get().unwrap_or_else(|| DEFAULT_FILL.to_owned())
}

/// Instance properties.
struct Props {
    stroke_width: OptProp<String>,
    fill: OptProp<String>,
    animation_duration: OptProp<String>,
    aria_label: OptProp<String>,
    style: OptProp<String>,
    style_class: OptProp<String>,
}
