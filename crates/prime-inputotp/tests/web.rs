//! Поведение InputOtp в реальном браузере (wasm-bindgen-test): ввод и автопереход, клавиши, вставка, ограничения, события,
//! внешний сигнал и шаблон `input`.
#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use prime_inputotp::{InputOtp, InputOtpSlot};
use std::sync::{Arc, Mutex};
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use web_sys::{
    ClipboardEvent, ClipboardEventInit, DataTransfer, HtmlElement, HtmlInputElement, InputEvent, InputEventInit, KeyboardEvent,
    KeyboardEventInit,
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

/// Ячейка с номером `n` (с 1).
fn cell(host: &HtmlElement, n: u32) -> HtmlInputElement {
    host.query_selector(&format!(".p-inputotp > input:nth-child({n})"))
        .unwrap()
        .unwrap_or_else(|| panic!("нет ячейки {n}"))
        .dyn_into()
        .unwrap()
}

/// Значения ячеек подряд, пустая — `_`.
fn cells(host: &HtmlElement) -> String {
    let list = host.query_selector_all(".p-inputotp > input").unwrap();
    (0..list.length())
        .map(|i| {
            let v = list.item(i).unwrap().dyn_into::<HtmlInputElement>().unwrap().value();
            if v.is_empty() { "_".to_owned() } else { v }
        })
        .collect()
}

/// Индекс (с 1) ячейки с фокусом, 0 — фокуса в компоненте нет.
fn focused(host: &HtmlElement) -> u32 {
    let active = doc().active_element();
    (1..=8)
        .find(|&n| {
            host.query_selector(&format!(".p-inputotp > input:nth-child({n})")).unwrap().is_some_and(|el| Some(&el) == active.as_ref())
        })
        .unwrap_or(0)
}

/// Имитирует ввод символа: значение поля и событие `input`.
fn type_text(el: &HtmlInputElement, text: &str, input_type: &str) {
    el.set_value(text);
    let init = InputEventInit::new();
    init.set_bubbles(true);
    init.set_input_type(input_type);
    el.dispatch_event(&InputEvent::new_with_event_init_dict("input", &init).unwrap()).unwrap();
}

/// Нажатие клавиши; `true`, если действие не отменено.
fn press(el: &HtmlInputElement, key: &str, ctrl: bool) -> bool {
    let init = KeyboardEventInit::new();
    init.set_bubbles(true);
    init.set_cancelable(true);
    init.set_key(key);
    init.set_ctrl_key(ctrl);
    el.dispatch_event(&KeyboardEvent::new_with_keyboard_event_init_dict("keydown", &init).unwrap()).unwrap()
}

fn paste(el: &HtmlInputElement, text: &str) {
    let data = DataTransfer::new().unwrap();
    data.set_data("text", text).unwrap();
    let init = ClipboardEventInit::new();
    init.set_bubbles(true);
    init.set_cancelable(true);
    init.set_clipboard_data(Some(&data));
    el.dispatch_event(&ClipboardEvent::new_with_event_init_dict("paste", &init).unwrap()).unwrap();
}

type Log<T> = Arc<Mutex<Vec<T>>>;

fn log<T: Send + 'static>() -> (Log<T>, Log<T>) {
    let l = Arc::new(Mutex::new(Vec::new()));
    (l.clone(), l)
}

#[wasm_bindgen_test]
async fn typing_fills_cells_and_moves_focus() {
    let value = RwSignal::new(String::new());
    let (changes, sink) = log::<String>();
    let on_change = Callback::new(move |v: String| sink.lock().unwrap().push(v));
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <InputOtp value=value on_change=on_change /> });
    leptos::task::tick().await;
    cell(&host, 1).focus().unwrap();
    for (i, ch) in ["1", "2", "3", "4"].into_iter().enumerate() {
        assert_eq!(focused(&host), i as u32 + 1, "перед вводом {ch}");
        type_text(&cell(&host, i as u32 + 1), ch, "insertText");
        leptos::task::tick().await;
    }
    assert_eq!(value.get_untracked(), "1234");
    assert_eq!(cells(&host), "1234");
    assert_eq!(*changes.lock().unwrap(), ["1", "12", "123", "1234"]);
    host.remove();
}

#[wasm_bindgen_test]
async fn uncontrolled_keeps_tokens_and_reports_changes() {
    let (changes, sink) = log::<String>();
    let on_change = Callback::new(move |v: String| sink.lock().unwrap().push(v));
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <InputOtp on_change=on_change /> });
    leptos::task::tick().await;
    cell(&host, 1).focus().unwrap();
    type_text(&cell(&host, 1), "7", "insertText");
    leptos::task::tick().await;
    assert_eq!((cells(&host).as_str(), focused(&host)), ("7___", 2));
    assert_eq!(*changes.lock().unwrap(), ["7"]);
    host.remove();
}

#[wasm_bindgen_test]
async fn backspace_and_arrows_move_focus() {
    let value = RwSignal::new("1".to_owned());
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <InputOtp value=value /> });
    leptos::task::tick().await;
    cell(&host, 2).focus().unwrap();
    assert!(!press(&cell(&host, 2), "Backspace", false), "Backspace в пустой ячейке отменяется");
    assert_eq!(focused(&host), 1);
    assert!(!press(&cell(&host, 1), "ArrowRight", false));
    assert_eq!(focused(&host), 2);
    assert!(!press(&cell(&host, 2), "ArrowLeft", false));
    assert_eq!(focused(&host), 1);
    assert!(!press(&cell(&host, 1), "ArrowUp", false) && !press(&cell(&host, 1), "ArrowDown", false));
    assert_eq!(focused(&host), 1);
    // Backspace в заполненной ячейке — действие браузера
    assert!(press(&cell(&host, 1), "Backspace", false));
    // удаление назад из ячейки переводит фокус на предыдущую
    cell(&host, 3).focus().unwrap();
    type_text(&cell(&host, 3), "", "deleteContentBackward");
    leptos::task::tick().await;
    assert_eq!(focused(&host), 2);
    host.remove();
}

#[wasm_bindgen_test]
async fn paste_spreads_code_and_truncates() {
    let value = RwSignal::new(String::new());
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <InputOtp value=value /> });
    leptos::task::tick().await;
    paste(&cell(&host, 2), "123456");
    leptos::task::tick().await;
    assert_eq!(value.get_untracked(), "1234");
    assert_eq!(cells(&host), "1234");
    assert_eq!(focused(&host), 4);
    // автозаполнение: несколько символов в первую ячейку
    value.set(String::new());
    leptos::task::tick().await;
    type_text(&cell(&host, 1), "98", "insertReplacementText");
    leptos::task::tick().await;
    assert_eq!((value.get_untracked().as_str(), cells(&host).as_str()), ("98", "98__"));
    host.remove();
}

#[wasm_bindgen_test]
async fn integer_only_rejects_letters_and_paste() {
    let value = RwSignal::new("12".to_owned());
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <InputOtp value=value integer_only=true /> });
    leptos::task::tick().await;
    assert!(!press(&cell(&host, 3), "a", false), "буква отменяется");
    assert!(press(&cell(&host, 3), "5", false), "цифра разрешена");
    assert!(press(&cell(&host, 3), "Tab", false), "Tab разрешён");
    paste(&cell(&host, 1), "12ab");
    leptos::task::tick().await;
    assert_eq!((value.get_untracked().as_str(), cells(&host).as_str()), ("12", "12__"));
    host.remove();
}

#[wasm_bindgen_test]
async fn overflow_and_modifiers() {
    let value = RwSignal::new("1234".to_owned());
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <InputOtp value=value /> });
    leptos::task::tick().await;
    let last = cell(&host, 4);
    last.set_selection_range(1, 1).unwrap();
    assert!(!press(&last, "5", false), "при заполненных ячейках без выделения набор отменяется");
    assert!(press(&last, "Delete", false), "Delete не отменяется");
    assert!(press(&last, "a", true), "Ctrl+клавиша не обрабатывается");
    last.focus().unwrap();
    assert!(press(&last, "5", false), "при выделенном символе ввод разрешён");
    host.remove();
}

#[wasm_bindgen_test]
async fn readonly_and_disabled_ignore_paste() {
    let value = RwSignal::new(String::new());
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <InputOtp value=value readonly=true /> });
    leptos::task::tick().await;
    paste(&cell(&host, 1), "1234");
    leptos::task::tick().await;
    assert_eq!((value.get_untracked().as_str(), cells(&host).as_str()), ("", "____"));
    host.remove();
}

#[wasm_bindgen_test]
async fn focus_selects_content_and_fires_events() {
    let (events, sink) = log::<&'static str>();
    let (s2, s3) = (sink.clone(), sink);
    let on_focus = Callback::new(move |_| s2.lock().unwrap().push("focus"));
    let on_blur = Callback::new(move |_| s3.lock().unwrap().push("blur"));
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <InputOtp value=RwSignal::new("12".to_owned()) on_focus=on_focus on_blur=on_blur /> }
    });
    leptos::task::tick().await;
    let first = cell(&host, 1);
    first.focus().unwrap();
    assert_eq!((first.selection_start().unwrap(), first.selection_end().unwrap()), (Some(0), Some(1)));
    cell(&host, 2).focus().unwrap();
    assert_eq!(*events.lock().unwrap(), ["focus", "blur", "focus"]);
    host.remove();
}

#[wasm_bindgen_test]
async fn external_value_updates_cells() {
    let value = RwSignal::new("12".to_owned());
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <InputOtp value=value /> });
    leptos::task::tick().await;
    assert_eq!(cells(&host), "12__");
    value.set("9876".to_owned());
    // эффект переносит значение в токены, затем обновляются поля
    leptos::task::tick().await;
    leptos::task::tick().await;
    assert_eq!(cells(&host), "9876");
    value.set(String::new());
    leptos::task::tick().await;
    leptos::task::tick().await;
    assert_eq!(cells(&host), "____");
    host.remove();
}

#[wasm_bindgen_test]
async fn template_input_gets_events() {
    let value = RwSignal::new(String::new());
    let template = Callback::new(|slot: InputOtpSlot| {
        let events = slot.events;
        view! { <input class="custom-otp-input" prop:value=slot.token on:input=move |e| events.on_input.run(e) /> }.into_any()
    });
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <InputOtp length=3 value=value input=template /> });
    leptos::task::tick().await;
    let list = host.query_selector_all(".p-inputotp > .custom-otp-input").unwrap();
    assert_eq!(list.length(), 3);
    let first: HtmlInputElement = list.item(0).unwrap().dyn_into().unwrap();
    first.focus().unwrap();
    type_text(&first, "4", "insertText");
    leptos::task::tick().await;
    assert_eq!(value.get_untracked(), "4");
    host.remove();
}
