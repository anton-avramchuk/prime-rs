//! Поведение DatePicker в реальном браузере (wasm-bindgen-test): открытие и закрытие, выбор даты, нескольких и
//! диапазона, навигация и виды, ввод текста, кнопки панели, `disabled`. Сценарии идут одним последовательным тестом.
#![cfg(target_arch = "wasm32")]
#![recursion_limit = "512"]

use leptos::prelude::*;
use prime_datepicker::calendar::{CalDate, SelectionMode};
use prime_datepicker::{DatePicker, DatePickerView, MonthYear};
use std::sync::{Arc, Mutex};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;
use web_sys::{Event, EventInit, HtmlElement, HtmlInputElement, KeyboardEvent, KeyboardEventInit, MouseEvent, MouseEventInit};

wasm_bindgen_test_configure!(run_in_browser);

type Log = Arc<Mutex<Vec<String>>>;

const TODAY: CalDate = CalDate { year: 2026, month: 10, day: 5 };

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

fn q(host: &HtmlElement, selector: &str) -> Option<HtmlElement> {
    host.query_selector(selector).unwrap().map(|e| e.dyn_into().unwrap())
}

fn must(host: &HtmlElement, selector: &str) -> HtmlElement {
    q(host, selector).unwrap_or_else(|| panic!("нет {selector}: {}", host.outer_html()))
}

fn all(host: &HtmlElement, selector: &str) -> Vec<HtmlElement> {
    let list = host.query_selector_all(selector).unwrap();
    (0..list.length()).map(|i| list.item(i).unwrap().dyn_into().unwrap()).collect()
}

fn input(host: &HtmlElement) -> HtmlInputElement {
    must(host, "input.p-datepicker-input").dyn_into().unwrap()
}

fn day(host: &HtmlElement, month: u32, d: u32) -> HtmlElement {
    must(host, &format!("[data-date=\"2026-{month}-{d}\"]"))
}

fn d(y: i32, m: u32, day: u32) -> CalDate {
    CalDate::new(y, m, day)
}

fn type_text(el: &HtmlInputElement, text: &str) {
    el.set_value(text);
    let init = EventInit::new();
    init.set_bubbles(true);
    el.dispatch_event(&Event::new_with_event_init_dict("input", &init).unwrap()).unwrap();
}

fn key(el: &HtmlElement, key: &str) {
    let init = KeyboardEventInit::new();
    init.set_key(key);
    init.set_bubbles(true);
    init.set_cancelable(true);
    el.dispatch_event(&KeyboardEvent::new_with_keyboard_event_init_dict("keydown", &init).unwrap()).unwrap();
}

fn mouse(target: &web_sys::EventTarget, kind: &str) {
    let init = MouseEventInit::new();
    init.set_bubbles(true);
    init.set_cancelable(true);
    target.dispatch_event(&MouseEvent::new_with_mouse_event_init_dict(kind, &init).unwrap()).unwrap();
}

fn title(host: &HtmlElement) -> String {
    all(host, ".p-datepicker-title button, .p-datepicker-decade")
        .iter()
        .map(|e| e.text_content().unwrap_or_default().trim().to_owned())
        .collect::<Vec<_>>()
        .join(" ")
}

async fn open_select_and_close() {
    let (host, log) = (host(), Log::default());
    let value = RwSignal::new(Vec::new());
    let (l_show, l_hide, l_sel) = (log.clone(), log.clone(), log.clone());
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! {
            <DatePicker
                value=value
                today=TODAY
                on_show=Callback::new(move |()| l_show.lock().unwrap().push("show".into()))
                on_hide=Callback::new(move |()| l_hide.lock().unwrap().push("hide".into()))
                on_select=Callback::new(move |date: CalDate| l_sel.lock().unwrap().push(format!("select:{}-{}-{}", date.year, date.month, date.day)))
            />
        }
    });
    settle().await;
    let field = input(&host);
    assert!(q(&host, ".p-datepicker-panel").is_none());
    field.click();
    settle().await;
    assert!(q(&host, ".p-datepicker-panel").is_some(), "клик по полю открывает панель");
    assert_eq!(title(&host), "November 2026");
    assert_eq!(must(&host, ".p-datepicker-panel").get_attribute("role").as_deref(), Some("dialog"));
    assert!(must(&host, "[data-date=\"2026-10-5\"]").parent_element().unwrap().class_name().contains("p-datepicker-today"));

    day(&host, 10, 12).click();
    sleep(260).await;
    assert_eq!(value.get_untracked(), [d(2026, 10, 12)]);
    assert_eq!(field.value(), "11/12/2026", "текст поля обновлён");
    assert!(q(&host, ".p-datepicker-panel").is_none(), "после выбора панель закрыта");
    assert_eq!(log.lock().unwrap().as_slice(), ["show", "select:2026-10-12", "hide"]);

    // повторное открытие показывает выбранный день
    field.click();
    settle().await;
    assert!(day(&host, 10, 12).class_name().contains("p-datepicker-day-selected"));
    // Escape закрывает
    key(&field.clone().unchecked_into(), "Escape");
    settle().await;
    assert!(q(&host, ".p-datepicker-panel").is_none(), "Escape закрывает");
    // клик снаружи
    field.click();
    settle().await;
    let body = doc().body().unwrap();
    mouse(&body, "click");
    settle().await;
    assert!(q(&host, ".p-datepicker-panel").is_none(), "клик снаружи закрывает");
    host.remove();
}

async fn navigation_and_views() {
    let (host, log) = (host(), Log::default());
    let (l_month, l_year) = (log.clone(), log.clone());
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! {
            <DatePicker
                inline=true
                today=TODAY
                on_month_change=Callback::new(move |m: MonthYear| l_month.lock().unwrap().push(format!("m:{}/{}", m.month, m.year)))
                on_year_change=Callback::new(move |m: MonthYear| l_year.lock().unwrap().push(format!("y:{}/{}", m.month, m.year)))
            />
        }
    });
    settle().await;
    assert_eq!(title(&host), "November 2026");
    must(&host, ".p-datepicker-next-button").click();
    settle().await;
    assert_eq!(title(&host), "December 2026");
    must(&host, ".p-datepicker-next-button").click();
    settle().await;
    assert_eq!(title(&host), "January 2027", "переход через год");
    must(&host, ".p-datepicker-prev-button").click();
    must(&host, ".p-datepicker-prev-button").click();
    must(&host, ".p-datepicker-prev-button").click();
    settle().await;
    assert_eq!(title(&host), "October 2026");

    // вид «месяцы»: стрелки листают год, выбор месяца возвращает к дням
    must(&host, ".p-datepicker-select-month").click();
    settle().await;
    assert!(q(&host, ".p-datepicker-month-view").is_some() && q(&host, ".p-datepicker-day-view").is_none());
    assert_eq!(title(&host), "2026");
    must(&host, ".p-datepicker-next-button").click();
    settle().await;
    assert_eq!(title(&host), "2027");
    all(&host, ".p-datepicker-month")[2].click();
    settle().await;
    assert_eq!(title(&host), "March 2027");
    assert!(q(&host, ".p-datepicker-day-view").is_some());

    // вид «годы»: десятилетие и выбор года ведёт к месяцам
    must(&host, ".p-datepicker-select-year").click();
    settle().await;
    assert_eq!(title(&host), "2020 - 2029");
    must(&host, ".p-datepicker-next-button").click();
    settle().await;
    assert_eq!(title(&host), "2030 - 2039");
    all(&host, ".p-datepicker-year")[0].click();
    settle().await;
    assert!(q(&host, ".p-datepicker-month-view").is_some());
    assert_eq!(title(&host), "2030");
    all(&host, ".p-datepicker-month")[0].click();
    settle().await;
    assert_eq!(title(&host), "January 2030");
    let log = log.lock().unwrap().clone();
    assert_eq!(log.first().map(String::as_str), Some("m:12/2026"), "{log:?}");
    assert!(log.contains(&"y:1/2030".to_owned()) || log.iter().any(|l| l.starts_with("y:")), "{log:?}");
    host.remove();
}

async fn multiple_and_range() {
    let host = host();
    let value = RwSignal::new(Vec::new());
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <DatePicker inline=true today=TODAY value=value selection_mode=SelectionMode::Multiple max_date_count=2usize /> }
    });
    settle().await;
    day(&host, 10, 3).click();
    day(&host, 10, 7).click();
    day(&host, 10, 9).click();
    settle().await;
    assert_eq!(value.get_untracked(), [d(2026, 10, 3), d(2026, 10, 7)], "предел max_date_count");
    day(&host, 10, 3).click();
    settle().await;
    assert_eq!(value.get_untracked(), [d(2026, 10, 7)], "повторный клик снимает");
    host.remove();

    let host = host_again();
    let value = RwSignal::new(Vec::new());
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <DatePicker inline=true today=TODAY value=value selection_mode=SelectionMode::Range /> }
    });
    settle().await;
    day(&host, 10, 5).click();
    settle().await;
    assert_eq!(value.get_untracked(), [d(2026, 10, 5)]);
    day(&host, 10, 9).click();
    settle().await;
    assert_eq!(value.get_untracked(), [d(2026, 10, 5), d(2026, 10, 9)]);
    assert!(day(&host, 10, 5).class_name().contains("p-datepicker-day-selected"));
    assert!(day(&host, 10, 7).class_name().contains("p-datepicker-day-selected-range"));
    assert!(day(&host, 10, 9).class_name().contains("p-datepicker-day-selected"));
    day(&host, 10, 2).click();
    settle().await;
    assert_eq!(value.get_untracked(), [d(2026, 10, 2)], "новый выбор начинает диапазон заново");
    host.remove();
}

fn host_again() -> HtmlElement {
    host()
}

async fn typing_and_limits() {
    let host = host();
    let value = RwSignal::new(Vec::new());
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <DatePicker today=TODAY value=value min_date=d(2026, 10, 10) max_date=d(2026, 10, 25) show_on_focus=false /> }
    });
    settle().await;
    let field = input(&host);
    type_text(&field, "11/15/2026");
    settle().await;
    assert_eq!(value.get_untracked(), [d(2026, 10, 15)], "корректная дата принята");
    type_text(&field, "11/30/2026");
    settle().await;
    assert_eq!(value.get_untracked(), [d(2026, 10, 15)], "дата вне границ игнорируется");
    type_text(&field, "garbage");
    settle().await;
    assert_eq!(value.get_untracked(), [d(2026, 10, 15)]);
    field.blur().unwrap();
    let _ = field.focus();
    field.blur().unwrap();
    settle().await;
    assert_eq!(field.value(), "11/15/2026", "при потере фокуса текст возвращается к значению");
    type_text(&field, "");
    settle().await;
    assert!(value.get_untracked().is_empty(), "пустой текст очищает значение");

    // внешнее изменение значения обновляет текст
    value.set(vec![d(2026, 10, 20)]);
    settle().await;
    assert_eq!(field.value(), "11/20/2026");
    host.remove();
}

async fn button_bar_clear_and_icons() {
    let (host, log) = (host(), Log::default());
    let (l1, l2, l3) = (log.clone(), log.clone(), log.clone());
    let value = RwSignal::new(vec![d(2026, 10, 20)]);
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! {
            <DatePicker
                today=TODAY
                value=value
                inline=true
                show_button_bar=true
                on_today_click=Callback::new(move |_| l1.lock().unwrap().push("today".into()))
                on_clear_click=Callback::new(move |()| l2.lock().unwrap().push("clear".into()))
                on_select=Callback::new(move |_| l3.lock().unwrap().push("select".into()))
            />
        }
    });
    settle().await;
    must(&host, ".p-datepicker-today-button").click();
    settle().await;
    assert_eq!(value.get_untracked(), [TODAY], "«Сегодня» выбирает сегодняшнюю дату");
    must(&host, ".p-datepicker-clear-button").click();
    settle().await;
    assert!(value.get_untracked().is_empty());
    assert_eq!(log.lock().unwrap().as_slice(), ["select", "today", "clear"]);
    host.remove();

    // значок календаря открывает панель, значок очистки очищает поле
    let host = host_again();
    let value = RwSignal::new(vec![d(2026, 10, 20)]);
    let _h = leptos::mount::mount_to(
        host.clone(),
        move || view! { <DatePicker today=TODAY value=value show_icon=true show_clear=true show_on_focus=false /> },
    );
    settle().await;
    assert!(q(&host, ".p-datepicker-panel").is_none());
    must(&host, ".p-datepicker-dropdown").click();
    settle().await;
    assert!(q(&host, ".p-datepicker-panel").is_some(), "кнопка открывает панель");
    assert!(day(&host, 10, 20).class_name().contains("p-datepicker-day-selected"));
    must(&host, ".p-datepicker-dropdown").click();
    settle().await;
    assert!(q(&host, ".p-datepicker-panel").is_none(), "повторный клик закрывает");
    mouse(&host.query_selector(".p-datepicker-clear-icon").unwrap().expect("нет значка очистки"), "click");
    settle().await;
    assert!(value.get_untracked().is_empty() && input(&host).value().is_empty());
    host.remove();
}

async fn month_view_selects_and_disabled() {
    let host = host();
    let value = RwSignal::new(Vec::new());
    let _h = leptos::mount::mount_to(
        host.clone(),
        move || view! { <DatePicker inline=true today=TODAY value=value view=DatePickerView::Month /> },
    );
    settle().await;
    all(&host, ".p-datepicker-month")[3].click();
    settle().await;
    assert_eq!(value.get_untracked(), [d(2026, 3, 1)], "в виде «месяцы» выбирается первое число месяца");
    assert!(all(&host, ".p-datepicker-month")[3].class_name().contains("p-datepicker-month-selected"));
    host.remove();

    let host = host_again();
    let value = RwSignal::new(Vec::new());
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <DatePicker inline=true today=TODAY value=value disabled=true /> });
    settle().await;
    day(&host, 10, 12).click();
    settle().await;
    assert!(value.get_untracked().is_empty(), "заблокированный не выбирает");
    assert!(must(&host, ".p-datepicker-panel").class_name().contains("p-disabled"));
    host.remove();
}

#[wasm_bindgen_test]
async fn datepicker_in_the_browser() {
    open_select_and_close().await;
    navigation_and_views().await;
    multiple_and_range().await;
    typing_and_limits().await;
    button_bar_clear_and_icons().await;
    month_view_selects_and_disabled().await;
}
