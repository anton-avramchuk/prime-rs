//! Структура DOM, варианты, `style`, `pt` и паритет ветвей FloatLabel (SSR).
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_core::ClassNames;
use prime_core::Pt;
use prime_floatlabel::{FloatLabel, FloatLabelPt, FloatLabelVariant, write_classes};

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
    let html = view! { <FloatLabel><input class="first" /><label class="second" /></FloatLabel> }.to_html();
    assert!(html.starts_with("<span"), "{html}");
    has(&html, &[r#"class="p-floatlabel p-floatlabel-over""#]);
    assert!(pos(&html, "first") < pos(&html, "second"), "{html}");
}

#[test]
fn empty_renders_root_only() {
    let html = view! { <FloatLabel /> }.to_html();
    has(&html, &[r#"class="p-floatlabel p-floatlabel-over""#]);
    lacks(&html, &["style="]);
}

#[test]
fn style_and_class() {
    let html = view! { <FloatLabel style="width: 20rem" style_class="mine" /> }.to_html();
    has(&html, &[r#"class="p-floatlabel p-floatlabel-over mine""#, r#"style="width: 20rem;""#]);
}

#[test]
fn pt_root_adds_without_replacing() {
    let pt = FloatLabelPt { root: Pt::new().class("custom").attr("data-x", "1").style("margin: 0") };
    let html = view! { <FloatLabel style_class="mine" pt=pt /> }.to_html();
    has(&html, &[r#"class="p-floatlabel p-floatlabel-over mine custom""#, r#"data-x="1""#, "margin: 0"]);
}

#[test]
fn static_and_reactive_branches_render_the_same() {
    let fixed = view! { <FloatLabel style="color: red" style_class="m">"x"</FloatLabel> }.to_html();
    let live = view! {
        <FloatLabel style=RwSignal::new(Some("color: red".to_owned())) style_class=RwSignal::new(Some("m".to_owned()))>"x"</FloatLabel>
    }
    .to_html();
    assert_eq!(norm(&fixed), norm(&live), "{fixed}\n{live}");
}

#[test]
fn write_classes_table() {
    for (variant, want) in [
        (FloatLabelVariant::Over, "p-floatlabel p-floatlabel-over"),
        (FloatLabelVariant::On, "p-floatlabel p-floatlabel-on"),
        (FloatLabelVariant::In, "p-floatlabel p-floatlabel-in"),
    ] {
        let mut cn = ClassNames::new();
        write_classes(&mut cn, variant);
        assert_eq!(cn.finish_cow(), want);
    }
    assert_eq!(FloatLabelVariant::default(), FloatLabelVariant::Over);
}

#[test]
fn variants_have_exactly_one_variant_class() {
    for (variant, want, others) in [
        (FloatLabelVariant::In, "p-floatlabel-in", ["p-floatlabel-over", "p-floatlabel-on"]),
        (FloatLabelVariant::On, "p-floatlabel-on", ["p-floatlabel-over", "p-floatlabel-in"]),
        (FloatLabelVariant::Over, "p-floatlabel-over", ["p-floatlabel-on", "p-floatlabel-in"]),
    ] {
        let html = view! { <FloatLabel variant=variant /> }.to_html();
        has(&html, &[want]);
        lacks(&html, &others);
    }
}

#[test]
fn variant_with_style_class_and_pt_keeps_order() {
    let pt = FloatLabelPt { root: Pt::new().class("custom") };
    let html = view! { <FloatLabel variant=FloatLabelVariant::On style_class="mine" pt=pt /> }.to_html();
    has(&html, &[r#"class="p-floatlabel p-floatlabel-on mine custom""#]);
}

#[test]
fn reactive_variant_replaces_class() {
    let variant = RwSignal::new(FloatLabelVariant::Over);
    let first = view! { <FloatLabel variant=variant /> }.to_html();
    has(&first, &["p-floatlabel-over"]);
    variant.set(FloatLabelVariant::On);
    let second = view! { <FloatLabel variant=variant /> }.to_html();
    has(&second, &["p-floatlabel-on"]);
    lacks(&second, &["p-floatlabel-over"]);
}

#[test]
fn field_and_label_stay_inside() {
    let html = view! { <FloatLabel><input id="u" class="p-inputtext" /><label for="u">"Имя"</label></FloatLabel> }.to_html();
    assert!(pos(&html, "<input") < pos(&html, "<label"), "{html}");
}
