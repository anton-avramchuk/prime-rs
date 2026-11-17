//! ConfirmPopup behavior in a real browser (wasm-bindgen-test): show on confirmation, markup, buttons, focus,
//! closing (`Escape`, outside click, `close`), keys, retargeting, templates and controlled show.
//!
//! Scenarios run as one sequential test: the `z-index` layer stack and window listeners are shared across the page, so
//! parallel tests would affect each other. Theme styles are not attached: no animations.
#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use prime_confirmation::{ConfirmButtonProps, ConfirmFocus, Confirmation, ConfirmationService, provide_confirmation_service};
use prime_confirmpopup::{ConfirmPopup, ConfirmPopupHeadless};
use std::cell::Cell;
use std::rc::Rc;
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

fn find(sel: &str) -> Option<HtmlElement> {
    doc().query_selector(sel).unwrap().and_then(|e| e.dyn_into().ok())
}

fn must(sel: &str) -> HtmlElement {
    find(sel).unwrap_or_else(|| panic!("no {sel}"))
}

fn panel_shown() -> bool {
    find(".p-confirmpopup").is_some()
}

async fn sleep(ms: i32) {
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms).unwrap();
    });
    JsFuture::from(promise).await.unwrap();
}

/// Lets effects and `Portal` insert the panel and attach window listeners (they are attached on the next tick).
async fn settle() {
    sleep(80).await;
}

fn target(left: i32, top: i32) -> HtmlElement {
    let el = doc().create_element("button").unwrap().dyn_into::<HtmlElement>().unwrap();
    el.set_text_content(Some("target"));
    el.set_attribute("style", &format!("position: absolute; left: {left}px; top: {top}px; width: 80px; height: 30px")).unwrap();
    doc().body().unwrap().append_child(&el).unwrap();
    el
}

fn escape() {
    let init = KeyboardEventInit::new();
    init.set_key("Escape");
    init.set_code("Escape");
    init.set_bubbles(true);
    doc().dispatch_event(&KeyboardEvent::new_with_keyboard_event_init_dict("keydown", &init).unwrap()).unwrap();
}

fn log_cb(log: &Log, name: &'static str) -> Callback<()> {
    let log = log.clone();
    Callback::new(move |()| log.lock().unwrap().push(name))
}

type Shared = Rc<Cell<Option<ConfirmationService>>>;

fn mount(host: &HtmlElement, build: impl FnOnce() -> AnyView + 'static) -> (Shared, Box<dyn std::any::Any>) {
    let shared: Shared = Rc::default();
    let s = shared.clone();
    let handle = leptos::mount::mount_to(host.clone(), move || {
        s.set(Some(provide_confirmation_service()));
        build()
    });
    (shared, Box::new(handle))
}

fn host() -> HtmlElement {
    let el = doc().create_element("div").unwrap().dyn_into::<HtmlElement>().unwrap();
    doc().body().unwrap().append_child(&el).unwrap();
    el
}

fn svc(shared: &Shared) -> ConfirmationService {
    shared.get().unwrap()
}

async fn markup_defaults_focus_and_answers() {
    let host = host();
    let (shared, handle) = mount(&host, || view! { <ConfirmPopup /> }.into_any());
    settle().await;
    assert!(!panel_shown(), "no panel without a confirmation");
    let t = target(40, 40);
    let log = Log::default();
    svc(&shared).confirm(
        Confirmation::new()
            .message("Are you sure?")
            .icon(ViewFn::from(|| view! { <i class="my-icon">"!"</i> }))
            .target(t.clone())
            .accept(log_cb(&log, "accept"))
            .reject(log_cb(&log, "reject")),
    );
    settle().await;
    let panel = must(".p-confirmpopup");
    assert!(panel.class_name().contains("p-component"), "{}", panel.class_name());
    assert_eq!(panel.get_attribute("role").as_deref(), Some("alertdialog"));
    assert_eq!(must(".p-confirmpopup-message").text_content().as_deref(), Some("Are you sure?"));
    assert!(must(".p-confirmpopup-icon .my-icon").text_content().is_some());
    assert!(find(".p-confirmpopup-content").is_some() && find(".p-confirmpopup-footer").is_some());
    let reject = must(".p-confirmpopup-reject-button");
    let accept = must(".p-confirmpopup-accept-button");
    assert_eq!(reject.text_content().as_deref(), Some("No"), "default translation");
    assert_eq!(accept.text_content().as_deref(), Some("Yes"));
    assert!(accept.class_name().contains("p-button-sm"), "small size: {}", accept.class_name());
    let z: i32 = panel.style().get_property_value("z-index").unwrap().parse().unwrap();
    assert!(z >= 1000, "overlay group layer: {z}");
    assert_eq!(panel.parent_element().unwrap().parent_element().unwrap().tag_name(), "BODY", "panel in body");
    let top: f64 = panel.style().get_property_value("top").unwrap().trim_end_matches("px").parse().unwrap();
    assert!(top >= 40.0 + 30.0, "panel below the target: {top}");
    assert_eq!(doc().active_element().unwrap().class_name(), accept.class_name(), "focus on the accept button");
    accept.click();
    settle().await;
    assert!(!panel_shown());
    assert_eq!(*log.lock().unwrap(), ["accept"]);
    assert!(doc().active_element().unwrap().is_same_node(Some(&t)), "focus returned to the target");

    svc(&shared)
        .confirm(Confirmation::new().message("again").target(t.clone()).accept(log_cb(&log, "accept")).reject(log_cb(&log, "reject")));
    settle().await;
    must(".p-confirmpopup-reject-button").click();
    settle().await;
    assert!(!panel_shown());
    assert_eq!(*log.lock().unwrap(), ["accept", "reject"]);
    t.remove();
    drop(handle);
    host.remove();
}

async fn escape_and_outside_click() {
    let host = host();
    let (shared, handle) = mount(&host, || view! { <ConfirmPopup /> }.into_any());
    settle().await;
    let t = target(40, 40);
    let log = Log::default();
    let confirm = |c: Confirmation| svc(&shared).confirm(c.target(t.clone()).accept(log_cb(&log, "accept")).reject(log_cb(&log, "reject")));
    confirm(Confirmation::new().message("a"));
    settle().await;
    escape();
    settle().await;
    assert!(!panel_shown());
    assert_eq!(*log.lock().unwrap(), ["reject"], "Escape = reject");

    confirm(Confirmation::new().message("b").close_on_escape(false));
    settle().await;
    escape();
    settle().await;
    assert!(panel_shown() && log.lock().unwrap().len() == 1, "close_on_escape=false");
    must(".p-confirmpopup-message").click();
    settle().await;
    assert!(panel_shown(), "click inside the panel does not close");
    t.click();
    settle().await;
    assert!(panel_shown(), "click on the target does not close");
    doc().body().unwrap().click();
    settle().await;
    assert!(!panel_shown(), "outside click closes");
    assert_eq!(*log.lock().unwrap(), ["reject"], "without accept and reject");

    confirm(Confirmation::new().message("c").dismissable_mask(false));
    settle().await;
    doc().body().unwrap().click();
    settle().await;
    assert!(panel_shown(), "dismissable_mask=false");
    svc(&shared).close();
    settle().await;
    assert!(!panel_shown(), "close() hides");
    t.remove();
    drop(handle);
    host.remove();
}

async fn keys_and_retarget() {
    let host = host();
    let (shared, handle) = mount(&host, || {
        view! {
            <ConfirmPopup key="mine" style="width: 150px" />
            <ConfirmPopup key="other" />
        }
        .into_any()
    });
    settle().await;
    let (t1, t2) = (target(40, 40), target(300, 200));
    svc(&shared).confirm(Confirmation::new().key("mine").message("one").target(t1.clone()));
    settle().await;
    assert_eq!(doc().query_selector_all(".p-confirmpopup").unwrap().length(), 1, "foreign key is ignored");
    let left1: f64 = must(".p-confirmpopup").style().get_property_value("left").unwrap().trim_end_matches("px").parse().unwrap();
    svc(&shared).confirm(Confirmation::new().key("mine").message("two").target(t2.clone()));
    settle().await;
    assert_eq!(must(".p-confirmpopup-message").text_content().as_deref(), Some("two"), "confirmation replaced");
    let panel = must(".p-confirmpopup");
    let left2: f64 = panel.style().get_property_value("left").unwrap().trim_end_matches("px").parse().unwrap();
    let top2: f64 = panel.style().get_property_value("top").unwrap().trim_end_matches("px").parse().unwrap();
    assert!(left2 > left1 + 100.0 && top2 >= 230.0, "panel moved to the new target: {left1} -> {left2}, {top2}");
    svc(&shared).close();
    settle().await;
    assert!(!panel_shown());
    t1.remove();
    t2.remove();
    drop(handle);
    host.remove();
}

async fn labels_buttons_focus_and_visibility() {
    let host = host();
    let (shared, handle) = mount(&host, || view! { <ConfirmPopup default_focus=ConfirmFocus::Reject style_class="mine" /> }.into_any());
    settle().await;
    let t = target(40, 40);
    svc(&shared).confirm(
        Confirmation::new()
            .message("m")
            .target(t.clone())
            .accept_label("Save")
            .reject_label("Cancel")
            .accept_button_style_class("extra-a")
            .reject_button_props(ConfirmButtonProps::new().label("Props label").outlined(true)),
    );
    settle().await;
    assert!(must(".p-confirmpopup").class_name().contains("mine"));
    let accept = must(".p-confirmpopup-accept-button");
    assert_eq!(accept.text_content().as_deref(), Some("Save"));
    assert!(accept.class_name().contains("extra-a"));
    let reject = must(".p-confirmpopup-reject-button");
    assert_eq!(reject.text_content().as_deref(), Some("Props label"), "props.label takes precedence over reject_label");
    assert!(reject.class_name().contains("p-button-outlined"), "{}", reject.class_name());
    assert_eq!(doc().active_element().unwrap().class_name(), reject.class_name(), "default focus on the reject button");
    svc(&shared).close();
    settle().await;

    svc(&shared).confirm(Confirmation::new().message("m").target(t.clone()).default_focus(ConfirmFocus::Accept).reject_visible_flag(false));
    settle().await;
    assert!(find(".p-confirmpopup-reject-button").is_none() && find(".p-confirmpopup-accept-button").is_some());
    assert!(doc().active_element().unwrap().class_list().contains("p-confirmpopup-accept-button"), "the confirmation default_focus takes precedence");
    svc(&shared).close();
    settle().await;

    svc(&shared).confirm(Confirmation::new().message("m").target(t.clone()).accept_visible_flag(false));
    settle().await;
    assert!(find(".p-confirmpopup-accept-button").is_none() && find(".p-confirmpopup-reject-button").is_some());
    svc(&shared).close();
    settle().await;

    svc(&shared).confirm(Confirmation::new().message("m").target(t.clone()).default_focus(ConfirmFocus::None));
    settle().await;
    assert!(!doc().active_element().unwrap().class_list().contains("p-confirmpopup-accept-button"), "ConfirmFocus::None");
    svc(&shared).close();
    settle().await;
    t.remove();
    drop(handle);
    host.remove();
}

async fn content_and_headless_templates() {
    let host = host();
    let (shared, handle) = mount(&host, || {
        view! {
            <ConfirmPopup
                key="content"
                content=Callback::new(|c: Confirmation| view! { <b class="tpl">{c.message.unwrap_or_default()}</b> }.into_any())
                accept_icon=ViewFn::from(|| view! { <i class="acc-ico">"+"</i> })
            />
            <ConfirmPopup
                key="headless"
                headless=Callback::new(|ctx: ConfirmPopupHeadless| {
                    let accept = ctx.accept;
                    view! { <div class="hl"><span>{ctx.confirmation.message.unwrap_or_default()}</span><button class="hl-ok" on:click=move |_| accept.run(())>"ok"</button></div> }
                        .into_any()
                })
            />
        }
        .into_any()
    });
    settle().await;
    let t = target(40, 40);
    let log = Log::default();
    svc(&shared).confirm(Confirmation::new().key("content").message("custom text").target(t.clone()));
    settle().await;
    assert_eq!(must(".p-confirmpopup-content .tpl").text_content().as_deref(), Some("custom text"));
    assert!(find(".p-confirmpopup-message").is_none() && find(".p-confirmpopup-footer").is_some());
    assert!(must(".p-confirmpopup-accept-button").query_selector(".acc-ico").unwrap().is_some(), "button icon");
    svc(&shared).close();
    settle().await;

    svc(&shared).confirm(Confirmation::new().key("headless").message("headless text").target(t.clone()).accept(log_cb(&log, "accept")));
    settle().await;
    assert!(find(".p-confirmpopup-content").is_none() && find(".p-confirmpopup-footer").is_none(), "headless replaces everything");
    assert_eq!(must(".hl span").text_content().as_deref(), Some("headless text"));
    must(".hl-ok").click();
    settle().await;
    assert!(!panel_shown());
    assert_eq!(*log.lock().unwrap(), ["accept"]);
    t.remove();
    drop(handle);
    host.remove();
}

async fn events_and_controlled_visible() {
    let host = host();
    let log = Log::default();
    let (l1, l2) = (log_cb(&log, "show"), log_cb(&log, "hide"));
    let (shared, handle) = mount(&host, move || view! { <ConfirmPopup on_show=l1 on_hide=l2 /> }.into_any());
    settle().await;
    let t = target(40, 40);
    svc(&shared).confirm(Confirmation::new().message("m").target(t.clone()));
    settle().await;
    assert_eq!(*log.lock().unwrap(), ["show"]);
    svc(&shared).close();
    settle().await;
    assert_eq!(*log.lock().unwrap(), ["show", "hide"]);
    drop(handle);

    let visible = RwSignal::new(false);
    let (_shared, handle) = mount(&host, move || view! { <ConfirmPopup visible=Signal::derive(move || Some(visible.get())) /> }.into_any());
    settle().await;
    assert!(!panel_shown());
    visible.set(true);
    settle().await;
    assert!(panel_shown(), "controlled show without a confirmation");
    visible.set(false);
    settle().await;
    assert!(!panel_shown());
    t.remove();
    drop(handle);
    host.remove();
}

async fn unmount_while_shown_removes_listeners() {
    let host = host();
    let (shared, handle) = mount(&host, || view! { <ConfirmPopup /> }.into_any());
    settle().await;
    let t = target(40, 40);
    let log = Log::default();
    svc(&shared).confirm(Confirmation::new().message("m").target(t.clone()).reject(log_cb(&log, "reject")));
    settle().await;
    assert!(panel_shown());
    drop(handle);
    settle().await;
    assert!(!panel_shown(), "panel removed on unmount");
    escape();
    settle().await;
    assert!(log.lock().unwrap().is_empty(), "Escape listener removed");
    t.remove();
    host.remove();
}

#[wasm_bindgen_test]
async fn confirmpopup_behaviour_in_a_real_browser() {
    markup_defaults_focus_and_answers().await;
    escape_and_outside_click().await;
    keys_and_retarget().await;
    labels_buttons_focus_and_visibility().await;
    content_and_headless_templates().await;
    events_and_controlled_visible().await;
    unmount_while_shown_removes_listeners().await;
}
