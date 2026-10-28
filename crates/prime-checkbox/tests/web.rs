//! Поведение Checkbox в реальном браузере (wasm-bindgen-test): режимы значения, `indeterminate`, `readonly`, `disabled`, события.
#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use prime_checkbox::{Checkbox, CheckboxChange};
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

fn inputs(host: &HtmlElement) -> Vec<HtmlInputElement> {
    let list = host.get_elements_by_tag_name("input");
    (0..list.length()).map(|i| list.item(i).unwrap().dyn_into().unwrap()).collect()
}

fn root_class(host: &HtmlElement, _i: u32) -> String {
    host.query_selector(".p-checkbox").unwrap().unwrap().class_name()
}

#[wasm_bindgen_test]
async fn binary_click_toggles_the_signal_and_calls_on_change() {
    let checked = RwSignal::new(false);
    let seen = Arc::new(AtomicUsize::new(0));
    let s2 = seen.clone();
    let cb = Callback::new(move |e: CheckboxChange| {
        s2.fetch_add(if e.checked { 1 } else { 10 }, Ordering::SeqCst);
    });
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Checkbox binary=true checked=checked on_change=cb /> });
    leptos::task::tick().await;
    let input = inputs(&host).remove(0);
    input.click();
    leptos::task::tick().await;
    assert!(checked.get_untracked() && root_class(&host, 0).contains("p-checkbox-checked"));
    assert!(host.query_selector(".p-checkbox-icon").unwrap().is_some());
    input.click();
    leptos::task::tick().await;
    assert!(!checked.get_untracked() && !root_class(&host, 0).contains("p-checkbox-checked"));
    assert_eq!(seen.load(Ordering::SeqCst), 11);
    host.remove();
}

#[wasm_bindgen_test]
async fn set_mode_adds_and_removes_values() {
    let model = RwSignal::new(vec!["b".to_owned()]);
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! {
            <Checkbox value="a" model=model />
            <Checkbox value="b" model=model />
        }
    });
    leptos::task::tick().await;
    let all = inputs(&host);
    assert!(all[1].checked() && !all[0].checked());
    all[0].click();
    leptos::task::tick().await;
    assert_eq!(model.get_untracked(), vec!["b".to_owned(), "a".to_owned()]);
    all[1].click();
    leptos::task::tick().await;
    assert_eq!(model.get_untracked(), vec!["a".to_owned()]);
    host.remove();
}

#[wasm_bindgen_test]
async fn click_clears_indeterminate() {
    let ind = RwSignal::new(true);
    let checked = RwSignal::new(false);
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Checkbox binary=true checked=checked indeterminate=ind /> });
    leptos::task::tick().await;
    inputs(&host)[0].click();
    leptos::task::tick().await;
    assert!(!ind.get_untracked() && checked.get_untracked());
    host.remove();
}

#[wasm_bindgen_test]
async fn readonly_and_disabled_do_not_change_state() {
    let checked = RwSignal::new(false);
    let n = Arc::new(AtomicUsize::new(0));
    let n2 = n.clone();
    let cb = Callback::new(move |_: CheckboxChange| {
        n2.fetch_add(1, Ordering::SeqCst);
    });
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! {
            <Checkbox binary=true checked=checked readonly=true on_change=cb />
            <Checkbox binary=true checked=checked disabled=true on_change=cb />
        }
    });
    leptos::task::tick().await;
    let all = inputs(&host);
    all[0].click();
    all[1].click();
    leptos::task::tick().await;
    assert!(!checked.get_untracked() && !all[0].checked() && !all[1].checked());
    assert_eq!(n.load(Ordering::SeqCst), 0);
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
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Checkbox on_focus=focus on_blur=blur /> });
    leptos::task::tick().await;
    let input = inputs(&host).remove(0);
    input.dispatch_event(&FocusEvent::new("focus").unwrap()).unwrap();
    input.dispatch_event(&FocusEvent::new("blur").unwrap()).unwrap();
    assert_eq!(n.load(Ordering::SeqCst), 11);
    host.remove();
}
