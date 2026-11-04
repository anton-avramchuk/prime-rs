//! Разметка ContextMenu (SSR): скрыт до показа, корень, пункты, группы, разделители, `aria-*`, шаблоны, `pt`.
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_contextmenu::{ContextMenu, ContextMenuItemContext, ContextMenuPt, ContextMenuRef};
use prime_core::Pt;
use prime_menu::MenuItem;

fn leaf(label: &str) -> MenuItem {
    MenuItem::new().label(label)
}

fn model() -> Vec<MenuItem> {
    vec![
        leaf("File").items(vec![leaf("New").items(vec![leaf("Share")]), leaf("Open")]),
        leaf("Hidden").visible(false),
        MenuItem::separator(),
        leaf("Users").disabled(true).url("/users").target("_blank").title("All users"),
        leaf("Quit").style_class("quit").link_class("lnk").icon_class("ico"),
    ]
}

/// Рендерит меню, показанное в точке (10, 20).
fn shown(f: impl FnOnce(ContextMenuRef) -> String) -> String {
    Owner::new().with(|| {
        let r = ContextMenuRef::new();
        r.show_at(10.0, 20.0);
        f(r)
    })
}

#[test]
fn nothing_is_rendered_until_shown() {
    let html = Owner::new().with(|| view! { <ContextMenu model=model() /> }.to_html());
    assert!(!html.contains("p-contextmenu"), "{html}");
    let hidden = Owner::new().with(|| {
        let r = ContextMenuRef::new();
        r.show_at(1.0, 2.0);
        r.hide();
        view! { <ContextMenu model=model() popup_ref=r /> }.to_html()
    });
    assert!(!hidden.contains("p-contextmenu"), "{hidden}");
}

#[test]
fn shown_markup_has_root_list_and_items() {
    let html = shown(|r| view! { <ContextMenu id="cm" model=model() popup_ref=r /> }.to_html());
    assert!(html.contains("id=\"cm\"") && html.contains("p-contextmenu p-component"), "{html}");
    assert!(!html.contains("p-contextmenu-mobile"), "{html}");
    assert!(
        html.contains("id=\"cm_list\"") && html.contains("class=\"p-contextmenu-root-list\"") && html.contains("role=\"menu\""),
        "{html}"
    );
    assert!(html.contains("tabindex=\"0\""), "{html}");
    assert_eq!(html.matches("role=\"menuitem\"").count(), 3, "File, Users, Quit: скрытый и разделитель не считаются: {html}");
    assert_eq!(html.matches("role=\"separator\"").count(), 1, "{html}");
    assert!(html.contains("id=\"cm_0\"") && html.contains("id=\"cm_3\"") && html.contains("id=\"cm_4\""), "{html}");
    assert!(!html.contains("Hidden"), "{html}");
    assert!(html.contains("position: absolute"), "панель вне потока: {html}");
    assert!(html.contains("visibility: hidden"), "до расчёта положения панель скрыта: {html}");
    assert!(!html.contains("p-tieredmenu"), "классы только contextmenu: {html}");
}

#[test]
fn groups_carry_aria_and_an_arrow_but_no_submenu_until_opened() {
    let html = shown(|r| view! { <ContextMenu id="cm" model=model() popup_ref=r /> }.to_html());
    assert!(html.contains("aria-haspopup=\"menu\"") && html.contains("aria-expanded=\"false\""), "{html}");
    assert_eq!(html.matches("aria-haspopup=\"menu\"").count(), 1, "только File — группа: {html}");
    assert!(html.contains("p-contextmenu-submenu-icon"), "{html}");
    assert!(!html.contains("p-contextmenu-submenu\"") && !html.contains("Share"), "подменю выводится только у раскрытой группы: {html}");
}

#[test]
fn items_have_state_attributes_and_positions() {
    let html = shown(|r| view! { <ContextMenu id="cm" model=model() popup_ref=r /> }.to_html());
    assert!(html.contains("aria-setsize=\"3\""), "{html}");
    assert!(html.contains("aria-posinset=\"1\"") && html.contains("aria-posinset=\"2\"") && html.contains("aria-posinset=\"3\""), "{html}");
    assert!(html.contains("data-p-highlight=\"false\"") && html.contains("data-p-focused=\"false\""), "{html}");
    assert!(
        html.contains("p-contextmenu-item p-disabled")
            && html.contains("data-p-disabled=\"true\"")
            && html.contains("aria-disabled=\"true\""),
        "{html}"
    );
    assert!(html.contains("aria-label=\"Users\"") && html.contains("aria-label=\"Quit\""), "{html}");
    assert!(html.contains("p-contextmenu-item quit"), "класс пункта: {html}");
}

#[test]
fn links_carry_attributes_icons_and_labels() {
    let model = vec![
        leaf("Users").url("/users").target("_blank").title("All users"),
        leaf("Quit").link_class("lnk").icon_class("ico").icon(ViewFn::from(|| view! { <i class="my-icon">"i"</i> })),
    ];
    let html = shown(|r| view! { <ContextMenu model=model popup_ref=r /> }.to_html());
    assert!(html.contains("href=\"/users\"") && html.contains("target=\"_blank\"") && html.contains("title=\"All users\""), "{html}");
    assert!(
        html.contains("p-contextmenu-item-link lnk") && html.contains("p-contextmenu-item-icon ico") && html.contains("my-icon"),
        "{html}"
    );
    assert!(html.contains("p-contextmenu-item-content") && html.contains("class=\"p-contextmenu-item-label\""), "{html}");
    assert_eq!(html.matches("p-contextmenu-item-icon").count(), 1, "значок только у пункта со значком: {html}");
}

#[test]
fn labels_pass_through_to_the_root_list() {
    let html = shown(|r| view! { <ContextMenu model=model() popup_ref=r aria_label="Main" aria_labelledby="lbl" /> }.to_html());
    assert!(html.contains("aria-label=\"Main\"") && html.contains("aria-labelledby=\"lbl\""), "{html}");
}

#[test]
fn item_template_and_submenu_icon_replace_defaults() {
    let html = shown(|r| {
        view! {
            <ContextMenu
                model=model()
                popup_ref=r
                item=Callback::new(|ctx: ContextMenuItemContext| view! { <b class="tpl">{ctx.item.label.unwrap_or_default()}</b> }.into_any())
                submenu_icon=ViewFn::from(|| view! { <i class="my-arrow">">"</i> })
            />
        }
        .to_html()
    });
    assert_eq!(html.matches("class=\"tpl\"").count(), 3, "{html}");
    assert!(!html.contains("p-contextmenu-item-link"), "{html}");
    let with_arrow = shown(|r| {
        view! { <ContextMenu model=model() popup_ref=r submenu_icon=ViewFn::from(|| view! { <i class="my-arrow">">"</i> }) /> }.to_html()
    });
    assert!(with_arrow.contains("my-arrow") && !with_arrow.contains("p-contextmenu-submenu-icon"), "{with_arrow}");
}

#[test]
fn style_class_style_and_pt() {
    let pt = ContextMenuPt {
        root: Pt::new().class("r").attr("data-x", "1"),
        root_list: Pt::new().class("l"),
        item: Pt::new().class("i"),
        item_content: Pt::new().class("c"),
        item_link: Pt::new().class("k"),
        item_label: Pt::new().class("t"),
        submenu_icon: Pt::new().class("s"),
        separator: Pt::new().class("sep"),
        ..ContextMenuPt::default()
    };
    let html = shown(|r| view! { <ContextMenu model=model() popup_ref=r style_class="mine" style="width: 10rem" pt=pt /> }.to_html());
    assert!(html.contains("p-contextmenu p-component mine r") && html.contains("data-x=\"1\"") && html.contains("width: 10rem"), "{html}");
    for needle in [
        "p-contextmenu-root-list l",
        "p-contextmenu-item i",
        "p-contextmenu-item-content c",
        "p-contextmenu-item-link k",
        "p-contextmenu-item-label t",
        "p-contextmenu-submenu-icon s",
        "p-contextmenu-separator sep",
    ] {
        assert!(html.contains(needle), "нет {needle}: {html}");
    }
}

#[test]
fn empty_model_renders_an_empty_list() {
    let html = shown(|r| view! { <ContextMenu popup_ref=r /> }.to_html());
    assert!(html.contains("p-contextmenu-root-list") && !html.contains("role=\"menuitem\""), "{html}");
}
