//! Структура DOM, положение ручек, `aria-*`, классы, `data-p*`, `disabled` и `pt` Slider (SSR).
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_core::Pt;
use prime_slider::{Slider, SliderOrientation, SliderPt};

fn count(html: &str, needle: &str) -> usize {
    html.matches(needle).count()
}

#[test]
fn single_structure_and_defaults() {
    let html = view! { <Slider default_value=30.0 /> }.to_html();
    assert!(html.starts_with("<div"), "{html}");
    for expected in [
        r#"class="p-slider p-component p-slider-horizontal""#,
        r#"data-pc-name="slider""#,
        r#"data-pc-section="root""#,
        r#"data-p="horizontal""#,
        r#"data-p-sliding="false""#,
        r#"class="p-slider-range""#,
        r#"data-pc-section="range""#,
        r#"class="p-slider-handle""#,
        r#"data-pc-section="handle""#,
        r#"role="slider""#,
        r#"tabindex="0""#,
        r#"aria-valuemin="0""#,
        r#"aria-valuenow="30""#,
        r#"aria-valuemax="100""#,
        r#"aria-orientation="horizontal""#,
    ] {
        assert!(html.contains(expected), "{expected}: {html}");
    }
    assert_eq!(count(&html, r#"role="slider""#), 1, "{html}");
    let (range, handle) = (html.find("p-slider-range").unwrap(), html.find("p-slider-handle").unwrap());
    assert!(range < handle, "{html}");
    for unwanted in ["p-disabled", "p-invalid", "p-slider-animate", "aria-label", "p-slider-handle-active"] {
        assert!(!html.contains(unwanted), "{unwanted}: {html}");
    }
}

#[test]
fn position_and_fill_follow_the_value() {
    let html = view! { <Slider value=RwSignal::new(50.0) max=200.0 /> }.to_html();
    assert!(html.contains("inset-inline-start: 25%"), "ручка: {html}");
    assert!(html.contains("width: 25%"), "заполнение: {html}");
    assert_eq!(count(&html, "position: absolute"), 2, "{html}");
    let html = view! { <Slider default_value=-10.0 /> }.to_html();
    assert!(html.contains("inset-inline-start: 0%") && html.contains("width: 0%"), "{html}");
    let html = view! { <Slider default_value=500.0 min=10.0 max=20.0 /> }.to_html();
    assert!(
        html.contains("inset-inline-start: 100%") && html.contains(r#"aria-valuemin="10""#) && html.contains(r#"aria-valuemax="20""#),
        "{html}"
    );
}

#[test]
fn vertical_uses_bottom_and_height() {
    let html = view! { <Slider default_value=40.0 orientation=SliderOrientation::Vertical /> }.to_html();
    assert!(html.contains("bottom: 40%") && html.contains("height: 40%"), "{html}");
    assert!(!html.contains("inset-inline-start") && !html.contains("width:"), "{html}");
    for expected in ["p-slider-vertical", r#"data-p="vertical""#, r#"aria-orientation="vertical""#] {
        assert!(html.contains(expected), "{expected}: {html}");
    }
    assert!(!html.contains("p-slider-horizontal"), "{html}");
}

#[test]
fn range_has_two_handles_in_any_order() {
    let html = view! { <Slider range=true values=RwSignal::new([20.0, 80.0]) /> }.to_html();
    assert_eq!(count(&html, r#"role="slider""#), 2, "{html}");
    assert!(html.contains(r#"data-pc-section="startHandler""#) && html.contains(r#"data-pc-section="endHandler""#), "{html}");
    assert!(html.contains(r#"aria-valuenow="20""#) && html.contains(r#"aria-valuenow="80""#), "{html}");
    assert!(html.contains("inset-inline-start: 20%; width: 60%"), "{html}");
    assert_eq!(count(&html, "p-slider-handle-active"), 1, "активна первая ручка: {html}");
    let first = html.find("p-slider-handle-active").unwrap();
    assert!(first < html.find(r#"data-pc-section="endHandler""#).unwrap(), "{html}");
    // заполнение между ручками независимо от порядка значений
    let html = view! { <Slider range=true values=RwSignal::new([80.0, 20.0]) /> }.to_html();
    assert!(html.contains("inset-inline-start: 20%; width: 60%"), "{html}");
    // `default_values` по умолчанию `[0, 0]`
    let html = view! { <Slider range=true /> }.to_html();
    assert_eq!(count(&html, r#"aria-valuenow="0""#), 2, "{html}");
    assert!(view! { <Slider range=true default_values=[10.0, 30.0] /> }.to_html().contains(r#"aria-valuenow="30""#));
}

#[test]
fn modifiers_and_data_p() {
    let html = view! { <Slider disabled=true invalid=true animate=true style_class="mine" /> }.to_html();
    assert!(html.contains("p-slider p-component p-disabled p-invalid p-slider-horizontal p-slider-animate mine"), "{html}");
    assert!(!html.contains("tabindex"), "у недоступного ползунка нет tabindex: {html}");
    assert!(view! { <Slider tabindex=3 /> }.to_html().contains(r#"tabindex="3""#));
}

#[test]
fn aria_labels_and_style() {
    let html = view! { <Slider aria_label="Volume" aria_labelledby="lbl" style="width: 14rem" /> }.to_html();
    assert!(html.contains(r#"aria-label="Volume""#) && html.contains(r#"aria-labelledby="lbl""#), "{html}");
    assert!(html.contains(r#"style="width: 14rem;""#), "{html}");
}

#[test]
fn pt_parts_are_added() {
    let pt = SliderPt {
        root: Pt::new().class("r").attr("data-r", "1"),
        range: Pt::new().class("rg"),
        handle: Pt::new().class("custom").attr("data-h", "2"),
        start_handler: Pt::new().class("s"),
        end_handler: Pt::new().class("e"),
    };
    let html = view! { <Slider pt=pt.clone() /> }.to_html();
    for expected in ["p-slider-horizontal r", r#"data-r="1""#, "p-slider-range rg", "p-slider-handle custom", r#"data-h="2""#] {
        assert!(html.contains(expected), "{expected}: {html}");
    }
    let html = view! { <Slider pt=pt range=true /> }.to_html();
    assert!(html.contains("p-slider-handle s") && html.contains("p-slider-handle e"), "{html}");
}

#[test]
fn reactive_props_render_like_static() {
    let max = RwSignal::new(Some(200.0));
    let reactive = view! { <Slider default_value=50.0 max=max /> }.to_html();
    let fixed = view! { <Slider default_value=50.0 max=200.0 /> }.to_html();
    let strip = |s: &str| s.replace("<!>", "");
    assert_eq!(strip(&reactive), strip(&fixed));
    max.set(Some(100.0));
    assert!(view! { <Slider default_value=50.0 max=max /> }.to_html().contains("inset-inline-start: 50%"));
}
