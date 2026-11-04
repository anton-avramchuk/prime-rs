//! Разметка Menubar (SSR): корень, слоты, кнопка, корневой список, пункты, группы, разделители, `aria-*`, шаблоны, `pt`.
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_core::Pt;
use prime_menu::MenuItem;
use prime_menubar::{Menubar, MenubarItemContext, MenubarPt};

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
fn markup_has_button_root_list_and_items() {
    let html = render(|| view! { <Menubar id="mb" model=model() /> }.to_html());
    assert!(html.contains("id=\"mb\"") && html.contains("p-menubar p-component"), "{html}");
    assert!(!html.contains("p-menubar-mobile"), "{html}");
    assert!(
        html.contains("id=\"mb_list\"") && html.contains("class=\"p-menubar-root-list\"") && html.contains("role=\"menubar\""),
        "{html}"
    );
    assert!(html.contains("p-menubar-button") && html.contains("role=\"button\"") && html.contains("aria-expanded=\"false\""), "{html}");
    assert!(
        html.contains("aria-controls=\"mb\"") && html.contains("aria-haspopup=\"true\"") && html.contains("aria-label=\"Navigation\""),
        "{html}"
    );
    assert_eq!(html.matches("role=\"menuitem\"").count(), 3, "File, Users, Quit: скрытый и разделитель не считаются: {html}");
    assert_eq!(html.matches("role=\"separator\"").count(), 1, "{html}");
    assert!(html.contains("id=\"mb_0\"") && html.contains("id=\"mb_3\"") && html.contains("id=\"mb_4\""), "{html}");
    assert!(!html.contains("Hidden") && !html.contains("p-tieredmenu"), "{html}");
}

#[test]
fn empty_model_has_no_button_and_an_empty_list() {
    let html = render(|| view! { <Menubar /> }.to_html());
    assert!(!html.contains("p-menubar-button"), "{html}");
    assert!(html.contains("p-menubar-root-list") && !html.contains("role=\"menuitem\""), "{html}");
}

#[test]
fn groups_carry_aria_and_a_downward_arrow_but_no_submenu_until_opened() {
    let html = render(|| view! { <Menubar id="mb" model=model() /> }.to_html());
    assert_eq!(html.matches("aria-haspopup=\"menu\"").count(), 1, "только File — группа: {html}");
    assert!(html.contains("aria-expanded=\"false\"") && html.contains("p-menubar-submenu-icon"), "{html}");
    assert!(!html.contains("p-menubar-submenu\"") && !html.contains("Share"), "подменю выводится только у раскрытой группы: {html}");
}

#[test]
fn items_have_state_attributes_and_positions() {
    let html = render(|| view! { <Menubar id="mb" model=model() /> }.to_html());
    assert!(html.contains("aria-setsize=\"3\""), "{html}");
    assert!(html.contains("aria-posinset=\"1\"") && html.contains("aria-posinset=\"2\"") && html.contains("aria-posinset=\"3\""), "{html}");
    assert!(html.contains("data-p-highlight=\"false\"") && html.contains("data-p-focused=\"false\""), "{html}");
    assert!(
        html.contains("p-menubar-item p-disabled") && html.contains("data-p-disabled=\"true\"") && html.contains("aria-disabled=\"true\""),
        "{html}"
    );
    assert!(html.contains("aria-label=\"Users\"") && html.contains("p-menubar-item quit"), "{html}");
}

#[test]
fn links_carry_attributes_icons_and_labels() {
    let model = vec![
        leaf("Users").url("/users").target("_blank").title("All users"),
        leaf("Quit").link_class("lnk").icon_class("ico").icon(ViewFn::from(|| view! { <i class="my-icon">"i"</i> })),
    ];
    let html = render(|| view! { <Menubar model=model /> }.to_html());
    assert!(html.contains("href=\"/users\"") && html.contains("target=\"_blank\"") && html.contains("title=\"All users\""), "{html}");
    assert!(html.contains("p-menubar-item-link lnk") && html.contains("p-menubar-item-icon ico") && html.contains("my-icon"), "{html}");
    assert!(html.contains("p-menubar-item-content") && html.contains("class=\"p-menubar-item-label\""), "{html}");
    assert_eq!(html.matches("p-menubar-item-icon").count(), 1, "{html}");
}

#[test]
fn labels_pass_through_to_the_root_list() {
    let html = render(|| view! { <Menubar model=model() aria_label="Main" aria_labelledby="lbl" /> }.to_html());
    assert!(html.contains("aria-label=\"Main\"") && html.contains("aria-labelledby=\"lbl\""), "{html}");
}

#[test]
fn start_and_end_slots_surround_the_list() {
    let html = render(|| {
        view! {
            <Menubar
                model=model()
                start=ViewFn::from(|| view! { <b class="logo">"Logo"</b> })
                end=ViewFn::from(|| view! { <i class="tail">"End"</i> })
            />
        }
        .to_html()
    });
    let (start, button, list, end) = (
        html.find("p-menubar-start").unwrap(),
        html.find("p-menubar-button").unwrap(),
        html.find("p-menubar-root-list").unwrap(),
        html.find("p-menubar-end").unwrap(),
    );
    assert!(start < button && button < list && list < end, "порядок слотов: {html}");
    assert!(html.contains("class=\"logo\"") && html.contains("class=\"tail\""), "{html}");
    let none = render(|| view! { <Menubar model=model() /> }.to_html());
    assert!(!none.contains("p-menubar-start") && !none.contains("p-menubar-end"), "{none}");
}

#[test]
fn item_template_submenu_icon_and_menu_icon_replace_defaults() {
    let html = render(|| {
        view! {
            <Menubar
                model=model()
                item=Callback::new(|ctx: MenubarItemContext| view! { <b class="tpl">{ctx.item.label.unwrap_or_default()}</b> }.into_any())
                menu_icon=ViewFn::from(|| view! { <i class="my-bars">"="</i> })
            />
        }
        .to_html()
    });
    assert_eq!(html.matches("class=\"tpl\"").count(), 3, "{html}");
    assert!(!html.contains("p-menubar-item-link") && html.contains("my-bars"), "{html}");
    let with_arrow =
        render(|| view! { <Menubar model=model() submenu_icon=ViewFn::from(|| view! { <i class="my-arrow">"v"</i> }) /> }.to_html());
    assert!(with_arrow.contains("my-arrow") && !with_arrow.contains("p-menubar-submenu-icon"), "{with_arrow}");
}

#[test]
fn style_class_style_and_pt() {
    let pt = MenubarPt {
        root: Pt::new().class("r").attr("data-x", "1"),
        root_list: Pt::new().class("l"),
        button: Pt::new().class("b"),
        item: Pt::new().class("i"),
        item_content: Pt::new().class("c"),
        item_link: Pt::new().class("k"),
        item_label: Pt::new().class("t"),
        submenu_icon: Pt::new().class("s"),
        separator: Pt::new().class("sep"),
        ..MenubarPt::default()
    };
    let html = render(|| view! { <Menubar model=model() style_class="mine" style="width: 10rem" pt=pt /> }.to_html());
    assert!(html.contains("p-menubar p-component mine r") && html.contains("data-x=\"1\"") && html.contains("width: 10rem"), "{html}");
    for needle in [
        "p-menubar-root-list l",
        "p-menubar-button b",
        "p-menubar-item i",
        "p-menubar-item-content c",
        "p-menubar-item-link k",
        "p-menubar-item-label t",
        "p-menubar-submenu-icon s",
        "p-menubar-separator sep",
    ] {
        assert!(html.contains(needle), "нет {needle}: {html}");
    }
}
