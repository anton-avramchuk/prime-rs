//! Структура DOM, атрибуты, классы, значение и паритет ветвей Textarea (SSR).
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_core::{Pt, Size, Variant};
use prime_textarea::Textarea;

fn strip(s: &str) -> String {
    let mut out = s.replace("<!>", "");
    while let Some(start) = out.find("<!--") {
        let end = out[start..].find("-->").map_or(out.len(), |e| start + e + 3);
        out.replace_range(start..end, "");
    }
    out
}

#[test]
fn renders_textarea_without_optional_attributes() {
    let html = view! { <Textarea /> }.to_html();
    assert!(html.starts_with("<textarea") && html.ends_with("</textarea>"), "{html}");
    assert!(html.contains(r#"class="p-textarea p-component""#), "{html}");
    for unwanted in
        ["disabled", "readonly", "required", "placeholder", "name=", "rows", "cols", "maxlength", "p-filled", "data-p", "style="]
    {
        assert!(!html.contains(unwanted), "{unwanted}: {html}");
    }
}

#[test]
fn attributes_are_passed() {
    let html =
        view! { <Textarea rows=5u32 cols=30u32 placeholder="Hi" name="n" id="f" disabled=true maxlength=9 aria_label="L" /> }.to_html();
    for want in [
        r#"rows="5""#,
        r#"cols="30""#,
        r#"placeholder="Hi""#,
        r#"name="n""#,
        r#"id="f""#,
        "disabled",
        r#"maxlength="9""#,
        r#"aria-label="L""#,
    ] {
        assert!(html.contains(want), "{want}: {html}");
    }
    assert!(!html.contains("readonly") && !html.contains("required"), "{html}");
}

#[test]
fn modifiers_in_primeng_order() {
    let html = view! { <Textarea auto_resize=true variant=Variant::Filled fluid=true size=Size::Small invalid=true /> }.to_html();
    assert!(
        html.contains(
            "p-textarea p-component p-textarea-resizable p-variant-filled p-textarea-fluid p-inputfield-sm p-textarea-sm p-invalid"
        ),
        "{html}"
    );
    let large = view! { <Textarea size=Size::Large /> }.to_html();
    assert!(large.contains("p-textarea-lg p-inputfield-lg"), "{large}");
}

#[test]
fn value_is_child_text_and_marks_filled() {
    let value = RwSignal::new("abc\ndef".to_owned());
    let html = view! { <Textarea value=value /> }.to_html();
    assert!(html.contains("p-filled") && html.contains(">abc\ndef</textarea>"), "{html}");
    let empty = view! { <Textarea value=RwSignal::new(String::new()) /> }.to_html();
    assert!(!empty.contains("p-filled"), "{empty}");
}

#[test]
fn value_text_is_escaped() {
    let html = view! { <Textarea value=RwSignal::new("<b>&</b>".to_owned()) /> }.to_html();
    assert!(html.contains("&lt;b&gt;&amp;&lt;/b&gt;") && !html.contains("<b>"), "{html}");
    let breakout = view! { <Textarea value=RwSignal::new("</textarea><script>x</script>".to_owned()) /> }.to_html();
    assert!(!breakout.contains("<script>") && breakout.matches("</textarea>").count() == 1, "{breakout}");
}

#[test]
fn leading_newline_survives_the_html_parser() {
    let html = view! { <Textarea value=RwSignal::new("\nabc".to_owned()) /> }.to_html();
    assert!(html.contains(">\n\nabc</textarea>"), "{html:?}");
}

#[test]
fn variant_comes_from_config() {
    Owner::new().with(|| {
        prime_core::provide_prime_config(prime_core::PrimeConfig { input_variant: Some(Variant::Filled), ..Default::default() });
        let html = view! { <Textarea /> }.to_html();
        assert!(html.contains("p-variant-filled"), "{html}");
    });
}

#[test]
fn style_class_style_and_pt_are_added() {
    let pt = Pt::new().class("custom").style("margin: 0").attr("data-x", "y");
    let html = view! { <Textarea style_class="mine" style="color: red" pt=pt /> }.to_html();
    assert!(html.contains("p-textarea p-component mine custom"), "{html}");
    assert!(html.contains("color: red") && html.contains("margin: 0") && html.contains(r#"data-x="y""#), "{html}");
}

#[test]
fn static_and_reactive_branches_match() {
    let stat = view! { <Textarea rows=4u32 placeholder="P" size=Size::Large invalid=true style_class="x" /> }.to_html();
    let reactive = view! {
        <Textarea
            rows=RwSignal::new(Some(4u32))
            placeholder=RwSignal::new(Some("P".to_owned()))
            size=RwSignal::new(Some(Size::Large))
            invalid=RwSignal::new(true)
            style_class=RwSignal::new(Some("x".to_owned()))
        />
    }
    .to_html();
    assert_eq!(strip(&stat), strip(&reactive));
}
