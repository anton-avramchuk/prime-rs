//! Поведение Chip в реальном браузере (wasm-bindgen-test): удаление мышью и клавиатурой, ошибка изображения, `disabled`.
#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use prime_chip::Chip;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use web_sys::{Event, HtmlElement, KeyboardEvent, KeyboardEventInit};

wasm_bindgen_test_configure!(run_in_browser);

fn doc() -> web_sys::Document {
    web_sys::window().unwrap().document().unwrap()
}

fn host() -> HtmlElement {
    let el = doc().create_element("div").unwrap().dyn_into::<HtmlElement>().unwrap();
    doc().body().unwrap().append_child(&el).unwrap();
    el
}

fn counter() -> (Arc<AtomicUsize>, Callback<Event>) {
    let n = Arc::new(AtomicUsize::new(0));
    let n2 = n.clone();
    (
        n,
        Callback::new(move |_: Event| {
            n2.fetch_add(1, Ordering::SeqCst);
        }),
    )
}

fn q(host: &HtmlElement, sel: &str) -> HtmlElement {
    host.query_selector(sel).unwrap().unwrap_or_else(|| panic!("нет {sel}")).dyn_into().unwrap()
}

fn key(el: &HtmlElement, key: &str) {
    let init = KeyboardEventInit::new();
    init.set_key(key);
    let ev = KeyboardEvent::new_with_keyboard_event_init_dict("keydown", &init).unwrap();
    el.dispatch_event(&ev).unwrap();
}

fn display(el: &HtmlElement) -> String {
    el.style().get_property_value("display").unwrap()
}

#[wasm_bindgen_test]
async fn click_hides_the_chip_and_calls_on_remove_once() {
    let (n, cb) = counter();
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Chip label="x" removable=true on_remove=cb /> });
    leptos::task::tick().await;
    let chip = q(&host, ".p-chip");
    assert_ne!(display(&chip), "none");
    q(&host, ".p-chip-remove-icon").click();
    leptos::task::tick().await;
    assert_eq!(display(&chip), "none");
    assert_eq!(n.load(Ordering::SeqCst), 1);
    host.remove();
}

#[wasm_bindgen_test]
async fn enter_and_backspace_remove_other_keys_do_not() {
    for (k, removes) in [("Enter", true), ("Backspace", true), ("Delete", false), (" ", false), ("a", false)] {
        let (n, cb) = counter();
        let host = host();
        let _h = leptos::mount::mount_to(host.clone(), move || view! { <Chip label="x" removable=true on_remove=cb /> });
        leptos::task::tick().await;
        key(&q(&host, ".p-chip-remove-icon"), k);
        leptos::task::tick().await;
        assert_eq!(n.load(Ordering::SeqCst), usize::from(removes), "клавиша {k}");
        assert_eq!(display(&q(&host, ".p-chip")) == "none", removes, "клавиша {k}");
        host.remove();
    }
}

#[wasm_bindgen_test]
async fn remove_element_is_focusable_unless_disabled() {
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), || {
        view! {
            <div data-testid="a"><Chip label="a" removable=true /></div>
            <div data-testid="b"><Chip label="b" removable=true disabled=true /></div>
        }
    });
    leptos::task::tick().await;
    assert_eq!(q(&host, "[data-testid=a] .p-chip-remove-icon").get_attribute("tabindex").as_deref(), Some("0"));
    assert_eq!(q(&host, "[data-testid=b] .p-chip-remove-icon").get_attribute("tabindex").as_deref(), Some("-1"));
    q(&host, "[data-testid=a] .p-chip-remove-icon").focus().unwrap();
    assert!(doc().active_element().unwrap().is_same_node(Some(&q(&host, "[data-testid=a] .p-chip-remove-icon"))));
    host.remove();
}

#[wasm_bindgen_test]
async fn image_error_calls_on_image_error_once() {
    let (n, cb) = counter();
    let host = host();
    // несуществующий адрес: браузер сообщит об ошибке загрузки
    let _h = leptos::mount::mount_to(
        host.clone(),
        move || view! { <Chip label="x" image="http://127.0.0.1:1/missing.png" on_image_error=cb /> },
    );
    leptos::task::tick().await;
    for _ in 0..50 {
        if n.load(Ordering::SeqCst) > 0 {
            break;
        }
        gloo_timers_sleep().await;
    }
    assert_eq!(n.load(Ordering::SeqCst), 1);
    host.remove();
}

/// Ожидание ~100 мс без внешних зависимостей.
async fn gloo_timers_sleep() {
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, 100).unwrap();
    });
    let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
}

#[wasm_bindgen_test]
async fn non_removable_chip_has_no_remove_element_and_stays_visible() {
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), || view! { <Chip label="x" /> });
    leptos::task::tick().await;
    assert!(host.query_selector(".p-chip-remove-icon").unwrap().is_none());
    assert_ne!(display(&q(&host, ".p-chip")), "none");
    host.remove();
}
