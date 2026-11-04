//! Поведение CascadeSelect в реальном браузере (wasm-bindgen-test): открытие, каскад кликом и наведением, выбор листа,
//! очистка, клик вне, клавиатура и мобильный класс.
//!
//! Сценарии идут одним последовательным тестом: слушатели окна и фокус общие на страницу. Стили темы не подключены.
#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use prime_cascadeselect::{CascadeOption, CascadeSelect};
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

/// Даёт эффектам примениться и слушателям окна повеситься.
async fn settle() {
    sleep(60).await;
}

fn label(host: &HtmlElement) -> String {
    q(host, ".p-cascadeselect-label").text_content().unwrap_or_default().trim().to_owned()
}

fn id_of(host: &HtmlElement, key: &str) -> String {
    let root_id = q(host, ".p-cascadeselect").id();
    format!("{root_id}_{key}")
}

fn option(host: &HtmlElement, key: &str) -> HtmlElement {
    q(host, &format!("li[id=\"{}\"]", id_of(host, key)))
}

fn content(host: &HtmlElement, key: &str) -> HtmlElement {
    option(host, key).query_selector(":scope > .p-cascadeselect-option-content").unwrap().unwrap().dyn_into().unwrap()
}

fn focused(host: &HtmlElement) -> Option<String> {
    host.query_selector("li.p-focus").unwrap().map(|e| e.id().rsplit_once('_').map(|_| e.id()).unwrap_or_default()).map(|id| {
        let root_id = q(host, ".p-cascadeselect").id();
        id.strip_prefix(&format!("{root_id}_")).unwrap_or(&id).to_owned()
    })
}

fn mouse(el: &web_sys::Element, name: &str, bubbles: bool) {
    let init = MouseEventInit::new();
    init.set_bubbles(bubbles);
    el.dispatch_event(&MouseEvent::new_with_mouse_event_init_dict(name, &init).unwrap()).unwrap();
}

/// Нажатие клавиши и ожидание реактивных обновлений.
async fn key(el: &HtmlElement, code: &str, key: &str) {
    let init = KeyboardEventInit::new();
    init.set_code(code);
    init.set_key(key);
    init.set_bubbles(true);
    init.set_cancelable(true);
    el.dispatch_event(&KeyboardEvent::new_with_keyboard_event_init_dict("keydown", &init).unwrap()).unwrap();
    sleep(20).await;
}

fn leaf(l: &str) -> CascadeOption {
    CascadeOption::new(l)
}

/// 0 Australia{ 0_0 NSW{Sydney, Newcastle(off), Wollongong(w)}, 0_1 Queensland{Brisbane} }, 1 Canada{ Ontario{Toronto} }, 2 Plain
fn options() -> Vec<CascadeOption> {
    vec![
        CascadeOption::new("Australia").children(vec![
            CascadeOption::new("New South Wales").children(vec![
                leaf("Sydney"),
                leaf("Newcastle").disabled(true),
                leaf("Wollongong").value("w"),
            ]),
            CascadeOption::new("Queensland").children(vec![leaf("Brisbane")]),
        ]),
        CascadeOption::new("Canada").children(vec![CascadeOption::new("Ontario").children(vec![leaf("Toronto")])]),
        leaf("Plain"),
    ]
}

async fn opens_the_panel() {
    let host = host();
    let value = RwSignal::new(None::<String>);
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! {
            <CascadeSelect
                breakpoint="1px"
                options=options()
                value=value
                placeholder="Pick"
            />
        }
    });
    settle().await;
    assert_eq!(label(&host), "Pick");
    q(&host, ".p-cascadeselect").click();
    settle().await;
    assert!(has(&host, ".p-cascadeselect-overlay") && q(&host, ".p-cascadeselect").class_name().contains("p-cascadeselect-open"));
    assert_eq!(count(&host, ".p-cascadeselect-list-container > ul.p-cascadeselect-list > li"), 3);
    assert_eq!(q(&host, "input").get_attribute("aria-expanded").as_deref(), Some("true"));
    assert!(!q(&host, ".p-cascadeselect-overlay").style().get_property_value("z-index").unwrap().is_empty());
    assert!(option(&host, "0").class_name().contains("p-cascadeselect-option-group"));
    assert!(!option(&host, "2").class_name().contains("p-cascadeselect-option-group"));
    assert!(!has(&host, &format!("li[id=\"{}\"]", id_of(&host, "0_0"))), "вложенные списки выводятся только у раскрытых групп");
    host.remove();
}

async fn cascade_and_leaf_selection() {
    let (host, log) = (host(), Log::default());
    let value = RwSignal::new(None::<String>);
    let (l1, l2, l3) = (log.clone(), log.clone(), log.clone());
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! {
            <CascadeSelect
                breakpoint="1px"
                options=options()
                value=value
                on_show=Callback::new(move |_| l1.lock().unwrap().push("show".into()))
                on_hide=Callback::new(move |_| l2.lock().unwrap().push("hide".into()))
                on_change=Callback::new(move |v: Option<String>| l3.lock().unwrap().push(format!("change:{}", v.unwrap_or_default())))
            />
        }
    });
    settle().await;
    q(&host, ".p-cascadeselect").click();
    settle().await;
    content(&host, "0").click();
    settle().await;
    assert!(option(&host, "0").class_name().contains("p-cascadeselect-option-active"));
    assert_eq!(count(&host, "ul.p-cascadeselect-option-list"), 1);
    content(&host, "0_0").click();
    settle().await;
    assert_eq!(count(&host, "ul.p-cascadeselect-option-list"), 2, "два вложенных списка");
    assert_eq!(count(&host, "li[role=\"treeitem\"]"), 3 + 2 + 3);
    assert_eq!(option(&host, "0_0_0").get_attribute("aria-level").as_deref(), Some("3"));
    content(&host, "0_1").click();
    settle().await;
    assert!(!has(&host, &format!("li[id=\"{}\"]", id_of(&host, "0_0_0"))), "соседняя группа закрывает прежнюю");
    content(&host, "0_1").click();
    settle().await;
    assert_eq!(count(&host, "ul.p-cascadeselect-option-list"), 1, "повторный клик сворачивает группу");
    content(&host, "0_0").click();
    settle().await;
    content(&host, "0_0_1").click();
    settle().await;
    assert_eq!(value.get_untracked(), None, "отключённый лист не выбирается");
    content(&host, "0_0_2").click();
    settle().await;
    assert_eq!(value.get_untracked().as_deref(), Some("w"));
    assert!(!has(&host, ".p-cascadeselect-overlay"), "выбор листа закрывает панель");
    assert_eq!(q(&host, ".p-cascadeselect-label").text_content().unwrap(), "Wollongong");
    assert_eq!(log.lock().unwrap().as_slice(), ["show", "change:w", "hide"]);
    q(&host, ".p-cascadeselect").click();
    settle().await;
    assert!(option(&host, "0_0_2").class_name().contains("p-cascadeselect-option-selected"), "при открытии раскрыт путь к выбранному");
    assert_eq!(option(&host, "0_0_2").get_attribute("aria-selected").as_deref(), Some("true"));
    mouse(&doc().body().unwrap().into(), "click", true);
    settle().await;
    assert!(!has(&host, ".p-cascadeselect-overlay"), "клик вне закрывает");
    host.remove();
}

async fn hover_rules_and_clear() {
    let host = host();
    let value = RwSignal::new(None::<String>);
    let _h = leptos::mount::mount_to(
        host.clone(),
        move || view! { <CascadeSelect breakpoint="1px" options=options() value=value show_clear=true /> },
    );
    settle().await;
    q(&host, ".p-cascadeselect").click();
    settle().await;
    mouse(&content(&host, "0"), "mouseenter", false);
    settle().await;
    assert!(option(&host, "0").class_name().contains("p-cascadeselect-option-active"), "до клика наведение раскрывает корневую группу");
    mouse(&content(&host, "0_0"), "mouseenter", false);
    settle().await;
    assert_eq!(
        count(&host, "ul.p-cascadeselect-option-list"),
        2,
        "раскрытие корневой группы наведением включает наведение для вложенных (dirty, как в PrimeNG)"
    );
    mouse(&content(&host, "0_1"), "mouseenter", false);
    settle().await;
    assert!(
        option(&host, "0_1").class_name().contains("p-cascadeselect-option-active")
            && !option(&host, "0_0").class_name().contains("p-cascadeselect-option-active"),
        "наведение на соседнюю группу переключает"
    );
    content(&host, "2").click();
    settle().await;
    assert_eq!(value.get_untracked().as_deref(), Some("Plain"));
    assert!(has(&host, ".p-cascadeselect-clear-icon"));
    mouse(&host.query_selector(".p-cascadeselect-clear-icon").unwrap().unwrap(), "click", true);
    settle().await;
    assert_eq!(value.get_untracked(), None);
    assert!(!has(&host, ".p-cascadeselect-overlay"), "очистка не открывает панель");
    assert!(!has(&host, ".p-cascadeselect-clear-icon"));
    host.remove();
}

async fn keyboard_walks_the_cascade() {
    let (host, log) = (host(), Log::default());
    let value = RwSignal::new(None::<String>);
    let l = log.clone();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! {
            <CascadeSelect
                breakpoint="1px"
                options=options()
                value=value
                show_clear=true
                on_change=Callback::new(move |v: Option<String>| l.lock().unwrap().push(v.unwrap_or_default()))
            />
        }
    });
    settle().await;
    let input = q(&host, "input");
    input.focus().unwrap();
    settle().await;
    assert!(q(&host, ".p-cascadeselect").class_name().contains("p-focus"));
    key(&input, "ArrowDown", "ArrowDown").await;
    settle().await;
    assert!(has(&host, ".p-cascadeselect-overlay"), "↓ открывает панель");
    key(&input, "ArrowDown", "ArrowDown").await;
    assert_eq!(focused(&host).as_deref(), Some("0"));
    assert!(option(&host, "0").class_name().contains("p-cascadeselect-option-active"), "фокус на группе раскрывает её");
    key(&input, "ArrowRight", "ArrowRight").await;
    assert_eq!(focused(&host).as_deref(), Some("0_0"), "→ входит в раскрытую группу");
    key(&input, "ArrowRight", "ArrowRight").await;
    assert_eq!(focused(&host).as_deref(), Some("0_0_0"));
    key(&input, "ArrowDown", "ArrowDown").await;
    assert_eq!(focused(&host).as_deref(), Some("0_0_2"), "отключённый пропущен");
    key(&input, "ArrowLeft", "ArrowLeft").await;
    assert_eq!(focused(&host).as_deref(), Some("0_0"), "← возвращает на группу-родителя");
    key(&input, "End", "End").await;
    assert_eq!(focused(&host).as_deref(), Some("0_1"));
    key(&input, "ArrowRight", "ArrowRight").await;
    key(&input, "ArrowRight", "ArrowRight").await;
    assert_eq!(focused(&host).as_deref(), Some("0_1_0"));
    key(&input, "Enter", "Enter").await;
    settle().await;
    assert_eq!(value.get_untracked().as_deref(), Some("Brisbane"), "Enter на листе выбирает значение");
    assert!(!has(&host, ".p-cascadeselect-overlay"));
    assert_eq!(log.lock().unwrap().as_slice(), ["Brisbane"]);

    key(&input, "Space", " ").await;
    settle().await;
    assert!(has(&host, ".p-cascadeselect-overlay") && focused(&host).as_deref() == Some("0_1_0"), "при открытии фокус на выбранном");
    key(&input, "Escape", "Escape").await;
    settle().await;
    assert!(!has(&host, ".p-cascadeselect-overlay"), "Escape закрывает");
    key(&input, "KeyP", "p").await;
    settle().await;
    assert!(has(&host, ".p-cascadeselect-overlay"), "печатный символ открывает панель");
    assert_eq!(focused(&host).as_deref(), Some("0_1_0"), "поиск идёт по списку уровня выбранного, совпадений нет — фокус на выбранном");
    key(&input, "KeyB", "b").await;
    assert_eq!(focused(&host).as_deref(), Some("0_1_0"), "поиск по началу подписи находит Brisbane");
    key(&input, "Tab", "Tab").await;
    settle().await;
    assert!(!has(&host, ".p-cascadeselect-overlay"), "Tab закрывает");
    key(&input, "Backspace", "Backspace").await;
    assert_eq!(value.get_untracked(), None, "Backspace очищает при show_clear");
    host.remove();
}

async fn disabled_and_mobile() {
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <CascadeSelect breakpoint="1px" options=options() disabled=true /> });
    settle().await;
    q(&host, ".p-cascadeselect").click();
    settle().await;
    assert!(!has(&host, ".p-cascadeselect-overlay"), "отключённое поле не открывается");
    host.remove();

    let host = self::host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <CascadeSelect breakpoint="100000px" options=options() /> });
    settle().await;
    assert!(q(&host, ".p-cascadeselect").class_name().contains("p-cascadeselect-mobile"));
    q(&host, ".p-cascadeselect").click();
    settle().await;
    assert!(q(&host, ".p-cascadeselect-overlay").class_name().contains("p-cascadeselect-mobile-active"));
    mouse(&content(&host, "0"), "mouseenter", false);
    settle().await;
    assert_eq!(count(&host, "ul.p-cascadeselect-option-list"), 0, "на узком окне наведение путь не меняет");
    content(&host, "0").click();
    settle().await;
    assert_eq!(count(&host, "ul.p-cascadeselect-option-list"), 1, "клик раскрывает");
    host.remove();
}

#[wasm_bindgen_test]
async fn cascadeselect_in_the_browser() {
    opens_the_panel().await;
    cascade_and_leaf_selection().await;
    hover_rules_and_clear().await;
    keyboard_walks_the_cascade().await;
    disabled_and_mobile().await;
}
