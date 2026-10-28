//! Поведение Panel в реальном браузере (wasm-bindgen-test): клик, клавиатура, `aria-expanded`, анимация, `tabindex`, события.
#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use prime_panel::{Panel, PanelAfterToggle, PanelBeforeToggle, Toggler};
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

async fn sleep(ms: i32) {
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms).unwrap();
    });
    let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
}

fn counter() -> Arc<AtomicUsize> {
    Arc::new(AtomicUsize::new(0))
}

#[wasm_bindgen_test]
async fn button_click_toggles_signal_aria_and_icon() {
    let collapsed = RwSignal::new(false);
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <Panel header="H" toggleable=true collapsed=collapsed>"body"</Panel> }
    });
    leptos::task::tick().await;
    let (button, root) = (q(&host, ".p-panel-toggle-button"), q(&host, ".p-panel"));
    let minus = button.inner_html();
    assert_eq!(button.get_attribute("aria-expanded").as_deref(), Some("true"));
    button.click();
    leptos::task::tick().await;
    assert!(collapsed.get_untracked());
    assert_eq!(button.get_attribute("aria-expanded").as_deref(), Some("false"));
    assert_eq!(q(&host, ".p-panel-content-container").get_attribute("aria-hidden").as_deref(), Some("true"));
    assert_eq!(q(&host, ".p-panel-content-container").get_attribute("tabindex").as_deref(), Some("-1"));
    assert!(root.class_name().contains("p-panel-collapsed") && !root.class_name().contains("p-panel-expanded"));
    assert_ne!(button.inner_html(), minus, "иконка сменилась на «плюс»");
    button.click();
    leptos::task::tick().await;
    assert!(!collapsed.get_untracked());
    assert_eq!(button.get_attribute("aria-expanded").as_deref(), Some("true"));
    assert!(q(&host, ".p-panel-content-container").get_attribute("tabindex").is_none());
}

#[wasm_bindgen_test]
async fn external_signal_drives_the_panel() {
    let collapsed = RwSignal::new(true);
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <Panel header="H" toggleable=true collapsed=collapsed>"body"</Panel> }
    });
    leptos::task::tick().await;
    assert_eq!(q(&host, ".p-panel-content-container").style().get_property_value("display").unwrap(), "none");
    collapsed.set(false);
    leptos::task::tick().await;
    assert_eq!(q(&host, ".p-panel-toggle-button").get_attribute("aria-expanded").as_deref(), Some("true"));
    assert_ne!(q(&host, ".p-panel-content-container").style().get_property_value("display").unwrap(), "none");
}

#[wasm_bindgen_test]
async fn animation_class_is_set_and_removed_and_after_toggle_fires() {
    let collapsed = RwSignal::new(true);
    let after = counter();
    let after2 = after.clone();
    let on_after = Callback::new(move |e: PanelAfterToggle| {
        after2.fetch_add(if e.collapsed { 10 } else { 1 }, Ordering::SeqCst);
    });
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <Panel header="H" toggleable=true collapsed=collapsed on_after_toggle=on_after>"body"</Panel> }
    });
    leptos::task::tick().await;
    let container = q(&host, ".p-panel-content-container");
    q(&host, ".p-panel-toggle-button").click();
    leptos::task::tick().await;
    assert!(container.class_name().contains("p-collapsible-enter-active"), "{}", container.class_name());
    assert_ne!(container.style().get_property_value("display").unwrap(), "none");
    sleep(700).await;
    assert!(!container.class_name().contains("p-collapsible-enter-active"));
    assert_eq!(after.load(Ordering::SeqCst), 1, "on_after_toggle с collapsed=false");

    q(&host, ".p-panel-toggle-button").click();
    leptos::task::tick().await;
    assert!(container.class_name().contains("p-collapsible-leave-active"));
    sleep(700).await;
    assert!(!container.class_name().contains("p-collapsible-leave-active"));
    assert_eq!(container.style().get_property_value("display").unwrap(), "none");
    assert_eq!(after.load(Ordering::SeqCst), 11);
}

#[wasm_bindgen_test]
async fn header_toggler_toggles_once_and_button_does_not_double() {
    let collapsed = RwSignal::new(false);
    let before = counter();
    let before2 = before.clone();
    let on_before = Callback::new(move |e: PanelBeforeToggle| {
        before2.fetch_add(if e.collapsed { 10 } else { 1 }, Ordering::SeqCst);
    });
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <Panel header="H" toggleable=true toggler=Toggler::Header collapsed=collapsed on_before_toggle=on_before>"body"</Panel> }
    });
    leptos::task::tick().await;
    q(&host, ".p-panel-header").click();
    leptos::task::tick().await;
    assert!(collapsed.get_untracked());
    q(&host, ".p-panel-toggle-button").click();
    leptos::task::tick().await;
    assert!(!collapsed.get_untracked(), "клик по кнопке переключает один раз");
    assert_eq!(before.load(Ordering::SeqCst), 11, "до переключения передаётся прежнее состояние");
}

#[wasm_bindgen_test]
async fn icon_toggler_ignores_header_click() {
    let collapsed = RwSignal::new(false);
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <Panel header="H" toggleable=true collapsed=collapsed>"body"</Panel> }
    });
    leptos::task::tick().await;
    q(&host, ".p-panel-title").click();
    leptos::task::tick().await;
    assert!(!collapsed.get_untracked());
}

#[wasm_bindgen_test]
async fn enter_and_space_on_button_toggle() {
    let collapsed = RwSignal::new(false);
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <Panel header="H" toggleable=true collapsed=collapsed>"body"</Panel> }
    });
    leptos::task::tick().await;
    // Нативная <button> превращает Enter и Space в click; проверяем, что клавиатурное нажатие даёт ровно одно переключение.
    let button = q(&host, ".p-panel-toggle-button");
    button.focus().unwrap();
    let init = web_sys::KeyboardEventInit::new();
    init.set_key("Enter");
    init.set_bubbles(true);
    let _ = button.dispatch_event(&web_sys::KeyboardEvent::new_with_keyboard_event_init_dict("keydown", &init).unwrap());
    button.click();
    leptos::task::tick().await;
    assert!(collapsed.get_untracked());
}

#[wasm_bindgen_test]
async fn descendants_lose_and_regain_tabindex() {
    let collapsed = RwSignal::new(false);
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! {
            <Panel header="H" toggleable=true collapsed=collapsed>
                <button class="inner">"b"</button>
            </Panel>
        }
    });
    leptos::task::tick().await;
    let inner = q(&host, ".inner");
    assert!(inner.get_attribute("tabindex").is_none());
    collapsed.set(true);
    leptos::task::tick().await;
    assert_eq!(inner.get_attribute("tabindex").as_deref(), Some("-1"));
    collapsed.set(false);
    leptos::task::tick().await;
    assert!(inner.get_attribute("tabindex").is_none());
}

#[wasm_bindgen_test]
async fn non_toggleable_has_no_button_and_stays_open() {
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Panel header="H" default_collapsed=true>"body"</Panel> });
    leptos::task::tick().await;
    assert!(host.query_selector(".p-panel-toggle-button").unwrap().is_none());
    assert_ne!(q(&host, ".p-panel-content-container").style().get_property_value("display").unwrap(), "none");
}
