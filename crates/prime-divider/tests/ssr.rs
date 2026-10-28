//! Структура DOM, классы, стиль, `pt` и паритет ветвей Divider (SSR).
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_core::Pt;
use prime_divider::{Divider, DividerAlign, DividerLayout, DividerPt, DividerType};

#[test]
fn defaults() {
    let html = view! { <Divider /> }.to_html();
    assert!(html.starts_with("<div"), "{html}");
    assert!(html.contains(r#"role="separator""#) && html.contains(r#"aria-orientation="horizontal""#), "{html}");
    for c in ["p-divider p-component", "p-divider-horizontal", "p-divider-solid", "p-divider-left"] {
        assert!(html.contains(c), "{c}: {html}");
    }
    assert!(html.contains(r#"data-p="horizontal solid""#) && html.contains("justify-content: center"), "{html}");
}

#[test]
fn horizontal_center_dotted() {
    let html = view! { <Divider align=DividerAlign::Center divider_type=DividerType::Dotted /> }.to_html();
    assert!(html.contains("p-divider-dotted") && html.contains("p-divider-center") && !html.contains("p-divider-left"), "{html}");
    assert!(html.contains(r#"data-p="center horizontal dotted""#), "{html}");
}

#[test]
fn vertical_defaults_to_center() {
    let html = view! { <Divider layout=DividerLayout::Vertical /> }.to_html();
    assert!(html.contains("p-divider-vertical") && html.contains("p-divider-center"), "{html}");
    assert!(html.contains(r#"aria-orientation="vertical""#) && html.contains("align-items: center"), "{html}");
}

#[test]
fn invalid_alignment_has_no_alignment_class_or_style() {
    let html = view! { <Divider layout=DividerLayout::Vertical align=DividerAlign::Left /> }.to_html();
    assert!(!html.contains("p-divider-left") && !html.contains("p-divider-center"), "{html}");
    assert!(!html.contains("style="), "{html}");
}

#[test]
fn alignment_style_per_side() {
    let right = view! { <Divider align=DividerAlign::Right /> }.to_html();
    assert!(right.contains("justify-content: flex-end") && right.contains("p-divider-right"), "{right}");
    let bottom = view! { <Divider layout=DividerLayout::Vertical align=DividerAlign::Bottom /> }.to_html();
    assert!(bottom.contains("align-items: flex-end") && bottom.contains("p-divider-bottom"), "{bottom}");
}

#[test]
fn custom_style_is_added_to_alignment_style() {
    let html = view! { <Divider style="margin: 4px" /> }.to_html();
    assert!(html.contains("margin: 4px") && html.contains("justify-content: center"), "{html}");
}

#[test]
fn style_class_keeps_own_classes() {
    let html = view! { <Divider style_class="custom" /> }.to_html();
    assert!(html.contains("custom") && html.contains("p-divider p-component"), "{html}");
}

#[test]
fn content_block_is_always_present() {
    let empty = view! { <Divider /> }.to_html();
    assert!(empty.contains(r#"<div class="p-divider-content""#), "{empty}");
    let with = view! { <Divider>"Text"</Divider> }.to_html();
    assert!(with.contains(">Text<"), "{with}");
    assert!(with.find("p-divider-content").unwrap() < with.find("Text").unwrap(), "{with}");
}

#[test]
fn pt_on_parts_keeps_own_classes() {
    let pt = DividerPt { root: Pt::new().class("root-extra").attr("data-testid", "d"), content: Pt::new().class("content-extra") };
    let html = view! { <Divider pt=pt>"x"</Divider> }.to_html();
    for c in ["root-extra", "p-divider p-component", "content-extra", "p-divider-content"] {
        assert!(html.contains(c), "{c}: {html}");
    }
    assert!(html.contains(r#"data-testid="d""#), "{html}");
}

#[test]
fn parity_table_static_vs_reactive() {
    type Case = (DividerLayout, DividerType, Option<DividerAlign>, Option<&'static str>, Option<&'static str>);
    let cases: Vec<Case> = vec![
        (DividerLayout::Horizontal, DividerType::Solid, None, None, None),
        (DividerLayout::Horizontal, DividerType::Dashed, Some(DividerAlign::Right), Some("margin: 1px"), Some("a b")),
        (DividerLayout::Vertical, DividerType::Dotted, Some(DividerAlign::Top), None, None),
        (DividerLayout::Vertical, DividerType::Solid, Some(DividerAlign::Left), Some("color: red;"), None),
    ];
    let owner = Owner::new();
    owner.with(|| {
        for (layout, ty, align, style, class) in cases {
            let o = |s: Option<&str>| s.map(str::to_owned);
            let fixed =
                view! { <Divider layout=layout divider_type=ty align=align style=o(style) style_class=o(class)>"X"</Divider> }.to_html();
            let live = view! {
                <Divider
                    layout=RwSignal::new(layout)
                    divider_type=RwSignal::new(ty)
                    align=RwSignal::new(align)
                    style=RwSignal::new(o(style))
                    style_class=RwSignal::new(o(class))
                >
                    "X"
                </Divider>
            }
            .to_html();
            assert_eq!(fixed, live, "{layout:?} {ty:?} {align:?} {style:?} {class:?}");
        }
    });
}

#[test]
fn reactive_layout_updates() {
    let owner = Owner::new();
    owner.with(|| {
        let layout = RwSignal::new(DividerLayout::Horizontal);
        assert!(view! { <Divider layout=layout /> }.to_html().contains("p-divider-horizontal"));
        layout.set(DividerLayout::Vertical);
        assert!(view! { <Divider layout=layout /> }.to_html().contains("p-divider-vertical"));
    });
}
