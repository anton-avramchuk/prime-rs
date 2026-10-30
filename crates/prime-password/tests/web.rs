//! Поведение Password в реальном браузере (wasm-bindgen-test): панель по фокусу и вводу, уровни, `Escape`, потеря фокуса,
//! показ пароля, очистка, фокус при клике по значкам, z-index и выравнивание.
#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use prime_password::{Password, PasswordStrength};
use std::sync::{Arc, Mutex};
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use web_sys::{Event, HtmlElement, HtmlInputElement, KeyboardEvent, KeyboardEventInit, MouseEvent, MouseEventInit};

wasm_bindgen_test_configure!(run_in_browser);

fn doc() -> web_sys::Document {
    web_sys::window().unwrap().document().unwrap()
}

fn host() -> HtmlElement {
    let el = doc().create_element("div").unwrap().dyn_into::<HtmlElement>().unwrap();
    doc().body().unwrap().append_child(&el).unwrap();
    el
}

fn q(host: &HtmlElement, selector: &str) -> web_sys::Element {
    host.query_selector(selector).unwrap().unwrap_or_else(|| panic!("нет {selector}"))
}

fn qh(host: &HtmlElement, selector: &str) -> HtmlElement {
    q(host, selector).dyn_into().unwrap()
}

fn has(host: &HtmlElement, selector: &str) -> bool {
    host.query_selector(selector).unwrap().is_some()
}

fn input(host: &HtmlElement) -> HtmlInputElement {
    qh(host, "input.p-password-input").dyn_into().unwrap()
}

async fn sleep(ms: i32) {
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms).unwrap();
    });
    let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
}

fn type_text(el: &HtmlInputElement, text: &str) {
    el.set_value(text);
    let init = web_sys::EventInit::new();
    init.set_bubbles(true);
    el.dispatch_event(&Event::new_with_event_init_dict("input", &init).unwrap()).unwrap();
}

fn press(el: &web_sys::Element, key: &str) {
    let init = KeyboardEventInit::new();
    init.set_bubbles(true);
    init.set_key(key);
    el.dispatch_event(&KeyboardEvent::new_with_keyboard_event_init_dict("keydown", &init).unwrap()).unwrap();
}

fn mouse(el: &web_sys::Element, kind: &str) -> bool {
    let init = MouseEventInit::new();
    init.set_bubbles(true);
    init.set_cancelable(true);
    el.dispatch_event(&MouseEvent::new_with_mouse_event_init_dict(kind, &init).unwrap()).unwrap()
}

fn click(el: &web_sys::Element) {
    mouse(el, "click");
}

#[wasm_bindgen_test]
async fn panel_opens_on_focus_with_prompt_and_closes_on_blur() {
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), || view! { <Password /> });
    leptos::task::tick().await;
    assert!(!has(&host, ".p-password-overlay"));
    input(&host).focus().unwrap();
    leptos::task::tick().await;
    assert!(has(&host, ".p-password-overlay"));
    assert_eq!(q(&host, ".p-password-meter-text").text_content().unwrap(), "Enter a password");
    assert!(q(&host, ".p-password").class_name().contains("p-inputwrapper-focus"));
    input(&host).blur().unwrap();
    sleep(450).await;
    assert!(!has(&host, ".p-password-overlay"));
    assert!(!q(&host, ".p-password").class_name().contains("p-inputwrapper-focus"));
}

#[wasm_bindgen_test]
async fn levels_follow_input() {
    let host = host();
    let value = RwSignal::new(String::new());
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Password value=value /> });
    leptos::task::tick().await;
    input(&host).focus().unwrap();
    for (text, class, label, width) in [
        ("a", "p-password-meter-weak", "Weak", "33.33%"),
        ("abcDEF", "p-password-meter-medium", "Medium", "66.66%"),
        ("abcDEF12", "p-password-meter-strong", "Strong", "100%"),
    ] {
        type_text(&input(&host), text);
        leptos::task::tick().await;
        let meter = qh(&host, ".p-password-meter-label");
        assert!(meter.class_name().contains(class), "{text}: {}", meter.class_name());
        assert_eq!(meter.style().get_property_value("width").unwrap(), width);
        assert_eq!(q(&host, ".p-password-meter-text").text_content().unwrap(), label);
    }
    assert_eq!(value.get_untracked(), "abcDEF12");
    type_text(&input(&host), "");
    leptos::task::tick().await;
    assert_eq!(q(&host, ".p-password-meter-text").text_content().unwrap(), "Enter a password");
    assert!(q(&host, ".p-password-meter-label").class_name() == "p-password-meter-label");
}

#[wasm_bindgen_test]
async fn escape_closes_and_input_reopens() {
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), || view! { <Password /> });
    leptos::task::tick().await;
    input(&host).focus().unwrap();
    leptos::task::tick().await;
    press(&input(&host), "Escape");
    sleep(450).await;
    assert!(!has(&host, ".p-password-overlay"));
    type_text(&input(&host), "x");
    leptos::task::tick().await;
    assert!(has(&host, ".p-password-overlay"));
}

#[wasm_bindgen_test]
async fn feedback_false_has_no_panel() {
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), || view! { <Password feedback=false /> });
    leptos::task::tick().await;
    input(&host).focus().unwrap();
    type_text(&input(&host), "abc");
    leptos::task::tick().await;
    assert!(!has(&host, ".p-password-overlay"));
}

#[wasm_bindgen_test]
async fn toggle_mask_switches_type_and_keeps_focus() {
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), || view! { <Password toggle_mask=true /> });
    leptos::task::tick().await;
    input(&host).focus().unwrap();
    assert_eq!(input(&host).type_(), "password");
    let icon = q(&host, ".p-password-unmask-icon");
    assert!(!mouse(&icon, "mousedown"), "mousedown должен быть отменён, чтобы поле не теряло фокус");
    click(&icon);
    leptos::task::tick().await;
    assert_eq!(input(&host).type_(), "text");
    assert!(has(&host, ".p-password-mask-icon") && !has(&host, ".p-password-unmask-icon"));
    click(&q(&host, ".p-password-mask-icon"));
    leptos::task::tick().await;
    assert_eq!(input(&host).type_(), "password");
}

#[wasm_bindgen_test]
async fn clear_icon_clears_value_and_calls_back() {
    let host = host();
    let value = RwSignal::new("secret".to_owned());
    let cleared = Arc::new(Mutex::new(0));
    let seen = cleared.clone();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        let seen = seen.clone();
        view! { <Password value=value show_clear=true on_clear=Callback::new(move |()| *seen.lock().unwrap() += 1) /> }
    });
    leptos::task::tick().await;
    click(&q(&host, ".p-password-clear-icon"));
    leptos::task::tick().await;
    assert_eq!(value.get_untracked(), "");
    assert_eq!(input(&host).value(), "");
    assert!(!has(&host, ".p-password-clear-icon"));
    assert_eq!(*cleared.lock().unwrap(), 1);
}

#[wasm_bindgen_test]
async fn custom_strength_is_used() {
    let host = host();
    let strength = Callback::new(|v: String| Some(if v.len() > 2 { PasswordStrength::Strong } else { PasswordStrength::Weak }));
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Password strength=strength /> });
    leptos::task::tick().await;
    input(&host).focus().unwrap();
    type_text(&input(&host), "abc");
    leptos::task::tick().await;
    assert!(q(&host, ".p-password-meter-label").class_name().contains("p-password-meter-strong"));
}

#[wasm_bindgen_test]
async fn panel_is_aligned_under_input_with_z_index() {
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), || view! { <Password /> });
    leptos::task::tick().await;
    input(&host).focus().unwrap();
    sleep(100).await;
    let panel = qh(&host, ".p-password-overlay");
    let (root, panel_rect) = (q(&host, ".p-password").get_bounding_client_rect(), panel.get_bounding_client_rect());
    assert!(panel_rect.top() >= root.bottom() - 1.0, "панель должна быть под полем: {} < {}", panel_rect.top(), root.bottom());
    assert!(panel_rect.width() >= root.width() - 1.0);
    assert!(panel.style().get_property_value("z-index").unwrap().parse::<i32>().unwrap() > 0);
    assert_eq!(web_sys::window().unwrap().get_computed_style(&panel).unwrap().unwrap().get_property_value("position").unwrap(), "absolute");
}

#[wasm_bindgen_test]
async fn callbacks_focus_and_blur() {
    let host = host();
    let log = Arc::new(Mutex::new(Vec::<&'static str>::new()));
    let (a, b) = (log.clone(), log.clone());
    let _h = leptos::mount::mount_to(host.clone(), move || {
        let (a, b) = (a.clone(), b.clone());
        view! {
            <Password
                on_focus=Callback::new(move |_| a.lock().unwrap().push("focus"))
                on_blur=Callback::new(move |_| b.lock().unwrap().push("blur"))
            />
        }
    });
    leptos::task::tick().await;
    input(&host).focus().unwrap();
    input(&host).blur().unwrap();
    assert_eq!(*log.lock().unwrap(), ["focus", "blur"]);
}
