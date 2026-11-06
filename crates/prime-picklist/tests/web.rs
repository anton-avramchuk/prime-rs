//! Поведение PickList в реальном браузере (wasm-bindgen-test): выбор, перенос в обе стороны, «всё», перестановка,
//! двойной клик, `keep_selection`, поиск и перенос видимых, `disabled`. Сценарии идут одним последовательным тестом.
#![cfg(target_arch = "wasm32")]
#![recursion_limit = "512"]

use leptos::prelude::*;
use prime_listbox::ListboxOption;
use prime_picklist::PickList;
use std::sync::{Arc, Mutex};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;
use web_sys::{Event, EventInit, HtmlElement, HtmlInputElement, MouseEvent, MouseEventInit};

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

fn all(root: &HtmlElement, selector: &str) -> Vec<HtmlElement> {
    let list = root.query_selector_all(selector).unwrap();
    (0..list.length()).map(|i| list.item(i).unwrap().dyn_into().unwrap()).collect()
}

fn labels(host: &HtmlElement, container: &str) -> String {
    let c = host.query_selector(container).unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
    all(&c, ".p-listbox-option").iter().map(|e| e.text_content().unwrap_or_default().trim().to_owned()).collect::<Vec<_>>().join(" ")
}

fn source(host: &HtmlElement) -> String {
    labels(host, ".p-picklist-source-list-container")
}

fn target(host: &HtmlElement) -> String {
    labels(host, ".p-picklist-target-list-container")
}

fn click_option(host: &HtmlElement, container: &str, index: usize) {
    let c = host.query_selector(container).unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
    all(&c, ".p-listbox-option")[index].click();
}

fn button(host: &HtmlElement, aria: &str) -> HtmlElement {
    host.query_selector(&format!("button[aria-label=\"{aria}\"]"))
        .unwrap()
        .unwrap_or_else(|| panic!("нет {aria}: {}", host.outer_html()))
        .dyn_into()
        .unwrap()
}

fn disabled(el: &HtmlElement) -> bool {
    el.has_attribute("disabled")
}

fn list(names: &[&str]) -> RwSignal<Vec<ListboxOption>> {
    RwSignal::new(names.iter().map(|n| ListboxOption::new(*n, n.to_lowercase())).collect())
}

const SRC: &str = ".p-picklist-source-list-container";
const TGT: &str = ".p-picklist-target-list-container";

async fn transfer_selected_and_all() {
    let (host, log) = (host(), Log::default());
    let (l1, l2, l3) = (log.clone(), log.clone(), log.clone());
    let (source_list, target_list) = (list(&["A", "B", "C", "D"]), list(&["X"]));
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! {
            <PickList
                source=source_list
                target=target_list
                on_move_to_target=Callback::new(move |items: Vec<ListboxOption>| l1.lock().unwrap().push(format!("to_target:{}", items.len())))
                on_move_to_source=Callback::new(move |items: Vec<ListboxOption>| l2.lock().unwrap().push(format!("to_source:{}", items.len())))
                on_move_all_to_target=Callback::new(move |items: Vec<ListboxOption>| l3.lock().unwrap().push(format!("all_to_target:{}", items.len())))
            />
        }
    });
    settle().await;
    assert_eq!((source(&host), target(&host)), ("A B C D".to_owned(), "X".to_owned()));
    assert!(disabled(&button(&host, "Move to Target")) && !disabled(&button(&host, "Move All to Target")));

    click_option(&host, SRC, 1);
    click_option(&host, SRC, 3);
    settle().await;
    assert!(!disabled(&button(&host, "Move to Target")));
    button(&host, "Move to Target").click();
    settle().await;
    assert_eq!((source(&host), target(&host)), ("A C".to_owned(), "X B D".to_owned()));
    assert!(disabled(&button(&host, "Move to Target")), "выбор источника очищен");

    click_option(&host, TGT, 0);
    settle().await;
    button(&host, "Move to Source").click();
    settle().await;
    assert_eq!((source(&host), target(&host)), ("A C X".to_owned(), "B D".to_owned()));

    button(&host, "Move All to Target").click();
    settle().await;
    assert_eq!((source(&host), target(&host)), (String::new(), "B D A C X".to_owned()));
    assert!(disabled(&button(&host, "Move All to Target")), "источник пуст");
    button(&host, "Move All to Source").click();
    settle().await;
    assert_eq!((source(&host), target(&host)), ("B D A C X".to_owned(), String::new()));
    assert_eq!(log.lock().unwrap().as_slice(), ["to_target:2", "to_source:1", "all_to_target:3"]);
    assert_eq!(source_list.get_untracked().len(), 5, "сигнал источника обновлён");
    host.remove();
}

async fn reorder_in_both_lists() {
    let (host, log) = (host(), Log::default());
    let (l1, l2) = (log.clone(), log.clone());
    let (source_list, target_list) = (list(&["A", "B", "C"]), list(&["X", "Y", "Z"]));
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! {
            <PickList
                source=source_list
                target=target_list
                on_source_reorder=Callback::new(move |sel: Vec<String>| l1.lock().unwrap().push(format!("src:{}", sel.join(","))))
                on_target_reorder=Callback::new(move |sel: Vec<String>| l2.lock().unwrap().push(format!("tgt:{}", sel.join(","))))
            />
        }
    });
    settle().await;
    let (src_ctl, tgt_ctl) = (".p-picklist-source-controls", ".p-picklist-target-controls");
    let ctl = |c: &str, aria: &str| -> HtmlElement {
        let root = host.query_selector(c).unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
        root.query_selector(&format!("button[aria-label=\"{aria}\"]")).unwrap().unwrap().dyn_into().unwrap()
    };
    assert!(disabled(&ctl(src_ctl, "Move Up")), "без выбора перестановка недоступна");
    click_option(&host, SRC, 2);
    settle().await;
    ctl(src_ctl, "Move Up").click();
    settle().await;
    assert_eq!(source(&host), "A C B");
    ctl(src_ctl, "Move Top").click();
    settle().await;
    assert_eq!(source(&host), "C A B");
    click_option(&host, TGT, 0);
    settle().await;
    ctl(tgt_ctl, "Move Bottom").click();
    settle().await;
    assert_eq!(target(&host), "Y Z X");
    ctl(tgt_ctl, "Move Down").click();
    assert_eq!(target(&host), "Y Z X", "уже внизу — на месте");
    assert_eq!(log.lock().unwrap().as_slice(), ["src:c", "src:c", "tgt:x", "tgt:x"]);
    host.remove();
}

async fn double_click_keep_selection_and_filter() {
    let host = host();
    let (source_list, target_list) = (list(&["Apple", "Banana", "Avocado"]), list(&[]));
    let target_sel = RwSignal::new(Vec::<String>::new());
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <PickList source=source_list target=target_list target_selection=target_sel filter=true keep_selection=true /> }
    });
    settle().await;
    // поиск: «всё в цель» переносит только найденное
    let filter: HtmlInputElement = host.query_selector(&format!("{SRC} input.p-listbox-filter")).unwrap().unwrap().dyn_into().unwrap();
    filter.set_value("ap");
    let init = EventInit::new();
    init.set_bubbles(true);
    filter.dispatch_event(&Event::new_with_event_init_dict("input", &init).unwrap()).unwrap();
    settle().await;
    assert_eq!(source(&host), "Apple");
    button(&host, "Move All to Target").click();
    settle().await;
    assert_eq!(target(&host), "Apple");
    assert_eq!(target_sel.get_untracked(), ["apple"], "keep_selection: перенесённые выбраны в цели");
    assert_eq!(source_list.get_untracked().len(), 2, "Banana и Avocado остались");

    // двойной клик переносит выбранные
    filter.set_value("");
    filter.dispatch_event(&Event::new_with_event_init_dict("input", &init).unwrap()).unwrap();
    settle().await;
    click_option(&host, SRC, 0);
    settle().await;
    let c = host.query_selector(SRC).unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
    let option = all(&c, ".p-listbox-option")[0].clone();
    let dbl = MouseEventInit::new();
    dbl.set_bubbles(true);
    option.dispatch_event(&MouseEvent::new_with_mouse_event_init_dict("dblclick", &dbl).unwrap()).unwrap();
    settle().await;
    assert_eq!(target(&host), "Apple Banana");
    assert_eq!(source(&host), "Avocado");
    host.remove();
}

async fn disabled_state() {
    let host = host();
    let selection = RwSignal::new(vec!["a".to_owned()]);
    let (source_list, target_list) = (list(&["A"]), list(&["X"]));
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <PickList source=source_list target=target_list source_selection=selection disabled=true /> }
    });
    settle().await;
    for aria in ["Move to Target", "Move All to Target", "Move All to Source", "Move Up"] {
        assert!(disabled(&button(&host, aria)), "{aria}");
    }
    host.remove();
}

#[wasm_bindgen_test]
async fn picklist_in_the_browser() {
    transfer_selected_and_all().await;
    reorder_in_both_lists().await;
    double_click_keep_selection_and_filter().await;
    disabled_state().await;
}
