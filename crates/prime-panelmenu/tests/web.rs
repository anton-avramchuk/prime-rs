//! PanelMenu behavior in a real browser (wasm-bindgen-test): expansion by click in single and multiple mode,
//! nested branches, commands and disabled items, header and tree keyboard, character search, `PanelMenuRef`.
//!
//! The scenarios run as one sequential test: focus and page listeners are shared. Theme styles are not loaded.
#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use prime_menu::MenuItem;
use prime_panelmenu::{PanelMenu, PanelMenuItemContext, PanelMenuRef};
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

fn by_id(host: &HtmlElement, id: &str) -> HtmlElement {
    host.query_selector(&format!("[id=\"{id}\"]")).unwrap().unwrap_or_else(|| panic!("no #{id}: {}", host.outer_html())).dyn_into().unwrap()
}

async fn sleep(ms: i32) {
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms).unwrap();
    });
    JsFuture::from(promise).await.unwrap();
}

/// Lets effects apply and the collapse animations finish.
async fn settle() {
    sleep(60).await;
}

fn header(host: &HtmlElement, i: usize) -> HtmlElement {
    by_id(host, &format!("m_{i}_header"))
}

fn region(host: &HtmlElement, i: usize) -> HtmlElement {
    by_id(host, &format!("m_{i}_content"))
}

fn expanded(host: &HtmlElement, i: usize) -> bool {
    let h = header(host, i);
    let (aria, highlight) = (h.get_attribute("aria-expanded").unwrap(), h.get_attribute("data-p-highlight").unwrap());
    assert_eq!(aria, highlight, "aria-expanded and data-p-highlight disagree for panel {i}");
    let class = region(host, i).class_name().contains("p-panelmenu-expanded");
    assert_eq!(class, aria == "true", "the expanded class and aria-expanded disagree for panel {i}");
    aria == "true"
}

fn item_content(host: &HtmlElement, key: &str) -> HtmlElement {
    by_id(host, &format!("m_{key}")).query_selector(":scope > .p-panelmenu-item-content").unwrap().unwrap().dyn_into().unwrap()
}

fn item_expanded(host: &HtmlElement, key: &str) -> bool {
    by_id(host, &format!("m_{key}")).get_attribute("aria-expanded").as_deref() == Some("true")
}

fn focused_item(host: &HtmlElement) -> Option<String> {
    host.query_selector("li.p-focus").unwrap().map(|e| e.get_attribute("id").unwrap())
}

fn active_id() -> String {
    doc().active_element().map(|e| e.id()).unwrap_or_default()
}

fn press(el: &HtmlElement, code: &str, key: &str) {
    let init = KeyboardEventInit::new();
    init.set_code(code);
    init.set_key(key);
    init.set_bubbles(true);
    init.set_cancelable(true);
    el.dispatch_event(&KeyboardEvent::new_with_keyboard_event_init_dict("keydown", &init).unwrap()).unwrap();
}

/// A key press and waiting for the reactive updates to apply.
async fn key(el: &HtmlElement, code: &str, key: &str) {
    press(el, code, key);
    sleep(10).await;
}

fn leaf(label: &str, log: &Log) -> MenuItem {
    let log = log.clone();
    let name = label.to_owned();
    MenuItem::new().label(label).command(Callback::new(move |_| log.lock().unwrap().push(name.clone())))
}

/// 0 Files{ 0_0 Docs{0_0_0 Invoices, 0_0_1 Clients(off), 0_0_2 Zebra}, 0_1 Open, 0_2 ─, 0_3 Print(off), 0_4 Images }, 1 Cloud{1_0 Upload},
/// 2 Tasks(off){2_0 Add}, 3 Quit (a leaf with a command)
fn model(log: &Log) -> Vec<MenuItem> {
    vec![
        MenuItem::new().label("Files").items(vec![
            MenuItem::new().label("Docs").items(vec![leaf("Invoices", log), leaf("Clients", log).disabled(true), leaf("Zebra", log)]),
            leaf("Open", log),
            MenuItem::separator(),
            leaf("Print", log).disabled(true),
            leaf("Images", log),
        ]),
        MenuItem::new().label("Cloud").items(vec![leaf("Upload", log)]),
        MenuItem::new().label("Tasks").disabled(true).items(vec![leaf("Add", log)]),
        leaf("Quit", log),
    ]
}

async fn single_mode_keeps_one_panel_open() {
    let (host, log) = (host(), Log::default());
    let items = model(&log);
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <PanelMenu id="m" model=items.clone() /> });
    settle().await;
    assert!(!expanded(&host, 0) && !expanded(&host, 1));
    assert_eq!(region(&host, 0).style().get_property_value("display").unwrap(), "none", "the collapsed area is hidden");

    header(&host, 0).click();
    settle().await;
    assert!(expanded(&host, 0) && !expanded(&host, 1));
    assert_eq!(region(&host, 0).style().get_property_value("display").unwrap(), "", "the expanded area is shown");
    assert!(header(&host, 0).class_name().contains("p-panelmenu-header-active"));

    header(&host, 1).click();
    settle().await;
    assert!(expanded(&host, 1) && !expanded(&host, 0), "single mode collapses the previous panel");
    sleep(500).await;
    assert_eq!(region(&host, 0).style().get_property_value("display").unwrap(), "none", "after the animation the area is hidden");

    header(&host, 1).click();
    settle().await;
    assert!(!expanded(&host, 1), "a second click collapses");

    // Quick re-toggle: a late completion of the previous animation does not hide the expanded area.
    header(&host, 1).click();
    settle().await;
    header(&host, 1).click();
    settle().await;
    header(&host, 1).click();
    sleep(600).await;
    assert!(expanded(&host, 1));
    assert_eq!(region(&host, 1).style().get_property_value("display").unwrap(), "", "the expanded area is not hidden by a late timer");
    assert!(!region(&host, 1).class_name().contains("p-collapsible"), "animation classes removed");
    host.remove();
}

async fn multiple_mode_keeps_all_open() {
    let (host, log) = (host(), Log::default());
    let items = model(&log);
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <PanelMenu id="m" model=items.clone() multiple=true /> });
    settle().await;
    header(&host, 0).click();
    header(&host, 1).click();
    settle().await;
    assert!(expanded(&host, 0) && expanded(&host, 1));
    header(&host, 0).click();
    settle().await;
    assert!(!expanded(&host, 0) && expanded(&host, 1));
    host.remove();
}

async fn nested_groups_commands_and_disabled() {
    let (host, log) = (host(), Log::default());
    let items = model(&log);
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <PanelMenu id="m" model=items.clone() /> });
    settle().await;
    header(&host, 0).click();
    settle().await;
    item_content(&host, "0_0").click();
    settle().await;
    assert!(item_expanded(&host, "0_0"), "a click on a group expands the branch");
    assert!(expanded(&host, 0), "the panel stayed expanded");
    assert!(
        by_id(&host, "m_0_0")
            .query_selector(":scope > .p-panelmenu-content-container")
            .unwrap()
            .unwrap()
            .class_name()
            .contains("p-panelmenu-expanded")
    );
    assert_eq!(by_id(&host, "m_0_0_0").query_selector("a").unwrap().unwrap().get_attribute("tabindex").as_deref(), Some("0"));

    item_content(&host, "0_0_0").click();
    assert_eq!(log.lock().unwrap().as_slice(), ["Invoices"], "a leaf invokes the command");
    assert_eq!(focused_item(&host), None, "focus is shown only when the list has focus");

    item_content(&host, "0_0_1").click();
    assert_eq!(log.lock().unwrap().len(), 1, "a disabled item does not invoke the command");

    item_content(&host, "0_0").click();
    settle().await;
    assert!(!item_expanded(&host, "0_0"), "a second click collapses the branch");
    assert_eq!(by_id(&host, "m_0_0_0").query_selector("a").unwrap().unwrap().get_attribute("tabindex").as_deref(), Some("-1"));

    // A leaf header invokes the command; a disabled header does not react.
    header(&host, 3).click();
    assert_eq!(log.lock().unwrap().last().map(String::as_str), Some("Quit"));
    header(&host, 2).click();
    settle().await;
    assert!(!expanded(&host, 2), "a disabled header does not expand");
    host.remove();
}

async fn keyboard_walks_headers_and_trees() {
    let (host, log) = (host(), Log::default());
    let items = model(&log);
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <PanelMenu id="m" model=items.clone() /> });
    settle().await;
    header(&host, 0).click();
    settle().await;

    // ↓ on an expanded header — into the tree, to the first item.
    header(&host, 0).focus().unwrap();
    key(&header(&host, 0), "ArrowDown", "ArrowDown").await;
    settle().await;
    assert_eq!(active_id(), "m_0_list");
    assert_eq!(focused_item(&host).as_deref(), Some("m_0_0"));
    assert_eq!(by_id(&host, "m_0_list").get_attribute("aria-activedescendant").as_deref(), Some("m_0_0"));

    let list = by_id(&host, "m_0_list");
    key(&list, "ArrowRight", "ArrowRight").await;
    settle().await;
    assert!(item_expanded(&host, "0_0"), "→ expands the branch");
    key(&list, "ArrowRight", "ArrowRight").await;
    assert_eq!(focused_item(&host).as_deref(), Some("m_0_0_0"), "→ on an expanded branch — down");
    key(&list, "ArrowDown", "ArrowDown").await;
    assert_eq!(focused_item(&host).as_deref(), Some("m_0_0_2"), "disabled one skipped");
    key(&list, "ArrowLeft", "ArrowLeft").await;
    assert_eq!(focused_item(&host).as_deref(), Some("m_0_0"), "← on a leaf — to the parent");
    key(&list, "ArrowLeft", "ArrowLeft").await;
    settle().await;
    assert!(!item_expanded(&host, "0_0"), "← on an expanded branch collapses it");
    key(&list, "End", "End").await;
    assert_eq!(focused_item(&host).as_deref(), Some("m_0_4"));
    key(&list, "Home", "Home").await;
    assert_eq!(focused_item(&host).as_deref(), Some("m_0_0"));
    key(&list, "ArrowDown", "ArrowDown").await;
    assert_eq!(focused_item(&host).as_deref(), Some("m_0_1"));
    key(&list, "ArrowDown", "ArrowDown").await;
    assert_eq!(focused_item(&host).as_deref(), Some("m_0_4"), "separator and disabled one skipped");

    // Search by label prefix.
    key(&list, "KeyO", "o").await;
    assert_eq!(focused_item(&host).as_deref(), Some("m_0_1"));

    // Enter on a leaf invokes the command.
    key(&list, "Enter", "Enter").await;
    assert_eq!(log.lock().unwrap().as_slice(), ["Open"]);

    // ↓ after the last one — to the next header, ↑ on the first — to its own header.
    key(&list, "End", "End").await;
    key(&list, "ArrowDown", "ArrowDown").await;
    assert_eq!(active_id(), "m_1_header", "after the last item — the next header");
    header(&host, 0).focus().unwrap();
    key(&header(&host, 0), "ArrowDown", "ArrowDown").await;
    settle().await;
    key(&by_id(&host, "m_0_list"), "ArrowUp", "ArrowUp").await;
    assert_eq!(active_id(), "m_0_header", "↑ on the first item — the panel header");

    // Headers: ↓ skips the disabled one, wraps around; Home/End; Enter toggles.
    header(&host, 0).click();
    settle().await;
    header(&host, 0).focus().unwrap();
    key(&header(&host, 0), "ArrowDown", "ArrowDown").await;
    assert_eq!(active_id(), "m_1_header");
    key(&header(&host, 1), "ArrowDown", "ArrowDown").await;
    assert_eq!(active_id(), "m_3_header", "the disabled panel is skipped");
    key(&header(&host, 3), "ArrowDown", "ArrowDown").await;
    assert_eq!(active_id(), "m_0_header", "wraps around");
    key(&header(&host, 0), "ArrowUp", "ArrowUp").await;
    assert_eq!(active_id(), "m_3_header");
    key(&header(&host, 3), "Home", "Home").await;
    assert_eq!(active_id(), "m_0_header");
    key(&header(&host, 0), "End", "End").await;
    assert_eq!(active_id(), "m_3_header");
    key(&header(&host, 1), "Enter", "Enter").await;
    settle().await;
    assert!(expanded(&host, 1), "Enter expands the panel");
    key(&header(&host, 1), "Space", " ").await;
    settle().await;
    assert!(!expanded(&host, 1), "Space collapses");

    // Focus leaving the component resets the highlight.
    header(&host, 0).click();
    settle().await;
    header(&host, 0).focus().unwrap();
    key(&header(&host, 0), "ArrowDown", "ArrowDown").await;
    settle().await;
    assert!(focused_item(&host).is_some());
    by_id(&host, "m_0_list").blur().unwrap();
    settle().await;
    assert_eq!(focused_item(&host), None);
    host.remove();
}

async fn panel_ref_and_templates() {
    let (host, log) = (host(), Log::default());
    let items = model(&log);
    let handle = StoredValue::new_local(None::<PanelMenuRef>);
    let _h = leptos::mount::mount_to(host.clone(), move || {
        let r = PanelMenuRef::new();
        handle.set_value(Some(r));
        let item = Callback::new(|ctx: PanelMenuItemContext| view! { <i class="tpl">{ctx.item.label.clone()}</i> }.into_any());
        view! { <PanelMenu id="m" model=items.clone() panel_ref=r item=item /> }
    });
    settle().await;
    let r = handle.get_value().unwrap();
    r.expand_all();
    settle().await;
    assert!(expanded(&host, 0) && expanded(&host, 1) && item_expanded(&host, "0_0"), "expand_all expands all groups");
    assert!(
        host.query_selector(".tpl").unwrap().is_some() && host.query_selector(".p-panelmenu-item-link").unwrap().is_none(),
        "the template replaced the link"
    );
    r.collapse_all();
    settle().await;
    assert!(!expanded(&host, 0) && !item_expanded(&host, "0_0"));
    r.toggle_all();
    settle().await;
    assert!(r.all_expanded() && expanded(&host, 0));
    host.remove();
}

#[wasm_bindgen_test]
async fn panelmenu_in_the_browser() {
    single_mode_keeps_one_panel_open().await;
    multiple_mode_keeps_all_open().await;
    nested_groups_commands_and_disabled().await;
    keyboard_walks_headers_and_trees().await;
    panel_ref_and_templates().await;
}
