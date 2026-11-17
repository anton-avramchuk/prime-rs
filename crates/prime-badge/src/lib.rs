//! Badge — port of `p-badge` from PrimeNG 21.1.10.
//!
//! Markup and classes match PrimeNG (`p-badge p-component` + modifiers, `data-p`), but instead of the
//! `<p-badge>` host element a `<span>` is rendered: the theme CSS is bound to classes, not to the tag name.
//!
//! Properties are [`Prop`]/[`OptProp`]: a value creates no reactive primitives, a signal subscribes the attributes.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_badge::{Badge, BadgeSeverity};
//!
//! let html = view! { <Badge value="5" severity=BadgeSeverity::Danger /> }.to_html();
//! assert!(html.contains("p-badge-circle") && html.contains("p-badge-danger"));
//! ```

mod state;

use leptos::either::Either;
use leptos::prelude::*;
use prime_core::{ClassNames, OptProp, Prop, Pt, all_static};
use std::borrow::Cow;
use std::sync::Arc;

pub use state::{BadgeSeverity, BadgeSize, BadgeState, classes, data_p, write_classes};

/// Pass-through of Badge parts.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct BadgePt {
    /// Root element.
    pub root: Pt,
}

/// Badge: value, severity, size; without a value a dot, with a single character a circle.
///
/// Each property accepts a value or a signal; a value creates no reactive primitives.
#[component]
pub fn Badge(
    /// Value (string). Empty is a dot (`p-badge-dot`), one character is a circle (`p-badge-circle`).
    #[prop(optional, into)]
    value: OptProp<String>,
    /// Severity.
    #[prop(optional, into)]
    severity: OptProp<BadgeSeverity>,
    /// Size.
    #[prop(optional, into)]
    size: OptProp<BadgeSize>,
    /// Hide the badge (`display: none`), keeping it in the DOM.
    #[prop(optional, into)]
    badge_disabled: Prop<bool>,
    /// Extra class.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Pass-through.
    #[prop(optional)]
    pt: Option<BadgePt>,
) -> impl IntoView {
    let pt = Arc::new(pt.unwrap_or_default().root);
    let is_static = all_static!(value, severity, size, badge_disabled, style_class);
    // All properties in one `Arc`: attribute closures clone the reference counter, not the value strings.
    let props = Props { value, severity, size, disabled: badge_disabled, style_class };
    let attrs = pt.attrs();

    if is_static {
        // Static path: values are computed once, there are no closures or `RenderEffect`.
        let (class, style, data, text) = (class_of(&props, &pt), style_of(&props, &pt), data_of(&props), props.value.get());
        Either::Left(badge_markup!(class, style, data, text, attrs))
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
        let text = move || props.value.get();
        Either::Right(badge_markup!(class, style, data, text, attrs))
    }
}

/// Badge markup: one for the static and reactive branches (only attribute expressions differ).
macro_rules! badge_markup {
    ($class:expr, $style:expr, $data:expr, $text:expr, $attrs:expr) => {
        view! {
            <span class={$class} style={$style} data-p={$data} {..$attrs}>
                {$text}
            </span>
        }
    };
}
use badge_markup;

fn class_of(props: &Props, pt: &Pt) -> Cow<'static, str> {
    let mut cn = ClassNames::new();
    props.with_state(|s| write_classes(&mut cn, s));
    props.style_class.with(|c| cn.add_user(c.as_deref()));
    cn.add_user(pt.extra_class());
    cn.finish_cow()
}

fn style_of(props: &Props, pt: &Pt) -> Option<String> {
    let hidden = props.disabled.get();
    (hidden || pt.has_style()).then(|| pt.merge_style(if hidden { "display: none" } else { "" }))
}

fn data_of(props: &Props) -> Cow<'static, str> {
    props.with_state(data_p)
}

/// Instance properties.
struct Props {
    value: OptProp<String>,
    severity: OptProp<BadgeSeverity>,
    size: OptProp<BadgeSize>,
    disabled: Prop<bool>,
    style_class: OptProp<String>,
}

impl Props {
    /// Calls `f` with a borrowed state snapshot (without copying the value).
    fn with_state<R>(&self, f: impl FnOnce(&BadgeState) -> R) -> R {
        self.value.with(|v| {
            f(&BadgeState { value: v.as_deref(), severity: self.severity.get(), size: self.size.get(), disabled: self.disabled.get() })
        })
    }
}
