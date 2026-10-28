//! Поведение Message в реальном браузере (wasm-bindgen-test): закрытие кнопкой и колбэком контейнера, автоскрытие по `life`.
#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use prime_message::{Message, MessageClose};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use web_sys::HtmlElement;

wasm_bindgen_test_configure!(run_in_browser);

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

fn is_hidden(el: &HtmlElement) -> bool {
    el.class_name().contains("p-message-leave-active") && el.style().get_property_value("display").unwrap() == "none"
}

#[wasm_bindgen_test]
async fn close_button_hides_the_message_and_calls_on_close_once() {
    let n = Arc::new(AtomicUsize::new(0));
    let n2 = n.clone();
    let cb = Callback::new(move |_: MessageClose| {
        n2.fetch_add(1, Ordering::SeqCst);
    });
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Message text="Bye" closable=true on_close=cb /> });
    leptos::task::tick().await;
    let root = q(&host, ".p-message");
    assert!(!is_hidden(&root));
    q(&host, ".p-message-close-button").click();
    leptos::task::tick().await;
    assert!(is_hidden(&root));
    assert_eq!(n.load(Ordering::SeqCst), 1);
    host.remove();
}

#[wasm_bindgen_test]
async fn container_template_close_callback_hides_the_message() {
    let tpl = Callback::new(|close: Callback<web_sys::Event>| {
        view! { <button id="mine" on:click=move |e| close.run(e.into())>"x"</button> }.into_any()
    });
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Message closable=true container_template=tpl /> });
    leptos::task::tick().await;
    q(&host, "#mine").click();
    leptos::task::tick().await;
    assert!(is_hidden(&q(&host, ".p-message")));
    host.remove();
}

#[wasm_bindgen_test]
async fn life_hides_the_message_after_the_timeout() {
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Message text="Soon gone" life=60u32 /> });
    leptos::task::tick().await;
    let root = q(&host, ".p-message");
    assert!(!is_hidden(&root));
    gloo_timers_sleep(150).await;
    assert!(is_hidden(&root));
    host.remove();
}

async fn gloo_timers_sleep(ms: i32) {
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms).unwrap();
    });
    wasm_bindgen_futures::JsFuture::from(promise).await.unwrap();
}
