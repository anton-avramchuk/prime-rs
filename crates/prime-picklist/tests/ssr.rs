//! Разметка PickList (SSR): пять блоков, кнопки и их состояние, заголовки, поиск, `pt`.
#![cfg(feature = "ssr")]
#![recursion_limit = "512"]

use leptos::prelude::*;
use prime_core::Pt;
use prime_listbox::ListboxOption;
use prime_picklist::{PickList, PickListPt};

fn render(f: impl FnOnce() -> String) -> String {
    Owner::new().with(f)
}

fn list(names: &[&str]) -> RwSignal<Vec<ListboxOption>> {
    RwSignal::new(names.iter().map(|n| ListboxOption::new(*n, n.to_lowercase())).collect())
}

/// Открывающий тег элемента, в котором встречается подстрока.
fn tag_of<'a>(html: &'a str, needle: &str) -> &'a str {
    let at = html.find(needle).unwrap_or_else(|| panic!("нет {needle}: {html}"));
    let start = html[..at].rfind('<').unwrap();
    let end = at + html[at..].find('>').unwrap();
    &html[start..=end]
}

#[test]
fn five_blocks_in_order() {
    let html = render(|| view! { <PickList source=list(&["Apple", "Pear"]) target=list(&[]) /> }.to_html());
    assert!(html.contains("p-picklist p-component"), "{html}");
    let order = [
        "p-picklist-source-controls",
        "p-picklist-source-list-container",
        "p-picklist-transfer-controls",
        "p-picklist-target-list-container",
        "p-picklist-target-controls",
    ];
    let positions: Vec<usize> = order.iter().map(|c| html.find(c).unwrap_or_else(|| panic!("нет {c}: {html}"))).collect();
    assert!(positions.windows(2).all(|w| w[0] < w[1]), "{positions:?}");
    assert!(html.contains("Apple") && html.contains("Pear"), "{html}");
    assert_eq!(html.matches("p-listbox p-component").count(), 2, "два списка: {html}");
}

#[test]
fn button_states_without_selection() {
    let html = render(|| view! { <PickList source=list(&["Apple"]) target=list(&[]) /> }.to_html());
    for label in ["Move Up", "Move Top", "Move Down", "Move Bottom"] {
        assert_eq!(html.matches(&format!("aria-label=\"{label}\"")).count(), 2, "{label}: {html}");
    }
    for label in ["Move to Target", "Move to Source", "Move All to Source"] {
        assert!(tag_of(&html, &format!("aria-label=\"{label}\"")).contains("disabled"), "{label} должна быть недоступна: {html}");
    }
    assert!(!tag_of(&html, "aria-label=\"Move All to Target\"").contains("disabled"), "в источнике есть элементы: {html}");
}

#[test]
fn selection_enables_transfer() {
    let html = render(|| {
        let selection = RwSignal::new(vec!["apple".to_owned()]);
        view! { <PickList source=list(&["Apple"]) target=list(&["Pear"]) source_selection=selection /> }.to_html()
    });
    assert!(!tag_of(&html, "aria-label=\"Move to Target\"").contains("disabled"), "{html}");
    assert!(tag_of(&html, "aria-label=\"Move to Source\"").contains("disabled"), "{html}");
    assert!(!tag_of(&html, "aria-label=\"Move All to Source\"").contains("disabled"), "{html}");
}

#[test]
fn disabled_disables_everything() {
    let html = render(|| {
        let selection = RwSignal::new(vec!["apple".to_owned()]);
        view! { <PickList source=list(&["Apple"]) target=list(&["Pear"]) source_selection=selection disabled=true /> }.to_html()
    });
    for label in ["Move to Target", "Move All to Target", "Move All to Source", "Move Up"] {
        assert!(tag_of(&html, &format!("aria-label=\"{label}\"")).contains("disabled"), "{label}: {html}");
    }
}

#[test]
fn headers_filters_and_controls_toggles() {
    let html = render(|| {
        view! {
            <PickList
                source=list(&["Apple"])
                target=list(&[])
                source_header="Available"
                target_header="Selected"
                filter=true
                source_filter_placeholder="Find source"
                show_target_filter=false
                show_target_controls=false
            />
        }
        .to_html()
    });
    assert!(html.contains("Available") && html.contains("Selected"), "{html}");
    assert!(html.contains("placeholder=\"Find source\"") && html.matches("p-listbox-filter").count() == 1, "{html}");
    assert!(!html.contains("p-picklist-target-controls") && html.contains("p-picklist-source-controls"), "{html}");
}

#[test]
fn style_and_pass_through() {
    let pt = PickListPt { root: Pt::new().class("root-x"), transfer_controls: Pt::new().class("transfer-x"), ..Default::default() };
    let html = render(|| view! { <PickList source=list(&[]) target=list(&[]) style="width: 40rem" style_class="mine" pt=pt /> }.to_html());
    assert!(html.contains("root-x") && html.contains("mine") && html.contains("width: 40rem"), "{html}");
    assert!(html.contains("p-picklist-transfer-controls transfer-x"), "{html}");
}
