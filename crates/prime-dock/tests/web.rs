//! Поведение Dock в реальном браузере (wasm-bindgen-test): клик и команда, отключённые пункты, фокус и клавиши во всех
//! положениях, `Home`/`End`/`Enter`/`Space`, события фокуса, мобильный режим по `matchMedia`.
#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use prime_dock::{Dock, DockCommand, DockItem, DockPosition};
use std::sync::{Arc, Mutex};
use wasm_bindgen::JsCast;
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
    host.query_selector(sel).unwrap().unwrap_or_else(|| panic!("нет {sel}")).dyn_into().unwrap()
}

fn list(host: &HtmlElement) -> HtmlElement {
    q(host, ".p-dock-list")
}

fn item(host: &HtmlElement, i: u32) -> HtmlElement {
    host.query_selector_all(".p-dock-item").unwrap().item(i).unwrap_or_else(|| panic!("нет пункта {i}")).dyn_into().unwrap()
}

/// Подпись пункта под фокусом.
fn focused(host: &HtmlElement) -> Option<String> {
    host.query_selector(".p-dock-item.p-focus").unwrap().and_then(|e| e.get_attribute("aria-label"))
}

/// Нажатие клавиши на списке; `false`, если действие по умолчанию отменено.
fn press(host: &HtmlElement, code: &str) -> bool {
    let init = KeyboardEventInit::new();
    init.set_code(code);
    init.set_key(code);
    init.set_bubbles(true);
    init.set_cancelable(true);
    list(host).dispatch_event(&KeyboardEvent::new_with_keyboard_event_init_dict("keydown", &init).unwrap()).unwrap()
}

fn model(log: &Log) -> Vec<DockItem> {
    let cmd = |name: &'static str, log: &Log| {
        let log = log.clone();
        Callback::new(move |c: DockCommand| log.lock().unwrap().push(format!("{name}:{}", c.item.label.unwrap_or_default())))
    };
    vec![
        DockItem::new().label("A").command(cmd("a", log)),
        DockItem::new().label("B").disabled(true).command(cmd("b", log)),
        DockItem::new().label("C").command(cmd("c", log)),
        DockItem::new().label("D").command(cmd("d", log)),
    ]
}

macro_rules! mount {
    ($host:expr, $log:expr, $($extra:tt)*) => {{
        let log = $log.clone();
        leptos::mount::mount_to($host.clone(), move || {
            let (f, b) = (log.clone(), log.clone());
            view! {
                <Dock
                    id="dk"
                    model=model(&log)
                    on_focus=Callback::new(move |_| f.lock().unwrap().push("focus".into()))
                    on_blur=Callback::new(move |_| b.lock().unwrap().push("blur".into()))
                    $($extra)*
                />
            }
        })
    }};
}

#[wasm_bindgen_test]
async fn click_runs_the_command_unless_the_item_is_disabled() {
    let (host, log) = (host(), Log::default());
    let _h = mount!(host, log,);
    leptos::task::tick().await;
    item(&host, 0).click();
    item(&host, 1).click();
    item(&host, 2).click();
    assert_eq!(*log.lock().unwrap(), ["a:A", "c:C"]);
    host.remove();
}

#[wasm_bindgen_test]
async fn focus_marks_the_first_enabled_item_and_blur_clears_it() {
    let (host, log) = (host(), Log::default());
    let _h = mount!(host, log,);
    leptos::task::tick().await;
    list(&host).focus().unwrap();
    leptos::task::tick().await;
    assert_eq!(focused(&host).as_deref(), Some("A"));
    assert_eq!(list(&host).get_attribute("aria-activedescendant").as_deref(), Some("dk_0"));
    assert_eq!(item(&host, 0).get_attribute("data-p-focused").as_deref(), Some("true"));
    list(&host).blur().unwrap();
    leptos::task::tick().await;
    assert_eq!(focused(&host), None);
    assert!(list(&host).get_attribute("aria-activedescendant").is_none());
    assert_eq!(*log.lock().unwrap(), ["focus", "blur"]);
    host.remove();
}

#[wasm_bindgen_test]
async fn horizontal_arrows_skip_disabled_items_and_stop_at_the_edges() {
    let (host, log) = (host(), Log::default());
    let _h = mount!(host, log,);
    leptos::task::tick().await;
    list(&host).focus().unwrap();
    assert!(!press(&host, "ArrowRight"), "действие по умолчанию отменено");
    leptos::task::tick().await;
    assert_eq!(focused(&host).as_deref(), Some("C"), "B отключён");
    press(&host, "ArrowRight");
    leptos::task::tick().await;
    assert_eq!(focused(&host).as_deref(), Some("D"));
    press(&host, "ArrowRight");
    leptos::task::tick().await;
    assert_eq!(focused(&host).as_deref(), Some("D"), "на границе остаётся");
    press(&host, "ArrowLeft");
    press(&host, "ArrowLeft");
    leptos::task::tick().await;
    assert_eq!(focused(&host).as_deref(), Some("A"));
    press(&host, "ArrowLeft");
    leptos::task::tick().await;
    assert_eq!(focused(&host).as_deref(), Some("A"));
    // поперечные стрелки перехватываются без действия
    assert!(!press(&host, "ArrowDown"));
    leptos::task::tick().await;
    assert_eq!(focused(&host).as_deref(), Some("A"));
    host.remove();
}

#[wasm_bindgen_test]
async fn vertical_docks_use_up_and_down() {
    for position in [DockPosition::Left, DockPosition::Right] {
        let (host, log) = (host(), Log::default());
        let _h = mount!(host, log, position = position);
        leptos::task::tick().await;
        list(&host).focus().unwrap();
        press(&host, "ArrowDown");
        leptos::task::tick().await;
        assert_eq!(focused(&host).as_deref(), Some("C"));
        press(&host, "ArrowUp");
        leptos::task::tick().await;
        assert_eq!(focused(&host).as_deref(), Some("A"));
        press(&host, "ArrowRight");
        leptos::task::tick().await;
        assert_eq!(focused(&host).as_deref(), Some("A"), "поперечная стрелка ничего не делает");
        host.remove();
    }
}

#[wasm_bindgen_test]
async fn home_end_and_enter_space_activate_the_focused_item() {
    let (host, log) = (host(), Log::default());
    let _h = mount!(host, log,);
    leptos::task::tick().await;
    list(&host).focus().unwrap();
    press(&host, "End");
    leptos::task::tick().await;
    assert_eq!(focused(&host).as_deref(), Some("D"));
    press(&host, "Enter");
    press(&host, "Home");
    leptos::task::tick().await;
    assert_eq!(focused(&host).as_deref(), Some("A"));
    press(&host, "Space");
    leptos::task::tick().await;
    assert_eq!(*log.lock().unwrap(), ["focus", "d:D", "a:A"]);
    host.remove();
}

#[wasm_bindgen_test]
async fn unhandled_keys_are_not_prevented() {
    let (host, log) = (host(), Log::default());
    let _h = mount!(host, log,);
    leptos::task::tick().await;
    list(&host).focus().unwrap();
    assert!(press(&host, "Tab"), "Tab не перехватывается");
    assert!(press(&host, "KeyA"));
    host.remove();
}

#[wasm_bindgen_test]
async fn mobile_class_follows_the_media_query() {
    let (host, log) = (host(), Log::default());
    let _wide = mount!(host, log, breakpoint = "100000px");
    leptos::task::tick().await;
    assert!(q(&host, ".p-dock").class_name().contains("p-dock-mobile"), "окно уже предела — мобильный режим");
    host.remove();
    let (host, log) = (self::host(), Log::default());
    let _narrow = mount!(host, log, breakpoint = "1px");
    leptos::task::tick().await;
    assert!(!q(&host, ".p-dock").class_name().contains("p-dock-mobile"));
    host.remove();
}
