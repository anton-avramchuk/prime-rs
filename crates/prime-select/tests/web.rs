//! Select behaviour in a real browser (wasm-bindgen-test): open and close, selection by mouse and key, skipping disabled,
//! type-ahead, search, groups, clear, locks, focus, external signals, panel position and z-index.
#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use prime_select::{Select, SelectChange, SelectGroup, SelectOption};
use std::sync::{Arc, Mutex};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;
use web_sys::{Event, FocusEvent, HtmlElement, HtmlInputElement, KeyboardEvent, KeyboardEventInit};

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
    host.query_selector(sel).unwrap().unwrap_or_else(|| panic!("no {sel}")).dyn_into().unwrap()
}

fn has(host: &HtmlElement, sel: &str) -> bool {
    host.query_selector(sel).unwrap().is_some()
}

fn label(host: &HtmlElement) -> HtmlElement {
    q(host, ".p-select-label")
}

fn li(host: &HtmlElement, i: u32) -> HtmlElement {
    host.query_selector_all(".p-select-option, .p-select-option-group")
        .unwrap()
        .item(i)
        .unwrap_or_else(|| panic!("no element {i}"))
        .dyn_into()
        .unwrap()
}

fn key(el: &HtmlElement, code: &str, key: &str, alt: bool) -> bool {
    let init = KeyboardEventInit::new();
    init.set_code(code);
    init.set_key(key);
    init.set_alt_key(alt);
    init.set_bubbles(true);
    init.set_cancelable(true);
    el.dispatch_event(&KeyboardEvent::new_with_keyboard_event_init_dict("keydown", &init).unwrap()).unwrap()
}

fn press(host: &HtmlElement, code: &str) {
    key(&label(host), code, code, false);
}

async fn sleep(ms: i32) {
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms).unwrap();
    });
    JsFuture::from(promise).await.unwrap();
}

fn cities() -> Vec<SelectOption> {
    vec![
        SelectOption::new("New York", "NY"),
        SelectOption::new("Rome", "RM").disabled(true),
        SelectOption::new("London", "LDN"),
        SelectOption::new("Istanbul", "IST"),
        SelectOption::new("Paris", "PRS"),
    ]
}

fn counter() -> (Arc<Mutex<Vec<String>>>, Callback<SelectChange>) {
    let log = Arc::new(Mutex::new(Vec::new()));
    let l = log.clone();
    (log, Callback::new(move |e: SelectChange| l.lock().unwrap().push(e.value.unwrap_or_else(|| "-".to_owned()))))
}

fn active_is(el: &HtmlElement) -> bool {
    el.is_same_node(doc().active_element().as_deref())
}

#[wasm_bindgen_test]
async fn click_opens_and_closes_the_panel_and_reports_show_and_hide() {
    let events = Arc::new(Mutex::new(Vec::<&'static str>::new()));
    let (a, b) = (events.clone(), events.clone());
    let (on_show, on_hide) =
        (Callback::new(move |()| a.lock().unwrap().push("show")), Callback::new(move |()| b.lock().unwrap().push("hide")));
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Select options=cities() on_show=on_show on_hide=on_hide /> });
    leptos::task::tick().await;
    let root = q(&host, ".p-select");
    assert!(!has(&host, ".p-select-overlay"));
    root.click();
    leptos::task::tick().await;
    assert!(has(&host, ".p-select-overlay"));
    assert_eq!(label(&host).get_attribute("aria-expanded").as_deref(), Some("true"));
    assert!(root.class_name().contains("p-select-open"));
    assert!(active_is(&label(&host)), "focus is on the label after the click");
    root.click();
    leptos::task::tick().await;
    assert!(!has(&host, ".p-select-overlay"));
    assert_eq!(label(&host).get_attribute("aria-expanded").as_deref(), Some("false"));
    assert_eq!(*events.lock().unwrap(), ["show", "hide"]);
    host.remove();
}

#[wasm_bindgen_test]
async fn panel_gets_a_z_index_and_position_while_open() {
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Select options=cities() /> });
    leptos::task::tick().await;
    q(&host, ".p-select").click();
    leptos::task::tick().await;
    let overlay = q(&host, ".p-select-overlay");
    let z: i32 = overlay.style().get_property_value("z-index").unwrap().parse().unwrap();
    assert!(z >= 1000, "overlay group z-index: {z}");
    assert!(!overlay.style().get_property_value("top").unwrap().is_empty(), "position not computed");
    assert!(!overlay.style().get_property_value("min-width").unwrap().is_empty());
    host.remove();
}

#[wasm_bindgen_test]
async fn outside_click_escape_and_tab_close_the_panel() {
    let value = RwSignal::new(None::<String>);
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Select options=cities() value=value /> });
    leptos::task::tick().await;
    q(&host, ".p-select").click();
    leptos::task::tick().await;
    doc().body().unwrap().click();
    leptos::task::tick().await;
    assert!(!has(&host, ".p-select-overlay"), "click outside did not close the panel");
    // Escape returns focus to the label and does not change the value
    q(&host, ".p-select").click();
    leptos::task::tick().await;
    press(&host, "Escape");
    leptos::task::tick().await;
    assert!(!has(&host, ".p-select-overlay") && value.get_untracked().is_none() && active_is(&label(&host)));
    // Tab selects the focused option and closes the panel
    q(&host, ".p-select").click();
    leptos::task::tick().await;
    li(&host, 2).dispatch_event(&Event::new("mouseenter").unwrap()).unwrap();
    press(&host, "Tab");
    leptos::task::tick().await;
    assert_eq!(value.get_untracked().as_deref(), Some("LDN"));
    assert!(!has(&host, ".p-select-overlay"));
    host.remove();
}

#[wasm_bindgen_test]
async fn clicking_an_option_selects_it_once_and_skips_disabled_ones() {
    let value = RwSignal::new(None::<String>);
    let (log, on_change) = counter();
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Select options=cities() value=value on_change=on_change /> });
    leptos::task::tick().await;
    q(&host, ".p-select").click();
    leptos::task::tick().await;
    li(&host, 1).click();
    leptos::task::tick().await;
    assert_eq!(value.get_untracked(), None, "disabled option was selected");
    assert!(has(&host, ".p-select-overlay"), "the panel closed on a click on a disabled option");
    li(&host, 2).click();
    leptos::task::tick().await;
    assert_eq!(value.get_untracked().as_deref(), Some("LDN"));
    assert!(!has(&host, ".p-select-overlay"));
    assert_eq!(label(&host).text_content().unwrap(), "London");
    assert!(active_is(&label(&host)));
    // selecting the same option again only closes the panel
    q(&host, ".p-select").click();
    leptos::task::tick().await;
    li(&host, 2).click();
    leptos::task::tick().await;
    assert_eq!(*log.lock().unwrap(), ["LDN"]);
    host.remove();
}

#[wasm_bindgen_test]
async fn keyboard_navigation_selects_with_enter_and_skips_disabled() {
    let value = RwSignal::new(None::<String>);
    let (log, on_change) = counter();
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Select options=cities() value=value on_change=on_change /> });
    leptos::task::tick().await;
    label(&host).focus().unwrap();
    assert!(!key(&label(&host), "ArrowDown", "ArrowDown", false), "arrow was not cancelled");
    leptos::task::tick().await;
    assert!(has(&host, ".p-select-overlay"), "arrow down should open the panel");
    press(&host, "ArrowDown");
    leptos::task::tick().await;
    assert!(li(&host, 0).class_name().contains("p-focus"), "the first arrow focuses the first option");
    press(&host, "ArrowDown");
    leptos::task::tick().await;
    assert!(li(&host, 2).class_name().contains("p-focus"), "the disabled option should be skipped");
    let id = li(&host, 2).id();
    assert_eq!(label(&host).get_attribute("aria-activedescendant").as_deref(), Some(id.as_str()));
    press(&host, "ArrowUp");
    leptos::task::tick().await;
    assert!(li(&host, 0).class_name().contains("p-focus"));
    press(&host, "End");
    leptos::task::tick().await;
    assert!(li(&host, 4).class_name().contains("p-focus"));
    press(&host, "Home");
    leptos::task::tick().await;
    assert!(li(&host, 0).class_name().contains("p-focus"));
    press(&host, "ArrowDown");
    press(&host, "Enter");
    leptos::task::tick().await;
    assert_eq!(value.get_untracked().as_deref(), Some("LDN"));
    assert!(!has(&host, ".p-select-overlay") && active_is(&label(&host)));
    // Space opens, and with the panel open it selects; focus goes straight to the selected one
    press(&host, "Space");
    leptos::task::tick().await;
    assert!(has(&host, ".p-select-overlay") && li(&host, 2).class_name().contains("p-focus"), "on open focus is on the selected one");
    // Alt+ArrowUp selects the focused option and closes the panel
    press(&host, "ArrowDown");
    key(&label(&host), "ArrowUp", "ArrowUp", true);
    leptos::task::tick().await;
    assert!(!has(&host, ".p-select-overlay"));
    assert_eq!(*log.lock().unwrap(), ["LDN", "IST"]);
    host.remove();
}

#[wasm_bindgen_test]
async fn typing_characters_opens_the_panel_and_focuses_the_match() {
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Select options=cities() /> });
    leptos::task::tick().await;
    label(&host).focus().unwrap();
    key(&label(&host), "KeyL", "l", false);
    sleep(30).await;
    assert!(has(&host, ".p-select-overlay"));
    assert!(li(&host, 2).class_name().contains("p-focus"), "focus should move to London");
    key(&label(&host), "KeyO", "o", false);
    sleep(30).await;
    assert!(li(&host, 2).class_name().contains("p-focus"));
    // typing does not find "Lox", focus stays
    key(&label(&host), "KeyX", "x", false);
    sleep(30).await;
    assert!(li(&host, 2).class_name().contains("p-focus"));
    // after 500 ms the buffer resets: "p" searches anew
    sleep(600).await;
    key(&label(&host), "KeyP", "p", false);
    sleep(30).await;
    assert!(li(&host, 4).class_name().contains("p-focus"), "Paris");
    host.remove();
}

#[wasm_bindgen_test]
async fn filter_narrows_the_list_and_shows_the_empty_message() {
    let (log, on_filter) = {
        let log = Arc::new(Mutex::new(Vec::<String>::new()));
        let l = log.clone();
        (log, Callback::new(move |e: prime_select::SelectFilter| l.lock().unwrap().push(e.filter)))
    };
    let host = host();
    let _h = leptos::mount::mount_to(
        host.clone(),
        move || view! { <Select options=cities() filter=true on_filter=on_filter reset_filter_on_hide=true /> },
    );
    leptos::task::tick().await;
    q(&host, ".p-select").click();
    leptos::task::tick().await;
    let input: HtmlInputElement = q(&host, ".p-select-filter").dyn_into().unwrap();
    assert!(input.is_same_node(doc().active_element().as_deref()), "on open focus is in the search field");
    input.set_value("or");
    input.dispatch_event(&Event::new("input").unwrap()).unwrap();
    leptos::task::tick().await;
    assert_eq!(host.query_selector_all(".p-select-option").unwrap().length(), 1);
    assert!(has(&host, ".p-select-filter") && input.is_same_node(Some(&q(&host, ".p-select-filter"))), "search field was recreated");
    input.set_value("zzz");
    input.dispatch_event(&Event::new("input").unwrap()).unwrap();
    leptos::task::tick().await;
    assert_eq!(q(&host, ".p-select-empty-message").text_content().unwrap(), "No results found");
    assert_eq!(*log.lock().unwrap(), ["or", "zzz"]);
    // closing resets the string
    q(&host, ".p-select-label").focus().unwrap();
    press(&host, "Escape");
    leptos::task::tick().await;
    q(&host, ".p-select").click();
    leptos::task::tick().await;
    assert_eq!(host.query_selector_all(".p-select-option").unwrap().length(), 5);
    assert_eq!(q(&host, ".p-select-filter").dyn_into::<HtmlInputElement>().unwrap().value(), "");
    host.remove();
}

#[wasm_bindgen_test]
async fn groups_filter_and_keyboard_skip_headers() {
    let value = RwSignal::new(None::<String>);
    let groups = vec![
        SelectGroup::new("Germany", vec![SelectOption::new("Berlin", "B"), SelectOption::new("Frankfurt", "F")]),
        SelectGroup::new("USA", vec![SelectOption::new("Chicago", "C")]),
    ];
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Select groups=groups.clone() value=value filter=true /> });
    leptos::task::tick().await;
    q(&host, ".p-select").click();
    leptos::task::tick().await;
    assert_eq!(host.query_selector_all(".p-select-option-group").unwrap().length(), 2);
    let input: HtmlInputElement = q(&host, ".p-select-filter").dyn_into().unwrap();
    key(&input.clone().into(), "ArrowDown", "ArrowDown", false);
    leptos::task::tick().await;
    assert!(li(&host, 1).class_name().contains("p-focus"), "group header skipped");
    key(&input.clone().into(), "ArrowDown", "ArrowDown", false);
    key(&input.clone().into(), "ArrowDown", "ArrowDown", false);
    leptos::task::tick().await;
    assert!(li(&host, 4).class_name().contains("p-focus") && li(&host, 4).text_content().unwrap() == "Chicago", "USD header skipped");
    // Home and End in the search field move the cursor, not the option focus
    input.set_value("abc");
    key(&input.clone().into(), "Home", "Home", false);
    assert_eq!(input.selection_start().unwrap(), Some(0));
    leptos::task::tick().await;
    assert!(!li(&host, 4).class_name().contains("p-focus"), "Home in the search field resets option focus");
    // the filter keeps only the group with a match
    input.set_value("chi");
    input.dispatch_event(&Event::new("input").unwrap()).unwrap();
    leptos::task::tick().await;
    assert_eq!(host.query_selector_all(".p-select-option-group").unwrap().length(), 1);
    assert_eq!(host.query_selector_all(".p-select-option").unwrap().length(), 1);
    key(&input.clone().into(), "ArrowDown", "ArrowDown", false);
    key(&input.into(), "Enter", "Enter", false);
    leptos::task::tick().await;
    assert_eq!(value.get_untracked().as_deref(), Some("C"));
    host.remove();
}

#[wasm_bindgen_test]
async fn clear_icon_and_delete_key_reset_the_value() {
    let value = RwSignal::new(Some("NY".to_owned()));
    let cleared = Arc::new(Mutex::new(0));
    let c = cleared.clone();
    let on_clear = Callback::new(move |_: Event| *c.lock().unwrap() += 1);
    let (log, on_change) = counter();
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <Select options=cities() value=value show_clear=true placeholder="Pick" on_clear=on_clear on_change=on_change /> }
    });
    leptos::task::tick().await;
    let init = web_sys::MouseEventInit::new();
    init.set_bubbles(true);
    let click = web_sys::MouseEvent::new_with_mouse_event_init_dict("click", &init).unwrap();
    host.query_selector(".p-select-clear-icon").unwrap().unwrap().dispatch_event(&click).unwrap();
    leptos::task::tick().await;
    assert_eq!(value.get_untracked(), None);
    assert!(!has(&host, ".p-select-overlay"), "clear must not open the panel");
    assert!(label(&host).class_name().contains("p-placeholder") && !has(&host, ".p-select-clear-icon"));
    assert_eq!((*cleared.lock().unwrap(), log.lock().unwrap().clone()), (1, vec!["-".to_owned()]));
    value.set(Some("PRS".to_owned()));
    leptos::task::tick().await;
    label(&host).focus().unwrap();
    assert!(!key(&label(&host), "Delete", "Delete", false));
    leptos::task::tick().await;
    assert_eq!(value.get_untracked(), None);
    host.remove();
}

#[wasm_bindgen_test]
async fn disabled_readonly_and_loading_block_opening() {
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! {
            <div id="d"><Select options=cities() disabled=true /></div>
            <div id="r"><Select options=cities() readonly=true /></div>
            <div id="l"><Select options=cities() loading=true /></div>
        }
    });
    leptos::task::tick().await;
    for id in ["#d", "#r", "#l"] {
        q(&host, &format!("{id} .p-select")).click();
        key(&q(&host, &format!("{id} .p-select-label")), "ArrowDown", "ArrowDown", false);
        leptos::task::tick().await;
    }
    assert!(!has(&host, ".p-select-overlay"));
    assert!(has(&host, ".p-select-loading-icon"));
    host.remove();
}

#[wasm_bindgen_test]
async fn label_element_and_focus_survive_selection_and_navigation() {
    let value = RwSignal::new(None::<String>);
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Select options=cities() value=value /> });
    leptos::task::tick().await;
    let before = label(&host);
    before.focus().unwrap();
    press(&host, "ArrowDown");
    press(&host, "ArrowDown");
    press(&host, "Enter");
    leptos::task::tick().await;
    assert_eq!(value.get_untracked().as_deref(), Some("NY"));
    assert!(before.is_same_node(Some(&label(&host))), "label was recreated");
    assert!(active_is(&label(&host)));
    host.remove();
}

#[wasm_bindgen_test]
async fn external_value_open_signal_and_options_update_the_dom() {
    let value = RwSignal::new(None::<String>);
    let open = RwSignal::new(false);
    let options = RwSignal::new(cities());
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Select options=options value=value open=open placeholder="Pick" /> });
    leptos::task::tick().await;
    assert_eq!(label(&host).text_content().unwrap(), "Pick");
    value.set(Some("PRS".to_owned()));
    leptos::task::tick().await;
    assert_eq!(label(&host).text_content().unwrap(), "Paris");
    open.set(true);
    leptos::task::tick().await;
    assert!(has(&host, ".p-select-overlay"));
    assert!(li(&host, 4).class_name().contains("p-select-option-selected"));
    options.update(|o| o.push(SelectOption::new("Oslo", "OSL")));
    leptos::task::tick().await;
    assert_eq!(host.query_selector_all(".p-select-option").unwrap().length(), 6);
    assert!(has(&host, ".p-select-overlay"), "the panel closed when options changed");
    open.set(false);
    leptos::task::tick().await;
    assert!(!has(&host, ".p-select-overlay"));
    // a value outside the list: the hint again
    value.set(Some("zzz".to_owned()));
    leptos::task::tick().await;
    assert_eq!(label(&host).text_content().unwrap(), "Pick");
    host.remove();
}

#[wasm_bindgen_test]
async fn hover_moves_focus_and_select_on_focus_selects_without_closing() {
    let value = RwSignal::new(None::<String>);
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Select options=cities() value=value select_on_focus=true /> });
    leptos::task::tick().await;
    q(&host, ".p-select").click();
    leptos::task::tick().await;
    li(&host, 3).dispatch_event(&Event::new("mouseenter").unwrap()).unwrap();
    leptos::task::tick().await;
    assert!(li(&host, 3).class_name().contains("p-focus"));
    assert_eq!(value.get_untracked().as_deref(), Some("IST"));
    assert!(has(&host, ".p-select-overlay"), "select_on_focus must not close the panel");
    host.remove();
}

#[wasm_bindgen_test]
async fn last_hidden_boundary_closes_the_panel_and_returns_focus() {
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Select options=cities() filter=true /> });
    leptos::task::tick().await;
    q(&host, ".p-select").click();
    leptos::task::tick().await;
    let hidden = host.query_selector_all(".p-hidden-focusable").unwrap();
    let last: HtmlElement = hidden.item(hidden.length() - 1).unwrap().dyn_into().unwrap();
    last.dispatch_event(&FocusEvent::new("focus").unwrap()).unwrap();
    leptos::task::tick().await;
    assert!(!has(&host, ".p-select-overlay") && active_is(&label(&host)));
    host.remove();
}
