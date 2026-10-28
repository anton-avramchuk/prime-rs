//! Структура DOM, порядок областей, роль, атрибуты, `style`, `pt` и паритет ветвей Toolbar (SSR).
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_core::Pt;
use prime_toolbar::{Toolbar, ToolbarPt};

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

fn slot(text: &'static str) -> ViewFn {
    ViewFn::from(move || view! { <i>{text}</i> })
}

#[test]
fn three_areas_in_order_with_role() {
    let html = view! { <Toolbar start_template=slot("s") center_template=slot("c") end_template=slot("e") /> }.to_html();
    assert!(html.starts_with("<div"), "{html}");
    has(&html, &["p-toolbar p-component", r#"role="toolbar""#]);
    let (start, center, end) = (pos(&html, "p-toolbar-start"), pos(&html, "p-toolbar-center"), pos(&html, "p-toolbar-end"));
    assert!(start < center && center < end, "{html}");
    has(&html, &["<i>s</i>", "<i>c</i>", "<i>e</i>"]);
}

#[test]
fn empty_areas_are_not_rendered() {
    let only_end = view! { <Toolbar end_template=slot("e") /> }.to_html();
    has(&only_end, &["p-toolbar-end"]);
    lacks(&only_end, &["p-toolbar-start", "p-toolbar-center"]);
    let none = view! { <Toolbar /> }.to_html();
    lacks(&none, &["p-toolbar-start", "p-toolbar-center", "p-toolbar-end", "aria-labelledby", "style="]);
    has(&none, &["p-toolbar p-component", r#"role="toolbar""#]);
}

#[test]
fn children_come_before_the_areas() {
    let html = view! {
        <Toolbar start_template=slot("s")>
            <b>"projected"</b>
        </Toolbar>
    }
    .to_html();
    assert!(pos(&html, "<b>projected</b>") < pos(&html, "p-toolbar-start"), "{html}");
}

#[test]
fn aria_labelledby_style_and_class() {
    let html = view! { <Toolbar aria_labelledby="title" style="border-radius: 3rem" style_class="mine" /> }.to_html();
    has(&html, &[r#"aria-labelledby="title""#, "border-radius: 3rem", "p-toolbar p-component mine"]);
}

#[test]
fn pt_parts() {
    let pt = ToolbarPt {
        root: Pt::new().class("r").attr("data-testid", "root"),
        start: Pt::new().class("s"),
        center: Pt::new().class("c"),
        end: Pt::new().class("e").style("margin: 0"),
    };
    let html = view! { <Toolbar pt=pt start_template=slot("s") center_template=slot("c") end_template=slot("e") /> }.to_html();
    has(
        &html,
        &["p-toolbar p-component r", r#"data-testid="root""#, "p-toolbar-start s", "p-toolbar-center c", "p-toolbar-end e", "margin: 0"],
    );
}

#[test]
fn static_and_reactive_branches_render_the_same() {
    let fixed = view! { <Toolbar aria_labelledby="t" style="color: red" style_class="m" start_template=slot("s") /> }.to_html();
    let live = view! {
        <Toolbar
            aria_labelledby=RwSignal::new(Some("t".to_owned()))
            style=RwSignal::new(Some("color: red".to_owned()))
            style_class=RwSignal::new(Some("m".to_owned()))
            start_template=slot("s")
        />
    }
    .to_html();
    let norm = |s: &str| s.replace("<!>", "").replace("<!---->", "");
    assert_eq!(norm(&fixed), norm(&live), "{fixed}\n{live}");
}

#[test]
fn reactive_class_updates() {
    let owner = Owner::new();
    owner.with(|| {
        let class = RwSignal::new(Some("a".to_owned()));
        let html = view! { <Toolbar style_class=class /> }.to_html();
        has(&html, &["p-toolbar p-component a"]);
        class.set(Some("b".to_owned()));
        let html = view! { <Toolbar style_class=class /> }.to_html();
        has(&html, &["p-toolbar p-component b"]);
        lacks(&html, &["p-toolbar p-component a"]);
    });
}
