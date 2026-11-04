//! Поведение Tree в реальном браузере (wasm-bindgen-test): раскрытие, выбор во всех режимах, флажки, фильтр, клавиатура и события.
//!
//! Сценарии идут одним последовательным тестом: фокус общий на страницу. Стили темы не подключены.
#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use prime_tree::{Tree, TreeFilterMode, TreeNode, TreeNodeEvent, TreeSelectionMode};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;
use web_sys::{HtmlElement, KeyboardEvent, KeyboardEventInit};

wasm_bindgen_test_configure!(run_in_browser);

type Log = Arc<Mutex<Vec<String>>>;

fn doc() -> web_sys::Document {
    web_sys::window().unwrap().document().unwrap()
}

fn host() -> HtmlElement {
    let el = doc().create_element("div").unwrap().dyn_into::<HtmlElement>().unwrap();
    doc().body().unwrap().append_child(&el).unwrap();
    el
}

fn q(host: &HtmlElement, sel: &str) -> HtmlElement {
    host.query_selector(sel).unwrap().unwrap_or_else(|| panic!("нет {sel}: {}", host.outer_html())).dyn_into().unwrap()
}

fn has(host: &HtmlElement, sel: &str) -> bool {
    host.query_selector(sel).unwrap().is_some()
}

fn count(host: &HtmlElement, sel: &str) -> u32 {
    host.query_selector_all(sel).unwrap().length()
}

async fn sleep(ms: i32) {
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms).unwrap();
    });
    JsFuture::from(promise).await.unwrap();
}

/// Даёт эффектам примениться.
async fn settle() {
    sleep(40).await;
}

fn node(host: &HtmlElement, key: &str) -> HtmlElement {
    q(host, &format!("li[data-id=\"{key}\"]"))
}

fn content(host: &HtmlElement, key: &str) -> HtmlElement {
    node(host, key).query_selector(":scope > .p-tree-node-content").unwrap().unwrap().dyn_into().unwrap()
}

fn toggle(host: &HtmlElement, key: &str) -> HtmlElement {
    content(host, key).query_selector(".p-tree-node-toggle-button").unwrap().unwrap().dyn_into().unwrap()
}

fn attr(el: &HtmlElement, name: &str) -> Option<String> {
    el.get_attribute(name)
}

fn active_key() -> Option<String> {
    doc().active_element().and_then(|e| e.get_attribute("data-id"))
}

/// Нажатие клавиши на узле и ожидание реактивных обновлений.
async fn key(el: &HtmlElement, code: &str, key: &str) {
    let init = KeyboardEventInit::new();
    init.set_code(code);
    init.set_key(key);
    init.set_bubbles(true);
    init.set_cancelable(true);
    el.dispatch_event(&KeyboardEvent::new_with_keyboard_event_init_dict("keydown", &init).unwrap()).unwrap();
    sleep(15).await;
}

fn n(key: &str, label: &str) -> TreeNode {
    TreeNode::new(key).label(label)
}

/// a{ a1{a11,a12}, a2 }, b, c{ c1 }
fn model() -> Vec<TreeNode> {
    vec![
        n("a", "Alpha").children(vec![n("a1", "Apple").children(vec![n("a11", "Red"), n("a12", "Green")]), n("a2", "Avocado")]),
        n("b", "Beta"),
        n("c", "Gamma").children(vec![n("c1", "Cherry")]),
    ]
}

fn logger(log: &Log, prefix: &'static str) -> Callback<TreeNodeEvent> {
    let log = log.clone();
    Callback::new(move |e: TreeNodeEvent| log.lock().unwrap().push(format!("{prefix}:{}", e.node.key)))
}

async fn toggling_expands_and_collapses() {
    let (host, log) = (host(), Log::default());
    let (l1, l2) = (logger(&log, "expand"), logger(&log, "collapse"));
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Tree id="t" value=model() on_node_expand=l1 on_node_collapse=l2 /> });
    settle().await;
    assert!(!has(&host, "li[data-id=\"a1\"]"));
    toggle(&host, "a").click();
    settle().await;
    assert!(has(&host, "li[data-id=\"a1\"]") && attr(&node(&host, "a"), "aria-expanded").as_deref() == Some("true"));
    assert_eq!(attr(&node(&host, "a1"), "aria-level").as_deref(), Some("2"));
    toggle(&host, "a1").click();
    settle().await;
    assert!(has(&host, "li[data-id=\"a11\"]"));
    toggle(&host, "a").click();
    settle().await;
    assert!(!has(&host, "li[data-id=\"a1\"]"), "свёрнутый узел скрывает детей");
    assert_eq!(log.lock().unwrap().as_slice(), ["expand:a", "expand:a1", "collapse:a"]);
    toggle(&host, "a").click();
    settle().await;
    assert!(has(&host, "li[data-id=\"a11\"]"), "раскрытие вложенного узла сохраняется");
    host.remove();
}

async fn single_and_multiple_selection() {
    let (host, log) = (host(), Log::default());
    let sel = RwSignal::new(Vec::<String>::new());
    let (l1, l2) = (logger(&log, "select"), logger(&log, "unselect"));
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <Tree id="t" value=model() selection_mode=TreeSelectionMode::Single selection=sel on_node_select=l1 on_node_unselect=l2 /> }
    });
    settle().await;
    content(&host, "b").click();
    settle().await;
    assert_eq!(sel.get_untracked(), ["b"]);
    assert_eq!(attr(&node(&host, "b"), "aria-selected").as_deref(), Some("true"));
    assert!(content(&host, "b").class_name().contains("p-tree-node-selected"));
    assert_eq!(attr(&node(&host, "b"), "tabindex").as_deref(), Some("0"), "выбранный узел — точка входа");
    toggle(&host, "a").click();
    settle().await;
    assert_eq!(sel.get_untracked(), ["b"], "клик по кнопке раскрытия выбор не меняет");
    content(&host, "a").click();
    assert_eq!(sel.get_untracked(), ["a"], "единичный выбор заменяет");
    content(&host, "a").click();
    assert!(sel.get_untracked().is_empty(), "повторный клик снимает");
    assert_eq!(log.lock().unwrap().as_slice(), ["select:b", "select:a", "unselect:a"]);
    host.remove();

    let host2 = self::host();
    let sel = RwSignal::new(Vec::<String>::new());
    let _h2 = leptos::mount::mount_to(host2.clone(), move || {
        view! { <Tree id="t" value=model() selection_mode=TreeSelectionMode::Multiple selection=sel /> }
    });
    settle().await;
    content(&host2, "a").click();
    content(&host2, "b").click();
    assert_eq!(sel.get_untracked(), ["a", "b"]);
    content(&host2, "a").click();
    assert_eq!(sel.get_untracked(), ["b"]);
    host2.remove();
}

async fn checkbox_selection_propagates() {
    let host = host();
    let sel = RwSignal::new(Vec::<String>::new());
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <Tree id="t" value=model() selection_mode=TreeSelectionMode::Checkbox selection=sel expanded_keys=RwSignal::new(HashSet::from(["a".to_owned(), "a1".to_owned()])) /> }
    });
    settle().await;
    assert_eq!(count(&host, ".p-tree-node-checkbox"), 7);
    content(&host, "a11").click();
    settle().await;
    assert_eq!(attr(&node(&host, "a11"), "aria-checked").as_deref(), Some("true"));
    assert_eq!(
        attr(&q(&host, "li[data-id=\"a1\"] .p-tree-node-checkbox"), "data-p-partialchecked").as_deref(),
        Some("true"),
        "родитель частично выбран"
    );
    assert!(q(&host, "li[data-id=\"a1\"] .p-tree-node-checkbox").class_name().contains("p-checkbox-checked"));
    content(&host, "a12").click();
    settle().await;
    assert_eq!(attr(&node(&host, "a1"), "aria-checked").as_deref(), Some("true"), "выбраны все дети — выбран родитель");
    content(&host, "a2").click();
    settle().await;
    assert_eq!(attr(&node(&host, "a"), "aria-checked").as_deref(), Some("true"));
    content(&host, "a").click();
    settle().await;
    assert!(sel.get_untracked().is_empty(), "снятие предка снимает всё поддерево");
    host.remove();
}

async fn filter_narrows_the_tree() {
    let (host, log) = (host(), Log::default());
    let l = log.clone();
    let on_filter = Callback::new(move |t: String| l.lock().unwrap().push(t));
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Tree id="t" value=model() filter=true on_filter=on_filter /> });
    settle().await;
    let input: web_sys::HtmlInputElement = q(&host, ".p-tree-filter-input").dyn_into().unwrap();
    input.set_value("gre");
    input.dispatch_event(&web_sys::Event::new("input").unwrap()).unwrap();
    settle().await;
    assert!(has(&host, "li[data-id=\"a12\"]"), "совпавший вложенный узел виден, предки раскрыты");
    assert!(!has(&host, "li[data-id=\"b\"]") && !has(&host, "li[data-id=\"a11\"]"), "остальное скрыто");
    assert_eq!(count(&host, ".p-tree-empty-message"), 0);
    input.set_value("zzz");
    input.dispatch_event(&web_sys::Event::new("input").unwrap()).unwrap();
    settle().await;
    assert!(has(&host, ".p-tree-empty-message"));
    input.set_value("");
    input.dispatch_event(&web_sys::Event::new("input").unwrap()).unwrap();
    settle().await;
    assert!(has(&host, "li[data-id=\"b\"]") && has(&host, "li[data-id=\"c\"]"), "пустое поле возвращает дерево");
    assert_eq!(log.lock().unwrap().as_slice(), ["gre", "zzz", ""]);
    host.remove();

    let host = self::host();
    let _h = leptos::mount::mount_to(
        host.clone(),
        move || view! { <Tree id="t" value=model() filter=true filter_mode=TreeFilterMode::Strict /> },
    );
    settle().await;
    let input: web_sys::HtmlInputElement = q(&host, ".p-tree-filter-input").dyn_into().unwrap();
    input.set_value("apple");
    input.dispatch_event(&web_sys::Event::new("input").unwrap()).unwrap();
    settle().await;
    assert!(has(&host, "li[data-id=\"a1\"]") && !has(&host, "li[data-id=\"a11\"]"), "strict: у совпавшего узла нет несовпавших детей");
    host.remove();
}

async fn keyboard_walks_the_tree() {
    let (host, log) = (host(), Log::default());
    let l = logger(&log, "select");
    let sel = RwSignal::new(Vec::<String>::new());
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <Tree id="t" value=model() selection_mode=TreeSelectionMode::Single selection=sel on_node_select=l /> }
    });
    settle().await;
    assert_eq!(attr(&node(&host, "a"), "tabindex").as_deref(), Some("0"));
    assert_eq!(attr(&node(&host, "b"), "tabindex").as_deref(), Some("-1"));
    node(&host, "a").focus().unwrap();
    key(&node(&host, "a"), "ArrowRight", "ArrowRight").await;
    settle().await;
    assert!(has(&host, "li[data-id=\"a1\"]"), "→ раскрывает узел");
    assert_eq!(active_key().as_deref(), Some("a1"), "и переходит к первому ребёнку");
    assert_eq!(attr(&node(&host, "a1"), "tabindex").as_deref(), Some("0"), "roving tabindex");
    assert_eq!(attr(&node(&host, "a"), "tabindex").as_deref(), Some("-1"));
    key(&node(&host, "a1"), "ArrowDown", "ArrowDown").await;
    assert_eq!(active_key().as_deref(), Some("a2"));
    key(&node(&host, "a2"), "ArrowDown", "ArrowDown").await;
    assert_eq!(active_key().as_deref(), Some("b"));
    key(&node(&host, "b"), "ArrowUp", "ArrowUp").await;
    key(&node(&host, "a2"), "ArrowLeft", "ArrowLeft").await;
    assert_eq!(active_key().as_deref(), Some("a"), "← на листе — к родителю");
    key(&node(&host, "a"), "ArrowLeft", "ArrowLeft").await;
    settle().await;
    assert!(!has(&host, "li[data-id=\"a1\"]"), "← на раскрытом сворачивает");
    key(&node(&host, "a"), "End", "End").await;
    assert_eq!(active_key().as_deref(), Some("c"));
    key(&node(&host, "c"), "Home", "Home").await;
    assert_eq!(active_key().as_deref(), Some("a"));
    key(&node(&host, "a"), "Enter", "Enter").await;
    assert_eq!(sel.get_untracked(), ["a"], "Enter выбирает");
    key(&node(&host, "a"), "Space", " ").await;
    assert!(sel.get_untracked().is_empty(), "Space снимает");
    assert_eq!(log.lock().unwrap().as_slice(), ["select:a"]);
    host.remove();
}

async fn lazy_nodes_and_double_click() {
    let (host, log) = (host(), Log::default());
    let value = RwSignal::new(vec![n("0", "Node 0").leaf(false), n("1", "Node 1")]);
    let l = log.clone();
    let on_expand = Callback::new(move |e: TreeNodeEvent| {
        l.lock().unwrap().push(format!("expand:{}", e.node.key));
        value.update(|v| v[0].children = vec![n("0-0", "Loaded")]);
    });
    let l = log.clone();
    let on_dbl = Callback::new(move |e: TreeNodeEvent| l.lock().unwrap().push(format!("dbl:{}", e.node.key)));
    let _h = leptos::mount::mount_to(
        host.clone(),
        move || view! { <Tree id="t" value=value on_node_expand=on_expand on_node_double_click=on_dbl /> },
    );
    settle().await;
    toggle(&host, "0").click();
    settle().await;
    assert!(has(&host, "li[data-id=\"0-0\"]"), "дети, добавленные после раскрытия, появились");
    assert!(has(&host, "li[data-id=\"0-0\"]") && attr(&node(&host, "0"), "aria-expanded").as_deref() == Some("true"));
    content(&host, "1").dispatch_event(&web_sys::MouseEvent::new("dblclick").unwrap()).unwrap();
    assert_eq!(log.lock().unwrap().as_slice(), ["expand:0", "dbl:1"]);
    host.remove();
}

#[wasm_bindgen_test]
async fn tree_in_the_browser() {
    toggling_expands_and_collapses().await;
    single_and_multiple_selection().await;
    checkbox_selection_propagates().await;
    filter_narrows_the_tree().await;
    keyboard_walks_the_tree().await;
    lazy_nodes_and_double_click().await;
}
