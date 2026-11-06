//! Поведение DataView в реальном браузере (wasm-bindgen-test): переключение страниц пагинатором, возврат `first`,
//! смена порядка сортировки, режим `lazy` и его события. Сценарии идут одним последовательным тестом.
#![cfg(target_arch = "wasm32")]
#![recursion_limit = "512"]

use leptos::prelude::*;
use prime_dataview::{DataView, DataViewLayout, LazyLoadEvent, PageEvent};
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

fn shown(host: &HtmlElement) -> Vec<String> {
    all(host, ".item").iter().map(|e| e.text_content().unwrap_or_default()).collect()
}

fn list() -> Callback<Vec<i32>, AnyView> {
    Callback::new(|page: Vec<i32>| {
        view! { <ul>{page.into_iter().map(|i| view! { <li class="item">{i}</li> }).collect_view()}</ul> }.into_any()
    })
}

fn grid() -> Callback<Vec<i32>, AnyView> {
    Callback::new(|page: Vec<i32>| {
        view! { <div>{page.into_iter().map(|i| view! { <b class="item">{i}</b> }).collect_view()}</div> }.into_any()
    })
}

async fn paging_and_layout() {
    let (host, log) = (host(), Log::default());
    let l = log.clone();
    let layout = RwSignal::new(DataViewLayout::List);
    let first = RwSignal::new(0usize);
    let numbers: Vec<i32> = (1..=25).collect();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! {
            <DataView
                value=numbers.clone()
                list=list()
                grid=grid()
                layout=layout
                paginator=true
                default_rows=10usize
                first=first
                on_page=Callback::new(move |e: PageEvent| l.lock().unwrap().push(format!("page:{}:{}", e.first, e.rows)))
            />
        }
    });
    settle().await;
    assert_eq!(shown(&host).len(), 10);
    assert_eq!(shown(&host)[0], "1");
    all(&host, "button.p-paginator-next")[0].click();
    settle().await;
    assert_eq!(shown(&host)[0], "11", "следующая страница");
    assert_eq!(first.get_untracked(), 10);
    all(&host, "button.p-paginator-last")[0].click();
    settle().await;
    assert_eq!(shown(&host), ["21", "22", "23", "24", "25"], "последняя страница короче");
    // внешняя смена first вне диапазона возвращается на допустимую страницу
    first.set(500);
    settle().await;
    assert_eq!(first.get_untracked(), 20, "first возвращён на последнюю страницу");
    layout.set(DataViewLayout::Grid);
    settle().await;
    assert!(host.query_selector(".p-dataview-grid").unwrap().is_some() && host.query_selector("b.item").unwrap().is_some());
    let log = log.lock().unwrap().clone();
    assert_eq!(&log[..2], ["page:10:10", "page:20:10"], "{log:?}");
    assert!(log[2..].iter().all(|e| e == "page:20:10"), "возврат first может повторить событие последней страницы: {log:?}");
    host.remove();
}

async fn sort_and_filter_react() {
    let host = host();
    let order = RwSignal::new(1);
    let min = RwSignal::new(0);
    let first = RwSignal::new(0usize);
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! {
            <DataView
                value=vec![5, 3, 8, 1, 9, 2, 7]
                list=list()
                compare=Callback::new(|(a, b): (i32, i32)| a.cmp(&b))
                sort_order=order
                filter=Callback::new(move |n: i32| n > min.get())
                paginator=true
                default_rows=3usize
                first=first
            />
        }
    });
    settle().await;
    assert_eq!(shown(&host), ["1", "2", "3"]);
    all(&host, "button.p-paginator-next")[0].click();
    settle().await;
    assert_eq!(shown(&host), ["5", "7", "8"]);
    order.set(-1);
    settle().await;
    assert_eq!(first.get_untracked(), 0, "смена порядка возвращает на первую страницу");
    assert_eq!(shown(&host), ["9", "8", "7"]);
    min.set(5);
    settle().await;
    assert_eq!(shown(&host), ["9", "8", "7"], "фильтр: 7, 8, 9");
    all(&host, "button.p-paginator-next")[0].click();
    settle().await;
    assert!(shown(&host).is_empty() || shown(&host).len() <= 3);
    host.remove();
}

async fn lazy_mode_events() {
    let (host, log) = (host(), Log::default());
    let l = log.clone();
    let order = RwSignal::new(0);
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! {
            <DataView
                value=vec![1, 2, 3]
                list=list()
                paginator=true
                lazy=true
                total_records=30usize
                default_rows=3usize
                sort_order=order
                on_lazy_load=Callback::new(move |e: LazyLoadEvent| l.lock().unwrap().push(format!("lazy:{}:{}:{}", e.first, e.rows, e.sort_order)))
            />
        }
    });
    settle().await;
    assert_eq!(log.lock().unwrap().as_slice(), ["lazy:0:3:0"], "начальная загрузка");
    assert_eq!(shown(&host), ["1", "2", "3"], "значение показывается как есть");
    all(&host, "button.p-paginator-next")[0].click();
    settle().await;
    order.set(1);
    settle().await;
    let log = log.lock().unwrap().clone();
    assert_eq!(log, ["lazy:0:3:0", "lazy:3:3:0", "lazy:0:3:1"], "{log:?}");
    host.remove();
}

async fn empty_and_loading() {
    let host = host();
    let loading = RwSignal::new(false);
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <DataView value={Vec::<i32>::new()} list=list() loading=loading /> });
    settle().await;
    assert!(host.query_selector(".p-dataview-empty-message").unwrap().is_some());
    loading.set(true);
    settle().await;
    assert!(host.query_selector(".p-dataview-loading-overlay").unwrap().is_some());
    assert!(host.query_selector(".p-dataview-empty-message").unwrap().is_none(), "при загрузке сообщения нет");
    host.remove();
}

#[wasm_bindgen_test]
async fn dataview_in_the_browser() {
    paging_and_layout().await;
    sort_and_filter_react().await;
    lazy_mode_events().await;
    empty_and_loading().await;
}
