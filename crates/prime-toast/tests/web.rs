//! Поведение Toast в реальном браузере (wasm-bindgen-test): сообщения от сервиса, серьёзности и значки, закрытие и анимация выхода,
//! `life`/`sticky`/наведение, ключи, `clear`, дубликаты, слой, свои представления, `breakpoints`, отписка при размонтировании.
#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use prime_toast::{MessageService, Toast, ToastMessage, ToastPosition, ToastSeverity, provide_message_service};
use std::sync::{Arc, Mutex};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;
use web_sys::{EventTarget, HtmlElement, MouseEvent, MouseEventInit};

wasm_bindgen_test_configure!(run_in_browser);

type Slot = Arc<Mutex<Option<MessageService>>>;
type Closed = Arc<Mutex<Vec<String>>>;

fn doc() -> web_sys::Document {
    web_sys::window().unwrap().document().unwrap()
}

fn host() -> HtmlElement {
    let el = doc().create_element("div").unwrap().dyn_into::<HtmlElement>().unwrap();
    doc().body().unwrap().append_child(&el).unwrap();
    el
}

fn count(sel: &str) -> u32 {
    doc().query_selector_all(sel).unwrap().length()
}

/// Любой элемент (в том числе SVG-значок, который не `HtmlElement`).
fn qe(sel: &str) -> web_sys::Element {
    doc().query_selector(sel).unwrap().unwrap_or_else(|| panic!("нет {sel}"))
}

fn q(sel: &str) -> HtmlElement {
    doc().query_selector(sel).unwrap().unwrap_or_else(|| panic!("нет {sel}")).dyn_into().unwrap()
}

async fn sleep(ms: i32) {
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms).unwrap();
    });
    JsFuture::from(promise).await.unwrap();
}

async fn settle() {
    leptos::task::tick().await;
    sleep(30).await;
}

fn fire(target: &EventTarget, kind: &str) {
    let init = MouseEventInit::new();
    init.set_bubbles(false);
    target.dispatch_event(&MouseEvent::new_with_mouse_event_init_dict(kind, &init).unwrap()).unwrap();
}

fn msg(severity: ToastSeverity, summary: &str) -> ToastMessage {
    ToastMessage::new(severity, summary, format!("{summary} detail"))
}

/// Монтирует приложение с сервисом и одним `Toast`; возвращает сервис, список закрытых и размонтирование.
fn mount_one() -> (MessageService, Closed, Box<dyn std::any::Any>, HtmlElement) {
    let slot = Slot::default();
    let closed = Closed::default();
    let (s, c) = (slot.clone(), closed.clone());
    let h = host();
    let handle = leptos::mount::mount_to(h.clone(), move || {
        let service = provide_message_service();
        *s.lock().unwrap() = Some(service);
        let c = c.clone();
        let on_close = Callback::new(move |m: ToastMessage| c.lock().unwrap().push(m.summary.unwrap_or_default()));
        view! { <Toast on_close=on_close /> }
    });
    let service = slot.lock().unwrap().take().unwrap();
    (service, closed, Box::new(handle), h)
}

async fn end(handle: Box<dyn std::any::Any>, h: HtmlElement) {
    drop(handle);
    settle().await;
    h.remove();
}

async fn message_markup_and_severities() {
    let (service, _, handle, h) = mount_one();
    settle().await;
    assert_eq!(count(".p-toast"), 1);
    assert_eq!(count(".p-toast-message"), 0, "без сообщений контейнер пуст");
    service.add(msg(ToastSeverity::Success, "Done").life(60000).id("m1"));
    settle().await;
    let m = q(".p-toast-message");
    for class in ["p-toast-message-success", "p-toast-message-enter-active"] {
        assert!(m.class_name().contains(class), "{class}: {}", m.class_name());
    }
    assert_eq!(m.get_attribute("role").as_deref(), Some("alert"));
    assert_eq!(m.get_attribute("aria-live").as_deref(), Some("assertive"));
    assert_eq!(m.get_attribute("aria-atomic").as_deref(), Some("true"));
    assert_eq!(m.get_attribute("data-p").as_deref(), Some("success"));
    assert_eq!(m.get_attribute("id").as_deref(), Some("m1"));
    assert_eq!(q(".p-toast-summary").text_content().as_deref(), Some("Done"));
    assert_eq!(q(".p-toast-detail").text_content().as_deref(), Some("Done detail"));
    assert!(m.query_selector(".p-toast-message-content svg.p-toast-message-icon").unwrap().is_some(), "значок серьёзности");
    assert_eq!(q(".p-toast-close-button").get_attribute("aria-label").as_deref(), Some("Close"));
    assert!(qe(".p-toast-close-button svg").get_attribute("class").unwrap_or_default().contains("p-toast-close-icon"), "значок закрытия");
    service.clear();
    sleep(800).await;

    for (severity, class) in [
        (ToastSeverity::Info, "p-toast-message-info"),
        (ToastSeverity::Warn, "p-toast-message-warn"),
        (ToastSeverity::Error, "p-toast-message-error"),
        (ToastSeverity::Secondary, "p-toast-message-secondary"),
        (ToastSeverity::Contrast, "p-toast-message-contrast"),
    ] {
        service.add(msg(severity, "x").life(60000));
        settle().await;
        assert!(q(".p-toast-message").class_name().contains(class), "{class}");
        service.clear();
        sleep(800).await;
    }
    service.add(msg(ToastSeverity::Custom("custom".into()), "x").life(60000));
    settle().await;
    let class = q(".p-toast-message").class_name();
    assert!(class.contains("p-toast-message ") && !class.contains("p-toast-message-custom"), "{class}");
    service.clear();
    sleep(800).await;

    service.add(msg(ToastSeverity::Info, "x").life(60000).closable(false).icon("pi-star").style_class("wide").content_style_class("inner"));
    settle().await;
    assert_eq!(count(".p-toast-close-button"), 0, "closable=false");
    assert!(q(".p-toast-message").class_name().contains("wide"));
    assert!(q(".p-toast-message-content").class_name().contains("inner"));
    assert!(q("span.p-toast-message-icon").class_name().contains("pi-star"), "свой значок");
    end(handle, h).await;
}

async fn close_button_leaves_and_reports_once() {
    let (service, closed, handle, h) = mount_one();
    service.add(msg(ToastSeverity::Info, "Bye").life(60000));
    settle().await;
    q(".p-toast-close-button").click();
    settle().await;
    let m = q(".p-toast-message");
    assert!(m.class_name().contains("p-toast-message-leave-active") && !m.class_name().contains("enter-active"), "{}", m.class_name());
    assert!(closed.lock().unwrap().is_empty(), "on_close — после анимации");
    sleep(800).await;
    assert_eq!(count(".p-toast-message"), 0, "после выхода сообщение убрано");
    assert_eq!(*closed.lock().unwrap(), ["Bye"], "on_close один раз");
    end(handle, h).await;
}

async fn life_sticky_and_hover() {
    let (service, closed, handle, h) = mount_one();
    service.add(msg(ToastSeverity::Info, "Short").life(200));
    service.add(msg(ToastSeverity::Info, "Sticky").life(200).sticky(true));
    settle().await;
    assert_eq!(count(".p-toast-message"), 2);
    sleep(1000).await;
    assert_eq!(count(".p-toast-message"), 1, "обычное ушло, sticky осталось");
    assert_eq!(q(".p-toast-summary").text_content().as_deref(), Some("Sticky"));
    assert_eq!(*closed.lock().unwrap(), ["Short"]);
    service.clear();
    sleep(800).await;
    assert_eq!(count(".p-toast-message"), 0, "clear закрывает и sticky");

    service.add(msg(ToastSeverity::Info, "Hover").life(300));
    sleep(100).await;
    let m = q(".p-toast-message");
    fire(&m, "mouseenter");
    sleep(600).await;
    assert!(!q(".p-toast-message").class_name().contains("leave-active"), "под курсором сообщение остаётся");
    fire(&m, "mouseleave");
    sleep(150).await;
    assert!(!q(".p-toast-message").class_name().contains("leave-active"), "таймер запущен заново");
    sleep(900).await;
    assert_eq!(count(".p-toast-message"), 0, "после ухода курсора сообщение закрылось по времени");
    end(handle, h).await;
}

async fn keys_clear_and_duplicates() {
    let slot = Slot::default();
    let h = host();
    let s = slot.clone();
    let handle = leptos::mount::mount_to(h.clone(), move || {
        let service = provide_message_service();
        *s.lock().unwrap() = Some(service);
        view! {
            <Toast key="a" position=ToastPosition::TopLeft />
            <Toast key="b" position=ToastPosition::BottomRight />
        }
    });
    let service = slot.lock().unwrap().take().unwrap();
    settle().await;
    service.add(msg(ToastSeverity::Info, "For A").key("a").life(60000));
    service.add(msg(ToastSeverity::Warn, "For B").key("b").life(60000));
    service.add(msg(ToastSeverity::Error, "Nobody").life(60000));
    settle().await;
    assert_eq!(count(".p-toast-top-left .p-toast-message"), 1);
    assert_eq!(count(".p-toast-bottom-right .p-toast-message"), 1);
    assert_eq!(q(".p-toast-top-left .p-toast-summary").text_content().as_deref(), Some("For A"));
    assert_eq!(count(".p-toast-message"), 2, "сообщение без ключа не попало ни в один Toast с ключом");
    service.clear_key("a");
    sleep(800).await;
    assert_eq!(count(".p-toast-top-left .p-toast-message"), 0, "clear_key(a) закрыл первый");
    assert_eq!(count(".p-toast-bottom-right .p-toast-message"), 1, "второй остался");
    service.clear();
    sleep(800).await;
    assert_eq!(count(".p-toast-message"), 0, "clear закрыл всё");
    drop(handle);
    settle().await;
    h.remove();

    // дубликаты
    let slot = Slot::default();
    let h = host();
    let s = slot.clone();
    let handle = leptos::mount::mount_to(h.clone(), move || {
        let service = provide_message_service();
        *s.lock().unwrap() = Some(service);
        view! { <Toast key="open" prevent_open_duplicates=true /><Toast key="all" prevent_duplicates=true /> }
    });
    let service = slot.lock().unwrap().take().unwrap();
    settle().await;
    let open = || msg(ToastSeverity::Info, "Same").key("open").life(200);
    service.add(open());
    service.add(open());
    service.add(msg(ToastSeverity::Error, "Same").key("open").life(60000));
    settle().await;
    assert_eq!(count(".p-toast-message"), 2, "та же серьёзность — дубликат, другая — нет");
    service.clear();
    sleep(800).await;
    service.add(open());
    settle().await;
    assert_eq!(count(".p-toast-message"), 1, "prevent_open_duplicates: после ухода такое же можно добавить снова");
    service.clear();
    sleep(800).await;

    let once = || msg(ToastSeverity::Info, "Once").key("all").life(60000);
    service.add(once());
    settle().await;
    assert_eq!(count(".p-toast-message"), 1);
    service.clear();
    sleep(800).await;
    service.add(once());
    settle().await;
    assert_eq!(count(".p-toast-message"), 0, "prevent_duplicates: однажды показанное больше не добавляется");
    drop(handle);
    settle().await;
    h.remove();
}

async fn several_messages_and_layer() {
    let (service, _, handle, h) = mount_one();
    let toast = q(".p-toast");
    assert_eq!(toast.style().get_property_value("z-index").unwrap(), "", "без сообщений слой не назначен");
    service.add_all(vec![
        msg(ToastSeverity::Success, "1").life(60000),
        msg(ToastSeverity::Info, "2").life(60000),
        msg(ToastSeverity::Warn, "3").life(60000),
    ]);
    settle().await;
    assert_eq!(count(".p-toast-message"), 3, "add_all показывает все");
    let z: i32 = toast.style().get_property_value("z-index").unwrap().parse().unwrap();
    assert!(z > 0, "z-index {z}");
    service.clear();
    sleep(800).await;
    assert_eq!(toast.style().get_property_value("z-index").unwrap(), "", "после ухода последнего слой освобождён");
    end(handle, h).await;
}

async fn template_headless_and_breakpoints() {
    let slot = Slot::default();
    let h = host();
    let s = slot.clone();
    let handle = leptos::mount::mount_to(h.clone(), move || {
        let service = provide_message_service();
        *s.lock().unwrap() = Some(service);
        view! {
            <Toast
                key="tpl"
                message=Callback::new(|m: ToastMessage| view! { <span id="tpl">{m.summary.unwrap_or_default()}</span> }.into_any())
                breakpoints=vec![("920px".to_owned(), vec![("width".to_owned(), "50%".to_owned())])]
            />
            <Toast
                key="bare"
                headless=Callback::new(|(m, close): (ToastMessage, Callback<()>)| {
                    view! { <button id="bare-close" on:click=move |_| close.run(())>{m.summary.unwrap_or_default()}</button> }.into_any()
                })
            />
        }
    });
    let service = slot.lock().unwrap().take().unwrap();
    settle().await;
    service.add(msg(ToastSeverity::Info, "Custom").key("tpl").life(60000));
    service.add(msg(ToastSeverity::Info, "Bare").key("bare").life(60000));
    settle().await;
    let templated = q("#tpl").closest(".p-toast-message").unwrap().unwrap();
    assert_eq!(q("#tpl").text_content().as_deref(), Some("Custom"));
    assert!(templated.query_selector(".p-toast-summary").unwrap().is_none(), "шаблон заменил значок и тексты");
    assert!(templated.query_selector(".p-toast-close-button").unwrap().is_some(), "кнопка закрытия осталась");
    let bare = q("#bare-close").closest(".p-toast-message").unwrap().unwrap();
    assert!(bare.query_selector(".p-toast-message-content").unwrap().is_none(), "headless заменил всё содержимое");
    q("#bare-close").click();
    sleep(900).await;
    assert_eq!(count("#bare-close"), 0, "функция закрытия закрыла сообщение");

    let head = doc().head().unwrap().inner_html();
    assert!(head.contains("max-width: 920px") && head.contains("width:50% !important"), "{head}");
    drop(handle);
    settle().await;
    assert!(!doc().head().unwrap().inner_html().contains("width:50% !important"), "правила убраны при размонтировании");
    h.remove();
}

async fn unmounting_unsubscribes_from_the_service() {
    let (service, closed, handle, h) = mount_one();
    service.add(msg(ToastSeverity::Info, "Gone").life(60000));
    settle().await;
    assert_eq!(count(".p-toast-message"), 1);
    end(handle, h).await;
    service.add(msg(ToastSeverity::Info, "Late"));
    service.clear();
    settle().await;
    assert_eq!(count(".p-toast"), 0);
    assert!(closed.lock().unwrap().is_empty(), "после размонтирования on_close не вызывается");
}

/// Сценарии идут по очереди: уведомления и таймеры общие для страницы.
#[wasm_bindgen_test]
async fn toast_behaviour() {
    message_markup_and_severities().await;
    close_button_leaves_and_reports_once().await;
    life_sticky_and_hover().await;
    keys_clear_and_duplicates().await;
    several_messages_and_layer().await;
    template_headless_and_breakpoints().await;
    unmounting_unsubscribes_from_the_service().await;
}
