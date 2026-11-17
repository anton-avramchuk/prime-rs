//! IftaLabel — port of `p-iftalabel` from PrimeNG 21.1.10.
//!
//! Renders `<span class="p-iftalabel">` with a nested field and `<label>`; the position of the label above the value and the paddings of the
//! field are set by the theme CSS. The `/* For PrimeNG */` block (`.ng-invalid.ng-dirty`) does not apply to this markup: the color of an
//! invalid value comes from the base rule `:has(.p-invalid) label`.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_iftalabel::IftaLabel;
//!
//! let html = view! { <IftaLabel><input class="p-inputtext" id="u" /><label for="u">"Name"</label></IftaLabel> }.to_html();
//! assert!(html.contains(r#"class="p-iftalabel""#));
//! ```

use leptos::either::Either;
use leptos::prelude::*;
use prime_rs_core::wrapper::{root_class, root_style};
use prime_rs_core::{OptProp, Pt, all_static};
use std::sync::Arc;

/// Pass-through for IftaLabel.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct IftaLabelPt {
    /// Root element.
    pub root: Pt,
}

/// Label above the value inside the field.
///
/// Properties accept a value or a signal.
#[component]
pub fn IftaLabel(
    /// Inline style.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Extra class.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Pass-through.
    #[prop(optional)]
    pt: Option<IftaLabelPt>,
    /// Content.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let root = pt.unwrap_or_default().root;
    let content = children.map(|c| c());
    let attrs = root.attrs();

    if all_static!(style, style_class) {
        let (class, style) = (root_class("p-iftalabel", &style_class, &root), root_style(&style, &root));
        Either::Left(iftalabel_markup!(class, style, attrs, content))
    } else {
        let root = Arc::new(root);
        let class = {
            let root = root.clone();
            move || root_class("p-iftalabel", &style_class, &root)
        };
        let style = move || root_style(&style, &root);
        Either::Right(iftalabel_markup!(class, style, attrs, content))
    }
}

/// Markup: one for the static and reactive branches (only the root attribute expressions differ).
macro_rules! iftalabel_markup {
    ($class:expr, $style:expr, $attrs:expr, $content:expr) => {
        view! { <span class={$class} style={$style} {..$attrs}>{$content}</span> }
    };
}
use iftalabel_markup;
