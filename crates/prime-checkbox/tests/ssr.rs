//! Структура DOM, классы, `data-p*`, режимы значения, `pt` и паритет ветвей Checkbox (SSR).
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_checkbox::{Checkbox, CheckboxPt};
use prime_core::{Pt, Size, Variant};

fn strip(s: &str) -> String {
    s.split("<!--").next().unwrap_or(s).to_owned()
}

#[test]
fn structure_is_input_then_box() {
    let html = view! { <Checkbox /> }.to_html();
    assert!(html.starts_with("<div"), "{html}");
    assert!(html.contains("p-checkbox p-component") && html.contains(r#"type="checkbox""#), "{html}");
    let input = html.find("p-checkbox-input").expect("input");
    let boxed = html.find("p-checkbox-box").expect("box");
    assert!(input < boxed, "{html}");
    for unwanted in ["p-checkbox-checked", "p-checkbox-icon", "readonly", "required", "name=", "tabindex"] {
        assert!(!html.contains(unwanted), "{unwanted}: {html}");
    }
    assert!(html.contains(r#"data-p-checked="false""#) && html.contains(r#"data-p-disabled="false""#), "{html}");
}

#[test]
fn binary_checked_shows_check_icon() {
    let checked = RwSignal::new(true);
    let html = view! { <Checkbox binary=true checked=checked /> }.to_html();
    for want in [
        "p-checkbox-checked p-highlight",
        "p-checkbox-icon",
        r#"data-p-checked="true""#,
        r#"data-p-highlight="true""#,
        r#"data-p="checked""#,
    ] {
        assert!(html.contains(want), "{want}: {html}");
    }
    assert!(html.contains("checked"), "{html}");
}

#[test]
fn indeterminate_is_not_checked_and_shows_minus() {
    let html = view! { <Checkbox binary=true checked=RwSignal::new(true) indeterminate=RwSignal::new(true) /> }.to_html();
    assert!(!html.contains("p-checkbox-checked") && html.contains("p-checkbox-icon"), "{html}");
    assert!(html.contains(r#"data-p-checked="false""#), "{html}");
}

#[test]
fn set_mode_checks_when_value_in_model() {
    let model = RwSignal::new(vec!["a".to_owned(), "c".to_owned()]);
    let on = view! { <Checkbox value="a" model=model /> }.to_html();
    let off = view! { <Checkbox value="b" model=model /> }.to_html();
    assert!(on.contains("p-checkbox-checked") && on.contains(r#"value="a""#), "{on}");
    assert!(!off.contains("p-checkbox-checked"), "{off}");
}

#[test]
fn modifiers_and_data_p() {
    let html = view! { <Checkbox disabled=true invalid=true size=Size::Small variant=Variant::Filled /> }.to_html();
    for want in ["p-disabled", "p-invalid", "p-variant-filled", "p-checkbox-sm p-inputfield-sm"] {
        assert!(html.contains(want), "{want}: {html}");
    }
    assert!(html.contains(r#"data-p="invalid disabled filled small""#), "{html}");
}

#[test]
fn variant_comes_from_config() {
    Owner::new().with(|| {
        prime_core::provide_prime_config(prime_core::PrimeConfig { input_variant: Some(Variant::Filled), ..Default::default() });
        let html = view! { <Checkbox /> }.to_html();
        assert!(html.contains("p-variant-filled"), "{html}");
    });
}

#[test]
fn input_attributes_are_passed() {
    let html = view! {
        <Checkbox input_id="i" name="n" tabindex=3 aria_label="L" aria_labelledby="B" required=true readonly=true input_class="ic" />
    }
    .to_html();
    for want in [r#"id="i""#, r#"name="n""#, r#"tabindex="3""#, r#"aria-label="L""#, r#"aria-labelledby="B""#, "required", "readonly", "ic"]
    {
        assert!(html.contains(want), "{want}: {html}");
    }
}

#[test]
fn icon_template_replaces_the_icon() {
    let icon = ViewFn::from(|| view! { <i data-testid="mine"></i> });
    let html = view! { <Checkbox binary=true default_checked=true icon_template=icon /> }.to_html();
    assert!(html.contains("data-testid=\"mine\"") && !html.contains("<svg"), "{html}");
}

#[test]
fn pt_parts_are_added() {
    let pt = CheckboxPt { root: Pt::new().class("r"), input: Pt::new().class("i"), box_: Pt::new().class("b"), icon: Pt::new().class("c") };
    let html = view! { <Checkbox binary=true default_checked=true style_class="mine" pt=pt /> }.to_html();
    for want in ["r", "mine", "i", "b", "c", "p-checkbox-input", "p-checkbox-box", "p-checkbox-icon"] {
        assert!(html.contains(want), "{want}: {html}");
    }
    assert!(html.contains("p-checkbox-box b") && html.contains("p-checkbox-icon c"), "{html}");
}

#[test]
fn static_and_reactive_branches_match() {
    let stat = view! { <Checkbox binary=true default_checked=true name="n" size=Size::Large invalid=true style_class="x" /> }.to_html();
    let reactive = view! {
        <Checkbox
            binary=true
            checked=RwSignal::new(true)
            name=RwSignal::new(Some("n".to_owned()))
            size=RwSignal::new(Some(Size::Large))
            invalid=RwSignal::new(true)
            style_class=RwSignal::new(Some("x".to_owned()))
        />
    }
    .to_html();
    assert_eq!(strip(&stat), strip(&reactive));
}
