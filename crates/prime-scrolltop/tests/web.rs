//! Поведение ScrollTop в реальном браузере (wasm-bindgen-test): порог, возврат, начальная проверка, клик, родитель,
//! положение, z-index и снятие подписки.
#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use prime_scrolltop::{ScrollTop, ScrollTopBehavior, ScrollTopTarget};
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use web_sys::HtmlElement;

wasm_bindgen_test_configure!(run_in_browser);

fn doc() -> web_sys::Document {
    web_sys::window().unwrap().document().unwrap()
}

fn host() -> HtmlElement {
    let el = doc().create_element("div").unwrap().dyn_into::<HtmlElement>().unwrap();
    doc().body().unwrap().append_child(&el).unwrap();
    el
}

/// Высокая страница, чтобы окно прокручивалось.
fn spacer() -> HtmlElement {
    let el = host();
    el.style().set_property("height", "4000px").unwrap();
    el
}

async fn sleep(ms: i32) {
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms).unwrap();
    });
    let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
}

fn scroll_window(y: i32) {
    web_sys::window().unwrap().scroll_to_with_x_and_y(0.0, f64::from(y));
}

fn button(root: &impl AsRef<web_sys::Element>) -> Option<HtmlElement> {
    root.as_ref().query_selector("button.p-scrolltop").unwrap().map(|b| b.dyn_into().unwrap())
}

fn window_button() -> Option<HtmlElement> {
    doc().query_selector("body > div > button.p-scrolltop").unwrap().map(|b| b.dyn_into().unwrap())
}

fn style_of(el: &HtmlElement, prop: &str) -> String {
    web_sys::window().unwrap().get_computed_style(el).unwrap().unwrap().get_property_value(prop).unwrap()
}

#[wasm_bindgen_test]
async fn window_shows_above_threshold_and_hides_on_return() {
    let tall = spacer();
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), || view! { <ScrollTop /> });
    leptos::task::tick().await;
    scroll_window(400);
    sleep(250).await;
    assert!(window_button().is_none(), "на пороге кнопки нет");
    scroll_window(401);
    sleep(250).await;
    let b = window_button().expect("выше порога кнопка показана");
    assert!(b.get_attribute("aria-label").as_deref() == Some("Scroll Top"));
    assert!(b.query_selector(".p-scrolltop-icon").unwrap().is_some());
    let z: i32 = b.style().get_property_value("z-index").unwrap().parse().expect("z-index выдан");
    assert!(z > 1000, "z-index {z} выше базы Overlay");
    assert_eq!(prime_core::zindex::current(), z);
    scroll_window(0);
    sleep(800).await;
    assert!(window_button().is_none(), "после возврата кнопка скрыта");
    assert_eq!(prime_core::zindex::current(), 0, "z-index возвращён");
    tall.remove();
    host.remove();
}

#[wasm_bindgen_test]
async fn already_scrolled_page_shows_button_on_mount() {
    let tall = spacer();
    scroll_window(800);
    sleep(50).await;
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), || view! { <ScrollTop /> });
    sleep(250).await;
    assert!(window_button().is_some(), "кнопка показана без нового scroll");
    scroll_window(0);
    sleep(800).await;
    tall.remove();
    host.remove();
}

#[wasm_bindgen_test]
async fn click_scrolls_window_to_top_and_hides_button() {
    let tall = spacer();
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), || view! { <ScrollTop behavior=ScrollTopBehavior::Auto /> });
    leptos::task::tick().await;
    scroll_window(900);
    sleep(250).await;
    window_button().expect("показана").click();
    sleep(800).await;
    assert_eq!(web_sys::window().unwrap().scroll_y().unwrap(), 0.0);
    assert!(window_button().is_none());
    tall.remove();
    host.remove();
}

#[wasm_bindgen_test]
async fn custom_threshold() {
    let tall = spacer();
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), || view! { <ScrollTop threshold=100.0 /> });
    leptos::task::tick().await;
    scroll_window(150);
    sleep(250).await;
    assert!(window_button().is_some());
    scroll_window(0);
    sleep(800).await;
    tall.remove();
    host.remove();
}

#[wasm_bindgen_test]
async fn parent_target_follows_parent_scroll() {
    let scroller = host();
    scroller.style().set_property("height", "100px").unwrap();
    scroller.style().set_property("overflow", "auto").unwrap();
    let content = doc().create_element("div").unwrap();
    content.set_attribute("style", "height: 1000px").unwrap();
    scroller.append_child(&content).unwrap();
    let _h = leptos::mount::mount_to(scroller.clone(), || {
        view! { <ScrollTop target=ScrollTopTarget::Parent threshold=100.0 behavior=ScrollTopBehavior::Auto /> }
    });
    sleep(250).await;
    let b = button(&scroller).expect("у родителя кнопка в DOM");
    assert_eq!(style_of(&b, "display"), "none", "ниже порога спрятана");
    assert!(b.class_name().contains("p-scrolltop-sticky"));
    scroller.set_scroll_top(150);
    sleep(250).await;
    assert_ne!(style_of(&b, "display"), "none", "выше порога видна");
    b.click();
    sleep(800).await;
    assert_eq!(scroller.scroll_top(), 0, "клик прокрутил родителя");
    assert_eq!(style_of(&b, "display"), "none");
    assert_eq!(prime_core::zindex::current(), 0);
    scroller.remove();
}

#[wasm_bindgen_test]
async fn unmount_removes_listener_and_releases_z_index() {
    let tall = spacer();
    let host = host();
    let handle = leptos::mount::mount_to(host.clone(), || view! { <ScrollTop /> });
    leptos::task::tick().await;
    scroll_window(600);
    sleep(250).await;
    assert!(window_button().is_some());
    assert_ne!(prime_core::zindex::current(), 0);
    drop(handle);
    sleep(20).await;
    assert_eq!(prime_core::zindex::current(), 0, "z-index возвращён при уничтожении");
    scroll_window(0);
    sleep(50).await;
    scroll_window(700);
    sleep(250).await;
    assert!(window_button().is_none(), "после уничтожения подписки нет");
    scroll_window(0);
    tall.remove();
    host.remove();
}
