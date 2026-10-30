//! Разметка ScrollTop (SSR): окно скрыто, родитель спрятан стилем, классы, подпись, значок, `pt`.
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_core::Pt;
use prime_scrolltop::{ScrollTop, ScrollTopPt, ScrollTopTarget};

#[test]
fn window_target_renders_nothing_on_server() {
    let html = view! { <ScrollTop /> }.to_html();
    assert!(!html.contains("p-scrolltop") && !html.contains("<button"), "{html}");
}

#[test]
fn parent_target_is_in_markup_but_hidden() {
    let html = view! { <ScrollTop target=ScrollTopTarget::Parent /> }.to_html();
    assert!(html.contains("<button"), "{html}");
    assert!(html.contains("p-scrolltop") && html.contains("p-scrolltop-sticky"), "{html}");
    assert!(html.contains("display: none"), "{html}");
    assert!(!html.contains("z-index"), "{html}");
}

#[test]
fn parent_button_defaults() {
    let html = view! { <ScrollTop target=ScrollTopTarget::Parent /> }.to_html();
    assert!(html.contains(r#"type="button""#), "{html}");
    assert!(html.contains("p-button-rounded") || html.contains("p-button-icon-only"), "{html}");
    assert!(html.contains(r#"aria-label="Scroll Top""#), "{html}");
    assert!(html.contains("p-scrolltop-icon") && html.contains("<svg"), "{html}");
    assert!(!html.contains("p-button-raised"), "{html}");
    assert!(html.contains("data-scrolltop="), "{html}");
}

#[test]
fn aria_label_and_custom_icon() {
    let html = view! {
        <ScrollTop target=ScrollTopTarget::Parent aria_label="Наверх" icon=ViewFn::from(|| view! { <b class="mine">"^"</b> }) />
    }
    .to_html();
    assert!(html.contains(r#"aria-label="Наверх""#) && !html.contains("Scroll Top"), "{html}");
    assert!(html.contains("p-scrolltop-icon"), "{html}");
    assert!(html.contains(r#"class="mine""#) && !html.contains("<svg"), "{html}");
}

#[test]
fn style_class_style_and_pt() {
    let pt = ScrollTopPt { root: Pt::new().class("from-pt").attr("data-testid", "top") };
    let html = view! { <ScrollTop target=ScrollTopTarget::Parent style_class="mine" style="margin: 4px" pt=pt /> }.to_html();
    assert!(html.contains("mine") && html.contains("from-pt"), "{html}");
    assert!(html.contains("margin: 4px") && html.contains("display: none"), "{html}");
    assert!(html.contains(r#"data-testid="top""#), "{html}");
}

#[test]
fn severity_raised_and_rounded_off() {
    let html =
        view! { <ScrollTop target=ScrollTopTarget::Parent severity=prime_button::ButtonSeverity::Contrast raised=true rounded=false /> }
            .to_html();
    assert!(html.contains("p-button-contrast") && html.contains("p-button-raised"), "{html}");
    assert!(!html.contains("p-button-rounded"), "{html}");
}
