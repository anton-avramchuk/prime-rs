//! Разметка TreeSelect (SSR): корень, скрытое поле, метка (запятая, чипы, подсказка), классы состояния, очистка, кнопка, `pt`.
//! Панель с деревом выводится только в открытом состоянии и проверяется в браузерных тестах.
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_core::{Pt, Size, Variant};
use prime_tree::{TreeNode, TreeSelectionMode};
use prime_treeselect::{TreeSelect, TreeSelectDisplay, TreeSelectPt};

fn n(key: &str, label: &str) -> TreeNode {
    TreeNode::new(key).label(label)
}

fn options() -> Vec<TreeNode> {
    vec![n("0", "Documents").children(vec![n("0-0", "Work"), n("0-1", "Home")]), n("1", "Events")]
}

fn render(f: impl FnOnce() -> String) -> String {
    Owner::new().with(f)
}

#[test]
fn empty_field_shows_placeholder() {
    let html = render(|| view! { <TreeSelect options=options() placeholder="Select Item" /> }.to_html());
    assert!(html.contains("p-treeselect p-component p-inputwrapper"), "{html}");
    assert!(html.contains("p-treeselect-label p-placeholder") && html.contains("Select Item"), "{html}");
    assert!(
        html.contains("p-hidden-accessible")
            && html.contains("role=\"combobox\"")
            && html.contains("readonly")
            && html.contains("aria-haspopup=\"tree\""),
        "{html}"
    );
    assert!(html.contains("aria-expanded=\"false\"") && !html.contains("aria-controls"), "{html}");
    assert!(
        html.contains("p-treeselect-label-container")
            && html.contains("p-treeselect-dropdown")
            && html.contains("aria-label=\"treeselect trigger\""),
        "{html}"
    );
    assert!(
        !html.contains("p-treeselect-overlay") && !html.contains("p-treeselect-open") && !html.contains("p-inputwrapper-filled"),
        "{html}"
    );
    assert!(html.contains("<svg") && html.contains("p-treeselect-dropdown-icon"), "{html}");
}

#[test]
fn empty_field_without_placeholder_is_hidden_label() {
    let html = render(|| view! { <TreeSelect options=options() /> }.to_html());
    assert!(html.contains("p-treeselect-label p-treeselect-label-empty") && !html.contains("p-placeholder"), "{html}");
}

#[test]
fn selected_nodes_are_joined_with_commas() {
    let html = render(|| {
        let value = RwSignal::new(vec!["1".to_owned(), "0-0".to_owned()]);
        view! { <TreeSelect options=options() value=value selection_mode=TreeSelectionMode::Multiple placeholder="Select" /> }.to_html()
    });
    assert!(html.contains("Events, Work") && !html.contains("p-placeholder") && !html.contains("Select"), "{html}");
    assert!(html.contains("p-inputwrapper-filled"), "{html}");
}

#[test]
fn chip_display_renders_chips() {
    let html = render(|| {
        let value = RwSignal::new(vec!["0-0".to_owned(), "0-1".to_owned()]);
        view! { <TreeSelect options=options() value=value selection_mode=TreeSelectionMode::Multiple display=TreeSelectDisplay::Chip /> }
            .to_html()
    });
    assert!(html.contains("p-treeselect-display-chip") && html.matches("p-treeselect-chip-item").count() == 2, "{html}");
    assert!(html.contains("p-treeselect-chip") && html.contains("Work") && html.contains("Home"), "{html}");
}

#[test]
fn clear_icon_requires_value_and_show_clear() {
    let with = |show: bool, selected: bool| {
        render(move || {
            let value = RwSignal::new(if selected { vec!["1".to_owned()] } else { vec![] });
            view! { <TreeSelect options=options() value=value show_clear=show /> }.to_html()
        })
    };
    assert!(with(true, true).contains("p-treeselect-clear-icon") && with(true, true).contains("p-treeselect-clearable"));
    assert!(!with(true, false).contains("p-treeselect-clear-icon"));
    assert!(!with(false, true).contains("p-treeselect-clear-icon"));
    let html = render(|| {
        view! { <TreeSelect options=options() show_clear=true disabled=true value=RwSignal::new(vec!["1".to_owned()]) /> }.to_html()
    });
    assert!(!html.contains("p-treeselect-clear-icon"), "у отключённого поля очистки нет: {html}");
}

#[test]
fn state_classes_and_attributes() {
    let html = render(|| {
        view! { <TreeSelect options=options() disabled=true invalid=true variant=Variant::Filled size=Size::Small fluid=true tabindex=3 input_id="ts" aria_label="Pick" /> }.to_html()
    });
    for class in ["p-disabled", "p-invalid", "p-variant-filled", "p-treeselect-fluid", "p-treeselect-sm p-inputfield-sm"] {
        assert!(html.contains(class), "{class}: {html}");
    }
    assert!(
        html.contains("disabled") && html.contains("tabindex=\"-1\"") && html.contains("id=\"ts\"") && html.contains("aria-label=\"Pick\""),
        "{html}"
    );
    let html = render(|| view! { <TreeSelect options=options() size=Size::Large tabindex=3 /> }.to_html());
    assert!(html.contains("p-treeselect-lg p-inputfield-lg") && html.contains("tabindex=\"3\""), "{html}");
}

#[test]
fn style_class_and_pt() {
    let pt = TreeSelectPt { root: Pt::new().class("r").attr("data-x", "1"), label: Pt::new().class("l"), ..TreeSelectPt::default() };
    let html = render(|| view! { <TreeSelect options=options() style="width: 15rem" style_class="mine" pt=pt /> }.to_html());
    assert!(html.contains("mine") && html.contains(" r") && html.contains("data-x=\"1\"") && html.contains("width: 15rem"), "{html}");
    assert!(html.contains("p-treeselect-label") && html.contains(" l"), "{html}");
}

#[test]
fn value_view_replaces_the_label() {
    let view_cb = Callback::new(|nodes: Vec<TreeNode>| view! { <b class="custom">{format!("{} выбрано", nodes.len())}</b> }.into_any());
    let html = render(|| {
        let value = RwSignal::new(vec!["1".to_owned()]);
        view! { <TreeSelect options=options() value=value value_view=view_cb /> }.to_html()
    });
    assert!(html.contains("class=\"custom\"") && html.contains("1 выбрано") && !html.contains(">Events<"), "{html}");
}
