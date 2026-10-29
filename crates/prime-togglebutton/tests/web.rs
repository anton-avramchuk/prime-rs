//! Поведение ToggleButton в реальном браузере (wasm-bindgen-test): клик, Enter и Space, `disabled`, `allow_empty`, события.
#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use prime_togglebutton::{ToggleButton, ToggleButtonChange};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use web_sys::{FocusEvent, HtmlElement, KeyboardEvent, KeyboardEventInit};

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

/// `on_change` считает: включение — `+1`, выключение — `+10`.
fn counter() -> (Arc<AtomicUsize>, Callback<ToggleButtonChange>) {
    let n = Arc::new(AtomicUsize::new(0));
    let n2 = n.clone();
    (
        n,
        Callback::new(move |e: ToggleButtonChange| {
            n2.fetch_add(if e.checked { 1 } else { 10 }, Ordering::SeqCst);
        }),
    )
}

/// Нажатие клавиши; возвращает, не отменено ли действие браузера по умолчанию.
fn press(el: &HtmlElement, code: &str) -> bool {
    let init = KeyboardEventInit::new();
    init.set_code(code);
    init.set_bubbles(true);
    init.set_cancelable(true);
    let e = KeyboardEvent::new_with_keyboard_event_init_dict("keydown", &init).unwrap();
    el.dispatch_event(&e).unwrap()
}

#[wasm_bindgen_test]
async fn click_toggles_signal_once_per_press() {
    let checked = RwSignal::new(false);
    let (n, cb) = counter();
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <ToggleButton checked=checked on_change=cb /> });
    leptos::task::tick().await;
    let root = q::<HtmlElement>(&host, ".p-togglebutton");
    root.click();
    leptos::task::tick().await;
    assert!(checked.get_untracked());
    assert_eq!(root.get_attribute("aria-pressed").as_deref(), Some("true"));
    assert!(root.class_name().contains("p-togglebutton-checked"));
    assert_eq!(q::<HtmlElement>(&host, ".p-togglebutton-label").text_content().unwrap(), "Yes");
    assert_eq!(n.load(Ordering::SeqCst), 1);
    root.click();
    leptos::task::tick().await;
    assert!(!checked.get_untracked());
    assert_eq!(n.load(Ordering::SeqCst), 11);
    host.remove();
}

#[wasm_bindgen_test]
async fn uncontrolled_button_keeps_internal_state() {
    let (n, cb) = counter();
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <ToggleButton default_checked=true on_change=cb /> });
    leptos::task::tick().await;
    let root = q::<HtmlElement>(&host, ".p-togglebutton");
    assert_eq!(root.get_attribute("aria-pressed").as_deref(), Some("true"));
    root.click();
    leptos::task::tick().await;
    assert_eq!(root.get_attribute("aria-pressed").as_deref(), Some("false"));
    assert_eq!(q::<HtmlElement>(&host, ".p-togglebutton-label").text_content().unwrap(), "No");
    assert_eq!(n.load(Ordering::SeqCst), 10);
    host.remove();
}

#[wasm_bindgen_test]
async fn enter_and_space_toggle_and_prevent_default() {
    let checked = RwSignal::new(false);
    let (n, cb) = counter();
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <ToggleButton checked=checked on_change=cb /> });
    leptos::task::tick().await;
    let root = q::<HtmlElement>(&host, ".p-togglebutton");
    assert!(!press(&root, "Enter"), "действие по умолчанию для Enter не отменено");
    leptos::task::tick().await;
    assert!(checked.get_untracked());
    assert!(!press(&root, "Space"), "прокрутка по Space не отменена");
    leptos::task::tick().await;
    assert!(!checked.get_untracked());
    assert_eq!(n.load(Ordering::SeqCst), 11);
    // другие клавиши игнорируются и не отменяются
    assert!(press(&root, "KeyA"));
    assert!(!checked.get_untracked());
    assert_eq!(n.load(Ordering::SeqCst), 11);
    host.remove();
}

#[wasm_bindgen_test]
async fn disabled_keeps_state_and_silences_on_change() {
    let checked = RwSignal::new(false);
    let (n, cb) = counter();
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <ToggleButton checked=checked disabled=true on_change=cb /> });
    leptos::task::tick().await;
    let root = q::<HtmlElement>(&host, ".p-togglebutton");
    root.click();
    press(&root, "Enter");
    press(&root, "Space");
    leptos::task::tick().await;
    assert!(!checked.get_untracked());
    assert_eq!(n.load(Ordering::SeqCst), 0);
    host.remove();
}

#[wasm_bindgen_test]
async fn allow_empty_false_blocks_turning_off() {
    let checked = RwSignal::new(true);
    let (n, cb) = counter();
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <ToggleButton checked=checked allow_empty=false on_change=cb /> });
    leptos::task::tick().await;
    let root = q::<HtmlElement>(&host, ".p-togglebutton");
    root.click();
    press(&root, "Space");
    leptos::task::tick().await;
    assert!(checked.get_untracked());
    assert_eq!(n.load(Ordering::SeqCst), 0);
    // выключенную включить можно
    checked.set(false);
    root.click();
    leptos::task::tick().await;
    assert!(checked.get_untracked());
    assert_eq!(n.load(Ordering::SeqCst), 1);
    host.remove();
}

#[wasm_bindgen_test]
async fn allow_empty_true_or_unset_allows_turning_off() {
    let checked = RwSignal::new(true);
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! {
            <div id="t"><ToggleButton checked=checked allow_empty=true /></div>
        }
    });
    leptos::task::tick().await;
    q::<HtmlElement>(&host, "#t .p-togglebutton").click();
    leptos::task::tick().await;
    assert!(!checked.get_untracked());
    host.remove();
}

#[wasm_bindgen_test]
async fn external_signal_updates_dom() {
    let checked = RwSignal::new(false);
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <ToggleButton checked=checked on_label="On" off_label="Off" /> });
    leptos::task::tick().await;
    let root = q::<HtmlElement>(&host, ".p-togglebutton");
    assert_eq!(q::<HtmlElement>(&host, ".p-togglebutton-label").text_content().unwrap(), "Off");
    checked.set(true);
    leptos::task::tick().await;
    assert_eq!(q::<HtmlElement>(&host, ".p-togglebutton-label").text_content().unwrap(), "On");
    assert_eq!(root.get_attribute("data-p-checked").as_deref(), Some("true"));
    assert_eq!(root.get_attribute("data-p").as_deref(), Some("checked"));
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
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <ToggleButton on_focus=focus on_blur=blur /> });
    leptos::task::tick().await;
    let root = q::<HtmlElement>(&host, ".p-togglebutton");
    root.focus().unwrap();
    assert_eq!(n.load(Ordering::SeqCst), 1);
    root.blur().unwrap();
    assert_eq!(n.load(Ordering::SeqCst), 11);
    host.remove();
}
