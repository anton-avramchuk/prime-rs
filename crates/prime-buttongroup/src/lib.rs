//! ButtonGroup — port of `p-buttongroup` from PrimeNG 21.1.10.
//!
//! Instead of the host `<p-buttongroup>` a `<span class="p-buttongroup p-component" role="group">` is rendered
//! with projected buttons. The joined look (a shared border, rounded corners only on the outer buttons) is set by the theme CSS.
//! The `/* For PrimeNG */` block from `buttongroupstyle.ts` targets the `p-button` hosts and is not needed here.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_buttongroup::ButtonGroup;
//!
//! let html = view! { <ButtonGroup><button class="p-button">"A"</button><button class="p-button">"B"</button></ButtonGroup> }.to_html();
//! assert!(html.contains("p-buttongroup p-component") && html.contains(r#"role="group""#));
//! ```

use leptos::either::Either;
use leptos::prelude::*;
use prime_rs_core::wrapper::{root_class, root_style};
use prime_rs_core::{OptProp, Pt, all_static};
use std::sync::Arc;

/// Pass-through for ButtonGroup.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ButtonGroupPt {
    /// Root element.
    pub root: Pt,
}

/// Group of joined buttons.
///
/// Properties accept a value or a signal.
#[component]
pub fn ButtonGroup(
    /// Inline style.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Extra class.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Pass-through.
    #[prop(optional)]
    pt: Option<ButtonGroupPt>,
    /// Content.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let root = pt.unwrap_or_default().root;
    let content = children.map(|c| c());
    let attrs = root.attrs();

    if all_static!(style, style_class) {
        let (class, style) = (root_class("p-buttongroup p-component", &style_class, &root), root_style(&style, &root));
        Either::Left(buttongroup_markup!(class, style, attrs, content))
    } else {
        let root = Arc::new(root);
        let class = {
            let root = root.clone();
            move || root_class("p-buttongroup p-component", &style_class, &root)
        };
        let style = move || root_style(&style, &root);
        Either::Right(buttongroup_markup!(class, style, attrs, content))
    }
}

/// Markup: one for the static and reactive branches (only the root attribute expressions differ).
macro_rules! buttongroup_markup {
    ($class:expr, $style:expr, $attrs:expr, $content:expr) => {
        view! { <span class={$class} style={$style} role="group" {..$attrs}>{$content}</span> }
    };
}
use buttongroup_markup;
