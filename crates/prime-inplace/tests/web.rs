//! Поведение Inplace в реальном браузере (wasm-bindgen-test): клик, `Enter`, `prevent_click`, `disabled`, закрытие,
//! внешний сигнал и колбэки.
#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use prime_inplace::Inplace;
use std::sync::{Arc, Mutex};
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use web_sys::{HtmlElement, KeyboardEvent, KeyboardEventInit};

wasm_bindgen_test_configure!(run_in_browser);

fn doc() -> web_sys::Document {
    web_sys::window().unwrap().document().unwrap()
}

fn host() -> HtmlElement {
    let el = doc().create_element("div").unwrap().dyn_into::<HtmlElement>().unwrap();
    doc().body().unwrap().append_child(&el).unwrap();
    el
}

fn q(host: &HtmlElement, selector: &str) -> HtmlElement {
    host.query_selector(selector).unwrap().unwrap_or_else(|| panic!("нет {selector}")).dyn_into().unwrap()
}

fn has(host: &HtmlElement, selector: &str) -> bool {
    host.query_selector(selector).unwrap().is_some()
}

/// Нажатие клавиши; `true`, если действие не отменено.
fn press(el: &HtmlElement, key: &str) -> bool {
    let init = KeyboardEventInit::new();
    init.set_bubbles(true);
    init.set_cancelable(true);
    init.set_key(key);
    el.dispatch_event(&KeyboardEvent::new_with_keyboard_event_init_dict("keydown", &init).unwrap()).unwrap()
}

fn display() -> ViewFn {
    ViewFn::from(|| view! { <span>"View"</span> })
}

fn content() -> Callback<Callback<()>, AnyView> {
    Callback::new(|close: Callback<()>| view! { <button class="inner-close" on:click=move |_| close.run(())>"done"</button> }.into_any())
}

#[wasm_bindgen_test]
async fn click_activates_and_close_callback_returns() {
    let host = host();
    let active = RwSignal::new(false);
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Inplace active=active display=display() content=content() /> });
    leptos::task::tick().await;
    assert!(has(&host, ".p-inplace-display") && !has(&host, ".p-inplace-content"));
    q(&host, ".p-inplace-display").click();
    leptos::task::tick().await;
    assert!(has(&host, ".p-inplace-content") && !has(&host, ".p-inplace-display"));
    assert!(active.get_untracked());
    q(&host, ".inner-close").click();
    leptos::task::tick().await;
    assert!(has(&host, ".p-inplace-display") && !has(&host, ".p-inplace-content"));
    assert!(!active.get_untracked());
}

#[wasm_bindgen_test]
async fn enter_activates_and_prevents_default() {
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), || view! { <Inplace display=display() content=content() /> });
    leptos::task::tick().await;
    assert!(!press(&q(&host, ".p-inplace-display"), "Enter"), "Enter должен отменять действие по умолчанию");
    leptos::task::tick().await;
    assert!(has(&host, ".p-inplace-content"));
}

#[wasm_bindgen_test]
async fn other_keys_do_nothing() {
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), || view! { <Inplace display=display() content=content() /> });
    leptos::task::tick().await;
    assert!(press(&q(&host, ".p-inplace-display"), " "));
    leptos::task::tick().await;
    assert!(!has(&host, ".p-inplace-content"));
}

#[wasm_bindgen_test]
async fn prevent_click_blocks_click_and_close_but_not_enter() {
    let host = host();
    let _h = leptos::mount::mount_to(
        host.clone(),
        || view! { <Inplace prevent_click=true closable=true display=display() content=content() /> },
    );
    leptos::task::tick().await;
    q(&host, ".p-inplace-display").click();
    leptos::task::tick().await;
    assert!(!has(&host, ".p-inplace-content"));
    press(&q(&host, ".p-inplace-display"), "Enter");
    leptos::task::tick().await;
    assert!(has(&host, ".p-inplace-content"));
    q(&host, ".p-button").click();
    q(&host, ".inner-close").click();
    leptos::task::tick().await;
    assert!(has(&host, ".p-inplace-content"), "закрытие при prevent_click не работает");
}

#[wasm_bindgen_test]
async fn disabled_blocks_everything() {
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), || view! { <Inplace disabled=true display=display() content=content() /> });
    leptos::task::tick().await;
    q(&host, ".p-inplace-display").click();
    press(&q(&host, ".p-inplace-display"), "Enter");
    leptos::task::tick().await;
    assert!(!has(&host, ".p-inplace-content"));
}

#[wasm_bindgen_test]
async fn close_button_returns_to_display() {
    let host = host();
    let active = RwSignal::new(true);
    let _h = leptos::mount::mount_to(
        host.clone(),
        move || view! { <Inplace active=active closable=true display=display() content=content() /> },
    );
    leptos::task::tick().await;
    q(&host, ".p-inplace-content .p-button").click();
    leptos::task::tick().await;
    assert!(has(&host, ".p-inplace-display"));
    assert!(!active.get_untracked());
}

#[wasm_bindgen_test]
async fn external_signal_switches_state() {
    let host = host();
    let active = RwSignal::new(false);
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Inplace active=active display=display() content=content() /> });
    leptos::task::tick().await;
    active.set(true);
    leptos::task::tick().await;
    assert!(has(&host, ".p-inplace-content"));
    active.set(false);
    leptos::task::tick().await;
    assert!(has(&host, ".p-inplace-display"));
}

#[wasm_bindgen_test]
async fn callbacks_fire_once_per_change() {
    let host = host();
    let log = Arc::new(Mutex::new(Vec::<&'static str>::new()));
    let (a, d) = (log.clone(), log.clone());
    let _h = leptos::mount::mount_to(host.clone(), move || {
        let (a, d) = (a.clone(), d.clone());
        view! {
            <Inplace
                display=display()
                content=content()
                on_activate=Callback::new(move |()| a.lock().unwrap().push("activate"))
                on_deactivate=Callback::new(move |()| d.lock().unwrap().push("deactivate"))
            />
        }
    });
    leptos::task::tick().await;
    q(&host, ".p-inplace-display").click();
    leptos::task::tick().await;
    q(&host, ".inner-close").click();
    leptos::task::tick().await;
    assert_eq!(*log.lock().unwrap(), ["activate", "deactivate"]);
}

#[wasm_bindgen_test]
async fn button_in_content_can_close() {
    use prime_button::Button;
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), || {
        view! {
            <Inplace
                display=display()
                content=Callback::new(|close: Callback<()>| {
                    view! { <Button button_type="button" text=true on_click=Callback::new(move |_| close.run(())) /> }.into_any()
                })
            />
        }
    });
    leptos::task::tick().await;
    q(&host, ".p-inplace-display").click();
    leptos::task::tick().await;
    q(&host, ".p-inplace-content .p-button").click();
    leptos::task::tick().await;
    assert!(has(&host, ".p-inplace-display"));
}
