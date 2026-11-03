//! Разметка TieredMenu (SSR): корень, корневой список, пункты, группы, разделители, `aria-*`, шаблоны, `pt`, режим `popup`.
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_core::Pt;
use prime_menu::MenuItem;
use prime_tieredmenu::{TieredMenu, TieredMenuItemContext, TieredMenuPt};

fn render(f: impl FnOnce() -> String) -> String {
    Owner::new().with(f)
}

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

#[test]
fn markup_has_root_list_and_items() {
    let html = render(|| view! { <TieredMenu id="tm" model=model() /> }.to_html());
    assert!(html.contains("id=\"tm\"") && html.contains("p-tieredmenu p-component"), "{html}");
    assert!(!html.contains("p-tieredmenu-overlay") && !html.contains("p-tieredmenu-mobile"), "{html}");
    assert!(
        html.contains("id=\"tm_list\"") && html.contains("class=\"p-tieredmenu-root-list\"") && html.contains("role=\"menu\""),
        "{html}"
    );
    assert!(html.contains("aria-orientation=\"vertical\"") && html.contains("tabindex=\"0\""), "{html}");
    assert_eq!(html.matches("role=\"menuitem\"").count(), 3, "File, Users, Quit: скрытый и разделитель не считаются: {html}");
    assert_eq!(html.matches("role=\"separator\"").count(), 1, "{html}");
    assert!(html.contains("id=\"tm_0\"") && html.contains("id=\"tm_3\"") && html.contains("id=\"tm_4\""), "{html}");
    assert!(!html.contains("Hidden"), "{html}");
}

#[test]
fn groups_carry_aria_and_an_arrow_but_no_submenu_until_opened() {
    let html = render(|| view! { <TieredMenu id="tm" model=model() /> }.to_html());
    assert!(html.contains("aria-haspopup=\"menu\"") && html.contains("aria-expanded=\"false\""), "{html}");
    assert_eq!(html.matches("aria-haspopup=\"menu\"").count(), 1, "только File — группа: {html}");
    assert!(html.contains("p-tieredmenu-submenu-icon"), "{html}");
    assert!(!html.contains("p-tieredmenu-submenu\"") && !html.contains("Share"), "подменю выводится только у раскрытой группы: {html}");
}

#[test]
fn items_have_state_attributes_and_positions() {
    let html = render(|| view! { <TieredMenu id="tm" model=model() /> }.to_html());
    assert!(html.contains("aria-setsize=\"3\""), "{html}");
    assert!(html.contains("aria-posinset=\"1\"") && html.contains("aria-posinset=\"2\"") && html.contains("aria-posinset=\"3\""), "{html}");
    assert!(html.contains("data-p-highlight=\"false\"") && html.contains("data-p-focused=\"false\""), "{html}");
    assert!(
        html.contains("p-tieredmenu-item p-disabled")
            && html.contains("data-p-disabled=\"true\"")
            && html.contains("aria-disabled=\"true\""),
        "{html}"
    );
    assert!(html.contains("aria-label=\"Users\"") && html.contains("aria-label=\"Quit\""), "{html}");
    assert!(html.contains("p-tieredmenu-item quit"), "класс пункта: {html}");
}

#[test]
fn links_carry_attributes_icons_and_labels() {
    let model = vec![
        leaf("Users").url("/users").target("_blank").title("All users"),
        leaf("Quit").link_class("lnk").icon_class("ico").icon(ViewFn::from(|| view! { <i class="my-icon">"i"</i> })),
    ];
    let html = render(|| view! { <TieredMenu model=model /> }.to_html());
    assert!(html.contains("href=\"/users\"") && html.contains("target=\"_blank\"") && html.contains("title=\"All users\""), "{html}");
    assert!(
        html.contains("p-tieredmenu-item-link lnk") && html.contains("p-tieredmenu-item-icon ico") && html.contains("my-icon"),
        "{html}"
    );
    assert!(html.contains("p-tieredmenu-item-content") && html.contains("class=\"p-tieredmenu-item-label\""), "{html}");
    assert_eq!(html.matches("p-tieredmenu-item-icon").count(), 1, "значок только у пункта со значком: {html}");
}

#[test]
fn disabled_menu_is_not_tabbable_and_labels_pass_through() {
    let html = render(|| view! { <TieredMenu model=model() disabled=true aria_label="Main" aria_labelledby="lbl" tabindex=3 /> }.to_html());
    assert!(html.contains("tabindex=\"-1\"") && html.contains("aria-label=\"Main\"") && html.contains("aria-labelledby=\"lbl\""), "{html}");
    let custom = render(|| view! { <TieredMenu model=model() tabindex=3 /> }.to_html());
    assert!(custom.contains("tabindex=\"3\""), "{custom}");
}

#[test]
fn item_template_and_submenu_icon_replace_defaults() {
    let html = render(|| {
        view! {
            <TieredMenu
                model=model()
                item=Callback::new(|ctx: TieredMenuItemContext| view! { <b class="tpl">{ctx.item.label.unwrap_or_default()}</b> }.into_any())
                submenu_icon=ViewFn::from(|| view! { <i class="my-arrow">">"</i> })
            />
        }
        .to_html()
    });
    assert_eq!(html.matches("class=\"tpl\"").count(), 3, "{html}");
    assert!(!html.contains("p-tieredmenu-item-link"), "{html}");
    let with_arrow = render(|| {
        view! {
            <TieredMenu model=model() submenu_icon=ViewFn::from(|| view! { <i class="my-arrow">">"</i> }) />
        }
        .to_html()
    });
    assert!(with_arrow.contains("my-arrow") && !with_arrow.contains("p-tieredmenu-submenu-icon"), "{with_arrow}");
}

#[test]
fn popup_renders_nothing_until_shown() {
    let html = render(|| view! { <TieredMenu model=model() popup=true /> }.to_html());
    assert!(!html.contains("p-tieredmenu"), "{html}");
}

#[test]
fn style_class_style_and_pt() {
    let pt = TieredMenuPt {
        root: Pt::new().class("r").attr("data-x", "1"),
        root_list: Pt::new().class("l"),
        item: Pt::new().class("i"),
        item_content: Pt::new().class("c"),
        item_link: Pt::new().class("k"),
        item_label: Pt::new().class("t"),
        submenu_icon: Pt::new().class("s"),
        separator: Pt::new().class("sep"),
        ..TieredMenuPt::default()
    };
    let html = render(|| view! { <TieredMenu model=model() style_class="mine" style="width: 10rem" pt=pt /> }.to_html());
    assert!(html.contains("p-tieredmenu p-component mine r") && html.contains("data-x=\"1\"") && html.contains("width: 10rem"), "{html}");
    for needle in [
        "p-tieredmenu-root-list l",
        "p-tieredmenu-item i",
        "p-tieredmenu-item-content c",
        "p-tieredmenu-item-link k",
        "p-tieredmenu-item-label t",
        "p-tieredmenu-submenu-icon s",
        "p-tieredmenu-separator sep",
    ] {
        assert!(html.contains(needle), "нет {needle}: {html}");
    }
}

#[test]
fn empty_model_renders_an_empty_list() {
    let html = render(|| view! { <TieredMenu /> }.to_html());
    assert!(html.contains("p-tieredmenu-root-list") && !html.contains("role=\"menuitem\""), "{html}");
}
