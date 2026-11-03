//! Разметка Steps (SSR): корень, список, пункты, номера, активный пункт, `readonly`, `aria-*`, `tabindex`, `pt`.
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_core::Pt;
use prime_steps::{Steps, StepsItem, StepsPt};

fn render(f: impl FnOnce() -> String) -> String {
    Owner::new().with(f)
}

fn model() -> Vec<StepsItem> {
    vec![
        StepsItem::new().id("s0").label("Personal").url("/personal").target("_blank"),
        StepsItem::new().label("Seat"),
        StepsItem::new().label("Payment").disabled(true).tabindex(4).style("opacity: .5"),
    ]
}

#[test]
fn markup_has_nav_list_and_items() {
    let html = render(|| view! { <Steps model=model() active_index=1usize /> }.to_html());
    assert!(html.contains("<nav") && html.contains("p-steps p-component") && html.contains("data-pc-name=\"steps\""), "{html}");
    assert!(html.contains("class=\"p-steps-list\"") && html.contains("data-pc-section=\"menu\""), "{html}");
    assert_eq!(html.matches("data-pc-section=\"menuitem\"").count(), 3, "{html}");
    assert_eq!(html.matches("role=\"link\"").count(), 3, "{html}");
    for n in 1..=3 {
        assert!(html.contains(&format!("class=\"p-steps-item-number\">{n}<")), "номер {n}: {html}");
    }
    assert!(html.contains("class=\"p-steps-item-label\">Seat<") && html.contains("id=\"s0\""), "{html}");
    assert!(html.contains("href=\"/personal\"") && html.contains("target=\"_blank\""), "{html}");
}

#[test]
fn active_item_is_marked() {
    let html = render(|| view! { <Steps model=model() active_index=1usize readonly=false /> }.to_html());
    assert_eq!(html.matches("p-steps-item-active").count(), 1, "{html}");
    assert_eq!(html.matches("aria-current=\"step\"").count(), 1, "{html}");
    assert!(html.contains("aria-expanded=\"true\"") && html.contains("aria-expanded=\"false\""), "{html}");
    assert!(!html.contains("p-readonly"), "{html}");
}

#[test]
fn readonly_disables_inactive_items() {
    let html = render(|| view! { <Steps model=model() active_index=0usize /> }.to_html());
    assert!(html.contains("p-steps p-component p-readonly"), "{html}");
    assert_eq!(html.matches("p-disabled").count(), 2, "два неактивных: {html}");
    assert_eq!(html.matches("aria-disabled=\"true\"").count(), 2, "{html}");
    assert_eq!(html.matches("aria-disabled=\"false\"").count(), 1, "{html}");
}

#[test]
fn disabled_item_stays_disabled_in_interactive_mode() {
    let html = render(|| view! { <Steps model=model() active_index=0usize readonly=false /> }.to_html());
    assert_eq!(html.matches("p-disabled").count(), 1, "{html}");
    assert!(html.contains("opacity: .5"), "стиль пункта: {html}");
}

#[test]
fn tabindex_follows_active_disabled_and_own_values() {
    let html = render(|| view! { <Steps model=model() active_index=0usize readonly=false /> }.to_html());
    assert_eq!(html.matches("tabindex=\"0\"").count(), 1, "активный: {html}");
    assert_eq!(html.matches("tabindex=\"-1\"").count(), 2, "второй по умолчанию, третий отключён: {html}");
    let enabled = render(|| {
        let model = vec![StepsItem::new().label("a"), StepsItem::new().label("b").tabindex(4)];
        view! { <Steps model=model active_index=0usize readonly=false /> }.to_html()
    });
    assert!(enabled.contains("tabindex=\"4\""), "{enabled}");
}

#[test]
fn signal_drives_the_active_item() {
    let html = render(|| {
        let active = RwSignal::new(2usize);
        view! { <Steps model=model() active_index_signal=active readonly=false /> }.to_html()
    });
    assert!(html.contains("p-steps-item p-steps-item-active p-disabled") || html.contains("p-steps-item-active"), "{html}");
}

#[test]
fn style_class_style_and_pt() {
    let pt = StepsPt {
        root: Pt::new().class("r").attr("data-x", "1"),
        list: Pt::new().class("l"),
        item: Pt::new().class("i"),
        item_link: Pt::new().class("k"),
        item_number: Pt::new().class("n"),
        item_label: Pt::new().class("t"),
    };
    let html = render(|| view! { <Steps model=model() style_class="mine" style="width: 20rem" pt=pt /> }.to_html());
    assert!(
        html.contains("p-steps p-component p-readonly mine r") && html.contains("data-x=\"1\"") && html.contains("width: 20rem"),
        "{html}"
    );
    for needle in ["p-steps-list l", "p-steps-item-link k", "p-steps-item-number n", "p-steps-item-label t"] {
        assert!(html.contains(needle), "нет {needle}: {html}");
    }
    assert!(html.contains("p-steps-item p-steps-item-active i") || html.contains("p-steps-item i"), "{html}");
}

#[test]
fn empty_model_renders_an_empty_list() {
    let html = render(|| view! { <Steps /> }.to_html());
    assert!(html.contains("p-steps-list") && !html.contains("p-steps-item-link"), "{html}");
}
