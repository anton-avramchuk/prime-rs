//! Тесты в реальном браузере (wasm-bindgen-test).
#![cfg(target_arch = "wasm32")]

use prime_dom::dom::{document, element_size};
use prime_dom::{AlignMode, FocusTrap, Origin, Ripple, align_overlay};
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use web_sys::{HtmlElement, MouseEvent, MouseEventInit};

wasm_bindgen_test_configure!(run_in_browser);

fn make(tag: &str, style: &str) -> HtmlElement {
    let doc = document().unwrap();
    let el = doc.create_element(tag).unwrap().dyn_into::<HtmlElement>().unwrap();
    el.set_attribute("style", style).unwrap();
    doc.body().unwrap().append_child(&el).unwrap();
    el
}

fn px(el: &HtmlElement, prop: &str) -> f64 {
    el.style().get_property_value(prop).unwrap().trim_end_matches("px").parse().unwrap()
}

#[wasm_bindgen_test]
fn overlay_below_target_when_room() {
    let target = make("div", "position:absolute;top:100px;left:20px;width:100px;height:30px");
    let overlay = make("div", "position:absolute;width:200px;height:150px");
    let p = align_overlay(&overlay, &target, AlignMode::Absolute, false).unwrap();
    assert_eq!(p.origin, Origin::Top);
    assert!(overlay.style().get_property_value("transform-origin").unwrap().ends_with("top"));
    assert_eq!(px(&overlay, "top"), 130.0);
    assert_eq!(px(&overlay, "left"), 20.0);
    target.remove();
    overlay.remove();
}

#[wasm_bindgen_test]
fn overlay_flips_above_when_no_room_below() {
    let win = web_sys::window().unwrap();
    let vh = win.inner_height().unwrap().as_f64().unwrap();
    let target = make("div", &format!("position:fixed;top:{}px;left:20px;width:100px;height:30px", vh - 40.0));
    let overlay = make("div", "position:fixed;width:200px;height:150px");
    let p = align_overlay(&overlay, &target, AlignMode::Absolute, false).unwrap();
    assert_eq!(p.origin, Origin::Bottom);
    assert!(overlay.style().get_property_value("transform-origin").unwrap().ends_with("bottom"));
    assert!(px(&overlay, "top") < vh - 40.0);
    target.remove();
    overlay.remove();
}

#[wasm_bindgen_test]
fn hidden_element_is_measured_and_restored() {
    let el = make("div", "display:none;width:50px;height:20px");
    let size = element_size(&el);
    assert_eq!((size.width, size.height), (50.0, 20.0));
    assert_eq!(el.style().get_property_value("display").unwrap(), "none");
    el.remove();
}

#[wasm_bindgen_test]
fn focus_trap_wraps_from_last_to_first_and_back() {
    let container = make("div", "");
    container.set_inner_html(r#"<button id="a">a</button><button id="b">b</button><button id="c">c</button>"#);
    let doc = document().unwrap();
    let by_id = |id: &str| doc.get_element_by_id(id).unwrap().dyn_into::<HtmlElement>().unwrap();
    let trap = FocusTrap::attach(&container).unwrap();

    by_id("c").focus().unwrap();
    // Tab с последнего элемента попадает на последний служебный элемент.
    trap.last_sentinel().focus().unwrap();
    assert_eq!(doc.active_element().unwrap().id(), "a");

    // Shift+Tab с первого элемента попадает на первый служебный элемент.
    trap.first_sentinel().focus().unwrap();
    assert_eq!(doc.active_element().unwrap().id(), "c");

    drop(trap);
    assert_eq!(container.children().length(), 3, "служебные элементы удалены");
    container.remove();
}

#[wasm_bindgen_test]
fn ripple_activates_on_mousedown_and_cleans_up() {
    let host = make("button", "position:relative;width:100px;height:40px");
    let ripple = Ripple::attach(&host).unwrap();
    assert!(host.class_list().contains("p-ripple"));
    let init = MouseEventInit::new();
    let ev = MouseEvent::new_with_mouse_event_init_dict("mousedown", &init).unwrap();
    host.dispatch_event(&ev).unwrap();
    assert_eq!(ripple.ink().get_attribute("data-p-ink-active").as_deref(), Some("true"));
    assert!(ripple.ink().class_list().contains("p-ink-active"));
    drop(ripple);
    assert_eq!(host.children().length(), 0);
    host.remove();
}

#[wasm_bindgen_test]
async fn portal_renders_into_body_and_is_removed_on_unmount() {
    use leptos::prelude::{ElementChild, GlobalAttributes};
    use leptos::view;
    use prime_dom::Portal;

    let doc = document().unwrap();
    let host = make("div", "");
    let body = doc.body().unwrap();
    let handle = leptos::mount::mount_to(host.clone(), move || {
        view! { <Portal mount=body.clone()><div id="portal-content">"hi"</div></Portal> }
    });

    leptos::task::tick().await;
    let found = doc.get_element_by_id("portal-content").expect("содержимое портала в документе");
    assert!(!host.contains(Some(&found)), "содержимое вынесено из host");
    assert_eq!(found.parent_element().unwrap().tag_name(), "DIV");
    assert!(doc.body().unwrap().contains(Some(&found)));

    drop(handle);
    leptos::task::tick().await;
    assert!(doc.get_element_by_id("portal-content").is_none(), "после размонтирования содержимое удалено");
    host.remove();
}

#[wasm_bindgen_test]
fn body_scroll_lock_is_counted() {
    let body = web_sys::window().unwrap().document().unwrap().body().unwrap();
    let locked = || body.class_list().contains("p-overflow-hidden");
    assert!(!locked());
    prime_dom::block_body_scroll();
    prime_dom::block_body_scroll();
    assert!(locked());
    assert!(!body.style().get_property_value("--p-scrollbar-width").unwrap().is_empty());
    prime_dom::unblock_body_scroll();
    assert!(locked(), "вторая блокировка ещё держит класс");
    prime_dom::unblock_body_scroll();
    assert!(!locked());
    assert!(body.style().get_property_value("--p-scrollbar-width").unwrap().is_empty());
    prime_dom::unblock_body_scroll();
    assert!(!locked(), "лишнее снятие безвредно");
}
