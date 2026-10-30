//! Разметка Password (SSR): корень, поле, значки, `pt`, панель закрыта.
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_core::{Pt, Variant};
use prime_password::{Password, PasswordPt};

#[test]
fn default_markup() {
    let html = view! { <Password /> }.to_html();
    assert!(html.contains("p-password p-component p-inputwrapper"), "{html}");
    assert!(html.contains("p-password-input") && html.contains("p-inputtext"), "{html}");
    assert!(html.contains(r#"type="password""#), "{html}");
    assert!(html.contains("position: relative"), "{html}");
    assert!(!html.contains("p-password-overlay"), "{html}");
    assert!(!html.contains("p-password-toggle-mask-icon") && !html.contains("p-password-clear-icon"), "{html}");
    assert!(!html.contains("p-inputwrapper-filled") && !html.contains("p-inputwrapper-focus"), "{html}");
}

#[test]
fn filled_value() {
    let value = RwSignal::new("abc".to_owned());
    let html = view! { <Password value=value /> }.to_html();
    assert!(html.contains("p-inputwrapper-filled"), "{html}");
    assert!(html.contains("abc"), "{html}");
}

#[test]
fn toggle_mask_renders_unmask_icon() {
    let html = view! { <Password toggle_mask=true /> }.to_html();
    assert!(html.contains("p-password-toggle-mask-icon") && html.contains("p-password-unmask-icon"), "{html}");
    assert!(!html.contains("p-password-mask-icon p-password") || html.contains("p-password-unmask-icon"), "{html}");
    assert!(html.contains("<svg"), "{html}");
}

#[test]
fn clear_icon_needs_value() {
    let empty = view! { <Password show_clear=true /> }.to_html();
    assert!(!empty.contains("p-password-clear-icon"), "{empty}");
    let value = RwSignal::new("abc".to_owned());
    let filled = view! { <Password show_clear=true value=value /> }.to_html();
    assert!(filled.contains("p-password-clear-icon"), "{filled}");
}

#[test]
fn custom_icons_replace_defaults() {
    let html = view! { <Password toggle_mask=true show_icon=ViewFn::from(|| view! { <b>"show"</b> }) /> }.to_html();
    assert!(html.contains("<b>show</b>") && html.contains("p-password-unmask-icon"), "{html}");
    assert!(!html.contains("<svg"), "{html}");
}

#[test]
fn variant_fluid_and_data_p() {
    let html = view! { <Password variant=Variant::Filled fluid=true /> }.to_html();
    assert!(html.contains("p-variant-filled") && html.contains("p-password-fluid"), "{html}");
    assert!(html.contains(r#"data-p="fluid""#), "{html}");
}

#[test]
fn input_props_reach_input() {
    let html = view! {
        <Password placeholder="Пароль" autocomplete="off" input_id="pw" aria_label="Пароль" max_length=20 disabled=true invalid=true input_style_class="extra" />
    }
    .to_html();
    for needle in [
        "placeholder=\"Пароль\"",
        "autocomplete=\"off\"",
        "id=\"pw\"",
        "aria-label=\"Пароль\"",
        "maxlength=\"20\"",
        "disabled",
        "p-invalid",
        "extra",
    ] {
        assert!(html.contains(needle), "{needle}: {html}");
    }
}

#[test]
fn root_style_class_and_pt() {
    let pt = PasswordPt { root: Pt::new().class("from-pt").attr("data-testid", "root"), ..Default::default() };
    let html = view! { <Password style="width: 10rem" style_class="mine" pt=pt /> }.to_html();
    assert!(html.contains("mine") && html.contains("from-pt"), "{html}");
    assert!(html.contains(r#"data-testid="root""#) && html.contains("width: 10rem"), "{html}");
    assert!(html.contains("position: relative"), "{html}");
}
