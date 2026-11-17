//! ScrollTop — port of PrimeNG 21.1.10 `p-scrolltop`.
//!
//! A "back to top" button (`Button` with the `p-scrolltop` class) that is shown when the window scroll (`target = Window`) or
//! the parent element scroll (`target = Parent`) is strictly greater than `threshold` (400 by default). A click scrolls the target to the start.
//!
//! For the window the button is mounted only while visible. The parent cannot be found before showing, so with `Parent` the button is always in the DOM and
//! in the hidden state is hidden with the `display: none` style.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_scrolltop::{ScrollTop, ScrollTopTarget};
//!
//! // On the server the window button is hidden, the parent's one is in the markup but hidden.
//! assert!(!view! { <ScrollTop /> }.to_html().contains("p-scrolltop"));
//! let html = view! { <ScrollTop target=ScrollTopTarget::Parent /> }.to_html();
//! assert!(html.contains("p-scrolltop-sticky") && html.contains("display: none"));
//! ```

#[cfg(target_arch = "wasm32")]
mod client;
mod state;

use leptos::prelude::*;
use prime_button::{Button, ButtonPt, ButtonSeverity};
use prime_core::{ClassNames, OptProp, Prop, Pt, ZGroup, use_prime_config};
use prime_icons::ChevronUpIcon;

pub use state::{Phase, ScrollTopBehavior, ScrollTopTarget, enter_classes, is_visible, leave_classes, root_classes};

/// Default scroll threshold.
const DEFAULT_THRESHOLD: f64 = 400.0;

/// Pass-through parts of ScrollTop.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ScrollTopPt {
    /// Root button.
    pub root: Pt,
}

/// A button returning to the start of the page or of the scrollable parent.
#[component]
pub fn ScrollTop(
    /// What to scroll: the window (default) or the parent.
    #[prop(optional)]
    target: ScrollTopTarget,
    /// Scroll after which the button is shown (strictly greater; 400 by default).
    #[prop(default = Prop::Static(DEFAULT_THRESHOLD), into)]
    threshold: Prop<f64>,
    /// Scroll behavior on click (smooth by default).
    #[prop(optional)]
    behavior: ScrollTopBehavior,
    /// Custom icon instead of `ChevronUpIcon`; wrapped in `span.p-scrolltop-icon`.
    #[prop(optional, into)]
    icon: Option<ViewFn>,
    /// The `aria-label` attribute; by default from the locale (`aria.scrollTop`).
    #[prop(optional, into)]
    aria_label: OptProp<String>,
    /// Button color.
    #[prop(optional, into)]
    severity: OptProp<ButtonSeverity>,
    /// Button with a shadow.
    #[prop(optional, into)]
    raised: Prop<bool>,
    /// Rounded button (yes by default).
    #[prop(default = Prop::Static(true), into)]
    rounded: Prop<bool>,
    /// Inline style.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Extra class.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Pass-through of parts.
    #[prop(optional)]
    pt: Option<ScrollTopPt>,
) -> impl IntoView {
    let id = prime_core::ids::next_unique();
    let config = use_prime_config();
    let aria_label = match aria_label {
        Prop::Static(None) => Prop::Static(Some(config.locale.text("aria.scrollTop").to_owned())),
        other => other,
    };
    let overlay_base = config.z_index.base(ZGroup::Overlay);

    let phase = RwSignal::new(Phase::Hidden);
    let animation = RwSignal::new("");
    let z_index = RwSignal::new(None::<i32>);

    #[cfg(target_arch = "wasm32")]
    client::setup(client::Setup {
        target,
        threshold: StoredValue::new(threshold),
        phase,
        animation,
        z_index,
        overlay_base,
        epoch: StoredValue::new(0),
        id,
    });
    #[cfg(not(target_arch = "wasm32"))]
    let _ = (threshold, overlay_base);

    let class = Signal::derive(move || {
        let mut cn = ClassNames::new();
        cn.add(root_classes(target));
        let animation = animation.get();
        cn.add_if(!animation.is_empty(), animation);
        style_class.with(|c| cn.add_user(c.as_deref()));
        Some(cn.finish_cow().into_owned())
    });
    let button_style = Signal::derive(move || {
        let mut parts = Vec::new();
        if let Some(z) = z_index.get() {
            parts.push(format!("z-index: {z}"));
        }
        if target == ScrollTopTarget::Parent && phase.get() == Phase::Hidden {
            parts.push("display: none".to_owned());
        }
        style.with(|s| parts.extend(s.clone()));
        (!parts.is_empty()).then(|| parts.join("; "))
    });
    let root = pt.unwrap_or_default().root.attr("data-scrolltop", id.to_string());
    let button_icon = ViewFn::from(move || match &icon {
        Some(custom) => view! { <span class="p-scrolltop-icon">{custom.run()}</span> }.into_any(),
        None => view! { <ChevronUpIcon class="p-scrolltop-icon" /> }.into_any(),
    });
    let click = Callback::new(move |_: web_sys::MouseEvent| scroll_to_top(target, behavior, id));
    let button = move || {
        view! {
            <Button
                button_type="button"
                icon=button_icon.clone()
                rounded=rounded.clone()
                raised=raised.clone()
                severity=severity.clone()
                aria_label=aria_label.clone()
                style=button_style
                style_class=class
                on_click=click
                pt=ButtonPt { root: root.clone(), ..Default::default() }
            />
        }
    };

    match target {
        ScrollTopTarget::Window => (move || (phase.get() != Phase::Hidden).then(&button)).into_any(),
        ScrollTopTarget::Parent => button().into_any(),
    }
}

/// Scrolls the target to the start.
fn scroll_to_top(target: ScrollTopTarget, behavior: ScrollTopBehavior, id: usize) {
    let options = web_sys::ScrollToOptions::new();
    options.set_top(0.0);
    options.set_behavior(match behavior {
        ScrollTopBehavior::Smooth => web_sys::ScrollBehavior::Smooth,
        ScrollTopBehavior::Auto => web_sys::ScrollBehavior::Auto,
    });
    match target {
        ScrollTopTarget::Window => {
            if let Some(window) = web_sys::window() {
                window.scroll_with_scroll_to_options(&options);
            }
        }
        ScrollTopTarget::Parent => {
            if let Some(parent) = parent_of(id) {
                parent.scroll_with_scroll_to_options(&options);
            }
        }
    }
}

/// Parent of the button with number `id`.
pub(crate) fn parent_of(id: usize) -> Option<web_sys::Element> {
    let document = web_sys::window()?.document()?;
    document.query_selector(&format!(r#"[data-scrolltop="{id}"]"#)).ok().flatten()?.parent_element()
}
