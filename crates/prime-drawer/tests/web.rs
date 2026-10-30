//! Поведение Drawer в реальном браузере (wasm-bindgen-test): показ в `body`, закрытие кнопкой, маской и Escape,
//! анимация выхода, позиции, блокировка прокрутки, ловушка фокуса, слои двух панелей, события.
#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use prime_drawer::{Drawer, DrawerPosition};
use std::sync::{Arc, Mutex};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;
use web_sys::{HtmlElement, KeyboardEvent, KeyboardEventInit};

wasm_bindgen_test_configure!(run_in_browser);

type Events = Arc<Mutex<Vec<&'static str>>>;

fn doc() -> web_sys::Document {
    web_sys::window().unwrap().document().unwrap()
}

fn host() -> HtmlElement {
    let el = doc().create_element("div").unwrap().dyn_into::<HtmlElement>().unwrap();
    doc().body().unwrap().append_child(&el).unwrap();
    el
}

fn find(sel: &str) -> Option<HtmlElement> {
    doc().query_selector(sel).unwrap().map(|e| e.dyn_into().unwrap())
}

fn q(sel: &str) -> HtmlElement {
    find(sel).unwrap_or_else(|| panic!("нет {sel}"))
}

fn count(sel: &str) -> u32 {
    doc().query_selector_all(sel).unwrap().length()
}

async fn sleep(ms: i32) {
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms).unwrap();
    });
    JsFuture::from(promise).await.unwrap();
}

/// Реактивные обновления, `Portal` (монтирует содержимое в эффекте) и снятие эффектов.
async fn settle() {
    leptos::task::tick().await;
    sleep(30).await;
}

fn escape() {
    let init = KeyboardEventInit::new();
    init.set_key("Escape");
    init.set_bubbles(true);
    let event = KeyboardEvent::new_with_keyboard_event_init_dict("keydown", &init).unwrap();
    web_sys::window().unwrap().dispatch_event(&event).unwrap();
}

fn events() -> (Events, Callback<()>, Callback<()>) {
    let log = Events::default();
    let (a, b) = (log.clone(), log.clone());
    (log, Callback::new(move |()| a.lock().unwrap().push("show")), Callback::new(move |()| b.lock().unwrap().push("hide")))
}

async fn shows_in_body_with_header_content_and_close_button() {
    let (host, visible) = (host(), RwSignal::new(false));
    let handle = leptos::mount::mount_to(host.clone(), move || {
        view! { <Drawer visible=visible header="Left Drawer"><p id="body">"Body"</p></Drawer> }
    });
    settle().await;
    assert_eq!(count(".p-drawer"), 0, "скрыт, пока visible ложно");
    visible.set(true);
    settle().await;
    let panel = q(".p-drawer");
    assert!(!host.contains(Some(&panel)), "панель вынесена из места объявления");
    assert!(doc().body().unwrap().contains(Some(&panel)));
    assert_eq!(panel.get_attribute("role").as_deref(), Some("complementary"));
    for class in ["p-component", "p-drawer-left", "p-drawer-open", "p-drawer-enter-left"] {
        assert!(panel.class_name().contains(class), "{class}: {}", panel.class_name());
    }
    assert_eq!(panel.get_attribute("data-p-open").as_deref(), Some("true"));
    assert_eq!(q(".p-drawer-title").text_content().as_deref(), Some("Left Drawer"));
    assert_eq!(q(".p-drawer-content #body").text_content().as_deref(), Some("Body"));
    assert_eq!(q(".p-drawer-close-button").get_attribute("aria-label").as_deref(), Some("Close"));
    assert_eq!(count(".p-drawer-footer"), 0, "подвала нет, пока он не задан");
    assert_eq!(count(".p-drawer-mask.p-overlay-mask"), 1);
    drop(handle);
    settle().await;
    assert_eq!(count(".p-drawer") + count(".p-drawer-mask"), 0, "после размонтирования всё убрано");
    host.remove();
}

async fn close_button_plays_the_leave_animation_then_removes_the_panel() {
    let (host, visible) = (host(), RwSignal::new(true));
    let (log, on_show, on_hide) = events();
    let handle = leptos::mount::mount_to(host.clone(), move || {
        view! { <Drawer visible=visible on_show=on_show on_hide=on_hide><p>"Body"</p></Drawer> }
    });
    settle().await;
    q(".p-drawer-close-button").click();
    settle().await;
    assert!(!visible.get_untracked(), "закрытие пишет ложь в visible");
    let panel = q(".p-drawer");
    assert!(panel.class_name().contains("p-drawer-leave-left"), "{}", panel.class_name());
    assert!(!panel.class_name().contains("p-drawer-open"));
    assert!(q(".p-drawer-mask").class_name().contains("p-overlay-mask-leave-active"));
    assert_eq!(*log.lock().unwrap(), ["show", "hide"]);
    sleep(800).await;
    assert_eq!(count(".p-drawer") + count(".p-drawer-mask"), 0, "после выхода панель и маска убраны");
    drop(handle);
    host.remove();
}

async fn mask_click_closes_only_when_dismissible() {
    for dismissible in [true, false] {
        let (host, visible) = (host(), RwSignal::new(true));
        let handle = leptos::mount::mount_to(host.clone(), move || {
            view! { <Drawer visible=visible dismissible=dismissible><p>"Body"</p></Drawer> }
        });
        settle().await;
        q(".p-drawer").click();
        settle().await;
        assert!(visible.get_untracked(), "клик по самой панели не закрывает");
        q(".p-drawer-mask").click();
        settle().await;
        assert_eq!(visible.get_untracked(), !dismissible, "dismissible={dismissible}");
        drop(handle);
        settle().await;
        host.remove();
    }
}

async fn without_modal_there_is_no_mask() {
    let (host, visible) = (host(), RwSignal::new(true));
    let handle = leptos::mount::mount_to(host.clone(), move || view! { <Drawer visible=visible modal=false><p>"Body"</p></Drawer> });
    settle().await;
    assert_eq!(count(".p-drawer"), 1);
    assert_eq!(count(".p-drawer-mask"), 0);
    drop(handle);
    settle().await;
    host.remove();
}

async fn escape_closes_only_when_enabled_and_listener_is_removed_afterwards() {
    for enabled in [true, false] {
        let (host, visible) = (host(), RwSignal::new(true));
        let (log, _, on_hide) = events();
        let handle = leptos::mount::mount_to(host.clone(), move || {
            view! { <Drawer visible=visible close_on_escape=enabled on_hide=on_hide><p>"Body"</p></Drawer> }
        });
        settle().await;
        escape();
        settle().await;
        assert_eq!(visible.get_untracked(), !enabled, "close_on_escape={enabled}");
        if enabled {
            escape();
            escape();
            settle().await;
            assert_eq!(*log.lock().unwrap(), ["hide"], "после закрытия слушатель снят");
        }
        drop(handle);
        settle().await;
        host.remove();
    }
}

async fn position_and_full_screen_choose_classes() {
    let cases = [
        (DrawerPosition::Right, false, "p-drawer-right", "p-drawer-enter-right"),
        (DrawerPosition::Top, false, "p-drawer-top", "p-drawer-enter-top"),
        (DrawerPosition::Bottom, false, "p-drawer-bottom", "p-drawer-enter-bottom"),
        (DrawerPosition::Full, false, "p-drawer-full", "p-drawer-enter-full"),
        (DrawerPosition::Left, true, "p-drawer-full", "p-drawer-enter-full"),
    ];
    for (position, full, class, enter) in cases {
        let (host, visible) = (host(), RwSignal::new(true));
        let handle = leptos::mount::mount_to(host.clone(), move || {
            view! { <Drawer visible=visible position=position full_screen=full><p>"Body"</p></Drawer> }
        });
        settle().await;
        let panel = q(".p-drawer");
        assert!(panel.class_name().contains(class) && panel.class_name().contains(enter), "{position:?} {full}: {}", panel.class_name());
        drop(handle);
        settle().await;
        host.remove();
    }
}

async fn block_scroll_marks_the_body_while_shown() {
    let (host, visible) = (host(), RwSignal::new(false));
    let handle = leptos::mount::mount_to(host.clone(), move || view! { <Drawer visible=visible block_scroll=true><p>"Body"</p></Drawer> });
    let locked = || doc().body().unwrap().class_list().contains("p-overflow-hidden");
    settle().await;
    assert!(!locked());
    visible.set(true);
    settle().await;
    assert!(locked(), "прокрутка заблокирована, пока панель показана");
    visible.set(false);
    sleep(800).await;
    assert!(!locked(), "после закрытия блокировка снята");
    drop(handle);
    host.remove();
}

async fn focus_trap_sentinels_exist_while_shown() {
    let (host, visible) = (host(), RwSignal::new(true));
    let handle =
        leptos::mount::mount_to(host.clone(), move || view! { <Drawer visible=visible><button id="inside">"Inside"</button></Drawer> });
    settle().await;
    let panel = q(".p-drawer");
    assert!(panel.query_selector("[data-pc-section=firstfocusableelement]").unwrap().is_some());
    assert!(panel.query_selector("[data-pc-section=lastfocusableelement]").unwrap().is_some());
    visible.set(false);
    settle().await;
    assert!(panel.query_selector("[data-pc-section=firstfocusableelement]").unwrap().is_none(), "ловушка снята при закрытии");
    drop(handle);
    sleep(100).await;
    host.remove();
}

async fn two_drawers_stack_and_escape_closes_the_top_one() {
    let (host, first, second) = (host(), RwSignal::new(true), RwSignal::new(false));
    let handle = leptos::mount::mount_to(host.clone(), move || {
        view! {
            <Drawer visible=first header="First"><p>"One"</p></Drawer>
            <Drawer visible=second header="Second" position=DrawerPosition::Right><p>"Two"</p></Drawer>
        }
    });
    settle().await;
    second.set(true);
    settle().await;
    let z = |sel: &str| q(sel).style().get_property_value("z-index").unwrap().parse::<i32>().unwrap();
    let (z1, z2) = (z(".p-drawer-left"), z(".p-drawer-right"));
    assert!(z2 > z1, "поздняя выше: {z1} {z2}");
    let masks: Vec<i32> = (0..count(".p-drawer-mask"))
        .map(|i| {
            let m = doc().query_selector_all(".p-drawer-mask").unwrap().item(i).unwrap().dyn_into::<HtmlElement>().unwrap();
            m.style().get_property_value("z-index").unwrap().parse().unwrap()
        })
        .collect();
    assert_eq!(masks, [z1 - 1, z2 - 1], "маска на единицу ниже своей панели");
    escape();
    settle().await;
    assert!(!second.get_untracked() && first.get_untracked(), "Escape закрыл только верхнюю");
    drop(handle);
    settle().await;
    host.remove();
}

async fn reopening_during_the_leave_animation_keeps_the_panel() {
    let (host, visible) = (host(), RwSignal::new(true));
    let (log, on_show, on_hide) = events();
    let handle = leptos::mount::mount_to(host.clone(), move || {
        view! { <Drawer visible=visible on_show=on_show on_hide=on_hide><p>"Body"</p></Drawer> }
    });
    settle().await;
    visible.set(false);
    settle().await;
    visible.set(true);
    settle().await;
    sleep(800).await;
    let panel = q(".p-drawer");
    assert!(panel.class_name().contains("p-drawer-open") && !panel.class_name().contains("p-drawer-leave"), "{}", panel.class_name());
    assert_eq!(*log.lock().unwrap(), ["show", "hide", "show"]);
    drop(handle);
    settle().await;
    host.remove();
}

async fn headless_and_footer_and_header_content() {
    let (host, visible) = (host(), RwSignal::new(true));
    let handle = leptos::mount::mount_to(host.clone(), move || {
        view! {
            <Drawer visible=visible headless=ViewFn::from(|| view! { <span id="custom">"Custom"</span> })>
                <p>"Hidden"</p>
            </Drawer>
        }
    });
    settle().await;
    let panel = q(".p-drawer");
    assert!(panel.query_selector("#custom").unwrap().is_some());
    assert_eq!(count(".p-drawer-header") + count(".p-drawer-content") + count(".p-drawer-footer"), 0);
    drop(handle);
    settle().await;
    let handle = leptos::mount::mount_to(host.clone(), move || {
        view! {
            <Drawer
                visible=visible
                closable=false
                header_content=ViewFn::from(|| view! { <span id="avatar">"A"</span> })
                footer=ViewFn::from(|| view! { <span id="foot">"F"</span> })
            >
                <p>"Body"</p>
            </Drawer>
        }
    });
    settle().await;
    assert_eq!(q(".p-drawer-header #avatar").text_content().as_deref(), Some("A"));
    assert_eq!(q(".p-drawer-footer #foot").text_content().as_deref(), Some("F"));
    assert_eq!(count(".p-drawer-close-button"), 0, "closable=false убирает кнопку");
    drop(handle);
    settle().await;
    host.remove();
}

/// Сценарии идут по очереди: панели выводятся в общий `body`, а слушатели окна общие.
#[wasm_bindgen_test]
async fn drawer_behaviour() {
    shows_in_body_with_header_content_and_close_button().await;
    close_button_plays_the_leave_animation_then_removes_the_panel().await;
    mask_click_closes_only_when_dismissible().await;
    without_modal_there_is_no_mask().await;
    escape_closes_only_when_enabled_and_listener_is_removed_afterwards().await;
    position_and_full_screen_choose_classes().await;
    block_scroll_marks_the_body_while_shown().await;
    focus_trap_sentinels_exist_while_shown().await;
    two_drawers_stack_and_escape_closes_the_top_one().await;
    reopening_during_the_leave_animation_keeps_the_panel().await;
    headless_and_footer_and_header_content().await;
}
