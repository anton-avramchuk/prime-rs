//! Страница Tree: демонстрации `basic`, `single`, `multiple`, `checkbox`, `controlled`, `event`, `filter`, `lazy` и `template`
//! из showcase PrimeNG (`apps/showcase/doc/tree`; перетаскивание, виртуальная прокрутка и контекстное меню не переносятся).

use super::Demo;
use leptos::prelude::*;
use prime_button::{Button, ButtonVariant};
use prime_toast::{Toast, ToastMessage, ToastSeverity, provide_message_service};
use prime_tree::{Tree, TreeFilterMode, TreeLoadingMode, TreeNode, TreeNodeEvent, TreeSelectionMode};
use std::collections::HashSet;

fn n(key: &str, label: &str) -> TreeNode {
    TreeNode::new(key).label(label)
}

fn files() -> Vec<TreeNode> {
    vec![
        n("0", "Documents").children(vec![
            n("0-0", "Work").children(vec![n("0-0-0", "Expenses.doc"), n("0-0-1", "Resume.doc")]),
            n("0-1", "Home").children(vec![n("0-1-0", "Invoices.txt")]),
        ]),
        n("1", "Pictures").children(vec![n("1-0", "barcelona.jpg"), n("1-1", "logo.jpg"), n("1-2", "primeui.png")]),
        n("2", "Movies").children(vec![
            n("2-0", "Al Pacino").children(vec![n("2-0-0", "Scarface.mp4"), n("2-0-1", "Serpico.mp4")]),
            n("2-1", "Robert De Niro").children(vec![n("2-1-0", "Goodfellas.mp4"), n("2-1-1", "Untouchables.mp4")]),
        ]),
    ]
}

fn width() -> &'static str {
    "width: 100%; max-width: 30rem"
}

#[component]
fn SingleDemo() -> impl IntoView {
    let sel = RwSignal::new(Vec::<String>::new());
    view! { <Tree id="single_tree" style=width() value=files() selection_mode=TreeSelectionMode::Single selection=sel /> }
}

#[component]
fn MultipleDemo() -> impl IntoView {
    let sel = RwSignal::new(Vec::<String>::new());
    view! {
        <div style=width()>
            <p style="margin: 0 0 0.5rem">"Выбор с Ctrl/Meta"</p>
            <Tree id="multiple_tree" value=files() selection_mode=TreeSelectionMode::Multiple selection=sel meta_key_selection=true />
        </div>
    }
}

#[component]
fn CheckboxDemo() -> impl IntoView {
    let sel = RwSignal::new(Vec::<String>::new());
    view! { <Tree id="checkbox_tree" style=width() value=files() selection_mode=TreeSelectionMode::Checkbox selection=sel /> }
}

#[component]
fn ControlledDemo() -> impl IntoView {
    let keys = RwSignal::new(HashSet::<String>::new());
    let all = || {
        let mut out = HashSet::new();
        fn walk(nodes: &[TreeNode], out: &mut HashSet<String>) {
            for n in nodes {
                if !n.children.is_empty() {
                    out.insert(n.key.clone());
                }
                walk(&n.children, out);
            }
        }
        walk(&files(), &mut out);
        out
    };
    view! {
        <div style="display: flex; flex-direction: column; gap: 1rem; width: 100%; max-width: 30rem">
            <div style="display: flex; gap: 0.5rem">
                <Button label="Expand all" variant=ButtonVariant::Text on_click=Callback::new(move |_| keys.set(all())) />
                <Button label="Collapse all" variant=ButtonVariant::Text on_click=Callback::new(move |_| keys.set(HashSet::new())) />
            </div>
            <Tree id="controlled_tree" value=files() expanded_keys=keys />
        </div>
    }
}

#[component]
fn EventDemo() -> impl IntoView {
    let toast = provide_message_service();
    let say = move |severity: ToastSeverity, summary: &'static str| {
        Callback::new(move |e: TreeNodeEvent| {
            toast.add(ToastMessage::new(severity.clone(), summary, e.node.label.unwrap_or_default()).life(2000))
        })
    };
    view! {
        <Toast />
        <Tree
            id="event_tree"
            style=width()
            value=files()
            selection_mode=TreeSelectionMode::Single
            on_node_select=say(ToastSeverity::Success, "Node Selected")
            on_node_unselect=say(ToastSeverity::Warn, "Node Unselected")
            on_node_expand=say(ToastSeverity::Info, "Node Expanded")
            on_node_collapse=say(ToastSeverity::Info, "Node Collapsed")
        />
    }
}

#[component]
fn LazyDemo() -> impl IntoView {
    let nodes = RwSignal::new((0..3).map(|i| n(&i.to_string(), &format!("Node {i}")).leaf(false)).collect::<Vec<_>>());
    let on_expand = Callback::new(move |e: TreeNodeEvent| {
        if !e.node.children.is_empty() {
            return;
        }
        let key = e.node.key.clone();
        nodes.update(|v| set_loading(v, &key, true));
        set_timeout(
            move || {
                nodes.update(|v| {
                    set_loading(v, &key, false);
                    set_children(
                        v,
                        &key,
                        vec![n(&format!("{key}-0"), &format!("{key} - 0")), n(&format!("{key}-1"), &format!("{key} - 1")).leaf(false)],
                    );
                });
            },
            std::time::Duration::from_millis(500),
        );
    });
    view! { <Tree id="lazy_tree" style=width() value=nodes loading_mode=TreeLoadingMode::Icon on_node_expand=on_expand /> }
}

fn set_loading(nodes: &mut [TreeNode], key: &str, loading: bool) {
    for node in nodes {
        if node.key == key {
            node.loading = loading;
        }
        set_loading(&mut node.children, key, loading);
    }
}

fn set_children(nodes: &mut [TreeNode], key: &str, children: Vec<TreeNode>) {
    for node in nodes {
        if node.key == key {
            node.children = children.clone();
        }
        set_children(&mut node.children, key, children.clone());
    }
}

fn template_item(node: TreeNode) -> AnyView {
    let bold = node.type_.as_deref() == Some("url");
    let label = node.label.unwrap_or_default();
    if bold {
        view! { <a href="https://vuejs.org/" target="_blank" style="font-weight: 600" data-testid="url-node">{label}</a> }.into_any()
    } else {
        view! { <b data-testid="default-node">{label}</b> }.into_any()
    }
}

/// Страница `/tree`.
#[component]
pub fn TreePage() -> impl IntoView {
    view! {
        <Demo name="basic" title="Basic">
            <Tree id="basic_tree" style=width() value=files() />
        </Demo>
        <Demo name="single" title="Single Selection">
            <SingleDemo />
        </Demo>
        <Demo name="multiple" title="Multiple Selection">
            <MultipleDemo />
        </Demo>
        <Demo name="checkbox" title="Checkbox Selection">
            <CheckboxDemo />
        </Demo>
        <Demo name="controlled" title="Controlled">
            <ControlledDemo />
        </Demo>
        <Demo name="event" title="Events">
            <EventDemo />
        </Demo>
        <Demo name="filter" title="Filter">
            <Tree id="filter_tree" style=width() value=files() filter=true filter_placeholder="Lenient Filter" filter_mode=TreeFilterMode::Lenient />
            <Tree id="strict_tree" style=width() value=files() filter=true filter_placeholder="Strict Filter" filter_mode=TreeFilterMode::Strict />
        </Demo>
        <Demo name="lazy" title="Lazy">
            <LazyDemo />
        </Demo>
        <Demo name="template" title="Template">
            <Tree
                id="template_tree"
                style=width()
                value=vec![n("0", "Web").children(vec![n("0-0", "About"), n("0-1", "Vue").type_("url")])]
                node=Callback::new(template_item)
            />
        </Demo>
    }
}
