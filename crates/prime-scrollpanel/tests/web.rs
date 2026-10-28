//! Поведение ScrollPanel в реальном браузере (wasm-bindgen-test): скрытие и размер полос, перетаскивание, клавиатура,
//! `scroll_top` и снятие слушателей при размонтировании.
#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use prime_core::Pt;
use prime_scrollpanel::{ScrollPanel, ScrollPanelPt, ScrollPanelRef};
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use web_sys::{HtmlElement, KeyboardEvent, KeyboardEventInit, MouseEvent, MouseEventInit};

wasm_bindgen_test_configure!(run_in_browser);

fn doc() -> web_sys::Document {
    web_sys::window().unwrap().document().unwrap()
}

fn host() -> HtmlElement {
    // Минимум CSS темы, без которого содержимое не прокручивается.
    let style = doc().create_element("style").unwrap();
    style.set_text_content(Some(".p-scrollpanel{position:relative}.p-scrollpanel-content-container{height:100%;width:100%;float:left}.p-scrollpanel-content{height:100%;overflow:auto}"));
    doc().head().unwrap().append_child(&style).unwrap();
    let el = doc().create_element("div").unwrap().dyn_into::<HtmlElement>().unwrap();
    doc().body().unwrap().append_child(&el).unwrap();
    el
}

fn q(host: &HtmlElement, sel: &str) -> HtmlElement {
    host.query_selector(sel).unwrap().unwrap_or_else(|| panic!("нет {sel}")).dyn_into().unwrap()
}

async fn sleep(ms: i32) {
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms).unwrap();
    });
    let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
}

fn key(target: &HtmlElement, kind: &str, code: &str) {
    let init = KeyboardEventInit::new();
    init.set_code(code);
    init.set_bubbles(true);
    init.set_cancelable(true);
    target.dispatch_event(&KeyboardEvent::new_with_keyboard_event_init_dict(kind, &init).unwrap()).unwrap();
}

fn mouse(target: &web_sys::EventTarget, kind: &str, y: i32) {
    let init = MouseEventInit::new();
    init.set_client_y(y);
    init.set_bubbles(true);
    init.set_cancelable(true);
    target.dispatch_event(&MouseEvent::new_with_mouse_event_init_dict(kind, &init).unwrap()).unwrap();
}

/// Панель 100×100 с содержимым высотой `inner` пикселей.
fn panel(host: &HtmlElement, inner: u32, panel_ref: ScrollPanelRef) -> impl Drop {
    let pt = ScrollPanelPt { content: Pt::new().style("box-sizing:border-box"), ..Default::default() };
    leptos::mount::mount_to(host.clone(), move || {
        view! {
            <ScrollPanel style="width:100px;height:100px" step=10.0 panel_ref=panel_ref pt=pt.clone()>
                <div style=format!("height:{inner}px;width:50px")></div>
            </ScrollPanel>
        }
    })
}

#[wasm_bindgen_test]
async fn bars_are_hidden_when_content_fits() {
    let host = host();
    let _h = panel(&host, 50, ScrollPanelRef::new());
    leptos::task::tick().await;
    sleep(80).await;
    let y = q(&host, ".p-scrollpanel-bar-y");
    assert_eq!(y.get_attribute("data-p-scrollpanel-hidden").as_deref(), Some("true"));
    assert!(y.class_name().contains("p-scrollpanel-hidden"));
}

#[wasm_bindgen_test]
async fn bar_size_and_position_follow_scroll() {
    let host = host();
    let _h = panel(&host, 500, ScrollPanelRef::new());
    leptos::task::tick().await;
    sleep(80).await;
    let (y, content) = (q(&host, ".p-scrollpanel-bar-y"), q(&host, ".p-scrollpanel-content"));
    assert_eq!(y.get_attribute("data-p-scrollpanel-hidden").as_deref(), Some("false"));
    assert!(!y.class_name().contains("p-scrollpanel-hidden"));
    let css = y.style().css_text();
    assert!(css.contains("height: 20%"), "{css}");
    let max = content.scroll_height() - content.client_height();
    content.set_scroll_top(max / 2);
    sleep(80).await;
    let css = y.style().css_text();
    assert!(css.contains("calc(40%"), "{css}");
    assert_eq!(y.get_attribute("aria-valuenow"), Some((max / 2).to_string()));
}

#[wasm_bindgen_test]
async fn scroll_top_is_clamped() {
    let host = host();
    let panel_ref = ScrollPanelRef::new();
    let _h = panel(&host, 500, panel_ref);
    leptos::task::tick().await;
    sleep(80).await;
    let content = q(&host, ".p-scrollpanel-content");
    panel_ref.scroll_top(100000.0);
    assert_eq!(content.scroll_top(), content.scroll_height() - content.client_height());
    panel_ref.scroll_top(-5.0);
    assert_eq!(content.scroll_top(), 0);
    panel_ref.refresh();
}

#[wasm_bindgen_test]
async fn arrow_keys_scroll_by_step_after_delay_and_keyup_cancels() {
    let host = host();
    let _h = panel(&host, 500, ScrollPanelRef::new());
    leptos::task::tick().await;
    sleep(80).await;
    let (y, content) = (q(&host, ".p-scrollpanel-bar-y"), q(&host, ".p-scrollpanel-content"));
    y.focus().unwrap();
    key(&y, "keydown", "ArrowDown");
    assert_eq!(content.scroll_top(), 0, "прокрутка не мгновенная");
    sleep(120).await;
    assert_eq!(content.scroll_top(), 10);
    key(&y, "keydown", "ArrowUp");
    key(&y, "keyup", "ArrowUp");
    sleep(120).await;
    assert_eq!(content.scroll_top(), 10, "keyup отменил таймер");
    key(&y, "keydown", "ArrowUp");
    sleep(120).await;
    assert_eq!(content.scroll_top(), 0);
}

#[wasm_bindgen_test]
async fn dragging_the_bar_scrolls_and_mouseup_releases() {
    let host = host();
    let _h = panel(&host, 500, ScrollPanelRef::new());
    leptos::task::tick().await;
    sleep(80).await;
    let (y, content) = (q(&host, ".p-scrollpanel-bar-y"), q(&host, ".p-scrollpanel-content"));
    mouse(&y, "mousedown", 10);
    assert!(y.class_name().contains("p-scrollpanel-grabbed"));
    assert!(doc().body().unwrap().class_name().contains("p-scrollpanel-grabbed"));
    mouse(&doc(), "mousemove", 20);
    sleep(80).await;
    assert_eq!(content.scroll_top(), 50, "сдвиг 10 при доле 0.2 даёт 50");
    mouse(&doc(), "mouseup", 20);
    assert!(!y.class_name().contains("p-scrollpanel-grabbed"));
    assert!(!doc().body().unwrap().class_name().contains("p-scrollpanel-grabbed"));
    mouse(&doc(), "mousemove", 40);
    sleep(80).await;
    assert_eq!(content.scroll_top(), 50, "после mouseup слушатели документа сняты");
}

#[wasm_bindgen_test]
async fn unmount_releases_grab_and_listeners() {
    let host = host();
    let handle = panel(&host, 500, ScrollPanelRef::new());
    leptos::task::tick().await;
    sleep(80).await;
    let content = q(&host, ".p-scrollpanel-content");
    mouse(&q(&host, ".p-scrollpanel-bar-y"), "mousedown", 10);
    assert!(doc().body().unwrap().class_name().contains("p-scrollpanel-grabbed"));
    drop(handle);
    assert!(!doc().body().unwrap().class_name().contains("p-scrollpanel-grabbed"));
    mouse(&doc(), "mousemove", 30);
    sleep(80).await;
    assert_eq!(content.scroll_top(), 0, "слушатель документа снят");
}

#[wasm_bindgen_test]
async fn max_height_without_height_sizes_the_root_to_content() {
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! {
            <ScrollPanel style="max-height:200px">
                <div style="height:80px"></div>
            </ScrollPanel>
        }
    });
    leptos::task::tick().await;
    sleep(80).await;
    let root = q(&host, ".p-scrollpanel");
    assert_eq!(root.style().get_property_value("height").unwrap(), "80px");
}
