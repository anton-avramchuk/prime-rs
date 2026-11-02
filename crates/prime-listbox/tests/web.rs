//! Поведение Listbox в реальном браузере (wasm-bindgen-test): выбор мышью и клавишами, мета-режим, множественный выбор,
//! диапазоны, `Ctrl+A`, поиск набором и полем, флажки, события, блокировки, фокус.
#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use prime_listbox::{Listbox, ListboxChange, ListboxEvent, ListboxFilter, ListboxGroup, ListboxOption, ListboxSelectAll};
use std::sync::{Arc, Mutex};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;
use web_sys::{Event, FocusEvent, HtmlElement, HtmlInputElement, KeyboardEvent, KeyboardEventInit, MouseEvent, MouseEventInit};

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

fn list(host: &HtmlElement) -> HtmlElement {
    q(host, ".p-listbox-list")
}

fn opt(host: &HtmlElement, i: u32) -> HtmlElement {
    host.query_selector_all(".p-listbox-option").unwrap().item(i).unwrap_or_else(|| panic!("нет варианта {i}")).dyn_into().unwrap()
}

fn count(host: &HtmlElement, sel: &str) -> u32 {
    host.query_selector_all(sel).unwrap().length()
}

fn selected(host: &HtmlElement) -> Vec<String> {
    let all = host.query_selector_all(".p-listbox-option").unwrap();
    (0..all.length())
        .map(|i| all.item(i).unwrap().dyn_into::<HtmlElement>().unwrap())
        .filter(|o| o.get_attribute("aria-selected").as_deref() == Some("true"))
        .map(|o| o.get_attribute("aria-label").unwrap())
        .collect()
}

fn focused_label(host: &HtmlElement) -> Option<String> {
    host.query_selector(".p-listbox-option.p-focus").unwrap().and_then(|o| o.get_attribute("aria-label"))
}

fn key_with(el: &HtmlElement, code: &str, key: &str, shift: bool, ctrl: bool) {
    let init = KeyboardEventInit::new();
    init.set_code(code);
    init.set_key(key);
    init.set_shift_key(shift);
    init.set_ctrl_key(ctrl);
    init.set_bubbles(true);
    init.set_cancelable(true);
    el.dispatch_event(&KeyboardEvent::new_with_keyboard_event_init_dict("keydown", &init).unwrap()).unwrap();
}

fn press(host: &HtmlElement, code: &str) {
    key_with(&list(host), code, code, false, false);
}

fn press_shift(host: &HtmlElement, code: &str) {
    key_with(&list(host), code, code, true, false);
}

fn click_with(el: &HtmlElement, ctrl: bool) {
    let init = MouseEventInit::new();
    init.set_ctrl_key(ctrl);
    init.set_bubbles(true);
    init.set_cancelable(true);
    el.dispatch_event(&MouseEvent::new_with_mouse_event_init_dict("click", &init).unwrap()).unwrap();
}

fn focus_list(host: &HtmlElement) {
    list(host).focus().unwrap();
}

async fn sleep(ms: i32) {
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms).unwrap();
    });
    JsFuture::from(promise).await.unwrap();
}

fn cities() -> Vec<ListboxOption> {
    vec![
        ListboxOption::new("New York", "NY"),
        ListboxOption::new("Rome", "RM").disabled(true),
        ListboxOption::new("London", "LDN"),
        ListboxOption::new("Istanbul", "IST"),
        ListboxOption::new("Paris", "PRS"),
    ]
}

type Log = Arc<Mutex<Vec<String>>>;

fn change_log() -> (Log, Callback<ListboxChange>) {
    let log: Log = Arc::new(Mutex::new(Vec::new()));
    let l = log.clone();
    let cb = Callback::new(move |e: ListboxChange| {
        let text = if e.values.is_empty() { "-".to_owned() } else { e.values.join(",") };
        l.lock().unwrap().push(text);
    });
    (log, cb)
}

#[wasm_bindgen_test]
async fn click_selects_and_second_click_clears_in_single_mode() {
    let (log, on_change) = change_log();
    let value = RwSignal::new(None::<String>);
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Listbox options=cities() value=value on_change=on_change /> });
    leptos::task::tick().await;
    opt(&host, 0).click();
    leptos::task::tick().await;
    assert_eq!(value.get_untracked().as_deref(), Some("NY"));
    assert!(opt(&host, 0).class_name().contains("p-listbox-option-selected"));
    assert_eq!(opt(&host, 0).get_attribute("aria-selected").as_deref(), Some("true"));
    opt(&host, 2).click();
    leptos::task::tick().await;
    assert_eq!(selected(&host), ["London"], "выбор заменяет прежний");
    opt(&host, 2).click();
    leptos::task::tick().await;
    assert_eq!(value.get_untracked(), None);
    assert!(selected(&host).is_empty());
    assert_eq!(*log.lock().unwrap(), ["NY", "LDN", "-"]);
    host.remove();
}

#[wasm_bindgen_test]
async fn external_signal_changes_the_selection() {
    let value = RwSignal::new(Some("NY".to_owned()));
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Listbox options=cities() value=value /> });
    leptos::task::tick().await;
    assert_eq!(selected(&host), ["New York"]);
    value.set(Some("PRS".to_owned()));
    leptos::task::tick().await;
    assert_eq!(selected(&host), ["Paris"]);
    host.remove();
}

#[wasm_bindgen_test]
async fn disabled_option_readonly_and_disabled_list_ignore_clicks() {
    let (log, on_change) = change_log();
    let value = RwSignal::new(None::<String>);
    let readonly = RwSignal::new(false);
    let disabled = RwSignal::new(false);
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <Listbox options=cities() value=value readonly=readonly disabled=disabled on_change=on_change /> }
    });
    leptos::task::tick().await;
    opt(&host, 1).click();
    leptos::task::tick().await;
    assert_eq!(value.get_untracked(), None, "недоступный вариант");
    readonly.set(true);
    opt(&host, 0).click();
    leptos::task::tick().await;
    assert_eq!(value.get_untracked(), None, "readonly");
    readonly.set(false);
    disabled.set(true);
    leptos::task::tick().await;
    assert_eq!(list(&host).get_attribute("tabindex").as_deref(), Some("-1"));
    opt(&host, 0).click();
    leptos::task::tick().await;
    assert_eq!(value.get_untracked(), None, "disabled");
    assert!(log.lock().unwrap().is_empty());
    host.remove();
}

#[wasm_bindgen_test]
async fn meta_key_selection_needs_ctrl_to_clear_in_single_mode() {
    let value = RwSignal::new(Some("NY".to_owned()));
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Listbox options=cities() value=value meta_key_selection=true /> });
    leptos::task::tick().await;
    click_with(&opt(&host, 0), false);
    leptos::task::tick().await;
    assert_eq!(value.get_untracked().as_deref(), Some("NY"), "без Ctrl выбор остаётся");
    click_with(&opt(&host, 0), true);
    leptos::task::tick().await;
    assert_eq!(value.get_untracked(), None);
    host.remove();
}

#[wasm_bindgen_test]
async fn multiple_click_adds_and_removes() {
    let (log, on_change) = change_log();
    let values = RwSignal::new(Vec::<String>::new());
    let host = host();
    let _h = leptos::mount::mount_to(
        host.clone(),
        move || view! { <Listbox options=cities() multiple=true values=values on_change=on_change /> },
    );
    leptos::task::tick().await;
    opt(&host, 0).click();
    opt(&host, 2).click();
    leptos::task::tick().await;
    assert_eq!(selected(&host), ["New York", "London"]);
    opt(&host, 0).click();
    leptos::task::tick().await;
    assert_eq!(values.get_untracked(), ["LDN"]);
    assert_eq!(*log.lock().unwrap(), ["NY", "NY,LDN", "LDN"]);
    host.remove();
}

#[wasm_bindgen_test]
async fn multiple_meta_selection_replaces_without_ctrl_and_adds_with_it() {
    let values = RwSignal::new(vec!["NY".to_owned(), "LDN".to_owned()]);
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <Listbox options=cities() multiple=true values=values meta_key_selection=true /> }
    });
    leptos::task::tick().await;
    click_with(&opt(&host, 3), false);
    leptos::task::tick().await;
    assert_eq!(values.get_untracked(), ["IST"]);
    click_with(&opt(&host, 4), true);
    leptos::task::tick().await;
    assert_eq!(values.get_untracked(), ["IST", "PRS"]);
    click_with(&opt(&host, 3), true);
    leptos::task::tick().await;
    assert_eq!(values.get_untracked(), ["PRS"]);
    host.remove();
}

#[wasm_bindgen_test]
async fn list_focus_highlights_first_valid_or_selected_option_and_blur_clears() {
    let value = RwSignal::new(None::<String>);
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Listbox id="kb" options=cities() value=value /> });
    leptos::task::tick().await;
    assert_eq!(focused_label(&host), None);
    focus_list(&host);
    leptos::task::tick().await;
    assert_eq!(focused_label(&host).as_deref(), Some("New York"));
    assert_eq!(list(&host).get_attribute("aria-activedescendant").as_deref(), Some("kb_0"));
    assert_eq!(opt(&host, 0).get_attribute("data-p-focused").as_deref(), Some("true"));
    list(&host).blur().unwrap();
    leptos::task::tick().await;
    assert_eq!(focused_label(&host), None);
    assert!(list(&host).get_attribute("aria-activedescendant").is_none());
    // с выбранным значением фокус на нём
    value.set(Some("IST".to_owned()));
    focus_list(&host);
    leptos::task::tick().await;
    assert_eq!(focused_label(&host).as_deref(), Some("Istanbul"));
    host.remove();
}

#[wasm_bindgen_test]
async fn arrows_home_end_skip_disabled_and_enter_space_select() {
    let value = RwSignal::new(None::<String>);
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Listbox options=cities() value=value /> });
    leptos::task::tick().await;
    focus_list(&host);
    press(&host, "ArrowDown");
    leptos::task::tick().await;
    assert_eq!(focused_label(&host).as_deref(), Some("London"), "Rome пропущен");
    press(&host, "End");
    leptos::task::tick().await;
    assert_eq!(focused_label(&host).as_deref(), Some("Paris"));
    press(&host, "ArrowDown");
    leptos::task::tick().await;
    assert_eq!(focused_label(&host).as_deref(), Some("Paris"), "дальше некуда");
    press(&host, "ArrowUp");
    leptos::task::tick().await;
    assert_eq!(focused_label(&host).as_deref(), Some("Istanbul"));
    press(&host, "Home");
    leptos::task::tick().await;
    assert_eq!(focused_label(&host).as_deref(), Some("New York"));
    press(&host, "Enter");
    leptos::task::tick().await;
    assert_eq!(value.get_untracked().as_deref(), Some("NY"));
    press(&host, "Space");
    leptos::task::tick().await;
    assert_eq!(value.get_untracked(), None, "Space снимает выбор");
    host.remove();
}

#[wasm_bindgen_test]
async fn shift_arrows_select_a_range_and_ctrl_a_selects_everything_valid() {
    let values = RwSignal::new(Vec::<String>::new());
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Listbox options=cities() multiple=true values=values /> });
    leptos::task::tick().await;
    focus_list(&host);
    leptos::task::tick().await;
    key_with(&list(&host), "ShiftLeft", "Shift", true, false);
    press_shift(&host, "ArrowDown");
    press_shift(&host, "ArrowDown");
    leptos::task::tick().await;
    assert_eq!(values.get_untracked(), ["NY", "LDN", "IST"], "диапазон без отключённого");
    press_shift(&host, "ArrowUp");
    leptos::task::tick().await;
    assert_eq!(values.get_untracked(), ["NY", "LDN"]);
    key_with(&list(&host), "KeyA", "a", false, true);
    leptos::task::tick().await;
    assert_eq!(values.get_untracked(), ["NY", "LDN", "IST", "PRS"]);
    host.remove();
}

#[wasm_bindgen_test]
async fn typing_moves_focus_to_a_matching_option_and_resets_after_a_pause() {
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Listbox options=cities() /> });
    leptos::task::tick().await;
    focus_list(&host);
    key_with(&list(&host), "KeyP", "p", false, false);
    leptos::task::tick().await;
    assert_eq!(focused_label(&host).as_deref(), Some("Paris"));
    sleep(650).await;
    key_with(&list(&host), "KeyI", "i", false, false);
    leptos::task::tick().await;
    assert_eq!(focused_label(&host).as_deref(), Some("Istanbul"));
    key_with(&list(&host), "KeyS", "s", false, false);
    leptos::task::tick().await;
    assert_eq!(focused_label(&host).as_deref(), Some("Istanbul"), "накопленная строка «is»");
    host.remove();
}

#[wasm_bindgen_test]
async fn select_on_focus_selects_while_navigating() {
    let value = RwSignal::new(None::<String>);
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Listbox options=cities() value=value select_on_focus=true /> });
    leptos::task::tick().await;
    focus_list(&host);
    press(&host, "ArrowDown");
    leptos::task::tick().await;
    assert_eq!(value.get_untracked().as_deref(), Some("LDN"));
    host.remove();
}

#[wasm_bindgen_test]
async fn hover_moves_focus_only_while_the_list_is_focused() {
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Listbox options=cities() /> });
    leptos::task::tick().await;
    opt(&host, 2).dispatch_event(&Event::new("mouseenter").unwrap()).unwrap();
    leptos::task::tick().await;
    assert_eq!(focused_label(&host), None, "список без фокуса");
    focus_list(&host);
    opt(&host, 3).dispatch_event(&Event::new("mouseenter").unwrap()).unwrap();
    leptos::task::tick().await;
    assert_eq!(focused_label(&host).as_deref(), Some("Istanbul"));
    host.remove();
}

#[wasm_bindgen_test]
async fn filter_narrows_the_list_reports_and_shows_the_empty_message() {
    let filters = Arc::new(Mutex::new(Vec::<String>::new()));
    let f = filters.clone();
    let on_filter = Callback::new(move |e: ListboxFilter| f.lock().unwrap().push(e.filter));
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <Listbox options=cities() filter=true empty_filter_message="No results" on_filter=on_filter /> }
    });
    leptos::task::tick().await;
    assert_eq!(count(&host, ".p-listbox-option"), 5);
    let input: HtmlInputElement = q(&host, ".p-listbox-filter").dyn_into().unwrap();
    input.set_value("lon");
    input.dispatch_event(&Event::new("input").unwrap()).unwrap();
    leptos::task::tick().await;
    assert_eq!(count(&host, ".p-listbox-option"), 1);
    assert_eq!(opt(&host, 0).get_attribute("aria-label").as_deref(), Some("London"));
    input.set_value("zzz");
    input.dispatch_event(&Event::new("input").unwrap()).unwrap();
    leptos::task::tick().await;
    assert!(!has(&host, ".p-listbox-list"));
    assert_eq!(q(&host, ".p-listbox-empty-message").text_content().as_deref(), Some("No results"));
    assert_eq!(*filters.lock().unwrap(), ["lon", "zzz"]);
    host.remove();
}

#[wasm_bindgen_test]
async fn filter_arrows_move_the_option_focus_and_enter_selects() {
    let value = RwSignal::new(None::<String>);
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Listbox id="fl" options=cities() value=value filter=true /> });
    leptos::task::tick().await;
    let input = q(&host, ".p-listbox-filter");
    input.focus().unwrap();
    key_with(&input, "ArrowDown", "ArrowDown", false, false);
    leptos::task::tick().await;
    assert_eq!(focused_label(&host).as_deref(), Some("New York"));
    assert_eq!(input.get_attribute("aria-activedescendant").as_deref(), Some("fl_0"));
    key_with(&input, "Enter", "Enter", false, false);
    leptos::task::tick().await;
    assert_eq!(value.get_untracked().as_deref(), Some("NY"));
    key_with(&input, "ArrowLeft", "ArrowLeft", false, false);
    leptos::task::tick().await;
    assert_eq!(focused_label(&host), None, "стрелка влево сбрасывает вариант");
    host.remove();
}

#[wasm_bindgen_test]
async fn checkbox_mode_toggles_options_and_select_all_respects_the_filter() {
    let (log, on_change) = change_log();
    let all = Arc::new(Mutex::new(Vec::<bool>::new()));
    let a = all.clone();
    let on_all = Callback::new(move |e: ListboxSelectAll| a.lock().unwrap().push(e.checked));
    let values = RwSignal::new(Vec::<String>::new());
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <Listbox options=cities() multiple=true checkbox=true filter=true values=values on_change=on_change on_select_all_change=on_all /> }
    });
    leptos::task::tick().await;
    // клик по флажку варианта выбирает его
    let first_box: HtmlElement = opt(&host, 0).query_selector(".p-checkbox-input").unwrap().unwrap().dyn_into().unwrap();
    first_box.click();
    leptos::task::tick().await;
    assert_eq!(values.get_untracked(), ["NY"]);
    assert!(opt(&host, 0).query_selector(".p-checkbox-checked").unwrap().is_some());
    let input_checked = first_box.dyn_into::<HtmlInputElement>().unwrap().checked();
    assert!(input_checked, "поле флажка отмечено");
    // «выбрать все» при фильтре «n» — только видимые доступные
    let filter: HtmlInputElement = q(&host, ".p-listbox-filter").dyn_into().unwrap();
    filter.set_value("n");
    filter.dispatch_event(&Event::new("input").unwrap()).unwrap();
    leptos::task::tick().await;
    let toggle: HtmlElement = q(&host, ".p-listbox-header .p-checkbox-input");
    toggle.click();
    leptos::task::tick().await;
    assert_eq!(values.get_untracked(), ["NY", "LDN", "IST"]);
    assert_eq!(*all.lock().unwrap(), [true]);
    assert!(q(&host, ".p-listbox-header .p-checkbox").class_name().contains("p-checkbox-checked"));
    toggle.click();
    leptos::task::tick().await;
    assert!(values.get_untracked().is_empty(), "повторный клик снимает выбор");
    assert_eq!(*all.lock().unwrap(), [true, false]);
    assert_eq!(log.lock().unwrap().len(), 3);
    host.remove();
}

#[wasm_bindgen_test]
async fn click_and_double_click_events_carry_the_option() {
    let clicks = Arc::new(Mutex::new(Vec::<String>::new()));
    let (a, b) = (clicks.clone(), clicks.clone());
    let on_click = Callback::new(move |e: ListboxEvent| a.lock().unwrap().push(format!("click:{}", e.option.value)));
    let on_dbl = Callback::new(move |e: ListboxEvent| b.lock().unwrap().push(format!("dbl:{}", e.option.value)));
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <Listbox options=cities() on_click=on_click on_dbl_click=on_dbl /> }
    });
    leptos::task::tick().await;
    opt(&host, 2).click();
    opt(&host, 2).dispatch_event(&Event::new("dblclick").unwrap()).unwrap();
    opt(&host, 1).click();
    opt(&host, 1).dispatch_event(&Event::new("dblclick").unwrap()).unwrap();
    leptos::task::tick().await;
    assert_eq!(*clicks.lock().unwrap(), ["click:LDN", "dbl:LDN"], "недоступный вариант событий не вызывает");
    host.remove();
}

#[wasm_bindgen_test]
async fn focus_and_blur_callbacks_fire() {
    let events = Arc::new(Mutex::new(Vec::<&'static str>::new()));
    let (a, b) = (events.clone(), events.clone());
    let on_focus = Callback::new(move |_: FocusEvent| a.lock().unwrap().push("focus"));
    let on_blur = Callback::new(move |_: FocusEvent| b.lock().unwrap().push("blur"));
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Listbox options=cities() on_focus=on_focus on_blur=on_blur /> });
    leptos::task::tick().await;
    focus_list(&host);
    list(&host).blur().unwrap();
    leptos::task::tick().await;
    assert_eq!(*events.lock().unwrap(), ["focus", "blur"]);
    host.remove();
}

#[wasm_bindgen_test]
async fn groups_navigation_skips_headers_and_selection_works() {
    let value = RwSignal::new(None::<String>);
    let groups = vec![
        ListboxGroup::new("Germany", vec![ListboxOption::new("Berlin", "B")]),
        ListboxGroup::new("Japan", vec![ListboxOption::new("Kyoto", "K")]),
    ];
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Listbox groups=groups.clone() value=value /> });
    leptos::task::tick().await;
    assert_eq!(count(&host, ".p-listbox-option-group"), 2);
    focus_list(&host);
    press(&host, "ArrowDown");
    leptos::task::tick().await;
    assert_eq!(focused_label(&host).as_deref(), Some("Kyoto"));
    press(&host, "Enter");
    leptos::task::tick().await;
    assert_eq!(value.get_untracked().as_deref(), Some("K"));
    host.remove();
}
