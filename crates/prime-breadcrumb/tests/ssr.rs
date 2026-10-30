//! Структура DOM, пункты, разделители, шаблоны и `pt` Breadcrumb (SSR).
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_breadcrumb::{Breadcrumb, BreadcrumbItem, BreadcrumbPt};
use prime_core::Pt;

fn strip(s: &str) -> String {
    let mut out = s.replace("<!>", "");
    while let Some(start) = out.find("<!--") {
        let end = out[start..].find("-->").map_or(out.len(), |e| start + e + 3);
        out.replace_range(start..end, "");
    }
    out
}

fn model() -> Vec<BreadcrumbItem> {
    ["Electronics", "Computer", "Accessories"].into_iter().map(|l| BreadcrumbItem::new().label(l)).collect()
}

fn count(html: &str, needle: &str) -> usize {
    html.matches(needle).count()
}

#[test]
fn empty_root() {
    let html = strip(&view! { <Breadcrumb /> }.to_html());
    assert!(html.starts_with("<nav"), "{html}");
    assert!(html.contains("class=\"p-breadcrumb p-component\"") && html.contains("<ol class=\"p-breadcrumb-list\"></ol>"), "{html}");
}

#[test]
fn class_and_style() {
    let html = strip(&view! { <Breadcrumb style_class="mine" style="width: 20rem" /> }.to_html());
    assert!(html.contains("p-breadcrumb p-component mine") && html.contains("width: 20rem;"), "{html}");
}

#[test]
fn pt_root_and_list() {
    let pt = BreadcrumbPt { root: Pt::new().attr("data-x", "1"), list: Pt::new().class("custom"), ..Default::default() };
    let html = strip(&view! { <Breadcrumb pt=pt /> }.to_html());
    assert!(html.contains("data-x=\"1\"") && html.contains("p-breadcrumb-list custom"), "{html}");
}

#[test]
fn items_in_order_with_labels() {
    let html = strip(&view! { <Breadcrumb model=model() /> }.to_html());
    assert_eq!(count(&html, "class=\"p-breadcrumb-item\""), 3, "{html}");
    assert!(html.find("Electronics").unwrap() < html.find("Computer").unwrap(), "{html}");
    assert!(html.contains("<span class=\"p-breadcrumb-item-label\">Accessories</span>"), "{html}");
}

#[test]
fn link_attributes() {
    let item = BreadcrumbItem::new().label("A").url("/a").target("_blank").title("T");
    let html = strip(&view! { <Breadcrumb model={vec![item]} /> }.to_html());
    for needle in ["href=\"/a\"", "target=\"_blank\"", "title=\"T\"", "tabindex=\"0\"", "p-breadcrumb-item-link"] {
        assert!(html.contains(needle), "{needle}: {html}");
    }
}

#[test]
fn item_without_url_has_no_href() {
    let html = strip(&view! { <Breadcrumb model=model() /> }.to_html());
    assert!(!html.contains("href"), "{html}");
}

#[test]
fn hidden_item_and_its_separator_are_skipped() {
    let mut m = model();
    m[2] = m[2].clone().visible(false);
    let html = strip(&view! { <Breadcrumb model=m /> }.to_html());
    assert_eq!(count(&html, "class=\"p-breadcrumb-item\""), 2, "{html}");
    assert!(!html.contains("Accessories"), "{html}");
    assert_eq!(count(&html, "p-breadcrumb-separator"), 1, "{html}");
}

#[test]
fn disabled_item() {
    let m = vec![BreadcrumbItem::new().label("A").disabled(true)];
    let html = strip(&view! { <Breadcrumb model=m /> }.to_html());
    assert!(html.contains("p-breadcrumb-item p-disabled"), "{html}");
    assert!(!html.contains("tabindex"), "{html}");
}

#[test]
fn icon_label_and_badge_order() {
    let m = vec![BreadcrumbItem::new().label("A").icon(ViewFn::from(|| view! { <b>"i"</b> })).badge("3")];
    let html = strip(&view! { <Breadcrumb model=m /> }.to_html());
    let (i, l, b) =
        (html.find("p-breadcrumb-item-icon").unwrap(), html.find("p-breadcrumb-item-label").unwrap(), html.find("p-badge").unwrap());
    assert!(i < l && l < b, "{html}");
    assert!(html.contains(">3</span>"), "{html}");
}

#[test]
fn home_first_with_default_icon_and_aria_label() {
    let html = strip(&view! { <Breadcrumb model=model() home=BreadcrumbItem::new() home_aria_label="Home" /> }.to_html());
    let home = html.find("p-breadcrumb-home-item").unwrap();
    assert!(home < html.find("p-breadcrumb-item\"").unwrap(), "{html}");
    assert!(html.contains("aria-label=\"Home\""), "{html}");
    assert!(html.contains("p-icon p-breadcrumb-item-icon"), "{html}");
}

#[test]
fn separator_counts() {
    let with_home = strip(&view! { <Breadcrumb model=model() home=BreadcrumbItem::new() /> }.to_html());
    assert_eq!(count(&with_home, "class=\"p-breadcrumb-separator\""), 3, "{with_home}");
    let without = strip(&view! { <Breadcrumb model=model() /> }.to_html());
    assert_eq!(count(&without, "class=\"p-breadcrumb-separator\""), 2, "{without}");
    let only_home = strip(&view! { <Breadcrumb home=BreadcrumbItem::new() /> }.to_html());
    assert_eq!(count(&only_home, "p-breadcrumb-separator"), 0, "{only_home}");
}

#[test]
fn default_separator_is_chevron_and_template_replaces_it() {
    let default = strip(&view! { <Breadcrumb model=model() /> }.to_html());
    assert!(default.contains("<svg"), "{default}");
    let custom = strip(&view! { <Breadcrumb model=model() separator=ViewFn::from(|| view! { "/" }) /> }.to_html());
    assert!(custom.contains("class=\"p-breadcrumb-separator\">/</li>"), "{custom}");
    assert!(!custom.contains("<svg"), "{custom}");
}

#[test]
fn item_template_replaces_link_but_keeps_li() {
    let tpl = Callback::new(|i: BreadcrumbItem| view! { <b>{i.label.unwrap_or_default()}</b> }.into_any());
    let html = strip(&view! { <Breadcrumb model=model() item=tpl /> }.to_html());
    assert_eq!(count(&html, "<b>"), 3, "{html}");
    assert!(!html.contains("p-breadcrumb-item-link"), "{html}");
    assert_eq!(count(&html, "class=\"p-breadcrumb-item\""), 3, "{html}");
}
