//! Поведение MultiSelect в реальном браузере (wasm-bindgen-test): открытие и закрытие, выбор без закрытия, предел, «выбрать все»,
//! чипы, очистка, клавиши (диапазоны, `Ctrl+A`, `Alt+↑`, `Tab`), поиск набором и полем, блокировки, внешние сигналы, положение.
#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use prime_multiselect::{
    MultiSelect, MultiSelectChange, MultiSelectDisplay, MultiSelectFilter, MultiSelectOption, MultiSelectRemove, MultiSelectSelectAll,
};
use std::sync::{Arc, Mutex};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;
use web_sys::{Event, HtmlElement, HtmlInputElement, KeyboardEvent, KeyboardEventInit};

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

fn count(host: &HtmlElement, sel: &str) -> u32 {
    host.query_selector_all(sel).unwrap().length()
}

/// Клик по SVG-элементу (у него нет `click()`).
fn click_svg(host: &HtmlElement, sel: &str) {
    let el = host.query_selector(sel).unwrap().unwrap_or_else(|| panic!("нет {sel}"));
    let init = web_sys::MouseEventInit::new();
    init.set_bubbles(true);
    init.set_cancelable(true);
    el.dispatch_event(&web_sys::MouseEvent::new_with_mouse_event_init_dict("click", &init).unwrap()).unwrap();
}

fn input(host: &HtmlElement) -> HtmlElement {
    q(host, ".p-hidden-accessible input")
}

fn opt(host: &HtmlElement, i: u32) -> HtmlElement {
    host.query_selector_all(".p-multiselect-option").unwrap().item(i).unwrap_or_else(|| panic!("нет варианта {i}")).dyn_into().unwrap()
}

fn label_text(host: &HtmlElement) -> String {
    q(host, ".p-multiselect-label").text_content().unwrap_or_default()
}

fn focused_label(host: &HtmlElement) -> Option<String> {
    host.query_selector(".p-multiselect-option.p-focus").unwrap().and_then(|o| o.get_attribute("aria-label"))
}

fn key_with(el: &HtmlElement, code: &str, key: &str, shift: bool, ctrl: bool, alt: bool) {
    let init = KeyboardEventInit::new();
    init.set_code(code);
    init.set_key(key);
    init.set_shift_key(shift);
    init.set_ctrl_key(ctrl);
    init.set_alt_key(alt);
    init.set_bubbles(true);
    init.set_cancelable(true);
    el.dispatch_event(&KeyboardEvent::new_with_keyboard_event_init_dict("keydown", &init).unwrap()).unwrap();
}

fn press(host: &HtmlElement, code: &str) {
    key_with(&input(host), code, code, false, false, false);
}

fn active_is(el: &HtmlElement) -> bool {
    el.is_same_node(doc().active_element().as_deref())
}

async fn sleep(ms: i32) {
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms).unwrap();
    });
    JsFuture::from(promise).await.unwrap();
}

fn cities() -> Vec<MultiSelectOption> {
    vec![
        MultiSelectOption::new("New York", "NY"),
        MultiSelectOption::new("Rome", "RM").disabled(true),
        MultiSelectOption::new("London", "LDN"),
        MultiSelectOption::new("Istanbul", "IST"),
        MultiSelectOption::new("Paris", "PRS"),
    ]
}

fn plain() -> Vec<MultiSelectOption> {
    vec![
        MultiSelectOption::new("New York", "NY"),
        MultiSelectOption::new("Rome", "RM"),
        MultiSelectOption::new("London", "LDN"),
        MultiSelectOption::new("Istanbul", "IST"),
        MultiSelectOption::new("Paris", "PRS"),
    ]
}

type Log = Arc<Mutex<Vec<String>>>;

fn change_log() -> (Log, Callback<MultiSelectChange>) {
    let log: Log = Arc::new(Mutex::new(Vec::new()));
    let l = log.clone();
    (log, Callback::new(move |e: MultiSelectChange| l.lock().unwrap().push(e.values.join(","))))
}

#[wasm_bindgen_test]
async fn click_opens_and_closes_the_panel_and_reports_show_and_hide() {
    let events = Arc::new(Mutex::new(Vec::<&'static str>::new()));
    let (a, b) = (events.clone(), events.clone());
    let (on_show, on_hide) =
        (Callback::new(move |()| a.lock().unwrap().push("show")), Callback::new(move |()| b.lock().unwrap().push("hide")));
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <MultiSelect options=cities() on_show=on_show on_hide=on_hide /> });
    leptos::task::tick().await;
    let root = q(&host, ".p-multiselect");
    assert!(!has(&host, ".p-multiselect-overlay"));
    root.click();
    leptos::task::tick().await;
    assert!(has(&host, ".p-multiselect-overlay"));
    assert_eq!(input(&host).get_attribute("aria-expanded").as_deref(), Some("true"));
    assert!(root.class_name().contains("p-multiselect-open"));
    assert!(active_is(&input(&host)), "после клика фокус на скрытом поле");
    root.click();
    leptos::task::tick().await;
    assert!(!has(&host, ".p-multiselect-overlay"));
    assert_eq!(*events.lock().unwrap(), ["show", "hide"]);
    host.remove();
}

#[wasm_bindgen_test]
async fn choosing_options_keeps_the_panel_open_and_updates_label_and_model() {
    let (log, on_change) = change_log();
    let values = RwSignal::new(Vec::<String>::new());
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <MultiSelect options=cities() values=values on_change=on_change placeholder="Select Cities" /> }
    });
    leptos::task::tick().await;
    assert_eq!(label_text(&host), "Select Cities");
    q(&host, ".p-multiselect").click();
    leptos::task::tick().await;
    opt(&host, 0).click();
    opt(&host, 2).click();
    leptos::task::tick().await;
    assert!(has(&host, ".p-multiselect-overlay"), "панель осталась открытой");
    assert_eq!(values.get_untracked(), ["NY", "LDN"]);
    assert_eq!(label_text(&host), "New York, London");
    assert_eq!(opt(&host, 0).get_attribute("aria-selected").as_deref(), Some("true"));
    assert!(opt(&host, 0).class_name().contains("p-multiselect-option-selected"));
    assert!(opt(&host, 0).query_selector(".p-checkbox-checked").unwrap().is_some());
    opt(&host, 0).click();
    leptos::task::tick().await;
    assert_eq!(values.get_untracked(), ["LDN"]);
    assert_eq!(*log.lock().unwrap(), ["NY", "NY,LDN", "LDN"]);
    opt(&host, 1).click();
    leptos::task::tick().await;
    assert_eq!(values.get_untracked(), ["LDN"], "недоступный вариант");
    host.remove();
}

#[wasm_bindgen_test]
async fn outside_click_escape_and_resize_close_the_panel() {
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <MultiSelect options=cities() /> });
    leptos::task::tick().await;
    q(&host, ".p-multiselect").click();
    leptos::task::tick().await;
    doc().body().unwrap().click();
    leptos::task::tick().await;
    assert!(!has(&host, ".p-multiselect-overlay"), "клик снаружи не закрыл панель");
    q(&host, ".p-multiselect").click();
    leptos::task::tick().await;
    press(&host, "Escape");
    leptos::task::tick().await;
    assert!(!has(&host, ".p-multiselect-overlay"));
    assert!(active_is(&input(&host)), "после Escape фокус на поле");
    q(&host, ".p-multiselect").click();
    leptos::task::tick().await;
    web_sys::window().unwrap().dispatch_event(&Event::new("resize").unwrap()).unwrap();
    leptos::task::tick().await;
    assert!(!has(&host, ".p-multiselect-overlay"), "resize не закрыл панель");
    host.remove();
}

#[wasm_bindgen_test]
async fn click_inside_the_panel_does_not_close_it() {
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <MultiSelect options=cities() /> });
    leptos::task::tick().await;
    q(&host, ".p-multiselect").click();
    leptos::task::tick().await;
    q(&host, ".p-multiselect-header").click();
    q(&host, ".p-multiselect-list-container").click();
    leptos::task::tick().await;
    assert!(has(&host, ".p-multiselect-overlay"));
    host.remove();
}

#[wasm_bindgen_test]
async fn panel_gets_a_z_index_and_position_while_open() {
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <MultiSelect options=cities() /> });
    leptos::task::tick().await;
    q(&host, ".p-multiselect").click();
    leptos::task::tick().await;
    let overlay = q(&host, ".p-multiselect-overlay");
    let z: i32 = overlay.style().get_property_value("z-index").unwrap().parse().unwrap();
    assert!(z >= 1000, "z-index группы оверлеев: {z}");
    assert!(!overlay.style().get_property_value("top").unwrap().is_empty(), "положение не рассчитано");
    host.remove();
}

#[wasm_bindgen_test]
async fn disabled_readonly_and_loading_do_not_open_and_ignore_clicks() {
    let host = host();
    let disabled = RwSignal::new(true);
    let readonly = RwSignal::new(false);
    let loading = RwSignal::new(false);
    let values = RwSignal::new(Vec::<String>::new());
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <MultiSelect options=cities() values=values disabled=disabled readonly=readonly loading=loading /> }
    });
    leptos::task::tick().await;
    q(&host, ".p-multiselect").click();
    leptos::task::tick().await;
    assert!(!has(&host, ".p-multiselect-overlay"), "disabled");
    assert_eq!(input(&host).get_attribute("tabindex").as_deref(), Some("-1"));
    disabled.set(false);
    readonly.set(true);
    q(&host, ".p-multiselect").click();
    leptos::task::tick().await;
    assert!(!has(&host, ".p-multiselect-overlay"), "readonly");
    readonly.set(false);
    loading.set(true);
    q(&host, ".p-multiselect").click();
    leptos::task::tick().await;
    assert!(!has(&host, ".p-multiselect-overlay"), "loading");
    host.remove();
}

#[wasm_bindgen_test]
async fn selection_limit_blocks_unselected_options_and_removing_frees_them() {
    let values = RwSignal::new(Vec::<String>::new());
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <MultiSelect options=plain() values=values selection_limit=2 /> });
    leptos::task::tick().await;
    q(&host, ".p-multiselect").click();
    leptos::task::tick().await;
    assert!(!has(&host, ".p-multiselect-header .p-checkbox"), "флажка «выбрать все» нет");
    opt(&host, 0).click();
    opt(&host, 1).click();
    opt(&host, 2).click();
    leptos::task::tick().await;
    assert_eq!(values.get_untracked(), ["NY", "RM"], "предел достигнут");
    assert!(opt(&host, 2).class_name().contains("p-disabled"));
    assert!(!opt(&host, 0).class_name().contains("p-disabled"), "выбранный остаётся доступным");
    opt(&host, 0).click();
    leptos::task::tick().await;
    assert_eq!(values.get_untracked(), ["RM"]);
    assert!(!opt(&host, 2).class_name().contains("p-disabled"));
    host.remove();
}

#[wasm_bindgen_test]
async fn toggle_all_selects_visible_enabled_options_with_the_filter() {
    let all = Arc::new(Mutex::new(Vec::<bool>::new()));
    let a = all.clone();
    let on_all = Callback::new(move |e: MultiSelectSelectAll| a.lock().unwrap().push(e.checked));
    let (log, on_change) = change_log();
    let values = RwSignal::new(Vec::<String>::new());
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <MultiSelect options=cities() values=values on_change=on_change on_select_all_change=on_all /> }
    });
    leptos::task::tick().await;
    q(&host, ".p-multiselect").click();
    leptos::task::tick().await;
    let toggle = q(&host, ".p-multiselect-header .p-checkbox-input");
    toggle.click();
    leptos::task::tick().await;
    assert_eq!(values.get_untracked(), ["NY", "LDN", "IST", "PRS"], "недоступный Rome пропущен");
    assert!(q(&host, ".p-multiselect-header .p-checkbox").class_name().contains("p-checkbox-checked"));
    toggle.click();
    leptos::task::tick().await;
    assert!(values.get_untracked().is_empty());
    // с поиском «n»: выбираются только NY, LDN, IST; прежний выбор вне поиска (PRS) сохраняется
    values.set(vec!["PRS".to_owned()]);
    let filter: HtmlInputElement = q(&host, ".p-multiselect-filter").dyn_into().unwrap();
    filter.set_value("n");
    filter.dispatch_event(&Event::new("input").unwrap()).unwrap();
    leptos::task::tick().await;
    toggle.click();
    leptos::task::tick().await;
    assert_eq!(values.get_untracked(), ["PRS", "NY", "LDN", "IST"]);
    assert_eq!(log.lock().unwrap().len(), 3);
    assert_eq!(*all.lock().unwrap(), [false], "как в PrimeNG: только пустой итог или итог из всех вариантов (Rome недоступен)");
    host.remove();
}

#[wasm_bindgen_test]
async fn chips_render_and_remove_without_opening_the_panel() {
    let removed = Arc::new(Mutex::new(Vec::<String>::new()));
    let r = removed.clone();
    let on_remove = Callback::new(move |e: MultiSelectRemove| r.lock().unwrap().push(format!("{}->{}", e.removed, e.values.join(","))));
    let (log, on_change) = change_log();
    let values = RwSignal::new(vec!["NY".to_owned(), "LDN".to_owned()]);
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! {
            <MultiSelect options=cities() values=values display=MultiSelectDisplay::Chip on_remove=on_remove on_change=on_change />
        }
    });
    leptos::task::tick().await;
    assert_eq!(count(&host, ".p-multiselect-chip-item"), 2);
    assert!(q(&host, ".p-multiselect").class_name().contains("p-multiselect-display-chip"));
    let remove: HtmlElement = host.query_selector_all(".p-chip-remove-icon").unwrap().item(0).unwrap().dyn_into().unwrap();
    remove.click();
    leptos::task::tick().await;
    assert_eq!(values.get_untracked(), ["LDN"]);
    assert_eq!(count(&host, ".p-multiselect-chip-item"), 1);
    assert!(!has(&host, ".p-multiselect-overlay"), "удаление чипа не открывает панель");
    assert_eq!(*removed.lock().unwrap(), ["NY->LDN"]);
    assert_eq!(*log.lock().unwrap(), ["LDN"]);
    values.set(vec!["NY".to_owned(), "LDN".to_owned(), "IST".to_owned()]);
    leptos::task::tick().await;
    assert_eq!(label_text(&host), "3 items selected", "как в PrimeNG: len == max");
    host.remove();
}

#[wasm_bindgen_test]
async fn clear_icon_resets_the_values_without_opening() {
    let cleared = Arc::new(Mutex::new(0));
    let c = cleared.clone();
    let on_clear = Callback::new(move |()| *c.lock().unwrap() += 1);
    let (log, on_change) = change_log();
    let values = RwSignal::new(vec!["NY".to_owned()]);
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <MultiSelect options=cities() values=values show_clear=true on_clear=on_clear on_change=on_change /> }
    });
    leptos::task::tick().await;
    click_svg(&host, ".p-multiselect-clear-icon");
    leptos::task::tick().await;
    assert!(values.get_untracked().is_empty());
    assert!(!has(&host, ".p-multiselect-overlay"));
    assert!(!has(&host, ".p-multiselect-clear-icon"), "значок пропал вместе с выбором");
    assert_eq!((*cleared.lock().unwrap(), log.lock().unwrap().len()), (1, 1));
    host.remove();
}

#[wasm_bindgen_test]
async fn arrow_keys_open_the_panel_navigate_and_enter_toggles() {
    let values = RwSignal::new(Vec::<String>::new());
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <MultiSelect id="kb" options=cities() values=values /> });
    leptos::task::tick().await;
    input(&host).focus().unwrap();
    press(&host, "ArrowDown");
    leptos::task::tick().await;
    assert!(has(&host, ".p-multiselect-overlay"), "стрелка вниз открывает панель");
    assert_eq!(focused_label(&host).as_deref(), Some("New York"));
    assert_eq!(input(&host).get_attribute("aria-activedescendant").as_deref(), Some("kb_0"));
    press(&host, "ArrowDown");
    leptos::task::tick().await;
    assert_eq!(focused_label(&host).as_deref(), Some("London"), "Rome пропущен");
    press(&host, "Enter");
    leptos::task::tick().await;
    assert_eq!(values.get_untracked(), ["LDN"]);
    assert!(has(&host, ".p-multiselect-overlay"), "Enter не закрывает панель");
    press(&host, "Space");
    leptos::task::tick().await;
    assert!(values.get_untracked().is_empty(), "Space снимает выбор");
    press(&host, "End");
    leptos::task::tick().await;
    assert_eq!(focused_label(&host).as_deref(), Some("Paris"));
    press(&host, "Home");
    leptos::task::tick().await;
    assert_eq!(focused_label(&host).as_deref(), Some("New York"));
    host.remove();
}

#[wasm_bindgen_test]
async fn enter_on_a_closed_panel_opens_it() {
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <MultiSelect options=cities() /> });
    leptos::task::tick().await;
    input(&host).focus().unwrap();
    press(&host, "Enter");
    leptos::task::tick().await;
    assert!(has(&host, ".p-multiselect-overlay"));
    host.remove();
}

#[wasm_bindgen_test]
async fn shift_arrows_ctrl_a_and_alt_up_work_on_selection() {
    let values = RwSignal::new(Vec::<String>::new());
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <MultiSelect options=plain() values=values filter=false /> });
    leptos::task::tick().await;
    input(&host).focus().unwrap();
    press(&host, "ArrowDown");
    leptos::task::tick().await;
    key_with(&input(&host), "ShiftLeft", "Shift", true, false, false);
    key_with(&input(&host), "ArrowDown", "ArrowDown", true, false, false);
    key_with(&input(&host), "ArrowDown", "ArrowDown", true, false, false);
    leptos::task::tick().await;
    assert_eq!(values.get_untracked(), ["NY", "RM", "LDN"], "диапазон от опорного");
    key_with(&input(&host), "KeyA", "a", false, true, false);
    leptos::task::tick().await;
    assert_eq!(values.get_untracked(), ["NY", "RM", "LDN", "IST", "PRS"]);
    values.set(Vec::new());
    key_with(&input(&host), "ArrowUp", "ArrowUp", false, false, true);
    leptos::task::tick().await;
    assert!(!has(&host, ".p-multiselect-overlay"), "Alt+↑ закрывает панель");
    assert_eq!(values.get_untracked(), ["LDN"], "и выбирает вариант под фокусом");
    host.remove();
}

#[wasm_bindgen_test]
async fn tab_selects_the_focused_option_and_closes_without_a_filter() {
    let values = RwSignal::new(Vec::<String>::new());
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <MultiSelect options=plain() values=values filter=false /> });
    leptos::task::tick().await;
    input(&host).focus().unwrap();
    press(&host, "ArrowDown");
    leptos::task::tick().await;
    press(&host, "Tab");
    leptos::task::tick().await;
    assert_eq!(values.get_untracked(), ["NY"]);
    assert!(!has(&host, ".p-multiselect-overlay"));
    host.remove();
}

#[wasm_bindgen_test]
async fn typing_opens_the_panel_and_moves_focus_to_a_matching_option() {
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <MultiSelect options=cities() /> });
    leptos::task::tick().await;
    input(&host).focus().unwrap();
    key_with(&input(&host), "KeyP", "p", false, false, false);
    leptos::task::tick().await;
    assert!(has(&host, ".p-multiselect-overlay"));
    assert_eq!(focused_label(&host).as_deref(), Some("Paris"));
    sleep(650).await;
    key_with(&input(&host), "KeyI", "i", false, false, false);
    leptos::task::tick().await;
    assert_eq!(focused_label(&host).as_deref(), Some("Istanbul"));
    host.remove();
}

#[wasm_bindgen_test]
async fn filter_narrows_the_list_reports_and_resets_on_hide() {
    let filters = Arc::new(Mutex::new(Vec::<String>::new()));
    let f = filters.clone();
    let on_filter = Callback::new(move |e: MultiSelectFilter| f.lock().unwrap().push(e.filter));
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <MultiSelect options=cities() empty_filter_message="No results" on_filter=on_filter reset_filter_on_hide=true /> }
    });
    leptos::task::tick().await;
    q(&host, ".p-multiselect").click();
    leptos::task::tick().await;
    assert_eq!(count(&host, ".p-multiselect-option"), 5);
    let filter: HtmlInputElement = q(&host, ".p-multiselect-filter").dyn_into().unwrap();
    filter.set_value("lon");
    filter.dispatch_event(&Event::new("input").unwrap()).unwrap();
    leptos::task::tick().await;
    assert_eq!(count(&host, ".p-multiselect-option"), 1);
    filter.set_value("zzz");
    filter.dispatch_event(&Event::new("input").unwrap()).unwrap();
    leptos::task::tick().await;
    assert_eq!(q(&host, ".p-multiselect-empty-message").text_content().as_deref(), Some("No results"));
    assert_eq!(*filters.lock().unwrap(), ["lon", "zzz"]);
    press(&host, "Escape");
    leptos::task::tick().await;
    q(&host, ".p-multiselect").click();
    leptos::task::tick().await;
    assert_eq!(count(&host, ".p-multiselect-option"), 5, "строка поиска сброшена при закрытии");
    host.remove();
}

#[wasm_bindgen_test]
async fn filter_arrows_move_focus_and_enter_toggles() {
    let values = RwSignal::new(Vec::<String>::new());
    let host = host();
    let _h = leptos::mount::mount_to(
        host.clone(),
        move || view! { <MultiSelect id="fl" options=cities() values=values autofocus_filter=true /> },
    );
    leptos::task::tick().await;
    q(&host, ".p-multiselect").click();
    leptos::task::tick().await;
    let filter = q(&host, ".p-multiselect-filter");
    assert!(active_is(&filter) || has(&host, ".p-multiselect-filter"), "поле поиска есть");
    filter.focus().unwrap();
    key_with(&filter, "ArrowDown", "ArrowDown", false, false, false);
    leptos::task::tick().await;
    assert_eq!(focused_label(&host).as_deref(), Some("New York"));
    assert_eq!(filter.get_attribute("aria-activedescendant").as_deref(), Some("fl_0"));
    key_with(&filter, "Enter", "Enter", false, false, false);
    leptos::task::tick().await;
    assert_eq!(values.get_untracked(), ["NY"]);
    key_with(&filter, "ArrowLeft", "ArrowLeft", false, false, false);
    leptos::task::tick().await;
    assert_eq!(focused_label(&host), None);
    host.remove();
}

#[wasm_bindgen_test]
async fn external_open_and_values_signals_drive_the_component() {
    let open = RwSignal::new(false);
    let values = RwSignal::new(vec!["PRS".to_owned()]);
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <MultiSelect options=cities() values=values open=open /> });
    leptos::task::tick().await;
    assert_eq!(label_text(&host), "Paris");
    open.set(true);
    leptos::task::tick().await;
    assert!(has(&host, ".p-multiselect-overlay"));
    let overlay = q(&host, ".p-multiselect-overlay");
    assert!(!overlay.style().get_property_value("z-index").unwrap().is_empty(), "эффект открытия выполнен");
    values.set(vec!["NY".to_owned(), "IST".to_owned()]);
    leptos::task::tick().await;
    assert_eq!(label_text(&host), "New York, Istanbul");
    open.set(false);
    leptos::task::tick().await;
    assert!(!has(&host, ".p-multiselect-overlay"));
    host.remove();
}

#[wasm_bindgen_test]
async fn focus_and_hover_highlight_the_option() {
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <MultiSelect options=cities() /> });
    leptos::task::tick().await;
    q(&host, ".p-multiselect").click();
    leptos::task::tick().await;
    opt(&host, 3).dispatch_event(&Event::new("mouseenter").unwrap()).unwrap();
    leptos::task::tick().await;
    assert_eq!(focused_label(&host).as_deref(), Some("Istanbul"));
    assert_eq!(opt(&host, 3).get_attribute("data-p-focused").as_deref(), Some("true"));
    host.remove();
}
