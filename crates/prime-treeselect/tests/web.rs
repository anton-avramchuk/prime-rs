//! TreeSelect behavior in a real browser (wasm-bindgen-test): opening and closing the panel, selection in three modes,
//! expanding ancestors, clear, outside click, keyboard, filter and events.
//!
//! Scenarios run as one sequential test: window listeners and focus are shared per page. Theme styles are not attached.
#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use prime_tree::{TreeNode, TreeSelectionMode};
use prime_treeselect::{TreeSelect, TreeSelectDisplay};
use std::sync::{Arc, Mutex};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;
use web_sys::{HtmlElement, KeyboardEvent, KeyboardEventInit, MouseEvent, MouseEventInit};

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
    host.query_selector(sel).unwrap().unwrap_or_else(|| panic!("no {sel}: {}", host.outer_html())).dyn_into().unwrap()
}

fn has(host: &HtmlElement, sel: &str) -> bool {
    host.query_selector(sel).unwrap().is_some()
}

async fn sleep(ms: i32) {
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms).unwrap();
    });
    JsFuture::from(promise).await.unwrap();
}

/// Lets effects apply and window listeners attach.
async fn settle() {
    sleep(60).await;
}

fn label(host: &HtmlElement) -> String {
    q(host, ".p-treeselect-label").text_content().unwrap_or_default().trim().to_owned()
}

fn node_content(host: &HtmlElement, key: &str) -> HtmlElement {
    q(host, &format!("li[data-id=\"{key}\"] > .p-tree-node-content"))
}

fn toggle(host: &HtmlElement, key: &str) -> HtmlElement {
    q(host, &format!("li[data-id=\"{key}\"] > .p-tree-node-content .p-tree-node-toggle-button"))
}

fn mouse(el: &web_sys::Element, name: &str) {
    let init = MouseEventInit::new();
    init.set_bubbles(true);
    el.dispatch_event(&MouseEvent::new_with_mouse_event_init_dict(name, &init).unwrap()).unwrap();
}

/// Key press and waiting for reactive updates.
async fn key(el: &HtmlElement, code: &str, key: &str) {
    let init = KeyboardEventInit::new();
    init.set_code(code);
    init.set_key(key);
    init.set_bubbles(true);
    init.set_cancelable(true);
    el.dispatch_event(&KeyboardEvent::new_with_keyboard_event_init_dict("keydown", &init).unwrap()).unwrap();
    sleep(20).await;
}

fn active_id() -> String {
    doc().active_element().map(|e| e.get_attribute("data-id").unwrap_or_else(|| e.id())).unwrap_or_default()
}

fn n(key: &str, label: &str) -> TreeNode {
    TreeNode::new(key).label(label)
}

/// 0 Documents{ 0-0 Work{0-0-0 Expenses}, 0-1 Home }, 1 Events
fn options() -> Vec<TreeNode> {
    vec![n("0", "Documents").children(vec![n("0-0", "Work").children(vec![n("0-0-0", "Expenses")]), n("0-1", "Home")]), n("1", "Events")]
}

async fn open_select_close_in_single_mode() {
    let (host, log) = (host(), Log::default());
    let value = RwSignal::new(Vec::<String>::new());
    let (l1, l2, l3) = (log.clone(), log.clone(), log.clone());
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! {
            <TreeSelect
                options=options()
                value=value
                placeholder="Pick"
                on_show=Callback::new(move |_| l1.lock().unwrap().push("show".into()))
                on_hide=Callback::new(move |_| l2.lock().unwrap().push("hide".into()))
                on_node_select=Callback::new(move |e: prime_tree::TreeNodeEvent| l3.lock().unwrap().push(format!("select:{}", e.node.key)))
            />
        }
    });
    settle().await;
    assert_eq!(label(&host), "Pick");
    assert!(!has(&host, ".p-treeselect-overlay"));
    q(&host, ".p-treeselect").click();
    settle().await;
    assert!(has(&host, ".p-treeselect-overlay") && has(&host, ".p-treeselect-tree-container .p-tree"));
    assert!(q(&host, ".p-treeselect").class_name().contains("p-treeselect-open"));
    assert_eq!(q(&host, "input").get_attribute("aria-expanded").as_deref(), Some("true"));
    assert!(!q(&host, ".p-treeselect-overlay").style().get_property_value("z-index").unwrap().is_empty(), "the panel got a z-index");
    assert_eq!(q(&host, ".p-treeselect-tree-container").style().get_property_value("max-height").unwrap(), "400px");

    q(&host, ".p-treeselect").click();
    settle().await;
    assert!(!has(&host, ".p-treeselect-overlay"), "repeated click on the field closes");

    q(&host, ".p-treeselect").click();
    settle().await;
    toggle(&host, "0").click();
    settle().await;
    node_content(&host, "0-1").click();
    settle().await;
    assert_eq!(value.get_untracked(), ["0-1"]);
    assert!(!has(&host, ".p-treeselect-overlay"), "single selection closes the panel");
    assert_eq!(label(&host), "Home");
    assert_eq!(doc().active_element().and_then(|e| e.get_attribute("role")).as_deref(), Some("combobox"), "focus returned to the field");
    assert_eq!(log.lock().unwrap().as_slice(), ["show", "hide", "show", "select:0-1", "hide"]);

    q(&host, ".p-treeselect").click();
    settle().await;
    assert!(has(&host, "li[data-id=\"0-1\"]"), "on open the ancestors of the selection are expanded");
    mouse(&doc().body().unwrap().into(), "click");
    settle().await;
    assert!(!has(&host, ".p-treeselect-overlay"), "outside click closes");
    host.remove();
}

async fn multiple_and_checkbox_modes_keep_the_panel_open() {
    let host = host();
    let value = RwSignal::new(Vec::<String>::new());
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <TreeSelect options=options() value=value selection_mode=TreeSelectionMode::Multiple display=TreeSelectDisplay::Chip show_clear=true /> }
    });
    settle().await;
    q(&host, ".p-treeselect").click();
    settle().await;
    node_content(&host, "1").click();
    settle().await;
    toggle(&host, "0").click();
    settle().await;
    node_content(&host, "0-1").click();
    settle().await;
    assert_eq!(value.get_untracked(), ["1", "0-1"]);
    assert!(has(&host, ".p-treeselect-overlay"), "in Multiple the panel stays open");
    assert_eq!(host.query_selector_all(".p-treeselect-chip-item").unwrap().length(), 2);
    mouse(&host.query_selector(".p-treeselect-clear-icon").unwrap().unwrap(), "click");
    settle().await;
    assert!(value.get_untracked().is_empty(), "clear resets the value");
    assert!(has(&host, ".p-treeselect-overlay"), "clear neither closes nor toggles the panel");
    assert!(!has(&host, ".p-treeselect-clear-icon"));
    host.remove();

    let host = self::host();
    let value = RwSignal::new(Vec::<String>::new());
    let _h = leptos::mount::mount_to(
        host.clone(),
        move || view! { <TreeSelect options=options() value=value selection_mode=TreeSelectionMode::Checkbox /> },
    );
    settle().await;
    q(&host, ".p-treeselect").click();
    settle().await;
    node_content(&host, "0").click();
    settle().await;
    let mut got = value.get_untracked();
    got.sort();
    assert_eq!(got, ["0", "0-0", "0-0-0", "0-1"], "the parent checkbox checks the subtree");
    assert!(has(&host, ".p-treeselect-overlay"));
    assert_eq!(label(&host).matches(", ").count(), 3);
    host.remove();
}

async fn keyboard_opens_and_closes() {
    let host = host();
    let value = RwSignal::new(Vec::<String>::new());
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <TreeSelect options=options() value=value /> });
    settle().await;
    let input = q(&host, "input");
    input.focus().unwrap();
    settle().await;
    assert!(q(&host, ".p-treeselect").class_name().contains("p-focus"));
    key(&input, "ArrowDown", "ArrowDown").await;
    settle().await;
    assert!(has(&host, ".p-treeselect-overlay"));
    assert_eq!(active_id(), "0", "Down opens the panel and focuses the first node");
    key(&q(&host, "li[data-id=\"0\"]"), "ArrowRight", "ArrowRight").await;
    settle().await;
    assert_eq!(active_id(), "0-0", "tree keys work inside the panel");
    input.focus().unwrap();
    key(&input, "Escape", "Escape").await;
    settle().await;
    assert!(!has(&host, ".p-treeselect-overlay"), "Escape closes");
    assert_eq!(doc().active_element().and_then(|e| e.get_attribute("role")).as_deref(), Some("combobox"));
    key(&input, "Enter", "Enter").await;
    settle().await;
    assert!(has(&host, ".p-treeselect-overlay"), "Enter opens");
    key(&input, "Tab", "Tab").await;
    settle().await;
    assert!(!has(&host, ".p-treeselect-overlay"), "Tab closes");
    host.remove();
}

async fn disabled_and_filter() {
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <TreeSelect options=options() disabled=true /> });
    settle().await;
    q(&host, ".p-treeselect").click();
    settle().await;
    assert!(!has(&host, ".p-treeselect-overlay"), "a disabled field does not open");
    key(&q(&host, "input"), "ArrowDown", "ArrowDown").await;
    assert!(!has(&host, ".p-treeselect-overlay"));
    host.remove();

    let host = self::host();
    let _h =
        leptos::mount::mount_to(host.clone(), move || view! { <TreeSelect options=options() filter=true filter_placeholder="Find" /> });
    settle().await;
    q(&host, ".p-treeselect").click();
    settle().await;
    let input: web_sys::HtmlInputElement = q(&host, ".p-tree-filter-input").dyn_into().unwrap();
    assert_eq!(input.placeholder(), "Find");
    input.set_value("expen");
    input.dispatch_event(&web_sys::Event::new("input").unwrap()).unwrap();
    settle().await;
    assert!(has(&host, "li[data-id=\"0-0-0\"]") && !has(&host, "li[data-id=\"1\"]"), "the filter narrows the tree in the panel");
    q(&host, ".p-treeselect").click();
    settle().await;
    q(&host, ".p-treeselect").click();
    settle().await;
    let input: web_sys::HtmlInputElement = q(&host, ".p-tree-filter-input").dyn_into().unwrap();
    assert_eq!(input.value(), "", "filter reset on close");
    assert!(has(&host, "li[data-id=\"1\"]"));
    host.remove();
}

#[wasm_bindgen_test]
async fn treeselect_in_the_browser() {
    open_select_close_in_single_mode().await;
    multiple_and_checkbox_modes_keep_the_panel_open().await;
    keyboard_opens_and_closes().await;
    disabled_and_filter().await;
}
