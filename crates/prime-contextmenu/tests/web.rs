//! Поведение ContextMenu в реальном браузере (wasm-bindgen-test): показ по `contextmenu` на цели и на документе, положение у
//! курсора и переворот у края окна, закрытие (клик вне, правая кнопка, Escape, лист, `resize`), подменю, клавиатура и шаблоны.
//!
//! Сценарии идут одним последовательным тестом: слушатели окна общие на страницу. Стили темы не подключены.
#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use prime_contextmenu::{ContextMenu, ContextMenuItemContext, ContextMenuRef};
use prime_menu::MenuItem;
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

fn q(sel: &str) -> HtmlElement {
    doc().query_selector(sel).unwrap().unwrap_or_else(|| panic!("нет {sel}: {}", doc().body().unwrap().inner_html())).dyn_into().unwrap()
}

fn has(sel: &str) -> bool {
    doc().query_selector(sel).unwrap().is_some()
}

async fn sleep(ms: i32) {
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms).unwrap();
    });
    JsFuture::from(promise).await.unwrap();
}

/// Даёт эффектам примениться и слушателю окна повеситься (в следующем тике).
async fn settle() {
    sleep(60).await;
}

fn content(id: &str, key: &str) -> HtmlElement {
    q(&format!("li[id=\"{id}_{key}\"] > .p-contextmenu-item-content"))
}

fn is_active(id: &str, key: &str) -> bool {
    q(&format!("li[id=\"{id}_{key}\"]")).class_name().contains("p-contextmenu-item-active")
}

fn focused() -> Option<String> {
    doc().query_selector("li.p-focus").unwrap().map(|e| e.get_attribute("id").unwrap())
}

/// Отправляет событие мыши; возвращает `false`, если действие по умолчанию отменено.
fn mouse(el: &web_sys::EventTarget, name: &str, x: i32, y: i32, button: i16) -> bool {
    let init = MouseEventInit::new();
    init.set_bubbles(true);
    init.set_cancelable(true);
    init.set_client_x(x);
    init.set_client_y(y);
    init.set_button(button);
    el.dispatch_event(&MouseEvent::new_with_mouse_event_init_dict(name, &init).unwrap()).unwrap()
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

/// Copy, Rename, ─, More{ Cut, Paste }
fn model(log: &Log) -> Vec<MenuItem> {
    vec![
        leaf("Copy", log),
        leaf("Rename", log),
        MenuItem::separator(),
        MenuItem::new().label("More").items(vec![leaf("Cut", log), leaf("Paste", log)]),
    ]
}

async fn right_click_on_the_target_shows_the_menu_at_the_cursor() {
    let (host, log) = (host(), Log::default());
    let events = Log::default();
    let (e1, e2) = (events.clone(), events.clone());
    let items = model(&log);
    let _h = leptos::mount::mount_to(host.clone(), move || {
        let (e1, e2) = (e1.clone(), e2.clone());
        view! {
            <div id="tgt" style="width: 100px; height: 100px">"Target"</div>
            <ContextMenu
                id="c"
                breakpoint="1px"
                target="#tgt"
                model=items.clone()
                on_show=Callback::new(move |()| e1.lock().unwrap().push("show".into()))
                on_hide=Callback::new(move |()| e2.lock().unwrap().push("hide".into()))
            />
        }
    });
    settle().await;
    assert!(!has(".p-contextmenu"), "до события меню нет");
    assert!(!mouse(&q("#tgt"), "contextmenu", 40, 30, 2), "действие по умолчанию отменено");
    settle().await;
    let panel = q("#c");
    assert!(panel.class_name().contains("p-contextmenu"), "{}", panel.class_name());
    assert_ne!(panel.style().get_property_value("visibility").unwrap(), "hidden", "положение посчитано");
    assert_eq!(panel.style().get_property_value("position").unwrap(), "absolute");
    let rect = panel.get_bounding_client_rect();
    assert!((rect.left() - 41.0).abs() < 1.5 && (rect.top() - 31.0).abs() < 1.5, "у курсора + 1px: {} {}", rect.left(), rect.top());
    assert!(!panel.style().get_property_value("z-index").unwrap().is_empty(), "слой назначен");
    assert!(doc().active_element().unwrap().is_same_node(Some(&q("#c_list"))), "фокус в корневом списке");
    assert_eq!(*events.lock().unwrap(), ["show"]);
    // Второй правый клик при открытом меню переставляет панель, не закрывая.
    mouse(&q("#tgt"), "contextmenu", 60, 50, 2);
    settle().await;
    let rect = q("#c").get_bounding_client_rect();
    assert!((rect.left() - 61.0).abs() < 1.5 && (rect.top() - 51.0).abs() < 1.5, "{} {}", rect.left(), rect.top());
    assert_eq!(*events.lock().unwrap(), ["show", "show"]);
    // Левый клик вне закрывает, правый и Ctrl-клик — нет.
    mouse(&doc().body().unwrap(), "click", 5, 5, 2);
    settle().await;
    assert!(has(".p-contextmenu"), "правая кнопка не закрывает");
    let init = MouseEventInit::new();
    init.set_bubbles(true);
    init.set_ctrl_key(true);
    doc().body().unwrap().dispatch_event(&MouseEvent::new_with_mouse_event_init_dict("click", &init).unwrap()).unwrap();
    settle().await;
    assert!(has(".p-contextmenu"), "Ctrl-клик не закрывает");
    mouse(&doc().body().unwrap(), "click", 5, 5, 0);
    settle().await;
    assert!(!has(".p-contextmenu"), "левый клик вне закрывает");
    assert_eq!(events.lock().unwrap().last().map(String::as_str), Some("hide"));
    host.remove();
}

async fn near_the_right_and_bottom_edges_the_menu_flips() {
    let (host, log) = (host(), Log::default());
    let items = model(&log);
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! {
            <div id="edge" style="width: 100px; height: 100px">"Target"</div>
            <ContextMenu id="e" breakpoint="1px" target="#edge" model=items.clone() />
        }
    });
    settle().await;
    let win = web_sys::window().unwrap();
    let (w, h) = (doc().document_element().unwrap().client_width(), win.inner_height().unwrap().as_f64().unwrap() as i32);
    mouse(&q("#edge"), "contextmenu", w - 5, h - 5, 2);
    settle().await;
    let rect = q("#e").get_bounding_client_rect();
    assert!(rect.right() <= f64::from(w) + 1.0 && rect.bottom() <= f64::from(h) + 1.0, "панель в окне: {rect:?}");
    assert!(rect.left() < f64::from(w - 5) && rect.top() < f64::from(h - 5), "перевернута влево и вверх");
    mouse(&doc().body().unwrap(), "click", 1, 1, 0);
    settle().await;
    host.remove();
}

async fn leaves_submenus_and_keyboard() {
    let (host, log) = (host(), Log::default());
    let items = model(&log);
    let r = Arc::new(Mutex::new(None::<ContextMenuRef>));
    let rr = r.clone();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        let menu = ContextMenuRef::new();
        *rr.lock().unwrap() = Some(menu);
        view! { <ContextMenu id="k" breakpoint="1px" model=items.clone() popup_ref=menu /> }
    });
    settle().await;
    let menu = r.lock().unwrap().unwrap();
    menu.show_at(30.0, 30.0);
    settle().await;
    assert!(menu.visible());
    // Клик по группе раскрывает подменю, лист закрывает меню и вызывает команду.
    content("k", "3").click();
    settle().await;
    assert!(is_active("k", "3") && has(".p-contextmenu-submenu"));
    content("k", "3_1").click();
    settle().await;
    assert!(!has(".p-contextmenu"), "лист закрывает меню");
    assert_eq!(*log.lock().unwrap(), ["Paste"]);

    // Клавиатура: ↓ ↓ → Escape.
    menu.show_at(30.0, 30.0);
    settle().await;
    let list = q("#k_list");
    press(&list, "ArrowDown", "ArrowDown");
    settle().await;
    assert_eq!(focused().as_deref(), Some("k_0"));
    press(&list, "End", "End");
    settle().await;
    assert_eq!(focused().as_deref(), Some("k_3"), "разделитель пропущен: End → More");
    press(&list, "ArrowRight", "ArrowRight");
    settle().await;
    assert!(is_active("k", "3") && focused().as_deref() == Some("k_3_0"));
    press(&list, "ArrowLeft", "ArrowLeft");
    settle().await;
    assert!(!has(".p-contextmenu-submenu") && focused().as_deref() == Some("k_3"));
    press(&list, "Home", "Home");
    press(&list, "KeyR", "r");
    settle().await;
    assert_eq!(focused().as_deref(), Some("k_1"), "поиск: Rename");
    press(&list, "Enter", "Enter");
    settle().await;
    assert!(!has(".p-contextmenu") && log.lock().unwrap().last().map(String::as_str) == Some("Rename"), "Enter выбирает пункт");
    menu.show_at(30.0, 30.0);
    settle().await;
    press(&q("#k_list"), "Escape", "Escape");
    settle().await;
    assert!(!has(".p-contextmenu"), "Escape закрывает");
    menu.show_at(30.0, 30.0);
    settle().await;
    press(&q("#k_list"), "Tab", "Tab");
    settle().await;
    assert!(!has(".p-contextmenu"), "Tab закрывает");
    // Изменение размера окна закрывает меню.
    menu.show_at(30.0, 30.0);
    settle().await;
    web_sys::window().unwrap().dispatch_event(&web_sys::Event::new("resize").unwrap()).unwrap();
    settle().await;
    assert!(!has(".p-contextmenu"), "resize закрывает");
    host.remove();
}

async fn global_menu_listens_on_the_document() {
    let (host, log) = (host(), Log::default());
    let items = model(&log);
    let handle =
        leptos::mount::mount_to(host.clone(), move || view! { <ContextMenu id="g" breakpoint="1px" global=true model=items.clone() /> });
    settle().await;
    mouse(&doc().body().unwrap(), "contextmenu", 20, 20, 2);
    settle().await;
    assert!(has("#g.p-contextmenu"), "правый клик где угодно");
    mouse(&doc().body().unwrap(), "click", 1, 1, 0);
    settle().await;
    assert!(!has(".p-contextmenu"));
    drop(handle);
    settle().await;
    mouse(&doc().body().unwrap(), "contextmenu", 20, 20, 2);
    settle().await;
    assert!(!has(".p-contextmenu"), "после размонтирования слушатель снят");
    host.remove();
}

async fn item_template_replaces_the_link() {
    let (host, log) = (host(), Log::default());
    let items = model(&log);
    let r = Arc::new(Mutex::new(None::<ContextMenuRef>));
    let rr = r.clone();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        let menu = ContextMenuRef::new();
        *rr.lock().unwrap() = Some(menu);
        view! {
            <ContextMenu
                id="t"
                breakpoint="1px"
                model=items.clone()
                popup_ref=menu
                item=Callback::new(|ctx: ContextMenuItemContext| view! { <b class="tpl">{ctx.item.label.unwrap_or_default()}</b> }.into_any())
            />
        }
    });
    settle().await;
    r.lock().unwrap().unwrap().show_at(10.0, 10.0);
    settle().await;
    assert!(has(".p-contextmenu-item-content > b.tpl") && !has(".p-contextmenu-item-link"));
    r.lock().unwrap().unwrap().hide();
    settle().await;
    host.remove();
}

async fn unmount_while_open_removes_listeners() {
    let (host, log) = (host(), Log::default());
    let items = model(&log);
    let r = Arc::new(Mutex::new(None::<ContextMenuRef>));
    let rr = r.clone();
    let handle = leptos::mount::mount_to(host.clone(), move || {
        let menu = ContextMenuRef::new();
        *rr.lock().unwrap() = Some(menu);
        view! { <ContextMenu id="u" breakpoint="1px" model=items.clone() popup_ref=menu /> }
    });
    settle().await;
    r.lock().unwrap().unwrap().show_at(10.0, 10.0);
    settle().await;
    drop(handle);
    settle().await;
    doc().body().unwrap().click();
    web_sys::window().unwrap().dispatch_event(&web_sys::Event::new("resize").unwrap()).unwrap();
    settle().await;
    assert!(!has(".p-contextmenu"));
    host.remove();
}

#[wasm_bindgen_test]
async fn contextmenu_behaviour_in_a_real_browser() {
    right_click_on_the_target_shows_the_menu_at_the_cursor().await;
    near_the_right_and_bottom_edges_the_menu_flips().await;
    leaves_submenus_and_keyboard().await;
    global_menu_listens_on_the_document().await;
    item_template_replaces_the_link().await;
    unmount_while_open_removes_listeners().await;
}
