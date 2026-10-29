//! Поведение InputNumber в реальном браузере (wasm-bindgen-test): набор через `beforeinput` и курсор, Backspace/Delete, вставка,
//! фиксация, границы, стрелки, кнопки и повтор, `readonly`/`disabled`, очистка, внешний сигнал и смена локали.
#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use prime_inputnumber::{ButtonLayout, InputNumber, InputNumberInput, InputNumberMode};
use std::sync::{Arc, Mutex};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;
use web_sys::{
    Event, FocusEvent, HtmlElement, HtmlInputElement, InputEvent, InputEventInit, KeyboardEvent, KeyboardEventInit, MouseEvent,
    MouseEventInit,
};

wasm_bindgen_test_configure!(run_in_browser);

fn doc() -> web_sys::Document {
    web_sys::window().unwrap().document().unwrap()
}

fn host() -> HtmlElement {
    let el = doc().create_element("div").unwrap().dyn_into::<HtmlElement>().unwrap();
    doc().body().unwrap().append_child(&el).unwrap();
    el
}

fn input(host: &HtmlElement) -> HtmlInputElement {
    host.query_selector("input").unwrap().expect("нет input").dyn_into().unwrap()
}

fn q(host: &HtmlElement, sel: &str) -> HtmlElement {
    host.query_selector(sel).unwrap().unwrap_or_else(|| panic!("нет {sel}")).dyn_into().unwrap()
}

/// `beforeinput`; возвращает `false`, если действие по умолчанию отменено.
fn before_input(el: &HtmlInputElement, kind: &str, data: Option<&str>) -> bool {
    let init = InputEventInit::new();
    init.set_bubbles(true);
    init.set_cancelable(true);
    init.set_input_type(kind);
    if let Some(d) = data {
        init.set_data(Some(d));
    }
    el.dispatch_event(&InputEvent::new_with_event_init_dict("beforeinput", &init).unwrap()).unwrap()
}

/// Набор символов подряд на месте курсора.
fn type_text(el: &HtmlInputElement, text: &str) {
    for c in text.chars() {
        before_input(el, "insertText", Some(&c.to_string()));
    }
}

fn caret(el: &HtmlInputElement) -> (u32, u32) {
    (el.selection_start().unwrap().unwrap(), el.selection_end().unwrap().unwrap())
}

fn key(el: &HtmlElement, key: &str, code: &str) -> bool {
    let init = KeyboardEventInit::new();
    init.set_key(key);
    init.set_code(code);
    init.set_bubbles(true);
    init.set_cancelable(true);
    el.dispatch_event(&KeyboardEvent::new_with_keyboard_event_init_dict("keydown", &init).unwrap()).unwrap()
}

fn mouse(el: &impl AsRef<web_sys::EventTarget>, name: &str) {
    let init = MouseEventInit::new();
    init.set_bubbles(true);
    init.set_cancelable(true);
    el.as_ref().dispatch_event(&MouseEvent::new_with_mouse_event_init_dict(name, &init).unwrap()).unwrap();
}

fn blur(el: &HtmlElement) {
    el.dispatch_event(&FocusEvent::new("blur").unwrap()).unwrap();
}

async fn sleep(ms: i32) {
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms).unwrap();
    });
    JsFuture::from(promise).await.unwrap();
}

/// Журнал `on_input`: значение (или `-`) и текст.
fn log() -> (Arc<Mutex<Vec<String>>>, Callback<InputNumberInput>) {
    let log = Arc::new(Mutex::new(Vec::new()));
    let l = log.clone();
    (
        log,
        Callback::new(move |e: InputNumberInput| {
            l.lock().unwrap().push(format!("{}|{}", e.value.map_or("-".to_owned(), |v| v.to_string()), e.formatted_value));
        }),
    )
}

#[wasm_bindgen_test]
async fn typing_digits_formats_and_keeps_caret_and_element() {
    let value = RwSignal::new(None::<f64>);
    let (log, on_input) = log();
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <InputNumber value=value on_input=on_input show_clear=true /> });
    leptos::task::tick().await;
    let el = input(&host);
    el.focus().unwrap();
    type_text(&el, "1234");
    leptos::task::tick().await;
    assert_eq!(el.value(), "1,234");
    assert_eq!(value.get_untracked(), Some(1234.0));
    assert_eq!(caret(&el), (5, 5));
    assert_eq!(el.get_attribute("aria-valuenow").as_deref(), Some("1234"));
    assert_eq!(*log.lock().unwrap(), ["1|1", "12|12", "123|123", "1234|1,234"]);
    assert!(el.is_same_node(Some(&input(&host))), "поле пересоздано при наборе");
    assert!(el.is_same_node(doc().active_element().as_deref()), "фокус потерян при наборе");
    // значок очистки появился и не пересоздал поле
    assert!(host.query_selector(".p-inputnumber-clear-icon").unwrap().is_some());
    host.remove();
}

#[wasm_bindgen_test]
async fn foreign_characters_are_rejected_and_default_is_prevented() {
    let value = RwSignal::new(Some(5.0));
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <InputNumber value=value /> });
    leptos::task::tick().await;
    let el = input(&host);
    assert!(!before_input(&el, "insertText", Some("a")), "браузерная вставка должна быть отменена");
    leptos::task::tick().await;
    assert_eq!((el.value().as_str(), value.get_untracked()), ("5", Some(5.0)));
    host.remove();
}

#[wasm_bindgen_test]
async fn typed_fraction_survives_the_external_sync_and_respects_the_limit() {
    let value = RwSignal::new(None::<f64>);
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <InputNumber value=value max_fraction_digits=2 /> });
    leptos::task::tick().await;
    let el = input(&host);
    type_text(&el, "1.");
    leptos::task::tick().await;
    assert_eq!((el.value().as_str(), value.get_untracked()), ("1.", Some(1.0)), "«1.» затёрто синхронизацией");
    type_text(&el, "259");
    leptos::task::tick().await;
    assert_eq!((el.value().as_str(), value.get_untracked()), ("1.25", Some(1.25)));
    host.remove();
}

#[wasm_bindgen_test]
async fn backspace_and_delete_skip_separators_and_ranges() {
    let value = RwSignal::new(Some(1234.0));
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <InputNumber value=value /> });
    leptos::task::tick().await;
    let el = input(&host);
    // сразу после запятой Backspace убирает «1»
    el.set_selection_range(2, 2).unwrap();
    before_input(&el, "deleteContentBackward", None);
    leptos::task::tick().await;
    assert_eq!((el.value().as_str(), value.get_untracked()), ("234", Some(234.0)));
    // Delete в начале
    el.set_selection_range(0, 0).unwrap();
    before_input(&el, "deleteContentForward", None);
    assert_eq!(el.value(), "34");
    // удаление выделения
    el.set_selection_range(0, 2).unwrap();
    before_input(&el, "deleteContentBackward", None);
    leptos::task::tick().await;
    assert_eq!((el.value().as_str(), value.get_untracked()), ("", None));
    host.remove();
}

#[wasm_bindgen_test]
async fn paste_parses_the_number() {
    let value = RwSignal::new(None::<f64>);
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <InputNumber value=value /> });
    leptos::task::tick().await;
    let el = input(&host);
    before_input(&el, "insertFromPaste", Some("1,234.5"));
    leptos::task::tick().await;
    assert_eq!((el.value().as_str(), value.get_untracked()), ("1,234.5", Some(1234.5)));
    before_input(&el, "insertFromPaste", Some("abc"));
    assert_eq!(el.value(), "1,234.5");
    host.remove();
}

#[wasm_bindgen_test]
async fn commit_on_blur_and_enter_clamps_to_bounds() {
    let value = RwSignal::new(None::<f64>);
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <InputNumber value=value min=0.0 max=100.0 min_fraction_digits=2 /> });
    leptos::task::tick().await;
    let el = input(&host);
    type_text(&el, "250");
    assert_eq!(value.get_untracked(), Some(250.0), "до фиксации значение не ограничено");
    blur(&el);
    leptos::task::tick().await;
    assert_eq!((el.value().as_str(), value.get_untracked()), ("100.00", Some(100.0)));
    el.set_value("");
    type_text(&el, "12");
    key(&el, "Enter", "Enter");
    leptos::task::tick().await;
    assert_eq!((el.value().as_str(), value.get_untracked()), ("12.00", Some(12.0)));
    // Home и End при заданных границах
    assert!(!key(&el, "End", "End"));
    assert_eq!(value.get_untracked(), Some(100.0));
    key(&el, "Home", "Home");
    leptos::task::tick().await;
    assert_eq!((el.value().as_str(), value.get_untracked()), ("0.00", Some(0.0)), "Home работает и при min = 0");
    host.remove();
}

#[wasm_bindgen_test]
async fn arrows_step_clamp_and_prevent_default() {
    let value = RwSignal::new(Some(0.9));
    let (log, on_input) = log();
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <InputNumber value=value step=0.25 max=1.0 on_input=on_input /> });
    leptos::task::tick().await;
    let el = input(&host);
    assert!(!key(&el, "ArrowUp", "ArrowUp"));
    leptos::task::tick().await;
    assert_eq!((el.value().as_str(), value.get_untracked()), ("1", Some(1.0)));
    key(&el, "ArrowDown", "ArrowDown");
    assert_eq!((el.value().as_str(), value.get_untracked()), ("0.75", Some(0.75)));
    assert_eq!(*log.lock().unwrap(), ["1|1", "0.75|0.75"]);
    host.remove();
}

#[wasm_bindgen_test]
async fn buttons_step_once_and_repeat_while_held() {
    let value = RwSignal::new(Some(0.0));
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <InputNumber value=value show_buttons=true button_layout=ButtonLayout::Horizontal /> }
    });
    leptos::task::tick().await;
    let up = q(&host, ".p-inputnumber-increment-button");
    mouse(&up, "mousedown");
    assert_eq!(value.get_untracked(), Some(1.0), "первый шаг сразу");
    mouse(&up, "mouseup");
    sleep(700).await;
    assert_eq!(value.get_untracked(), Some(1.0), "после отпускания повтора нет");
    mouse(&up, "mousedown");
    sleep(800).await;
    mouse(&up, "mouseleave");
    let held = value.get_untracked().unwrap();
    assert!(held > 5.0, "после задержки шаг повторяется каждые 40 мс: {held}");
    sleep(300).await;
    assert_eq!(value.get_untracked(), Some(held), "уход указателя остановил повтор");
    let down = q(&host, ".p-inputnumber-decrement-button");
    mouse(&down, "mousedown");
    mouse(&down, "mouseup");
    assert_eq!(value.get_untracked(), Some(held - 1.0));
    // клавиши Enter и Space на кнопке
    key(&up, "Enter", "Enter");
    q(&host, ".p-inputnumber-increment-button").dispatch_event(&Event::new("keyup").unwrap()).unwrap();
    assert_eq!(value.get_untracked(), Some(held));
    host.remove();
}

#[wasm_bindgen_test]
async fn readonly_and_disabled_do_not_change_the_value() {
    let value = RwSignal::new(Some(5.0));
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! {
            <div id="ro"><InputNumber value=value readonly=true show_buttons=true /></div>
            <div id="dis"><InputNumber value=value disabled=true show_buttons=true /></div>
        }
    });
    leptos::task::tick().await;
    for id in ["#ro", "#dis"] {
        let el: HtmlInputElement = q(&host, &format!("{id} input")).dyn_into().unwrap();
        before_input(&el, "insertText", Some("9"));
        key(&el, "ArrowUp", "ArrowUp");
        let up = q(&host, &format!("{id} .p-inputnumber-increment-button"));
        mouse(&up, "mousedown");
        mouse(&up, "mouseup");
    }
    leptos::task::tick().await;
    assert_eq!(value.get_untracked(), Some(5.0));
    host.remove();
}

#[wasm_bindgen_test]
async fn clear_icon_resets_the_value_and_calls_on_clear() {
    let value = RwSignal::new(Some(5.0));
    let cleared = Arc::new(Mutex::new(0));
    let c = cleared.clone();
    let on_clear = Callback::new(move |()| *c.lock().unwrap() += 1);
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <InputNumber value=value show_clear=true on_clear=on_clear /> });
    leptos::task::tick().await;
    mouse(&host.query_selector(".p-inputnumber-clear-icon").unwrap().unwrap(), "click");
    leptos::task::tick().await;
    assert_eq!((input(&host).value().as_str(), value.get_untracked()), ("", None));
    assert_eq!(*cleared.lock().unwrap(), 1);
    assert!(host.query_selector(".p-inputnumber-clear-icon").unwrap().is_none(), "значок остался при пустом значении");
    host.remove();
}

#[wasm_bindgen_test]
async fn external_signal_and_locale_change_reformat_the_text() {
    let value = RwSignal::new(Some(1234.5));
    let locale = RwSignal::new(Some("en-US".to_owned()));
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <InputNumber value=value locale=locale min_fraction_digits=2 mode=InputNumberMode::Decimal /> }
    });
    leptos::task::tick().await;
    let el = input(&host);
    assert_eq!(el.value(), "1,234.50");
    value.set(Some(99.0));
    leptos::task::tick().await;
    assert_eq!(el.value(), "99.00");
    locale.set(Some("de-DE".to_owned()));
    leptos::task::tick().await;
    assert_eq!(el.value(), "99,00");
    value.set(None);
    leptos::task::tick().await;
    assert_eq!(el.value(), "");
    host.remove();
}

#[wasm_bindgen_test]
async fn focus_class_and_caret_click_clamping() {
    let value = RwSignal::new(Some(5.0));
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <InputNumber value=value mode=InputNumberMode::Currency currency="USD" /> }
    });
    leptos::task::tick().await;
    let el = input(&host);
    assert_eq!(el.value(), "$5.00");
    el.dispatch_event(&FocusEvent::new("focus").unwrap()).unwrap();
    leptos::task::tick().await;
    assert!(q(&host, ".p-inputnumber").class_name().contains("p-inputwrapper-focus"));
    el.set_selection_range(0, 0).unwrap();
    mouse(&el, "click");
    assert_eq!(caret(&el), (1, 1), "курсор должен уйти из префикса");
    el.dispatch_event(&FocusEvent::new("blur").unwrap()).unwrap();
    leptos::task::tick().await;
    assert!(!q(&host, ".p-inputnumber").class_name().contains("p-inputwrapper-focus"));
    host.remove();
}
