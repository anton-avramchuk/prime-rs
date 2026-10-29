//! Поведение SelectButton в реальном браузере (wasm-bindgen-test): выбор и снятие, `allow_empty`, множественный выбор,
//! `disabled`, клавиши, события, синхронность `aria-pressed` и реакция на внешние сигналы.
#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use prime_selectbutton::{SelectButton, SelectButtonChange, SelectButtonOption, SelectButtonOptionClick, SelectButtonValue};
use std::sync::{Arc, Mutex};
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use web_sys::{HtmlElement, KeyboardEvent, KeyboardEventInit};

wasm_bindgen_test_configure!(run_in_browser);

fn doc() -> web_sys::Document {
    web_sys::window().unwrap().document().unwrap()
}

fn host() -> HtmlElement {
    let el = doc().create_element("div").unwrap().dyn_into::<HtmlElement>().unwrap();
    doc().body().unwrap().append_child(&el).unwrap();
    el
}

/// Кнопка с номером `n` (с нуля).
fn button(host: &HtmlElement, n: u32) -> HtmlElement {
    host.query_selector(&format!(".p-selectbutton > .p-togglebutton:nth-child({})", n + 1))
        .unwrap()
        .unwrap_or_else(|| panic!("нет кнопки {n}"))
        .dyn_into()
        .unwrap()
}

/// Строка состояния: по символу на кнопку (`1` — включена).
fn pressed(host: &HtmlElement) -> String {
    let list = host.query_selector_all(".p-selectbutton > .p-togglebutton").unwrap();
    (0..list.length())
        .map(|i| {
            let el: HtmlElement = list.item(i).unwrap().dyn_into().unwrap();
            if el.get_attribute("aria-pressed").as_deref() == Some("true") { '1' } else { '0' }
        })
        .collect()
}

fn options() -> Vec<SelectButtonOption> {
    vec![
        SelectButtonOption::new("One-Way", "one-way"),
        SelectButtonOption::new("Return", "return"),
        SelectButtonOption::new("Multi", "multi"),
    ]
}

fn press(el: &HtmlElement, code: &str) {
    let init = KeyboardEventInit::new();
    init.set_code(code);
    init.set_bubbles(true);
    init.set_cancelable(true);
    el.dispatch_event(&KeyboardEvent::new_with_keyboard_event_init_dict("keydown", &init).unwrap()).unwrap();
}

/// Журнал событий: `change:<значение>` и `click:<индекс>`.
fn log() -> (Arc<Mutex<Vec<String>>>, Callback<SelectButtonChange>, Callback<SelectButtonOptionClick>) {
    let log = Arc::new(Mutex::new(Vec::new()));
    let (a, b) = (log.clone(), log.clone());
    (
        log,
        Callback::new(move |e: SelectButtonChange| {
            let v = match e.value {
                SelectButtonValue::Single(v) => format!("{v:?}"),
                SelectButtonValue::Multiple(v) => v.join(","),
            };
            a.lock().unwrap().push(format!("change:{v}"));
        }),
        Callback::new(move |e: SelectButtonOptionClick| b.lock().unwrap().push(format!("click:{}", e.index))),
    )
}

#[wasm_bindgen_test]
async fn single_selection_switches_and_clears() {
    let value = RwSignal::new(Some("one-way".to_owned()));
    let (log, on_change, on_click) = log();
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <SelectButton options=options() value=value on_change=on_change on_option_click=on_click /> }
    });
    leptos::task::tick().await;
    assert_eq!(pressed(&host), "100");
    button(&host, 1).click();
    leptos::task::tick().await;
    assert_eq!(value.get_untracked().as_deref(), Some("return"));
    assert_eq!(pressed(&host), "010");
    button(&host, 1).click();
    leptos::task::tick().await;
    assert_eq!(value.get_untracked(), None);
    assert_eq!(pressed(&host), "000");
    assert_eq!(*log.lock().unwrap(), ["change:Some(\"return\")", "click:1", "change:None", "click:1"]);
    host.remove();
}

#[wasm_bindgen_test]
async fn uncontrolled_keeps_internal_state() {
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <SelectButton options=options() default_value="multi" /> });
    leptos::task::tick().await;
    assert_eq!(pressed(&host), "001");
    button(&host, 0).click();
    leptos::task::tick().await;
    assert_eq!(pressed(&host), "100");
    host.remove();
}

#[wasm_bindgen_test]
async fn allow_empty_false_rejects_and_keeps_button_pressed() {
    let value = RwSignal::new(Some("return".to_owned()));
    let (log, on_change, on_click) = log();
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <SelectButton options=options() value=value allow_empty=false on_change=on_change on_option_click=on_click /> }
    });
    leptos::task::tick().await;
    button(&host, 1).click();
    leptos::task::tick().await;
    assert_eq!(value.get_untracked().as_deref(), Some("return"));
    assert_eq!(pressed(&host), "010", "aria-pressed разошёлся со значением");
    assert!(log.lock().unwrap().is_empty());
    button(&host, 2).click();
    leptos::task::tick().await;
    assert_eq!(pressed(&host), "001");
    assert_eq!(log.lock().unwrap().len(), 2);
    host.remove();
}

#[wasm_bindgen_test]
async fn multiple_adds_in_order_and_removes() {
    let values = RwSignal::new(vec!["one-way".to_owned()]);
    let (log, on_change, _) = log();
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <SelectButton options=options() multiple=true values=values on_change=on_change /> }
    });
    leptos::task::tick().await;
    button(&host, 2).click();
    button(&host, 1).click();
    leptos::task::tick().await;
    assert_eq!(values.get_untracked(), ["one-way", "multi", "return"]);
    assert_eq!(pressed(&host), "111");
    button(&host, 0).click();
    leptos::task::tick().await;
    assert_eq!(values.get_untracked(), ["multi", "return"]);
    assert_eq!(pressed(&host), "011");
    assert_eq!(log.lock().unwrap().last().map(String::as_str), Some("change:multi,return"));
    host.remove();
}

#[wasm_bindgen_test]
async fn multiple_allow_empty_false_keeps_the_last_value() {
    let values = RwSignal::new(vec!["return".to_owned()]);
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <SelectButton options=options() multiple=true values=values allow_empty=false /> }
    });
    leptos::task::tick().await;
    button(&host, 1).click();
    leptos::task::tick().await;
    assert_eq!(values.get_untracked(), ["return"]);
    assert_eq!(pressed(&host), "010");
    button(&host, 0).click();
    button(&host, 1).click();
    leptos::task::tick().await;
    assert_eq!(values.get_untracked(), ["one-way"]);
    assert_eq!(pressed(&host), "100");
    host.remove();
}

#[wasm_bindgen_test]
async fn disabled_option_and_disabled_component_do_not_change_value() {
    let value = RwSignal::new(None::<String>);
    let (log, on_change, _) = log();
    let host = host();
    let opts = vec![SelectButtonOption::new("A", "a"), SelectButtonOption::new("B", "b").disabled(true)];
    let o2 = opts.clone();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! {
            <div id="opt"><SelectButton options=opts.clone() value=value on_change=on_change /></div>
            <div id="all"><SelectButton options=o2.clone() value=value disabled=true on_change=on_change /></div>
        }
    });
    leptos::task::tick().await;
    let q = |sel: &str| -> HtmlElement { host.query_selector(sel).unwrap().unwrap().dyn_into().unwrap() };
    q("#opt .p-togglebutton:nth-child(2)").click();
    q("#all .p-togglebutton:nth-child(1)").click();
    press(&q("#opt .p-togglebutton:nth-child(2)"), "Enter");
    leptos::task::tick().await;
    assert_eq!(value.get_untracked(), None);
    assert!(log.lock().unwrap().is_empty());
    q("#opt .p-togglebutton:nth-child(1)").click();
    leptos::task::tick().await;
    assert_eq!(value.get_untracked().as_deref(), Some("a"));
    host.remove();
}

#[wasm_bindgen_test]
async fn enter_and_space_select_once_per_press() {
    let value = RwSignal::new(None::<String>);
    let (log, on_change, _) = log();
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <SelectButton options=options() value=value on_change=on_change /> });
    leptos::task::tick().await;
    press(&button(&host, 0), "Enter");
    leptos::task::tick().await;
    assert_eq!(pressed(&host), "100");
    press(&button(&host, 2), "Space");
    leptos::task::tick().await;
    assert_eq!(pressed(&host), "001");
    assert_eq!(log.lock().unwrap().len(), 2);
    host.remove();
}

#[wasm_bindgen_test]
async fn external_signals_update_dom_and_options_change() {
    let value = RwSignal::new(None::<String>);
    let opts = RwSignal::new(options());
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <SelectButton options=opts value=value /> });
    leptos::task::tick().await;
    assert_eq!(pressed(&host), "000");
    value.set(Some("multi".to_owned()));
    leptos::task::tick().await;
    assert_eq!(pressed(&host), "001");
    opts.update(|o| o.insert(0, SelectButtonOption::new("New", "new")));
    leptos::task::tick().await;
    assert_eq!(pressed(&host), "0001");
    assert_eq!(button(&host, 0).text_content().unwrap(), "New");
    host.remove();
}

#[wasm_bindgen_test]
async fn selecting_keeps_button_elements_and_focus() {
    let value = RwSignal::new(None::<String>);
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <SelectButton options=options() value=value /> });
    leptos::task::tick().await;
    let first = button(&host, 0);
    first.focus().unwrap();
    press(&first, "Space");
    leptos::task::tick().await;
    assert!(first.is_same_node(Some(&button(&host, 0))), "кнопки пересозданы после выбора");
    assert!(first.is_same_node(doc().active_element().as_deref()), "фокус потерян после выбора");
    host.remove();
}
