//! Структура DOM, порядок блоков, приоритет слотов, `pt` и паритет ветвей Card (SSR).
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_card::{Card, CardPt};
use prime_core::Pt;

fn slot(text: &'static str) -> ViewFn {
    ViewFn::from(move || view! { <b>{text}</b> })
}

fn pos(html: &str, needle: &str) -> usize {
    html.find(needle).unwrap_or_else(|| panic!("нет `{needle}` в {html}"))
}

#[test]
fn empty_card_has_only_root_body_and_content() {
    let html = view! { <Card /> }.to_html();
    assert!(html.starts_with("<div") && html.contains("p-card p-component"), "{html}");
    assert!(html.contains("p-card-body") && html.contains("p-card-content"), "{html}");
    for unwanted in ["p-card-header", "p-card-title", "p-card-subtitle", "p-card-footer"] {
        assert!(!html.contains(unwanted), "{unwanted}: {html}");
    }
}

#[test]
fn simple_card_has_title_text_and_content() {
    let html = view! {
        <Card header="Simple Card">
            <p>"Содержимое"</p>
        </Card>
    }
    .to_html();
    assert!(html.contains("p-card-title") && html.contains(">Simple Card<"), "{html}");
    assert!(html.contains("<p>Содержимое</p>"), "{html}");
    assert!(pos(&html, "p-card-title") < pos(&html, "p-card-content"), "{html}");
}

#[test]
fn advanced_card_block_order() {
    let html = view! {
        <Card header_template=slot("H") title_template=slot("T") subtitle_template=slot("S") footer_template=slot("F")>
            <p>"C"</p>
        </Card>
    }
    .to_html();
    let order = ["p-card-header", "p-card-body", "p-card-title", "p-card-subtitle", "p-card-content", "p-card-footer"];
    let positions: Vec<usize> = order.iter().map(|n| pos(&html, n)).collect();
    assert!(positions.windows(2).all(|w| w[0] < w[1]), "порядок блоков: {html}");
    // шапка вне тела, подвал внутри тела
    let body_end = html.rfind("</div></div>").unwrap_or(html.len());
    assert!(pos(&html, "p-card-footer") < body_end, "{html}");
}

#[test]
fn subheader_text_is_rendered() {
    let html = view! { <Card header="T" subheader="Sub" /> }.to_html();
    assert!(html.contains("p-card-subtitle") && html.contains(">Sub<"), "{html}");
}

#[test]
fn slot_wins_over_text() {
    let html = view! { <Card header="Text" title_template=slot("Slot") subheader="SubText" subtitle_template=slot("SubSlot") /> }.to_html();
    assert!(html.contains("<b>Slot</b>") && !html.contains("Text<"), "{html}");
    assert!(html.contains("<b>SubSlot</b>") && !html.contains(">SubText<"), "{html}");
}

#[test]
fn header_and_footer_appear_only_with_their_slots() {
    let only_footer = view! { <Card footer_template=slot("F") /> }.to_html();
    assert!(only_footer.contains("p-card-footer") && !only_footer.contains("p-card-header"), "{only_footer}");
    let only_header = view! { <Card header_template=slot("H") /> }.to_html();
    assert!(only_header.contains("p-card-header") && !only_header.contains("p-card-footer"), "{only_header}");
}

#[test]
fn style_class_and_style_keep_own_classes() {
    let html = view! { <Card style_class="custom" style="width: 25rem" /> }.to_html();
    assert!(html.contains("custom") && html.contains("p-card p-component") && html.contains("width: 25rem"), "{html}");
}

#[test]
fn pt_on_parts_keeps_own_classes() {
    let pt = CardPt {
        root: Pt::new().class("root-extra").attr("data-testid", "card"),
        body: Pt::new().class("body-extra"),
        title: Pt::new().class("title-extra"),
        content: Pt::new().class("content-extra"),
        ..Default::default()
    };
    let html = view! { <Card header="T" pt=pt>"x"</Card> }.to_html();
    for c in
        ["root-extra", "p-card p-component", "body-extra", "p-card-body", "title-extra", "p-card-title", "content-extra", "p-card-content"]
    {
        assert!(html.contains(c), "{c}: {html}");
    }
    assert!(html.contains(r#"data-testid="card""#), "{html}");
}

#[test]
fn parity_table_static_vs_reactive() {
    type Case = (Option<&'static str>, Option<&'static str>, Option<&'static str>, Option<&'static str>);
    let cases: Vec<Case> = vec![
        (None, None, None, None),
        (Some("Simple Card"), None, None, None),
        (Some("T"), Some("Sub"), Some("width: 25rem"), Some("custom")),
        (None, Some("Only sub"), Some("color: red;"), None),
    ];
    let owner = Owner::new();
    owner.with(|| {
        for (header, subheader, style, class) in cases {
            let o = |s: Option<&str>| s.map(str::to_owned);
            let fixed = view! {
                <Card header=o(header) subheader=o(subheader) style=o(style) style_class=o(class) footer_template=slot("F")>
                    <p>"C"</p>
                </Card>
            }
            .to_html();
            let live = view! {
                <Card
                    header=RwSignal::new(o(header))
                    subheader=RwSignal::new(o(subheader))
                    style=RwSignal::new(o(style))
                    style_class=RwSignal::new(o(class))
                    footer_template=slot("F")
                >
                    <p>"C"</p>
                </Card>
            }
            .to_html();
            assert_eq!(fixed, live, "{header:?} {subheader:?} {style:?} {class:?}");
        }
    });
}

#[test]
fn reactive_header_updates() {
    let owner = Owner::new();
    owner.with(|| {
        let header = RwSignal::new(Some("a".to_owned()));
        assert!(view! { <Card header=header /> }.to_html().contains(">a<"));
        header.set(Some("b".into()));
        assert!(view! { <Card header=header /> }.to_html().contains(">b<"));
    });
}
