//! Структура DOM, ARIA полос, слот содержимого, `style`, `pt` и паритет ветвей ScrollPanel (SSR).
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_core::Pt;
use prime_scrollpanel::{ScrollPanel, ScrollPanelPt, ScrollPanelRef};

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

/// Идентификатор содержимого из `aria-controls` первой полосы.
fn controls(html: &str) -> String {
    let start = pos(html, "aria-controls=\"") + "aria-controls=\"".len();
    html[start..].split('"').next().unwrap().to_owned()
}

/// Убирает маркеры гидрации и номера счётчика идентификаторов.
fn norm(html: &str) -> String {
    let html = html.replace("<!>", "").replace("<!---->", "");
    let mut out = String::new();
    let mut rest = html.as_str();
    while let Some(i) = rest.find("pn_id_") {
        out.push_str(&rest[..i + "pn_id_".len()]);
        rest = rest[i + "pn_id_".len()..].trim_start_matches(|c: char| c.is_ascii_digit());
    }
    out + rest
}

#[test]
fn structure_and_order() {
    let html = view! { <ScrollPanel>"тело"</ScrollPanel> }.to_html();
    assert!(html.starts_with("<div"), "{html}");
    has(&html, &["p-scrollpanel p-component", "p-scrollpanel-content-container", "p-scrollpanel-content", "тело"]);
    let (container, content, bar_x, bar_y) = (
        pos(&html, "p-scrollpanel-content-container"),
        pos(&html, "p-scrollpanel-content\""),
        pos(&html, "p-scrollpanel-bar-x"),
        pos(&html, "p-scrollpanel-bar-y"),
    );
    assert!(container < content && content < bar_x && bar_x < bar_y, "{html}");
    has(&html, &["p-scrollpanel-bar p-scrollpanel-bar-x", "p-scrollpanel-bar p-scrollpanel-bar-y"]);
    lacks(&html, &["style="]);
}

#[test]
fn scrollbars_are_accessible() {
    let html = view! { <ScrollPanel /> }.to_html();
    assert_eq!(html.matches(r#"role="scrollbar""#).count(), 2, "{html}");
    assert_eq!(html.matches(r#"tabindex="0""#).count(), 2, "{html}");
    assert_eq!(html.matches(r#"data-pc-group-section="bar""#).count(), 2, "{html}");
    has(&html, &[r#"aria-orientation="horizontal""#, r#"aria-orientation="vertical""#, r#"aria-valuenow="0""#]);
    let id = controls(&html);
    assert!(id.starts_with("pn_id_") && id.ends_with("_content"), "{id}");
    assert_eq!(html.matches(&format!("aria-controls=\"{id}\"")).count(), 2, "{html}");
    has(&html, &[&format!("id=\"{id}\"")]);
}

#[test]
fn content_ids_are_unique() {
    let (a, b) = (view! { <ScrollPanel /> }.to_html(), view! { <ScrollPanel /> }.to_html());
    assert_ne!(controls(&a), controls(&b));
}

#[test]
fn content_template_replaces_children() {
    let slot = ViewFn::from(|| view! { <i>"слот"</i> });
    let html = view! { <ScrollPanel content_template=slot>"дети"</ScrollPanel> }.to_html();
    has(&html, &["<i>слот</i>"]);
    lacks(&html, &["дети"]);
}

#[test]
fn style_and_style_class_are_applied() {
    let html = view! { <ScrollPanel style="width: 100%; height: 150px" style_class="custombar" /> }.to_html();
    has(&html, &["p-scrollpanel p-component custombar", "width: 100%; height: 150px"]);
}

#[test]
fn pt_parts_extend_own_classes_and_attrs() {
    let pt = ScrollPanelPt {
        root: Pt::new().class("r").attr("data-x", "1"),
        content_container: Pt::new().class("cc"),
        content: Pt::new().class("c"),
        bar_x: Pt::new().class("bx"),
        bar_y: Pt::new().class("by").style("opacity: 1"),
    };
    let html = view! { <ScrollPanel pt=pt /> }.to_html();
    has(
        &html,
        &[
            "p-scrollpanel p-component r",
            r#"data-x="1""#,
            "p-scrollpanel-content-container cc",
            "p-scrollpanel-content c",
            "p-scrollpanel-bar p-scrollpanel-bar-x bx",
            "p-scrollpanel-bar p-scrollpanel-bar-y by",
            "opacity: 1",
        ],
    );
}

#[test]
fn static_and_reactive_branches_render_the_same() {
    let fixed = view! { <ScrollPanel style="height: 10px" style_class="m">"т"</ScrollPanel> }.to_html();
    let live = view! {
        <ScrollPanel style=RwSignal::new(Some("height: 10px".to_owned())) style_class=RwSignal::new(Some("m".to_owned()))>
            "т"
        </ScrollPanel>
    }
    .to_html();
    assert_eq!(norm(&fixed), norm(&live), "{fixed}\n{live}");
}

#[test]
fn reactive_class_updates() {
    let owner = Owner::new();
    owner.with(|| {
        let class = RwSignal::new(Some("a".to_owned()));
        let html = view! { <ScrollPanel style_class=class /> }.to_html();
        has(&html, &["p-scrollpanel p-component a"]);
        class.set(Some("b".to_owned()));
        let html = view! { <ScrollPanel style_class=class /> }.to_html();
        has(&html, &["p-scrollpanel p-component b"]);
        lacks(&html, &["p-scrollpanel p-component a"]);
    });
}

#[test]
fn panel_ref_methods_are_noops_without_a_browser() {
    Owner::new().with(|| {
        let panel_ref = ScrollPanelRef::new();
        let html = view! { <ScrollPanel panel_ref=panel_ref /> }.to_html();
        has(&html, &["p-scrollpanel"]);
        panel_ref.refresh();
        panel_ref.scroll_top(100.0);
    });
}
