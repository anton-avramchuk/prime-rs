//! Tag — port of `p-tag` from PrimeNG 21.1.10.
//!
//! Markup and classes match PrimeNG (`p-tag p-component` + modifiers, `data-p`, order of `children`, icon, label),
//! but a `<span>` is rendered instead of the `<p-tag>` host element: the theme CSS is bound to classes, not to the tag name.
//!
//! Properties are [`Prop`]/[`OptProp`]; if all of them are static, the component renders ready-made values without closures and `RenderEffect`.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_tag::{Tag, TagSeverity};
//!
//! let html = view! { <Tag value="New" severity=TagSeverity::Success rounded=true /> }.to_html();
//! assert!(html.contains("p-tag-success") && html.contains("p-tag-rounded") && html.contains(">New<"));
//! ```

mod state;

use leptos::either::Either;
use leptos::prelude::*;
use prime_core::{ClassNames, OptProp, Prop, Pt, all_static};
use std::borrow::Cow;
use std::sync::Arc;

pub use state::{TagSeverity, TagState, data_p, write_classes};

/// Pass-through of Tag parts.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TagPt {
    /// Root element.
    pub root: Pt,
    /// Icon wrapper (`span.p-tag-icon`).
    pub icon: Pt,
    /// Label (`span.p-tag-label`).
    pub label: Pt,
}

/// Tag: value, severity, rounding, icon and arbitrary content.
///
/// Each property accepts a value or a signal. `children` are rendered first, then the icon and the label (as in PrimeNG).
#[component]
pub fn Tag(
    /// Label text.
    #[prop(optional, into)]
    value: OptProp<String>,
    /// Severity.
    #[prop(optional, into)]
    severity: OptProp<TagSeverity>,
    /// Rounded edges.
    #[prop(optional, into)]
    rounded: Prop<bool>,
    /// Icon; rendered in `span.p-tag-icon`.
    #[prop(optional, into)]
    icon: Option<ViewFn>,
    /// Inline style.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Extra class.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Pass-through.
    #[prop(optional)]
    pt: Option<TagPt>,
    /// Content before the icon and label.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let pt = pt.unwrap_or_default();
    let is_static = all_static!(value, severity, rounded, style, style_class);
    let props = Props { value, severity, rounded, style, style_class };
    let content = children.map(|c| c());
    let icon_view = icon.map(|icon| view! { <span class=pt.icon.merge_class("p-tag-icon") {..pt.icon.attrs()}>{icon.run()}</span> });
    let (attrs, label_class, label_attrs) = (pt.root.attrs(), pt.label.merge_class("p-tag-label"), pt.label.attrs());

    if is_static {
        let (class, style, data, value) = (class_of(&props, &pt), style_of(&props, &pt), data_of(&props), props.value.get());
        Either::Left(tag_markup!(class, style, data, attrs, content, icon_view, label_class, label_attrs, value))
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
        let value = move || props.value.get();
        Either::Right(tag_markup!(class, style, data, attrs, content, icon_view, label_class, label_attrs, value))
    }
}

/// Tag markup: one for the static and reactive branches (only attribute and text expressions differ).
macro_rules! tag_markup {
    ($class:expr, $style:expr, $data:expr, $attrs:expr, $content:expr, $icon:expr, $label_class:expr, $label_attrs:expr, $value:expr) => {
        view! {
            <span class={$class} style={$style} data-p={$data} {..$attrs}>
                {$content}
                {$icon}
                <span class={$label_class} {..$label_attrs}>
                    {$value}
                </span>
            </span>
        }
    };
}
use tag_markup;

fn state_of(props: &Props) -> TagState {
    TagState { severity: props.severity.get(), rounded: props.rounded.get() }
}

fn class_of(props: &Props, pt: &TagPt) -> Cow<'static, str> {
    let mut cn = ClassNames::new();
    write_classes(&mut cn, &state_of(props));
    props.style_class.with(|c| cn.add_user(c.as_deref()));
    cn.add_user(pt.root.extra_class());
    cn.finish_cow()
}

fn style_of(props: &Props, pt: &TagPt) -> Option<String> {
    props.style.with(|st| (st.is_some() || pt.root.has_style()).then(|| pt.root.merge_style(st.as_deref().unwrap_or(""))))
}

fn data_of(props: &Props) -> Cow<'static, str> {
    data_p(&state_of(props))
}

/// Instance properties.
struct Props {
    value: OptProp<String>,
    severity: OptProp<TagSeverity>,
    rounded: Prop<bool>,
    style: OptProp<String>,
    style_class: OptProp<String>,
}
