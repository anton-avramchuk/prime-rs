//! Поведение Galleria в реальном браузере (wasm-bindgen-test): навигация по кадрам, индикаторам и миниатюрам,
//! клавиши, автосмена, полноэкранный режим. Сценарии идут одним последовательным тестом. Стили темы не подключены.
#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use prime_galleria::{Galleria, GalleriaImage};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;
use web_sys::{HtmlElement, KeyboardEvent, KeyboardEventInit, MouseEvent, MouseEventInit};

wasm_bindgen_test_configure!(run_in_browser);

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

fn images(n: usize) -> Vec<GalleriaImage> {
    (0..n).map(|i| GalleriaImage::new(format!("big{i}.jpg")).thumbnail(format!("small{i}.jpg")).alt(format!("Alt {i}"))).collect()
}

fn q(root: &impl AsRef<web_sys::Element>, selector: &str) -> Option<HtmlElement> {
    root.as_ref().query_selector(selector).unwrap().map(|e| e.dyn_into().unwrap())
}

fn must(root: &impl AsRef<web_sys::Element>, selector: &str) -> HtmlElement {
    q(root, selector).unwrap_or_else(|| panic!("нет {selector}: {}", root.as_ref().outer_html()))
}

fn all(root: &impl AsRef<web_sys::Element>, selector: &str) -> Vec<HtmlElement> {
    let list = root.as_ref().query_selector_all(selector).unwrap();
    (0..list.length()).map(|i| list.item(i).unwrap().dyn_into().unwrap()).collect()
}

fn shown_src(root: &impl AsRef<web_sys::Element>) -> String {
    must(root, ".p-galleria-item img").get_attribute("src").unwrap()
}

fn key(el: &HtmlElement, code: &str) -> bool {
    let init = KeyboardEventInit::new();
    init.set_code(code);
    init.set_bubbles(true);
    init.set_cancelable(true);
    el.dispatch_event(&KeyboardEvent::new_with_keyboard_event_init_dict("keydown", &init).unwrap()).unwrap()
}

fn mouse(el: &web_sys::EventTarget, kind: &str) {
    let init = MouseEventInit::new();
    init.set_bubbles(true);
    init.set_cancelable(true);
    el.dispatch_event(&MouseEvent::new_with_mouse_event_init_dict(kind, &init).unwrap()).unwrap();
}

async fn item_navigation_and_indicators() {
    let host = host();
    let active = RwSignal::new(0usize);
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <Galleria value=images(4) active_index=active show_item_navigators=true show_indicators=true /> }
    });
    settle().await;
    assert_eq!(shown_src(&host), "big0.jpg");
    let next = must(&host, ".p-galleria-next-button");
    let prev = must(&host, ".p-galleria-prev-button");
    assert!(prev.class_name().contains("p-disabled"));
    next.click();
    settle().await;
    assert_eq!((active.get_untracked(), shown_src(&host).as_str()), (1, "big1.jpg"));
    assert!(!prev.class_name().contains("p-disabled"));
    next.click();
    next.click();
    settle().await;
    assert_eq!(active.get_untracked(), 3);
    assert!(next.class_name().contains("p-disabled"), "в конце без circular «вперёд» недоступна");
    next.click();
    settle().await;
    assert_eq!(active.get_untracked(), 3, "дальше последнего не идёт");
    prev.click();
    settle().await;
    assert_eq!(active.get_untracked(), 2);

    let indicators = all(&host, ".p-galleria-indicator");
    assert_eq!(indicators.len(), 4);
    assert!(indicators[2].class_name().contains("p-galleria-indicator-active"));
    indicators[0].click();
    settle().await;
    assert_eq!(active.get_untracked(), 0);
    assert!(
        indicators[0].class_name().contains("p-galleria-indicator-active")
            && !indicators[2].class_name().contains("p-galleria-indicator-active")
    );
    assert!(!key(&indicators[3], "Enter"), "Enter на индикаторе поглощается");
    settle().await;
    assert_eq!(active.get_untracked(), 3);
    key(&indicators[1], "Space");
    settle().await;
    assert_eq!(active.get_untracked(), 1);
    host.remove();
}

async fn circular_and_hover_indicators() {
    let host = host();
    let active = RwSignal::new(0usize);
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <Galleria value=images(3) active_index=active show_item_navigators=true circular=true show_indicators=true change_item_on_indicator_hover=true /> }
    });
    settle().await;
    must(&host, ".p-galleria-prev-button").click();
    settle().await;
    assert_eq!(active.get_untracked(), 2, "circular: назад с первого — на последний");
    must(&host, ".p-galleria-next-button").click();
    settle().await;
    assert_eq!(active.get_untracked(), 0, "circular: вперёд с последнего — на первый");
    let indicators = all(&host, ".p-galleria-indicator");
    mouse(&indicators[2], "mouseenter");
    settle().await;
    assert_eq!(active.get_untracked(), 2, "наведение меняет кадр");
    host.remove();
}

async fn thumbnails_click_keys_and_buttons() {
    let host = host();
    let active = RwSignal::new(0usize);
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Galleria value=images(6) active_index=active num_visible=3 /> });
    settle().await;
    let thumbs = all(&host, ".p-galleria-thumbnail");
    assert_eq!(thumbs.len(), 6);
    assert_eq!(thumbs[0].get_attribute("tabindex").as_deref(), Some("0"));
    assert_eq!(thumbs[1].get_attribute("tabindex").as_deref(), Some("-1"));
    thumbs[3].click();
    settle().await;
    assert_eq!((active.get_untracked(), shown_src(&host).as_str()), (3, "big3.jpg"));
    let ribbon = must(&host, ".p-galleria-thumbnail-items");
    assert!(ribbon.get_attribute("style").unwrap().contains("translate3d(-66.66"), "лента сдвинулась: {:?}", ribbon.get_attribute("style"));
    let items = all(&host, ".p-galleria-thumbnail-item");
    assert!(items[3].class_name().contains("p-galleria-thumbnail-item-current"));
    assert!(
        items[2].class_name().contains("p-galleria-thumbnail-item-start")
            && items[4].class_name().contains("p-galleria-thumbnail-item-end")
    );

    // клавиши: стрелки двигают фокус, Enter выбирает
    key(&items[3], "ArrowRight");
    settle().await;
    assert_eq!(thumbs[4].get_attribute("tabindex").as_deref(), Some("0"));
    assert_eq!(active.get_untracked(), 3, "стрелка только переносит фокус");
    key(&items[4], "Enter");
    settle().await;
    assert_eq!(active.get_untracked(), 4);
    key(&items[4], "Home");
    key(&items[0], "Enter");
    settle().await;
    assert_eq!(active.get_untracked(), 0);
    key(&items[0], "End");
    key(&items[5], "Space");
    settle().await;
    assert_eq!(active.get_untracked(), 5);

    // кнопки страниц
    let next = must(&host, ".p-galleria-thumbnail-next-button");
    let prev = must(&host, ".p-galleria-thumbnail-prev-button");
    assert!(next.class_name().contains("p-disabled"));
    prev.click();
    settle().await;
    assert_eq!(active.get_untracked(), 4);
    host.remove();
}

async fn autoplay_stops_on_user_action() {
    let host = host();
    let active = RwSignal::new(0usize);
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <Galleria value=images(5) active_index=active auto_play=true transition_interval=60 /> }
    });
    sleep(200).await;
    let moved = active.get_untracked();
    assert!(moved >= 2, "автосмена продвинулась: {moved}");
    all(&host, ".p-galleria-thumbnail")[0].click();
    settle().await;
    assert_eq!(active.get_untracked(), 0);
    sleep(250).await;
    assert_eq!(active.get_untracked(), 0, "клик остановил автосмену");
    host.remove();

    // не останавливать по клику
    let host = host_again();
    let active = RwSignal::new(0usize);
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <Galleria value=images(5) active_index=active auto_play=true transition_interval=60 should_stop_autoplay_by_click=false circular=true /> }
    });
    settle().await;
    all(&host, ".p-galleria-thumbnail")[0].click();
    // две-три смены из пяти кадров по кругу: больше — и круг замкнётся на 0
    sleep(130).await;
    assert!(active.get_untracked() > 0, "автосмена продолжается: {}", active.get_untracked());
    host.remove();
}

fn host_again() -> HtmlElement {
    host()
}

async fn full_screen_lifecycle() {
    let host = host();
    let visible = RwSignal::new(false);
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Galleria value=images(3) full_screen=true visible=visible /> });
    settle().await;
    let body = doc().body().unwrap();
    assert!(q(&body, ".p-galleria-mask").is_none(), "закрыта: маски нет");
    visible.set(true);
    sleep(100).await;
    let mask = must(&body, ".p-galleria-mask");
    assert_eq!(mask.get_attribute("role").as_deref(), Some("dialog"));
    assert!(mask.class_name().contains("p-overlay-mask-enter-active"));
    let root = must(&mask, ".p-galleria");
    assert!(root.class_name().contains("p-galleria-fullscreen"), "{}", root.class_name());
    assert!(
        body.style().get_property_value("overflow").unwrap_or_default().contains("hidden")
            || body.class_name().contains("p-overflow-hidden"),
        "прокрутка страницы заблокирована"
    );
    assert!(mask.style().get_property_value("z-index").unwrap().parse::<i32>().is_ok(), "слой назначен");

    // клик по содержимому не закрывает
    must(&mask, ".p-galleria-item").click();
    settle().await;
    assert!(visible.get_untracked());
    // клик по самой маске закрывает
    mouse(&mask, "click");
    settle().await;
    assert!(!visible.get_untracked());
    assert!(q(&body, ".p-galleria-mask").is_some(), "во время выхода маска ещё в DOM");
    assert!(must(&body, ".p-galleria").class_name().contains("p-galleria-leave-active"));
    sleep(900).await;
    assert!(q(&body, ".p-galleria-mask").is_none(), "после выхода убрана");

    // кнопка закрытия
    visible.set(true);
    sleep(100).await;
    let mask = must(&body, ".p-galleria-mask");
    must(&mask, ".p-galleria-close-button").click();
    settle().await;
    assert!(!visible.get_untracked());
    sleep(900).await;
    assert!(q(&body, ".p-galleria-mask").is_none());
    host.remove();
}

#[wasm_bindgen_test]
async fn galleria_in_the_browser() {
    item_navigation_and_indicators().await;
    circular_and_hover_indicators().await;
    thumbnails_click_keys_and_buttons().await;
    autoplay_stops_on_user_action().await;
    full_screen_lifecycle().await;
}
