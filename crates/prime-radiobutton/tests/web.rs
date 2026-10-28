//! Поведение RadioButton в реальном браузере (wasm-bindgen-test): выбор в группе, `disabled`, события.
#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use prime_radiobutton::{RadioButton, RadioButtonClick};
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

fn roots(host: &HtmlElement) -> Vec<HtmlElement> {
    let list = host.get_elements_by_class_name("p-radiobutton");
    (0..list.length()).map(|i| list.item(i).unwrap().dyn_into().unwrap()).collect()
}

#[wasm_bindgen_test]
async fn selecting_updates_model_group_and_calls_on_click() {
    let model = RwSignal::new(None::<String>);
    let seen = Arc::new(AtomicUsize::new(0));
    let s2 = seen.clone();
    let cb = Callback::new(move |e: RadioButtonClick| {
        s2.fetch_add(if e.value.as_deref() == Some("b") { 1 } else { 10 }, Ordering::SeqCst);
    });
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! {
            <RadioButton name="g" value="a" model=model on_click=cb />
            <RadioButton name="g" value="b" model=model on_click=cb />
        }
    });
    leptos::task::tick().await;
    let all = inputs(&host);
    all[1].click();
    leptos::task::tick().await;
    assert_eq!(model.get_untracked().as_deref(), Some("b"));
    assert!(
        roots(&host)[1].class_name().contains("p-radiobutton-checked") && !roots(&host)[0].class_name().contains("p-radiobutton-checked")
    );
    all[0].click();
    leptos::task::tick().await;
    assert_eq!(model.get_untracked().as_deref(), Some("a"));
    assert!(
        roots(&host)[0].class_name().contains("p-radiobutton-checked") && !roots(&host)[1].class_name().contains("p-radiobutton-checked")
    );
    assert_eq!(seen.load(Ordering::SeqCst), 11);
    host.remove();
}

#[wasm_bindgen_test]
async fn model_change_updates_the_checked_state() {
    let model = RwSignal::new(Some("a".to_owned()));
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <RadioButton name="g" value="a" model=model /> });
    leptos::task::tick().await;
    assert!(inputs(&host)[0].checked());
    model.set(Some("zz".to_owned()));
    leptos::task::tick().await;
    assert!(!inputs(&host)[0].checked() && !roots(&host)[0].class_name().contains("p-radiobutton-checked"));
    host.remove();
}

#[wasm_bindgen_test]
async fn disabled_does_not_select() {
    let model = RwSignal::new(None::<String>);
    let n = Arc::new(AtomicUsize::new(0));
    let n2 = n.clone();
    let cb = Callback::new(move |_: RadioButtonClick| {
        n2.fetch_add(1, Ordering::SeqCst);
    });
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <RadioButton value="a" model=model disabled=true on_click=cb /> });
    leptos::task::tick().await;
    inputs(&host)[0].click();
    leptos::task::tick().await;
    assert!(model.get_untracked().is_none() && !inputs(&host)[0].checked());
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
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <RadioButton on_focus=focus on_blur=blur /> });
    leptos::task::tick().await;
    let input = inputs(&host).remove(0);
    input.dispatch_event(&FocusEvent::new("focus").unwrap()).unwrap();
    input.dispatch_event(&FocusEvent::new("blur").unwrap()).unwrap();
    assert_eq!(n.load(Ordering::SeqCst), 11);
    host.remove();
}
