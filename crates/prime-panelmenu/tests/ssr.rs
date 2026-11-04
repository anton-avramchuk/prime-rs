//! Разметка PanelMenu (SSR): корень, панели, заголовки, области, деревья, `aria-*`, начальное раскрытие, шаблоны, `pt`.
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_core::Pt;
use prime_menu::MenuItem;
use prime_panelmenu::{PanelMenu, PanelMenuItemContext, PanelMenuPt, PanelMenuRef};

fn render(f: impl FnOnce() -> String) -> String {
    Owner::new().with(f)
}

fn leaf(label: &str) -> MenuItem {
    MenuItem::new().label(label)
}

fn model() -> Vec<MenuItem> {
    vec![
        leaf("Files").items(vec![
            leaf("Docs").items(vec![leaf("Invoices")]),
            MenuItem::separator(),
            leaf("Images"),
            leaf("Hidden").visible(false),
        ]),
        leaf("Skipped").visible(false),
        leaf("Cloud").disabled(true).items(vec![leaf("Upload")]),
        leaf("Quit").url("/quit").target("_blank").title("Leave").style_class("quit").link_class("lnk").icon_class("ico"),
    ]
}

#[test]
fn markup_has_panels_headers_regions_and_trees() {
    let html = render(|| view! { <PanelMenu id="pm" model=model() /> }.to_html());
    assert!(html.contains("id=\"pm\"") && html.contains("p-panelmenu p-component"), "{html}");
    assert_eq!(html.matches("class=\"p-panelmenu-panel\"").count(), 3, "скрытый корневой пункт не выводится: {html}");
    assert_eq!(html.matches("role=\"button\"").count(), 3, "{html}");
    assert!(html.contains("id=\"pm_0_header\"") && html.contains("id=\"pm_2_header\"") && html.contains("id=\"pm_3_header\""), "{html}");
    assert!(!html.contains("pm_1_header") && !html.contains("Skipped"), "{html}");
    assert!(
        html.contains("aria-controls=\"pm_0_content\"")
            && html.contains("role=\"region\"")
            && html.contains("aria-labelledby=\"pm_0_header\""),
        "{html}"
    );
    assert!(
        html.contains("p-panelmenu-header-content")
            && html.contains("p-panelmenu-header-link")
            && html.contains("p-panelmenu-header-label"),
        "{html}"
    );
    assert!(html.contains("p-panelmenu-content-wrapper") && html.contains("class=\"p-panelmenu-content\""), "{html}");
    assert_eq!(html.matches("class=\"p-panelmenu-root-list p-panelmenu-submenu\"").count(), 3, "{html}");
    assert!(html.contains("id=\"pm_0_list\"") && html.contains("role=\"tree\"") && html.contains("tabindex=\"-1\""), "{html}");
    assert!(html.contains("<svg") && html.contains("p-panelmenu-submenu-icon"), "{html}");
}

#[test]
fn collapsed_by_default_and_expanded_from_the_model() {
    let html = render(|| view! { <PanelMenu id="pm" model=model() /> }.to_html());
    assert!(!html.contains("p-panelmenu-expanded") && !html.contains("p-panelmenu-header-active"), "{html}");
    assert!(html.contains("aria-expanded=\"false\"") && !html.contains("aria-expanded=\"true\""), "{html}");
    assert!(html.contains("display: none"), "свёрнутая область скрыта: {html}");

    let mut open = model();
    open[0].expanded = true;
    let html = render(|| view! { <PanelMenu id="pm" model=open /> }.to_html());
    assert_eq!(html.matches("p-panelmenu-expanded").count(), 1, "{html}");
    assert!(html.contains("p-panelmenu-header p-panelmenu-header-active") && html.contains("data-p-highlight=\"true\""), "{html}");
    assert!(html.contains("aria-expanded=\"true\""), "{html}");
}

#[test]
fn items_carry_tree_aria_and_classes() {
    let mut m = model();
    m[0].expanded = true;
    m[0].items[0].expanded = true;
    let html = render(|| view! { <PanelMenu id="pm" model=m /> }.to_html());
    assert_eq!(
        html.matches("role=\"treeitem\"").count(),
        4,
        "Docs, Invoices, Images и Upload (панель Cloud): скрытый и разделитель не считаются: {html}"
    );
    assert_eq!(html.matches("role=\"separator\"").count(), 1, "{html}");
    assert!(html.contains("p-menuitem-separator"), "{html}");
    assert!(html.contains("id=\"pm_0_0\"") && html.contains("id=\"pm_0_0_0\"") && html.contains("id=\"pm_0_2\""), "{html}");
    assert!(!html.contains("Hidden"), "{html}");
    assert!(html.contains("aria-setsize=\"2\"") && html.contains("aria-posinset=\"2\""), "Docs/Images из двух: {html}");
    assert!(html.contains("aria-level=\"1\"") && html.contains("aria-level=\"2\""), "{html}");
    assert!(html.contains("class=\"p-panelmenu-item\"") && html.contains("data-p-focused=\"false\""), "{html}");
    assert!(
        html.contains("p-panelmenu-item-content") && html.contains("p-panelmenu-item-link") && html.contains("p-panelmenu-item-label"),
        "{html}"
    );
    assert!(html.contains("class=\"p-panelmenu-submenu\""), "вложенный список: {html}");
}

#[test]
fn nested_group_toggles_its_own_region() {
    let mut m = model();
    m[0].expanded = true;
    let html = render(|| view! { <PanelMenu id="pm" model=m /> }.to_html());
    assert!(html.contains("id=\"pm_0_0_list\""), "вложенный список выводится и в свёрнутом виде: {html}");
    assert_eq!(html.matches("p-panelmenu-expanded").count(), 1, "раскрыта только корневая панель: {html}");
}

#[test]
fn disabled_and_link_properties() {
    let html = render(|| view! { <PanelMenu id="pm" model=model() /> }.to_html());
    assert!(
        html.contains("p-panelmenu-header p-disabled")
            && html.contains("aria-disabled=\"true\"")
            && html.contains("data-p-disabled=\"true\""),
        "{html}"
    );
    assert!(html.contains("href=\"/quit\"") && html.contains("target=\"_blank\"") && html.contains("title=\"Leave\""), "{html}");
    assert!(html.contains("p-panelmenu-header quit") && html.contains("p-panelmenu-header-link lnk"), "{html}");
    assert!(!html.contains("p-panelmenu-header-icon"), "значок выводится только при icon: {html}");
}

#[test]
fn link_tabindex_follows_expansion() {
    let mut m = model();
    m[0].expanded = true;
    let html = render(|| view! { <PanelMenu id="pm" model=m /> }.to_html());
    assert!(html.contains("tabindex=\"0\""), "ссылки раскрытой панели доступны для Tab: {html}");
    let closed = render(|| view! { <PanelMenu id="pm" model=model() /> }.to_html());
    assert!(!closed.contains("p-panelmenu-item-link\" tabindex=\"0\""), "{closed}");
}

#[test]
fn root_style_class_and_pt() {
    let pt = PanelMenuPt {
        root: Pt::new().class("r").attr("data-x", "1"),
        header: Pt::new().class("h"),
        item: Pt::new().class("i"),
        ..PanelMenuPt::default()
    };
    let mut m = model();
    m[0].expanded = true;
    let html = render(|| view! { <PanelMenu id="pm" model=m style="width: 10rem" style_class="mine" pt=pt /> }.to_html());
    assert!(html.contains("p-panelmenu p-component mine r") || (html.contains("mine") && html.contains(" r")), "{html}");
    assert!(html.contains("data-x=\"1\"") && html.contains("width: 10rem"), "{html}");
    assert!(html.contains("p-panelmenu-header h") || html.contains("h p-panelmenu-header") || html.contains(" h\""), "{html}");
    assert!(html.contains("p-panelmenu-item i") || html.contains("i p-panelmenu-item") || html.contains(" i\""), "{html}");
}

#[test]
fn item_template_replaces_the_link() {
    let mut m = model();
    m[0].expanded = true;
    let item = Callback::new(|ctx: PanelMenuItemContext| {
        let tag = if ctx.root { "H" } else { "I" };
        let text = format!("{tag}{}", ctx.item.label.clone().unwrap_or_default());
        view! { <b class="custom">{text}</b> }.into_any()
    });
    let html = render(|| view! { <PanelMenu id="pm" model=m item=item /> }.to_html());
    assert!(!html.contains("p-panelmenu-header-link") && !html.contains("p-panelmenu-item-link"), "{html}");
    assert!(html.contains("HFiles") && html.contains("IImages"), "{html}");
}

#[test]
fn custom_chevrons() {
    let mut m = model();
    m[0].expanded = true;
    let html = render(|| {
        view! { <PanelMenu id="pm" model=m submenu_icon=|| view! { <i class="sub" /> } header_icon=|| view! { <i class="head" /> } /> }
            .to_html()
    });
    assert!(html.contains("class=\"sub\"") && html.contains("class=\"head\""), "{html}");
}

#[test]
fn panel_ref_expands_and_collapses() {
    Owner::new().with(|| {
        let r = PanelMenuRef::new();
        let html = view! { <PanelMenu id="pm" model=model() panel_ref=r /> }.to_html();
        assert!(!html.contains("p-panelmenu-expanded"));
        assert!(!r.all_expanded());
        r.expand_all();
        assert!(r.all_expanded() && r.is_expanded("0") && r.is_expanded("0_0") && r.is_expanded("2"));
        r.toggle_all();
        assert!(!r.is_expanded("0") && !r.all_expanded());
        r.toggle_all();
        assert!(r.is_expanded("0_0"));
        r.collapse_all();
        assert!(!r.is_expanded("0"));
    });
}

#[test]
fn model_expanded_seeds_the_ref() {
    Owner::new().with(|| {
        let r = PanelMenuRef::new();
        let mut m = model();
        m[0].expanded = true;
        let _ = view! { <PanelMenu id="pm" model=m panel_ref=r /> }.to_html();
        assert!(r.is_expanded("0") && !r.is_expanded("2"));
    });
}

#[test]
fn empty_model_renders_an_empty_root() {
    let html = render(|| view! { <PanelMenu /> }.to_html());
    assert!(html.contains("p-panelmenu p-component") && !html.contains("p-panelmenu-panel"), "{html}");
}

#[test]
fn leaf_header_has_no_chevron() {
    let html = render(|| view! { <PanelMenu id="pm" model=vec![leaf("Only")] /> }.to_html());
    assert!(html.contains("p-panelmenu-header-label") && !html.contains("<svg") && !html.contains("p-panelmenu-header-active"), "{html}");
}
