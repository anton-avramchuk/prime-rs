//! Разметка Menu (SSR): список, группы, разделители, невидимые пункты, всплывающий режим.
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_core::Pt;
use prime_menu::{Menu, MenuItem, MenuPt, MenuRef};

fn render(f: impl FnOnce() -> String) -> String {
    Owner::new().with(f)
}

#[test]
fn flat_list_markup() {
    let html = render(|| {
        view! { <Menu id="m" model=vec![MenuItem::new().label("New"), MenuItem::new().label("Search").url("/s").target("_blank").title("T")] /> }.to_html()
    });
    assert!(html.contains("p-menu p-component"), "{html}");
    assert!(html.contains("position: relative;"), "{html}");
    assert!(html.contains("role=\"menu\"") && html.contains("tabindex=\"0\""), "{html}");
    assert_eq!(html.matches("role=\"menuitem\"").count(), 2, "{html}");
    assert!(html.contains("id=\"m_0\"") && html.contains("id=\"m_1\""), "{html}");
    assert!(html.contains("p-menu-item-label") && html.contains("New") && html.contains("Search"), "{html}");
    assert!(html.contains("href=\"/s\"") && html.contains("target=\"_blank\"") && html.contains("title=\"T\""), "{html}");
    assert!(html.contains("aria-label=\"New\"") && html.contains("data-p-disabled=\"false\""), "{html}");
    assert!(!html.contains("p-menu-overlay") && !html.contains("data-p=\"overlay\""), "{html}");
}

#[test]
fn groups_separators_and_hidden() {
    let html = render(|| {
        let model = vec![
            MenuItem::new().label("Documents").items(vec![MenuItem::new().label("New"), MenuItem::new().label("Gone").visible(false)]),
            MenuItem::separator(),
            MenuItem::new().label("Profile").items(vec![MenuItem::new().label("Settings").disabled(true)]),
        ];
        view! { <Menu model=model /> }.to_html()
    });
    assert_eq!(html.matches("p-menu-submenu-label").count(), 2, "{html}");
    assert_eq!(html.matches("role=\"separator\"").count(), 1, "{html}");
    assert_eq!(html.matches("role=\"menuitem\"").count(), 2, "{html}");
    assert!(!html.contains("Gone"), "{html}");
    assert!(html.contains("p-disabled") && html.contains("aria-disabled=\"true\""), "{html}");
}

#[test]
fn icon_classes_and_templates() {
    let html = render(|| {
        let model = vec![
            MenuItem::new()
                .label("A")
                .icon(|| view! { <i class="ic"></i> })
                .icon_class("ix")
                .link_class("lx")
                .style_class("sx")
                .style("color: red;"),
        ];
        view! {
            <Menu
                model=model
                style_class="custom"
                style="width: 5px;"
                aria_label="Nav"
                start=|| view! { <b>"S"</b> }
                end=|| view! { <b>"E"</b> }
            />
        }
        .to_html()
    });
    assert!(html.contains("p-menu-item-icon ix") && html.contains("class=\"ic\""), "{html}");
    assert!(html.contains("p-menu-item-link lx") && html.contains("p-menu-item sx") && html.contains("color: red;"), "{html}");
    assert!(html.contains("p-component custom") && html.contains("width: 5px;") && html.contains("aria-label=\"Nav\""), "{html}");
    assert!(html.contains("p-menu-start") && html.contains("p-menu-end"), "{html}");
}

#[test]
fn item_and_header_templates_replace_content() {
    let html = render(|| {
        let model = vec![MenuItem::new().label("G").items(vec![MenuItem::new().label("I")])];
        view! {
            <Menu
                model=model
                item=Callback::new(|i: MenuItem| view! { <em class="tpl">{i.label}</em> }.into_any())
                submenu_header=Callback::new(|i: MenuItem| view! { <u class="hdr">{i.label}</u> }.into_any())
            />
        }
        .to_html()
    });
    assert!(html.contains("class=\"tpl\"") && !html.contains("p-menu-item-link"), "{html}");
    assert!(html.contains("class=\"hdr\""), "{html}");
}

#[test]
fn popup_is_not_rendered_while_hidden() {
    let html = render(|| view! { <Menu model=vec![MenuItem::new().label("New")] popup=true menu_ref=MenuRef::new() /> }.to_html());
    assert!(!html.contains("p-menu") && !html.contains("New"), "{html}");
}

#[test]
fn pt_applies_to_parts() {
    let html = render(|| {
        let pt =
            MenuPt { root: Pt::new().class("r1"), list: Pt::new().class("l1"), item_label: Pt::new().class("b1"), ..Default::default() };
        view! { <Menu model=vec![MenuItem::new().label("A")] pt=pt /> }.to_html()
    });
    assert!(html.contains("r1") && html.contains("l1") && html.contains("b1"), "{html}");
}
