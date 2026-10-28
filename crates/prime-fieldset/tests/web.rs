//! Поведение Fieldset в реальном браузере (wasm-bindgen-test): клик, `aria-expanded`, анимация, `tabindex`, события.
#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use prime_fieldset::{Fieldset, FieldsetAfterToggle, FieldsetBeforeToggle};
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

#[wasm_bindgen_test]
async fn click_toggles_signal_aria_and_icon() {
    let collapsed = RwSignal::new(false);
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <Fieldset legend="L" toggleable=true collapsed=collapsed>"body"</Fieldset> }
    });
    leptos::task::tick().await;
    let (button, root, region) =
        (q(&host, ".p-fieldset-toggle-button"), q(&host, ".p-fieldset"), q(&host, ".p-fieldset-content-container"));
    let minus = button.inner_html();
    assert_eq!(button.get_attribute("aria-expanded").as_deref(), Some("true"));
    button.click();
    leptos::task::tick().await;
    assert!(collapsed.get_untracked());
    assert_eq!(button.get_attribute("aria-expanded").as_deref(), Some("false"));
    assert_eq!(region.get_attribute("aria-hidden").as_deref(), Some("true"));
    assert_eq!(region.get_attribute("tabindex").as_deref(), Some("-1"));
    assert!(root.class_name().contains("p-fieldset-collapsed"));
    assert_ne!(button.inner_html(), minus, "иконка сменилась на «плюс»");
    button.click();
    leptos::task::tick().await;
    assert!(!collapsed.get_untracked());
    assert!(region.get_attribute("tabindex").is_none() && !root.class_name().contains("p-fieldset-collapsed"));
}

#[wasm_bindgen_test]
async fn external_signal_drives_the_fieldset() {
    let collapsed = RwSignal::new(true);
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <Fieldset legend="L" toggleable=true collapsed=collapsed>"body"</Fieldset> }
    });
    leptos::task::tick().await;
    assert_eq!(q(&host, ".p-fieldset-content-container").style().get_property_value("display").unwrap(), "none");
    collapsed.set(false);
    leptos::task::tick().await;
    assert_eq!(q(&host, ".p-fieldset-toggle-button").get_attribute("aria-expanded").as_deref(), Some("true"));
    assert_ne!(q(&host, ".p-fieldset-content-container").style().get_property_value("display").unwrap(), "none");
}

#[wasm_bindgen_test]
async fn animation_class_is_set_and_removed_and_events_fire() {
    let collapsed = RwSignal::new(true);
    let (before, after) = (Arc::new(AtomicUsize::new(0)), Arc::new(AtomicUsize::new(0)));
    let (b2, a2) = (before.clone(), after.clone());
    let on_before = Callback::new(move |e: FieldsetBeforeToggle| {
        b2.fetch_add(if e.collapsed { 10 } else { 1 }, Ordering::SeqCst);
    });
    let on_after = Callback::new(move |e: FieldsetAfterToggle| {
        a2.fetch_add(if e.collapsed { 10 } else { 1 }, Ordering::SeqCst);
    });
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! {
            <Fieldset legend="L" toggleable=true collapsed=collapsed on_before_toggle=on_before on_after_toggle=on_after>
                "body"
            </Fieldset>
        }
    });
    leptos::task::tick().await;
    let container = q(&host, ".p-fieldset-content-container");
    q(&host, ".p-fieldset-toggle-button").click();
    leptos::task::tick().await;
    assert!(container.class_name().contains("p-collapsible-enter-active"), "{}", container.class_name());
    sleep(700).await;
    assert!(!container.class_name().contains("p-collapsible-enter-active"));
    assert_eq!((before.load(Ordering::SeqCst), after.load(Ordering::SeqCst)), (10, 1), "до — прежнее состояние, после — новое");

    q(&host, ".p-fieldset-toggle-button").click();
    leptos::task::tick().await;
    assert!(container.class_name().contains("p-collapsible-leave-active"));
    sleep(700).await;
    assert_eq!(container.style().get_property_value("display").unwrap(), "none");
    assert_eq!((before.load(Ordering::SeqCst), after.load(Ordering::SeqCst)), (11, 11));
}

#[wasm_bindgen_test]
async fn descendants_lose_and_regain_tabindex() {
    let collapsed = RwSignal::new(false);
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! {
            <Fieldset legend="L" toggleable=true collapsed=collapsed>
                <button class="inner">"b"</button>
            </Fieldset>
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
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Fieldset legend="L" default_collapsed=true>"body"</Fieldset> });
    leptos::task::tick().await;
    assert!(host.query_selector("button").unwrap().is_none());
    assert_ne!(q(&host, ".p-fieldset-content-container").style().get_property_value("display").unwrap(), "none");
}
