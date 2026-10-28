//! Структура DOM, классы, `data-p*`, группа, `pt` и паритет ветвей RadioButton (SSR).
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_core::{Pt, Size, Variant};
use prime_radiobutton::{RadioButton, RadioButtonPt};

fn strip(s: &str) -> String {
    s.split("<!--").next().unwrap_or(s).to_owned()
}

#[test]
fn structure_is_input_then_box_with_icon() {
    let html = view! { <RadioButton /> }.to_html();
    assert!(html.starts_with("<div"), "{html}");
    assert!(html.contains("p-radiobutton p-component") && html.contains(r#"type="radio""#), "{html}");
    let input = html.find("p-radiobutton-input").expect("input");
    let boxed = html.find("p-radiobutton-box").expect("box");
    let icon = html.find("p-radiobutton-icon").expect("icon");
    assert!(input < boxed && boxed < icon, "{html}");
    for unwanted in ["p-radiobutton-checked", " p-disabled", "required", "name=", "tabindex", " value="] {
        assert!(!html.contains(unwanted), "{unwanted}: {html}");
    }
    assert!(html.contains(r#"data-p-checked="false""#) && html.contains(r#"aria-checked="false""#), "{html}");
}

#[test]
fn group_checks_the_matching_value() {
    let model = RwSignal::new(Some("b".to_owned()));
    let on = view! { <RadioButton value="b" model=model name="g" /> }.to_html();
    let off = view! { <RadioButton value="a" model=model name="g" /> }.to_html();
    for want in ["p-radiobutton-checked", r#"aria-checked="true""#, r#"data-p-checked="true""#, r#"data-p="checked""#, r#"value="b""#] {
        assert!(on.contains(want), "{want}: {on}");
    }
    assert!(!off.contains("p-radiobutton-checked") && off.contains(r#"aria-checked="false""#), "{off}");
    let none = view! { <RadioButton value="a" model=RwSignal::new(None) /> }.to_html();
    assert!(!none.contains("p-radiobutton-checked"), "{none}");
}

#[test]
fn modifiers_and_data_p() {
    let html = view! { <RadioButton disabled=true invalid=true size=Size::Large variant=Variant::Filled /> }.to_html();
    for want in [" p-disabled", "p-invalid", "p-variant-filled", "p-radiobutton-lg p-inputfield-lg", r#"data-p-disabled="true""#] {
        assert!(html.contains(want), "{want}: {html}");
    }
    assert!(html.contains(r#"data-p="invalid disabled filled large""#), "{html}");
}

#[test]
fn variant_comes_from_config() {
    Owner::new().with(|| {
        prime_core::provide_prime_config(prime_core::PrimeConfig { input_variant: Some(Variant::Filled), ..Default::default() });
        let html = view! { <RadioButton /> }.to_html();
        assert!(html.contains("p-variant-filled"), "{html}");
    });
}

#[test]
fn input_attributes_are_passed() {
    let html =
        view! { <RadioButton input_id="i" name="n" value="v" tabindex=2 aria_label="L" aria_labelledby="B" required=true /> }.to_html();
    for want in [r#"id="i""#, r#"name="n""#, r#"value="v""#, r#"tabindex="2""#, r#"aria-label="L""#, r#"aria-labelledby="B""#, "required"] {
        assert!(html.contains(want), "{want}: {html}");
    }
}

#[test]
fn pt_parts_are_added() {
    let pt =
        RadioButtonPt { root: Pt::new().class("r"), input: Pt::new().class("i"), box_: Pt::new().class("b"), icon: Pt::new().class("c") };
    let html = view! { <RadioButton style_class="mine" pt=pt /> }.to_html();
    for want in ["p-radiobutton p-component mine r", "p-radiobutton-input i", "p-radiobutton-box b", "p-radiobutton-icon c"] {
        assert!(html.contains(want), "{want}: {html}");
    }
}

#[test]
fn static_and_reactive_branches_match() {
    let stat = view! { <RadioButton default_checked=true name="n" value="v" size=Size::Small invalid=true style_class="x" /> }.to_html();
    let reactive = view! {
        <RadioButton
            model=RwSignal::new(Some("v".to_owned()))
            name=RwSignal::new(Some("n".to_owned()))
            value=RwSignal::new(Some("v".to_owned()))
            size=RwSignal::new(Some(Size::Small))
            invalid=RwSignal::new(true)
            style_class=RwSignal::new(Some("x".to_owned()))
        />
    }
    .to_html();
    assert_eq!(strip(&stat), strip(&reactive));
}
