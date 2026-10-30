//! Структура DOM, проценты, подписи, шаблоны и `pt` MeterGroup (SSR).
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_core::Pt;
use prime_metergroup::{LabelPosition, MeterGroup, MeterGroupPt, MeterItem, MeterOrientation};

fn strip(s: &str) -> String {
    let mut out = s.replace("<!>", "");
    while let Some(start) = out.find("<!--") {
        let end = out[start..].find("-->").map_or(out.len(), |e| start + e + 3);
        out.replace_range(start..end, "");
    }
    out
}

fn items() -> Vec<MeterItem> {
    vec![MeterItem::new("Приложения", "#34d399", 20.0), MeterItem::new("Сообщения", "#fbbf24", 30.0)]
}

#[test]
fn root_defaults_and_total() {
    let html = strip(&view! { <MeterGroup value={items()} /> }.to_html());
    for needle in [
        "role=\"meter\"",
        "aria-valuemin=\"0\"",
        "aria-valuemax=\"100\"",
        "aria-valuenow=\"50\"",
        "p-metergroup p-component p-metergroup-horizontal",
        "data-p=\"horizontal\"",
    ] {
        assert!(html.contains(needle), "{needle}: {html}");
    }
}

#[test]
fn vertical_uses_height() {
    let html =
        strip(&view! { <MeterGroup value={vec![MeterItem::new("a", "red", 40.0)]} orientation=MeterOrientation::Vertical /> }.to_html());
    assert!(html.contains("p-metergroup-vertical") && html.contains("height: 40%"), "{html}");
    assert!(!html.contains("width: 40%"), "{html}");
}

#[test]
fn meter_width_color_and_range() {
    let html = strip(&view! { <MeterGroup value={vec![MeterItem::new("a", "#f00", 50.0)]} max=200.0 /> }.to_html());
    assert!(html.contains("background-color: #f00") && html.contains("width: 25%"), "{html}");
}

#[test]
fn clamps_and_skips_non_positive() {
    let list = vec![MeterItem::new("big", "red", 150.0), MeterItem::new("neg", "blue", -10.0), MeterItem::new("zero", "green", 0.0)];
    let html = strip(&view! { <MeterGroup value={list} /> }.to_html());
    assert_eq!(html.matches("class=\"p-metergroup-meter\"").count(), 1, "{html}");
    assert!(html.contains("width: 100%"), "{html}");
}

#[test]
fn label_text_and_marker() {
    let html = strip(&view! { <MeterGroup value={vec![MeterItem::new("Приложения", "#34d399", 16.0)]} /> }.to_html());
    assert!(html.contains("Приложения (16%)"), "{html}");
    assert!(html.contains("p-metergroup-label-marker") && html.contains("background-color: #34d399"), "{html}");
    assert!(html.contains("p-metergroup-label-list p-metergroup-label-list-horizontal"), "{html}");
}

#[test]
fn labels_follow_position() {
    let at_end = strip(&view! { <MeterGroup value={items()} /> }.to_html());
    assert!(at_end.find("p-metergroup-meters").unwrap() < at_end.find("p-metergroup-label-list").unwrap(), "{at_end}");
    let at_start = strip(&view! { <MeterGroup value={items()} label_position=LabelPosition::Start /> }.to_html());
    assert!(at_start.find("p-metergroup-label-list").unwrap() < at_start.find("p-metergroup-meters").unwrap(), "{at_start}");
    assert_eq!(at_start.matches("p-metergroup-label-list ").count(), 1, "{at_start}");
}

#[test]
fn vertical_label_list() {
    let html = strip(&view! { <MeterGroup value={items()} label_orientation=MeterOrientation::Vertical /> }.to_html());
    assert!(html.contains("p-metergroup-label-list-vertical"), "{html}");
}

#[test]
fn item_icon_replaces_marker_and_takes_color() {
    let list = vec![MeterItem::new("a", "#123456", 10.0).icon(ViewFn::from(|| view! { <svg></svg> }))];
    let html = strip(&view! { <MeterGroup value={list} /> }.to_html());
    assert!(html.contains("p-metergroup-label-icon") && html.contains("color: #123456") && html.contains("<svg"), "{html}");
    assert!(!html.contains("p-metergroup-label-marker"), "{html}");
}

#[test]
fn label_template_replaces_list_and_gets_context() {
    let label = Callback::new(|c: prime_metergroup::MeterGroupContext| {
        view! { <i>{format!("{}/{}/{:?}", c.items.len(), c.total_percent, c.percentages)}</i> }.into_any()
    });
    let html = strip(&view! { <MeterGroup value={items()} label=label /> }.to_html());
    assert!(html.contains("<i>2/50/[20.0, 50.0]</i>"), "{html}");
    assert!(!html.contains("p-metergroup-label-list"), "{html}");
}

#[test]
fn start_end_meter_templates() {
    let start = Callback::new(|_| view! { <u>"S"</u> }.into_any());
    let end = Callback::new(|_| view! { <u>"E"</u> }.into_any());
    let meter = Callback::new(|(item, i): (MeterItem, usize)| view! { <s>{format!("{i}{}", item.label)}</s> }.into_any());
    let html = strip(&view! { <MeterGroup value={items()} start=start end=end meter=meter /> }.to_html());
    assert!(html.find("<u>S</u>").unwrap() < html.find("p-metergroup-meters").unwrap(), "{html}");
    assert!(html.find("<u>E</u>").unwrap() > html.find("p-metergroup-meters").unwrap(), "{html}");
    assert!(html.contains("<s>0Приложения</s>") && html.contains("<s>1Сообщения</s>"), "{html}");
    assert!(!html.contains("class=\"p-metergroup-meter\""), "{html}");
}

#[test]
fn style_class_style_and_pt() {
    let pt = MeterGroupPt { root: Pt::new().attr("data-x", "1"), meter: Pt::new().class("custom"), ..Default::default() };
    let html = strip(&view! { <MeterGroup value={items()} style_class="mine" style="width: 5px" pt=pt /> }.to_html());
    assert!(html.contains("p-metergroup-horizontal mine") && html.contains("width: 5px") && html.contains("data-x=\"1\""), "{html}");
    assert_eq!(html.matches("p-metergroup-meter custom").count(), 2, "{html}");
}

#[test]
fn equal_min_max_is_full() {
    let html = strip(&view! { <MeterGroup value={vec![MeterItem::new("a", "red", 5.0)]} min=5.0 max=5.0 /> }.to_html());
    assert!(html.contains("width: 100%"), "{html}");
}
