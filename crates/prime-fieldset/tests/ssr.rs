//! Структура DOM, классы, ARIA, слоты, `pt` и паритет ветвей Fieldset (SSR).
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_core::Pt;
use prime_fieldset::{Fieldset, FieldsetPt};

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
fn simple_fieldset_structure() {
    let html = view! { <Fieldset legend="Header"><p>"Body"</p></Fieldset> }.to_html();
    assert!(html.starts_with("<fieldset"), "{html}");
    has(
        &html,
        &[
            "p-fieldset p-component",
            "<legend",
            "p-fieldset-legend",
            "p-fieldset-legend-label",
            ">Header<",
            "<p>Body</p>",
            r#"role="region""#,
        ],
    );
    lacks(
        &html,
        &[
            "p-fieldset-toggleable",
            "<button",
            "p-fieldset-toggle-button",
            "aria-expanded",
            "aria-hidden",
            "aria-labelledby",
            "display: none",
            "data-p=",
        ],
    );
    let (legend, container, wrapper, content) = (
        pos(&html, "<legend"),
        pos(&html, "p-fieldset-content-container"),
        pos(&html, "p-fieldset-content-wrapper"),
        pos(&html, "p-fieldset-content\""),
    );
    assert!(legend < container && container < wrapper && wrapper < content, "{html}");
}

#[test]
fn toggleable_classes_and_aria() {
    let expanded = view! { <Fieldset id="f1" legend="L" toggleable=true collapsed=RwSignal::new(false)>"x"</Fieldset> }.to_html();
    has(
        &expanded,
        &[
            "p-fieldset-toggleable",
            r#"data-p="toggleable""#,
            "<button",
            "p-fieldset-toggle-button",
            r#"type="button""#,
            r#"id="f1_header""#,
            r#"aria-expanded="true""#,
            r#"aria-controls="f1_content""#,
            r#"aria-label="L""#,
            r#"id="f1_content""#,
            r#"aria-labelledby="f1_header""#,
            r#"aria-hidden="false""#,
            "p-fieldset-toggle-icon",
        ],
    );
    lacks(&expanded, &["p-fieldset-collapsed", "display: none", "tabindex"]);
    assert_eq!(expanded.matches(r#"data-p="toggleable""#).count(), 2, "корень и легенда: {expanded}");

    let collapsed = view! { <Fieldset id="f1" legend="L" toggleable=true collapsed=RwSignal::new(true)>"x"</Fieldset> }.to_html();
    has(&collapsed, &["p-fieldset-collapsed", r#"aria-expanded="false""#, r#"aria-hidden="true""#, r#"tabindex="-1""#, "display: none"]);
}

#[test]
fn button_wraps_icon_label_and_slot_in_order() {
    let html = view! {
        <Fieldset legend="L" toggleable=true header_template=ViewFn::from(|| view! { <i class="hdr">"h"</i> })>
            "x"
        </Fieldset>
    }
    .to_html();
    let (legend, button, icon, label, hdr, end) = (
        pos(&html, "<legend"),
        pos(&html, "<button"),
        pos(&html, "p-fieldset-toggle-icon"),
        pos(&html, "p-fieldset-legend-label"),
        pos(&html, "class=\"hdr\""),
        pos(&html, "</button>"),
    );
    assert!(legend < button && button < icon && icon < label && label < hdr && hdr < end, "{html}");
}

#[test]
fn default_collapsed_without_signal() {
    let html = view! { <Fieldset legend="L" toggleable=true default_collapsed=true>"x"</Fieldset> }.to_html();
    has(&html, &["p-fieldset-collapsed", r#"aria-expanded="false""#, "display: none"]);
    let plain = view! { <Fieldset legend="L" default_collapsed=true>"x"</Fieldset> }.to_html();
    lacks(&plain, &["p-fieldset-collapsed", "display: none", "aria-hidden"]);
}

#[test]
fn plus_and_minus_differ() {
    let closed = view! { <Fieldset legend="L" toggleable=true default_collapsed=true>"x"</Fieldset> }.to_html();
    let open = view! { <Fieldset legend="L" toggleable=true>"x"</Fieldset> }.to_html();
    let icon = |s: &str| s.split("<svg").nth(1).and_then(|r| r.split("</svg>").next()).map(str::to_owned);
    assert!(icon(&closed).is_some() && icon(&open).is_some());
    assert_ne!(icon(&closed), icon(&open), "плюс и минус различаются");
}

#[test]
fn icon_templates_replace_icons_by_state() {
    let expand = ViewFn::from(|| view! { <u class="plus">"+"</u> });
    let collapse = ViewFn::from(|| view! { <u class="minus">"-"</u> });
    let closed = view! {
        <Fieldset legend="L" toggleable=true default_collapsed=true expand_icon_template=expand.clone() collapse_icon_template=collapse.clone()>
            "x"
        </Fieldset>
    }
    .to_html();
    has(&closed, &["class=\"plus\""]);
    lacks(&closed, &["class=\"minus\"", "<svg"]);
    let open = view! {
        <Fieldset legend="L" toggleable=true expand_icon_template=expand collapse_icon_template=collapse>
            "x"
        </Fieldset>
    }
    .to_html();
    has(&open, &["class=\"minus\"", "<span class=\"p-fieldset-toggle-icon\""]);
    lacks(&open, &["class=\"plus\"", "<svg"]);
}

#[test]
fn legend_without_toggle_has_label_and_slot() {
    let html = view! { <Fieldset legend="L" header_template=ViewFn::from(|| view! { <i class="hdr">"h"</i> })>"x"</Fieldset> }.to_html();
    let (label, hdr) = (pos(&html, "p-fieldset-legend-label"), pos(&html, "class=\"hdr\""));
    assert!(label < hdr, "{html}");
}

#[test]
fn style_and_style_class_are_merged() {
    let html = view! { <Fieldset legend="L" style="color: red" style_class="mine">"x"</Fieldset> }.to_html();
    has(&html, &["p-fieldset p-component mine", "color: red"]);
}

#[test]
fn pt_parts() {
    let pt = FieldsetPt {
        root: Pt::new().class("r").attr("data-testid", "root"),
        legend: Pt::new().class("l"),
        legend_label: Pt::new().class("ll"),
        toggle_button: Pt::new().class("b"),
        toggle_icon: Pt::new().class("i"),
        content_container: Pt::new().class("cc"),
        content_wrapper: Pt::new().class("cw"),
        content: Pt::new().class("c"),
    };
    let html = view! {
        <Fieldset legend="L" toggleable=true pt=pt>
            "x"
        </Fieldset>
    }
    .to_html();
    has(
        &html,
        &[
            "p-fieldset p-component p-fieldset-toggleable r",
            r#"data-testid="root""#,
            "p-fieldset-legend l",
            "p-fieldset-legend-label ll",
            "p-fieldset-toggle-button b",
            "p-fieldset-toggle-icon i",
            "p-fieldset-content-container cc",
            "p-fieldset-content-wrapper cw",
            "p-fieldset-content c",
        ],
    );
}

#[test]
fn static_and_reactive_branches_render_the_same() {
    let fixed = view! { <Fieldset id="a" legend="L" style_class="m">"body"</Fieldset> }.to_html();
    let live = view! {
        <Fieldset id="a" legend=RwSignal::new(Some("L".to_owned())) style_class=RwSignal::new(Some("m".to_owned()))>
            "body"
        </Fieldset>
    }
    .to_html();
    let norm = |s: &str| s.replace("<!>", "").replace("<!---->", "");
    assert_eq!(norm(&fixed), norm(&live), "{fixed}\n{live}");
}
