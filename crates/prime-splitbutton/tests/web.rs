//! Поведение SplitButton в реальном браузере (wasm-bindgen-test): клики по обеим кнопкам, клавиши, закрытие снаружи, команды
//! пунктов, события показа и скрытия, положение панели.
//!
//! Сценарии идут одним последовательным тестом: слушатели `window` у открытых меню общие на страницу.
#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use prime_menu::{MenuCommand, MenuItem};
use prime_splitbutton::SplitButton;
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

fn has(host: &HtmlElement, sel: &str) -> bool {
    host.query_selector(sel).unwrap().is_some()
}

/// Нажатие клавиши на элементе; `false`, если действие по умолчанию отменено.
fn key(el: &HtmlElement, code: &str) -> bool {
    let init = KeyboardEventInit::new();
    init.set_code(code);
    init.set_key(code);
    init.set_bubbles(true);
    init.set_cancelable(true);
    el.dispatch_event(&KeyboardEvent::new_with_keyboard_event_init_dict("keydown", &init).unwrap()).unwrap()
}

fn expanded(host: &HtmlElement) -> String {
    q(host, ".p-splitbutton-dropdown").get_attribute("aria-expanded").unwrap_or_default()
}

fn model(log: &Log) -> Vec<MenuItem> {
    let (a, b) = (log.clone(), log.clone());
    vec![
        MenuItem::new().label("Update").command(Callback::new(move |_: MenuCommand| a.lock().unwrap().push("update".into()))),
        MenuItem::new().label("Delete").command(Callback::new(move |_: MenuCommand| b.lock().unwrap().push("delete".into()))),
    ]
}

/// Монтирует SplitButton с журналом событий.
macro_rules! mount {
    ($host:expr, $log:expr, $($extra:tt)*) => {{
        let log = $log.clone();
        leptos::mount::mount_to($host.clone(), move || {
            let (a, b, c, d) = (log.clone(), log.clone(), log.clone(), log.clone());
            view! {
                <SplitButton
                    label="Save"
                    model=model(&log)
                    on_click=Callback::new(move |_| a.lock().unwrap().push("click".into()))
                    on_dropdown_click=Callback::new(move |_| b.lock().unwrap().push("dropdown".into()))
                    on_menu_show=Callback::new(move |()| c.lock().unwrap().push("show".into()))
                    on_menu_hide=Callback::new(move |()| d.lock().unwrap().push("hide".into()))
                    $($extra)*
                />
            }
        })
    }};
}

/// Пауза: слушатели `window` у меню подключаются через `set_timeout(0)` после показа.
async fn pause() {
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, 30).unwrap();
    });
    wasm_bindgen_futures::JsFuture::from(promise).await.unwrap();
}

async fn settle() {
    leptos::task::tick().await;
    leptos::task::tick().await;
    leptos::task::tick().await;
}

async fn dropdown_opens_closes_and_reports() {
    let (host, log) = (host(), Log::default());
    let _h = mount!(host, log,);
    settle().await;
    assert_eq!(expanded(&host), "false");
    assert!(!has(&host, ".p-menu-overlay"));
    q(&host, ".p-splitbutton-dropdown").click();
    settle().await;
    assert!(has(&host, ".p-menu-overlay"), "меню открыто");
    assert_eq!(expanded(&host), "true");
    assert_eq!(*log.lock().unwrap(), ["dropdown", "show"]);
    q(&host, ".p-splitbutton-dropdown").click();
    settle().await;
    assert!(!has(&host, ".p-menu-overlay"), "повторный клик закрывает меню");
    assert_eq!(expanded(&host), "false");
    assert_eq!(*log.lock().unwrap(), ["dropdown", "show", "dropdown", "hide"]);
    host.remove();
}

async fn main_button_click_reports_and_closes_the_menu() {
    let (host, log) = (host(), Log::default());
    let _h = mount!(host, log,);
    settle().await;
    q(&host, ".p-splitbutton-dropdown").click();
    settle().await;
    assert!(has(&host, ".p-menu-overlay"));
    q(&host, ".p-splitbutton-button").click();
    settle().await;
    assert!(!has(&host, ".p-menu-overlay"), "основная кнопка закрывает меню");
    assert_eq!(*log.lock().unwrap(), ["dropdown", "show", "click", "hide"]);
    host.remove();
}

async fn arrow_keys_toggle_the_menu_and_prevent_the_default_action() {
    let (host, log) = (host(), Log::default());
    let _h = mount!(host, log,);
    settle().await;
    let dropdown = q(&host, ".p-splitbutton-dropdown");
    assert!(!key(&dropdown, "ArrowDown"), "действие по умолчанию отменено");
    settle().await;
    assert!(has(&host, ".p-menu-overlay"), "стрелка вниз открыла меню");
    assert!(!key(&dropdown, "ArrowUp"));
    settle().await;
    assert!(!has(&host, ".p-menu-overlay"), "стрелка вверх переключила меню обратно");
    assert!(key(&dropdown, "Enter"), "другие клавиши не отменяются");
    host.remove();
}

async fn click_outside_closes_the_menu() {
    let (host, log) = (host(), Log::default());
    let _h = mount!(host, log,);
    settle().await;
    q(&host, ".p-splitbutton-dropdown").click();
    settle().await;
    assert!(has(&host, ".p-menu-overlay"));
    pause().await;
    doc().body().unwrap().click();
    settle().await;
    assert!(!has(&host, ".p-menu-overlay"));
    assert_eq!(expanded(&host), "false");
    assert!(log.lock().unwrap().contains(&"hide".to_owned()));
    host.remove();
}

async fn choosing_an_item_runs_its_command_and_closes_the_menu() {
    let (host, log) = (host(), Log::default());
    let _h = mount!(host, log,);
    settle().await;
    q(&host, ".p-splitbutton-dropdown").click();
    settle().await;
    q(&host, ".p-menu-item-link").click();
    settle().await;
    assert!(log.lock().unwrap().contains(&"update".to_owned()), "команда первого пункта выполнена");
    assert!(!has(&host, ".p-menu-overlay"), "выбор пункта закрыл меню");
    assert_eq!(expanded(&host), "false");
    host.remove();
}

async fn disabled_dropdown_does_not_open() {
    let (host, log) = (host(), Log::default());
    let _h = mount!(host, log, menu_button_disabled = true);
    settle().await;
    q(&host, ".p-splitbutton-dropdown").click();
    settle().await;
    assert!(!has(&host, ".p-menu-overlay"));
    assert!(log.lock().unwrap().is_empty());
    host.remove();
}

async fn panel_is_placed_inside_the_root_with_a_layer() {
    let (host, log) = (host(), Log::default());
    let _h = mount!(host, log,);
    settle().await;
    q(&host, ".p-splitbutton-dropdown").click();
    settle().await;
    let overlay = q(&host, ".p-menu-overlay");
    assert!(q(&host, ".p-splitbutton").contains(Some(&overlay)), "панель внутри корня");
    let z: i32 = overlay.style().get_property_value("z-index").unwrap().parse().unwrap();
    assert!(z >= 1000, "слой группы меню: {z}");
    assert!(!overlay.style().get_property_value("top").unwrap().is_empty(), "положение рассчитано");
    host.remove();
}

/// Все сценарии подряд: меню закрывается в конце каждого.
#[wasm_bindgen_test]
async fn splitbutton_scenarios() {
    dropdown_opens_closes_and_reports().await;
    main_button_click_reports_and_closes_the_menu().await;
    arrow_keys_toggle_the_menu_and_prevent_the_default_action().await;
    click_outside_closes_the_menu().await;
    choosing_an_item_runs_its_command_and_closes_the_menu().await;
    disabled_dropdown_does_not_open().await;
    panel_is_placed_inside_the_root_with_a_layer().await;
}
