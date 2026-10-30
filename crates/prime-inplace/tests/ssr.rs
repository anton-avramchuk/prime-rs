//! Разметка Inplace (SSR): вид, содержимое, `disabled`, кнопка закрытия, `pt`.
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_core::Pt;
use prime_inplace::{Inplace, InplacePt};

fn display() -> ViewFn {
    ViewFn::from(|| view! { <span>"View"</span> })
}

fn content() -> Callback<Callback<()>, AnyView> {
    Callback::new(|_close| view! { <p>"Body"</p> }.into_any())
}

#[test]
fn display_by_default() {
    let html = view! { <Inplace display=display() content=content() /> }.to_html();
    assert!(html.contains("p-inplace p-component") && html.contains(r#"aria-live="polite""#), "{html}");
    assert!(html.contains("p-inplace-display") && html.contains(r#"role="button""#) && html.contains(r#"tabindex="0""#), "{html}");
    assert!(html.contains("View") && !html.contains("Body") && !html.contains("p-inplace-content"), "{html}");
    assert!(!html.contains("p-disabled") && !html.contains("data-p-disabled"), "{html}");
}

#[test]
fn active_shows_content_only() {
    let active = RwSignal::new(true);
    let html = view! { <Inplace active=active display=display() content=content() /> }.to_html();
    assert!(html.contains("p-inplace-content") && html.contains("Body"), "{html}");
    assert!(!html.contains("p-inplace-display") && !html.contains("View"), "{html}");
    assert!(!html.contains("<button"), "кнопка закрытия только при closable: {html}");
}

#[test]
fn disabled_display() {
    let html = view! { <Inplace disabled=true display=display() content=content() /> }.to_html();
    assert!(html.contains("p-inplace-display p-disabled") && html.contains(r#"data-p-disabled="true""#), "{html}");
}

#[test]
fn closable_renders_button_with_times_icon() {
    let active = RwSignal::new(true);
    let html = view! { <Inplace active=active closable=true close_aria_label="Close" display=display() content=content() /> }.to_html();
    assert!(html.contains("<button") && html.contains(r#"type="button""#) && html.contains(r#"aria-label="Close""#), "{html}");
    assert!(html.contains("<svg"), "{html}");
}

#[test]
fn custom_close_icon() {
    let active = RwSignal::new(true);
    let icon = ViewFn::from(|| view! { <b>"x"</b> });
    let html = view! { <Inplace active=active closable=true close_icon=icon display=display() content=content() /> }.to_html();
    assert!(html.contains("<b>x</b>") && !html.contains("<svg"), "{html}");
}

#[test]
fn style_class_and_pt() {
    let pt =
        InplacePt { root: Pt::new().class("from-pt").attr("data-testid", "root"), display: Pt::new().class("disp"), ..Default::default() };
    let html = view! { <Inplace style="width: 10rem" style_class="mine" pt=pt display=display() content=content() /> }.to_html();
    assert!(html.contains("mine") && html.contains("from-pt") && html.contains("width: 10rem"), "{html}");
    assert!(html.contains(r#"data-testid="root""#) && html.contains("p-inplace-display disp"), "{html}");
}
