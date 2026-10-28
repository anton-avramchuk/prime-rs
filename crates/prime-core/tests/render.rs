//! Рендер корня с pass-through (SSR).
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_core::Pt;

#[test]
fn root_keeps_own_classes_and_adds_pt() {
    let pt = Pt::new().class("custom").attr("data-testid", "btn");
    let html = view! { <button class=pt.merge_class("p-button p-component") {..pt.attrs()}></button> }.to_html();
    assert!(html.contains("p-button p-component custom"), "{html}");
    assert!(html.contains(r#"data-testid="btn""#), "{html}");
}
