//! Divider - port of PrimeNG 21.1.10 `p-divider`.
//!
//! Markup, classes, `role="separator"`, `aria-orientation`, the inline alignment style and `data-p` match PrimeNG;
//! instead of the `<p-divider>` host element a `<div class="p-divider ...">` with a `p-divider-content` block inside is rendered.
//!
//! Properties are [`Prop`]/[`OptProp`]; if all of them are static, the component renders ready-made values without closures and `RenderEffect`.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_divider::{Divider, DividerAlign, DividerType};
//!
//! let html = view! { <Divider align=DividerAlign::Right divider_type=DividerType::Dashed>"OR"</Divider> }.to_html();
//! assert!(html.contains("p-divider-right") && html.contains("p-divider-dashed") && html.contains("justify-content: flex-end"));
//! ```

mod state;

use leptos::either::Either;
use leptos::prelude::*;
use prime_core::{ClassNames, OptProp, Prop, Pt, all_static};
use std::borrow::Cow;
use std::sync::Arc;

pub use state::{DividerAlign, DividerLayout, DividerState, DividerType, align_style, data_p, write_classes};

/// Pass-through for Divider parts.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DividerPt {
    /// Root element.
    pub root: Pt,
    /// Content block (`div.p-divider-content`).
    pub content: Pt,
}

/// Divider: direction, line type, content alignment.
///
/// Each property accepts a value or a signal. `children` are rendered inside `div.p-divider-content`.
#[component]
pub fn Divider(
    /// Direction (`horizontal` by default).
    #[prop(optional, into)]
    layout: Prop<DividerLayout>,
    /// Line type (`solid` by default); there is no `type` attribute in the DOM.
    #[prop(optional, into)]
    divider_type: Prop<DividerType>,
    /// Content alignment.
    #[prop(optional, into)]
    align: OptProp<DividerAlign>,
    /// Inline style.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Extra class.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Pass-through.
    #[prop(optional)]
    pt: Option<DividerPt>,
    /// Content inside the `p-divider-content` block.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let pt = pt.unwrap_or_default();
    let is_static = all_static!(layout, divider_type, align, style, style_class);
    let props = Props { layout, divider_type, align, style, style_class };
    let content = children.map(|c| c());
    let (attrs, content_class, content_attrs) = (pt.root.attrs(), pt.content.merge_class("p-divider-content"), pt.content.attrs());

    if is_static {
        let (class, style, data, orientation) =
            (class_of(&props, &pt), style_of(&props, &pt), data_of(&props), props.layout.get().as_str());
        Either::Left(divider_markup!(class, style, data, orientation, attrs, content_class, content_attrs, content))
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
        let orientation = move || props.layout.get().as_str();
        Either::Right(divider_markup!(class, style, data, orientation, attrs, content_class, content_attrs, content))
    }
}

/// Divider markup: one for the static and reactive branches (only attribute expressions differ).
macro_rules! divider_markup {
    ($class:expr, $style:expr, $data:expr, $orientation:expr, $attrs:expr, $content_class:expr, $content_attrs:expr, $content:expr) => {
        view! {
            <div class={$class} style={$style} role="separator" aria-orientation={$orientation} data-p={$data} {..$attrs}>
                <div class={$content_class} {..$content_attrs}>
                    {$content}
                </div>
            </div>
        }
    };
}
use divider_markup;

fn state_of(props: &Props) -> DividerState {
    DividerState { layout: props.layout.get(), divider_type: props.divider_type.get(), align: props.align.get() }
}

fn class_of(props: &Props, pt: &DividerPt) -> Cow<'static, str> {
    let mut cn = ClassNames::new();
    write_classes(&mut cn, &state_of(props));
    props.style_class.with(|c| cn.add_user(c.as_deref()));
    cn.add_user(pt.root.extra_class());
    cn.finish_cow()
}

/// Root style: alignment, then `style` and the style from `pt`.
fn style_of(props: &Props, pt: &DividerPt) -> Option<String> {
    let align = align_style(&state_of(props)).map(|(k, v)| format!("{k}: {v}")).unwrap_or_default();
    props.style.with(|st| {
        let merged = pt.root.merge_style(&Pt::new().style(st.as_deref().unwrap_or("")).merge_style(&align));
        (!merged.is_empty()).then_some(merged)
    })
}

fn data_of(props: &Props) -> Cow<'static, str> {
    data_p(&state_of(props))
}

/// Instance properties.
struct Props {
    layout: Prop<DividerLayout>,
    divider_type: Prop<DividerType>,
    align: OptProp<DividerAlign>,
    style: OptProp<String>,
    style_class: OptProp<String>,
}
