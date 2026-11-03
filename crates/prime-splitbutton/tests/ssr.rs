//! Разметка SplitButton (SSR): состав, модификаторы, наследование свойств кнопками, `content`, доступность, блокировки, `pt`.
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_button::ButtonSeverity;
use prime_core::{Pt, Size};
use prime_menu::MenuItem;
use prime_splitbutton::{SplitButton, SplitButtonPt};

fn render(f: impl FnOnce() -> String) -> String {
    Owner::new().with(f)
}

fn items() -> Vec<MenuItem> {
    vec![MenuItem::new().label("Update"), MenuItem::new().label("Delete")]
}

/// Открывающий тег первой кнопки с заданным классом.
fn button_tag<'a>(html: &'a str, class: &str) -> &'a str {
    let at = html
        .split("<button")
        .skip(1)
        .find(|b| b.split('>').next().unwrap().contains(class))
        .unwrap_or_else(|| panic!("нет {class}: {html}"));
    at.split('>').next().unwrap()
}

fn has_disabled(tag: &str) -> bool {
    tag.split_whitespace().any(|t| t == "disabled" || t.starts_with("disabled="))
}

#[test]
fn markup_has_root_two_buttons_and_a_closed_menu() {
    let html = render(|| view! { <SplitButton label="Save" model=items() /> }.to_html());
    assert!(html.contains("p-splitbutton p-component"), "{html}");
    assert_eq!(html.matches("<button").count(), 2, "{html}");
    let main = button_tag(&html, "p-splitbutton-button");
    assert!(main.contains("p-button p-component"), "{main}");
    assert!(html.contains("p-button-label") && html.contains("Save"), "{html}");
    let dropdown = button_tag(&html, "p-splitbutton-dropdown");
    assert!(dropdown.contains("p-button-icon-only") && dropdown.contains("aria-haspopup=\"true\""), "{dropdown}");
    assert!(dropdown.contains("aria-expanded=\"false\"") && dropdown.contains("aria-controls=\"pn_id_"), "{dropdown}");
    assert!(html.contains("<svg"), "стрелка по умолчанию: {html}");
    assert!(!html.contains("p-menu-overlay") && !html.contains("Update"), "меню закрыто, панели нет: {html}");
}

#[test]
fn order_is_main_button_dropdown_menu() {
    let html = render(|| view! { <SplitButton label="Save" model=items() /> }.to_html());
    let (main, dropdown) = (html.find("p-splitbutton-button").unwrap(), html.find("p-splitbutton-dropdown").unwrap());
    assert!(main < dropdown, "{html}");
}

#[test]
fn modifiers_and_severity_reach_the_root_and_both_buttons() {
    let html = render(|| {
        view! { <SplitButton label="Save" model=items() raised=true rounded=true size=Size::Small severity=ButtonSeverity::Success /> }
            .to_html()
    });
    assert!(html.contains("p-splitbutton-raised") && html.contains("p-splitbutton-rounded") && html.contains("p-splitbutton-sm"), "{html}");
    assert!(html.contains("data-p-severity=\"success\""), "{html}");
    for class in ["p-splitbutton-button", "p-splitbutton-dropdown"] {
        let tag = button_tag(&html, class);
        assert!(tag.contains("p-button-success") && tag.contains("p-button-sm"), "{class}: {tag}");
    }
}

#[test]
fn outlined_and_text_modifiers() {
    let outlined = render(|| view! { <SplitButton label="A" model=items() outlined=true /> }.to_html());
    assert!(outlined.contains("p-splitbutton-outlined"), "{outlined}");
    assert!(button_tag(&outlined, "p-splitbutton-dropdown").contains("p-button-outlined"), "{outlined}");
    let text = render(|| view! { <SplitButton label="A" model=items() text=true /> }.to_html());
    assert!(text.contains("p-splitbutton-text"), "{text}");
    assert!(button_tag(&text, "p-splitbutton-dropdown").contains("p-button-text"), "{text}");
    let large = render(|| view! { <SplitButton label="A" model=items() size=Size::Large /> }.to_html());
    assert!(large.contains("p-splitbutton-lg"), "{large}");
}

#[test]
fn disabled_flags_are_independent() {
    let all = render(|| view! { <SplitButton label="A" model=items() disabled=true /> }.to_html());
    assert!(has_disabled(button_tag(&all, "p-splitbutton-button")) && has_disabled(button_tag(&all, "p-splitbutton-dropdown")), "{all}");
    let main_only = render(|| view! { <SplitButton label="A" model=items() button_disabled=true /> }.to_html());
    assert!(has_disabled(button_tag(&main_only, "p-splitbutton-button")), "{main_only}");
    assert!(!has_disabled(button_tag(&main_only, "p-splitbutton-dropdown")), "{main_only}");
    let menu_only = render(|| view! { <SplitButton label="A" model=items() menu_button_disabled=true /> }.to_html());
    assert!(!has_disabled(button_tag(&menu_only, "p-splitbutton-button")), "{menu_only}");
    assert!(has_disabled(button_tag(&menu_only, "p-splitbutton-dropdown")), "{menu_only}");
}

#[test]
fn aria_labels_tabindex_and_custom_icons() {
    let html = render(|| {
        view! {
            <SplitButton
                label="Save"
                model=items()
                button_aria_label="Save file"
                expand_aria_label="More"
                tabindex=3
                dropdown_icon=ViewFn::from(|| view! { <i class="cog">"cog"</i> })
                icon=ViewFn::from(|| view! { <i class="chk">"chk"</i> })
            />
        }
        .to_html()
    });
    assert!(button_tag(&html, "p-splitbutton-button").contains("aria-label=\"Save file\""), "{html}");
    assert!(button_tag(&html, "p-splitbutton-button").contains("tabindex=\"3\""), "{html}");
    assert!(button_tag(&html, "p-splitbutton-dropdown").contains("aria-label=\"More\""), "{html}");
    assert!(html.contains("class=\"cog\"") && html.contains("class=\"chk\""), "{html}");
    assert!(!html.contains("<svg"), "обе иконки заменены: {html}");
}

#[test]
fn content_replaces_the_label() {
    let html = render(|| {
        view! { <SplitButton label="Ignored" model=items() content=ViewFn::from(|| view! { <b class="logo">"PrimeNG"</b> }) /> }.to_html()
    });
    assert!(html.contains("class=\"logo\"") && html.contains("PrimeNG"), "{html}");
    assert!(!html.contains("p-button-label"), "метки нет: {html}");
}

#[test]
fn root_style_class_and_pt() {
    let pt = SplitButtonPt {
        root: Pt::new().class("r").attr("data-x", "1"),
        button: Pt::new().class("mb"),
        dropdown: Pt::new().class("dd").attr("data-y", "2"),
    };
    let html = render(|| view! { <SplitButton label="A" model=items() style="width: 12rem" style_class="mine" pt=pt /> }.to_html());
    assert!(html.contains("p-splitbutton p-component mine r") && html.contains("data-x=\"1\"") && html.contains("width: 12rem"), "{html}");
    assert!(button_tag(&html, "p-splitbutton-button").contains("mb"), "{html}");
    let dropdown = button_tag(&html, "p-splitbutton-dropdown");
    assert!(dropdown.contains("dd") && dropdown.contains("data-y=\"2\""), "{dropdown}");
}
