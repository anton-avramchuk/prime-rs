//! Поведение Popover в реальном браузере (wasm-bindgen-test): показ и скрытие, закрытие кликом снаружи и Escape,
//! смена цели, явная цель, `dismissable`, фокус, `resize`, `z-index`, события.
#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use prime_popover::{Popover, PopoverRef};
use std::sync::{Arc, Mutex};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;
use web_sys::{Event, HtmlElement, KeyboardEvent, KeyboardEventInit};

wasm_bindgen_test_configure!(run_in_browser);

type Events = Arc<Mutex<Vec<&'static str>>>;

fn doc() -> web_sys::Document {
    web_sys::window().unwrap().document().unwrap()
}

fn host() -> HtmlElement {
    let el = doc().create_element("div").unwrap().dyn_into::<HtmlElement>().unwrap();
    el.style().set_property("position", "relative").unwrap();
    doc().body().unwrap().append_child(&el).unwrap();
    el
}

fn q(host: &HtmlElement, sel: &str) -> HtmlElement {
    host.query_selector(sel).unwrap().unwrap_or_else(|| panic!("нет {sel}")).dyn_into().unwrap()
}

fn has(host: &HtmlElement, sel: &str) -> bool {
    host.query_selector(sel).unwrap().is_some()
}

async fn sleep(ms: i32) {
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms).unwrap();
    });
    JsFuture::from(promise).await.unwrap();
}

fn escape() {
    let init = KeyboardEventInit::new();
    init.set_key("Escape");
    init.set_bubbles(true);
    let event = KeyboardEvent::new_with_keyboard_event_init_dict("keydown", &init).unwrap();
    web_sys::window().unwrap().dispatch_event(&event).unwrap();
}

/// Две кнопки-триггера, нейтральный блок и панель с полем `autofocus` и текстом.
fn mount(host: &HtmlElement, dismissable: bool, events: Events) -> PopoverRef {
    let slot = Arc::new(Mutex::new(None));
    let s = slot.clone();
    let (a, b) = (events.clone(), events);
    let handle = leptos::mount::mount_to(host.clone(), move || {
        let op = PopoverRef::new();
        *s.lock().unwrap() = Some(op);
        let on_show = Callback::new(move |()| a.lock().unwrap().push("show"));
        let on_hide = Callback::new(move |()| b.lock().unwrap().push("hide"));
        view! {
            <button id="a" on:click=move |e| op.toggle(&e)>"A"</button>
            <button id="b" on:click=move |e| op.toggle(&e)>"B"</button>
            <div id="outside">"outside"</div>
            <Popover popover_ref=op dismissable=dismissable on_show=on_show on_hide=on_hide aria_label="Share">
                <input id="field" autofocus />
                <p id="body">"Body"</p>
            </Popover>
        }
    });
    std::mem::forget(handle);
    slot.lock().unwrap().take().unwrap()
}

async fn toggle_shows_and_hides_the_dialog() {
    let host = host();
    mount(&host, true, Default::default());
    leptos::task::tick().await;
    assert!(!has(&host, ".p-popover"));
    q(&host, "#a").click();
    leptos::task::tick().await;
    let panel = q(&host, ".p-popover");
    assert_eq!(panel.get_attribute("role").as_deref(), Some("dialog"));
    assert_eq!(panel.get_attribute("aria-modal").as_deref(), Some("true"));
    assert_eq!(panel.get_attribute("aria-label").as_deref(), Some("Share"));
    assert!(panel.class_name().contains("p-component"));
    assert!(q(&panel, ".p-popover-content #body").text_content().unwrap().contains("Body"));
    q(&host, "#a").click();
    leptos::task::tick().await;
    assert!(!has(&host, ".p-popover"));
    host.remove();
}

async fn click_outside_hides_but_inside_does_not() {
    let host = host();
    mount(&host, true, Default::default());
    q(&host, "#a").click();
    leptos::task::tick().await;
    sleep(20).await;
    q(&host, "#body").click();
    leptos::task::tick().await;
    assert!(has(&host, ".p-popover"), "клик внутри не закрывает");
    q(&host, "#outside").click();
    leptos::task::tick().await;
    assert!(!has(&host, ".p-popover"), "клик снаружи закрывает");
    host.remove();
}

async fn the_opening_click_does_not_close_the_panel() {
    let host = host();
    mount(&host, true, Default::default());
    q(&host, "#a").click();
    sleep(30).await;
    assert!(has(&host, ".p-popover"));
    host.remove();
}

async fn dismissable_false_ignores_outside_click_but_escape_hides() {
    let host = host();
    mount(&host, false, Default::default());
    q(&host, "#a").click();
    leptos::task::tick().await;
    sleep(20).await;
    q(&host, "#outside").click();
    leptos::task::tick().await;
    assert!(has(&host, ".p-popover"));
    escape();
    leptos::task::tick().await;
    assert!(!has(&host, ".p-popover"));
    host.remove();
}

async fn toggling_from_another_target_keeps_the_panel_open() {
    let host = host();
    mount(&host, true, Default::default());
    q(&host, "#a").click();
    leptos::task::tick().await;
    sleep(20).await;
    q(&host, "#b").click();
    sleep(30).await;
    assert!(has(&host, ".p-popover"), "панель остаётся у новой цели");
    q(&host, "#b").click();
    leptos::task::tick().await;
    assert!(!has(&host, ".p-popover"));
    host.remove();
}

async fn explicit_target_aligns_the_panel_to_it() {
    let host = host();
    let op = mount(&host, true, Default::default());
    let target = q(&host, "#outside");
    target.style().set_property("margin-top", "40px").unwrap();
    op.show_at(&target);
    leptos::task::tick().await;
    let panel = q(&host, ".p-popover");
    let top: f64 = panel.style().get_property_value("top").unwrap().trim_end_matches("px").parse().unwrap();
    assert!(top >= f64::from(target.offset_top()), "панель под явной целью: {top}");
    op.hide();
    leptos::task::tick().await;
    assert!(!has(&host, ".p-popover"));
    host.remove();
}

async fn autofocus_element_gets_focus_and_z_index_is_set() {
    let host = host();
    mount(&host, true, Default::default());
    q(&host, "#a").click();
    leptos::task::tick().await;
    sleep(30).await;
    assert!(q(&host, "#field").is_same_node(doc().active_element().as_deref()));
    let z: i32 = q(&host, ".p-popover").style().get_property_value("z-index").unwrap().parse().unwrap();
    assert!(z > 0, "z-index {z}");
    host.remove();
}

async fn resize_hides_the_panel() {
    let host = host();
    mount(&host, true, Default::default());
    q(&host, "#a").click();
    leptos::task::tick().await;
    sleep(20).await;
    web_sys::window().unwrap().dispatch_event(&Event::new("resize").unwrap()).unwrap();
    leptos::task::tick().await;
    let touch = web_sys::window().unwrap().navigator().max_touch_points() > 0;
    assert_eq!(has(&host, ".p-popover"), touch);
    host.remove();
}

async fn show_then_hide_fire_events_once_in_order() {
    let events = Events::default();
    let host = host();
    mount(&host, true, events.clone());
    q(&host, "#a").click();
    leptos::task::tick().await;
    sleep(20).await;
    q(&host, "#a").click();
    leptos::task::tick().await;
    assert_eq!(*events.lock().unwrap(), ["show", "hide"]);
    host.remove();
}

async fn listeners_are_removed_after_hide() {
    let events = Events::default();
    let host = host();
    mount(&host, true, events.clone());
    q(&host, "#a").click();
    leptos::task::tick().await;
    sleep(20).await;
    q(&host, "#a").click();
    leptos::task::tick().await;
    escape();
    q(&host, "#outside").click();
    leptos::task::tick().await;
    assert_eq!(*events.lock().unwrap(), ["show", "hide"], "после скрытия слушатели молчат");
    host.remove();
}

async fn panel_flips_above_a_target_at_the_bottom_edge() {
    let host = host();
    mount(&host, true, Default::default());
    let a = q(&host, "#a");
    a.style().set_property("position", "fixed").unwrap();
    a.style().set_property("bottom", "0").unwrap();
    a.click();
    leptos::task::tick().await;
    sleep(20).await;
    let panel = q(&host, ".p-popover");
    assert!(panel.class_name().contains("p-popover-flipped"), "{}", panel.class_name());
    assert_eq!(panel.get_attribute("data-p-popover-flipped").as_deref(), Some("true"));
    assert!(!panel.style().get_property_value("--p-popover-arrow-left").unwrap().is_empty());
    a.click();
    leptos::task::tick().await;
    host.remove();
}

async fn scrolling_an_ancestor_of_the_target_hides_the_panel() {
    let host = host();
    mount(&host, true, Default::default());
    let a = q(&host, "#a");
    let scroller = doc().create_element("div").unwrap().dyn_into::<HtmlElement>().unwrap();
    scroller.style().set_property("overflow", "auto").unwrap();
    scroller.style().set_property("height", "40px").unwrap();
    host.insert_before(&scroller, Some(&a)).unwrap();
    scroller.append_child(&a).unwrap();
    a.click();
    leptos::task::tick().await;
    sleep(20).await;
    assert!(has(&host, ".p-popover"));
    scroller.dispatch_event(&Event::new("scroll").unwrap()).unwrap();
    leptos::task::tick().await;
    assert!(!has(&host, ".p-popover"), "прокрутка предка цели скрывает панель");
    host.remove();
}

/// Сценарии идут по очереди: слушатели `window` у панелей общие, и параллельные тесты закрывали бы чужие панели.
#[wasm_bindgen_test]
async fn popover_behaviour() {
    toggle_shows_and_hides_the_dialog().await;
    click_outside_hides_but_inside_does_not().await;
    the_opening_click_does_not_close_the_panel().await;
    dismissable_false_ignores_outside_click_but_escape_hides().await;
    toggling_from_another_target_keeps_the_panel_open().await;
    explicit_target_aligns_the_panel_to_it().await;
    autofocus_element_gets_focus_and_z_index_is_set().await;
    resize_hides_the_panel().await;
    show_then_hide_fire_events_once_in_order().await;
    listeners_are_removed_after_hide().await;
    panel_flips_above_a_target_at_the_bottom_edge().await;
    scrolling_an_ancestor_of_the_target_hides_the_panel().await;
}
