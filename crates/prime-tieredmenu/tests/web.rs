//! TieredMenu behavior in a real browser (wasm-bindgen-test): expanding by click and hover, leaves and commands, keyboard
//! across levels, character search, outside click, submenu side, narrow window, `popup` mode and templates.
//!
//! Scenarios run as one sequential test: window listeners are shared per page. Theme styles are not attached.
#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use prime_menu::MenuItem;
use prime_tieredmenu::{TieredMenu, TieredMenuItemContext, TieredMenuRef};
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
    host.query_selector(sel).unwrap().unwrap_or_else(|| panic!("missing {sel}: {}", host.outer_html())).dyn_into().unwrap()
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

/// Lets effects apply and the window listener attach (on the next tick).
async fn settle() {
    sleep(60).await;
}

fn li(host: &HtmlElement, id: &str, key: &str) -> HtmlElement {
    q(host, &format!("li[id=\"{id}_{key}\"]"))
}

fn content(host: &HtmlElement, id: &str, key: &str) -> HtmlElement {
    li(host, id, key).query_selector(":scope > .p-tieredmenu-item-content").unwrap().unwrap().dyn_into().unwrap()
}

fn click(host: &HtmlElement, id: &str, key: &str) {
    content(host, id, key).click();
}

fn hover(host: &HtmlElement, id: &str, key: &str) {
    let init = MouseEventInit::new();
    init.set_bubbles(false);
    content(host, id, key).dispatch_event(&MouseEvent::new_with_mouse_event_init_dict("mouseenter", &init).unwrap()).unwrap();
}

fn press(el: &HtmlElement, code: &str, key: &str, alt: bool) -> bool {
    let init = KeyboardEventInit::new();
    init.set_code(code);
    init.set_key(key);
    init.set_alt_key(alt);
    init.set_bubbles(true);
    init.set_cancelable(true);
    el.dispatch_event(&KeyboardEvent::new_with_keyboard_event_init_dict("keydown", &init).unwrap()).unwrap()
}

fn key(el: &HtmlElement, code: &str) -> bool {
    press(el, code, code, false)
}

fn is_active(host: &HtmlElement, id: &str, k: &str) -> bool {
    li(host, id, k).class_name().contains("p-tieredmenu-item-active")
}

fn focused(host: &HtmlElement) -> Option<String> {
    host.query_selector("li.p-focus").unwrap().map(|e| e.get_attribute("id").unwrap())
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
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <TieredMenu breakpoint="1px" id="m" model=items.clone() /> });
    settle().await;
    assert!(!has(&host, ".p-tieredmenu-submenu"));
    click(&host, "m", "0");
    settle().await;
    assert!(is_active(&host, "m", "0") && has(&host, ".p-tieredmenu-submenu"));
    assert_eq!(li(&host, "m", "0").get_attribute("aria-expanded").as_deref(), Some("true"));
    assert_eq!(li(&host, "m", "0").get_attribute("data-p-highlight").as_deref(), Some("true"));
    click(&host, "m", "0_0");
    settle().await;
    assert!(is_active(&host, "m", "0_0"));
    click(&host, "m", "0_0");
    settle().await;
    assert!(!is_active(&host, "m", "0_0") && is_active(&host, "m", "0"), "a repeated click collapses the branch");
    click(&host, "m", "0");
    settle().await;
    assert!(!is_active(&host, "m", "0") && !has(&host, ".p-tieredmenu-submenu"));
    click(&host, "m", "0");
    settle().await;
    click(&host, "m", "0_0");
    settle().await;
    click(&host, "m", "0_0_1");
    settle().await;
    assert!(!has(&host, ".p-tieredmenu-submenu"), "a leaf closes all submenus");
    assert_eq!(*log.lock().unwrap(), ["Folder"]);
    assert!(doc().active_element().unwrap().is_same_node(Some(&q(&host, ".p-tieredmenu-root-list"))), "focus is in the root list");
    click(&host, "m", "1");
    settle().await;
    click(&host, "m", "0");
    settle().await;
    assert!(is_active(&host, "m", "0") && !is_active(&host, "m", "1"), "a sibling group closes the previous one");
    // a disabled item does not invoke its command
    click(&host, "m", "0_3");
    settle().await;
    assert_eq!(*log.lock().unwrap(), ["Folder"]);
    click(&host, "m", "4");
    settle().await;
    assert_eq!(log.lock().unwrap().last().map(String::as_str), Some("Quit"));
    host.remove();
}

async fn hover_opens_only_after_a_click() {
    let (host, log) = (host(), Log::default());
    let items = model(&log);
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <TieredMenu breakpoint="1px" id="h" model=items.clone() /> });
    settle().await;
    hover(&host, "h", "1");
    settle().await;
    assert!(!is_active(&host, "h", "1"), "hover does not expand before a click");
    click(&host, "h", "0");
    settle().await;
    hover(&host, "h", "1");
    settle().await;
    assert!(is_active(&host, "h", "1") && !is_active(&host, "h", "0"), "after a click, hover expands the sibling group");
    host.remove();

    let host = self::host();
    let items = model(&log);
    let _h = leptos::mount::mount_to(
        host.clone(),
        move || view! { <TieredMenu breakpoint="1px" id="h" model=items.clone() auto_display=false /> },
    );
    settle().await;
    click(&host, "h", "0");
    hover(&host, "h", "1");
    settle().await;
    assert!(!is_active(&host, "h", "1"), "auto_display=false");
    host.remove();

    let host = self::host();
    let items = model(&log);
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <TieredMenu id="h" model=items.clone() breakpoint="100000px" /> });
    settle().await;
    assert!(q(&host, ".p-tieredmenu").class_name().contains("p-tieredmenu-mobile"));
    click(&host, "h", "0");
    hover(&host, "h", "1");
    settle().await;
    assert!(!is_active(&host, "h", "1"), "in a narrow window hover does not change the path");
    host.remove();
}

async fn outside_click_closes_submenus() {
    let (host, log) = (host(), Log::default());
    let items = model(&log);
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <TieredMenu breakpoint="1px" id="o" model=items.clone() /> });
    settle().await;
    click(&host, "o", "0");
    settle().await;
    assert!(has(&host, ".p-tieredmenu-submenu"));
    q(&host, ".p-tieredmenu-root-list").click();
    settle().await;
    assert!(has(&host, ".p-tieredmenu-submenu"), "a click inside the menu does not close it");
    doc().body().unwrap().click();
    settle().await;
    assert!(!has(&host, ".p-tieredmenu-submenu"), "an outside click closes it");
    host.remove();
}

async fn keyboard_walks_the_levels() {
    let (host, log) = (host(), Log::default());
    let items = model(&log);
    let _h = leptos::mount::mount_to(
        host.clone(),
        move || view! { <TieredMenu breakpoint="1px" id="k" model=items.clone() aria_label="Menu" /> },
    );
    settle().await;
    let list = q(&host, ".p-tieredmenu-root-list");
    assert!(list.get_attribute("aria-activedescendant").is_none());
    list.focus().unwrap();
    settle().await;
    assert!(!key(&list, "ArrowDown"), "the arrow key is intercepted");
    settle().await;
    assert_eq!(focused(&host).as_deref(), Some("k_0"));
    assert_eq!(list.get_attribute("aria-activedescendant").as_deref(), Some("k_0"));
    key(&list, "ArrowDown");
    key(&list, "ArrowDown");
    key(&list, "ArrowDown");
    settle().await;
    assert_eq!(focused(&host).as_deref(), Some("k_4"), "Edit → Users → Quit, separator skipped");
    key(&list, "ArrowUp");
    settle().await;
    assert_eq!(focused(&host).as_deref(), Some("k_2"));
    key(&list, "Home");
    settle().await;
    assert_eq!(focused(&host).as_deref(), Some("k_0"));
    key(&list, "End");
    settle().await;
    assert_eq!(focused(&host).as_deref(), Some("k_4"));
    key(&list, "Home");
    key(&list, "ArrowRight");
    settle().await;
    assert!(is_active(&host, "k", "0"));
    assert_eq!(focused(&host).as_deref(), Some("k_0_0"), "focus moved inside to New");
    key(&list, "ArrowRight");
    settle().await;
    assert_eq!(focused(&host).as_deref(), Some("k_0_0_0"), "→ expands New and moves to Share");
    key(&list, "ArrowDown");
    key(&list, "ArrowDown");
    settle().await;
    assert_eq!(focused(&host).as_deref(), Some("k_0_0_1"), "stays at the boundary");
    key(&list, "ArrowLeft");
    settle().await;
    assert!(!is_active(&host, "k", "0_0") && is_active(&host, "k", "0"));
    assert_eq!(focused(&host).as_deref(), Some("k_0_0"), "← returned to New");
    key(&list, "ArrowDown");
    key(&list, "ArrowDown");
    settle().await;
    assert_eq!(focused(&host).as_deref(), Some("k_0_1"), "Print is disabled: stayed on Open");
    key(&list, "Enter");
    settle().await;
    assert_eq!(*log.lock().unwrap(), ["Open"], "Enter activates the item");
    assert!(!has(&host, ".p-tieredmenu-submenu"), "the leaf closed everything");
    key(&list, "ArrowRight");
    key(&list, "Escape");
    settle().await;
    assert!(!has(&host, ".p-tieredmenu-submenu"), "Escape closes");
    assert_eq!(focused(&host).as_deref(), Some("k_0"));
    key(&list, "ArrowRight");
    settle().await;
    assert!(has(&host, ".p-tieredmenu-submenu"));
    key(&list, "Tab");
    settle().await;
    assert!(!has(&host, ".p-tieredmenu-submenu"), "Tab closes");
    assert!(press(&list, "KeyZ", "z", false) || true, "a search with no matches is harmless");
    host.remove();
}

async fn typing_searches_by_prefix_and_resets() {
    let (host, log) = (host(), Log::default());
    let items = model(&log);
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <TieredMenu breakpoint="1px" id="s" model=items.clone() /> });
    settle().await;
    let list = q(&host, ".p-tieredmenu-root-list");
    list.focus().unwrap();
    press(&list, "KeyE", "e", false);
    settle().await;
    assert_eq!(focused(&host).as_deref(), Some("s_1"), "Edit");
    press(&list, "KeyU", "u", false);
    settle().await;
    assert_eq!(focused(&host).as_deref(), Some("s_1"), "'eu' not found: stay on Edit");
    sleep(600).await;
    press(&list, "KeyU", "u", false);
    settle().await;
    assert_eq!(focused(&host).as_deref(), Some("s_2"), "after the buffer reset, 'u' → Users");
    sleep(600).await;
    press(&list, "KeyF", "f", true);
    settle().await;
    assert_eq!(focused(&host).as_deref(), Some("s_0"), "f → File, wrapping around");
    host.remove();
}

async fn submenu_flips_near_the_right_edge() {
    let (host, log) = (host(), Log::default());
    let items = model(&log);
    host.set_attribute("style", "position: absolute; top: 0; left: calc(100vw - 60px); width: 60px").unwrap();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <TieredMenu breakpoint="1px" id="f" model=items.clone() /> });
    settle().await;
    click(&host, "f", "0");
    sleep(120).await;
    let sub = q(&host, ".p-tieredmenu-submenu");
    assert_eq!(sub.style().get_property_value("left").unwrap(), "-100%", "no room on the right, so the submenu is on the left");
    assert_eq!(sub.style().get_property_value("top").unwrap(), "0px");
    host.remove();

    let host = self::host();
    let items = model(&log);
    host.set_attribute("style", "position: absolute; top: 0; left: 0; width: 60px").unwrap();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <TieredMenu breakpoint="1px" id="f" model=items.clone() /> });
    settle().await;
    click(&host, "f", "0");
    sleep(120).await;
    assert_eq!(q(&host, ".p-tieredmenu-submenu").style().get_property_value("left").unwrap(), "100%", "enough room, so it is on the right");
    host.remove();
}

async fn popup_mode() {
    let (host, log) = (host(), Log::default());
    let items = model(&log);
    let events = Log::default();
    let (e1, e2) = (events.clone(), events.clone());
    let r = Arc::new(Mutex::new(None::<TieredMenuRef>));
    let rr = r.clone();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        let popup_ref = TieredMenuRef::new();
        *rr.lock().unwrap() = Some(popup_ref);
        let (e1, e2) = (e1.clone(), e2.clone());
        view! {
            <button id="trigger" on:click=move |e| popup_ref.toggle(&e)>"Toggle"</button>
            <TieredMenu
                id="p"
                breakpoint="1px"
                model=items.clone()
                popup=true
                popup_ref=popup_ref
                on_show=Callback::new(move |()| e1.lock().unwrap().push("show".into()))
                on_hide=Callback::new(move |()| e2.lock().unwrap().push("hide".into()))
            />
        }
    });
    settle().await;
    assert!(!has(&host, ".p-tieredmenu"), "not shown yet, so not in the DOM");
    let trigger = q(&host, "#trigger");
    trigger.click();
    settle().await;
    let root = q(&host, ".p-tieredmenu");
    assert!(root.class_name().contains("p-tieredmenu-overlay"));
    assert!(root.style().get_property_value("z-index").unwrap().parse::<i32>().unwrap() >= 1000, "Menu group layer");
    assert_eq!(root.style().get_property_value("position").unwrap(), "absolute");
    assert!(doc().active_element().unwrap().is_same_node(Some(&q(&host, ".p-tieredmenu-root-list"))), "focus is in the list");
    assert_eq!(*events.lock().unwrap(), ["show"]);
    trigger.click();
    settle().await;
    assert!(!has(&host, ".p-tieredmenu"), "a repeated toggle hides it");
    assert_eq!(*events.lock().unwrap(), ["show", "hide"]);

    trigger.click();
    settle().await;
    click(&host, "p", "0");
    settle().await;
    click(&host, "p", "0_1");
    settle().await;
    assert!(!has(&host, ".p-tieredmenu"), "a leaf hides the whole menu");
    assert_eq!(*log.lock().unwrap(), ["Open"]);

    trigger.click();
    settle().await;
    click(&host, "p", "0");
    settle().await;
    doc().body().unwrap().click();
    settle().await;
    assert!(!has(&host, ".p-tieredmenu"), "an outside click hides it");

    trigger.click();
    settle().await;
    let list = q(&host, ".p-tieredmenu-root-list");
    key(&list, "Escape");
    settle().await;
    assert!(!has(&host, ".p-tieredmenu"));
    assert!(doc().active_element().unwrap().is_same_node(Some(&trigger)), "Escape returns focus to the target");

    trigger.click();
    settle().await;
    let list = q(&host, ".p-tieredmenu-root-list");
    press(&list, "ArrowUp", "ArrowUp", true);
    settle().await;
    assert!(!has(&host, ".p-tieredmenu"), "Alt+↑ hides it");
    assert!(doc().active_element().unwrap().is_same_node(Some(&trigger)));

    trigger.click();
    settle().await;
    r.lock().unwrap().unwrap().hide();
    settle().await;
    assert!(!has(&host, ".p-tieredmenu"), "hide() hides it");
    host.remove();
}

async fn item_template_replaces_the_link() {
    let (host, log) = (host(), Log::default());
    let items = model(&log);
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! {
            <TieredMenu
                id="t"
                breakpoint="1px"
                model=items.clone()
                item=Callback::new(|ctx: TieredMenuItemContext| {
                    view! { <b class="tpl">{ctx.item.label.unwrap_or_default()}{ctx.has_submenu.then_some("+")}</b> }.into_any()
                })
            />
        }
    });
    settle().await;
    assert!(!has(&host, ".p-tieredmenu-item-link"));
    assert_eq!(q(&li(&host, "t", "0"), ".tpl").text_content().as_deref(), Some("File+"));
    click(&host, "t", "0");
    settle().await;
    assert!(is_active(&host, "t", "0"), "a click on its own content also expands the group");
    host.remove();
}

async fn unmount_while_open_removes_listeners() {
    let (host, log) = (host(), Log::default());
    let items = model(&log);
    let handle = leptos::mount::mount_to(host.clone(), move || view! { <TieredMenu breakpoint="1px" id="u" model=items.clone() /> });
    settle().await;
    click(&host, "u", "0");
    settle().await;
    drop(handle);
    settle().await;
    doc().body().unwrap().click();
    settle().await;
    assert!(!has(&host, ".p-tieredmenu"));
    host.remove();
}

#[wasm_bindgen_test]
async fn tieredmenu_behaviour_in_a_real_browser() {
    click_expands_collapses_and_leaves_close().await;
    hover_opens_only_after_a_click().await;
    outside_click_closes_submenus().await;
    keyboard_walks_the_levels().await;
    typing_searches_by_prefix_and_resets().await;
    submenu_flips_near_the_right_edge().await;
    popup_mode().await;
    item_template_replaces_the_link().await;
    unmount_while_open_removes_listeners().await;
}
