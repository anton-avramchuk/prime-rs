//! Fluid — port of `p-fluid` from PrimeNG 21.1.10.
//!
//! Renders `<div class="p-fluid">` and provides descendants with the context [`prime_core::fluid`]: `Button`, `InputText`,
//! `Textarea`, `InputNumber` and `Select` without an explicit `fluid` stretch to the full width. An explicit `fluid` (including `false`)
//! takes precedence over the context. `Fluid` has no styles of its own (there is no `@primeuix/styles/fluid` module).
//!
//! ```
//! use leptos::prelude::*;
//! use prime_fluid::Fluid;
//!
//! let html = view! { <Fluid><input class="p-inputtext" /></Fluid> }.to_html();
//! assert!(html.contains(r#"class="p-fluid""#));
//! ```

use leptos::either::Either;
use leptos::prelude::*;
use prime_core::wrapper::{root_class, root_style};
use prime_core::{OptProp, Pt, all_static};
use std::sync::Arc;

/// Pass-through for Fluid.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FluidPt {
    /// Root element.
    pub root: Pt,
}

/// Container that stretches nested fields and buttons to the full width.
///
/// Properties accept a value or a signal.
#[component]
pub fn Fluid(
    /// Inline style.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Extra class.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Pass-through.
    #[prop(optional)]
    pt: Option<FluidPt>,
    /// Content.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let root = pt.unwrap_or_default().root;
    // The context is put into a child owner, so it does not leak to siblings that come after `Fluid` in the same owner.
    // The owner is kept alive until the parent is cleaned up: on its own it would be released immediately and take the descendants' reactive values with it.
    let content = children.map(|c| match Owner::current() {
        Some(parent) => {
            let owner = parent.child();
            let content = owner.with(|| {
                prime_core::fluid::provide_fluid();
                c()
            });
            on_cleanup(move || drop(owner));
            content
        }
        None => c(),
    });
    let attrs = root.attrs();

    if all_static!(style, style_class) {
        let (class, style) = (root_class("p-fluid", &style_class, &root), root_style(&style, &root));
        Either::Left(fluid_markup!(class, style, attrs, content))
    } else {
        let root = Arc::new(root);
        let class = {
            let root = root.clone();
            move || root_class("p-fluid", &style_class, &root)
        };
        let style = move || root_style(&style, &root);
        Either::Right(fluid_markup!(class, style, attrs, content))
    }
}

/// Markup: one for the static and reactive branches (only the root attribute expressions differ).
macro_rules! fluid_markup {
    ($class:expr, $style:expr, $attrs:expr, $content:expr) => {
        view! { <div class={$class} style={$style} {..$attrs}>{$content}</div> }
    };
}
use fluid_markup;
