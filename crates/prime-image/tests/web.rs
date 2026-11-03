//! Поведение Image в реальном браузере (wasm-bindgen-test): открытие и закрытие просмотра, панель действий, поворот и масштаб,
//! слой, прокрутка страницы, фокус, события, представления и ошибка загрузки.
//!
//! Сценарии идут одним последовательным тестом: стек слоёв `z-index` и счётчик блокировки прокрутки общие на страницу,
//! поэтому параллельные тесты влияли бы друг на друга. Стили темы в этих тестах не подключены: анимаций нет,
//! просмотр убирается по запасному таймеру.
#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use prime_image::{Image, ImagePreviewContext};
use std::sync::{Arc, Mutex};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;
use web_sys::{HtmlElement, KeyboardEvent, KeyboardEventInit};

wasm_bindgen_test_configure!(run_in_browser);

type Log = Arc<Mutex<Vec<&'static str>>>;

fn doc() -> web_sys::Document {
    web_sys::window().unwrap().document().unwrap()
}

fn host() -> HtmlElement {
    let el = doc().create_element("div").unwrap().dyn_into::<HtmlElement>().unwrap();
    doc().body().unwrap().append_child(&el).unwrap();
    el
}

/// Элемент в документе целиком: просмотр выводится в `body`, а не в хосте.
fn find(sel: &str) -> Option<HtmlElement> {
    doc().query_selector(sel).unwrap().and_then(|e| e.dyn_into().ok())
}

fn must(sel: &str) -> HtmlElement {
    find(sel).unwrap_or_else(|| panic!("нет {sel}"))
}

fn body_locked() -> bool {
    doc().body().unwrap().class_list().contains("p-overflow-hidden")
}

async fn sleep(ms: i32) {
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms).unwrap();
    });
    JsFuture::from(promise).await.unwrap();
}

/// Даёт эффектам и `Portal` вставить маску.
async fn settle() {
    sleep(60).await;
}

/// Ждёт окончания выхода по запасному таймеру.
async fn after_leave() {
    sleep(700).await;
}

fn escape() {
    let init = KeyboardEventInit::new();
    init.set_key("Escape");
    init.set_code("Escape");
    init.set_bubbles(true);
    init.set_cancelable(true);
    doc().dispatch_event(&KeyboardEvent::new_with_keyboard_event_init_dict("keydown", &init).unwrap()).unwrap();
}

fn transform() -> String {
    must(".p-image-original").style().get_property_value("transform").unwrap()
}

fn btn_disabled(sel: &str) -> bool {
    must(sel).has_attribute("disabled")
}

fn log_cb(log: &Log, name: &'static str) -> Callback<()> {
    let log = log.clone();
    Callback::new(move |()| log.lock().unwrap().push(name))
}

fn cleanup(host: &HtmlElement) {
    host.remove();
}

async fn plain_image_has_no_preview() {
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), || view! { <Image src="a.jpg" alt="x" /> });
    settle().await;
    assert!(host.query_selector(".p-image-preview-mask").unwrap().is_none());
    assert!(find(".p-image-mask").is_none());
    cleanup(&host);
}

async fn open_shows_mask_toolbar_and_locks_the_page() {
    let (host, log) = (host(), Log::default());
    let (l1, l2) = (log_cb(&log, "show"), log_cb(&log, "hide"));
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <Image src="small.jpg" preview_image_src="big.jpg" preview=true width="250" on_show=l1 on_hide=l2 /> }
    });
    settle().await;
    assert!(!body_locked());
    must(".p-image-preview-mask").click();
    settle().await;
    let mask = must(".p-image-mask");
    assert!(mask.class_name().contains("p-overlay-mask"), "{}", mask.class_name());
    assert_eq!(mask.get_attribute("role").as_deref(), Some("dialog"));
    assert_eq!(mask.get_attribute("aria-modal").as_deref(), Some("true"));
    assert!(mask.parent_element().unwrap().tag_name() != "P-IMAGE", "маска вне корня Image");
    let z: i32 = mask.style().get_property_value("z-index").unwrap().parse().unwrap();
    assert!(z >= 1100, "слой группы Modal: {z}");
    assert!(body_locked(), "прокрутка страницы заблокирована");
    let buttons = doc().query_selector_all(".p-image-toolbar button").unwrap();
    assert_eq!(buttons.length(), 5);
    let classes: Vec<String> = (0..5).map(|i| buttons.item(i).unwrap().dyn_into::<HtmlElement>().unwrap().class_name()).collect();
    for (class, needle) in classes.iter().zip(["rotate-right", "rotate-left", "zoom-out", "zoom-in", "close"]) {
        assert!(class.contains("p-image-action") && class.contains(needle), "{class} / {needle}");
    }
    assert_eq!(must(".p-image-rotate-right-button").get_attribute("aria-label").as_deref(), Some("Rotate Right"));
    assert_eq!(must(".p-image-close-button").get_attribute("aria-label").as_deref(), Some("Close"));
    assert_eq!(must("img.p-image-original").get_attribute("src").as_deref(), Some("big.jpg"), "источник просмотра");
    assert_eq!(*log.lock().unwrap(), ["show"]);
    sleep(80).await;
    let active = doc().active_element().unwrap();
    assert!(active.class_list().contains("p-image-close-button"), "фокус на кнопке закрытия: {}", active.class_name());
    must(".p-image-close-button").click();
    after_leave().await;
    assert!(find(".p-image-mask").is_none(), "маска убрана");
    assert!(!body_locked(), "прокрутка разблокирована");
    assert_eq!(*log.lock().unwrap(), ["show", "hide"]);
    cleanup(&host);
}

async fn preview_falls_back_to_src() {
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), || view! { <Image src="same.jpg" preview=true /> });
    settle().await;
    must(".p-image-preview-mask").click();
    settle().await;
    assert_eq!(must("img.p-image-original").get_attribute("src").as_deref(), Some("same.jpg"));
    must(".p-image-close-button").click();
    after_leave().await;
    cleanup(&host);
}

async fn rotate_and_zoom_change_transform_and_limits() {
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), || view! { <Image src="a.jpg" preview=true /> });
    settle().await;
    must(".p-image-preview-mask").click();
    settle().await;
    assert_eq!(transform(), "rotate(0deg) scale(1)");
    must(".p-image-rotate-right-button").click();
    settle().await;
    assert_eq!(transform(), "rotate(90deg) scale(1)");
    must(".p-image-rotate-left-button").click();
    must(".p-image-rotate-left-button").click();
    settle().await;
    assert_eq!(transform(), "rotate(-90deg) scale(1)");
    must(".p-image-zoom-in-button").click();
    settle().await;
    assert_eq!(transform(), "rotate(-90deg) scale(1.1)");
    assert!(find(".p-image-mask").is_some(), "клики по панели не закрывают");
    for _ in 0..3 {
        must(".p-image-zoom-in-button").click();
    }
    settle().await;
    assert_eq!(transform(), "rotate(-90deg) scale(1.4)");
    assert!(btn_disabled(".p-image-zoom-in-button") && must(".p-image-zoom-in-button").class_name().contains("p-disabled"));
    assert!(!btn_disabled(".p-image-zoom-out-button"));
    for _ in 0..8 {
        must(".p-image-zoom-out-button").click();
    }
    settle().await;
    assert_eq!(transform(), "rotate(-90deg) scale(0.6)");
    assert!(btn_disabled(".p-image-zoom-out-button") && must(".p-image-zoom-out-button").class_name().contains("p-disabled"));
    assert!(!btn_disabled(".p-image-zoom-in-button"));
    // повторное открытие начинается с нуля
    must(".p-image-close-button").click();
    after_leave().await;
    must(".p-image-preview-mask").click();
    settle().await;
    assert_eq!(transform(), "rotate(0deg) scale(1)", "поворот и масштаб сброшены");
    must(".p-image-close-button").click();
    after_leave().await;
    cleanup(&host);
}

async fn mask_click_closes_but_image_and_toolbar_clicks_do_not() {
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), || view! { <Image src="a.jpg" preview=true /> });
    settle().await;
    must(".p-image-preview-mask").click();
    settle().await;
    must("img.p-image-original").click();
    settle().await;
    assert!(find(".p-image-mask").is_some(), "клик по картинке не закрывает");
    must(".p-image-toolbar").click();
    settle().await;
    assert!(find(".p-image-mask").is_some(), "клик по панели не закрывает");
    must(".p-image-rotate-right-button").click();
    must(".p-image-mask").click();
    after_leave().await;
    assert!(find(".p-image-mask").is_none(), "клик по маске закрывает, даже после действия панели");
    assert!(!body_locked());
    cleanup(&host);
}

async fn escape_closes_and_returns_focus() {
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), || view! { <Image src="a.jpg" preview=true /> });
    settle().await;
    must(".p-image-preview-mask").click();
    settle().await;
    escape();
    sleep(80).await;
    let active = doc().active_element().unwrap();
    assert!(active.class_list().contains("p-image-preview-mask"), "фокус вернулся на кнопку просмотра: {}", active.class_name());
    after_leave().await;
    assert!(find(".p-image-mask").is_none());
    assert!(!body_locked());
    // после закрытия Escape ничего не делает и слушатель снят
    escape();
    settle().await;
    assert!(find(".p-image-mask").is_none());
    cleanup(&host);
}

async fn reopening_during_leave_keeps_a_single_scroll_lock() {
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), || view! { <Image src="a.jpg" preview=true /> });
    settle().await;
    must(".p-image-preview-mask").click();
    settle().await;
    must(".p-image-close-button").click();
    sleep(100).await;
    must(".p-image-preview-mask").click();
    sleep(100).await;
    assert!(find(".p-image-mask").is_some(), "маска осталась");
    must(".p-image-close-button").click();
    after_leave().await;
    assert!(find(".p-image-mask").is_none());
    assert!(!body_locked(), "прокрутка разблокирована ровно один раз");
    cleanup(&host);
}

async fn templates_replace_indicator_icons_and_preview() {
    let host = host();
    let seen = Arc::new(Mutex::new(Vec::<String>::new()));
    let seen2 = seen.clone();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        let seen = seen2.clone();
        view! {
            <Image
                src="a.jpg"
                preview=true
                indicator=ViewFn::from(|| view! { <i class="my-ind">"i"</i> })
                zoom_in_icon=ViewFn::from(|| view! { <b class="my-zoom">"+"</b> })
                preview_content=Callback::new(move |ctx: ImagePreviewContext| {
                    seen.lock().unwrap().push(ctx.class.clone());
                    let cb = ctx.preview_callback;
                    view! { <img class=ctx.class.clone() style=move || ctx.style.get() src="tpl.jpg" on:click=move |_| cb.run(()) /> }.into_any()
                })
            />
        }
    });
    settle().await;
    assert!(must(".p-image-preview-mask").query_selector(".my-ind").unwrap().is_some());
    must(".p-image-preview-mask").click();
    settle().await;
    assert!(find(".p-image-zoom-in-button .my-zoom").is_some());
    assert_eq!(must("img.p-image-original").get_attribute("src").as_deref(), Some("tpl.jpg"));
    assert!(seen.lock().unwrap()[0].contains("p-image-original"));
    must(".p-image-rotate-right-button").click();
    settle().await;
    assert_eq!(transform(), "rotate(90deg) scale(1)", "стиль из контекста следует за действиями");
    must("img.p-image-original").click();
    settle().await;
    assert!(find(".p-image-mask").is_some(), "preview_callback: клик по своей картинке не закрывает");
    must(".p-image-close-button").click();
    after_leave().await;
    cleanup(&host);
}

async fn image_error_event_is_reported() {
    let (host, log) = (host(), Log::default());
    let l = log.clone();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        let l = l.clone();
        view! { <Image src="/definitely-missing.jpg" on_image_error=Callback::new(move |_| l.lock().unwrap().push("error")) /> }
    });
    sleep(500).await;
    assert_eq!(*log.lock().unwrap(), ["error"]);
    cleanup(&host);
}

async fn unmount_while_open_releases_the_page() {
    let host = host();
    let handle = leptos::mount::mount_to(host.clone(), || view! { <Image src="a.jpg" preview=true /> });
    settle().await;
    must(".p-image-preview-mask").click();
    settle().await;
    assert!(body_locked());
    drop(handle);
    settle().await;
    assert!(!body_locked(), "блокировка снята при размонтировании");
    assert!(find(".p-image-mask").is_none());
    cleanup(&host);
}

#[wasm_bindgen_test]
async fn image_behaviour_in_a_real_browser() {
    plain_image_has_no_preview().await;
    open_shows_mask_toolbar_and_locks_the_page().await;
    preview_falls_back_to_src().await;
    rotate_and_zoom_change_transform_and_limits().await;
    mask_click_closes_but_image_and_toolbar_clicks_do_not().await;
    escape_closes_and_returns_focus().await;
    reopening_during_leave_keeps_a_single_scroll_lock().await;
    templates_replace_indicator_icons_and_preview().await;
    image_error_event_is_reported().await;
    unmount_while_open_releases_the_page().await;
}
