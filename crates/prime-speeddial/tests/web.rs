//! Поведение SpeedDial в реальном браузере (wasm-bindgen-test): показ и скрытие, команды, клик вне, `Escape`, клавиатура
//! во всех направлениях, маска, управляемый `visible`, раскладка и шаблоны.
//!
//! Сценарии идут одним последовательным тестом: слушатели окна общие на страницу. Стили темы не подключены.
#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use prime_speeddial::{SpeedDial, SpeedDialDirection, SpeedDialItem, SpeedDialItemContext, SpeedDialType};
use std::sync::{Arc, Mutex};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;
use web_sys::{HtmlElement, KeyboardEvent, KeyboardEventInit};

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

fn q(host: &HtmlElement, sel: &str) -> HtmlElement {
    host.query_selector(sel).unwrap().unwrap_or_else(|| panic!("нет {sel}")).dyn_into().unwrap()
}

fn all(host: &HtmlElement, sel: &str) -> Vec<HtmlElement> {
    let list = host.query_selector_all(sel).unwrap();
    (0..list.length()).map(|i| list.item(i).unwrap().dyn_into().unwrap()).collect()
}

async fn sleep(ms: i32) {
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms).unwrap();
    });
    JsFuture::from(promise).await.unwrap();
}

/// Даёт эффектам примениться и слушателю окна повеситься (он вешается в следующем такте).
async fn settle() {
    sleep(60).await;
}

fn key(el: &HtmlElement, code: &str) -> bool {
    let init = KeyboardEventInit::new();
    init.set_code(code);
    init.set_key(code);
    init.set_bubbles(true);
    init.set_cancelable(true);
    el.dispatch_event(&KeyboardEvent::new_with_keyboard_event_init_dict("keydown", &init).unwrap()).unwrap()
}

fn is_open(host: &HtmlElement) -> bool {
    q(host, ".p-speeddial").class_name().contains("p-speeddial-open")
}

fn log_cmd(log: &Log, name: &'static str) -> SpeedDialItem {
    let log = log.clone();
    SpeedDialItem::new().label(name).command(Callback::new(move |_| log.lock().unwrap().push(name.to_owned())))
}

fn model(log: &Log) -> Vec<SpeedDialItem> {
    vec![log_cmd(log, "a"), log_cmd(log, "b").disabled(true), log_cmd(log, "c"), log_cmd(log, "d")]
}

fn focused_label(host: &HtmlElement) -> Option<String> {
    all(host, ".p-speeddial-item.p-focus")
        .first()
        .and_then(|li| li.query_selector("button").unwrap())
        .and_then(|b| b.get_attribute("aria-label"))
}

async fn toggle_events_and_item_commands() {
    let (host, log) = (host(), Log::default());
    let (l1, l2, l3, l4) = (log.clone(), log.clone(), log.clone(), log.clone());
    let items = model(&log);
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! {
            <SpeedDial
                id="sd"
                model=items.clone()
                on_visible_change=Callback::new(move |v: bool| l1.lock().unwrap().push(format!("change:{v}")))
                on_click=Callback::new(move |_| l2.lock().unwrap().push("click".into()))
                on_show=Callback::new(move |()| l3.lock().unwrap().push("show".into()))
                on_hide=Callback::new(move |()| l4.lock().unwrap().push("hide".into()))
            />
        }
    });
    settle().await;
    let button = q(&host, ".p-speeddial-button");
    assert_eq!(button.get_attribute("aria-expanded").as_deref(), Some("false"));
    assert!(!is_open(&host));
    button.click();
    settle().await;
    assert!(is_open(&host) && button.get_attribute("aria-expanded").as_deref() == Some("true"));
    assert_eq!(all(&host, ".p-speeddial-action")[0].get_attribute("tabindex").as_deref(), Some("0"), "у действий tabindex при показе");
    assert!(all(&host, ".p-speeddial-action")[1].get_attribute("tabindex").is_none(), "у отключённого нет tabindex");
    button.click();
    settle().await;
    assert!(!is_open(&host));
    button.click();
    settle().await;
    all(&host, ".p-speeddial-action")[2].click();
    settle().await;
    assert!(!is_open(&host), "команда скрывает список");
    all(&host, ".p-speeddial-action")[1].click();
    settle().await;
    assert_eq!(
        *log.lock().unwrap(),
        ["change:true", "show", "click", "change:false", "hide", "click", "change:true", "show", "click", "c", "change:false", "hide"]
    );
    host.remove();
}

async fn outside_click_escape_disabled_and_mask() {
    let (host, log) = (host(), Log::default());
    let (items, items2) = (model(&log), model(&log));
    let stay = RwSignal::new(false);
    let (_h, _h2) = (
        leptos::mount::mount_to(host.clone(), move || view! { <div class="a"><SpeedDial id="a" model=items.clone() mask=true /></div> }),
        leptos::mount::mount_to(host.clone(), move || {
            view! { <div class="b"><SpeedDial id="b" model=items2.clone() hide_on_click_outside=stay disabled=false /></div> }
        }),
    );
    settle().await;
    let a = q(&host, ".a");
    let (button_a, button_b) = (q(&a, ".p-speeddial-button"), q(&q(&host, ".b"), ".p-speeddial-button"));
    assert!(a.query_selector(".p-speeddial-mask").unwrap().is_none(), "маски нет, пока закрыто");
    button_a.click();
    settle().await;
    let mask = q(&a, ".p-speeddial-mask");
    assert!(mask.class_name().contains("p-overlay-mask"), "{}", mask.class_name());
    doc().body().unwrap().click();
    settle().await;
    assert!(!is_open(&a) && a.query_selector(".p-speeddial-mask").unwrap().is_none(), "клик вне скрывает, маска убрана");
    button_a.click();
    settle().await;
    q(&a, ".p-speeddial-list").click();
    settle().await;
    assert!(is_open(&a), "клик внутри компонента не скрывает");
    key(&button_a, "Escape");
    settle().await;
    assert!(!is_open(&a), "Escape на кнопке скрывает");
    assert!(doc().active_element().unwrap().is_same_node(Some(&button_a)), "фокус на кнопке");

    let b = q(&host, ".b");
    stay.set(false);
    button_b.click();
    settle().await;
    doc().body().unwrap().click();
    settle().await;
    assert!(is_open(&b), "hide_on_click_outside=false: клик снаружи не скрывает");
    stay.set(true);
    doc().body().unwrap().click();
    settle().await;
    assert!(!is_open(&b), "hide_on_click_outside=true: клик снаружи скрывает");
    host.remove();
}

async fn keyboard_in_every_direction() {
    use SpeedDialDirection as D;
    // (направление, клавиша «следующий», клавиша «предыдущий»)
    let table = [
        (D::Up, "ArrowUp", "ArrowDown"),
        (D::Down, "ArrowDown", "ArrowUp"),
        (D::Left, "ArrowLeft", "ArrowRight"),
        (D::Right, "ArrowRight", "ArrowLeft"),
        (D::UpLeft, "ArrowRight", "ArrowLeft"),
        (D::UpRight, "ArrowLeft", "ArrowRight"),
        (D::DownLeft, "ArrowLeft", "ArrowRight"),
        (D::DownRight, "ArrowRight", "ArrowLeft"),
    ];
    for (direction, next, prev) in table {
        let (host, log) = (host(), Log::default());
        let items = model(&log);
        let _h = leptos::mount::mount_to(host.clone(), move || view! { <SpeedDial id="k" model=items.clone() direction=direction /> });
        settle().await;
        let (button, list) = (q(&host, ".p-speeddial-button"), q(&host, ".p-speeddial-list"));
        assert!(!key(&button, "ArrowDown"), "{direction:?}: стрелка на кнопке перехвачена");
        settle().await;
        assert!(is_open(&host), "{direction:?}: список открыт");
        assert_eq!(focused_label(&host).as_deref(), Some("a"), "{direction:?}: первый доступный пункт");
        assert_eq!(list.get_attribute("aria-activedescendant").as_deref(), Some("k_0"));
        key(&list, next);
        settle().await;
        assert_eq!(focused_label(&host).as_deref(), Some("c"), "{direction:?} {next}: b отключён");
        key(&list, next);
        key(&list, next);
        settle().await;
        assert_eq!(focused_label(&host).as_deref(), Some("d"), "{direction:?}: на границе остаётся");
        key(&list, prev);
        settle().await;
        assert_eq!(focused_label(&host).as_deref(), Some("c"), "{direction:?} {prev}");
        key(&list, "Home");
        settle().await;
        assert_eq!(focused_label(&host).as_deref(), Some("a"), "{direction:?}: Home");
        key(&list, "End");
        settle().await;
        assert_eq!(focused_label(&host).as_deref(), Some("d"), "{direction:?}: End");
        key(&list, "Enter");
        settle().await;
        assert_eq!(*log.lock().unwrap(), ["d"], "{direction:?}: Enter активирует пункт");
        assert!(!is_open(&host), "{direction:?}: после Enter список скрыт");
        assert!(doc().active_element().unwrap().is_same_node(Some(&button)), "{direction:?}: фокус вернулся на кнопку");
        assert!(list.get_attribute("aria-activedescendant").is_none());
        host.remove();
    }
}

async fn toggler_up_arrow_focuses_the_last_item_and_unhandled_keys_pass() {
    let (host, log) = (host(), Log::default());
    let items = model(&log);
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <SpeedDial id="t" model=items.clone() /> });
    settle().await;
    let (button, list) = (q(&host, ".p-speeddial-button"), q(&host, ".p-speeddial-list"));
    assert!(key(&button, "KeyA"), "посторонняя клавиша не перехватывается");
    assert!(!key(&button, "ArrowUp"));
    settle().await;
    assert_eq!(focused_label(&host).as_deref(), Some("d"), "ArrowUp на кнопке — последний пункт");
    assert!(key(&list, "Tab"), "Tab в списке не перехватывается");
    key(&list, "Space");
    settle().await;
    assert_eq!(*log.lock().unwrap(), ["d"], "Space активирует пункт");
    key(&button, "ArrowLeft");
    settle().await;
    assert_eq!(focused_label(&host).as_deref(), Some("a"), "ArrowLeft на кнопке — первый пункт");
    key(&list, "Escape");
    settle().await;
    assert!(!is_open(&host), "Escape в списке скрывает");
    host.remove();
}

async fn controlled_visible_disabled_and_hidden_items() {
    let (host, log) = (host(), Log::default());
    let visible = RwSignal::new(false);
    let items = vec![log_cmd(&log, "a"), log_cmd(&log, "h").visible(false), log_cmd(&log, "c")];
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <SpeedDial id="v" model=items.clone() visible=visible /> });
    settle().await;
    visible.set(true);
    settle().await;
    assert!(is_open(&host), "сигнал открывает список");
    q(&host, ".p-speeddial-button").click();
    settle().await;
    assert!(!visible.get_untracked() && !is_open(&host), "кнопка меняет сигнал");
    assert!(all(&host, ".p-speeddial-item")[1].class_name().contains("p-hidden"), "скрытый пункт");
    q(&host, ".p-speeddial-button").click();
    settle().await;
    let list = q(&host, ".p-speeddial-list");
    list.focus().unwrap();
    settle().await;
    key(&list, "Home");
    settle().await;
    assert_eq!(focused_label(&host).as_deref(), Some("a"));
    key(&list, "ArrowUp");
    settle().await;
    assert_eq!(focused_label(&host).as_deref(), Some("c"), "скрытый пункт пропускается");
    key(&list, "ArrowDown");
    settle().await;
    assert_eq!(focused_label(&host).as_deref(), Some("a"));
    host.remove();

    let host = host_with_disabled().await;
    host.remove();
}

async fn host_with_disabled() -> HtmlElement {
    let (host, log) = (host(), Log::default());
    let items = model(&log);
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <SpeedDial id="d" model=items.clone() disabled=true /> });
    settle().await;
    let button = q(&host, ".p-speeddial-button");
    assert!(button.has_attribute("disabled") && q(&host, ".p-speeddial").class_name().contains("p-disabled"));
    button.click();
    settle().await;
    assert!(!is_open(&host), "отключённый компонент не открывается");
    host
}

async fn layout_styles_and_offsets() {
    let (host, log) = (host(), Log::default());
    let items = model(&log);
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <SpeedDial id="c" model=items.clone() r#type=SpeedDialType::Circle radius=80.0 transition_delay=40 /> }
    });
    settle().await;
    let (list, lis) = (q(&host, ".p-speeddial-list"), all(&host, ".p-speeddial-item"));
    assert_eq!(lis.len(), 4);
    assert!(
        lis[0].style().get_property_value("left").unwrap().contains("calc(80px"),
        "{}",
        lis[0].style().get_property_value("left").unwrap()
    );
    assert!(lis[0].style().get_property_value("top").unwrap().contains("var(--item-diff-y"));
    assert_eq!(lis[0].style().get_property_value("transition-delay").unwrap(), "120ms", "закрыто: задержка с конца");
    assert!(!list.style().get_property_value("--item-diff-x").unwrap().is_empty(), "смещения посчитаны после монтирования");
    q(&host, ".p-speeddial-button").click();
    settle().await;
    assert_eq!(all(&host, ".p-speeddial-item")[3].style().get_property_value("transition-delay").unwrap(), "120ms");
    assert_eq!(
        all(&host, ".p-speeddial-item")[0].style().get_property_value("transition-delay").unwrap(),
        "0ms",
        "открыто: задержка с начала"
    );
    host.remove();
}

async fn templates_and_custom_toggler() {
    let (host, log) = (host(), Log::default());
    let items = model(&log);
    let seen = log.clone();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        let seen = seen.clone();
        view! {
            <SpeedDial
                id="x"
                model=items.clone()
                button_content=Callback::new(move |toggle: Callback<web_sys::MouseEvent>| {
                    view! { <button class="my-toggler" on:click=move |e| toggle.run(e)>"T"</button> }.into_any()
                })
                item_content=Callback::new(move |ctx: SpeedDialItemContext| {
                    let (click, label) = (ctx.click, ctx.item.label.unwrap_or_default());
                    let seen = seen.clone();
                    view! { <b class="my-item" on:click=move |e| { seen.lock().unwrap().push(format!("tpl:{label}")); click.run(e.into()) }>{ctx.index}</b> }.into_any()
                })
            />
        }
    });
    settle().await;
    assert!(host.query_selector(".p-speeddial-button").unwrap().is_none());
    q(&host, ".my-toggler").click();
    settle().await;
    assert!(is_open(&host), "обратный вызов переключения");
    all(&host, ".my-item")[2].click();
    settle().await;
    assert!(!is_open(&host), "click из контекста скрывает список");
    assert_eq!(*log.lock().unwrap(), ["tpl:c", "c"], "и вызывает команду пункта");
    host.remove();
}

async fn unmount_removes_the_outside_listener() {
    let (host, log) = (host(), Log::default());
    let items = model(&log);
    let hidden = log.clone();
    let handle = leptos::mount::mount_to(host.clone(), move || {
        let hidden = hidden.clone();
        view! { <SpeedDial id="u" model=items.clone() on_hide=Callback::new(move |()| hidden.lock().unwrap().push("hide".into())) /> }
    });
    settle().await;
    q(&host, ".p-speeddial-button").click();
    settle().await;
    drop(handle);
    settle().await;
    doc().body().unwrap().click();
    settle().await;
    assert!(log.lock().unwrap().is_empty(), "после размонтирования слушатель снят");
    host.remove();
}

#[wasm_bindgen_test]
async fn speeddial_behaviour_in_a_real_browser() {
    toggle_events_and_item_commands().await;
    outside_click_escape_disabled_and_mask().await;
    keyboard_in_every_direction().await;
    toggler_up_arrow_focuses_the_last_item_and_unhandled_keys_pass().await;
    controlled_visible_disabled_and_hidden_items().await;
    layout_styles_and_offsets().await;
    templates_and_custom_toggler().await;
    unmount_removes_the_outside_listener().await;
}
