//! Поведение Slider в реальном браузере (wasm-bindgen-test): клавиши, клик по полосе, перетаскивание и снятие слушателей,
//! `disabled`, диапазон, внешний сигнал, реактивные границы, сохранение ручек и фокуса.
#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use prime_slider::{Slider, SliderChange, SliderOrientation, SliderSlideEnd, SliderValue};
use std::sync::{Arc, Mutex};
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use web_sys::{EventTarget, HtmlElement, KeyboardEvent, KeyboardEventInit, MouseEvent, MouseEventInit};

wasm_bindgen_test_configure!(run_in_browser);

fn doc() -> web_sys::Document {
    web_sys::window().unwrap().document().unwrap()
}

fn host() -> HtmlElement {
    let el = doc().create_element("div").unwrap().dyn_into::<HtmlElement>().unwrap();
    doc().body().unwrap().append_child(&el).unwrap();
    el
}

fn root(host: &HtmlElement) -> HtmlElement {
    host.query_selector(".p-slider").unwrap().expect("нет .p-slider").dyn_into().unwrap()
}

/// Ручка с номером `n` (с нуля).
fn handle(host: &HtmlElement, n: u32) -> HtmlElement {
    let list = host.query_selector_all(".p-slider-handle").unwrap();
    list.item(n).unwrap_or_else(|| panic!("нет ручки {n}")).dyn_into().unwrap()
}

fn now(el: &HtmlElement) -> String {
    el.get_attribute("aria-valuenow").unwrap_or_default()
}

/// Событие мыши; возвращает `false`, если действие по умолчанию отменено.
fn mouse(target: &EventTarget, name: &str, x: f64, y: f64) -> bool {
    let init = MouseEventInit::new();
    init.set_bubbles(true);
    init.set_cancelable(true);
    init.set_client_x(x as i32);
    init.set_client_y(y as i32);
    target.dispatch_event(&MouseEvent::new_with_mouse_event_init_dict(name, &init).unwrap()).unwrap()
}

fn key(el: &HtmlElement, code: &str) -> bool {
    let init = KeyboardEventInit::new();
    init.set_code(code);
    init.set_bubbles(true);
    init.set_cancelable(true);
    el.dispatch_event(&KeyboardEvent::new_with_keyboard_event_init_dict("keydown", &init).unwrap()).unwrap()
}

fn window() -> EventTarget {
    web_sys::window().unwrap().into()
}

/// Журнал событий: `change:<v>` и `end:<v>` (диапазон — `a,b`).
fn log() -> (Arc<Mutex<Vec<String>>>, Callback<SliderChange>, Callback<SliderSlideEnd>) {
    let log = Arc::new(Mutex::new(Vec::new()));
    let (a, b) = (log.clone(), log.clone());
    let fmt = |v: SliderValue| match v {
        SliderValue::Single(v) => v.to_string(),
        SliderValue::Range([x, y]) => format!("{x},{y}"),
    };
    (
        log,
        Callback::new(move |e: SliderChange| a.lock().unwrap().push(format!("change:{}", fmt(e.value)))),
        Callback::new(move |e: SliderSlideEnd| b.lock().unwrap().push(format!("end:{}", fmt(e.value)))),
    )
}

fn sized(extra: &str) -> String {
    format!("width: 200px; height: 20px; position: relative; {extra}")
}

#[wasm_bindgen_test]
async fn keys_change_value_prevent_default_and_keep_focus() {
    let value = RwSignal::new(50.0);
    let (log, on_change, _) = log();
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Slider value=value on_change=on_change /> });
    leptos::task::tick().await;
    let h = handle(&host, 0);
    assert!(!key(&h, "ArrowRight"), "действие по умолчанию не отменено");
    leptos::task::tick().await;
    assert_eq!((value.get_untracked(), now(&h).as_str()), (51.0, "51"));
    assert!(h.is_same_node(doc().active_element().as_deref()), "фокус должен остаться на ручке");
    key(&h, "ArrowLeft");
    key(&h, "ArrowLeft");
    key(&h, "PageUp");
    assert_eq!(value.get_untracked(), 59.0);
    key(&h, "PageDown");
    assert_eq!(value.get_untracked(), 49.0);
    key(&h, "Home");
    assert_eq!(value.get_untracked(), 0.0);
    key(&h, "ArrowDown");
    assert_eq!(value.get_untracked(), 0.0, "не ниже min");
    key(&h, "End");
    key(&h, "ArrowUp");
    assert_eq!(value.get_untracked(), 100.0, "не выше max");
    assert!(key(&h, "KeyA"), "посторонняя клавиша не должна отменяться");
    assert_eq!(log.lock().unwrap().len(), 9);
    assert_eq!(log.lock().unwrap()[0], "change:51");
    host.remove();
}

#[wasm_bindgen_test]
async fn step_moves_keys_by_step_and_keeps_handle_element() {
    let value = RwSignal::new(20.0);
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Slider value=value step=20.0 /> });
    leptos::task::tick().await;
    let h = handle(&host, 0);
    key(&h, "ArrowRight");
    leptos::task::tick().await;
    assert_eq!(value.get_untracked(), 40.0);
    key(&h, "PageDown");
    assert_eq!(value.get_untracked(), 20.0);
    assert!(h.is_same_node(Some(&handle(&host, 0))), "ручка пересоздана после изменения");
    host.remove();
}

#[wasm_bindgen_test]
async fn bar_click_moves_handle_and_ends_slide() {
    let value = RwSignal::new(0.0);
    let (log, on_change, on_end) = log();
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <Slider value=value style=sized("") on_change=on_change on_slide_end=on_end /> }
    });
    leptos::task::tick().await;
    let (r, rect) = (root(&host), root(&host).get_bounding_client_rect());
    mouse(&r, "click", rect.left() + 100.0, rect.top() + 5.0);
    leptos::task::tick().await;
    assert_eq!(value.get_untracked(), 50.0);
    assert_eq!(*log.lock().unwrap(), ["change:50", "end:50"]);
    assert!(handle(&host, 0).is_same_node(doc().active_element().as_deref()));
    host.remove();
}

#[wasm_bindgen_test]
async fn vertical_bar_click_counts_from_the_bottom() {
    let value = RwSignal::new(0.0);
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <Slider value=value orientation=SliderOrientation::Vertical style="width: 20px; height: 200px; position: relative" /> }
    });
    leptos::task::tick().await;
    let (r, rect) = (root(&host), root(&host).get_bounding_client_rect());
    mouse(&r, "click", rect.left() + 5.0, rect.top() + 150.0);
    leptos::task::tick().await;
    assert_eq!(value.get_untracked(), 25.0);
    host.remove();
}

#[wasm_bindgen_test]
async fn drag_updates_value_ends_once_and_unbinds_window_listeners() {
    let value = RwSignal::new(0.0);
    let (log, on_change, on_end) = log();
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <Slider value=value style=sized("") on_change=on_change on_slide_end=on_end /> }
    });
    leptos::task::tick().await;
    let (r, rect) = (root(&host), root(&host).get_bounding_client_rect());
    let h = handle(&host, 0);
    assert_eq!(r.get_attribute("data-p-sliding").as_deref(), Some("false"));
    assert!(!mouse(&h, "mousedown", rect.left(), rect.top()), "mousedown не отменён");
    leptos::task::tick().await;
    assert_eq!(r.get_attribute("data-p-sliding").as_deref(), Some("true"));
    mouse(&window(), "mousemove", rect.left() + 50.0, rect.top());
    assert_eq!(value.get_untracked(), 25.0);
    mouse(&window(), "mousemove", rect.left() + 100.0, rect.top());
    assert_eq!(value.get_untracked(), 50.0);
    mouse(&window(), "mouseup", rect.left() + 100.0, rect.top());
    leptos::task::tick().await;
    assert_eq!(r.get_attribute("data-p-sliding").as_deref(), Some("false"));
    // слушателей больше нет: перемещение после отпускания ничего не меняет
    mouse(&window(), "mousemove", rect.left() + 200.0, rect.top());
    mouse(&window(), "mouseup", rect.left() + 200.0, rect.top());
    assert_eq!(value.get_untracked(), 50.0);
    assert_eq!(*log.lock().unwrap(), ["change:25", "change:50", "end:50"]);
    // `click` после перетаскивания (как отпускание над полосой) не считается кликом по полосе
    mouse(&h, "click", rect.left() + 200.0, rect.top());
    assert_eq!(value.get_untracked(), 50.0);
    assert_eq!(log.lock().unwrap().len(), 3);
    host.remove();
}

#[wasm_bindgen_test]
async fn drag_with_step_moves_by_whole_steps() {
    let value = RwSignal::new(20.0);
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Slider value=value step=20.0 style=sized("") /> });
    leptos::task::tick().await;
    let (_, rect) = (root(&host), root(&host).get_bounding_client_rect());
    mouse(&handle(&host, 0), "mousedown", rect.left() + 40.0, rect.top());
    mouse(&window(), "mousemove", rect.left() + 140.0, rect.top());
    assert_eq!(value.get_untracked(), 60.0, "70 % → 60 (два шага от 20)");
    mouse(&window(), "mousemove", rect.left() + 0.0, rect.top());
    assert_eq!(value.get_untracked(), 0.0);
    mouse(&window(), "mouseup", rect.left(), rect.top());
    host.remove();
}

#[wasm_bindgen_test]
async fn disabled_ignores_clicks_drags_and_keys() {
    let value = RwSignal::new(50.0);
    let (log, on_change, on_end) = log();
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <Slider value=value disabled=true style=sized("") on_change=on_change on_slide_end=on_end /> }
    });
    leptos::task::tick().await;
    let (r, rect) = (root(&host), root(&host).get_bounding_client_rect());
    let h = handle(&host, 0);
    mouse(&r, "click", rect.left() + 10.0, rect.top());
    mouse(&h, "mousedown", rect.left(), rect.top());
    mouse(&window(), "mousemove", rect.left() + 100.0, rect.top());
    mouse(&window(), "mouseup", rect.left() + 100.0, rect.top());
    key(&h, "ArrowRight");
    leptos::task::tick().await;
    assert_eq!(value.get_untracked(), 50.0);
    assert!(log.lock().unwrap().is_empty());
    assert!(h.get_attribute("tabindex").is_none());
    host.remove();
}

#[wasm_bindgen_test]
async fn range_keys_keep_pair_ordered_and_focus_follows_the_value() {
    let values = RwSignal::new([20.0, 80.0]);
    let (log, on_change, _) = log();
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Slider range=true values=values on_change=on_change /> });
    leptos::task::tick().await;
    key(&handle(&host, 1), "ArrowRight");
    assert_eq!(values.get_untracked(), [20.0, 81.0]);
    // первая ручка уходит за вторую: пара остаётся упорядоченной, фокус переходит на ручку с этим значением
    key(&handle(&host, 0), "End");
    leptos::task::tick().await;
    assert_eq!(values.get_untracked(), [81.0, 100.0]);
    assert!(handle(&host, 1).is_same_node(doc().active_element().as_deref()), "фокус должен перейти на вторую ручку");
    assert!(handle(&host, 1).class_name().contains("p-slider-handle-active"));
    assert!(!handle(&host, 0).class_name().contains("p-slider-handle-active"));
    assert_eq!((now(&handle(&host, 0)).as_str(), now(&handle(&host, 1)).as_str()), ("81", "100"));
    assert_eq!(*log.lock().unwrap(), ["change:20,81", "change:81,100"]);
    host.remove();
}

#[wasm_bindgen_test]
async fn range_bar_click_moves_the_active_handle() {
    let values = RwSignal::new([20.0, 80.0]);
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Slider range=true values=values style=sized("") /> });
    leptos::task::tick().await;
    let (r, rect) = (root(&host), root(&host).get_bounding_client_rect());
    mouse(&r, "click", rect.left() + 100.0, rect.top());
    assert_eq!(values.get_untracked(), [50.0, 80.0], "двигается первая (активная по умолчанию) ручка");
    key(&handle(&host, 1), "ArrowLeft");
    mouse(&r, "click", rect.left() + 180.0, rect.top());
    assert_eq!(values.get_untracked(), [50.0, 90.0], "активной стала вторая ручка");
    host.remove();
}

#[wasm_bindgen_test]
async fn external_signal_and_reactive_bounds_update_dom() {
    let value = RwSignal::new(0.0);
    let max = RwSignal::new(Some(100.0));
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Slider value=value max=max /> });
    leptos::task::tick().await;
    let h = handle(&host, 0);
    value.set(50.0);
    leptos::task::tick().await;
    assert_eq!(now(&h), "50");
    assert!(h.get_attribute("style").unwrap().contains("inset-inline-start: 50%"));
    max.set(Some(200.0));
    leptos::task::tick().await;
    assert!(h.get_attribute("style").unwrap().contains("inset-inline-start: 25%"));
    assert_eq!(h.get_attribute("aria-valuemax").as_deref(), Some("200"));
    host.remove();
}
