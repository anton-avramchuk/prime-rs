//! Структура DOM, свойства бейджа и `pt` OverlayBadge (SSR).
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_badge::BadgePt;
use prime_core::Pt;
use prime_overlaybadge::{BadgeSeverity, BadgeSize, OverlayBadge, OverlayBadgePt};

fn strip(s: &str) -> String {
    let mut out = s.replace("<!>", "");
    while let Some(start) = out.find("<!--") {
        let end = out[start..].find("-->").map_or(out.len(), |e| start + e + 3);
        out.replace_range(start..end, "");
    }
    out
}

#[test]
fn children_come_before_badge() {
    let html = strip(&view! { <OverlayBadge value="2"><i>"x"</i></OverlayBadge> }.to_html());
    assert!(html.starts_with("<div class=\"p-overlaybadge\""), "{html}");
    assert!(html.find("<i>").unwrap() < html.find("p-badge").unwrap(), "{html}");
}

#[test]
fn badge_gets_severity_size_and_class_but_root_does_not() {
    let html = strip(
        &view! { <OverlayBadge value="2" severity=BadgeSeverity::Danger badge_size=BadgeSize::Large style_class="mine" /> }.to_html(),
    );
    assert!(html.contains("p-badge-danger") && html.contains("p-badge-lg") && html.contains("mine"), "{html}");
    assert!(html.starts_with("<div class=\"p-overlaybadge\""), "{html}");
}

#[test]
fn no_value_is_a_dot() {
    let html = strip(&view! { <OverlayBadge /> }.to_html());
    assert!(html.contains("p-badge-dot"), "{html}");
}

#[test]
fn disabled_badge_is_hidden() {
    let html = strip(&view! { <OverlayBadge value="1" badge_disabled=true><i>"x"</i></OverlayBadge> }.to_html());
    assert!(html.contains("display: none"), "{html}");
    assert!(html.contains("<i>x</i>"), "{html}");
}

#[test]
fn style_goes_to_badge() {
    let html = strip(&view! { <OverlayBadge value="1" style="color: red" /> }.to_html());
    assert!(html.contains("color: red"), "{html}");
    assert!(html.starts_with("<div class=\"p-overlaybadge\">"), "{html}");
}

#[test]
fn pt_root_and_badge() {
    let pt = OverlayBadgePt { root: Pt::new().attr("data-x", "1"), pc_badge: BadgePt { root: Pt::new().class("custom") } };
    let html = strip(&view! { <OverlayBadge value="1" pt=pt /> }.to_html());
    assert!(html.contains("data-x=\"1\"") && html.contains("class=\"p-overlaybadge\""), "{html}");
    assert!(html.contains("custom"), "{html}");
}
