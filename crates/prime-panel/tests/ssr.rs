//! Структура DOM, классы, ARIA, слоты, `pt` и паритет ветвей Panel (SSR).
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_core::Pt;
use prime_panel::{IconPos, Panel, PanelPt};

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
fn simple_panel_structure() {
    let html = view! { <Panel header="Header"><p>"Body"</p></Panel> }.to_html();
    has(
        &html,
        &[
            "p-panel p-component",
            "p-panel-header",
            "p-panel-title",
            ">Header<",
            "p-panel-header-actions p-panel-icons-end",
            "p-panel-content",
        ],
    );
    lacks(
        &html,
        &["p-panel-toggleable", "p-panel-toggle-button", "p-panel-footer", "aria-expanded", "aria-hidden", "display: none", "data-p="],
    );
    let (header, container, wrapper, content) = (
        pos(&html, "p-panel-header\""),
        pos(&html, "p-panel-content-container"),
        pos(&html, "p-panel-content-wrapper"),
        pos(&html, "p-panel-content\""),
    );
    assert!(header < container && container < wrapper && wrapper < content, "{html}");
    has(&html, &[r#"role="region""#, "aria-labelledby=", "<p>Body</p>"]);
}

#[test]
fn toggleable_classes_and_aria() {
    let expanded = view! { <Panel id="p1" header="H" toggleable=true collapsed=RwSignal::new(false)>"x"</Panel> }.to_html();
    has(
        &expanded,
        &[
            "p-panel-toggleable p-panel-expanded",
            r#"data-p="toggleable""#,
            "p-panel-toggle-button",
            r#"aria-expanded="true""#,
            r#"aria-controls="p1_content""#,
            r#"aria-label="H""#,
            r#"id="p1_content""#,
            r#"aria-labelledby="p1_header""#,
            r#"aria-hidden="false""#,
            r#"id="p1_header""#,
            r#"id="p1-titlebar""#,
            r#"type="button""#,
        ],
    );
    lacks(&expanded, &["p-panel-collapsed", "display: none", "tabindex"]);

    let collapsed = view! { <Panel id="p1" header="H" toggleable=true collapsed=RwSignal::new(true)>"x"</Panel> }.to_html();
    has(
        &collapsed,
        &["p-panel-toggleable p-panel-collapsed", r#"aria-expanded="false""#, r#"aria-hidden="true""#, r#"tabindex="-1""#, "display: none"],
    );
    lacks(&collapsed, &["p-panel-expanded"]);
}

#[test]
fn default_collapsed_without_signal() {
    let html = view! { <Panel header="H" toggleable=true default_collapsed=true>"x"</Panel> }.to_html();
    has(&html, &["p-panel-collapsed", r#"aria-expanded="false""#, "display: none"]);
    let plain = view! { <Panel header="H" default_collapsed=true>"x"</Panel> }.to_html();
    lacks(&plain, &["p-panel-collapsed", "display: none", "aria-hidden"]);
}

#[test]
fn toggle_button_markup_matches_primeng() {
    let closed = view! { <Panel header="H" toggleable=true default_collapsed=true>"x"</Panel> }.to_html();
    has(
        &closed,
        &[
            "p-button p-component p-button-icon-only p-button-secondary p-button-rounded p-button-text p-panel-toggle-button",
            "p-button-icon",
            "<svg",
        ],
    );
    let open = view! { <Panel header="H" toggleable=true>"x"</Panel> }.to_html();
    let icon = |s: &str| s.split("p-button-icon\"").nth(1).and_then(|r| r.split("</button>").next()).map(str::to_owned);
    assert_ne!(icon(&closed), icon(&open), "плюс и минус различаются");
}

#[test]
fn button_takes_header_id_without_title() {
    let html = view! { <Panel id="q" toggleable=true>"x"</Panel> }.to_html();
    has(&html, &[r#"id="q_header""#, r#"aria-labelledby="q_header""#]);
    assert_eq!(html.matches(r#"id="q_header""#).count(), 1, "{html}");
    lacks(&html, &["p-panel-title"]);
}

#[test]
fn header_can_be_hidden() {
    let html = view! { <Panel show_header=false header="H">"body"</Panel> }.to_html();
    lacks(&html, &["p-panel-header", "p-panel-title"]);
    has(&html, &["p-panel-content", "body"]);
}

#[test]
fn icon_pos_classes() {
    for (p, class) in
        [(IconPos::Start, "p-panel-icons-start"), (IconPos::End, "p-panel-icons-end"), (IconPos::Center, "p-panel-icons-center")]
    {
        let html = view! { <Panel header="H" icon_pos=p>"x"</Panel> }.to_html();
        has(&html, &[&format!("p-panel-header-actions {class}")]);
    }
}

#[test]
fn slots_are_placed() {
    let html = view! {
        <Panel
            header="T"
            toggleable=true
            header_template=ViewFn::from(|| view! { <i class="hdr">"h"</i> })
            icons_template=ViewFn::from(|| view! { <i class="ico">"i"</i> })
            footer_template=ViewFn::from(|| view! { <b class="ftr">"f"</b> })
        >
            "body"
        </Panel>
    }
    .to_html();
    let (title, hdr, actions, ico, button, content, footer) = (
        pos(&html, "p-panel-title"),
        pos(&html, "class=\"hdr\""),
        pos(&html, "p-panel-header-actions"),
        pos(&html, "class=\"ico\""),
        pos(&html, "p-panel-toggle-button"),
        pos(&html, "p-panel-content\""),
        pos(&html, "p-panel-footer"),
    );
    assert!(title < hdr && hdr < actions && actions < ico && ico < button && button < content && content < footer, "{html}");
    has(&html, &["class=\"ftr\""]);
}

#[test]
fn header_icons_template_replaces_icon_and_gets_state() {
    let tpl = Callback::new(|collapsed: bool| view! { <u class="own">{if collapsed { "closed" } else { "open" }}</u> }.into_any());
    let html = view! { <Panel header="H" toggleable=true default_collapsed=true header_icons_template=tpl>"x"</Panel> }.to_html();
    has(&html, &["class=\"own\"", "closed"]);
    lacks(&html, &["<svg"]);
}

#[test]
fn style_and_style_class_are_merged() {
    let html = view! { <Panel header="H" style="color: red" style_class="mine">"x"</Panel> }.to_html();
    has(&html, &["p-panel p-component mine", "color: red"]);
}

#[test]
fn pt_parts() {
    let pt = PanelPt {
        root: Pt::new().class("r").attr("data-testid", "root"),
        header: Pt::new().class("h"),
        title: Pt::new().class("t"),
        header_actions: Pt::new().class("a"),
        toggle_button: Pt::new().class("b"),
        content_container: Pt::new().class("cc"),
        content_wrapper: Pt::new().class("cw"),
        content: Pt::new().class("c"),
        footer: Pt::new().class("f"),
    };
    let html = view! {
        <Panel header="H" toggleable=true pt=pt footer_template=ViewFn::from(|| view! { "f" })>
            "x"
        </Panel>
    }
    .to_html();
    has(
        &html,
        &[
            "p-panel p-component p-panel-toggleable p-panel-expanded r",
            r#"data-testid="root""#,
            "p-panel-header h",
            "p-panel-title t",
            "p-panel-icons-end a",
            "p-panel-toggle-button b",
            "p-panel-content-container cc",
            "p-panel-content-wrapper cw",
            "p-panel-content c",
            "p-panel-footer f",
        ],
    );
}

#[test]
fn static_and_reactive_branches_render_the_same() {
    let fixed = view! { <Panel id="a" header="H" style_class="m" icon_pos=IconPos::Start>"body"</Panel> }.to_html();
    let live = view! {
        <Panel
            id="a"
            header=RwSignal::new(Some("H".to_owned()))
            style_class=RwSignal::new(Some("m".to_owned()))
            icon_pos=RwSignal::new(IconPos::Start)
        >
            "body"
        </Panel>
    }
    .to_html();
    let norm = |s: &str| s.replace("<!>", "").replace("<!---->", "");
    assert_eq!(norm(&fixed), norm(&live), "{fixed}\n{live}");
}
