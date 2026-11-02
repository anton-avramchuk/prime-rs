//! Поведение Menu в реальном браузере (wasm-bindgen-test): клик и команда, отключённые пункты, фокус и клавиши,
//! всплывающий режим (показ, клик снаружи, Escape, клик по пункту, `resize`, `z-index`, события).
#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use prime_menu::{Menu, MenuItem, MenuRef};
use std::sync::{Arc, Mutex};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;
use web_sys::{Event, HtmlElement, KeyboardEvent, KeyboardEventInit};

wasm_bindgen_test_configure!(run_in_browser);

type Log = Arc<Mutex<Vec<String>>>;

fn doc() -> web_sys::Document {
    web_sys::window().unwrap().document().unwrap()
}

fn host() -> HtmlElement {
    let el = doc().create_element("div").unwrap().dyn_into::<HtmlElement>().unwrap();
    el.style().set_property("position", "relative").unwrap();
    doc().body().unwrap().append_child(&el).unwrap();
    el
}

fn q(host: &HtmlElement, sel: &str) -> HtmlElement {
    host.query_selector(sel).unwrap().unwrap_or_else(|| panic!("нет {sel}")).dyn_into().unwrap()
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

fn press(el: &HtmlElement, code: &str, alt: bool) {
    let init = KeyboardEventInit::new();
    init.set_code(code);
    init.set_alt_key(alt);
    init.set_bubbles(true);
    init.set_cancelable(true);
    el.dispatch_event(&KeyboardEvent::new_with_keyboard_event_init_dict("keydown", &init).unwrap()).unwrap();
}

fn active_id() -> Option<String> {
    doc().active_element().map(|e| e.id())
}

fn model(log: &Log) -> Vec<MenuItem> {
    let (a, b) = (log.clone(), log.clone());
    vec![
        MenuItem::new()
            .label("New")
            .command(Callback::new(move |c: prime_menu::MenuCommand| a.lock().unwrap().push(format!("new:{}", c.item.label.unwrap())))),
        MenuItem::new().label("Off").disabled(true).command(Callback::new(move |_| b.lock().unwrap().push("off".into()))),
        MenuItem::separator(),
        MenuItem::new().label("Search").url("#search"),
        MenuItem::new().label("Hidden").visible(false),
    ]
}

async fn static_menu() {
    let host = host();
    let log: Log = Default::default();
    let l = log.clone();
    let handle = leptos::mount::mount_to(host.clone(), move || {
        let (f, b) = (l.clone(), l.clone());
        view! {
            <Menu
                id="m"
                model=model(&l)
                on_focus=Callback::new(move |()| f.lock().unwrap().push("focus".into()))
                on_blur=Callback::new(move |()| b.lock().unwrap().push("blur".into()))
            />
            <button id="other">"x"</button>
        }
    });
    leptos::task::tick().await;
    assert_eq!(count(&host, "li.p-menu-item"), 3);
    assert_eq!(count(&host, "li.p-menu-separator"), 1);
    assert!(!has(&host, ".p-menu-overlay"));

    // клик: команда, без url переход отменён
    q(&host, "#m_0 .p-menu-item-link").click();
    leptos::task::tick().await;
    assert_eq!(log.lock().unwrap().clone(), ["focus", "new:New"]);
    assert!(q(&host, "#m_0").class_name().contains("p-focus"));
    // отключённый
    q(&host, "#m_1 .p-menu-item-content").click();
    assert!(!log.lock().unwrap().contains(&"off".to_string()));
    assert!(q(&host, "#m_1").class_name().contains("p-disabled"));

    // фокус списка: первый доступный пункт (сперва сбросим состояние клика)
    q(&host, "#other").focus().unwrap();
    leptos::task::tick().await;
    let list = q(&host, "ul.p-menu-list");
    list.focus().unwrap();
    leptos::task::tick().await;
    assert_eq!(list.get_attribute("aria-activedescendant").as_deref(), Some("m_0"));
    assert_eq!(q(&host, "#m_0").get_attribute("data-p-focused").as_deref(), Some("true"));
    // ↓ пропускает отключённый пункт, дальше последний остаётся последним
    press(&list, "ArrowDown", false);
    leptos::task::tick().await;
    assert_eq!(list.get_attribute("aria-activedescendant").as_deref(), Some("m_3"));
    press(&list, "ArrowDown", false);
    leptos::task::tick().await;
    assert_eq!(list.get_attribute("aria-activedescendant").as_deref(), Some("m_3"));
    press(&list, "Home", false);
    leptos::task::tick().await;
    assert_eq!(list.get_attribute("aria-activedescendant").as_deref(), Some("m_0"));
    press(&list, "End", false);
    leptos::task::tick().await;
    assert_eq!(list.get_attribute("aria-activedescendant").as_deref(), Some("m_3"));
    press(&list, "ArrowUp", false);
    leptos::task::tick().await;
    assert_eq!(list.get_attribute("aria-activedescendant").as_deref(), Some("m_0"));
    // Enter запускает команду фокусного пункта
    log.lock().unwrap().clear();
    press(&list, "Enter", false);
    leptos::task::tick().await;
    assert_eq!(log.lock().unwrap().clone(), ["new:New"]);

    // потеря фокуса снимает состояние
    q(&host, "#other").focus().unwrap();
    leptos::task::tick().await;
    assert!(log.lock().unwrap().contains(&"blur".to_string()));
    assert!(list.get_attribute("aria-activedescendant").is_none());
    assert!(!q(&host, "#m_0").class_name().contains("p-focus"));
    drop(handle);
    host.remove();
}

struct Popup {
    host: HtmlElement,
    r: MenuRef,
    log: Log,
}

fn mount_popup() -> Popup {
    let host = host();
    let log: Log = Default::default();
    let slot = Arc::new(Mutex::new(None));
    let (s, l) = (slot.clone(), log.clone());
    let handle = leptos::mount::mount_to(host.clone(), move || {
        let r = MenuRef::new();
        *s.lock().unwrap() = Some(r);
        let (a, b) = (l.clone(), l.clone());
        view! {
            <button id="btn" on:click=move |e| r.toggle(&e)>"menu"</button>
            <div id="outside">"outside"</div>
            <Menu
                id="p"
                popup=true
                menu_ref=r
                model=model(&l)
                on_show=Callback::new(move |()| a.lock().unwrap().push("show".into()))
                on_hide=Callback::new(move |()| b.lock().unwrap().push("hide".into()))
            />
        }
    });
    std::mem::forget(handle);
    let r = slot.lock().unwrap().take().unwrap();
    Popup { host, r, log }
}

async fn popup_toggle_outside_escape_and_item() {
    let Popup { host, r: _, log } = mount_popup();
    leptos::task::tick().await;
    assert!(!has(&host, ".p-menu"));

    q(&host, "#btn").click();
    leptos::task::tick().await;
    let root = q(&host, ".p-menu");
    assert!(root.class_name().contains("p-menu-overlay"));
    assert_eq!(root.style().get_property_value("position").unwrap(), "absolute");
    assert!(!root.style().get_property_value("z-index").unwrap().is_empty());
    assert_eq!(active_id().as_deref(), Some("p_list"));
    assert_eq!(log.lock().unwrap().clone(), ["show"]);

    // клик по кнопке переключает
    q(&host, "#btn").click();
    leptos::task::tick().await;
    assert!(!has(&host, ".p-menu"));
    assert_eq!(log.lock().unwrap().clone(), ["show", "hide"]);

    // клик по пункту закрывает и вызывает команду
    q(&host, "#btn").click();
    leptos::task::tick().await;
    sleep(20).await;
    q(&host, "#p_0 .p-menu-item-link").click();
    leptos::task::tick().await;
    assert!(!has(&host, ".p-menu"));
    assert!(log.lock().unwrap().contains(&"new:New".to_string()));

    // клик снаружи
    log.lock().unwrap().clear();
    q(&host, "#btn").click();
    leptos::task::tick().await;
    sleep(20).await;
    q(&host, "#outside").click();
    leptos::task::tick().await;
    assert!(!has(&host, ".p-menu"));
    assert_eq!(log.lock().unwrap().clone(), ["show", "hide"]);

    // клик внутри панели вне пунктов не закрывает
    q(&host, "#btn").click();
    leptos::task::tick().await;
    sleep(20).await;
    q(&host, ".p-menu-list").click();
    leptos::task::tick().await;
    assert!(has(&host, ".p-menu"));

    // Escape закрывает и возвращает фокус на кнопку
    press(&q(&host, ".p-menu-list"), "Escape", false);
    leptos::task::tick().await;
    assert!(!has(&host, ".p-menu"));
    assert_eq!(active_id().as_deref(), Some("btn"));

    // Alt+↑ — то же
    q(&host, "#btn").click();
    leptos::task::tick().await;
    press(&q(&host, ".p-menu-list"), "ArrowUp", true);
    leptos::task::tick().await;
    assert!(!has(&host, ".p-menu"));

    // resize закрывает (на настольном устройстве)
    q(&host, "#btn").click();
    leptos::task::tick().await;
    sleep(20).await;
    web_sys::window().unwrap().dispatch_event(&Event::new("resize").unwrap()).unwrap();
    leptos::task::tick().await;
    let touch = web_sys::window().unwrap().navigator().max_touch_points() > 0;
    assert_eq!(has(&host, ".p-menu"), touch);
    host.remove();
}

async fn popup_keys_and_programmatic_hide() {
    let Popup { host, r, log } = mount_popup();
    leptos::task::tick().await;
    q(&host, "#btn").click();
    leptos::task::tick().await;
    let list = q(&host, ".p-menu-list");
    // popup: при фокусе пункт не выбирается сам, первая ↓ выбирает первый
    assert!(list.get_attribute("aria-activedescendant").is_none());
    press(&list, "ArrowDown", false);
    leptos::task::tick().await;
    assert_eq!(list.get_attribute("aria-activedescendant").as_deref(), Some("p_0"));
    log.lock().unwrap().clear();
    press(&list, "Enter", false);
    leptos::task::tick().await;
    assert!(!has(&host, ".p-menu"));
    assert!(log.lock().unwrap().contains(&"new:New".to_string()));
    assert_eq!(active_id().as_deref(), Some("btn"));

    q(&host, "#btn").click();
    leptos::task::tick().await;
    assert!(r.visible());
    r.hide();
    leptos::task::tick().await;
    assert!(!has(&host, ".p-menu"));
    host.remove();
}

#[wasm_bindgen_test]
async fn menu_behaviour() {
    static_menu().await;
    popup_toggle_outside_escape_and_item().await;
    popup_keys_and_programmatic_hide().await;
}
