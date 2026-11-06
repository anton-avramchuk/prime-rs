//! Поведение Table в реальном браузере (wasm-bindgen-test): сортировка кликом и клавишей, многоуровневая сортировка,
//! выбор строк кликом, флажки и радиокнопки, флажок в заголовке, раскрытие, пагинатор. Сценарии идут одним последовательным тестом.
#![cfg(target_arch = "wasm32")]
#![recursion_limit = "512"]

use leptos::prelude::*;
use prime_table::model::{SelectionMode, SortMeta, SortMode};
use prime_table::{Column, Table};
use std::sync::{Arc, Mutex};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;
use web_sys::{HtmlElement, KeyboardEvent, KeyboardEventInit, MouseEvent, MouseEventInit};

wasm_bindgen_test_configure!(run_in_browser);

#[derive(Clone, Debug)]
struct Product {
    code: &'static str,
    name: &'static str,
    quantity: u32,
}

fn products() -> Vec<Product> {
    vec![
        Product { code: "c1", name: "Watch", quantity: 24 },
        Product { code: "c2", name: "Band", quantity: 61 },
        Product { code: "c3", name: "Shirt", quantity: 2 },
        Product { code: "c4", name: "Purse", quantity: 61 },
        Product { code: "c5", name: "Ring", quantity: 7 },
    ]
}

fn key() -> Callback<Product, String> {
    Callback::new(|p: Product| p.code.to_owned())
}

fn sortable_columns() -> Vec<Column<Product>> {
    vec![
        Column::text("code", "Code", |p: &Product| p.code.to_owned()).sortable(|a, b| a.code.cmp(b.code)),
        Column::text("name", "Name", |p: &Product| p.name.to_owned()).sortable(|a, b| a.name.cmp(b.name)),
        Column::text("quantity", "Quantity", |p: &Product| p.quantity.to_string()).sortable(|a, b| a.quantity.cmp(&b.quantity)),
    ]
}

fn plain_columns() -> Vec<Column<Product>> {
    vec![Column::text("code", "Code", |p: &Product| p.code.to_owned()), Column::text("name", "Name", |p: &Product| p.name.to_owned())]
}

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

fn names(host: &HtmlElement) -> String {
    all(host, "tbody > tr:not(.p-datatable-row-expansion) > td:nth-child(2)")
        .iter()
        .map(|e| e.text_content().unwrap_or_default())
        .collect::<Vec<_>>()
        .join(" ")
}

fn header(host: &HtmlElement, index: usize) -> HtmlElement {
    all(host, "thead th")[index].clone()
}

fn click_with(el: &HtmlElement, ctrl: bool) {
    let init = MouseEventInit::new();
    init.set_bubbles(true);
    init.set_cancelable(true);
    init.set_ctrl_key(ctrl);
    el.dispatch_event(&MouseEvent::new_with_mouse_event_init_dict("click", &init).unwrap()).unwrap();
}

fn key_down(el: &HtmlElement, key: &str) {
    let init = KeyboardEventInit::new();
    init.set_key(key);
    init.set_bubbles(true);
    init.set_cancelable(true);
    el.dispatch_event(&KeyboardEvent::new_with_keyboard_event_init_dict("keydown", &init).unwrap()).unwrap();
}

async fn single_and_multiple_sort() {
    let (host, log) = (host(), Arc::new(Mutex::new(Vec::<String>::new())));
    let l = log.clone();
    let mode = RwSignal::new(SortMode::Single);
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! {
            <Table
                value=products()
                columns=sortable_columns()
                row_key=key()
                sort_mode=mode
                on_sort=Callback::new(move |meta: Vec<SortMeta>| {
                    l.lock().unwrap().push(meta.iter().map(|m| format!("{}:{}", m.field, m.order)).collect::<Vec<_>>().join(","))
                })
            />
        }
    });
    settle().await;
    assert_eq!(names(&host), "Watch Band Shirt Purse Ring", "исходный порядок");
    header(&host, 1).click();
    settle().await;
    assert_eq!(names(&host), "Band Purse Ring Shirt Watch");
    assert_eq!(header(&host, 1).get_attribute("aria-sort").as_deref(), Some("ascending"));
    assert!(header(&host, 1).class_name().contains("p-datatable-column-sorted"));
    header(&host, 1).click();
    settle().await;
    assert_eq!(names(&host), "Watch Shirt Ring Purse Band", "повторный клик — по убыванию");
    assert_eq!(header(&host, 1).get_attribute("aria-sort").as_deref(), Some("descending"));
    key_down(&header(&host, 2), "Enter");
    settle().await;
    assert_eq!(names(&host), "Shirt Ring Watch Band Purse", "Enter сортирует по количеству; равные сохраняют порядок");
    assert_eq!(header(&host, 1).get_attribute("aria-sort").as_deref(), Some("none"), "прежний столбец сброшен");
    assert_eq!(log.lock().unwrap().as_slice(), ["name:1", "name:-1", "quantity:1"]);

    // многоуровневая сортировка с клавишей Ctrl
    mode.set(SortMode::Multiple);
    header(&host, 2).click();
    settle().await;
    click_with(&header(&host, 1), true);
    settle().await;
    let meta_log = log.lock().unwrap().clone();
    assert!(meta_log.last().unwrap().contains("quantity") && meta_log.last().unwrap().contains("name"), "{meta_log:?}");
    assert!(host.query_selector(".p-sortable-column-badge").unwrap().is_some(), "счётчики порядка");
    host.remove();
}

async fn selection_by_click_and_checkbox() {
    let (host, log) = (host(), Arc::new(Mutex::new(Vec::<String>::new())));
    let (l1, l2) = (log.clone(), log.clone());
    let selection = RwSignal::new(Vec::new());
    let mut columns = vec![Column::selection_checkbox()];
    columns.extend(plain_columns());
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! {
            <Table
                value=products()
                columns=columns.clone()
                row_key=key()
                selection_mode=Some(SelectionMode::Multiple)
                selection=selection
                on_row_select=Callback::new(move |p: Product| l1.lock().unwrap().push(format!("select:{}", p.code)))
                on_row_unselect=Callback::new(move |p: Product| l2.lock().unwrap().push(format!("unselect:{}", p.code)))
            />
        }
    });
    settle().await;
    let rows = all(&host, "tbody > tr");
    rows[1].click();
    settle().await;
    assert_eq!(selection.get_untracked(), ["c2"]);
    assert!(all(&host, "tbody > tr")[1].class_name().contains("p-datatable-row-selected"));
    assert_eq!(all(&host, "tbody > tr")[1].get_attribute("aria-selected").as_deref(), Some("true"));
    all(&host, "tbody > tr")[3].click();
    settle().await;
    assert_eq!(selection.get_untracked(), ["c2", "c4"]);
    all(&host, "tbody > tr")[1].click();
    settle().await;
    assert_eq!(selection.get_untracked(), ["c4"], "повторный клик снимает");

    // флажок строки отмечается тоже
    let boxes = all(&host, "tbody .p-checkbox-input");
    assert!(
        boxes[3].has_attribute("checked") || boxes[3].get_attribute("aria-checked").as_deref() == Some("true"),
        "флажок строки отражает выбор"
    );
    boxes[0].click();
    settle().await;
    assert_eq!(selection.get_untracked(), ["c4", "c1"], "клик по флажку добавляет, строка не переключается дважды");

    // флажок в заголовке: выбрать все и снять все
    let header_box = all(&host, "thead .p-checkbox-input")[0].clone();
    header_box.click();
    settle().await;
    assert_eq!(selection.get_untracked().len(), 5);
    header_box.click();
    settle().await;
    assert!(selection.get_untracked().is_empty());
    let log = log.lock().unwrap().clone();
    assert_eq!(&log[..3], ["select:c2", "select:c4", "unselect:c2"], "{log:?}");
    host.remove();
}

async fn radio_and_meta_key() {
    let host = host();
    let selection = RwSignal::new(Vec::new());
    let mut columns = vec![Column::selection_radio()];
    columns.extend(plain_columns());
    let _h = leptos::mount::mount_to(
        host.clone(),
        move || view! { <Table value=products() columns=columns.clone() row_key=key() selection=selection /> },
    );
    settle().await;
    all(&host, "tbody .p-radiobutton-input")[2].click();
    settle().await;
    assert_eq!(selection.get_untracked(), ["c3"]);
    all(&host, "tbody .p-radiobutton-input")[4].click();
    settle().await;
    assert_eq!(selection.get_untracked(), ["c5"], "радиокнопки выбирают одну строку");
    host.remove();

    let host = host_again();
    let selection = RwSignal::new(Vec::new());
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! {
            <Table
                value=products()
                columns=plain_columns()
                row_key=key()
                selection_mode=Some(SelectionMode::Multiple)
                meta_key_selection=true
                selection=selection
            />
        }
    });
    settle().await;
    click_with(&all(&host, "tbody > tr")[0], false);
    click_with(&all(&host, "tbody > tr")[2], false);
    settle().await;
    assert_eq!(selection.get_untracked(), ["c3"], "без Ctrl выбор заменяется");
    click_with(&all(&host, "tbody > tr")[0], true);
    settle().await;
    assert_eq!(selection.get_untracked(), ["c3", "c1"], "с Ctrl выбор дополняется");
    click_with(&all(&host, "tbody > tr")[2], true);
    settle().await;
    assert_eq!(selection.get_untracked(), ["c1"], "с Ctrl выбранная снимается");
    host.remove();
}

fn host_again() -> HtmlElement {
    host()
}

async fn expansion_and_paging() {
    let (host, log) = (host(), Arc::new(Mutex::new(Vec::<String>::new())));
    let (l1, l2) = (log.clone(), log.clone());
    let mut columns = vec![Column::expander()];
    columns.extend(plain_columns());
    let expanded = RwSignal::new(Vec::new());
    let expansion = Callback::new(|p: Product| view! { <span class="detail">{format!("Details of {}", p.name)}</span> }.into_any());
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! {
            <Table
                value=products()
                columns=columns.clone()
                row_key=key()
                expanded_keys=expanded
                row_expansion=expansion
                on_row_expand=Callback::new(move |p: Product| l1.lock().unwrap().push(format!("expand:{}", p.code)))
                on_row_collapse=Callback::new(move |p: Product| l2.lock().unwrap().push(format!("collapse:{}", p.code)))
            />
        }
    });
    settle().await;
    assert!(host.query_selector(".p-datatable-row-expansion").unwrap().is_none());
    all(&host, ".p-datatable-row-toggle-button")[1].click();
    settle().await;
    assert_eq!(expanded.get_untracked(), ["c2"]);
    assert_eq!(all(&host, ".p-datatable-row-expansion").len(), 1);
    assert!(all(&host, ".detail")[0].text_content().unwrap().contains("Details of Band"));
    all(&host, ".p-datatable-row-toggle-button")[3].click();
    settle().await;
    assert_eq!(all(&host, ".p-datatable-row-expansion").len(), 2);
    all(&host, ".p-datatable-row-toggle-button")[1].click();
    settle().await;
    assert_eq!(expanded.get_untracked(), ["c4"]);
    assert_eq!(log.lock().unwrap().as_slice(), ["expand:c2", "expand:c4", "collapse:c2"]);
    host.remove();

    // пагинатор и возврат на первую страницу при сортировке
    let host = host_again();
    let first = RwSignal::new(0usize);
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <Table value=products() columns=sortable_columns() row_key=key() paginator=true default_rows=2usize first=first /> }
    });
    settle().await;
    assert_eq!(all(&host, "tbody > tr").len(), 2);
    all(&host, "button.p-paginator-next")[0].click();
    settle().await;
    assert_eq!(first.get_untracked(), 2);
    assert_eq!(names(&host), "Shirt Purse");
    header(&host, 1).click();
    settle().await;
    assert_eq!(first.get_untracked(), 0, "сортировка возвращает на первую страницу");
    assert_eq!(names(&host), "Band Purse");
    host.remove();
}

async fn empty_and_loading() {
    let host = host();
    let loading = RwSignal::new(false);
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <Table value={Vec::<Product>::new()} columns=plain_columns() row_key=key() loading=loading /> }
    });
    settle().await;
    assert!(host.query_selector(".p-datatable-empty-message").unwrap().is_some());
    loading.set(true);
    settle().await;
    assert!(host.query_selector(".p-datatable-mask").unwrap().is_some());
    assert!(host.query_selector(".p-datatable-empty-message").unwrap().is_none());
    host.remove();
}

#[wasm_bindgen_test]
async fn table_in_the_browser() {
    single_and_multiple_sort().await;
    selection_by_click_and_checkbox().await;
    radio_and_meta_key().await;
    expansion_and_paging().await;
    empty_and_loading().await;
}
