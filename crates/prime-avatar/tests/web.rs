//! Avatar в реальном браузере (wasm-bindgen-test): ошибка загрузки изображения.
#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use prime_avatar::Avatar;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use web_sys::{Event, HtmlElement};

wasm_bindgen_test_configure!(run_in_browser);

fn host() -> HtmlElement {
    let doc = web_sys::window().unwrap().document().unwrap();
    let el = doc.create_element("div").unwrap().dyn_into::<HtmlElement>().unwrap();
    doc.body().unwrap().append_child(&el).unwrap();
    el
}

async fn sleep_100ms() {
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, 100).unwrap();
    });
    let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
}

#[wasm_bindgen_test]
async fn image_error_calls_on_image_error_once() {
    let n = Arc::new(AtomicUsize::new(0));
    let n2 = n.clone();
    let cb = Callback::new(move |_: Event| {
        n2.fetch_add(1, Ordering::SeqCst);
    });
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Avatar image="http://127.0.0.1:1/missing.png" on_image_error=cb /> });
    leptos::task::tick().await;
    for _ in 0..50 {
        if n.load(Ordering::SeqCst) > 0 {
            break;
        }
        sleep_100ms().await;
    }
    assert_eq!(n.load(Ordering::SeqCst), 1);
    host.remove();
}

#[wasm_bindgen_test]
async fn label_avatar_renders_without_image_events() {
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), || view! { <Avatar label="P" /> });
    leptos::task::tick().await;
    assert!(host.query_selector(".p-avatar-label").unwrap().is_some());
    assert!(host.query_selector("img").unwrap().is_none());
    host.remove();
}
