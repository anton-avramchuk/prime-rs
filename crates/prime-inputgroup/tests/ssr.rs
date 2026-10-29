//! Структура DOM, классы, `style`, `pt` и паритет ветвей InputGroup и InputGroupAddon (SSR).
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_core::Pt;
use prime_inputgroup::{InputGroup, InputGroupAddon, InputGroupAddonPt, InputGroupPt};

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

#[test]
fn group_wraps_children_in_order() {
    let html = view! {
        <InputGroup>
            <InputGroupAddon>"$"</InputGroupAddon>
            <input class="p-inputtext" />
            <InputGroupAddon>".00"</InputGroupAddon>
        </InputGroup>
    }
    .to_html();
    assert!(html.starts_with("<div"), "{html}");
    has(&html, &[r#"class="p-inputgroup""#, "p-inputgroupaddon"]);
    lacks(&html, &["role=", "data-p"]);
    assert!(pos(&html, "$") < pos(&html, "p-inputtext") && pos(&html, "p-inputtext") < pos(&html, ".00"), "{html}");
}

#[test]
fn addon_has_own_class_and_content() {
    let html = view! { <InputGroupAddon><i class="pi pi-user"></i></InputGroupAddon> }.to_html();
    has(&html, &[r#"class="p-inputgroupaddon""#, r#"<i class="pi pi-user">"#]);
}

#[test]
fn empty_components_render() {
    let group = view! { <InputGroup /> }.to_html();
    has(&group, &[r#"class="p-inputgroup""#]);
    lacks(&group, &["style="]);
    let addon = view! { <InputGroupAddon /> }.to_html();
    has(&addon, &[r#"class="p-inputgroupaddon""#]);
    lacks(&addon, &["style="]);
}

#[test]
fn style_and_class() {
    let group = view! { <InputGroup style="width: 20rem" style_class="mine" /> }.to_html();
    has(&group, &["p-inputgroup mine", "width: 20rem"]);
    let addon = view! { <InputGroupAddon style="width: 3rem" style_class="mine" /> }.to_html();
    has(&addon, &["p-inputgroupaddon mine", "width: 3rem"]);
}

#[test]
fn pt_root() {
    let group_pt = InputGroupPt { root: Pt::new().class("g").attr("data-testid", "group").style("margin: 0") };
    let group = view! { <InputGroup pt=group_pt /> }.to_html();
    has(&group, &["p-inputgroup g", r#"data-testid="group""#, "margin: 0"]);
    let addon_pt = InputGroupAddonPt { root: Pt::new().class("a") };
    let addon = view! { <InputGroupAddon pt=addon_pt /> }.to_html();
    has(&addon, &["p-inputgroupaddon a"]);
}

#[test]
fn static_and_reactive_branches_render_the_same() {
    let norm = |s: &str| s.replace("<!>", "").replace("<!---->", "");
    let fixed = view! { <InputGroup style="color: red" style_class="m">"x"</InputGroup> }.to_html();
    let live = view! {
        <InputGroup style=RwSignal::new(Some("color: red".to_owned())) style_class=RwSignal::new(Some("m".to_owned()))>"x"</InputGroup>
    }
    .to_html();
    assert_eq!(norm(&fixed), norm(&live), "{fixed}\n{live}");
    let fixed = view! { <InputGroupAddon style="color: red" style_class="m">"x"</InputGroupAddon> }.to_html();
    let live = view! {
        <InputGroupAddon style=RwSignal::new(Some("color: red".to_owned())) style_class=RwSignal::new(Some("m".to_owned()))>"x"</InputGroupAddon>
    }
    .to_html();
    assert_eq!(norm(&fixed), norm(&live), "{fixed}\n{live}");
}

#[test]
fn reactive_class_updates() {
    let owner = Owner::new();
    owner.with(|| {
        let class = RwSignal::new(Some("a".to_owned()));
        let html = view! { <InputGroup style_class=class /> }.to_html();
        has(&html, &["p-inputgroup a"]);
        class.set(Some("b".to_owned()));
        let html = view! { <InputGroup style_class=class /> }.to_html();
        has(&html, &["p-inputgroup b"]);
        lacks(&html, &["p-inputgroup a"]);
    });
}
