//! Структура DOM, классы, `style`, `pt` и паритет ветвей ButtonGroup (SSR).
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_buttongroup::{ButtonGroup, ButtonGroupPt};
use prime_core::Pt;

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
fn root_wraps_children_in_order() {
    let html = view! { <ButtonGroup><button class="first" /><button class="second" /></ButtonGroup> }.to_html();
    assert!(html.starts_with("<span"), "{html}");
    has(&html, &[r#"class="p-buttongroup p-component""#, r#"role="group""#]);
    assert!(pos(&html, "first") < pos(&html, "second"), "{html}");
}

#[test]
fn empty_renders_root_only() {
    let html = view! { <ButtonGroup /> }.to_html();
    has(&html, &[r#"class="p-buttongroup p-component""#]);
    lacks(&html, &["style="]);
}

#[test]
fn style_and_class() {
    let html = view! { <ButtonGroup style="width: 20rem" style_class="mine" /> }.to_html();
    has(&html, &[r#"class="p-buttongroup p-component mine""#, r#"style="width: 20rem;""#]);
}

#[test]
fn pt_root_adds_without_replacing() {
    let pt = ButtonGroupPt { root: Pt::new().class("custom").attr("data-x", "1").style("margin: 0") };
    let html = view! { <ButtonGroup style_class="mine" pt=pt /> }.to_html();
    has(&html, &[r#"class="p-buttongroup p-component mine custom""#, r#"data-x="1""#, "margin: 0"]);
}

#[test]
fn static_and_reactive_branches_render_the_same() {
    let fixed = view! { <ButtonGroup style="color: red" style_class="m">"x"</ButtonGroup> }.to_html();
    let live = view! {
        <ButtonGroup style=RwSignal::new(Some("color: red".to_owned())) style_class=RwSignal::new(Some("m".to_owned()))>"x"</ButtonGroup>
    }
    .to_html();
    assert_eq!(norm(&fixed), norm(&live), "{fixed}\n{live}");
}

#[test]
fn three_buttons_keep_order() {
    let html = view! {
        <ButtonGroup>
            <button class="p-button">"A"</button>
            <button class="p-button">"B"</button>
            <button class="p-button">"C"</button>
        </ButtonGroup>
    }
    .to_html();
    assert_eq!(html.matches("<button").count(), 3, "{html}");
    assert!(pos(&html, ">A<") < pos(&html, ">B<") && pos(&html, ">B<") < pos(&html, ">C<"), "{html}");
}
