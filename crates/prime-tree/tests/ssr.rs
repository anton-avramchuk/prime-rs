//! Разметка Tree (SSR): корень, список, узлы, ARIA, раскрытие из модели, режимы выбора, флажки, фильтр, пусто, шаблоны, `pt`.
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_core::Pt;
use prime_tree::{Tree, TreeFilterMode, TreeLoadingMode, TreeNode, TreePt, TreeSelectionMode};
use std::collections::HashSet;

fn render(f: impl FnOnce() -> String) -> String {
    Owner::new().with(f)
}

fn n(key: &str, label: &str) -> TreeNode {
    TreeNode::new(key).label(label)
}

fn model() -> Vec<TreeNode> {
    vec![
        n("0", "Documents").children(vec![n("0-0", "Work").children(vec![n("0-0-0", "Expenses")]), n("0-1", "Home")]),
        n("1", "Events"),
        n("2", "Lazy").leaf(false),
    ]
}

#[test]
fn markup_has_root_list_and_nodes() {
    let html = render(|| view! { <Tree id="t" value=model() /> }.to_html());
    assert!(html.contains("id=\"t\"") && html.contains("p-tree p-component"), "{html}");
    assert!(!html.contains("p-tree-selectable") && !html.contains("p-tree-loading"), "{html}");
    assert!(
        html.contains("class=\"p-tree-root\"") && html.contains("class=\"p-tree-root-children\"") && html.contains("role=\"tree\""),
        "{html}"
    );
    assert_eq!(html.matches("role=\"treeitem\"").count(), 3, "дети свёрнутого узла не выводятся: {html}");
    assert!(html.contains("data-id=\"0\"") && html.contains("data-id=\"1\"") && !html.contains("data-id=\"0-0\""), "{html}");
    assert!(
        html.contains("p-tree-node-content") && html.contains("p-tree-node-toggle-button") && html.contains("p-tree-node-label"),
        "{html}"
    );
    assert!(html.contains("p-tree-node p-tree-node-leaf"), "лист помечен: {html}");
    assert_eq!(html.matches("p-tree-node-leaf").count(), 1, "узел с leaf=false не лист: {html}");
    assert!(html.contains("aria-level=\"1\"") && html.contains("aria-posinset=\"3\"") && html.contains("aria-setsize=\"2\""), "{html}");
    assert!(html.contains("padding-left: 0rem"), "{html}");
}

#[test]
fn expanded_nodes_show_children() {
    let mut m = model();
    m[0].expanded = true;
    m[0].children[0].expanded = true;
    let html = render(|| view! { <Tree id="t" value=m /> }.to_html());
    assert_eq!(html.matches("role=\"treeitem\"").count(), 6, "{html}");
    assert_eq!(html.matches("class=\"p-tree-node-children\"").count(), 2, "{html}");
    assert!(html.contains("role=\"group\"") && html.contains("aria-level=\"3\"") && html.contains("padding-left: 3rem"), "{html}");
    assert!(html.contains("aria-expanded=\"true\"") && html.contains("aria-expanded=\"false\""), "{html}");
}

#[test]
fn expanded_keys_signal_is_honoured() {
    Owner::new().with(|| {
        let keys = RwSignal::new(HashSet::from(["0".to_owned()]));
        let html = view! { <Tree id="t" value=model() expanded_keys=keys /> }.to_html();
        assert!(html.contains("data-id=\"0-0\"") && html.contains("data-id=\"0-1\""), "{html}");
    });
}

#[test]
fn single_selection_markup() {
    Owner::new().with(|| {
        let sel = RwSignal::new(vec!["1".to_owned()]);
        let html = view! { <Tree id="t" value=model() selection_mode=TreeSelectionMode::Single selection=sel /> }.to_html();
        assert!(html.contains("p-tree p-component p-tree-selectable"), "{html}");
        assert!(
            html.contains("aria-selected=\"true\"") && html.contains("aria-selected=\"false\"") && !html.contains("aria-checked"),
            "{html}"
        );
        assert_eq!(html.matches("p-tree-node-selected").count(), 1, "{html}");
        assert!(html.contains("p-tree-node-selectable"), "{html}");
    });
}

#[test]
fn checkbox_markup_and_partial_state() {
    Owner::new().with(|| {
        let mut m = model();
        m[0].expanded = true;
        let sel = RwSignal::new(vec!["0-1".to_owned()]);
        let html = view! { <Tree id="t" value=m selection_mode=TreeSelectionMode::Checkbox selection=sel /> }.to_html();
        assert!(html.contains("aria-checked=\"true\"") && !html.contains("aria-selected"), "{html}");
        assert!(html.contains("p-checkbox") && html.contains("p-tree-node-checkbox") && html.contains("type=\"checkbox\""), "{html}");
        assert!(html.contains("data-p-partialchecked=\"true\""), "родитель выбранного ребёнка частично выбран: {html}");
        assert!(html.contains("p-checkbox-checked"), "{html}");
        assert!(!html.contains("p-tree-node-selected"), "без highlight_on_select подсветки нет: {html}");
        let sel = RwSignal::new(vec!["1".to_owned()]);
        let html =
            view! { <Tree id="t" value=model() selection_mode=TreeSelectionMode::Checkbox selection=sel highlight_on_select=true /> }
                .to_html();
        assert_eq!(html.matches("p-tree-node-selected").count(), 1, "с highlight_on_select выбранный узел подсвечен: {html}");
    });
}

#[test]
fn not_selectable_node_has_no_selectable_class() {
    let m = vec![n("a", "A"), n("b", "B").selectable(false)];
    let html = render(|| view! { <Tree id="t" value=m selection_mode=TreeSelectionMode::Multiple /> }.to_html());
    assert_eq!(html.matches("p-tree-node-selectable").count(), 1, "{html}");
}

#[test]
fn tab_stop_is_first_node_then_selected() {
    let html = render(|| view! { <Tree id="t" value=model() /> }.to_html());
    assert_eq!(html.matches("tabindex=\"0\"").count(), 1, "{html}");
    assert!(html.find("tabindex=\"0\"").unwrap() < html.find("data-id=\"1\"").unwrap(), "первый узел: {html}");
    Owner::new().with(|| {
        let sel = RwSignal::new(vec!["1".to_owned()]);
        let html = view! { <Tree id="t" value=model() selection_mode=TreeSelectionMode::Single selection=sel /> }.to_html();
        assert_eq!(html.matches("tabindex=\"0\"").count(), 1, "{html}");
        assert!(html.find("tabindex=\"0\"").unwrap() > html.find("data-id=\"0\"").unwrap(), "выбранный узел: {html}");
    });
}

#[test]
fn empty_loading_and_filter() {
    let html = render(|| view! { <Tree id="t" value={Vec::<TreeNode>::new()} /> }.to_html());
    assert!(html.contains("p-tree-empty-message") && html.contains("No results found") && !html.contains("p-tree-root"), "{html}");
    let html = render(|| view! { <Tree id="t" value={Vec::<TreeNode>::new()} empty_message="Nothing" /> }.to_html());
    assert!(html.contains("Nothing"), "{html}");
    let html = render(|| view! { <Tree id="t" value=model() loading=true /> }.to_html());
    assert!(
        html.contains("p-tree-loading") && html.contains("p-tree-mask p-overlay-mask") && html.contains("p-tree-loading-icon"),
        "{html}"
    );
    assert!(!html.contains("p-tree-empty-message"), "{html}");
    let html = render(|| {
        view! { <Tree id="t" value=model() filter=true filter_placeholder="Search" filter_mode=TreeFilterMode::Strict /> }.to_html()
    });
    assert!(
        html.contains("p-tree-filter-input")
            && html.contains("type=\"search\"")
            && html.contains("placeholder=\"Search\"")
            && html.contains("p-inputicon"),
        "{html}"
    );
}

#[test]
fn scroll_height_and_aria() {
    let html = render(|| view! { <Tree id="t" value=model() scroll_height="250px" aria_label="Files" indentation=2.0 /> }.to_html());
    assert!(html.contains("max-height: 250px") && html.contains("aria-label=\"Files\""), "{html}");
    let html = render(|| view! { <Tree id="t" value=model() scroll_height="flex" /> }.to_html());
    assert!(html.contains("p-tree-flex-scrollable") && !html.contains("max-height"), "{html}");
}

#[test]
fn loading_icon_mode_shows_spinner_in_toggle() {
    let m = vec![n("0", "Node").leaf(false).loading(true)];
    let html = render(|| view! { <Tree id="t" value=m loading_mode=TreeLoadingMode::Icon /> }.to_html());
    assert!(html.contains("p-icon-spin") || html.contains("p-tree-node-toggle-icon"), "{html}");
    assert!(!html.contains("visibility: hidden"), "{html}");
}

#[test]
fn template_icons_and_pt() {
    let node = Callback::new(|n: TreeNode| view! { <b class="custom">{format!("{}!", n.label.unwrap_or_default())}</b> }.into_any());
    let mut m = vec![
        n("0", "Folder")
            .children(vec![n("0-0", "File")])
            .icons(|| view! { <i class="open" /> }, || view! { <i class="shut" /> })
            .expanded(true),
    ];
    m[0].children[0].icon = Some(ViewFn::from(|| view! { <i class="own" /> }));
    let pt = TreePt { root: Pt::new().class("r").attr("data-x", "1"), node: Pt::new().class("nn"), ..TreePt::default() };
    let html = render(|| {
        view! { <Tree id="t" value=m node=node style="width: 20rem" style_class="mine" pt=pt toggler_icon=|| view! { <u class="tg" /> } /> }
            .to_html()
    });
    assert!(html.contains("Folder!") && html.contains("File!") && html.contains("class=\"custom\""), "{html}");
    assert!(html.contains("class=\"open\"") && !html.contains("class=\"shut\"") && html.contains("class=\"own\""), "{html}");
    assert!(
        html.contains("mine") && html.contains("data-x=\"1\"") && html.contains("width: 20rem") && html.contains("class=\"tg\""),
        "{html}"
    );
    assert!(html.contains("p-tree-node nn") || html.contains(" nn"), "{html}");
}
