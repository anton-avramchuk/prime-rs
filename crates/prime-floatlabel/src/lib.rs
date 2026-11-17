//! FloatLabel — port of `p-floatlabel` from PrimeNG 21.1.10.
//!
//! Renders `<span class="p-floatlabel p-floatlabel-over|on|in">` with a nested field and `<label>`; the position and lift
//! of the label are set by the theme CSS according to the field state. The `/* For PrimeNG */` block (`.ng-invalid.ng-dirty`) is not needed here:
//! the color of an invalid value comes from the base rule `:has(.p-invalid) label`.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_floatlabel::{FloatLabel, FloatLabelVariant};
//!
//! let html = view! {
//!     <FloatLabel variant=FloatLabelVariant::On><input class="p-inputtext" id="u" /><label for="u">"Name"</label></FloatLabel>
//! }.to_html();
//! assert!(html.contains("p-floatlabel p-floatlabel-on"));
//! ```

use leptos::either::Either;
use leptos::prelude::*;
use prime_rs_core::wrapper::root_style;
use prime_rs_core::{ClassNames, OptProp, Prop, Pt, all_static};
use std::borrow::Cow;
use std::sync::Arc;

/// Label placement variant.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FloatLabelVariant {
    /// Label above the field when lifted (default).
    #[default]
    Over,
    /// Label sits on the top border of the field.
    On,
    /// Label inside the field.
    In,
}

/// Appends the own root classes: `p-floatlabel` and exactly one variant class.
pub fn write_classes(cn: &mut ClassNames, variant: FloatLabelVariant) {
    cn.add(match variant {
        FloatLabelVariant::Over => "p-floatlabel p-floatlabel-over",
        FloatLabelVariant::On => "p-floatlabel p-floatlabel-on",
        FloatLabelVariant::In => "p-floatlabel p-floatlabel-in",
    });
}

/// Pass-through for FloatLabel.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FloatLabelPt {
    /// Root element.
    pub root: Pt,
}

/// Label that rises on focus or input.
///
/// Properties accept a value or a signal.
#[component]
pub fn FloatLabel(
    /// Label placement.
    #[prop(optional, into)]
    variant: Prop<FloatLabelVariant>,
    /// Inline style.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Extra class.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Pass-through.
    #[prop(optional)]
    pt: Option<FloatLabelPt>,
    /// Content.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let root = pt.unwrap_or_default().root;
    let content = children.map(|c| c());
    let attrs = root.attrs();

    if all_static!(style, style_class, variant) {
        let (class, style) = (class_of(variant.get_untracked(), &style_class, &root), root_style(&style, &root));
        Either::Left(floatlabel_markup!(class, style, attrs, content))
    } else {
        let root = Arc::new(root);
        let class = {
            let root = root.clone();
            move || class_of(variant.get(), &style_class, &root)
        };
        let style = move || root_style(&style, &root);
        Either::Right(floatlabel_markup!(class, style, attrs, content))
    }
}

/// Markup: one for the static and reactive branches (only the root attribute expressions differ).
macro_rules! floatlabel_markup {
    ($class:expr, $style:expr, $attrs:expr, $content:expr) => {
        view! { <span class={$class} style={$style} {..$attrs}>{$content}</span> }
    };
}
use floatlabel_markup;

fn class_of(variant: FloatLabelVariant, style_class: &OptProp<String>, root: &Pt) -> Cow<'static, str> {
    let mut cn = ClassNames::new();
    write_classes(&mut cn, variant);
    style_class.with(|c| cn.add_user(c.as_deref()));
    cn.add_user(root.extra_class());
    cn.finish_cow()
}
