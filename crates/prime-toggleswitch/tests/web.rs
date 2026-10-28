//! Поведение ToggleSwitch в реальном браузере (wasm-bindgen-test): клик по корню и по `<input>`, `readonly`, `disabled`, события.
#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use prime_toggleswitch::{ToggleSwitch, ToggleSwitchChange};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use web_sys::{FocusEvent, HtmlElement, HtmlInputElement};

wasm_bindgen_test_configure!(run_in_browser);

fn doc() -> web_sys::Document {
    web_sys::window().unwrap().document().unwrap()
}

fn host() -> HtmlElement {
    let el = doc().create_element("div").unwrap().dyn_into::<HtmlElement>().unwrap();
    doc().body().unwrap().append_child(&el).unwrap();
    el
}

fn q<T: JsCast>(host: &HtmlElement, sel: &str) -> T {
    host.query_selector(sel).unwrap().unwrap_or_else(|| panic!("нет {sel}")).dyn_into().unwrap()
}

fn counter() -> (Arc<AtomicUsize>, Callback<ToggleSwitchChange>) {
    let n = Arc::new(AtomicUsize::new(0));
    let n2 = n.clone();
    (
        n,
        Callback::new(move |e: ToggleSwitchChange| {
            n2.fetch_add(if e.checked { 1 } else { 10 }, Ordering::SeqCst);
        }),
    )
}

#[wasm_bindgen_test]
async fn click_on_slider_and_on_input_each_toggle_once() {
    let checked = RwSignal::new(false);
    let (n, cb) = counter();
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <ToggleSwitch checked=checked on_change=cb /> });
    leptos::task::tick().await;
    q::<HtmlElement>(&host, ".p-toggleswitch-slider").click();
    leptos::task::tick().await;
    assert!(checked.get_untracked() && q::<HtmlInputElement>(&host, "input").checked());
    assert!(q::<HtmlElement>(&host, ".p-toggleswitch").class_name().contains("p-toggleswitch-checked"));
    q::<HtmlInputElement>(&host, "input").click();
    leptos::task::tick().await;
    assert!(!checked.get_untracked() && !q::<HtmlInputElement>(&host, "input").checked());
    assert_eq!(n.load(Ordering::SeqCst), 11);
    host.remove();
}

#[wasm_bindgen_test]
async fn uncontrolled_switch_toggles_the_dom_state() {
    let (n, cb) = counter();
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <ToggleSwitch on_change=cb /> });
    leptos::task::tick().await;
    let input = q::<HtmlInputElement>(&host, "input");
    q::<HtmlElement>(&host, ".p-toggleswitch-handle").click();
    assert!(input.checked());
    input.click();
    assert!(!input.checked());
    q::<HtmlElement>(&host, ".p-toggleswitch-slider").click();
    assert!(input.checked());
    assert_eq!(n.load(Ordering::SeqCst), 12);
    host.remove();
}

#[wasm_bindgen_test]
async fn readonly_and_disabled_keep_state() {
    let checked = RwSignal::new(true);
    let (n, cb) = counter();
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! {
            <div id="ro"><ToggleSwitch checked=checked readonly=true on_change=cb /></div>
            <div id="dis"><ToggleSwitch checked=checked disabled=true on_change=cb /></div>
        }
    });
    leptos::task::tick().await;
    for sel in ["#ro input", "#dis input", "#ro .p-toggleswitch-slider"] {
        q::<HtmlElement>(&host, sel).click();
    }
    leptos::task::tick().await;
    assert!(checked.get_untracked());
    assert!(q::<HtmlInputElement>(&host, "#ro input").checked());
    assert_eq!(n.load(Ordering::SeqCst), 0);
    host.remove();
}

#[wasm_bindgen_test]
async fn signal_updates_classes_and_handle_template() {
    let checked = RwSignal::new(false);
    let tpl = Callback::new(|c: bool| view! { <b>{if c { "on" } else { "off" }}</b> }.into_any());
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <ToggleSwitch checked=checked handle_template=tpl /> });
    leptos::task::tick().await;
    assert_eq!(q::<HtmlElement>(&host, ".p-toggleswitch-handle").text_content().unwrap(), "off");
    checked.set(true);
    leptos::task::tick().await;
    assert_eq!(q::<HtmlElement>(&host, ".p-toggleswitch-handle").text_content().unwrap(), "on");
    assert_eq!(q::<HtmlElement>(&host, ".p-toggleswitch").get_attribute("data-p-checked").as_deref(), Some("true"));
    host.remove();
}

#[wasm_bindgen_test]
async fn focus_and_blur_callbacks_fire() {
    let n = Arc::new(AtomicUsize::new(0));
    let (a, b) = (n.clone(), n.clone());
    let focus = Callback::new(move |_: FocusEvent| {
        a.fetch_add(1, Ordering::SeqCst);
    });
    let blur = Callback::new(move |_: FocusEvent| {
        b.fetch_add(10, Ordering::SeqCst);
    });
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <ToggleSwitch on_focus=focus on_blur=blur /> });
    leptos::task::tick().await;
    let input = q::<HtmlInputElement>(&host, "input");
    input.dispatch_event(&FocusEvent::new("focus").unwrap()).unwrap();
    input.dispatch_event(&FocusEvent::new("blur").unwrap()).unwrap();
    assert_eq!(n.load(Ordering::SeqCst), 11);
    host.remove();
}
