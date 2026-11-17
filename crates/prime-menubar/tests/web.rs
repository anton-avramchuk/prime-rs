//! Menubar behavior in a real browser (wasm-bindgen-test): expanding by click and hover, leaves and commands, auto-hide,
//! click outside, root and submenu keyboard, character search, mobile button, templates and unmount.
//!
//! Scenarios run as one sequential test: window listeners are shared across the page. Theme styles are not attached.
#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use prime_menu::MenuItem;
use prime_menubar::{Menubar, MenubarItemContext};
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

/// Lets effects apply and the window listener attach.
async fn settle() {
    sleep(60).await;
}

fn li(host: &HtmlElement, id: &str, key: &str) -> HtmlElement {
    q(host, &format!("li[id=\"{id}_{key}\"]"))
}

fn content(host: &HtmlElement, id: &str, key: &str) -> HtmlElement {
    li(host, id, key).query_selector(":scope > .p-menubar-item-content").unwrap().unwrap().dyn_into().unwrap()
}

fn click(host: &HtmlElement, id: &str, key: &str) {
    content(host, id, key).click();
}

fn is_active(host: &HtmlElement, id: &str, key: &str) -> bool {
    li(host, id, key).class_name().contains("p-menubar-item-active")
}

fn focused(host: &HtmlElement) -> Option<String> {
    host.query_selector("li.p-focus").unwrap().map(|e| e.get_attribute("id").unwrap())
}

fn mouse(el: &web_sys::EventTarget, name: &str, bubbles: bool) {
    let init = MouseEventInit::new();
    init.set_bubbles(bubbles);
    el.dispatch_event(&MouseEvent::new_with_mouse_event_init_dict(name, &init).unwrap()).unwrap();
}

fn press(el: &HtmlElement, code: &str, key: &str) {
    let init = KeyboardEventInit::new();
    init.set_code(code);
    init.set_key(key);
    init.set_bubbles(true);
    init.set_cancelable(true);
    el.dispatch_event(&KeyboardEvent::new_with_keyboard_event_init_dict("keydown", &init).unwrap()).unwrap();
}

fn leaf(label: &str, log: &Log) -> MenuItem {
    let log = log.clone();
    let name = label.to_owned();
    MenuItem::new().label(label).command(Callback::new(move |_| log.lock().unwrap().push(name.clone())))
}

/// File{ New{Share,Folder}, Open, ─, Print(off) }, Edit{Undo,Redo}, Users, ─, Quit
fn model(log: &Log) -> Vec<MenuItem> {
    vec![
        MenuItem::new().label("File").items(vec![
            MenuItem::new().label("New").items(vec![leaf("Share", log), leaf("Folder", log)]),
            leaf("Open", log),
            MenuItem::separator(),
            leaf("Print", log).disabled(true),
        ]),
        MenuItem::new().label("Edit").items(vec![leaf("Undo", log), leaf("Redo", log)]),
        leaf("Users", log),
        MenuItem::separator(),
        leaf("Quit", log),
    ]
}

async fn click_expands_collapses_and_leaves_close() {
    let (host, log) = (host(), Log::default());
    let items = model(&log);
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Menubar breakpoint="1px" id="m" model=items.clone() /> });
    settle().await;
    assert!(!has(&host, ".p-menubar-submenu"));
    click(&host, "m", "0");
    settle().await;
    assert!(is_active(&host, "m", "0") && has(&host, ".p-menubar-submenu"));
    assert_eq!(li(&host, "m", "0").get_attribute("aria-expanded").as_deref(), Some("true"));
    assert_eq!(q(&host, ".p-menubar-submenu").style().get_property_value("display").unwrap(), "flex");
    click(&host, "m", "0_0");
    settle().await;
    assert!(is_active(&host, "m", "0_0"));
    click(&host, "m", "0_0");
    settle().await;
    assert!(!is_active(&host, "m", "0_0") && is_active(&host, "m", "0"), "a repeated click collapses the branch");
    click(&host, "m", "0");
    settle().await;
    assert!(!has(&host, ".p-menubar-submenu"));
    click(&host, "m", "0");
    settle().await;
    click(&host, "m", "0_0");
    settle().await;
    click(&host, "m", "0_0_1");
    settle().await;
    assert!(!has(&host, ".p-menubar-submenu"), "a leaf closes all submenus");
    assert_eq!(*log.lock().unwrap(), ["Folder"]);
    assert!(doc().active_element().unwrap().is_same_node(Some(&q(&host, ".p-menubar-root-list"))), "focus in the root list");
    click(&host, "m", "0");
    settle().await;
    click(&host, "m", "0_3");
    settle().await;
    assert_eq!(*log.lock().unwrap(), ["Folder"], "a disabled item does not invoke the command");
    click(&host, "m", "4");
    settle().await;
    assert_eq!(log.lock().unwrap().last().map(String::as_str), Some("Quit"));
    assert!(!has(&host, ".p-menubar-submenu"));
    host.remove();
}

async fn hover_opens_only_after_a_click_and_respects_auto_display() {
    let (host, log) = (host(), Log::default());
    let items = model(&log);
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Menubar breakpoint="1px" id="h" model=items.clone() /> });
    settle().await;
    mouse(&content(&host, "h", "1"), "mouseenter", false);
    settle().await;
    assert!(!is_active(&host, "h", "1"), "before a click hover does not expand");
    click(&host, "h", "0");
    mouse(&content(&host, "h", "1"), "mouseenter", false);
    settle().await;
    assert!(is_active(&host, "h", "1") && !is_active(&host, "h", "0"), "after a click hover expands the neighboring group");
    host.remove();

    let host = self::host();
    let items = model(&log);
    let _h =
        leptos::mount::mount_to(host.clone(), move || view! { <Menubar breakpoint="1px" id="h" model=items.clone() auto_display=false /> });
    settle().await;
    click(&host, "h", "0");
    mouse(&content(&host, "h", "1"), "mouseenter", false);
    settle().await;
    assert!(!is_active(&host, "h", "1"), "auto_display=false");
    host.remove();
}

async fn auto_hide_closes_after_the_mouse_leaves() {
    let (host, log) = (host(), Log::default());
    let items = model(&log);
    let _h = leptos::mount::mount_to(
        host.clone(),
        move || view! { <Menubar breakpoint="1px" id="a" model=items.clone() auto_hide=true auto_hide_delay=80 /> },
    );
    settle().await;
    click(&host, "a", "0");
    settle().await;
    let list = q(&host, ".p-menubar-root-list");
    mouse(&list, "mouseleave", false);
    sleep(30).await;
    assert!(is_active(&host, "a", "0"), "while the delay has not elapsed");
    mouse(&list, "mouseenter", false);
    sleep(150).await;
    assert!(is_active(&host, "a", "0"), "the cursor returning cancels the close");
    mouse(&list, "mouseleave", false);
    sleep(200).await;
    assert!(!has(&host, ".p-menubar-submenu"), "after auto_hide_delay the submenu is closed");
    host.remove();

    let host = self::host();
    let items = model(&log);
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Menubar breakpoint="1px" id="a" model=items.clone() /> });
    settle().await;
    click(&host, "a", "0");
    settle().await;
    mouse(&q(&host, ".p-menubar-root-list"), "mouseleave", false);
    sleep(250).await;
    assert!(is_active(&host, "a", "0"), "without auto_hide the submenu stays");
    host.remove();
}

async fn outside_click_closes_submenus() {
    let (host, log) = (host(), Log::default());
    let items = model(&log);
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Menubar breakpoint="1px" id="o" model=items.clone() /> });
    settle().await;
    click(&host, "o", "0");
    settle().await;
    assert!(has(&host, ".p-menubar-submenu"));
    doc().body().unwrap().click();
    settle().await;
    assert!(!has(&host, ".p-menubar-submenu"), "a click outside closes");
    click(&host, "o", "1");
    settle().await;
    web_sys::window().unwrap().dispatch_event(&web_sys::Event::new("resize").unwrap()).unwrap();
    settle().await;
    assert!(!has(&host, ".p-menubar-submenu"), "resize closes");
    host.remove();
}

async fn keyboard_walks_root_and_submenus() {
    let (host, log) = (host(), Log::default());
    let items = model(&log);
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Menubar breakpoint="1px" id="k" model=items.clone() /> });
    settle().await;
    let list = q(&host, ".p-menubar-root-list");
    list.focus().unwrap();
    settle().await;
    assert_eq!(focused(&host).as_deref(), Some("k_0"), "focus from outside sets the first item");
    assert_eq!(list.get_attribute("aria-activedescendant").as_deref(), Some("k_0"));
    press(&list, "ArrowRight", "ArrowRight");
    settle().await;
    assert_eq!(focused(&host).as_deref(), Some("k_1"), "→ along the root");
    press(&list, "ArrowRight", "ArrowRight");
    press(&list, "ArrowRight", "ArrowRight");
    settle().await;
    assert_eq!(focused(&host).as_deref(), Some("k_4"), "Users → Quit, separator skipped");
    press(&list, "ArrowLeft", "ArrowLeft");
    settle().await;
    assert_eq!(focused(&host).as_deref(), Some("k_2"), "← along the root");
    press(&list, "ArrowDown", "ArrowDown");
    settle().await;
    assert!(!has(&host, ".p-menubar-submenu") && focused(&host).as_deref() == Some("k_2"), "↓ on a root leaf does nothing");
    press(&list, "Home", "Home");
    press(&list, "ArrowDown", "ArrowDown");
    settle().await;
    assert!(is_active(&host, "k", "0") && focused(&host).as_deref() == Some("k_0_0"), "↓ expands the group and enters");
    press(&list, "ArrowDown", "ArrowDown");
    settle().await;
    assert_eq!(focused(&host).as_deref(), Some("k_0_1"));
    press(&list, "ArrowDown", "ArrowDown");
    settle().await;
    assert_eq!(focused(&host).as_deref(), Some("k_0_1"), "Print is disabled: stay on Open");
    press(&list, "ArrowUp", "ArrowUp");
    press(&list, "ArrowRight", "ArrowRight");
    settle().await;
    assert!(is_active(&host, "k", "0_0") && focused(&host).as_deref() == Some("k_0_0_0"), "→ expands the nested group");
    press(&list, "ArrowLeft", "ArrowLeft");
    settle().await;
    assert!(!is_active(&host, "k", "0_0") && is_active(&host, "k", "0") && focused(&host).as_deref() == Some("k_0_0"), "← to the parent");
    press(&list, "ArrowLeft", "ArrowLeft");
    settle().await;
    assert!(!has(&host, ".p-menubar-submenu") && focused(&host).as_deref() == Some("k_0"), "← from a root submenu closes it");
    // Enter on a leaf invokes the command and closes everything.
    press(&list, "ArrowDown", "ArrowDown");
    press(&list, "ArrowDown", "ArrowDown");
    settle().await;
    press(&list, "Enter", "Enter");
    settle().await;
    assert_eq!(log.lock().unwrap().last().map(String::as_str), Some("Open"));
    assert!(!has(&host, ".p-menubar-submenu"));
    // Escape closes.
    press(&list, "ArrowDown", "ArrowDown");
    settle().await;
    assert!(has(&host, ".p-menubar-submenu"));
    press(&list, "Escape", "Escape");
    settle().await;
    assert!(!has(&host, ".p-menubar-submenu"), "Escape closes");
    press(&list, "ArrowDown", "ArrowDown");
    settle().await;
    press(&list, "Tab", "Tab");
    settle().await;
    assert!(!has(&host, ".p-menubar-submenu"), "Tab closes");
    host.remove();
}

async fn typing_searches_by_prefix() {
    let (host, log) = (host(), Log::default());
    let items = model(&log);
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Menubar breakpoint="1px" id="s" model=items.clone() /> });
    settle().await;
    let list = q(&host, ".p-menubar-root-list");
    list.focus().unwrap();
    press(&list, "KeyE", "e");
    settle().await;
    assert_eq!(focused(&host).as_deref(), Some("s_1"), "Edit");
    sleep(600).await;
    press(&list, "KeyU", "u");
    settle().await;
    assert_eq!(focused(&host).as_deref(), Some("s_2"), "Users");
    host.remove();
}

async fn mobile_button_toggles_the_list() {
    let (host, log) = (host(), Log::default());
    let items = model(&log);
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Menubar id="b" model=items.clone() breakpoint="100000px" /> });
    settle().await;
    assert!(q(&host, "#b").class_name().contains("p-menubar-mobile"));
    let button = q(&host, ".p-menubar-button");
    assert_eq!(button.get_attribute("aria-expanded").as_deref(), Some("false"));
    button.click();
    settle().await;
    assert!(q(&host, "#b").class_name().contains("p-menubar-mobile-active"));
    assert_eq!(button.get_attribute("aria-expanded").as_deref(), Some("true"));
    assert!(!q(&host, ".p-menubar-root-list").style().get_property_value("z-index").unwrap().is_empty(), "layer assigned");
    assert_eq!(focused(&host).as_deref(), Some("b_0"), "focus on the first item");
    // In a narrow window hover does not change the path.
    click(&host, "b", "0");
    mouse(&content(&host, "b", "1"), "mouseenter", false);
    settle().await;
    assert!(is_active(&host, "b", "0") && !is_active(&host, "b", "1"));
    // A leaf closes the panel.
    click(&host, "b", "2");
    settle().await;
    assert!(!q(&host, "#b").class_name().contains("p-menubar-mobile-active"), "a leaf closes the panel");
    // Reopening and closing via the button.
    button.click();
    settle().await;
    assert!(q(&host, "#b").class_name().contains("p-menubar-mobile-active"));
    button.click();
    settle().await;
    assert!(!q(&host, "#b").class_name().contains("p-menubar-mobile-active"));
    assert!(q(&host, ".p-menubar-root-list").style().get_property_value("z-index").unwrap().is_empty());
    // Enter on the button opens, a click outside closes.
    press(&button, "Enter", "Enter");
    settle().await;
    assert!(q(&host, "#b").class_name().contains("p-menubar-mobile-active"));
    doc().body().unwrap().click();
    settle().await;
    assert!(!q(&host, "#b").class_name().contains("p-menubar-mobile-active"), "a click outside closes the panel");
    host.remove();
}

async fn slots_and_item_template() {
    let (host, log) = (host(), Log::default());
    let items = model(&log);
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! {
            <Menubar
                id="t"
                breakpoint="1px"
                model=items.clone()
                start=ViewFn::from(|| view! { <b class="logo">"Logo"</b> })
                end=ViewFn::from(|| view! { <i class="tail">"End"</i> })
                item=Callback::new(|ctx: MenubarItemContext| view! { <b class="tpl">{ctx.item.label.unwrap_or_default()}</b> }.into_any())
            />
        }
    });
    settle().await;
    assert!(has(&host, ".p-menubar-start > .logo") && has(&host, ".p-menubar-end > .tail"));
    assert!(has(&host, ".p-menubar-item-content > b.tpl") && !has(&host, ".p-menubar-item-link"));
    host.remove();
}

async fn unmount_while_open_removes_listeners() {
    let (host, log) = (host(), Log::default());
    let items = model(&log);
    let handle = leptos::mount::mount_to(host.clone(), move || view! { <Menubar breakpoint="1px" id="u" model=items.clone() /> });
    settle().await;
    click(&host, "u", "0");
    settle().await;
    drop(handle);
    settle().await;
    doc().body().unwrap().click();
    web_sys::window().unwrap().dispatch_event(&web_sys::Event::new("resize").unwrap()).unwrap();
    settle().await;
    assert!(!has(&host, ".p-menubar"));
    host.remove();
}

#[wasm_bindgen_test]
async fn menubar_behaviour_in_a_real_browser() {
    click_expands_collapses_and_leaves_close().await;
    hover_opens_only_after_a_click_and_respects_auto_display().await;
    auto_hide_closes_after_the_mouse_leaves().await;
    outside_click_closes_submenus().await;
    keyboard_walks_root_and_submenus().await;
    typing_searches_by_prefix().await;
    mobile_button_toggles_the_list().await;
    slots_and_item_template().await;
    unmount_while_open_removes_listeners().await;
}
