//! Поведение Rating в реальном браузере (wasm-bindgen-test): выбор и сброс кликом, `readonly`, `disabled`, `change` радиокнопки,
//! события, внешний сигнал, реактивное число звёзд, видимый фокус и сохранение элементов звёзд.
#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use prime_rating::{Rating, RatingRate};
use std::sync::{Arc, Mutex};
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

/// Опция звезды с номером `n` (с 1).
fn option(host: &HtmlElement, n: u32) -> HtmlElement {
    host.query_selector(&format!(".p-rating > .p-rating-option:nth-child({n})"))
        .unwrap()
        .unwrap_or_else(|| panic!("нет звезды {n}"))
        .dyn_into()
        .unwrap()
}

fn input(host: &HtmlElement, n: u32) -> HtmlInputElement {
    option(host, n).query_selector("input").unwrap().unwrap().dyn_into().unwrap()
}

/// Состояние: по символу на звезду (`1` — активна), по классу `p-rating-option-active`.
fn active(host: &HtmlElement) -> String {
    let list = host.query_selector_all(".p-rating > .p-rating-option").unwrap();
    (0..list.length())
        .map(|i| {
            let el: HtmlElement = list.item(i).unwrap().dyn_into().unwrap();
            if el.class_name().contains("p-rating-option-active") { '1' } else { '0' }
        })
        .collect()
}

/// Журнал `on_rate`: значение или `-` для сброса.
fn log() -> (Arc<Mutex<Vec<String>>>, Callback<RatingRate>) {
    let log = Arc::new(Mutex::new(Vec::new()));
    let l = log.clone();
    (log, Callback::new(move |e: RatingRate| l.lock().unwrap().push(e.value.map_or("-".to_owned(), |v| v.to_string()))))
}

#[wasm_bindgen_test]
async fn click_selects_and_second_click_clears() {
    let value = RwSignal::new(None::<u32>);
    let (log, on_rate) = log();
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Rating value=value on_rate=on_rate /> });
    leptos::task::tick().await;
    option(&host, 3).click();
    leptos::task::tick().await;
    assert_eq!(value.get_untracked(), Some(3));
    assert_eq!(active(&host), "11100");
    assert!(input(&host, 3).checked() && !input(&host, 2).checked());
    option(&host, 5).click();
    leptos::task::tick().await;
    assert_eq!((value.get_untracked(), active(&host).as_str()), (Some(5), "11111"));
    option(&host, 5).click();
    leptos::task::tick().await;
    assert_eq!((value.get_untracked(), active(&host).as_str()), (None, "00000"));
    assert!(!input(&host, 5).checked(), "радиокнопка осталась отмеченной после сброса");
    assert_eq!(*log.lock().unwrap(), ["3", "5", "-"]);
    host.remove();
}

#[wasm_bindgen_test]
async fn uncontrolled_keeps_internal_state() {
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Rating default_value=2 /> });
    leptos::task::tick().await;
    assert_eq!(active(&host), "11000");
    option(&host, 4).click();
    leptos::task::tick().await;
    assert_eq!(active(&host), "11110");
    host.remove();
}

#[wasm_bindgen_test]
async fn readonly_and_disabled_do_not_change_value() {
    let value = RwSignal::new(Some(2u32));
    let (log, on_rate) = log();
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! {
            <div id="ro"><Rating value=value readonly=true on_rate=on_rate /></div>
            <div id="dis"><Rating value=value disabled=true on_rate=on_rate /></div>
        }
    });
    leptos::task::tick().await;
    for sel in ["#ro .p-rating-option:nth-child(4)", "#dis .p-rating-option:nth-child(4)"] {
        host.query_selector(sel).unwrap().unwrap().dyn_into::<HtmlElement>().unwrap().click();
    }
    leptos::task::tick().await;
    assert_eq!(value.get_untracked(), Some(2));
    assert!(log.lock().unwrap().is_empty());
    host.remove();
}

#[wasm_bindgen_test]
async fn radio_click_is_handled_once_by_change_without_clearing() {
    let value = RwSignal::new(Some(1u32));
    let (log, on_rate) = log();
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Rating value=value on_rate=on_rate /> });
    leptos::task::tick().await;
    // нативный `click()` по радиокнопке — как стрелка клавиатуры: приходят `click` и `change`
    input(&host, 2).click();
    leptos::task::tick().await;
    assert_eq!(value.get_untracked(), Some(2));
    input(&host, 3).click();
    leptos::task::tick().await;
    assert_eq!(value.get_untracked(), Some(3));
    assert_eq!(*log.lock().unwrap(), ["2", "3"]);
    host.remove();
}

#[wasm_bindgen_test]
async fn external_signal_and_reactive_stars_update_dom() {
    let value = RwSignal::new(None::<u32>);
    let stars = RwSignal::new(Some(3u32));
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Rating value=value stars=stars /> });
    leptos::task::tick().await;
    assert_eq!(active(&host), "000");
    value.set(Some(2));
    leptos::task::tick().await;
    assert_eq!(active(&host), "110");
    assert!(input(&host, 2).checked());
    stars.set(Some(6));
    leptos::task::tick().await;
    assert_eq!(active(&host), "110000");
    host.remove();
}

#[wasm_bindgen_test]
async fn selecting_keeps_star_elements_and_focus() {
    let value = RwSignal::new(None::<u32>);
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Rating value=value /> });
    leptos::task::tick().await;
    let first = option(&host, 2);
    first.click();
    leptos::task::tick().await;
    assert!(first.is_same_node(Some(&option(&host, 2))), "звёзды пересозданы после выбора");
    assert!(input(&host, 2).is_same_node(doc().active_element().as_deref()), "после клика фокус должен стоять на радиокнопке звезды");
    // следующий выбор тоже не роняет фокус
    option(&host, 4).click();
    leptos::task::tick().await;
    assert!(input(&host, 4).is_same_node(doc().active_element().as_deref()));
    host.remove();
}

#[wasm_bindgen_test]
async fn focus_and_blur_callbacks_fire_and_mouse_focus_is_not_visible() {
    let n = Arc::new(Mutex::new(Vec::<&'static str>::new()));
    let (a, b) = (n.clone(), n.clone());
    let focus = Callback::new(move |_: FocusEvent| a.lock().unwrap().push("focus"));
    let blur = Callback::new(move |_: FocusEvent| b.lock().unwrap().push("blur"));
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Rating on_focus=focus on_blur=blur /> });
    leptos::task::tick().await;
    option(&host, 2).click();
    leptos::task::tick().await;
    assert_eq!(*n.lock().unwrap(), ["focus"]);
    assert!(!option(&host, 2).class_name().contains("p-focus-visible"), "фокус после клика мышью не должен быть видимым");
    input(&host, 2).blur().unwrap();
    leptos::task::tick().await;
    assert_eq!(*n.lock().unwrap(), ["focus", "blur"]);
    host.remove();
}
