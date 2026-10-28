//! Поведение Textarea в реальном браузере (wasm-bindgen-test): привязка значения, события, автоподгонка высоты.
#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use prime_textarea::Textarea;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use web_sys::{Event, HtmlElement, HtmlTextAreaElement};

wasm_bindgen_test_configure!(run_in_browser);

fn doc() -> web_sys::Document {
    web_sys::window().unwrap().document().unwrap()
}

fn host() -> HtmlElement {
    let el = doc().create_element("div").unwrap().dyn_into::<HtmlElement>().unwrap();
    doc().body().unwrap().append_child(&el).unwrap();
    el
}

fn area(host: &HtmlElement) -> HtmlTextAreaElement {
    host.query_selector("textarea").unwrap().expect("нет textarea").dyn_into().unwrap()
}

fn typed(el: &HtmlTextAreaElement, text: &str) {
    el.set_value(text);
    el.dispatch_event(&Event::new("input").unwrap()).unwrap();
}

fn style_of(el: &HtmlTextAreaElement) -> web_sys::CssStyleDeclaration {
    HtmlElement::style(el.as_ref())
}

fn height_px(el: &HtmlTextAreaElement) -> f64 {
    el.offset_height() as f64
}

#[wasm_bindgen_test]
async fn input_writes_into_the_signal_and_signal_updates_the_field() {
    let value = RwSignal::new(String::new());
    let n = Arc::new(AtomicUsize::new(0));
    let n2 = n.clone();
    let cb = Callback::new(move |_: Event| {
        n2.fetch_add(1, Ordering::SeqCst);
    });
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Textarea value=value on_input=cb /> });
    leptos::task::tick().await;
    let el = area(&host);
    assert!(!el.class_name().contains("p-filled"));
    typed(&el, "abc");
    leptos::task::tick().await;
    assert_eq!(value.get_untracked(), "abc");
    assert_eq!(n.load(Ordering::SeqCst), 1);
    assert!(el.class_name().contains("p-filled"));
    value.set("zz\nyy".to_owned());
    leptos::task::tick().await;
    assert_eq!(el.value(), "zz\nyy");
    host.remove();
}

#[wasm_bindgen_test]
async fn auto_resize_grows_with_content_and_calls_on_resize() {
    let value = RwSignal::new(String::new());
    let n = Arc::new(AtomicUsize::new(0));
    let n2 = n.clone();
    let cb = Callback::new(move |_: Option<Event>| {
        n2.fetch_add(1, Ordering::SeqCst);
    });
    let host = host();
    let _h = leptos::mount::mount_to(
        host.clone(),
        move || view! { <Textarea value=value auto_resize=true rows=1u32 on_resize=cb style="width: 200px" /> },
    );
    leptos::task::tick().await;
    let el = area(&host);
    let before = height_px(&el);
    assert!(n.load(Ordering::SeqCst) >= 1, "подгонка при монтировании");
    typed(&el, "1\n2\n3\n4\n5\n6\n7\n8\n9\n10");
    leptos::task::tick().await;
    assert!(height_px(&el) > before + 50.0, "высота выросла: {before} -> {}", height_px(&el));
    assert_eq!(style_of(&el).get_property_value("overflow").unwrap(), "hidden");
    host.remove();
}

#[wasm_bindgen_test]
async fn auto_resize_stops_at_max_height_with_scroll() {
    let value = RwSignal::new("1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n11\n12".to_owned());
    let host = host();
    let _h = leptos::mount::mount_to(
        host.clone(),
        move || view! { <Textarea value=value auto_resize=true rows=1u32 style="width: 200px; max-height: 60px" /> },
    );
    leptos::task::tick().await;
    let el = area(&host);
    assert_eq!(style_of(&el).get_property_value("height").unwrap(), "60px");
    assert_eq!(style_of(&el).get_property_value("overflow-y").unwrap(), "scroll");
    host.remove();
}

#[wasm_bindgen_test]
async fn uncontrolled_auto_resize_reacts_to_input() {
    let n = Arc::new(AtomicUsize::new(0));
    let n2 = n.clone();
    let cb = Callback::new(move |e: Option<Event>| {
        n2.fetch_add(if e.is_some() { 100 } else { 1 }, Ordering::SeqCst);
    });
    let host = host();
    let _h =
        leptos::mount::mount_to(host.clone(), move || view! { <Textarea auto_resize=true rows=1u32 on_resize=cb style="width: 200px" /> });
    leptos::task::tick().await;
    let el = area(&host);
    let before = height_px(&el);
    typed(&el, "1\n2\n3\n4\n5\n6\n7\n8");
    assert!(height_px(&el) > before + 40.0);
    assert!(n.load(Ordering::SeqCst) >= 101, "событие ввода передано в on_resize");
    host.remove();
}
