//! Разметка, значение, атрибуты, шаблон и `pt` InputOtp (SSR).
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_core::{Pt, Size};
use prime_inputotp::{InputOtp, InputOtpPt, InputOtpSlot};

fn count(html: &str, needle: &str) -> usize {
    html.matches(needle).count()
}

#[test]
fn default_has_four_cells_with_classes() {
    let html = view! { <InputOtp /> }.to_html();
    assert!(html.starts_with("<div"), "{html}");
    assert!(html.contains(r#"class="p-inputotp p-component""#), "{html}");
    assert_eq!(count(&html, "<input"), 4, "{html}");
    assert_eq!(count(&html, r#"class="p-inputtext p-component p-inputotp-input""#), 4, "{html}");
    assert_eq!(count(&html, r#"type="text""#), 4, "{html}");
    assert_eq!(count(&html, r#"inputmode="text""#), 4, "{html}");
    for unwanted in ["disabled", "readonly", "required", "p-invalid", "p-filled", "password"] {
        assert!(!html.contains(unwanted), "{unwanted}: {html}");
    }
}

#[test]
fn length_sets_cell_count_and_maxlength() {
    let html = view! { <InputOtp length=6 /> }.to_html();
    assert_eq!(count(&html, "p-inputotp-input"), 6, "{html}");
    assert_eq!(count(&html, r#"maxlength="6""#), 1, "{html}");
    assert_eq!(count(&html, r#"maxlength="1""#), 5, "{html}");
    assert!(html.find(r#"maxlength="6""#) < html.find(r#"maxlength="1""#), "первая ячейка принимает вставку: {html}");
}

#[test]
fn value_fills_cells_in_order() {
    let html = view! { <InputOtp value=RwSignal::new("12".to_owned()) /> }.to_html();
    assert_eq!(count(&html, r#"value="1""#), 1, "{html}");
    assert_eq!(count(&html, r#"value="2""#), 1, "{html}");
    assert_eq!(count(&html, r#"value="""#), 2, "{html}");
    assert!(html.find(r#"value="1""#) < html.find(r#"value="2""#), "{html}");
}

#[test]
fn extra_value_characters_are_not_shown() {
    let html = view! { <InputOtp value=RwSignal::new("123456".to_owned()) /> }.to_html();
    assert_eq!(count(&html, "<input"), 4, "{html}");
    for shown in ["1", "2", "3", "4"] {
        assert_eq!(count(&html, &format!(r#"value="{shown}""#)), 1, "{shown}: {html}");
    }
    for hidden in ["5", "6"] {
        assert!(!html.contains(&format!(r#"value="{hidden}""#)), "{hidden}: {html}");
    }
}

#[test]
fn mask_and_integer_only() {
    let html = view! { <InputOtp mask=true /> }.to_html();
    assert_eq!(count(&html, r#"type="password""#), 4, "{html}");
    assert!(!html.contains(r#"type="text""#), "{html}");
    let html = view! { <InputOtp integer_only=true /> }.to_html();
    assert_eq!(count(&html, r#"inputmode="numeric""#), 4, "{html}");
}

#[test]
fn size_invalid_variant_classes() {
    let html = view! { <InputOtp size=Size::Small invalid=true /> }.to_html();
    assert_eq!(count(&html, "p-inputtext-sm"), 4, "{html}");
    assert_eq!(count(&html, "p-invalid"), 4, "{html}");
    let html = view! { <InputOtp size=Size::Large variant=prime_core::Variant::Filled /> }.to_html();
    assert_eq!(count(&html, "p-inputtext-lg"), 4, "{html}");
    assert_eq!(count(&html, "p-variant-filled"), 4, "{html}");
}

#[test]
fn disabled_readonly_required_name_tabindex() {
    let html = view! { <InputOtp disabled=true readonly=true required=true name="otp" tabindex=3 /> }.to_html();
    assert_eq!(count(&html, "disabled"), 4, "{html}");
    assert_eq!(count(&html, "readonly"), 4, "{html}");
    assert_eq!(count(&html, "required"), 4, "{html}");
    assert_eq!(count(&html, r#"name="otp""#), 4, "{html}");
    assert_eq!(count(&html, r#"tabindex="3""#), 4, "{html}");
}

#[test]
fn autofocus_only_on_first_cell() {
    let html = view! { <InputOtp autofocus=true /> }.to_html();
    assert_eq!(count(&html, "autofocus"), 1, "{html}");
    let second = html.match_indices("<input").nth(1).unwrap().0;
    assert!(html.find("autofocus").unwrap() < second, "{html}");
}

#[test]
fn style_class_style_and_pt() {
    let pt =
        InputOtpPt { root: Pt::new().class("root-x").attr("data-testid", "otp"), input: Pt::new().class("custom").attr("data-cell", "1") };
    let html = view! { <InputOtp style_class="mine" style="margin: 4px" pt=pt /> }.to_html();
    assert!(html.contains(r#"class="p-inputotp p-component mine root-x""#), "{html}");
    assert!(html.contains(r#"style="margin: 4px;""#), "{html}");
    assert!(html.contains(r#"data-testid="otp""#), "{html}");
    assert_eq!(count(&html, "custom"), 4, "{html}");
    assert_eq!(count(&html, r#"data-cell="1""#), 4, "{html}");
}

#[test]
fn input_template_replaces_cells() {
    let template = Callback::new(|slot: InputOtpSlot| {
        let InputOtpSlot { token, index, events } = slot;
        view! { <input class="custom-otp-input" data-index=index value=token on:input=move |e| events.on_input.run(e) /> }.into_any()
    });
    let html = view! { <InputOtp length=5 value=RwSignal::new("97".to_owned()) input=template /> }.to_html();
    assert_eq!(count(&html, "custom-otp-input"), 5, "{html}");
    assert!(!html.contains("p-inputotp-input"), "{html}");
    assert_eq!(count(&html, r#"value="9""#), 1, "{html}");
    assert_eq!(count(&html, r#"data-index="4""#), 1, "{html}");
}
