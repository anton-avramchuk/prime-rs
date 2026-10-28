//! Поведение Accordion в реальном браузере (wasm-bindgen-test): режимы, события, клавиатура, анимация, `tabindex`.
#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use prime_accordion::{Accordion, AccordionContent, AccordionHeader, AccordionPanel, AccordionToggle};
use std::sync::{Arc, Mutex};
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

fn all(host: &HtmlElement, sel: &str) -> Vec<HtmlElement> {
    let list = host.query_selector_all(sel).unwrap();
    (0..list.length()).map(|i| list.item(i).unwrap().dyn_into().unwrap()).collect()
}

fn noop() -> Callback<AccordionToggle> {
    Callback::new(|_| {})
}

fn v(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

async fn sleep(ms: i32) {
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms).unwrap();
    });
    let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
}

fn key(target: &HtmlElement, code: &str) {
    let init = web_sys::KeyboardEventInit::new();
    init.set_code(code);
    init.set_bubbles(true);
    init.set_cancelable(true);
    let _ = target.dispatch_event(&web_sys::KeyboardEvent::new_with_keyboard_event_init_dict("keydown", &init).unwrap());
}

fn active_element() -> Option<web_sys::Element> {
    doc().active_element()
}

fn mount(
    host: &HtmlElement,
    value: RwSignal<Vec<String>>,
    multiple: bool,
    on_open: Callback<AccordionToggle>,
    on_close: Callback<AccordionToggle>,
) {
    leptos::mount::mount_to(host.clone(), move || {
        view! {
            <Accordion id="a" value=value multiple=multiple on_open=on_open on_close=on_close>
                <AccordionPanel value="0">
                    <AccordionHeader>"Header I"</AccordionHeader>
                    <AccordionContent><button class="inner">"b"</button></AccordionContent>
                </AccordionPanel>
                <AccordionPanel value="1">
                    <AccordionHeader>"Header II"</AccordionHeader>
                    <AccordionContent>"Content II"</AccordionContent>
                </AccordionPanel>
                <AccordionPanel value="2" disabled=true>
                    <AccordionHeader>"Header III"</AccordionHeader>
                    <AccordionContent>"Content III"</AccordionContent>
                </AccordionPanel>
                <AccordionPanel value="3">
                    <AccordionHeader>"Header IV"</AccordionHeader>
                    <AccordionContent>"Content IV"</AccordionContent>
                </AccordionPanel>
            </Accordion>
        }
    })
    .forget();
}

#[wasm_bindgen_test]
async fn single_mode_opens_one_and_closes_active_on_second_click() {
    let value = RwSignal::new(v(&["0"]));
    let host = host();
    mount(&host, value, false, noop(), noop());
    leptos::task::tick().await;
    let headers = all(&host, ".p-accordionheader");
    headers[1].click();
    leptos::task::tick().await;
    assert_eq!(value.get_untracked(), v(&["1"]));
    assert_eq!(headers[0].get_attribute("aria-expanded").as_deref(), Some("false"));
    assert_eq!(headers[1].get_attribute("aria-expanded").as_deref(), Some("true"));
    assert!(all(&host, ".p-accordionpanel")[1].class_name().contains("p-accordionpanel-active"));
    headers[1].click();
    leptos::task::tick().await;
    assert!(value.get_untracked().is_empty());
}

#[wasm_bindgen_test]
async fn multiple_mode_keeps_other_panels() {
    let value = RwSignal::new(v(&["0"]));
    let host = host();
    mount(&host, value, true, noop(), noop());
    leptos::task::tick().await;
    all(&host, ".p-accordionheader")[1].click();
    leptos::task::tick().await;
    assert_eq!(value.get_untracked(), v(&["0", "1"]));
    all(&host, ".p-accordionheader")[0].click();
    leptos::task::tick().await;
    assert_eq!(value.get_untracked(), v(&["1"]));
}

#[wasm_bindgen_test]
async fn disabled_panel_ignores_clicks_and_keys() {
    let value = RwSignal::new(Vec::new());
    let host = host();
    mount(&host, value, false, noop(), noop());
    leptos::task::tick().await;
    let header = all(&host, ".p-accordionheader")[2].clone();
    header.click();
    key(&header, "Enter");
    leptos::task::tick().await;
    assert!(value.get_untracked().is_empty());
}

#[wasm_bindgen_test]
async fn events_report_open_and_close_with_the_panel_value() {
    let log = Arc::new(Mutex::new(Vec::<String>::new()));
    let (l1, l2) = (log.clone(), log.clone());
    let on_open = Callback::new(move |e: AccordionToggle| l1.lock().unwrap().push(format!("open {}", e.index)));
    let on_close = Callback::new(move |e: AccordionToggle| l2.lock().unwrap().push(format!("close {}", e.index)));
    let value = RwSignal::new(v(&["0"]));
    let host = host();
    mount(&host, value, false, on_open, on_close);
    leptos::task::tick().await;
    let headers = all(&host, ".p-accordionheader");
    headers[1].click();
    leptos::task::tick().await;
    headers[1].click();
    leptos::task::tick().await;
    assert_eq!(*log.lock().unwrap(), ["open 1", "close 1"], "закрытие панели 0 при открытии панели 1 событием не сопровождается");
}

#[wasm_bindgen_test]
async fn keyboard_toggles_and_navigates_skipping_disabled_and_wrapping() {
    let value = RwSignal::new(Vec::new());
    let host = host();
    mount(&host, value, false, noop(), noop());
    leptos::task::tick().await;
    let headers = all(&host, ".p-accordionheader");
    headers[0].focus().unwrap();
    key(&headers[0], "Enter");
    leptos::task::tick().await;
    assert_eq!(value.get_untracked(), v(&["0"]), "Enter переключает один раз");
    key(&headers[0], "Space");
    leptos::task::tick().await;
    assert!(value.get_untracked().is_empty());

    key(&headers[0], "ArrowDown");
    assert_eq!(active_element().as_ref(), Some(headers[1].as_ref()));
    key(&headers[1], "ArrowDown");
    assert_eq!(active_element().as_ref(), Some(headers[3].as_ref()), "отключённая панель пропущена");
    key(&headers[3], "ArrowDown");
    assert_eq!(active_element().as_ref(), Some(headers[0].as_ref()), "по кругу к первой");
    key(&headers[0], "ArrowUp");
    assert_eq!(active_element().as_ref(), Some(headers[3].as_ref()), "по кругу к последней");
    key(&headers[3], "Home");
    assert_eq!(active_element().as_ref(), Some(headers[0].as_ref()));
    key(&headers[0], "End");
    assert_eq!(active_element().as_ref(), Some(headers[3].as_ref()));
}

#[wasm_bindgen_test]
async fn animation_class_hides_content_and_restores_tabindex() {
    let value = RwSignal::new(Vec::new());
    let host = host();
    mount(&host, value, false, noop(), noop());
    leptos::task::tick().await;
    let motion = all(&host, ".p-motion")[0].clone();
    let inner = all(&host, ".inner")[0].clone();
    assert_eq!(motion.style().get_property_value("display").unwrap(), "none", "скрыта с первого рендера");
    assert_eq!(inner.get_attribute("tabindex").as_deref(), Some("-1"));

    all(&host, ".p-accordionheader")[0].click();
    leptos::task::tick().await;
    assert!(motion.class_name().contains("p-collapsible-enter-active"), "{}", motion.class_name());
    assert_ne!(motion.style().get_property_value("display").unwrap(), "none");
    assert!(inner.get_attribute("tabindex").is_none());
    sleep(700).await;
    assert!(!motion.class_name().contains("p-collapsible-enter-active"));

    all(&host, ".p-accordionheader")[0].click();
    leptos::task::tick().await;
    assert!(motion.class_name().contains("p-collapsible-leave-active"));
    sleep(700).await;
    assert_eq!(motion.style().get_property_value("display").unwrap(), "none");
    assert_eq!(inner.get_attribute("tabindex").as_deref(), Some("-1"));
}

#[wasm_bindgen_test]
async fn select_on_focus_toggles_on_focus() {
    let value = RwSignal::new(Vec::new());
    let host = host();
    leptos::mount::mount_to(host.clone(), move || {
        view! {
            <Accordion select_on_focus=true value=value>
                <AccordionPanel value="0">
                    <AccordionHeader>"H"</AccordionHeader>
                    <AccordionContent>"C"</AccordionContent>
                </AccordionPanel>
            </Accordion>
        }
    })
    .forget();
    leptos::task::tick().await;
    all(&host, ".p-accordionheader")[0].focus().unwrap();
    leptos::task::tick().await;
    assert_eq!(value.get_untracked(), v(&["0"]));
}
