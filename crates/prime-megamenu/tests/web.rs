//! Поведение MegaMenu в реальном браузере (wasm-bindgen-test): раскрытие кликом и наведением, наложение с колонками,
//! листья и команды, клик вне, клавиатура обеих ориентаций, поиск, мобильная кнопка.
//!
//! Сценарии идут одним последовательным тестом: слушатели окна общие на страницу. Стили темы не подключены.
#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use prime_megamenu::{MegaMenu, MegaMenuItem, MegaMenuOrientation, MegaMenuSubmenu};
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

fn q(host: &HtmlElement, sel: &str) -> HtmlElement {
    host.query_selector(sel).unwrap().unwrap_or_else(|| panic!("нет {sel}: {}", host.outer_html())).dyn_into().unwrap()
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

/// Даёт эффектам примениться и слушателю окна повеситься.
async fn settle() {
    sleep(60).await;
}

fn li(host: &HtmlElement, id: &str, key: &str) -> HtmlElement {
    q(host, &format!("li[id=\"{id}_{key}\"]"))
}

fn content(host: &HtmlElement, id: &str, key: &str) -> HtmlElement {
    li(host, id, key).query_selector(":scope > .p-megamenu-item-content").unwrap().unwrap().dyn_into().unwrap()
}

fn is_active(host: &HtmlElement, id: &str, key: &str) -> bool {
    li(host, id, key).class_name().contains("p-megamenu-item-active")
}

fn focused(host: &HtmlElement) -> Option<String> {
    host.query_selector("li.p-focus").unwrap().map(|e| e.get_attribute("id").unwrap())
}

fn mouse(el: &web_sys::EventTarget, name: &str, bubbles: bool) {
    let init = MouseEventInit::new();
    init.set_bubbles(bubbles);
    el.dispatch_event(&MouseEvent::new_with_mouse_event_init_dict(name, &init).unwrap()).unwrap();
}

/// Нажатие клавиши и ожидание применения реактивных обновлений.
async fn key(el: &HtmlElement, code: &str, key: &str) {
    let init = KeyboardEventInit::new();
    init.set_code(code);
    init.set_key(key);
    init.set_bubbles(true);
    init.set_cancelable(true);
    el.dispatch_event(&KeyboardEvent::new_with_keyboard_event_init_dict("keydown", &init).unwrap()).unwrap();
    sleep(10).await;
}

fn leaf(label: &str, log: &Log) -> MenuItem {
    let log = log.clone();
    let name = label.to_owned();
    MenuItem::new().label(label).command(Callback::new(move |_| log.lock().unwrap().push(name.clone())))
}

/// 0 Furniture{ c0:[Living{Armchair,Bed}], c1:[Kitchen{Chair,Dead(off)}, Bath{Soap}], c2:[Bedroom{Cupboard}] }, 1 Plain(cmd), 2 Tech{ c0:[Phones{Android}] }
fn model(log: &Log) -> Vec<MegaMenuItem> {
    let sub = |name: &str, items: Vec<MenuItem>| MegaMenuSubmenu::new(name).items(items);
    vec![
        MegaMenuItem::new().label("Furniture").columns(vec![
            vec![sub("Living", vec![leaf("Armchair", log), leaf("Bed", log)])],
            vec![sub("Kitchen", vec![leaf("Chair", log), leaf("Dead", log).disabled(true)]), sub("Bath", vec![leaf("Soap", log)])],
            vec![sub("Bedroom", vec![leaf("Cupboard", log)])],
        ]),
        MegaMenuItem::from(leaf("Plain", log)),
        MegaMenuItem::new().label("Tech").columns(vec![vec![sub("Phones", vec![leaf("Android", log)])]]),
    ]
}

async fn click_opens_and_closes_the_overlay() {
    let (host, log) = (host(), Log::default());
    let items = model(&log);
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <MegaMenu breakpoint="1px" id="m" model=items.clone() /> });
    settle().await;
    assert_eq!(count(&host, ".p-megamenu-overlay"), 0);
    content(&host, "m", "0").click();
    settle().await;
    assert!(is_active(&host, "m", "0"));
    assert_eq!(li(&host, "m", "0").get_attribute("aria-expanded").as_deref(), Some("true"));
    assert_eq!(count(&host, ".p-megamenu-overlay"), 1);
    assert_eq!(count(&host, ".p-megamenu-col-4"), 3, "три колонки — col-4");
    assert_eq!(count(&host, "ul.p-megamenu-submenu"), 4);
    assert_eq!(count(&host, "li.p-megamenu-submenu-label"), 4);
    assert_eq!(li(&host, "m", "0_1_0_0").get_attribute("aria-level").as_deref(), Some("2"));
    assert_eq!(li(&host, "m", "0_1_0_1").get_attribute("aria-disabled").as_deref(), Some("true"));
    assert_eq!(li(&host, "m", "0_1_0_1").get_attribute("aria-setsize").as_deref(), Some("2"));

    content(&host, "m", "0").click();
    settle().await;
    assert!(!is_active(&host, "m", "0") && count(&host, ".p-megamenu-overlay") == 0, "повторный клик по корню закрывает");

    content(&host, "m", "0").click();
    settle().await;
    content(&host, "m", "0_0_0_1").click();
    settle().await;
    assert_eq!(log.lock().unwrap().as_slice(), ["Bed"], "лист вызывает команду");
    assert_eq!(count(&host, ".p-megamenu-overlay"), 0, "и закрывает наложение");

    content(&host, "m", "0").click();
    settle().await;
    content(&host, "m", "0_1_0_1").click();
    settle().await;
    assert_eq!(log.lock().unwrap().len(), 1, "отключённый не вызывает команду");
    assert_eq!(count(&host, ".p-megamenu-overlay"), 1, "и не закрывает наложение");

    mouse(&doc().body().unwrap(), "click", true);
    settle().await;
    assert_eq!(count(&host, ".p-megamenu-overlay"), 0, "клик вне закрывает");

    content(&host, "m", "1").click();
    assert_eq!(log.lock().unwrap().last().map(String::as_str), Some("Plain"), "корневой лист вызывает команду");
    host.remove();
}

async fn hover_follows_after_a_click() {
    let (host, log) = (host(), Log::default());
    let items = model(&log);
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <MegaMenu breakpoint="1px" id="m" model=items.clone() /> });
    settle().await;
    mouse(&content(&host, "m", "2"), "mouseenter", false);
    settle().await;
    assert_eq!(count(&host, ".p-megamenu-overlay"), 0, "без клика наведение не раскрывает");
    content(&host, "m", "0").click();
    settle().await;
    mouse(&content(&host, "m", "2"), "mouseenter", false);
    settle().await;
    assert!(is_active(&host, "m", "2") && !is_active(&host, "m", "0"), "после клика наведение переключает наложение");
    mouse(&content(&host, "m", "1"), "mouseenter", false);
    settle().await;
    assert_eq!(count(&host, ".p-megamenu-overlay"), 0, "наведение на обычный корень закрывает");
    host.remove();
}

async fn keyboard_horizontal() {
    let (host, log) = (host(), Log::default());
    let items = model(&log);
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <MegaMenu breakpoint="1px" id="m" model=items.clone() /> });
    settle().await;
    let list = q(&host, "#m_list");
    list.focus().unwrap();
    settle().await;
    assert_eq!(focused(&host).as_deref(), Some("m_0"), "фокус снаружи — первый пункт");
    assert_eq!(list.get_attribute("aria-activedescendant").as_deref(), Some("m_0"));
    key(&list, "ArrowRight", "ArrowRight").await;
    assert_eq!(focused(&host).as_deref(), Some("m_1"));
    key(&list, "ArrowLeft", "ArrowLeft").await;
    key(&list, "ArrowDown", "ArrowDown").await;
    assert!(is_active(&host, "m", "0"));
    assert_eq!(focused(&host).as_deref(), Some("m_0_0_0_0"), "↓ раскрывает и входит в первую колонку");
    key(&list, "ArrowDown", "ArrowDown").await;
    key(&list, "ArrowDown", "ArrowDown").await;
    assert_eq!(focused(&host).as_deref(), Some("m_0_1_0_0"));
    key(&list, "ArrowRight", "ArrowRight").await;
    assert_eq!(focused(&host).as_deref(), Some("m_0_2_0_0"), "→ — следующая колонка");
    key(&list, "ArrowLeft", "ArrowLeft").await;
    key(&list, "ArrowLeft", "ArrowLeft").await;
    assert_eq!(focused(&host).as_deref(), Some("m_0_0_0_0"));
    key(&list, "End", "End").await;
    assert_eq!(focused(&host).as_deref(), Some("m_0_2_0_0"));
    key(&list, "KeyS", "s").await;
    assert_eq!(focused(&host).as_deref(), Some("m_0_1_1_0"), "поиск по началу подписи");
    key(&list, "Enter", "Enter").await;
    assert_eq!(log.lock().unwrap().as_slice(), ["Soap"], "Enter выполняет команду");
    assert_eq!(count(&host, ".p-megamenu-overlay"), 0);
    assert_eq!(focused(&host).as_deref(), Some("m_0"), "после листа фокус на первом корне");

    key(&list, "ArrowDown", "ArrowDown").await;
    assert!(is_active(&host, "m", "0"));
    key(&list, "Escape", "Escape").await;
    assert_eq!(count(&host, ".p-megamenu-overlay"), 0);
    assert_eq!(focused(&host).as_deref(), Some("m_0"), "Escape — на корень");

    // Tab закрывает всё.
    key(&list, "ArrowRight", "ArrowRight").await;
    key(&list, "ArrowRight", "ArrowRight").await;
    key(&list, "ArrowDown", "ArrowDown").await;
    assert!(is_active(&host, "m", "2"));
    key(&list, "Tab", "Tab").await;
    assert_eq!(count(&host, ".p-megamenu-overlay"), 0);
    host.remove();
}

async fn keyboard_vertical() {
    let (host, log) = (host(), Log::default());
    let items = model(&log);
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <MegaMenu breakpoint="1px" id="m" model=items.clone() orientation=MegaMenuOrientation::Vertical /> }
    });
    settle().await;
    assert!(q(&host, "#m").class_name().contains("p-megamenu-vertical"));
    let list = q(&host, "#m_list");
    list.focus().unwrap();
    settle().await;
    key(&list, "ArrowDown", "ArrowDown").await;
    assert_eq!(focused(&host).as_deref(), Some("m_1"), "↓ идёт по корню");
    key(&list, "ArrowUp", "ArrowUp").await;
    key(&list, "ArrowRight", "ArrowRight").await;
    assert!(is_active(&host, "m", "0"));
    assert_eq!(focused(&host).as_deref(), Some("m_0_0_0_0"), "→ раскрывает и входит");
    key(&list, "ArrowLeft", "ArrowLeft").await;
    assert_eq!(count(&host, ".p-megamenu-overlay"), 0);
    assert_eq!(focused(&host).as_deref(), Some("m_0"), "← из первой колонки — на корень");
    host.remove();
}

async fn mobile_button() {
    let (host, log) = (host(), Log::default());
    let items = model(&log);
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <MegaMenu breakpoint="100000px" id="m" model=items.clone() /> });
    settle().await;
    let root = q(&host, "#m");
    assert!(root.class_name().contains("p-megamenu-mobile") && !root.class_name().contains("p-megamenu-mobile-active"));
    let button = q(&host, ".p-megamenu-button");
    button.click();
    settle().await;
    assert!(root.class_name().contains("p-megamenu-mobile-active"));
    assert_eq!(button.get_attribute("aria-expanded").as_deref(), Some("true"));
    assert!(!q(&host, "#m_list").style().get_property_value("z-index").unwrap().is_empty(), "открытый список получает z-index");
    content(&host, "m", "0").click();
    settle().await;
    assert_eq!(count(&host, ".p-megamenu-col-12"), 3, "на узком окне колонки на всю ширину");
    assert!(root.class_name().contains("p-megamenu-mobile-active"), "группа раскрылась внутри открытой панели");
    mouse(&doc().body().unwrap(), "click", true);
    settle().await;
    assert!(!root.class_name().contains("p-megamenu-mobile-active"), "клик вне закрывает панель");
    button.click();
    settle().await;
    button.click();
    settle().await;
    assert_eq!(button.get_attribute("aria-expanded").as_deref(), Some("false"));
    assert_eq!(q(&host, "#m_list").style().get_property_value("z-index").unwrap(), "", "z-index снят");
    host.remove();
}

#[wasm_bindgen_test]
async fn megamenu_in_the_browser() {
    click_opens_and_closes_the_overlay().await;
    hover_follows_after_a_click().await;
    keyboard_horizontal().await;
    keyboard_vertical().await;
    mobile_button().await;
}
