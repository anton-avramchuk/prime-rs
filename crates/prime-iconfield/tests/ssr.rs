//! Структура DOM, положения значка, `style`, `pt` и паритет ветвей IconField и InputIcon (SSR).
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_core::ClassNames;
use prime_core::Pt;
use prime_iconfield::{IconField, IconFieldPt, IconPosition, InputIcon, InputIconPt, write_classes};

fn has(html: &str, wants: &[&str]) {
    for want in wants {
        assert!(html.contains(want), "{want}: {html}");
    }
}

fn lacks(html: &str, unwanted: &[&str]) {
    for u in unwanted {
        assert!(!html.contains(u), "{u}: {html}");
    }
}

fn pos(html: &str, needle: &str) -> usize {
    html.find(needle).unwrap_or_else(|| panic!("нет {needle}: {html}"))
}

fn norm(s: &str) -> String {
    s.replace("<!>", "").replace("<!---->", "")
}

#[test]
fn field_wraps_children_in_order() {
    let html = view! { <IconField><input class="first" /><label class="second" /></IconField> }.to_html();
    assert!(html.starts_with("<span"), "{html}");
    has(&html, &[r#"class="p-iconfield p-iconfield-left""#]);
    assert!(pos(&html, "first") < pos(&html, "second"), "{html}");
}

#[test]
fn field_empty() {
    let html = view! { <IconField /> }.to_html();
    has(&html, &[r#"class="p-iconfield p-iconfield-left""#]);
    lacks(&html, &["style="]);
}

#[test]
fn field_style_and_class() {
    let html = view! { <IconField style="width: 20rem" style_class="mine" /> }.to_html();
    has(&html, &[r#"class="p-iconfield p-iconfield-left mine""#, r#"style="width: 20rem;""#]);
}

#[test]
fn field_pt() {
    let pt = IconFieldPt { root: Pt::new().class("custom").attr("data-x", "1").style("margin: 0") };
    let html = view! { <IconField style_class="mine" pt=pt /> }.to_html();
    has(&html, &[r#"class="p-iconfield p-iconfield-left mine custom""#, r#"data-x="1""#, "margin: 0"]);
}

#[test]
fn field_branches_match() {
    let fixed = view! { <IconField style="color: red" style_class="m">"x"</IconField> }.to_html();
    let live = view! {
        <IconField style=RwSignal::new(Some("color: red".to_owned())) style_class=RwSignal::new(Some("m".to_owned()))>"x"</IconField>
    }
    .to_html();
    assert_eq!(norm(&fixed), norm(&live), "{fixed}\n{live}");
}

#[test]
fn icon_wraps_children_in_order() {
    let html = view! { <InputIcon><svg class="first" /><svg class="second" /></InputIcon> }.to_html();
    assert!(html.starts_with("<span"), "{html}");
    has(&html, &[r#"class="p-inputicon""#]);
    assert!(pos(&html, "first") < pos(&html, "second"), "{html}");
}

#[test]
fn icon_empty() {
    let html = view! { <InputIcon /> }.to_html();
    has(&html, &[r#"class="p-inputicon""#]);
    lacks(&html, &["style="]);
}

#[test]
fn icon_style_and_class() {
    let html = view! { <InputIcon style="width: 20rem" style_class="mine" /> }.to_html();
    has(&html, &[r#"class="p-inputicon mine""#, r#"style="width: 20rem;""#]);
}

#[test]
fn icon_pt() {
    let pt = InputIconPt { root: Pt::new().class("custom").attr("data-x", "1").style("margin: 0") };
    let html = view! { <InputIcon style_class="mine" pt=pt /> }.to_html();
    has(&html, &[r#"class="p-inputicon mine custom""#, r#"data-x="1""#, "margin: 0"]);
}

#[test]
fn icon_branches_match() {
    let fixed = view! { <InputIcon style="color: red" style_class="m">"x"</InputIcon> }.to_html();
    let live = view! {
        <InputIcon style=RwSignal::new(Some("color: red".to_owned())) style_class=RwSignal::new(Some("m".to_owned()))>"x"</InputIcon>
    }
    .to_html();
    assert_eq!(norm(&fixed), norm(&live), "{fixed}\n{live}");
}

#[test]
fn write_classes_table() {
    for (position, want) in [(IconPosition::Left, "p-iconfield p-iconfield-left"), (IconPosition::Right, "p-iconfield p-iconfield-right")] {
        let mut cn = ClassNames::new();
        write_classes(&mut cn, position);
        assert_eq!(cn.finish_cow(), want);
    }
    assert_eq!(IconPosition::default(), IconPosition::Left);
}

#[test]
fn right_position_drops_left() {
    let html = view! { <IconField icon_position=IconPosition::Right /> }.to_html();
    has(&html, &["p-iconfield-right"]);
    lacks(&html, &["p-iconfield-left"]);
}

#[test]
fn icon_and_field_in_source_order() {
    let html = view! { <IconField><InputIcon><svg class="custom-icon" /></InputIcon><input class="p-inputtext" /></IconField> }.to_html();
    has(&html, &[r#"<span class="p-inputicon">"#, r#"<svg class="custom-icon""#]);
    assert!(pos(&html, "p-inputicon") < pos(&html, "p-inputtext"), "{html}");
    assert_eq!(html.matches("p-iconfield ").count(), 1, "{html}");
}

#[test]
fn input_icon_style_class_and_pt() {
    let pt = InputIconPt { root: Pt::new().class("custom") };
    let html = view! { <InputIcon style_class="pi" pt=pt /> }.to_html();
    has(&html, &[r#"class="p-inputicon pi custom""#]);
}

#[test]
fn reactive_position_replaces_class() {
    let position = RwSignal::new(IconPosition::Left);
    let first = view! { <IconField icon_position=position /> }.to_html();
    has(&first, &["p-iconfield-left"]);
    position.set(IconPosition::Right);
    let second = view! { <IconField icon_position=position /> }.to_html();
    has(&second, &["p-iconfield-right"]);
    lacks(&second, &["p-iconfield-left"]);
}
