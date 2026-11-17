//! IconField and InputIcon — port of `p-iconfield` and `p-inputicon` from PrimeNG 21.1.10.
//!
//! `IconField` renders `<span class="p-iconfield p-iconfield-left|right">`, `InputIcon` — `<span class="p-inputicon">`.
//! The icon position inside the field is set by the theme CSS according to the order of child elements (`:first-child`/`:last-child`); the class
//! `p-iconfield-left|right` is informational. The icon is passed as a child element (for example an SVG from `prime-icons`).
//! `InputIcon` has no theme of its own in PrimeNG: its rules are part of `iconfield`.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_iconfield::{IconField, InputIcon};
//!
//! let html = view! { <IconField><InputIcon><svg /></InputIcon><input class="p-inputtext" /></IconField> }.to_html();
//! assert!(html.contains("p-iconfield p-iconfield-left") && html.contains("p-inputicon"));
//! ```

use leptos::either::Either;
use leptos::prelude::*;
use prime_core::wrapper::{root_class, root_style};
use prime_core::{ClassNames, OptProp, Prop, Pt, all_static};
use std::borrow::Cow;
use std::sync::Arc;

/// Icon position relative to the field.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum IconPosition {
    /// Icon on the left (default).
    #[default]
    Left,
    /// Icon on the right.
    Right,
}

/// Appends the own root classes of `IconField`.
pub fn write_classes(cn: &mut ClassNames, position: IconPosition) {
    cn.add(match position {
        IconPosition::Left => "p-iconfield p-iconfield-left",
        IconPosition::Right => "p-iconfield p-iconfield-right",
    });
}

/// Pass-through for IconField.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct IconFieldPt {
    /// Root element.
    pub root: Pt,
}

/// Field with an icon inside.
///
/// Properties accept a value or a signal.
#[component]
pub fn IconField(
    /// Icon position.
    #[prop(optional, into)]
    icon_position: Prop<IconPosition>,
    /// Inline style.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Extra class.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Pass-through.
    #[prop(optional)]
    pt: Option<IconFieldPt>,
    /// Content.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let root = pt.unwrap_or_default().root;
    let content = children.map(|c| c());
    let attrs = root.attrs();

    if all_static!(style, style_class, icon_position) {
        let (class, style) = (class_of(icon_position.get_untracked(), &style_class, &root), root_style(&style, &root));
        Either::Left(iconfield_markup!(class, style, attrs, content))
    } else {
        let root = Arc::new(root);
        let class = {
            let root = root.clone();
            move || class_of(icon_position.get(), &style_class, &root)
        };
        let style = move || root_style(&style, &root);
        Either::Right(iconfield_markup!(class, style, attrs, content))
    }
}

/// Markup: one for the static and reactive branches (only the root attribute expressions differ).
macro_rules! iconfield_markup {
    ($class:expr, $style:expr, $attrs:expr, $content:expr) => {
        view! { <span class={$class} style={$style} {..$attrs}>{$content}</span> }
    };
}
use iconfield_markup;

fn class_of(icon_position: IconPosition, style_class: &OptProp<String>, root: &Pt) -> Cow<'static, str> {
    let mut cn = ClassNames::new();
    write_classes(&mut cn, icon_position);
    style_class.with(|c| cn.add_user(c.as_deref()));
    cn.add_user(root.extra_class());
    cn.finish_cow()
}

/// Pass-through for InputIcon.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct InputIconPt {
    /// Root element.
    pub root: Pt,
}

/// Icon inside `IconField`.
///
/// Properties accept a value or a signal.
#[component]
pub fn InputIcon(
    /// Inline style.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Extra class.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Pass-through.
    #[prop(optional)]
    pt: Option<InputIconPt>,
    /// Content.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let root = pt.unwrap_or_default().root;
    let content = children.map(|c| c());
    let attrs = root.attrs();

    if all_static!(style, style_class) {
        let (class, style) = (root_class("p-inputicon", &style_class, &root), root_style(&style, &root));
        Either::Left(inputicon_markup!(class, style, attrs, content))
    } else {
        let root = Arc::new(root);
        let class = {
            let root = root.clone();
            move || root_class("p-inputicon", &style_class, &root)
        };
        let style = move || root_style(&style, &root);
        Either::Right(inputicon_markup!(class, style, attrs, content))
    }
}

/// Markup: one for the static and reactive branches (only the root attribute expressions differ).
macro_rules! inputicon_markup {
    ($class:expr, $style:expr, $attrs:expr, $content:expr) => {
        view! { <span class={$class} style={$style} {..$attrs}>{$content}</span> }
    };
}
use inputicon_markup;
