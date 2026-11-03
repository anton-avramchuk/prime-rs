//! Поведение ConfirmDialog в реальном браузере (wasm-bindgen-test): показ по подтверждению, разметка, кнопки, типы закрытия,
//! крестик/`Escape`/маска как отказ, `Close` сервиса, ключи, замена подтверждения, фокус, положение и шаблоны.
//!
//! Сценарии идут одним последовательным тестом: стек слоёв `z-index`, счётчик блокировки прокрутки и слушатели окна общие на
//! страницу. Стили темы не подключены: анимаций нет, окно убирается по запасному таймеру `Dialog`.
#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use prime_confirmation::{
    ConfirmButtonProps, ConfirmEventType, ConfirmFocus, Confirmation, ConfirmationService, provide_confirmation_service,
};
use prime_confirmdialog::{ConfirmDialog, ConfirmDialogHeadless};
use std::cell::Cell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;
use web_sys::{HtmlElement, KeyboardEvent, KeyboardEventInit};

wasm_bindgen_test_configure!(run_in_browser);

type Log = Arc<Mutex<Vec<String>>>;
type Shared = Rc<Cell<Option<ConfirmationService>>>;

fn doc() -> web_sys::Document {
    web_sys::window().unwrap().document().unwrap()
}

fn find(sel: &str) -> Option<HtmlElement> {
    doc().query_selector(sel).unwrap().and_then(|e| e.dyn_into().ok())
}

fn must(sel: &str) -> HtmlElement {
    find(sel).unwrap_or_else(|| panic!("нет {sel}"))
}

fn dialog_shown() -> bool {
    find(".p-confirmdialog").is_some()
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

/// Даёт эффектам и `Portal` вставить окно и повесить слушатели.
async fn settle() {
    sleep(120).await;
}

/// Ждёт окончания выхода по запасному таймеру `Dialog`.
async fn after_leave() {
    sleep(1000).await;
}

fn escape() {
    let init = KeyboardEventInit::new();
    init.set_key("Escape");
    init.set_code("Escape");
    init.set_bubbles(true);
    doc().dispatch_event(&KeyboardEvent::new_with_keyboard_event_init_dict("keydown", &init).unwrap()).unwrap();
}

/// Нажатие мыши на самой маске (так `Dialog` ловит клик вне окна).
fn mousedown(el: &HtmlElement) {
    let init = web_sys::MouseEventInit::new();
    init.set_bubbles(true);
    init.set_cancelable(true);
    el.dispatch_event(&web_sys::MouseEvent::new_with_mouse_event_init_dict("mousedown", &init).unwrap()).unwrap();
}

fn push(log: &Log, text: &str) {
    log.lock().unwrap().push(text.to_owned());
}

fn cb(log: &Log, name: &'static str) -> Callback<()> {
    let log = log.clone();
    Callback::new(move |()| push(&log, name))
}

fn on_hide(log: &Log) -> Callback<Option<ConfirmEventType>> {
    let log = log.clone();
    Callback::new(move |kind: Option<ConfirmEventType>| push(&log, &format!("hide:{}", kind.map_or("none", ConfirmEventType::as_str))))
}

fn host() -> HtmlElement {
    let el = doc().create_element("div").unwrap().dyn_into::<HtmlElement>().unwrap();
    doc().body().unwrap().append_child(&el).unwrap();
    el
}

fn mount(host: &HtmlElement, build: impl FnOnce() -> AnyView + 'static) -> (Shared, Box<dyn std::any::Any>) {
    let shared: Shared = Rc::default();
    let s = shared.clone();
    let handle = leptos::mount::mount_to(host.clone(), move || {
        s.set(Some(provide_confirmation_service()));
        build()
    });
    (shared, Box::new(handle))
}

fn svc(shared: &Shared) -> ConfirmationService {
    shared.get().unwrap()
}

async fn markup_defaults_and_answers() {
    let host = host();
    let log = Log::default();
    let hide = on_hide(&log);
    let (shared, handle) = mount(&host, move || view! { <ConfirmDialog on_hide=hide /> }.into_any());
    settle().await;
    assert!(!dialog_shown(), "без подтверждения окна нет");
    let c = |c: Confirmation| c.header("Confirmation").message("Are you sure?").accept(cb(&log, "accept")).reject(cb(&log, "reject"));
    svc(&shared).confirm(c(Confirmation::new().icon(ViewFn::from(|| view! { <i class="my-icon">"!"</i> }))));
    settle().await;
    let dialog = must(".p-dialog.p-confirmdialog");
    assert_eq!(dialog.get_attribute("role").as_deref(), Some("alertdialog"));
    assert!(find(".p-dialog-mask").is_some(), "маска");
    assert_eq!(must(".p-dialog-title").text_content().as_deref(), Some("Confirmation"));
    assert_eq!(must(".p-confirmdialog-message").text_content().as_deref(), Some("Are you sure?"));
    assert!(find(".p-confirmdialog-icon .my-icon").is_some());
    let buttons = doc().query_selector_all(".p-dialog-footer button").unwrap();
    assert_eq!(buttons.length(), 2);
    let first = buttons.item(0).unwrap().dyn_into::<HtmlElement>().unwrap();
    assert!(first.class_name().contains("p-confirmdialog-reject-button"), "сначала отказ: {}", first.class_name());
    assert_eq!(first.text_content().as_deref(), Some("No"));
    assert_eq!(must(".p-confirmdialog-accept-button").text_content().as_deref(), Some("Yes"));
    assert!(body_locked(), "прокрутка страницы заблокирована");
    sleep(120).await;
    assert!(doc().active_element().unwrap().class_list().contains("p-confirmdialog-accept-button"), "фокус на кнопке согласия");
    must(".p-confirmdialog-accept-button").click();
    after_leave().await;
    assert!(!dialog_shown() && !body_locked());
    assert_eq!(*log.lock().unwrap(), ["accept", "hide:accept"]);

    log.lock().unwrap().clear();
    svc(&shared).confirm(c(Confirmation::new()));
    settle().await;
    must(".p-confirmdialog-reject-button").click();
    after_leave().await;
    assert!(!dialog_shown());
    assert_eq!(*log.lock().unwrap(), ["reject", "hide:reject"]);
    drop(handle);
    host.remove();
}

async fn cancel_paths_are_rejections() {
    let host = host();
    let log = Log::default();
    let hide = on_hide(&log);
    let (shared, handle) = mount(&host, move || view! { <ConfirmDialog on_hide=hide /> }.into_any());
    settle().await;
    let confirm = |extra: fn(Confirmation) -> Confirmation| {
        svc(&shared).confirm(extra(Confirmation::new().message("q").accept(cb(&log, "accept")).reject(cb(&log, "reject"))))
    };
    confirm(|c| c);
    settle().await;
    must(".p-dialog-close-button").click();
    after_leave().await;
    assert!(!dialog_shown());
    assert_eq!(*log.lock().unwrap(), ["reject", "hide:cancel"], "крестик = отказ с Cancel");

    log.lock().unwrap().clear();
    confirm(|c| c);
    settle().await;
    escape();
    after_leave().await;
    assert!(!dialog_shown());
    assert_eq!(*log.lock().unwrap(), ["reject", "hide:cancel"], "Escape = отказ с Cancel");

    log.lock().unwrap().clear();
    confirm(|c| c.close_on_escape(false));
    settle().await;
    escape();
    sleep(300).await;
    assert!(dialog_shown() && log.lock().unwrap().is_empty(), "close_on_escape=false");
    svc(&shared).close();
    after_leave().await;
    assert!(!dialog_shown());
    assert_eq!(*log.lock().unwrap(), ["hide:none"], "Close сервиса — без колбэков");

    log.lock().unwrap().clear();
    confirm(|c| c.dismissable_mask(true));
    settle().await;
    mousedown(&must(".p-dialog-mask"));
    after_leave().await;
    assert!(!dialog_shown());
    assert_eq!(*log.lock().unwrap(), ["reject", "hide:cancel"], "клик по маске = отказ");

    log.lock().unwrap().clear();
    confirm(|c| c);
    settle().await;
    mousedown(&must(".p-dialog-mask"));
    sleep(300).await;
    assert!(dialog_shown(), "без dismissable_mask маска не закрывает");
    must(".p-confirmdialog-accept-button").click();
    after_leave().await;
    assert_eq!(*log.lock().unwrap(), ["accept", "hide:accept"]);
    assert!(!body_locked());
    drop(handle);
    host.remove();
}

async fn keys_replacement_and_overrides() {
    let host = host();
    let (shared, handle) = mount(&host, || {
        view! {
            <ConfirmDialog key="mine" header="Default header" message="Default message" accept_label="Okay" />
            <ConfirmDialog key="other" />
        }
        .into_any()
    });
    settle().await;
    svc(&shared).confirm(Confirmation::new().key("mine"));
    settle().await;
    assert_eq!(doc().query_selector_all(".p-confirmdialog").unwrap().length(), 1, "чужой ключ игнорируется");
    assert_eq!(must(".p-dialog-title").text_content().as_deref(), Some("Default header"), "значения компонента по умолчанию");
    assert_eq!(must(".p-confirmdialog-message").text_content().as_deref(), Some("Default message"));
    assert_eq!(must(".p-confirmdialog-accept-button").text_content().as_deref(), Some("Okay"));
    svc(&shared).confirm(
        Confirmation::new()
            .key("mine")
            .header("Own header")
            .message("Own message")
            .accept_button_props(ConfirmButtonProps::new().label("Delete"))
            .reject_label("Cancel")
            .accept_button_style_class("extra-a"),
    );
    settle().await;
    assert_eq!(doc().query_selector_all(".p-confirmdialog").unwrap().length(), 1, "окно заменено на месте");
    assert_eq!(must(".p-dialog-title").text_content().as_deref(), Some("Own header"), "подтверждение главнее");
    assert_eq!(must(".p-confirmdialog-message").text_content().as_deref(), Some("Own message"));
    assert_eq!(
        must(".p-confirmdialog-accept-button").text_content().as_deref(),
        Some("Okay"),
        "accept_label компонента главнее props.label"
    );
    assert!(must(".p-confirmdialog-accept-button").class_name().contains("extra-a"));
    assert_eq!(must(".p-confirmdialog-reject-button").text_content().as_deref(), Some("Cancel"));
    svc(&shared).close();
    after_leave().await;
    assert!(!dialog_shown());
    drop(handle);
    host.remove();
}

async fn focus_visibility_and_position() {
    let host = host();
    let (shared, handle) = mount(&host, || view! { <ConfirmDialog default_focus=ConfirmFocus::Reject /> }.into_any());
    settle().await;
    svc(&shared).confirm(Confirmation::new().message("m"));
    settle().await;
    sleep(120).await;
    assert!(doc().active_element().unwrap().class_list().contains("p-confirmdialog-reject-button"), "фокус по умолчанию — отказ");
    svc(&shared).close();
    after_leave().await;

    svc(&shared)
        .confirm(Confirmation::new().message("m").default_focus(ConfirmFocus::Close).reject_visible_flag(false).position("bottomright"));
    settle().await;
    sleep(120).await;
    assert!(find(".p-confirmdialog-reject-button").is_none() && find(".p-confirmdialog-accept-button").is_some());
    assert!(doc().active_element().unwrap().class_list().contains("p-dialog-close-button"), "фокус на кнопке закрытия");
    assert!(must(".p-dialog-mask").class_name().contains("p-dialog-bottomright"), "{}", must(".p-dialog-mask").class_name());
    svc(&shared).close();
    after_leave().await;

    svc(&shared).confirm(Confirmation::new().message("m").accept_visible_flag(false).default_focus(ConfirmFocus::None).closable(false));
    settle().await;
    assert!(find(".p-confirmdialog-accept-button").is_none() && find(".p-confirmdialog-reject-button").is_some());
    assert!(find(".p-dialog-close-button").is_none(), "closable=false");
    svc(&shared).close();
    after_leave().await;
    drop(handle);
    host.remove();
}

async fn templates() {
    let host = host();
    let log = Log::default();
    let (shared, handle) = mount(&host, || {
        view! {
            <ConfirmDialog
                key="message"
                header_content=ViewFn::from(|| view! { <b class="my-header">"H"</b> })
                message_content=Callback::new(|c: Confirmation| view! { <p class="my-message">{c.message.unwrap_or_default()}</p> }.into_any())
                accept_icon=ViewFn::from(|| view! { <i class="acc-ico">"+"</i> })
            />
            <ConfirmDialog key="icon" icon_content=ViewFn::from(|| view! { <u class="my-icon">"i"</u> }) />
            <ConfirmDialog key="footer" footer=ViewFn::from(|| view! { <span class="my-footer">"F"</span> }) />
            <ConfirmDialog
                key="headless"
                headless=Callback::new(|ctx: ConfirmDialogHeadless| {
                    let accept = ctx.accept;
                    view! { <div class="hl"><span>{ctx.confirmation.message.unwrap_or_default()}</span><button class="hl-ok" on:click=move |_| accept.run(())>"ok"</button></div> }
                        .into_any()
                })
            />
        }
        .into_any()
    });
    settle().await;
    svc(&shared).confirm(Confirmation::new().key("message").message("custom"));
    settle().await;
    assert!(find(".my-header").is_some(), "header_content");
    assert_eq!(must(".my-message").text_content().as_deref(), Some("custom"));
    assert!(find(".p-confirmdialog-message").is_none());
    assert!(must(".p-confirmdialog-accept-button").query_selector(".acc-ico").unwrap().is_some(), "значок кнопки");
    svc(&shared).close();
    after_leave().await;

    svc(&shared).confirm(Confirmation::new().key("icon").message("text").icon(ViewFn::from(|| view! { <i class="own-icon">"x"</i> })));
    settle().await;
    assert!(find(".p-confirmdialog-icon .my-icon").is_none() && find(".my-icon").is_some(), "icon_content вместо значка");
    assert!(find(".own-icon").is_none());
    assert_eq!(must(".p-confirmdialog-message").text_content().as_deref(), Some("text"));
    svc(&shared).close();
    after_leave().await;

    svc(&shared).confirm(Confirmation::new().key("footer").message("m"));
    settle().await;
    assert!(find(".my-footer").is_some() && find(".p-confirmdialog-accept-button").is_none(), "footer вместо кнопок");
    svc(&shared).close();
    after_leave().await;

    svc(&shared).confirm(Confirmation::new().key("headless").message("headless text").accept(cb(&log, "accept")));
    settle().await;
    assert_eq!(must(".hl span").text_content().as_deref(), Some("headless text"));
    assert!(find(".p-dialog-header").is_none() && find(".p-dialog-footer").is_none(), "headless заменяет всё");
    must(".hl-ok").click();
    after_leave().await;
    assert!(!dialog_shown());
    assert_eq!(*log.lock().unwrap(), ["accept"]);
    drop(handle);
    host.remove();
}

async fn unmount_while_shown_releases_the_page() {
    let host = host();
    let (shared, handle) = mount(&host, || view! { <ConfirmDialog /> }.into_any());
    settle().await;
    svc(&shared).confirm(Confirmation::new().message("m"));
    settle().await;
    assert!(dialog_shown() && body_locked());
    drop(handle);
    settle().await;
    assert!(!dialog_shown(), "окно убрано при размонтировании");
    assert!(!body_locked(), "прокрутка разблокирована");
    host.remove();
}

#[wasm_bindgen_test]
async fn confirmdialog_behaviour_in_a_real_browser() {
    markup_defaults_and_answers().await;
    cancel_paths_are_rejections().await;
    keys_replacement_and_overrides().await;
    focus_visibility_and_position().await;
    templates().await;
    unmount_while_shown_releases_the_page().await;
}
