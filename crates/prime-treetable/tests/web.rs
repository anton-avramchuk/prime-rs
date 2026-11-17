//! TreeTable behavior in a real browser (wasm-bindgen-test): expanding and collapsing, per-level sorting,
//! selection by click and Ctrl, checkboxes with propagation and partial state, root paginator.
//! Scenarios run as one sequential test.
#![cfg(target_arch = "wasm32")]
#![recursion_limit = "512"]

use leptos::prelude::*;
use prime_table::Column;
use prime_treetable::{TreeNode, TreeSelectionMode, TreeTable};
use std::sync::{Arc, Mutex};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;
use web_sys::{HtmlElement, MouseEvent, MouseEventInit};

wasm_bindgen_test_configure!(run_in_browser);

#[derive(Clone, Debug)]
struct Item {
    name: &'static str,
    size: u32,
}

fn it(name: &'static str, size: u32) -> Item {
    Item { name, size }
}

fn tree() -> Vec<TreeNode<Item>> {
    vec![
        TreeNode::new("0", it("Documents", 75)).children(vec![
            TreeNode::new("0-0", it("Work", 55))
                .children(vec![TreeNode::new("0-0-0", it("Expenses.doc", 30)), TreeNode::new("0-0-1", it("Resume.doc", 25))]),
            TreeNode::new("0-1", it("Home", 20)),
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

fn doc() -> web_sys::Document {
    web_sys::window().unwrap().document().unwrap()
}

fn host() -> HtmlElement {
    let el = doc().create_element("div").unwrap().dyn_into::<HtmlElement>().unwrap();
    doc().body().unwrap().append_child(&el).unwrap();
    el
}

async fn sleep(ms: i32) {
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms).unwrap();
    });
    JsFuture::from(promise).await.unwrap();
}

async fn settle() {
    sleep(40).await;
}

fn all(host: &HtmlElement, selector: &str) -> Vec<HtmlElement> {
    let list = host.query_selector_all(selector).unwrap();
    (0..list.length()).map(|i| list.item(i).unwrap().dyn_into().unwrap()).collect()
}

fn names(host: &HtmlElement) -> String {
    all(host, "tbody > tr > td:nth-child(1)")
        .iter()
        .map(|e| e.text_content().unwrap_or_default().trim().to_owned())
        .collect::<Vec<_>>()
        .join(" ")
}

fn click_with(el: &HtmlElement, ctrl: bool) {
    let init = MouseEventInit::new();
    init.set_bubbles(true);
    init.set_cancelable(true);
    init.set_ctrl_key(ctrl);
    el.dispatch_event(&MouseEvent::new_with_mouse_event_init_dict("click", &init).unwrap()).unwrap();
}

fn toggler(host: &HtmlElement, row: usize) -> HtmlElement {
    all(host, ".p-treetable-toggler")[row].clone()
}

async fn expand_collapse_and_sort() {
    let (host, log) = (host(), Arc::new(Mutex::new(Vec::<String>::new())));
    let (l1, l2) = (log.clone(), log.clone());
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! {
            <TreeTable
                value=tree()
                columns=columns()
                on_node_expand=Callback::new(move |i: Item| l1.lock().unwrap().push(format!("expand:{}", i.name)))
                on_node_collapse=Callback::new(move |i: Item| l2.lock().unwrap().push(format!("collapse:{}", i.name)))
            />
        }
    });
    settle().await;
    assert_eq!(names(&host), "Documents Pictures Movies");
    toggler(&host, 0).click();
    settle().await;
    assert_eq!(names(&host), "Documents Work Home Pictures Movies");
    assert_eq!(all(&host, "tbody > tr")[0].get_attribute("aria-expanded").as_deref(), Some("true"));
    toggler(&host, 1).click();
    settle().await;
    assert_eq!(names(&host), "Documents Work Expenses.doc Resume.doc Home Pictures Movies");
    let leaf = toggler(&host, 2);
    assert!(leaf.get_attribute("style").unwrap().contains("visibility: hidden"), "a leaf has no button");
    toggler(&host, 0).click();
    settle().await;
    assert_eq!(names(&host), "Documents Pictures Movies", "collapsing hides the whole subtree");
    assert_eq!(log.lock().unwrap().as_slice(), ["expand:Documents", "expand:Work", "collapse:Documents"]);

    // sorting applies at every level
    toggler(&host, 0).click();
    settle().await;
    all(&host, "thead th")[1].click();
    settle().await;
    assert_eq!(names(&host), "Documents Home Work Resume.doc Expenses.doc Pictures Movies", "ascending by size at every level");
    all(&host, "thead th")[1].click();
    settle().await;
    assert_eq!(names(&host), "Movies Pictures Documents Work Expenses.doc Resume.doc Home", "descending by size");
    host.remove();
}

async fn selection_by_click() {
    let host = host();
    let selection = RwSignal::new(Vec::new());
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <TreeTable value=tree() columns=columns() selection_mode=Some(TreeSelectionMode::Multiple) meta_key_selection=true selection=selection /> }
    });
    settle().await;
    click_with(&all(&host, "tbody > tr")[0], false);
    settle().await;
    assert_eq!(selection.get_untracked(), ["0"]);
    assert!(all(&host, "tbody > tr")[0].class_name().contains("p-treetable-row-selected"));
    click_with(&all(&host, "tbody > tr")[2], true);
    settle().await;
    assert_eq!(selection.get_untracked(), ["0", "2"]);
    click_with(&all(&host, "tbody > tr")[0], true);
    settle().await;
    assert_eq!(selection.get_untracked(), ["2"], "Ctrl unselects the selected node");
    // a click on the expand button does not select the node
    toggler(&host, 1).click();
    settle().await;
    assert_eq!(selection.get_untracked(), ["2"]);
    host.remove();
}

async fn checkbox_propagation() {
    let (host, log) = (host(), Arc::new(Mutex::new(Vec::<String>::new())));
    let (l1, l2) = (log.clone(), log.clone());
    let selection = RwSignal::new(Vec::new());
    let expanded = RwSignal::new(vec!["0".to_owned(), "0-0".to_owned()]);
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! {
            <TreeTable
                value=tree()
                columns=columns()
                selection_mode=Some(TreeSelectionMode::Checkbox)
                selection=selection
                expanded_keys=expanded
                on_node_select=Callback::new(move |i: Item| l1.lock().unwrap().push(format!("select:{}", i.name)))
                on_node_unselect=Callback::new(move |i: Item| l2.lock().unwrap().push(format!("unselect:{}", i.name)))
            />
        }
    });
    settle().await;
    assert_eq!(names(&host), "Documents Work Expenses.doc Resume.doc Home Pictures Movies");
    let boxes = || all(&host, "tbody .p-treetable-node-checkbox input");
    boxes()[2].click(); // Expenses.doc
    settle().await;
    assert_eq!(selection.get_untracked(), ["0-0-0"]);
    let rows = all(&host, "tbody .p-treetable-node-checkbox");
    assert!(!rows[1].class_name().contains("p-checkbox-checked"), "Work: partial");
    assert!(host.query_selector("tbody tr:nth-child(2) .p-checkbox-icon").unwrap().is_some(), "partially selected Work has a state icon");
    boxes()[3].click(); // Resume.doc
    settle().await;
    assert_eq!(selection.get_untracked(), ["0-0-0", "0-0-1", "0-0"], "when both children are selected, Work is selected");
    assert!(all(&host, "tbody .p-treetable-node-checkbox")[1].class_name().contains("p-checkbox-checked"));
    boxes()[0].click(); // Documents: Work children are selected, but Home is not
    settle().await;
    let sel = selection.get_untracked();
    assert!(sel.contains(&"0".to_owned()) && sel.contains(&"0-1".to_owned()), "selecting the root selects the whole subtree: {sel:?}");
    boxes()[0].click();
    settle().await;
    assert!(selection.get_untracked().is_empty(), "unselecting the root unselects the subtree");
    let log = log.lock().unwrap().clone();
    assert_eq!(&log[..2], ["select:Expenses.doc", "select:Resume.doc"], "{log:?}");
    host.remove();
}

async fn paginator_over_roots() {
    let host = host();
    let first = RwSignal::new(0usize);
    let expanded = RwSignal::new(vec!["0".to_owned()]);
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <TreeTable value=tree() columns=columns() paginator=true default_rows=2usize first=first expanded_keys=expanded /> }
    });
    settle().await;
    assert_eq!(names(&host), "Documents Work Home Pictures", "two roots with nested rows");
    all(&host, "button.p-paginator-next")[0].click();
    settle().await;
    assert_eq!(first.get_untracked(), 2);
    assert_eq!(names(&host), "Movies");
    all(&host, "thead th")[0].click();
    settle().await;
    assert_eq!(first.get_untracked(), 0, "sorting returns to the first page");
    host.remove();
}

#[wasm_bindgen_test]
async fn treetable_in_the_browser() {
    expand_collapse_and_sort().await;
    selection_by_click().await;
    checkbox_propagation().await;
    paginator_over_roots().await;
}
