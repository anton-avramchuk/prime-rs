//! Поведение OrderList в реальном браузере (wasm-bindgen-test): выбор в списке, четыре кнопки, `on_reorder`,
//! сохранение выбора, `disabled`. Сценарии идут одним последовательным тестом.
#![cfg(target_arch = "wasm32")]
#![recursion_limit = "512"]

use leptos::prelude::*;
use prime_listbox::ListboxOption;
use prime_orderlist::OrderList;
use std::sync::{Arc, Mutex};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;
use web_sys::HtmlElement;

wasm_bindgen_test_configure!(run_in_browser);

type Log = Arc<Mutex<Vec<String>>>;

fn doc() -> web_sys::Document {
    web_sys::window().unwrap().document().unwrap()
}

fn host() -> HtmlElement {
    let el = doc().create_element("div").unwrap().dyn_into::<HtmlElement>().unwrap();
    doc().body().unwrap().append_child(&el).unwrap();
    el
}

async fn sleep(ms: i32) {
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms).unwrap();
    });
    JsFuture::from(promise).await.unwrap();
}

async fn settle() {
    sleep(40).await;
}

fn all(host: &HtmlElement, selector: &str) -> Vec<HtmlElement> {
    let list = host.query_selector_all(selector).unwrap();
    (0..list.length()).map(|i| list.item(i).unwrap().dyn_into().unwrap()).collect()
}

fn labels(host: &HtmlElement) -> String {
    all(host, ".p-listbox-option").iter().map(|e| e.text_content().unwrap_or_default().trim().to_owned()).collect::<Vec<_>>().join(" ")
}

fn button(host: &HtmlElement, class: &str) -> HtmlElement {
    host.query_selector(class).unwrap().unwrap_or_else(|| panic!("нет {class}: {}", host.outer_html())).dyn_into().unwrap()
}

fn items(names: &[&str]) -> RwSignal<Vec<ListboxOption>> {
    RwSignal::new(names.iter().map(|n| ListboxOption::new(*n, n.to_lowercase())).collect())
}

async fn buttons_reorder_selected() {
    let (host, log) = (host(), Log::default());
    let value = items(&["A", "B", "C", "D", "E"]);
    let (l_re, l_sel) = (log.clone(), log.clone());
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! {
            <OrderList
                value=value
                on_reorder=Callback::new(move |sel: Vec<String>| l_re.lock().unwrap().push(format!("reorder:{}", sel.join(","))))
                on_selection_change=Callback::new(move |sel: Vec<String>| l_sel.lock().unwrap().push(format!("select:{}", sel.join(","))))
            />
        }
    });
    settle().await;
    assert_eq!(labels(&host), "A B C D E");
    let up = button(&host, ".p-orderlist-move-up-button");
    assert!(up.has_attribute("disabled"), "без выбора кнопки недоступны");

    all(&host, ".p-listbox-option")[2].click();
    settle().await;
    assert!(!up.has_attribute("disabled"), "после выбора кнопки доступны");
    up.click();
    settle().await;
    assert_eq!(labels(&host), "A C B D E", "C поднялся");
    assert!(all(&host, ".p-listbox-option")[1].class_name().contains("p-listbox-option-selected"), "выбор следует за элементом");
    button(&host, ".p-orderlist-move-top-button").click();
    settle().await;
    assert_eq!(labels(&host), "C A B D E");
    button(&host, ".p-orderlist-move-down-button").click();
    settle().await;
    assert_eq!(labels(&host), "A C B D E");
    button(&host, ".p-orderlist-move-bottom-button").click();
    settle().await;
    assert_eq!(labels(&host), "A B D E C");
    let log_now = log.lock().unwrap().clone();
    assert_eq!(log_now.first().map(String::as_str), Some("select:c"), "{log_now:?}");
    assert_eq!(log_now.iter().filter(|l| *l == "reorder:c").count(), 4, "{log_now:?}");
    assert_eq!(value.get_untracked().iter().map(|o| o.value.clone()).collect::<Vec<_>>(), ["a", "b", "d", "e", "c"]);
    host.remove();
}

async fn several_selected_keep_their_order() {
    let host = host();
    let value = items(&["A", "B", "C", "D", "E"]);
    let selection = RwSignal::new(vec!["b".to_owned(), "d".to_owned()]);
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <OrderList value=value selection=selection /> });
    settle().await;
    button(&host, ".p-orderlist-move-up-button").click();
    settle().await;
    assert_eq!(labels(&host), "B A D C E");
    button(&host, ".p-orderlist-move-up-button").click();
    settle().await;
    assert_eq!(labels(&host), "B D A C E", "блок у края остаётся на месте, второй подтягивается");
    button(&host, ".p-orderlist-move-bottom-button").click();
    settle().await;
    assert_eq!(labels(&host), "A C E B D");
    assert_eq!(selection.get_untracked(), ["b", "d"], "выбор сохранён");
    host.remove();
}

async fn disabled_and_filter() {
    let host = host();
    let value = items(&["Apple", "Banana", "Cherry"]);
    let selection = RwSignal::new(vec!["banana".to_owned()]);
    let _h =
        leptos::mount::mount_to(host.clone(), move || view! { <OrderList value=value selection=selection filter=true disabled=true /> });
    settle().await;
    assert!(button(&host, ".p-orderlist-move-up-button").has_attribute("disabled"), "disabled блокирует кнопки");
    host.remove();

    let host = host_again();
    let value = items(&["Apple", "Banana", "Cherry"]);
    let selection = RwSignal::new(vec!["cherry".to_owned()]);
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <OrderList value=value selection=selection filter=true /> });
    settle().await;
    button(&host, ".p-orderlist-move-top-button").click();
    settle().await;
    assert_eq!(labels(&host), "Cherry Apple Banana", "перестановка идёт по полному списку");
    host.remove();
}

fn host_again() -> HtmlElement {
    host()
}

#[wasm_bindgen_test]
async fn orderlist_in_the_browser() {
    buttons_reorder_selected().await;
    several_selected_keep_their_order().await;
    disabled_and_filter().await;
}
