//! Поведение Steps в реальном браузере (wasm-bindgen-test): клик в режимах чтения и интерактивном, отключённые пункты,
//! переход по ссылке, клавиши и roving `tabindex`, `Enter`/`Space`, двусторонний сигнал.
#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use prime_steps::{Steps, StepsItem};
use std::sync::{Arc, Mutex};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;
use web_sys::{HtmlElement, KeyboardEvent, KeyboardEventInit, MouseEvent, MouseEventInit};

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

fn links(host: &HtmlElement) -> Vec<HtmlElement> {
    let list = host.query_selector_all(".p-steps-item-link").unwrap();
    (0..list.length()).map(|i| list.item(i).unwrap().dyn_into().unwrap()).collect()
}

async fn settle() {
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, 40).unwrap();
    });
    JsFuture::from(promise).await.unwrap();
}

/// Клик с возможностью отмены: `false`, если переход отменён.
fn click(el: &HtmlElement) -> bool {
    let init = MouseEventInit::new();
    init.set_bubbles(true);
    init.set_cancelable(true);
    el.dispatch_event(&MouseEvent::new_with_mouse_event_init_dict("click", &init).unwrap()).unwrap()
}

fn press(el: &HtmlElement, code: &str) -> bool {
    let init = KeyboardEventInit::new();
    init.set_code(code);
    init.set_bubbles(true);
    init.set_cancelable(true);
    el.dispatch_event(&KeyboardEvent::new_with_keyboard_event_init_dict("keydown", &init).unwrap()).unwrap()
}

fn tabs(host: &HtmlElement) -> Vec<String> {
    links(host).iter().map(|l| l.get_attribute("tabindex").unwrap()).collect()
}

fn model(log: &Log) -> Vec<StepsItem> {
    let cmd = |name: &'static str, log: &Log| {
        let log = log.clone();
        move |c: prime_steps::StepsCommand| log.lock().unwrap().push(format!("{name}:{}", c.index))
    };
    vec![
        StepsItem::new().label("a").command(Callback::new(cmd("a", log))),
        StepsItem::new().label("b").url("#b").command(Callback::new(cmd("b", log))),
        StepsItem::new().label("c").disabled(true).command(Callback::new(cmd("c", log))),
        StepsItem::new().label("d").command(Callback::new(cmd("d", log))),
    ]
}

#[wasm_bindgen_test]
async fn readonly_ignores_clicks_and_prevents_navigation() {
    let (host, log) = (host(), Log::default());
    let (items, changes) = (model(&log), log.clone());
    let _h = leptos::mount::mount_to(host.clone(), move || {
        let changes = changes.clone();
        view! { <Steps model=items.clone() active_index=0usize on_active_index_change=Callback::new(move |i| changes.lock().unwrap().push(format!("change:{i}"))) /> }
    });
    settle().await;
    assert!(!click(&links(&host)[1]), "в режиме чтения переход отменён даже у пункта с url");
    assert!(!click(&links(&host)[0]));
    settle().await;
    assert!(log.lock().unwrap().is_empty(), "команды и события не вызваны");
    host.remove();
}

#[wasm_bindgen_test]
async fn interactive_click_reports_the_index_and_runs_the_command() {
    let (host, log) = (host(), Log::default());
    let active = RwSignal::new(0usize);
    let (items, changes) = (model(&log), log.clone());
    let _h = leptos::mount::mount_to(host.clone(), move || {
        let changes = changes.clone();
        view! {
            <Steps
                model=items.clone()
                active_index_signal=active
                readonly=false
                on_active_index_change=Callback::new(move |i| changes.lock().unwrap().push(format!("change:{i}")))
            />
        }
    });
    settle().await;
    assert!(!click(&links(&host)[3]), "без url переход отменён");
    settle().await;
    assert_eq!(active.get_untracked(), 3, "сигнал обновлён");
    assert_eq!(*log.lock().unwrap(), ["change:3", "d:3"]);
    let items_li = host.query_selector_all(".p-steps-item").unwrap();
    assert!(items_li.item(3).unwrap().dyn_into::<HtmlElement>().unwrap().class_name().contains("p-steps-item-active"));
    assert_eq!(links(&host)[3].get_attribute("aria-expanded").as_deref(), Some("true"));
    assert!(click(&links(&host)[1]), "у пункта с url переход не отменяется");
    settle().await;
    assert_eq!(active.get_untracked(), 1);
    assert_eq!(log.lock().unwrap().last().map(String::as_str), Some("b:1"));
    let before = log.lock().unwrap().len();
    click(&links(&host)[2]);
    settle().await;
    assert_eq!(log.lock().unwrap().len(), before, "отключённый пункт игнорируется");
    assert_eq!(active.get_untracked(), 1);
    host.remove();
}

#[wasm_bindgen_test]
async fn arrow_keys_move_focus_with_a_roving_tabindex() {
    let (host, log) = (host(), Log::default());
    let items = model(&log);
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Steps model=items.clone() active_index=0usize readonly=false /> });
    settle().await;
    let ls = links(&host);
    assert_eq!(tabs(&host), ["0", "-1", "-1", "-1"], "по умолчанию — активный пункт");
    assert!(!press(&ls[0], "ArrowRight"), "стрелка перехвачена");
    settle().await;
    assert_eq!(tabs(&host), ["-1", "0", "-1", "-1"]);
    assert!(doc().active_element().unwrap().is_same_node(Some(&ls[1])));
    press(&ls[1], "End");
    settle().await;
    assert_eq!(tabs(&host), ["-1", "-1", "-1", "0"], "End: последний, даже если он не активен");
    press(&ls[3], "ArrowRight");
    settle().await;
    assert!(doc().active_element().unwrap().is_same_node(Some(&ls[3])), "правее последнего не уходит");
    press(&ls[3], "Home");
    settle().await;
    assert!(doc().active_element().unwrap().is_same_node(Some(&ls[0])));
    press(&ls[0], "ArrowLeft");
    assert!(doc().active_element().unwrap().is_same_node(Some(&ls[0])), "левее первого не уходит");
    press(&ls[0], "ArrowRight");
    settle().await;
    assert!(press(&ls[1], "Tab"), "Tab не перехватывается");
    settle().await;
    assert_eq!(tabs(&host), ["0", "-1", "-1", "-1"], "Tab возвращает tabindex активному пункту");
    host.remove();
}

#[wasm_bindgen_test]
async fn enter_and_space_act_like_a_click() {
    let (host, log) = (host(), Log::default());
    let active = RwSignal::new(0usize);
    let items = model(&log);
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <Steps model=items.clone() active_index_signal=active readonly=false /> }
    });
    settle().await;
    let ls = links(&host);
    assert!(!press(&ls[3], "Enter"), "Enter обработан");
    settle().await;
    assert_eq!(active.get_untracked(), 3);
    assert_eq!(*log.lock().unwrap(), ["d:3"]);
    press(&ls[0], "Space");
    settle().await;
    assert_eq!(active.get_untracked(), 0);
    press(&ls[2], "Enter");
    settle().await;
    assert_eq!(active.get_untracked(), 0, "отключённый пункт Enter не выбирает");
    assert_eq!(tabs(&host), ["0", "-1", "-1", "-1"], "после смены шага фокусный tabindex у нового активного");
    assert!(press(&ls[0], "KeyA"), "посторонняя клавиша не перехватывается");
    host.remove();
}
