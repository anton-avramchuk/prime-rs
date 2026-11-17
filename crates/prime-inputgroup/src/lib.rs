//! InputGroup and InputGroupAddon - port of PrimeNG 21.1.10 `p-inputgroup` and `p-inputgroup-addon`.
//!
//! Structure and classes match PrimeNG: `div.p-inputgroup` and `div.p-inputgroupaddon` with projected content.
//! Instead of the `<p-inputgroup>` and `<p-inputgroup-addon>` hosts, `<div class="...">` is rendered. The `/*For PrimeNG*/` block
//! from `inputgroupstyle.ts` (rules for the Angular wrappers `p-button`, `p-inputwrapper`, `p-inputmask`) is not needed.
//!
//! Properties are [`OptProp`]; if all of them are static, the component renders ready-made values without closures and `RenderEffect`.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_inputgroup::{InputGroup, InputGroupAddon};
//!
//! let html = view! { <InputGroup><InputGroupAddon>"$"</InputGroupAddon><input class="p-inputtext" /></InputGroup> }.to_html();
//! assert!(html.contains("p-inputgroup") && html.contains("p-inputgroupaddon"));
//! ```

use leptos::either::Either;
use leptos::prelude::*;
use prime_rs_core::{ClassNames, OptProp, Pt, all_static};
use std::borrow::Cow;
use std::sync::Arc;

/// Pass-through InputGroup.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct InputGroupPt {
    /// Root element.
    pub root: Pt,
}

/// Pass-through InputGroupAddon.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct InputGroupAddonPt {
    /// Root element.
    pub root: Pt,
}

/// Group: an input with text, icons and buttons next to it.
///
/// Properties accept a value or a signal.
#[component]
pub fn InputGroup(
    /// Inline style.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Extra class.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Pass-through.
    #[prop(optional)]
    pt: Option<InputGroupPt>,
    /// Group content.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    group_view("p-inputgroup", style, style_class, pt.unwrap_or_default().root, children)
}

/// Group addon: text, an icon or a button next to the input.
///
/// Properties accept a value or a signal.
#[component]
pub fn InputGroupAddon(
    /// Inline style.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Extra class.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Pass-through.
    #[prop(optional)]
    pt: Option<InputGroupAddonPt>,
    /// Addon content.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    group_view("p-inputgroupaddon", style, style_class, pt.unwrap_or_default().root, children)
}

/// Markup shared by both components; only the own class differs.
fn group_view(
    base: &'static str,
    style: OptProp<String>,
    style_class: OptProp<String>,
    root: Pt,
    children: Option<Children>,
) -> impl IntoView {
    let is_static = all_static!(style, style_class);
    let props = Props { style, style_class };
    let content = children.map(|c| c());
    let attrs = root.attrs();

    if is_static {
        let (class, style) = (class_of(base, &props, &root), style_of(&props, &root));
        Either::Left(group_markup!(class, style, attrs, content))
    } else {
        let (props, root) = (Arc::new(props), Arc::new(root));
        let class = {
            let (props, root) = (props.clone(), root.clone());
            move || class_of(base, &props, &root)
        };
        let style = move || style_of(&props, &root);
        Either::Right(group_markup!(class, style, attrs, content))
    }
}

/// Markup: one for the static and reactive branches (only the root attribute expressions differ).
macro_rules! group_markup {
    ($class:expr, $style:expr, $attrs:expr, $content:expr) => {
        view! { <div class={$class} style={$style} {..$attrs}>{$content}</div> }
    };
}
use group_markup;

fn class_of(base: &'static str, props: &Props, root: &Pt) -> Cow<'static, str> {
    let mut cn = ClassNames::new();
    cn.add(base);
    props.style_class.with(|c| cn.add_user(c.as_deref()));
    cn.add_user(root.extra_class());
    cn.finish_cow()
}

fn style_of(props: &Props, root: &Pt) -> Option<String> {
    props.style.with(|st| (st.is_some() || root.has_style()).then(|| root.merge_style(st.as_deref().unwrap_or(""))))
}

/// Instance properties.
struct Props {
    style: OptProp<String>,
    style_class: OptProp<String>,
}
