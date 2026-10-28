//! Структура DOM, классы, `data-p`, текст и дети, слоты, `pt` и паритет ветвей Message (SSR).
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_core::{Pt, Size};
use prime_message::{Message, MessagePt, MessageSeverity, MessageVariant};

/// Убирает маркеры гидратации (`<!>` и `<!--…-->`): их число зависит от ветви, но не от разметки.
fn strip(s: &str) -> String {
    let mut out = s.replace("<!>", "");
    while let Some(start) = out.find("<!--") {
        let end = out[start..].find("-->").map_or(out.len(), |e| start + e + 3);
        out.replace_range(start..end, "");
    }
    out
}

#[test]
fn defaults_give_info_alert_structure() {
    let html = view! { <Message /> }.to_html();
    assert!(html.starts_with("<div"), "{html}");
    for want in [r#"role="alert""#, r#"aria-live="polite""#, "p-message p-component p-message-info", r#"data-p="info""#] {
        assert!(html.contains(want), "{want}: {html}");
    }
    let wrapper = html.find("p-message-content-wrapper").expect("wrapper");
    let content = html.find(r#"class="p-message-content""#).expect("content");
    assert!(wrapper < content, "{html}");
    for unwanted in ["p-message-close-button", "p-message-icon", "p-message-outlined", "p-message-sm", "display: none"] {
        assert!(!html.contains(unwanted), "{unwanted}: {html}");
    }
    assert_eq!(html.matches("p-message-text").count(), 1, "только span с детьми: {html}");
}

#[test]
fn severity_variant_size_classes_and_data_p() {
    let html = view! { <Message severity=MessageSeverity::Success variant=MessageVariant::Outlined size=Size::Small /> }.to_html();
    for want in ["p-message-success", "p-message-outlined", "p-message-sm", r#"data-p="outlined success small""#] {
        assert!(html.contains(want), "{want}: {html}");
    }
    let simple = view! { <Message severity=MessageSeverity::Warn variant=MessageVariant::Simple size=Size::Large /> }.to_html();
    assert!(simple.contains(r#"data-p="simple warn large""#) && simple.contains("p-message-lg"), "{simple}");
}

#[test]
fn text_then_children_in_two_spans() {
    let html = view! { <Message text="Info">"kids"</Message> }.to_html();
    assert_eq!(html.matches("p-message-text").count(), 2, "{html}");
    let text = html.find(">Info<").expect("text");
    let kids = html.find("kids").expect("children");
    assert!(text < kids, "{html}");
}

#[test]
fn icon_class_and_template() {
    let html = view! { <Message icon="pi pi-send" /> }.to_html();
    assert!(html.contains(r#"class="p-message-icon pi pi-send""#), "{html}");
    let tpl = ViewFn::from(|| view! { <b data-testid="ic"></b> });
    let html = view! { <Message icon_template=tpl /> }.to_html();
    assert!(html.contains("data-testid=\"ic\"") && !html.contains("p-message-icon"), "{html}");
}

#[test]
fn closable_renders_button_with_aria_label_and_icon() {
    let html = view! { <Message closable=true /> }.to_html();
    for want in ["<button", r#"type="button""#, "p-message-close-button", r#"aria-label="Close""#, "<svg", "p-message-close-icon"] {
        assert!(html.contains(want), "{want}: {html}");
    }
    let custom = view! { <Message closable=true close_icon="pi pi-x" /> }.to_html();
    assert!(custom.contains(r#"class="p-message-close-icon pi pi-x""#) && !custom.contains("<svg"), "{custom}");
}

#[test]
fn container_template_replaces_text_and_children() {
    let tpl = Callback::new(|_close: Callback<web_sys::Event>| view! { <u data-testid="c"></u> }.into_any());
    let html = view! { <Message text="Hidden" container_template=tpl>"also hidden"</Message> }.to_html();
    assert!(html.contains("data-testid=\"c\"") && !html.contains("Hidden") && !html.contains("also hidden"), "{html}");
    assert!(!html.contains("p-message-text"), "{html}");
}

#[test]
fn pt_parts_are_added() {
    let pt = MessagePt {
        root: Pt::new().class("r"),
        content_wrapper: Pt::new().class("w"),
        content: Pt::new().class("c"),
        text: Pt::new().class("t"),
        close_button: Pt::new().class("b"),
        ..Default::default()
    };
    let html = view! { <Message text="x" closable=true style_class="mine" pt=pt /> }.to_html();
    for want in
        ["p-message-info mine r", "p-message-content-wrapper w", "p-message-content c", "p-message-text t", "p-message-close-button b"]
    {
        assert!(html.contains(want), "{want}: {html}");
    }
}

#[test]
fn static_and_reactive_branches_match() {
    let stat =
        view! { <Message severity=MessageSeverity::Error size=Size::Large text="T" icon="ic" style_class="x">"k"</Message> }.to_html();
    let reactive = view! {
        <Message
            severity=RwSignal::new(MessageSeverity::Error)
            size=RwSignal::new(Some(Size::Large))
            text=RwSignal::new(Some("T".to_owned()))
            icon=RwSignal::new(Some("ic".to_owned()))
            style_class=RwSignal::new(Some("x".to_owned()))
        >
            "k"
        </Message>
    }
    .to_html();
    assert_eq!(strip(&stat), strip(&reactive));
}
