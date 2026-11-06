//! Разметка TreeTable (SSR): строки дерева с уровнями, кнопки раскрытия, сортировка по уровням, пагинатор корней,
//! выбор и флажки, пусто/загрузка, модификаторы, `pt`.
#![cfg(feature = "ssr")]
#![recursion_limit = "512"]

use leptos::prelude::*;
use prime_core::{Pt, Size};
use prime_table::Column;
use prime_table::model::SortMeta;
use prime_treetable::{TreeNode, TreeSelectionMode, TreeTable, TreeTablePt};

#[derive(Clone, Debug)]
struct Item {
    name: &'static str,
    size: u32,
}

fn it(name: &'static str, size: u32) -> Item {
    Item { name, size }
}

/// Documents { Work { Expenses.doc, Resume.doc }, Home { Invoices.txt } }, Pictures { barcelona.png }, Movies
fn tree() -> Vec<TreeNode<Item>> {
    vec![
        TreeNode::new("0", it("Documents", 75)).children(vec![
            TreeNode::new("0-0", it("Work", 55))
                .children(vec![TreeNode::new("0-0-0", it("Expenses.doc", 30)), TreeNode::new("0-0-1", it("Resume.doc", 25))]),
            TreeNode::new("0-1", it("Home", 20)).children(vec![TreeNode::new("0-1-0", it("Invoices.txt", 20))]),
        ]),
        TreeNode::new("1", it("Pictures", 150)).children(vec![TreeNode::new("1-0", it("barcelona.png", 90))]),
        TreeNode::new("2", it("Movies", 1000)),
    ]
}

fn columns() -> Vec<Column<Item>> {
    vec![
        Column::text("name", "Name", |i: &Item| i.name.to_owned()).sortable(|a, b| a.name.cmp(b.name)),
        Column::text("size", "Size", |i: &Item| format!("{}kb", i.size)).sortable(|a, b| a.size.cmp(&b.size)),
    ]
}

fn render(f: impl FnOnce() -> String) -> String {
    Owner::new().with(f)
}

fn pos(html: &str, needle: &str) -> usize {
    html.find(needle).unwrap_or_else(|| panic!("нет {needle}: {html}"))
}

fn in_order(html: &str, names: &[&str]) -> bool {
    let order: Vec<usize> = names.iter().map(|n| pos(html, &format!(">{n}<"))).collect();
    order.windows(2).all(|w| w[0] < w[1])
}

/// Открывающий тег элемента, в котором встречается подстрока.
fn tag_of<'a>(html: &'a str, needle: &str) -> &'a str {
    let at = html.find(needle).unwrap_or_else(|| panic!("нет {needle}: {html}"));
    let start = html[..at].rfind('<').unwrap();
    let end = at + html[at..].find('>').unwrap();
    &html[start..=end]
}

#[test]
fn collapsed_tree_shows_roots() {
    let html = render(|| view! { <TreeTable value=tree() columns=columns() /> }.to_html());
    assert!(html.contains("p-treetable p-component"), "{html}");
    assert!(
        html.contains("p-treetable-table-container") && html.contains("p-treetable-thead") && html.contains("p-treetable-tbody"),
        "{html}"
    );
    assert!(html.contains(">Name<") && html.contains(">Size<"), "{html}");
    assert!(in_order(&html, &["Documents", "Pictures", "Movies"]), "{html}");
    assert!(!html.contains(">Work<") && !html.contains(">barcelona.png<"), "{html}");
    assert_eq!(html.matches("p-treetable-toggler").count(), 3, "кнопка в каждой строке: {html}");
    assert!(html.contains("role=\"treegrid\""), "{html}");
}

#[test]
fn toggler_visibility_and_indent() {
    let html = render(|| {
        let expanded = RwSignal::new(vec!["0".to_owned(), "0-0".to_owned()]);
        view! { <TreeTable value=tree() columns=columns() expanded_keys=expanded /> }.to_html()
    });
    assert!(in_order(&html, &["Documents", "Work", "Expenses.doc", "Resume.doc", "Home", "Pictures", "Movies"]), "{html}");
    assert!(!html.contains(">Invoices.txt<"), "дети нераскрытого Home скрыты: {html}");
    // у листа кнопка скрыта, у раскрываемого видна; отступ по уровню 16 px
    let leaf = &html[..pos(&html, ">Expenses.doc<")];
    let leaf_toggler = &leaf[leaf.rfind("p-treetable-toggler").unwrap()..];
    assert!(leaf_toggler.contains("visibility: hidden") && leaf_toggler.contains("margin-inline-start: 32px"), "{leaf_toggler}");
    let work = &html[..pos(&html, ">Work<")];
    let work_toggler = &work[work.rfind("p-treetable-toggler").unwrap()..];
    assert!(work_toggler.contains("visibility: visible") && work_toggler.contains("margin-inline-start: 16px"), "{work_toggler}");
    assert!(html.contains("aria-expanded=\"true\"") && html.contains("aria-expanded=\"false\""), "{html}");
    assert!(html.contains("aria-level=\"3\"") && html.contains("aria-level=\"1\""), "{html}");
}

#[test]
fn node_flag_expands_initially() {
    let nodes = vec![TreeNode::new("a", it("Docs", 1)).expanded(true).children(vec![TreeNode::new("b", it("Child", 1))])];
    let html = render(|| view! { <TreeTable value=nodes columns=columns() /> }.to_html());
    assert!(html.contains(">Child<"), "{html}");
}

#[test]
fn sort_applies_per_level() {
    let html = render(|| {
        let expanded = RwSignal::new(vec!["0".to_owned()]);
        let sort = RwSignal::new(vec![SortMeta::new("size", -1)]);
        view! { <TreeTable value=tree() columns=columns() expanded_keys=expanded sort=sort /> }.to_html()
    });
    assert!(in_order(&html, &["Movies", "Pictures", "Documents", "Work", "Home"]), "корни и дети по убыванию размера: {html}");
    assert!(
        html.contains("aria-sort=\"descending\"") && html.contains("p-treetable-column-sorted") && html.contains("p-sortable-column"),
        "{html}"
    );
    assert_eq!(html.matches("p-treetable-sort-icon").count(), 2, "{html}");
}

#[test]
fn paginator_counts_roots_not_rows() {
    let html = render(|| {
        let expanded = RwSignal::new(vec!["0".to_owned()]);
        view! { <TreeTable value=tree() columns=columns() expanded_keys=expanded paginator=true default_rows=1usize /> }.to_html()
    });
    assert!(html.contains("p-treetable-paginator-bottom") && html.contains("p-paginator"), "{html}");
    assert!(
        html.contains(">Documents<") && html.contains(">Work<") && !html.contains(">Pictures<"),
        "первая страница — один корень с потомками: {html}"
    );
    let html = render(|| {
        let first = RwSignal::new(1usize);
        view! { <TreeTable value=tree() columns=columns() paginator=true default_rows=1usize first=first /> }.to_html()
    });
    assert!(html.contains(">Pictures<") && !html.contains(">Documents<"), "{html}");
}

#[test]
fn row_selection_and_hover() {
    let html = render(|| {
        let selection = RwSignal::new(vec!["1".to_owned()]);
        view! { <TreeTable value=tree() columns=columns() selection_mode=Some(TreeSelectionMode::Single) selection=selection /> }.to_html()
    });
    assert!(html.contains("p-treetable-hoverable-rows"), "{html}");
    assert_eq!(html.matches("p-treetable-row-selected").count(), 1, "{html}");
    assert_eq!(html.matches("aria-selected=\"true\"").count(), 1, "{html}");
    assert_eq!(html.matches("aria-selected=\"false\"").count(), 2, "{html}");
}

#[test]
fn checkbox_selection_states() {
    let html = render(|| {
        let expanded = RwSignal::new(vec!["0".to_owned(), "0-0".to_owned()]);
        let selection = RwSignal::new(vec!["0-0-0".to_owned()]);
        view! { <TreeTable value=tree() columns=columns() expanded_keys=expanded selection=selection selection_mode=Some(TreeSelectionMode::Checkbox) /> }.to_html()
    });
    assert!(!html.contains("p-treetable-hoverable-rows"), "в режиме флажков строки не выбираются кликом: {html}");
    assert_eq!(html.matches("p-treetable-node-checkbox").count(), 7, "флажок в каждой видимой строке: {html}");
    assert_eq!(html.matches("p-checkbox-checked").count(), 1, "выбран только лист Expenses.doc: {html}");
    let partial_work = tag_of(&html, "p-treetable-node-checkbox");
    assert!(partial_work.contains("p-checkbox"), "{partial_work}");
}

#[test]
fn modifiers_empty_loading_and_slots() {
    let html = render(|| view! { <TreeTable value=tree() columns=columns() show_gridlines=true size=Size::Small /> }.to_html());
    assert!(html.contains("p-treetable-gridlines") && html.contains("p-treetable-sm"), "{html}");
    let html = render(|| view! { <TreeTable value={Vec::<TreeNode<Item>>::new()} columns=columns() /> }.to_html());
    assert!(html.contains("p-treetable-empty-message") && html.contains("No results found") && html.contains("colspan=\"2\""), "{html}");
    let html = render(|| view! { <TreeTable value=tree() columns=columns() loading=true /> }.to_html());
    assert!(html.contains("p-treetable-mask p-overlay-mask") && html.contains("p-treetable-loading-icon"), "{html}");
    let columns = vec![Column::text("name", "Name", |i: &Item| i.name.to_owned()).footer("Total")];
    let html = render(|| {
        view! {
            <TreeTable
                value=tree()
                columns=columns.clone()
                caption=ViewFn::from(|| view! { <b>"File Viewer"</b> })
                summary=ViewFn::from(|| view! { <i>"Summary"</i> })
            />
        }
        .to_html()
    });
    assert!(html.contains("p-treetable-header") && html.contains("<b>File Viewer</b>") && html.contains("p-treetable-footer"), "{html}");
    assert!(html.contains("p-treetable-tfoot") && html.contains(">Total<"), "{html}");
}

#[test]
fn style_and_pass_through() {
    let pt = TreeTablePt { root: Pt::new().class("root-x"), thead: Pt::new().class("thead-x"), ..Default::default() };
    let html = render(|| {
        view! { <TreeTable value=tree() columns=columns() style="width: 40rem" style_class="mine" table_style="min-width: 30rem" pt=pt /> }
            .to_html()
    });
    assert!(
        html.contains("root-x") && html.contains("mine") && html.contains("width: 40rem") && html.contains("min-width: 30rem"),
        "{html}"
    );
    assert!(html.contains("p-treetable-thead thead-x"), "{html}");
}
