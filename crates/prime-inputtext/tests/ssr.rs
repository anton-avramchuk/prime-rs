//! Структура DOM, атрибуты, классы, `pt` и паритет ветвей InputText (SSR).
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_core::{Pt, Size, Variant};
use prime_inputtext::InputText;

#[test]
fn renders_input_with_default_type() {
    let html = view! { <InputText /> }.to_html();
    assert!(html.starts_with("<input"), "{html}");
    assert!(html.contains(r#"type="text""#) && html.contains("p-inputtext p-component"), "{html}");
    for unwanted in ["disabled", "readonly", "required", "placeholder", "name=", "maxlength", "p-filled"] {
        assert!(!html.contains(unwanted), "{unwanted}: {html}");
    }
}

#[test]
fn attributes_are_passed() {
    let html =
        view! { <InputText placeholder="Name" name="n" id="f" disabled=true maxlength=5 input_type="email" aria_label="L" /> }.to_html();
    for want in
        [r#"placeholder="Name""#, r#"name="n""#, r#"id="f""#, "disabled", r#"maxlength="5""#, r#"type="email""#, r#"aria-label="L""#]
    {
        assert!(html.contains(want), "{want}: {html}");
    }
    assert!(!html.contains("readonly") && !html.contains("required"), "{html}");
}

#[test]
fn size_variant_invalid_fluid() {
    let html = view! { <InputText size=Size::Small variant=Variant::Filled invalid=true fluid=true /> }.to_html();
    for want in ["p-inputtext-sm", "p-variant-filled", "p-invalid", "p-inputtext-fluid"] {
        assert!(html.contains(want), "{want}: {html}");
    }
    assert!(html.contains(r#"data-p="invalid fluid filled small""#), "{html}");
}

#[test]
fn value_is_rendered_and_marks_filled() {
    let value = RwSignal::new("abc".to_owned());
    let html = view! { <InputText value=value /> }.to_html();
    assert!(html.contains("p-filled") && html.contains(r#"value="abc""#), "{html}");
    let empty = view! { <InputText value=RwSignal::new(String::new()) /> }.to_html();
    assert!(!empty.contains("p-filled"), "{empty}");
}

#[test]
fn variant_comes_from_config() {
    Owner::new().with(|| {
        prime_core::provide_prime_config(prime_core::PrimeConfig { input_variant: Some(Variant::Filled), ..Default::default() });
        let html = view! { <InputText /> }.to_html();
        assert!(html.contains("p-variant-filled"), "{html}");
    });
}

#[test]
fn pt_class_is_added() {
    let html = view! { <InputText style_class="mine" pt=Pt::new().class("custom") /> }.to_html();
    assert!(html.contains("custom") && html.contains("mine") && html.contains("p-inputtext"), "{html}");
}

#[test]
fn static_and_reactive_branches_match() {
    let stat = view! { <InputText placeholder="P" size=Size::Large invalid=true style_class="x" /> }.to_html();
    let reactive = view! {
        <InputText
            placeholder=RwSignal::new(Some("P".to_owned()))
            size=RwSignal::new(Some(Size::Large))
            invalid=RwSignal::new(true)
            style_class=RwSignal::new(Some("x".to_owned()))
        />
    }
    .to_html();
    let strip = |s: &str| s.split("<!--").next().unwrap_or(s).to_owned();
    assert_eq!(strip(&stat), strip(&reactive));
}

#[test]
fn fluid_inherited_from_context() {
    let outside = view! { <InputText/> }.to_html();
    assert!(!outside.contains("p-inputtext-fluid"), "{outside}");
    Owner::new().with(|| {
        prime_core::fluid::provide_fluid();
        let inherited = view! { <InputText/> }.to_html();
        assert!(inherited.contains("p-inputtext-fluid"), "{inherited}");
        let forced_off = view! { <InputText fluid=false/> }.to_html();
        assert!(!forced_off.contains("p-inputtext-fluid"), "{forced_off}");
        let live = RwSignal::new(None::<bool>);
        let reactive = view! { <InputText fluid=live/> }.to_html();
        assert!(reactive.contains("p-inputtext-fluid"), "{reactive}");
        live.set(Some(false));
        let reactive_off = view! { <InputText fluid=live/> }.to_html();
        assert!(!reactive_off.contains("p-inputtext-fluid"), "{reactive_off}");
    });
}
