//! Структура DOM, классы, `style`, `pt` и паритет ветвей IftaLabel (SSR).
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_core::Pt;
use prime_iftalabel::{IftaLabel, IftaLabelPt};

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
    let html = view! { <IftaLabel><input class="first" /><label class="second" /></IftaLabel> }.to_html();
    assert!(html.starts_with("<span"), "{html}");
    has(&html, &[r#"class="p-iftalabel""#]);
    assert!(pos(&html, "first") < pos(&html, "second"), "{html}");
}

#[test]
fn empty_renders_root_only() {
    let html = view! { <IftaLabel /> }.to_html();
    has(&html, &[r#"class="p-iftalabel""#]);
    lacks(&html, &["style="]);
}

#[test]
fn style_and_class() {
    let html = view! { <IftaLabel style="width: 20rem" style_class="mine" /> }.to_html();
    has(&html, &[r#"class="p-iftalabel mine""#, r#"style="width: 20rem;""#]);
}

#[test]
fn pt_root_adds_without_replacing() {
    let pt = IftaLabelPt { root: Pt::new().class("custom").attr("data-x", "1").style("margin: 0") };
    let html = view! { <IftaLabel style_class="mine" pt=pt /> }.to_html();
    has(&html, &[r#"class="p-iftalabel mine custom""#, r#"data-x="1""#, "margin: 0"]);
}

#[test]
fn static_and_reactive_branches_render_the_same() {
    let fixed = view! { <IftaLabel style="color: red" style_class="m">"x"</IftaLabel> }.to_html();
    let live = view! {
        <IftaLabel style=RwSignal::new(Some("color: red".to_owned())) style_class=RwSignal::new(Some("m".to_owned()))>"x"</IftaLabel>
    }
    .to_html();
    assert_eq!(norm(&fixed), norm(&live), "{fixed}\n{live}");
}

#[test]
fn field_and_label_stay_inside() {
    let html = view! { <IftaLabel><input id="u" class="p-inputtext p-invalid" /><label for="u">"Имя"</label></IftaLabel> }.to_html();
    has(&html, &[r#"for="u""#, "p-invalid", "Имя"]);
    assert!(pos(&html, "<input") < pos(&html, "<label"), "{html}");
    assert_eq!(html.matches("p-iftalabel").count(), 1, "{html}");
}
