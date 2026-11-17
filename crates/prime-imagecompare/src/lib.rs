//! ImageCompare — port of `p-imagecompare` from PrimeNG 21.1.10.
//!
//! Markup and classes as in PrimeNG: root `div.p-imagecompare` with two views (`left`, `right`, usually `<img>`) and
//! a slider `input[type=range].p-imagecompare-slider`. Input on the slider clips the `clip-path` of the element in front of it
//! (the right image); the theme rule `img + img` sets the default clipping (50 %).
//!
//! ```
//! use leptos::prelude::*;
//! use prime_imagecompare::ImageCompare;
//!
//! let html = Owner::new()
//!     .with(|| view! { <ImageCompare left=|| view! { <img src="a.jpg" /> } right=|| view! { <img src="b.jpg" /> } /> }.to_html());
//! assert!(html.contains("p-imagecompare-slider") && html.contains("type=\"range\"") && html.contains("a.jpg"));
//! ```

mod state;

use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use prime_core::{OptProp, Pt};
use std::sync::Arc;

pub use state::clip_path;

/// Pass-through for the parts of ImageCompare.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ImageComparePt {
    /// Root element.
    pub root: Pt,
    /// Slider.
    pub slider: Pt,
}

/// Comparison of two images with a slider.
///
/// `left` and `right` are rendered without wrappers; for the theme rule `img + img` both should be `<img>`.
#[component]
pub fn ImageCompare(
    /// Left view (the bottom image).
    #[prop(into)]
    left: ViewFn,
    /// Right view (the top image; the slider clips it).
    #[prop(into)]
    right: ViewFn,
    /// `tabindex` attribute of the root.
    #[prop(optional, into)]
    tabindex: OptProp<i32>,
    /// `aria-label` attribute of the root.
    #[prop(optional, into)]
    aria_label: OptProp<String>,
    /// `aria-labelledby` attribute of the root.
    #[prop(optional, into)]
    aria_labelledby: OptProp<String>,
    /// Inline style of the root.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Extra class of the root.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Pass-through for the parts.
    #[prop(optional)]
    pt: Option<ImageComparePt>,
) -> impl IntoView {
    let pt = Arc::new(pt.unwrap_or_default());
    let class = {
        let pt = pt.clone();
        move || {
            let mut cn = prime_core::ClassNames::new();
            cn.add("p-imagecompare");
            style_class.with(|c| cn.add_user(c.as_deref()));
            cn.add_user(pt.root.extra_class());
            cn.finish_cow()
        }
    };
    let root_style = {
        let pt = pt.clone();
        move || {
            let merged = style.with(|s| pt.root.merge_style(s.as_deref().unwrap_or_default()));
            (!merged.is_empty()).then_some(merged)
        }
    };
    let slider_class = pt.slider.merge_class("p-imagecompare-slider");
    let (root_attrs, slider_attrs) = (pt.root.attrs(), pt.slider.attrs());
    let on_input = move |e: leptos::web_sys::Event| slide(&e);

    view! {
        <div class={class} style={root_style} tabindex={move || tabindex.get()} aria-label={move || aria_label.get()} aria-labelledby={move || aria_labelledby.get()} {..root_attrs}>
            {left.run()}
            {right.run()}
            <input type="range" min="0" max="100" value="50" class={slider_class} on:input={on_input} {..slider_attrs} />
        </div>
    }
}

/// Input handler (`onSlide`): clips the element in front of the slider; the direction is taken from the nearest `dir="rtl"`.
fn slide(event: &leptos::web_sys::Event) {
    let Some(input) = event.target().and_then(|t| t.dyn_into::<leptos::web_sys::HtmlInputElement>().ok()) else { return };
    let Ok(value) = input.value().parse::<f64>() else { return };
    let Some(image) = input.previous_element_sibling().and_then(|p| p.dyn_into::<leptos::web_sys::HtmlElement>().ok()) else { return };
    let rtl = input.closest("[dir=\"rtl\"]").ok().flatten().is_some();
    let _ = image.style().set_property("clip-path", &clip_path(value, rtl));
}
