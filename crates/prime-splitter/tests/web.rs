//! Поведение Splitter в реальном браузере (wasm-bindgen-test): перетаскивание, минимальные размеры, клавиши, события,
//! снятие подписок и хранилище.
#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use prime_splitter::{Splitter, SplitterLayout, SplitterResizeEvent, SplitterStorage};
use std::sync::{Arc, Mutex};
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use web_sys::{HtmlElement, KeyboardEvent, KeyboardEventInit, MouseEvent, MouseEventInit};

wasm_bindgen_test_configure!(run_in_browser);

fn doc() -> web_sys::Document {
    web_sys::window().unwrap().document().unwrap()
}

/// Контейнер 400 × 400 px: размер корня известен, расчёты точные.
fn host() -> HtmlElement {
    let el = doc().create_element("div").unwrap().dyn_into::<HtmlElement>().unwrap();
    el.style().set_property("width", "400px").unwrap();
    el.style().set_property("height", "400px").unwrap();
    doc().body().unwrap().append_child(&el).unwrap();
    el
}

fn panels(n: usize) -> Vec<ViewFn> {
    (1..=n).map(|i| ViewFn::from(move || view! { <p>{format!("Panel {i}")}</p> })).collect()
}

fn q(host: &HtmlElement, selector: &str) -> HtmlElement {
    host.query_selector(selector).unwrap().unwrap_or_else(|| panic!("нет {selector}")).dyn_into().unwrap()
}

fn basis(host: &HtmlElement, i: u32) -> String {
    let list = host.query_selector_all(".p-splitterpanel").unwrap();
    list.item(i).unwrap().dyn_into::<HtmlElement>().unwrap().style().get_property_value("flex-basis").unwrap()
}

fn mouse(target: &web_sys::EventTarget, kind: &str, x: i32, y: i32) {
    let init = MouseEventInit::new();
    init.set_bubbles(true);
    init.set_cancelable(true);
    init.set_client_x(x);
    init.set_client_y(y);
    target.dispatch_event(&MouseEvent::new_with_mouse_event_init_dict(kind, &init).unwrap()).unwrap();
}

fn window() -> web_sys::Window {
    web_sys::window().unwrap()
}

fn key(target: &HtmlElement, kind: &str, code: &str) -> bool {
    let init = KeyboardEventInit::new();
    init.set_bubbles(true);
    init.set_cancelable(true);
    init.set_key(code);
    target.dispatch_event(&KeyboardEvent::new_with_keyboard_event_init_dict(kind, &init).unwrap()).unwrap()
}

#[wasm_bindgen_test]
async fn drag_moves_boundary_and_toggles_resizing_state() {
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), || view! { <Splitter panels=panels(2) /> });
    leptos::task::tick().await;
    assert_eq!((basis(&host, 0).as_str(), basis(&host, 1).as_str()), ("calc(50% - 4px)", "calc(50% - 4px)"));
    mouse(&q(&host, ".p-splitter-gutter"), "mousedown", 200, 10);
    leptos::task::tick().await;
    assert!(q(&host, ".p-splitter").class_name().contains("p-splitter-resizing"));
    let gutter = q(&host, ".p-splitter-gutter");
    assert!(gutter.class_name().contains("p-splitter-gutter-resizing"));
    assert_eq!(gutter.get_attribute("data-p-gutter-resizing").as_deref(), Some("true"));
    mouse(&window(), "mousemove", 240, 10);
    leptos::task::tick().await;
    assert_eq!((basis(&host, 0).as_str(), basis(&host, 1).as_str()), ("calc(60% - 4px)", "calc(40% - 4px)"));
    assert_eq!(q(&host, ".p-splitter-gutter-handle").get_attribute("aria-valuenow").as_deref(), Some("60.0000"));
    mouse(&window(), "mouseup", 240, 10);
    leptos::task::tick().await;
    assert!(!q(&host, ".p-splitter").class_name().contains("p-splitter-resizing"));
    assert_eq!(q(&host, ".p-splitter-gutter").get_attribute("data-p-gutter-resizing").as_deref(), Some("false"));
}

#[wasm_bindgen_test]
async fn listeners_are_removed_after_mouseup() {
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), || view! { <Splitter panels=panels(2) /> });
    leptos::task::tick().await;
    mouse(&q(&host, ".p-splitter-gutter"), "mousedown", 200, 10);
    mouse(&window(), "mousemove", 240, 10);
    mouse(&window(), "mouseup", 240, 10);
    mouse(&window(), "mousemove", 300, 10);
    leptos::task::tick().await;
    assert_eq!(basis(&host, 0), "calc(60% - 4px)", "после отпускания движение игнорируется");
}

#[wasm_bindgen_test]
async fn drag_left_and_events_carry_sizes() {
    let host = host();
    let log = Arc::new(Mutex::new(Vec::<(&'static str, Vec<f64>)>::new()));
    let (start, end) = (log.clone(), log.clone());
    let _h = leptos::mount::mount_to(host.clone(), move || {
        let (start, end) = (start.clone(), end.clone());
        view! {
            <Splitter
                panels=panels(2)
                on_resize_start=Callback::new(move |e: SplitterResizeEvent| start.lock().unwrap().push(("start", e.sizes)))
                on_resize_end=Callback::new(move |e: SplitterResizeEvent| end.lock().unwrap().push(("end", e.sizes)))
            />
        }
    });
    leptos::task::tick().await;
    mouse(&q(&host, ".p-splitter-gutter"), "mousedown", 200, 10);
    mouse(&window(), "mousemove", 160, 10);
    mouse(&window(), "mouseup", 160, 10);
    leptos::task::tick().await;
    assert_eq!(*log.lock().unwrap(), [("start", vec![50.0, 50.0]), ("end", vec![40.0, 60.0])]);
}

#[wasm_bindgen_test]
async fn min_sizes_reject_the_whole_change() {
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), || view! { <Splitter panels=panels(2) min_sizes=vec![30.0, 0.0] /> });
    leptos::task::tick().await;
    mouse(&q(&host, ".p-splitter-gutter"), "mousedown", 200, 10);
    mouse(&window(), "mousemove", 100, 10); // первая панель стала бы 25 %
    leptos::task::tick().await;
    assert_eq!(basis(&host, 0), "calc(50% - 4px)");
    mouse(&window(), "mousemove", 120, 10); // 30 % — на границе допустимо
    leptos::task::tick().await;
    assert_eq!(basis(&host, 0), "calc(30% - 4px)");
    mouse(&window(), "mouseup", 120, 10);
}

#[wasm_bindgen_test]
async fn vertical_drag_uses_height() {
    let host = host();
    let _h = leptos::mount::mount_to(
        host.clone(),
        || view! { <Splitter panels=panels(2) layout=SplitterLayout::Vertical style="height: 400px" /> },
    );
    leptos::task::tick().await;
    mouse(&q(&host, ".p-splitter-gutter"), "mousedown", 10, 200);
    mouse(&window(), "mousemove", 10, 240);
    mouse(&window(), "mouseup", 10, 240);
    leptos::task::tick().await;
    assert_eq!((basis(&host, 0).as_str(), basis(&host, 1).as_str()), ("calc(60% - 4px)", "calc(40% - 4px)"));
}

#[wasm_bindgen_test]
async fn three_panels_move_only_the_neighbours() {
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), || view! { <Splitter panels=panels(3) panel_sizes=vec![25.0, 25.0, 50.0] /> });
    leptos::task::tick().await;
    let gutters = host.query_selector_all(".p-splitter-gutter").unwrap();
    assert_eq!(gutters.length(), 2);
    let second: HtmlElement = gutters.item(1).unwrap().dyn_into().unwrap();
    mouse(&second, "mousedown", 200, 10);
    mouse(&window(), "mousemove", 240, 10);
    mouse(&window(), "mouseup", 240, 10);
    leptos::task::tick().await;
    assert_eq!(
        (basis(&host, 0).as_str(), basis(&host, 1).as_str(), basis(&host, 2).as_str()),
        ("calc(25% - 8px)", "calc(35% - 8px)", "calc(40% - 8px)")
    );
}

#[wasm_bindgen_test]
async fn arrow_keys_step_in_pixels_and_keyup_finishes() {
    let host = host();
    let log = Arc::new(Mutex::new(Vec::<&'static str>::new()));
    let (start, end) = (log.clone(), log.clone());
    let _h = leptos::mount::mount_to(host.clone(), move || {
        let (start, end) = (start.clone(), end.clone());
        view! {
            <Splitter
                panels=panels(2)
                on_resize_start=Callback::new(move |_| start.lock().unwrap().push("start"))
                on_resize_end=Callback::new(move |_| end.lock().unwrap().push("end"))
            />
        }
    });
    leptos::task::tick().await;
    let handle = q(&host, ".p-splitter-gutter-handle");
    assert!(!key(&handle, "keydown", "ArrowRight"), "стрелка отменяет действие по умолчанию");
    leptos::task::tick().await;
    assert_eq!((basis(&host, 0).as_str(), basis(&host, 1).as_str()), ("calc(51.25% - 4px)", "calc(48.75% - 4px)"));
    key(&handle, "keydown", "ArrowRight");
    key(&handle, "keydown", "ArrowLeft");
    key(&handle, "keydown", "ArrowLeft");
    leptos::task::tick().await;
    assert_eq!(basis(&host, 0), "calc(50% - 4px)", "+5 +5 −5 −5 пикселей — снова поровну");
    assert_eq!(*log.lock().unwrap(), ["start"], "начало один раз за серию");
    key(&handle, "keyup", "ArrowLeft");
    assert_eq!(*log.lock().unwrap(), ["start", "end"]);
}

#[wasm_bindgen_test]
async fn keys_of_the_other_axis_do_nothing_but_are_cancelled() {
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), || view! { <Splitter panels=panels(2) /> });
    leptos::task::tick().await;
    let handle = q(&host, ".p-splitter-gutter-handle");
    assert!(!key(&handle, "keydown", "ArrowUp"), "действие по умолчанию стрелки отменяется всегда");
    assert!(key(&handle, "keydown", "Enter"));
    leptos::task::tick().await;
    assert_eq!(basis(&host, 0), "calc(50% - 4px)");
}

#[wasm_bindgen_test]
async fn vertical_keys_use_up_and_down() {
    let host = host();
    let _h = leptos::mount::mount_to(
        host.clone(),
        || view! { <Splitter panels=panels(2) layout=SplitterLayout::Vertical style="height: 400px" /> },
    );
    leptos::task::tick().await;
    let handle = q(&host, ".p-splitter-gutter-handle");
    key(&handle, "keydown", "ArrowUp");
    key(&handle, "keydown", "ArrowRight");
    leptos::task::tick().await;
    assert_eq!(basis(&host, 0), "calc(51.25% - 4px)");
    key(&handle, "keyup", "ArrowUp");
}

#[wasm_bindgen_test]
async fn state_is_saved_and_restored() {
    let storage = window().session_storage().unwrap().unwrap();
    storage.remove_item("splitter-test-save").unwrap();
    let host_one = host();
    let h1 = leptos::mount::mount_to(host_one.clone(), || view! { <Splitter panels=panels(2) state_key="splitter-test-save" /> });
    leptos::task::tick().await;
    mouse(&q(&host_one, ".p-splitter-gutter"), "mousedown", 200, 10);
    mouse(&window(), "mousemove", 240, 10);
    mouse(&window(), "mouseup", 240, 10);
    leptos::task::tick().await;
    assert_eq!(storage.get_item("splitter-test-save").unwrap().as_deref(), Some("[60,40]"));
    drop(h1);
    let host_two = host();
    let _h2 = leptos::mount::mount_to(host_two.clone(), || view! { <Splitter panels=panels(2) state_key="splitter-test-save" /> });
    leptos::task::tick().await;
    leptos::task::tick().await;
    assert_eq!((basis(&host_two, 0).as_str(), basis(&host_two, 1).as_str()), ("calc(60% - 4px)", "calc(40% - 4px)"));
    storage.remove_item("splitter-test-save").unwrap();
}

#[wasm_bindgen_test]
async fn local_storage_and_invalid_state() {
    let local = window().local_storage().unwrap().unwrap();
    local.set_item("splitter-test-local", "[20,80]").unwrap();
    let session = window().session_storage().unwrap().unwrap();
    session.set_item("splitter-test-bad", "not json").unwrap();
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), || {
        view! {
            <Splitter panels=panels(2) state_key="splitter-test-local" state_storage=SplitterStorage::Local />
            <Splitter panels=panels(2) state_key="splitter-test-bad" />
        }
    });
    leptos::task::tick().await;
    leptos::task::tick().await;
    let roots = host.query_selector_all(".p-splitter").unwrap();
    let first: HtmlElement = roots.item(0).unwrap().dyn_into().unwrap();
    let second: HtmlElement = roots.item(1).unwrap().dyn_into().unwrap();
    assert_eq!(basis(&first, 0), "calc(20% - 4px)");
    assert_eq!(basis(&second, 0), "calc(50% - 4px)", "некорректное состояние игнорируется");
    local.remove_item("splitter-test-local").unwrap();
    session.remove_item("splitter-test-bad").unwrap();
}

#[wasm_bindgen_test]
async fn unmount_during_drag_removes_listeners() {
    let host = host();
    let handle = leptos::mount::mount_to(host.clone(), || view! { <Splitter panels=panels(2) /> });
    leptos::task::tick().await;
    mouse(&q(&host, ".p-splitter-gutter"), "mousedown", 200, 10);
    drop(handle);
    leptos::task::tick().await;
    // слушатели сняты: движение и отпускание не вызывают паники на уничтоженных сигналах
    mouse(&window(), "mousemove", 240, 10);
    mouse(&window(), "mouseup", 240, 10);
    leptos::task::tick().await;
}
