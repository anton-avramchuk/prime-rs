//! Поведение Knob в реальном браузере (wasm-bindgen-test): клавиши, клик по характерным точкам и разрыву, шаг, перетаскивание с
//! выходом за SVG, снятие слушателей, пропуск `click` после перетаскивания, `readonly`/`disabled`, внешний сигнал и границы.
#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use prime_knob::Knob;
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

fn svg(host: &HtmlElement) -> web_sys::Element {
    host.query_selector("svg").unwrap().expect("нет svg")
}

fn path(host: &HtmlElement, class: &str) -> String {
    host.query_selector(&format!("path.{class}")).unwrap().unwrap().get_attribute("d").unwrap()
}

fn mouse(target: &EventTarget, name: &str, x: f64, y: f64) -> bool {
    let init = MouseEventInit::new();
    init.set_bubbles(true);
    init.set_cancelable(true);
    init.set_client_x(x as i32);
    init.set_client_y(y as i32);
    target.dispatch_event(&MouseEvent::new_with_mouse_event_init_dict(name, &init).unwrap()).unwrap()
}

fn key(el: &web_sys::Element, code: &str) -> bool {
    let init = KeyboardEventInit::new();
    init.set_code(code);
    init.set_bubbles(true);
    init.set_cancelable(true);
    el.dispatch_event(&KeyboardEvent::new_with_keyboard_event_init_dict("keydown", &init).unwrap()).unwrap()
}

fn window() -> EventTarget {
    web_sys::window().unwrap().into()
}

/// Точка на расстоянии `r` пикселей от центра SVG под углом `deg` градусов (0° — вправо, 90° — вверх).
fn at(svg: &web_sys::Element, deg: f64, r: f64) -> (f64, f64) {
    let rect = svg.get_bounding_client_rect();
    let (cx, cy) = (rect.left() + rect.width() / 2.0, rect.top() + rect.height() / 2.0);
    (cx + r * deg.to_radians().cos(), cy - r * deg.to_radians().sin())
}

fn aria_now(svg: &web_sys::Element) -> String {
    svg.get_attribute("aria-valuenow").unwrap_or_default()
}

fn changes() -> (Arc<Mutex<Vec<f64>>>, Callback<f64>) {
    let log = Arc::new(Mutex::new(Vec::new()));
    let l = log.clone();
    (log, Callback::new(move |v: f64| l.lock().unwrap().push(v)))
}

#[wasm_bindgen_test]
async fn keys_change_value_prevent_default_and_clamp() {
    let value = RwSignal::new(50.0);
    let (log, on_change) = changes();
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Knob value=value on_change=on_change /> });
    leptos::task::tick().await;
    let s = svg(&host);
    assert!(!key(&s, "ArrowRight"), "действие по умолчанию не отменено");
    leptos::task::tick().await;
    assert_eq!((value.get_untracked(), aria_now(&s).as_str()), (51.0, "51"));
    key(&s, "ArrowLeft");
    key(&s, "ArrowDown");
    key(&s, "PageUp");
    assert_eq!(value.get_untracked(), 59.0);
    key(&s, "PageDown");
    key(&s, "Home");
    assert_eq!(value.get_untracked(), 0.0);
    key(&s, "ArrowDown");
    assert_eq!(value.get_untracked(), 0.0, "не ниже min");
    key(&s, "End");
    key(&s, "ArrowUp");
    assert_eq!(value.get_untracked(), 100.0, "не выше max");
    assert!(key(&s, "KeyA"), "посторонняя клавиша не должна отменяться");
    assert_eq!(log.lock().unwrap().len(), 9);
    assert_eq!(log.lock().unwrap()[0], 51.0);
    host.remove();
}

#[wasm_bindgen_test]
async fn click_maps_angle_to_value_and_ignores_the_gap() {
    let value = RwSignal::new(5.0);
    let (log, on_change) = changes();
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Knob value=value on_change=on_change /> });
    leptos::task::tick().await;
    let s = svg(&host);
    for (deg, expected) in [(90.0, 50.0), (180.0, 20.0), (0.0, 80.0), (210.0, 10.0)] {
        let (x, y) = at(&s, deg, 40.0);
        mouse(&s, "click", x, y);
        leptos::task::tick().await;
        assert_eq!(value.get_untracked(), expected, "{deg}°");
    }
    assert_eq!(log.lock().unwrap().len(), 4);
    // разрыв внизу: значение не меняется, событие не вызывается
    let (x, y) = at(&s, -90.0, 40.0);
    mouse(&s, "click", x, y);
    assert_eq!(value.get_untracked(), 10.0);
    assert_eq!(log.lock().unwrap().len(), 4);
    host.remove();
}

#[wasm_bindgen_test]
async fn step_rounds_the_clicked_value() {
    let value = RwSignal::new(0.0);
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Knob value=value step=10.0 /> });
    leptos::task::tick().await;
    let s = svg(&host);
    // угол 99° соответствует значению 47 из 0..100 (240° − 3·47)
    let (x, y) = at(&s, 99.0, 40.0);
    mouse(&s, "click", x, y);
    assert_eq!(value.get_untracked(), 50.0);
    host.remove();
}

#[wasm_bindgen_test]
async fn drag_follows_pointer_outside_svg_and_unbinds_on_mouseup() {
    let value = RwSignal::new(0.0);
    let (log, on_change) = changes();
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Knob value=value on_change=on_change /> });
    leptos::task::tick().await;
    let s = svg(&host);
    let (x0, y0) = at(&s, 210.0, 40.0);
    assert!(!mouse(&s, "mousedown", x0, y0), "mousedown не отменён");
    assert!(s.is_same_node(doc().active_element().as_deref()), "после нажатия фокус на SVG");
    // указатель далеко за пределами SVG, но на нужном угле
    let (x, y) = at(&s, 90.0, 300.0);
    mouse(&window(), "mousemove", x, y);
    assert_eq!(value.get_untracked(), 50.0);
    let (x, y) = at(&s, 0.0, 500.0);
    mouse(&window(), "mousemove", x, y);
    assert_eq!(value.get_untracked(), 80.0);
    mouse(&window(), "mouseup", x, y);
    leptos::task::tick().await;
    // слушателей больше нет
    let (x, y) = at(&s, 180.0, 300.0);
    mouse(&window(), "mousemove", x, y);
    assert_eq!(value.get_untracked(), 80.0);
    assert_eq!(log.lock().unwrap().len(), 2);
    // `click` после перетаскивания не повторяет изменение
    let (x, y) = at(&s, 0.0, 40.0);
    mouse(&s, "click", x, y);
    assert_eq!(log.lock().unwrap().len(), 2);
    // а нажатие без перемещения завершается обычным кликом
    mouse(&s, "mousedown", x, y);
    mouse(&window(), "mouseup", x, y);
    mouse(&s, "click", x, y);
    assert_eq!(log.lock().unwrap().len(), 3);
    host.remove();
}

#[wasm_bindgen_test]
async fn readonly_and_disabled_ignore_input() {
    let value = RwSignal::new(50.0);
    let (log, on_change) = changes();
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! {
            <div id="ro"><Knob value=value readonly=true on_change=on_change /></div>
            <div id="dis"><Knob value=value disabled=true on_change=on_change /></div>
        }
    });
    leptos::task::tick().await;
    for id in ["#ro svg", "#dis svg"] {
        let s = host.query_selector(id).unwrap().unwrap();
        let (x, y) = at(&s, 180.0, 40.0);
        mouse(&s, "click", x, y);
        mouse(&s, "mousedown", x, y);
        mouse(&window(), "mousemove", x, y);
        mouse(&window(), "mouseup", x, y);
        key(&s, "ArrowRight");
        key(&s, "End");
        assert_eq!(s.get_attribute("tabindex").as_deref(), Some("-1"));
    }
    assert_eq!(value.get_untracked(), 50.0);
    assert!(log.lock().unwrap().is_empty());
    host.remove();
}

#[wasm_bindgen_test]
async fn external_signal_and_reactive_bounds_update_dom() {
    let value = RwSignal::new(0.0);
    let max = RwSignal::new(Some(100.0));
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Knob value=value max=max value_template="{value}%" /> });
    leptos::task::tick().await;
    let s = svg(&host);
    let before = path(&host, "p-knob-value");
    value.set(50.0);
    leptos::task::tick().await;
    assert_eq!(aria_now(&s), "50");
    assert_eq!(path(&host, "p-knob-value"), "M 30 84.641 A 40 40 0 0 1 50 10");
    assert_ne!(before, path(&host, "p-knob-value"));
    assert_eq!(host.query_selector("text").unwrap().unwrap().text_content().unwrap().trim(), "50%");
    max.set(Some(200.0));
    leptos::task::tick().await;
    assert_eq!(s.get_attribute("aria-valuemax").as_deref(), Some("200"));
    assert_ne!(path(&host, "p-knob-value"), "M 30 84.641 A 40 40 0 0 1 50 10", "дуга пересчитана под новый max");
    host.remove();
}
