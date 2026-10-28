//! Структура DOM, классы, `pt` и паритет статической и реактивной ветвей Tag (SSR).
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_core::Pt;
use prime_tag::{Tag, TagPt, TagSeverity};

fn icon() -> ViewFn {
    ViewFn::from(|| view! { <svg data-testid="i"></svg> })
}

#[test]
fn value_is_rendered_in_the_label_span() {
    let html = view! { <Tag value="New" /> }.to_html();
    assert!(html.starts_with("<span"), "{html}");
    assert!(html.contains("p-tag p-component"), "{html}");
    assert!(html.contains("p-tag-label") && html.contains(">New<"), "{html}");
}

#[test]
fn empty_tag_keeps_an_empty_label_like_primeng() {
    let html = view! { <Tag /> }.to_html();
    assert!(html.contains("p-tag-label"), "{html}");
    assert!(!html.contains("p-tag-icon") && !html.contains("p-tag-rounded"), "{html}");
}

#[test]
fn severity_and_rounded_set_classes_and_data_p() {
    let html = view! { <Tag value="x" severity=TagSeverity::Success rounded=true /> }.to_html();
    assert!(html.contains("p-tag-success") && html.contains("p-tag-rounded"), "{html}");
    assert!(html.contains(r#"data-p="rounded success""#), "{html}");
}

#[test]
fn style_class_and_style_keep_own_classes() {
    let html = view! { <Tag value="x" style_class="custom" style="margin: 4px" /> }.to_html();
    assert!(html.contains("custom") && html.contains("p-tag p-component"), "{html}");
    assert!(html.contains("margin: 4px"), "{html}");
}

#[test]
fn children_come_first_then_icon_then_label() {
    let html = view! {
        <Tag value="Текст" icon=icon()>
            <i>"перед"</i>
        </Tag>
    }
    .to_html();
    let pos = |needle: &str| html.find(needle).unwrap_or_else(|| panic!("нет `{needle}`: {html}"));
    assert!(pos("<i>перед</i>") < pos("p-tag-icon") && pos("p-tag-icon") < pos("p-tag-label"), "{html}");
}

#[test]
fn no_icon_element_without_icon() {
    let html = view! { <Tag value="x" /> }.to_html();
    assert!(!html.contains("p-tag-icon"), "{html}");
}

#[test]
fn pt_on_parts_keeps_own_classes() {
    let pt = TagPt {
        root: Pt::new().class("root-extra").attr("data-testid", "tag").style("color: red"),
        icon: Pt::new().class("icon-extra"),
        label: Pt::new().class("label-extra"),
    };
    let html = view! { <Tag value="x" icon=icon() pt=pt /> }.to_html();
    for c in ["root-extra", "p-tag p-component", "icon-extra", "p-tag-icon", "label-extra", "p-tag-label"] {
        assert!(html.contains(c), "{c}: {html}");
    }
    assert!(html.contains(r#"data-testid="tag""#) && html.contains("color: red"), "{html}");
}

#[test]
fn parity_table_static_vs_reactive() {
    type Case = (Option<&'static str>, Option<TagSeverity>, bool, Option<&'static str>, Option<&'static str>);
    let cases: Vec<Case> = vec![
        (None, None, false, None, None),
        (Some("New"), None, false, None, None),
        (Some("Success"), Some(TagSeverity::Success), true, None, None),
        (Some("x"), Some(TagSeverity::Contrast), false, Some("margin: 4px"), Some("a b")),
        (Some("я"), Some(TagSeverity::Danger), true, Some("color: red;"), None),
    ];
    let owner = Owner::new();
    owner.with(|| {
        for (value, severity, rounded, style, class) in cases {
            let o = |s: Option<&str>| s.map(str::to_owned);
            let fixed = view! { <Tag value=o(value) severity=severity rounded=rounded style=o(style) style_class=o(class) /> }.to_html();
            let live = view! {
                <Tag
                    value=RwSignal::new(o(value))
                    severity=RwSignal::new(severity)
                    rounded=RwSignal::new(rounded)
                    style=RwSignal::new(o(style))
                    style_class=RwSignal::new(o(class))
                />
            }
            .to_html();
            assert_eq!(fixed, live, "{value:?} {severity:?} {rounded} {style:?} {class:?}");
        }
    });
}

#[test]
fn one_signal_switches_path_without_changing_output() {
    let owner = Owner::new();
    owner.with(|| {
        let a = view! { <Tag value="x" severity=TagSeverity::Info /> }.to_html();
        let b = view! { <Tag value="x" severity=TagSeverity::Info rounded=RwSignal::new(false) /> }.to_html();
        assert_eq!(a, b);
    });
}

#[test]
fn reactive_value_updates() {
    let owner = Owner::new();
    owner.with(|| {
        let value = RwSignal::new(Some("a".to_owned()));
        let html = view! { <Tag value=value /> }.to_html();
        assert!(html.contains(">a<"), "{html}");
        value.set(Some("b".into()));
        let html = view! { <Tag value=value /> }.to_html();
        assert!(html.contains(">b<"), "{html}");
    });
}
