//! Поведение Paginator в реальном браузере (wasm-bindgen-test): кнопки и ссылки, события, границы, переход списком и полем,
//! смена числа строк, внешние сигналы и отступ назад при уменьшении числа записей.
#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use prime_paginator::{Paginator, PaginatorState, RowsPerPage};
use std::sync::{Arc, Mutex};
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use web_sys::{Event, HtmlElement, HtmlInputElement, InputEvent, InputEventInit};

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

fn has(host: &HtmlElement, sel: &str) -> bool {
    host.query_selector(sel).unwrap().is_some()
}

fn links(host: &HtmlElement) -> Vec<String> {
    let all = host.query_selector_all(".p-paginator-page").unwrap();
    (0..all.length()).map(|i| all.item(i).unwrap().text_content().unwrap_or_default().trim().to_owned()).collect()
}

fn selected_link(host: &HtmlElement) -> String {
    q(host, ".p-paginator-page-selected").text_content().unwrap_or_default().trim().to_owned()
}

fn link(host: &HtmlElement, number: &str) -> HtmlElement {
    let all = host.query_selector_all(".p-paginator-page").unwrap();
    (0..all.length())
        .map(|i| all.item(i).unwrap().dyn_into::<HtmlElement>().unwrap())
        .find(|b| b.text_content().unwrap_or_default().trim() == number)
        .unwrap_or_else(|| panic!("нет ссылки {number}"))
}

fn is_disabled(el: &HtmlElement) -> bool {
    el.has_attribute("disabled")
}

type Log = Arc<Mutex<Vec<PaginatorState>>>;

fn page_log() -> (Log, Callback<PaginatorState>) {
    let log: Log = Arc::new(Mutex::new(Vec::new()));
    let l = log.clone();
    (log, Callback::new(move |s: PaginatorState| l.lock().unwrap().push(s)))
}

#[wasm_bindgen_test]
async fn next_prev_first_last_buttons_move_between_pages_and_report() {
    let (log, on_page_change) = page_log();
    let first = RwSignal::new(0);
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <Paginator first=first rows=RwSignal::new(10) total_records=120 on_page_change=on_page_change /> }
    });
    leptos::task::tick().await;
    assert_eq!(selected_link(&host), "1");
    assert!(is_disabled(&q(&host, ".p-paginator-prev")) && !is_disabled(&q(&host, ".p-paginator-next")));
    q(&host, ".p-paginator-next").click();
    leptos::task::tick().await;
    assert_eq!(first.get_untracked(), 10);
    assert_eq!(selected_link(&host), "2");
    q(&host, ".p-paginator-last").click();
    leptos::task::tick().await;
    assert_eq!(first.get_untracked(), 110);
    assert_eq!(selected_link(&host), "12");
    assert!(is_disabled(&q(&host, ".p-paginator-next")) && q(&host, ".p-paginator-last").class_name().contains("p-disabled"));
    q(&host, ".p-paginator-prev").click();
    leptos::task::tick().await;
    assert_eq!(first.get_untracked(), 100);
    q(&host, ".p-paginator-first").click();
    leptos::task::tick().await;
    assert_eq!(first.get_untracked(), 0);
    let pages: Vec<usize> = log.lock().unwrap().iter().map(|s| s.page).collect();
    assert_eq!(pages, [1, 11, 10, 0]);
    let last = log.lock().unwrap()[1];
    assert_eq!((last.first, last.rows, last.page_count), (110, 10, 12));
    host.remove();
}

#[wasm_bindgen_test]
async fn link_clicks_jump_and_the_window_slides() {
    let (log, on_page_change) = page_log();
    let first = RwSignal::new(0);
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <Paginator first=first rows=RwSignal::new(10) total_records=120 on_page_change=on_page_change /> }
    });
    leptos::task::tick().await;
    assert_eq!(links(&host), ["1", "2", "3", "4", "5"]);
    link(&host, "5").click();
    leptos::task::tick().await;
    assert_eq!(first.get_untracked(), 40);
    assert_eq!(links(&host), ["3", "4", "5", "6", "7"]);
    assert_eq!(selected_link(&host), "5");
    assert_eq!(q(&host, ".p-paginator-page-selected").get_attribute("aria-current").as_deref(), Some("page"));
    link(&host, "7").click();
    link(&host, "7").click();
    leptos::task::tick().await;
    assert_eq!(links(&host), ["5", "6", "7", "8", "9"]);
    assert_eq!(log.lock().unwrap().len(), 3, "клик по текущей ссылке тоже вызывает событие, как в PrimeNG");
    host.remove();
}

#[wasm_bindgen_test]
async fn external_first_rows_and_total_update_the_view() {
    let first = RwSignal::new(0);
    let rows = RwSignal::new(10);
    let total = RwSignal::new(120usize);
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <Paginator first=first rows=rows total_records=total show_current_page_report=true current_page_report_template="{first}-{last} of {totalRecords}" /> }
    });
    leptos::task::tick().await;
    assert_eq!(q(&host, ".p-paginator-current").text_content().as_deref(), Some("1-10 of 120"));
    first.set(30);
    leptos::task::tick().await;
    assert_eq!(selected_link(&host), "4");
    assert_eq!(q(&host, ".p-paginator-current").text_content().as_deref(), Some("31-40 of 120"));
    rows.set(20);
    leptos::task::tick().await;
    assert_eq!(links(&host).len(), 5);
    // внешняя смена rows не переставляет first (как в PrimeNG): страница пересчитывается из first
    assert_eq!(q(&host, ".p-paginator-current").text_content().as_deref(), Some("31-50 of 120"));
    total.set(0);
    leptos::task::tick().await;
    assert!(links(&host).is_empty());
    assert!(is_disabled(&q(&host, ".p-paginator-next")));
    host.remove();
}

#[wasm_bindgen_test]
async fn shrinking_total_records_retreats_to_a_valid_page() {
    let (log, on_page_change) = page_log();
    let first = RwSignal::new(110);
    let total = RwSignal::new(120usize);
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <Paginator first=first rows=RwSignal::new(10) total_records=total on_page_change=on_page_change /> }
    });
    leptos::task::tick().await;
    total.set(50);
    leptos::task::tick().await;
    leptos::task::tick().await;
    assert_eq!(first.get_untracked(), 40, "отступ до последней существующей страницы");
    assert_eq!(selected_link(&host), "5");
    assert!(!log.lock().unwrap().is_empty());
    host.remove();
}

#[wasm_bindgen_test]
async fn rows_per_page_dropdown_changes_rows_and_keeps_the_first_record() {
    let (log, on_page_change) = page_log();
    let first = RwSignal::new(20);
    let rows = RwSignal::new(10);
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! {
            <Paginator
                first=first
                rows=rows
                total_records=120
                rows_per_page_options=vec![RowsPerPage::Rows(10), RowsPerPage::Rows(20), RowsPerPage::ShowAll("All".into())]
                on_page_change=on_page_change
            />
        }
    });
    leptos::task::tick().await;
    let dropdown = q(&host, ".p-paginator-rpp-dropdown");
    assert_eq!(q(&host, ".p-paginator-rpp-dropdown .p-select-label").text_content().as_deref(), Some("10"));
    dropdown.click();
    leptos::task::tick().await;
    let option = host.query_selector_all(".p-select-option").unwrap().item(1).unwrap().dyn_into::<HtmlElement>().unwrap();
    assert_eq!(option.get_attribute("aria-label").as_deref(), Some("20"));
    option.click();
    leptos::task::tick().await;
    assert_eq!(rows.get_untracked(), 20);
    assert_eq!(first.get_untracked(), 20, "страница 2 при 20 строках начинается с записи 20");
    assert_eq!(q(&host, ".p-paginator-rpp-dropdown .p-select-label").text_content().as_deref(), Some("20"));
    assert_eq!(log.lock().unwrap().last().map(|s| (s.page, s.rows)), Some((1, 20)));
    // «показать все»: одна страница
    dropdown.click();
    leptos::task::tick().await;
    let all = host.query_selector_all(".p-select-option").unwrap().item(2).unwrap().dyn_into::<HtmlElement>().unwrap();
    assert_eq!(all.get_attribute("aria-label").as_deref(), Some("All"));
    all.click();
    leptos::task::tick().await;
    assert_eq!((rows.get_untracked(), first.get_untracked()), (120, 0));
    assert_eq!(links(&host), ["1"]);
    host.remove();
}

#[wasm_bindgen_test]
async fn jump_to_page_dropdown_navigates() {
    let first = RwSignal::new(0);
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <Paginator first=first rows=RwSignal::new(10) total_records=120 show_jump_to_page_dropdown=true show_page_links=false /> }
    });
    leptos::task::tick().await;
    let dropdown = q(&host, ".p-paginator-jtp-dropdown");
    assert_eq!(q(&host, ".p-paginator-jtp-dropdown .p-select-label").text_content().as_deref(), Some("1 of 12"));
    dropdown.click();
    leptos::task::tick().await;
    assert_eq!(host.query_selector_all(".p-select-option").unwrap().length(), 12);
    let option = host.query_selector_all(".p-select-option").unwrap().item(4).unwrap().dyn_into::<HtmlElement>().unwrap();
    option.click();
    leptos::task::tick().await;
    assert_eq!(first.get_untracked(), 40);
    assert_eq!(q(&host, ".p-paginator-jtp-dropdown .p-select-label").text_content().as_deref(), Some("5 of 12"));
    host.remove();
}

#[wasm_bindgen_test]
async fn jump_to_page_input_navigates_and_follows_the_page() {
    let first = RwSignal::new(0);
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <Paginator first=first rows=RwSignal::new(10) total_records=120 show_jump_to_page_input=true /> }
    });
    leptos::task::tick().await;
    let input: HtmlInputElement = q(&host, ".p-paginator-jtp-input input").dyn_into().unwrap();
    assert_eq!(input.value(), "1");
    input.focus().unwrap();
    input.select();
    let init = InputEventInit::new();
    init.set_bubbles(true);
    init.set_cancelable(true);
    init.set_input_type("insertText");
    init.set_data(Some("7"));
    input.dispatch_event(&InputEvent::new_with_event_init_dict("beforeinput", &init).unwrap()).unwrap();
    leptos::task::tick().await;
    leptos::task::tick().await;
    assert_eq!(first.get_untracked(), 60);
    q(&host, ".p-paginator-next").click();
    leptos::task::tick().await;
    leptos::task::tick().await;
    assert_eq!(input.value(), "8", "поле следует за страницей");
    host.remove();
}

#[wasm_bindgen_test]
async fn hidden_paginator_appears_when_more_pages_exist() {
    let total = RwSignal::new(5usize);
    let host = host();
    let _h =
        leptos::mount::mount_to(host.clone(), move || view! { <Paginator rows=RwSignal::new(10) total_records=total always_show=false /> });
    leptos::task::tick().await;
    assert!(q(&host, ".p-paginator").style().get_property_value("display").unwrap() == "none");
    total.set(55);
    leptos::task::tick().await;
    assert!(q(&host, ".p-paginator").style().get_property_value("display").unwrap().is_empty());
    assert!(has(&host, ".p-paginator-page"));
    host.remove();
}

#[wasm_bindgen_test]
async fn disabled_buttons_do_not_fire_events() {
    let (log, on_page_change) = page_log();
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <Paginator rows=RwSignal::new(10) total_records=120 on_page_change=on_page_change /> }
    });
    leptos::task::tick().await;
    q(&host, ".p-paginator-prev").click();
    q(&host, ".p-paginator-first").click();
    leptos::task::tick().await;
    assert!(log.lock().unwrap().is_empty(), "на первой странице «назад» ничего не делает");
    host.dispatch_event(&Event::new("noop").unwrap()).unwrap();
    host.remove();
}
