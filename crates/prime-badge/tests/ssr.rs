//! Структура DOM, `display: none`, `pt` (SSR).
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_badge::{Badge, BadgePt, BadgeSeverity, BadgeSize};
use prime_core::Pt;

#[test]
fn value_is_rendered_as_text_in_a_span() {
    let html = view! { <Badge value="5" /> }.to_html();
    assert!(html.starts_with("<span"), "{html}");
    assert!(html.contains(">5<"), "{html}");
    assert!(html.contains(r#"class="p-badge p-component p-badge-circle""#), "{html}");
    assert!(html.contains(r#"data-p="circle""#), "{html}");
}

#[test]
fn defaults_render_a_dot_without_text_severity_or_size() {
    let html = view! { <Badge /> }.to_html();
    assert!(html.contains("p-badge-dot"), "{html}");
    assert!(html.contains(r#"data-p="empty""#), "{html}");
    assert!(!html.contains("style="), "{html}");
    for unwanted in ["p-badge-sm", "p-badge-lg", "p-badge-xl", "p-badge-info", "p-badge-secondary"] {
        assert!(!html.contains(unwanted), "{unwanted}: {html}");
    }
}

#[test]
fn severity_size_and_style_class() {
    let html = view! { <Badge value="12" severity=BadgeSeverity::Danger size=BadgeSize::XLarge style_class="custom" /> }.to_html();
    assert!(html.contains("p-badge-danger") && html.contains("p-badge-xl"), "{html}");
    assert!(html.contains("custom") && html.contains("p-badge p-component"), "{html}");
}

#[test]
fn badge_disabled_hides_but_keeps_in_dom() {
    let html = view! { <Badge value="1" badge_disabled=true /> }.to_html();
    assert!(html.contains(r#"style="display: none;""#), "{html}");
    assert!(html.contains("disabled"), "data-p: {html}");
    let shown = view! { <Badge value="1" badge_disabled=false /> }.to_html();
    assert!(!shown.contains("display"), "{shown}");
}

#[test]
fn pt_adds_class_and_attrs_keeping_own_classes() {
    let pt = BadgePt { root: Pt::new().class("custom").attr("data-testid", "b").style("margin: 1px") };
    let html = view! { <Badge value="ab" pt=pt /> }.to_html();
    assert!(html.contains("p-badge p-component") && html.contains("custom"), "{html}");
    assert!(html.contains(r#"data-testid="b""#) && html.contains("margin: 1px"), "{html}");
}

#[test]
fn reactive_value_and_severity() {
    let owner = Owner::new();
    owner.with(|| {
        let value = RwSignal::new(Some("1".to_owned()));
        let severity = RwSignal::new(None::<BadgeSeverity>);
        let html = view! { <Badge value=value severity=severity /> }.to_html();
        assert!(html.contains("p-badge-circle"), "{html}");
        value.set(Some("100".into()));
        severity.set(Some(BadgeSeverity::Success));
        let html = view! { <Badge value=value severity=severity /> }.to_html();
        assert!(html.contains("p-badge-success") && !html.contains("p-badge-circle"), "{html}");
    });
}

#[test]
fn static_and_reactive_props_render_identically() {
    let owner = Owner::new();
    owner.with(|| {
        let fixed = view! { <Badge value="7" severity=BadgeSeverity::Info size=BadgeSize::Large style_class="x" /> }.to_html();
        let value = RwSignal::new(Some("7".to_owned()));
        let severity = RwSignal::new(Some(BadgeSeverity::Info));
        let size = RwSignal::new(Some(BadgeSize::Large));
        let class = RwSignal::new(Some("x".to_owned()));
        let live = view! { <Badge value=value severity=severity size=size style_class=class /> }.to_html();
        assert_eq!(fixed, live);
    });
}

#[test]
fn parity_table_static_vs_reactive() {
    type Case = (Option<&'static str>, Option<BadgeSeverity>, Option<BadgeSize>, bool, Option<&'static str>);
    let cases: Vec<Case> = vec![
        (None, None, None, false, None),
        (Some("5"), None, None, false, None),
        (Some("New"), Some(BadgeSeverity::Success), None, false, None),
        (Some(""), None, Some(BadgeSize::Small), false, None),
        (Some("12"), Some(BadgeSeverity::Danger), Some(BadgeSize::XLarge), true, Some("custom")),
        (Some("я"), Some(BadgeSeverity::Contrast), Some(BadgeSize::Large), false, Some("a b")),
    ];
    let owner = Owner::new();
    owner.with(|| {
        for (value, severity, size, disabled, class) in cases {
            let fixed = view! {
                <Badge value=value.map(str::to_owned) severity=severity size=size badge_disabled=disabled style_class=class.map(str::to_owned) />
            }
            .to_html();
            let (v, sv, sz) = (RwSignal::new(value.map(str::to_owned)), RwSignal::new(severity), RwSignal::new(size));
            let (d, c) = (RwSignal::new(disabled), RwSignal::new(class.map(str::to_owned)));
            let live = view! { <Badge value=v severity=sv size=sz badge_disabled=d style_class=c /> }.to_html();
            assert_eq!(fixed, live, "{value:?} {severity:?} {size:?} {disabled} {class:?}");
        }
    });
}
