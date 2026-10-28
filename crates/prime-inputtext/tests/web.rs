//! Поведение InputText в реальном браузере (wasm-bindgen-test): двусторонняя привязка, события, `disabled`/`readonly`.
#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use prime_inputtext::InputText;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use web_sys::{Event, FocusEvent, HtmlElement, HtmlInputElement};

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

fn typed(el: &HtmlInputElement, text: &str) {
    el.set_value(text);
    el.dispatch_event(&Event::new("input").unwrap()).unwrap();
}

#[wasm_bindgen_test]
async fn input_writes_into_the_signal_and_calls_on_input() {
    let value = RwSignal::new(String::new());
    let n = Arc::new(AtomicUsize::new(0));
    let n2 = n.clone();
    let cb = Callback::new(move |_: Event| {
        n2.fetch_add(1, Ordering::SeqCst);
    });
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <InputText value=value on_input=cb /> });
    leptos::task::tick().await;
    let el = input(&host);
    assert!(!el.class_name().contains("p-filled"));
    typed(&el, "abc");
    leptos::task::tick().await;
    assert_eq!(value.get_untracked(), "abc");
    assert_eq!(n.load(Ordering::SeqCst), 1);
    assert!(el.class_name().contains("p-filled"));
    host.remove();
}

#[wasm_bindgen_test]
async fn signal_updates_the_field() {
    let value = RwSignal::new("a".to_owned());
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <InputText value=value /> });
    leptos::task::tick().await;
    let el = input(&host);
    assert_eq!(el.value(), "a");
    value.set("zz".to_owned());
    leptos::task::tick().await;
    assert_eq!(el.value(), "zz");
    value.set(String::new());
    leptos::task::tick().await;
    assert!(!el.class_name().contains("p-filled"));
    host.remove();
}

#[wasm_bindgen_test]
async fn change_focus_blur_callbacks_fire() {
    let n = Arc::new(AtomicUsize::new(0));
    let (a, b, c) = (n.clone(), n.clone(), n.clone());
    let change = Callback::new(move |_: Event| {
        a.fetch_add(1, Ordering::SeqCst);
    });
    let focus = Callback::new(move |_: FocusEvent| {
        b.fetch_add(10, Ordering::SeqCst);
    });
    let blur = Callback::new(move |_: FocusEvent| {
        c.fetch_add(100, Ordering::SeqCst);
    });
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <InputText on_change=change on_focus=focus on_blur=blur /> });
    leptos::task::tick().await;
    let el = input(&host);
    el.dispatch_event(&Event::new("change").unwrap()).unwrap();
    el.dispatch_event(&FocusEvent::new("focus").unwrap()).unwrap();
    el.dispatch_event(&FocusEvent::new("blur").unwrap()).unwrap();
    assert_eq!(n.load(Ordering::SeqCst), 111);
    host.remove();
}

#[wasm_bindgen_test]
async fn disabled_and_readonly_reach_the_element() {
    let disabled = RwSignal::new(true);
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <InputText disabled=disabled readonly=true /> });
    leptos::task::tick().await;
    let el = input(&host);
    assert!(el.disabled() && el.read_only());
    disabled.set(false);
    leptos::task::tick().await;
    assert!(!el.disabled());
    host.remove();
}
