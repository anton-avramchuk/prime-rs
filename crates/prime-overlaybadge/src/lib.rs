//! OverlayBadge — port of `p-overlaybadge` from PrimeNG 21.1.10.
//!
//! Root `div.p-overlaybadge` (instead of the host `<p-overlaybadge>`) with child content and a nested [`Badge`].
//! `style_class` and `style` apply to the badge, as in PrimeNG; the corner position of the badge is set by the theme CSS.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_overlaybadge::{OverlayBadge, BadgeSeverity};
//!
//! let html = view! { <OverlayBadge value="2" severity=BadgeSeverity::Danger><span>"x"</span></OverlayBadge> }.to_html();
//! assert!(html.contains("p-overlaybadge") && html.contains("p-badge-danger"));
//! ```

use leptos::prelude::*;
use prime_badge::{Badge, BadgePt};
use prime_rs_core::{OptProp, Prop, Pt};

pub use prime_badge::{BadgeSeverity, BadgeSize};

/// Pass-through for the parts of OverlayBadge.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct OverlayBadgePt {
    /// Root element.
    pub root: Pt,
    /// Nested badge.
    pub pc_badge: BadgePt,
}

/// Badge overlaid on the top-right corner of the content.
///
/// Each property accepts a value or a signal; `style` is read once at creation.
#[component]
pub fn OverlayBadge(
    /// Badge value; without a value the badge is a dot.
    #[prop(optional, into)]
    value: OptProp<String>,
    /// Badge severity.
    #[prop(optional, into)]
    severity: OptProp<BadgeSeverity>,
    /// Badge size.
    #[prop(optional, into)]
    badge_size: OptProp<BadgeSize>,
    /// Hide the badge, leaving the content.
    #[prop(optional, into)]
    badge_disabled: Prop<bool>,
    /// Extra class of the badge.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Inline style of the badge.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Pass-through.
    #[prop(optional)]
    pt: Option<OverlayBadgePt>,
    /// Content the badge is overlaid on.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let OverlayBadgePt { root, mut pc_badge } = pt.unwrap_or_default();
    if let Some(st) = style.get_untracked() {
        let merged = pc_badge.root.merge_style(&st);
        pc_badge.root = pc_badge.root.style(merged);
    }
    let attrs = root.attrs();
    let class = root.merge_class("p-overlaybadge");
    let style = root.has_style().then(|| root.merge_style(""));
    let kids = children.map(|c| c());
    view! {
        <div class=class style={style} {..attrs}>
            {kids}
            <Badge value=value severity=severity size=badge_size badge_disabled=badge_disabled style_class=style_class pt=pc_badge />
        </div>
    }
}
