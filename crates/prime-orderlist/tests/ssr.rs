//! Разметка OrderList (SSR): корень, панель кнопок и их состояние, список, положение кнопок, `pt`.
#![cfg(feature = "ssr")]
#![recursion_limit = "512"]

use leptos::prelude::*;
use prime_core::Pt;
use prime_listbox::ListboxOption;
use prime_orderlist::{ControlsPosition, OrderList, OrderListPt};

fn render(f: impl FnOnce() -> String) -> String {
    Owner::new().with(f)
}

fn items() -> RwSignal<Vec<ListboxOption>> {
    RwSignal::new(vec![ListboxOption::new("Apple", "a"), ListboxOption::new("Banana", "b"), ListboxOption::new("Cherry", "c")])
}

/// Открывающий тег элемента, в котором встречается класс.
fn tag_of<'a>(html: &'a str, class: &str) -> &'a str {
    let at = html.find(class).unwrap_or_else(|| panic!("нет {class}: {html}"));
    let start = html[..at].rfind('<').unwrap();
    let end = at + html[at..].find('>').unwrap();
    &html[start..=end]
}

#[test]
fn markup_and_buttons() {
    let html = render(|| view! { <OrderList value=items() /> }.to_html());
    assert!(html.contains("p-orderlist p-component p-orderlist-controls-left"), "{html}");
    assert!(html.contains("p-orderlist-controls"), "{html}");
    for (label, class) in [
        ("Move Up", "p-orderlist-move-up-button"),
        ("Move Top", "p-orderlist-move-top-button"),
        ("Move Down", "p-orderlist-move-down-button"),
        ("Move Bottom", "p-orderlist-move-bottom-button"),
    ] {
        let tag = tag_of(&html, class);
        assert!(tag.contains(&format!("aria-label=\"{label}\"")) && tag.contains("disabled"), "{label}: {html}");
    }
    let up = html.find("p-orderlist-move-up-button").unwrap();
    let top = html.find("p-orderlist-move-top-button").unwrap();
    let down = html.find("p-orderlist-move-down-button").unwrap();
    let bottom = html.find("p-orderlist-move-bottom-button").unwrap();
    assert!(up < top && top < down && down < bottom, "порядок кнопок: вверх, в начало, вниз, в конец");
    assert!(html.contains("p-listbox") && html.contains("Apple") && html.contains("Cherry"), "{html}");
    assert!(html.contains("aria-multiselectable=\"true\""), "множественный выбор: {html}");
}

#[test]
fn selection_enables_the_buttons() {
    let html = render(|| {
        let selection = RwSignal::new(vec!["b".to_owned()]);
        view! { <OrderList value=items() selection=selection /> }.to_html()
    });
    assert!(!tag_of(&html, "p-orderlist-move-up-button").contains("disabled"), "{html}");
    assert!(!tag_of(&html, "p-orderlist-move-bottom-button").contains("disabled"), "{html}");
    assert!(html.contains("p-listbox-option-selected") || html.contains("aria-selected=\"true\""), "{html}");
}

#[test]
fn disabled_keeps_buttons_disabled() {
    let html = render(|| {
        let selection = RwSignal::new(vec!["b".to_owned()]);
        view! { <OrderList value=items() selection=selection disabled=true /> }.to_html()
    });
    assert!(tag_of(&html, "p-orderlist-move-up-button").contains("disabled"), "{html}");
}

#[test]
fn controls_position_and_filter() {
    let html = render(|| {
        view! { <OrderList value=items() controls_position=ControlsPosition::Right filter=true filter_placeholder="Search" /> }.to_html()
    });
    assert!(html.contains("p-orderlist-controls-right") && !html.contains("p-orderlist-controls-left"), "{html}");
    assert!(html.contains("placeholder=\"Search\""), "{html}");
}

#[test]
fn style_and_pass_through() {
    let pt = OrderListPt { root: Pt::new().class("root-x"), controls: Pt::new().class("controls-x"), ..Default::default() };
    let html = render(|| view! { <OrderList value=items() style="width: 30rem" style_class="mine" pt=pt /> }.to_html());
    assert!(html.contains("root-x") && html.contains("mine") && html.contains("width: 30rem"), "{html}");
    assert!(html.contains("p-orderlist-controls controls-x"), "{html}");
}
