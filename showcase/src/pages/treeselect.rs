//! Страница TreeSelect: демонстрации `basic`, `multiple`, `checkbox`, `filter`, `clearicon`, `disabled`, `invalid` и `template`
//! из showcase PrimeNG (`apps/showcase/doc/treeselect`; формы Angular, виртуальная прокрутка и ленивая загрузка не переносятся).

use super::Demo;
use leptos::prelude::*;
use prime_tree::{TreeNode, TreeSelectionMode};
use prime_treeselect::{TreeSelect, TreeSelectDisplay};

fn n(key: &str, label: &str) -> TreeNode {
    TreeNode::new(key).label(label)
}

fn files() -> Vec<TreeNode> {
    vec![
        n("0", "Documents").children(vec![
            n("0-0", "Work").children(vec![n("0-0-0", "Expenses.doc"), n("0-0-1", "Resume.doc")]),
            n("0-1", "Home").children(vec![n("0-1-0", "Invoices.txt")]),
        ]),
        n("1", "Pictures").children(vec![n("1-0", "barcelona.jpg"), n("1-1", "logo.jpg")]),
        n("2", "Movies").children(vec![n("2-0", "Al Pacino").children(vec![n("2-0-0", "Scarface.mp4")]), n("2-1", "Robert De Niro")]),
    ]
}

const WIDTH: &str = "width: 100%; max-width: 20rem";

#[component]
fn Basic() -> impl IntoView {
    let value = RwSignal::new(Vec::<String>::new());
    view! { <TreeSelect id="basic_select" options=files() value=value placeholder="Select Item" style=WIDTH /> }
}

#[component]
fn Multiple() -> impl IntoView {
    let value = RwSignal::new(Vec::<String>::new());
    view! {
        <TreeSelect
            options=files()
            value=value
            selection_mode=TreeSelectionMode::Multiple
            display=TreeSelectDisplay::Chip
            placeholder="Select Items"
            style=WIDTH
        />
    }
}

#[component]
fn Checkbox() -> impl IntoView {
    let value = RwSignal::new(Vec::<String>::new());
    view! { <TreeSelect options=files() value=value selection_mode=TreeSelectionMode::Checkbox placeholder="Select Items" style=WIDTH /> }
}

#[component]
fn Filter() -> impl IntoView {
    let value = RwSignal::new(Vec::<String>::new());
    view! { <TreeSelect options=files() value=value filter=true filter_placeholder="Search" placeholder="Select Item" style=WIDTH /> }
}

#[component]
fn Clear() -> impl IntoView {
    let value = RwSignal::new(vec!["0-0".to_owned()]);
    view! { <TreeSelect options=files() value=value show_clear=true placeholder="Select Item" style=WIDTH /> }
}

/// Страница `/treeselect`.
#[component]
pub fn TreeSelectPage() -> impl IntoView {
    view! {
        <h2>"TreeSelect"</h2>
        <Demo name="basic" title="Basic">
            <Basic />
        </Demo>
        <Demo name="multiple" title="Multiple">
            <Multiple />
        </Demo>
        <Demo name="checkbox" title="Checkbox">
            <Checkbox />
        </Demo>
        <Demo name="filter" title="Filter">
            <Filter />
        </Demo>
        <Demo name="clearicon" title="Clear Icon">
            <Clear />
        </Demo>
        <Demo name="disabled" title="Disabled">
            <TreeSelect options=files() disabled=true placeholder="Disabled" style=WIDTH />
        </Demo>
        <Demo name="invalid" title="Invalid">
            <TreeSelect options=files() invalid=true placeholder="Select Item" style=WIDTH />
        </Demo>
        <Demo name="template" title="Template">
            <TreeSelect
                options=files()
                value=RwSignal::new(vec!["1-0".to_owned()])
                style=WIDTH
                value_view=Callback::new(|nodes: Vec<TreeNode>| view! { <b data-testid="value-view">{nodes.len()}" selected"</b> }.into_any())
            />
        </Demo>
    }
}
