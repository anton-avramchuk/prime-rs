//! Разметка MegaMenu (SSR): корень, ориентации, слоты, кнопка, корневой список, пункты, группы, разделители, `aria-*`, шаблоны, `pt`.
//! Наложение выводится только у раскрытого пункта, поэтому его разметка проверяется в браузерных тестах.
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_core::Pt;
use prime_megamenu::{MegaMenu, MegaMenuItem, MegaMenuItemContext, MegaMenuOrientation, MegaMenuPt, MegaMenuSubmenu};
use prime_menu::MenuItem;

fn render(f: impl FnOnce() -> String) -> String {
    Owner::new().with(f)
}

fn model() -> Vec<MegaMenuItem> {
    let sub = |name: &str| MegaMenuSubmenu::new(name).items(vec![MenuItem::new().label("Leaf")]);
    vec![
        MegaMenuItem::new().label("Furniture").columns(vec![vec![sub("Living")], vec![sub("Kitchen")]]),
        MegaMenuItem::new().label("Hidden").visible(false),
        MegaMenuItem::from(MenuItem::separator()),
        MegaMenuItem::new().label("Off").disabled(true),
        MegaMenuItem::from(
            MenuItem::new().label("Quit").url("/q").target("_blank").title("Bye").style_class("quit").link_class("lnk").icon_class("ico"),
        ),
    ]
}

#[test]
fn markup_has_button_root_list_and_items() {
    let html = render(|| view! { <MegaMenu id="mm" model=model() /> }.to_html());
    assert!(html.contains("id=\"mm\"") && html.contains("p-megamenu p-component p-megamenu-horizontal"), "{html}");
    assert!(!html.contains("p-megamenu-mobile") && !html.contains("p-megamenu-vertical"), "{html}");
    assert!(
        html.contains("id=\"mm_list\"") && html.contains("class=\"p-megamenu-root-list\"") && html.contains("role=\"menubar\""),
        "{html}"
    );
    assert!(html.contains("aria-orientation=\"horizontal\"") && html.contains("tabindex=\"0\""), "{html}");
    assert!(html.contains("p-megamenu-button") && html.contains("role=\"button\"") && html.contains("aria-expanded=\"false\""), "{html}");
    assert!(html.contains("aria-controls=\"mm\"") && html.contains("aria-label=\"Navigation\""), "{html}");
    assert_eq!(html.matches("role=\"menuitem\"").count(), 3, "Furniture, Off, Quit: скрытый и разделитель не считаются: {html}");
    assert_eq!(html.matches("role=\"separator\"").count(), 1, "{html}");
    assert!(html.contains("p-megamenu-separator") && !html.contains("Hidden"), "{html}");
    assert!(html.contains("id=\"mm_0\"") && html.contains("id=\"mm_3\"") && html.contains("id=\"mm_4\""), "{html}");
    assert!(!html.contains("p-megamenu-overlay"), "наложение — только у раскрытого пункта: {html}");
}

#[test]
fn group_item_has_aria_arrow_and_levels() {
    let html = render(|| view! { <MegaMenu id="mm" model=model() /> }.to_html());
    assert!(html.contains("aria-haspopup=\"menu\"") && html.contains("aria-expanded=\"false\""), "{html}");
    assert!(html.contains("p-megamenu-submenu-icon") && html.contains("<svg"), "{html}");
    assert!(html.contains("aria-level=\"1\"") && html.contains("aria-setsize=\"3\"") && html.contains("aria-posinset=\"1\""), "{html}");
    assert!(html.contains("data-p-active=\"false\"") && html.contains("data-p-focused=\"false\""), "{html}");
}

#[test]
fn disabled_and_link_properties() {
    let html = render(|| view! { <MegaMenu id="mm" model=model() /> }.to_html());
    assert!(
        html.contains("p-megamenu-item p-disabled") && html.contains("aria-disabled=\"true\"") && html.contains("data-p-disabled=\"true\""),
        "{html}"
    );
    assert!(html.contains("href=\"/q\"") && html.contains("target=\"_blank\"") && html.contains("title=\"Bye\""), "{html}");
    assert!(html.contains("p-megamenu-item quit") && html.contains("p-megamenu-item-link lnk"), "{html}");
}

#[test]
fn vertical_orientation() {
    let html = render(|| view! { <MegaMenu id="mm" model=model() orientation=MegaMenuOrientation::Vertical /> }.to_html());
    assert!(html.contains("p-megamenu-vertical") && !html.contains("p-megamenu-horizontal"), "{html}");
    assert!(html.contains("aria-orientation=\"vertical\""), "{html}");
    assert!(html.contains("p-megamenu-submenu-icon"), "{html}");
}

#[test]
fn empty_model_has_no_button() {
    let html = render(|| view! { <MegaMenu /> }.to_html());
    assert!(html.contains("p-megamenu-root-list") && !html.contains("p-megamenu-button"), "{html}");
}

#[test]
fn slots_scroll_height_and_disabled() {
    let html = render(|| {
        view! {
            <MegaMenu
                id="mm"
                model=model()
                start=ViewFn::from(|| view! { <b>"S"</b> })
                end=ViewFn::from(|| view! { <i>"E"</i> })
                scroll_height="10rem"
                disabled=true
            />
        }
        .to_html()
    });
    assert!(html.contains("p-megamenu-start") && html.contains("p-megamenu-end"), "{html}");
    assert!(html.find("p-megamenu-start").unwrap() < html.find("p-megamenu-button").unwrap(), "{html}");
    assert!(html.find("p-megamenu-root-list").unwrap() < html.find("p-megamenu-end").unwrap(), "{html}");
    assert!(html.contains("max-height: 10rem; overflow: auto") && html.contains("tabindex=\"-1\""), "{html}");
}

#[test]
fn root_style_class_and_pt() {
    let pt = MegaMenuPt { root: Pt::new().class("r").attr("data-x", "1"), item: Pt::new().class("i"), ..MegaMenuPt::default() };
    let html = render(|| view! { <MegaMenu id="mm" model=model() style="width: 10rem" style_class="mine" pt=pt /> }.to_html());
    assert!(html.contains("mine") && html.contains(" r") && html.contains("data-x=\"1\"") && html.contains("width: 10rem"), "{html}");
    assert!(html.contains("p-megamenu-item i") || html.contains(" i\""), "{html}");
}

#[test]
fn item_template_and_custom_icons() {
    let item = Callback::new(|ctx: MegaMenuItemContext| {
        let text = format!("{}{}", if ctx.root { "R" } else { "L" }, ctx.item.label.clone().unwrap_or_default());
        view! { <b class="tpl">{text}</b> }.into_any()
    });
    let html = render(|| view! { <MegaMenu id="mm" model=model() item=item menu_icon=|| view! { <i class="burger" /> } /> }.to_html());
    assert!(!html.contains("p-megamenu-item-link") && html.contains("RFurniture") && html.contains("RQuit"), "{html}");
    assert!(html.contains("class=\"burger\""), "{html}");
    let html = render(|| view! { <MegaMenu id="mm" model=model() submenu_icon=|| view! { <i class="sub" /> } /> }.to_html());
    assert!(html.contains("class=\"sub\"") && !html.contains("p-megamenu-submenu-icon"), "{html}");
}
