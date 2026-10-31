//! Поведение Dialog в реальном браузере (wasm-bindgen-test): показ в `body`, кнопки, закрытие кнопкой, Escape и маской,
//! модальность, позиции, перетаскивание, изменение размера, максимизация, `breakpoints`, фокус, слои, события.
#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use prime_dialog::{Dialog, DialogPosition};
use std::sync::{Arc, Mutex};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;
use web_sys::{EventTarget, HtmlElement, KeyboardEvent, KeyboardEventInit, MouseEvent, MouseEventInit};

wasm_bindgen_test_configure!(run_in_browser);

type Log = Arc<Mutex<Vec<String>>>;

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

fn locked() -> bool {
    doc().body().unwrap().class_list().contains("p-overflow-hidden")
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

fn mouse(target: &EventTarget, kind: &str, x: i32, y: i32) {
    let init = MouseEventInit::new();
    init.set_bubbles(true);
    init.set_cancelable(true);
    init.set_client_x(x);
    init.set_client_y(y);
    target.dispatch_event(&MouseEvent::new_with_mouse_event_init_dict(kind, &init).unwrap()).unwrap();
}

fn window() -> EventTarget {
    web_sys::window().unwrap().into()
}

fn recorder() -> (Log, impl Fn(&'static str) -> Callback<()> + Clone) {
    let log = Log::default();
    let l = log.clone();
    (log, move |name: &'static str| {
        let l = l.clone();
        Callback::new(move |()| l.lock().unwrap().push(name.to_owned()))
    })
}

fn px(el: &HtmlElement, prop: &str) -> f64 {
    el.style().get_property_value(prop).unwrap().trim_end_matches("px").parse().unwrap_or(f64::NAN)
}

async fn shows_in_body_with_mask_header_and_close_button() {
    let (host, visible) = (host(), RwSignal::new(false));
    let handle = leptos::mount::mount_to(host.clone(), move || {
        view! { <Dialog visible=visible header="Header"><p id="body">"Body"</p></Dialog> }
    });
    settle().await;
    assert_eq!(count(".p-dialog") + count(".p-dialog-mask"), 0, "скрыт, пока visible ложно");
    visible.set(true);
    settle().await;
    let (mask, panel) = (q(".p-dialog-mask"), q(".p-dialog-mask > .p-dialog"));
    assert!(!host.contains(Some(&panel)), "диалог вынесен из места объявления");
    assert_eq!(panel.get_attribute("role").as_deref(), Some("dialog"));
    assert_eq!(panel.get_attribute("aria-modal").as_deref(), Some("true"));
    for class in ["p-component", "p-dialog-enter-active"] {
        assert!(panel.class_name().contains(class), "{class}: {}", panel.class_name());
    }
    let title = q(".p-dialog-title");
    assert_eq!(title.text_content().as_deref(), Some("Header"));
    assert_eq!(panel.get_attribute("aria-labelledby"), title.get_attribute("id"));
    assert_eq!(q(".p-dialog-content #body").text_content().as_deref(), Some("Body"));
    assert_eq!(q(".p-dialog-header-actions .p-dialog-close-button").get_attribute("aria-label").as_deref(), Some("Close"));
    assert_eq!((count(".p-dialog-maximize-button"), count(".p-dialog-footer")), (0, 0));
    assert!(!mask.class_name().contains("p-overlay-mask"), "не модальный: маска без p-overlay-mask");
    assert!(mask.style().get_property_value("pointer-events").unwrap() == "none");
    assert!(!locked(), "не модальный: прокрутка не блокируется");
    drop(handle);
    settle().await;
    assert_eq!(count(".p-dialog") + count(".p-dialog-mask"), 0, "после размонтирования всё убрано");
    host.remove();
}

async fn close_button_plays_leave_animation_and_reports_events() {
    let (host, visible) = (host(), RwSignal::new(true));
    let (log, cb) = recorder();
    let (on_show, on_hide) = (cb("show"), cb("hide"));
    let handle = leptos::mount::mount_to(host.clone(), move || {
        view! { <Dialog visible=visible modal=true on_show=on_show on_hide=on_hide><p>"Body"</p></Dialog> }
    });
    settle().await;
    q(".p-dialog-close-button").click();
    settle().await;
    assert!(!visible.get_untracked());
    assert!(q(".p-dialog").class_name().contains("p-dialog-leave-active"));
    assert!(q(".p-dialog-mask").class_name().contains("p-overlay-mask-leave-active"));
    assert_eq!(*log.lock().unwrap(), ["show", "hide"]);
    sleep(800).await;
    assert_eq!(count(".p-dialog") + count(".p-dialog-mask"), 0);
    drop(handle);
    host.remove();
}

async fn modal_blocks_scroll_and_mask_closes_only_when_dismissable() {
    for dismissable in [true, false] {
        let (host, visible) = (host(), RwSignal::new(true));
        let handle = leptos::mount::mount_to(host.clone(), move || {
            view! { <Dialog visible=visible modal=true dismissable_mask=dismissable><p id="in">"Body"</p></Dialog> }
        });
        settle().await;
        assert!(locked(), "модальный: прокрутка заблокирована");
        assert!(q(".p-dialog-mask").class_name().contains("p-overlay-mask"));
        assert_eq!(q(".p-dialog-mask").style().get_property_value("pointer-events").unwrap(), "auto");
        mouse(&q("#in"), "mousedown", 5, 5);
        settle().await;
        assert!(visible.get_untracked(), "нажатие внутри окна не закрывает");
        mouse(&q(".p-dialog-mask"), "mousedown", 1, 1);
        settle().await;
        assert_eq!(visible.get_untracked(), !dismissable, "dismissable_mask={dismissable}");
        visible.set(false);
        sleep(800).await;
        assert!(!locked(), "после закрытия блокировка снята (dismissable={dismissable})");
        drop(handle);
        settle().await;
        host.remove();
    }
}

async fn escape_respects_close_on_escape_and_closable() {
    for (on_escape, closable) in [(true, true), (false, true), (true, false)] {
        let (host, visible) = (host(), RwSignal::new(true));
        let (log, cb) = recorder();
        let on_hide = cb("hide");
        let handle = leptos::mount::mount_to(host.clone(), move || {
            view! { <Dialog visible=visible close_on_escape=on_escape closable=closable on_hide=on_hide><p>"Body"</p></Dialog> }
        });
        settle().await;
        escape();
        settle().await;
        let expect_closed = on_escape && closable;
        assert_eq!(!visible.get_untracked(), expect_closed, "close_on_escape={on_escape} closable={closable}");
        if expect_closed {
            escape();
            escape();
            settle().await;
            assert_eq!(*log.lock().unwrap(), ["hide"], "после закрытия слушатель снят");
        }
        assert_eq!(count(".p-dialog-close-button"), usize::from(closable) as u32);
        drop(handle);
        settle().await;
        host.remove();
    }
}

async fn position_sets_mask_class_and_alignment() {
    let (host, visible) = (host(), RwSignal::new(true));
    let handle = leptos::mount::mount_to(host.clone(), move || {
        view! { <Dialog visible=visible position=DialogPosition::BottomRight><p>"Body"</p></Dialog> }
    });
    settle().await;
    let mask = q(".p-dialog-mask");
    assert!(mask.class_name().contains("p-dialog-bottomright"), "{}", mask.class_name());
    let style = mask.style();
    assert_eq!(style.get_property_value("justify-content").unwrap(), "flex-end");
    assert_eq!(style.get_property_value("align-items").unwrap(), "flex-end");
    drop(handle);
    settle().await;
    host.remove();
}

async fn dragging_the_header_moves_the_window_within_the_viewport() {
    let (host, visible) = (host(), RwSignal::new(true));
    let (log, _) = recorder();
    let l = log.clone();
    let on_drag_end = Callback::new(move |_: MouseEvent| l.lock().unwrap().push("dragend".to_owned()));
    let handle = leptos::mount::mount_to(host.clone(), move || {
        view! { <Dialog visible=visible style="width: 300px" on_drag_end=on_drag_end><p>"Body"</p></Dialog> }
    });
    settle().await;
    let (panel, header) = (q(".p-dialog"), q(".p-dialog-header"));
    let start = panel.get_bounding_client_rect();
    // нажатие на кнопку закрытия не начинает перетаскивание
    mouse(&q(".p-dialog-close-button"), "mousedown", 50, 50);
    mouse(&window(), "mousemove", 80, 80);
    assert_eq!(panel.style().get_property_value("position").unwrap(), "", "кнопки заголовка не тянут окно");
    mouse(&window(), "mouseup", 80, 80);

    mouse(&header, "mousedown", 100, 100);
    mouse(&window(), "mousemove", 130, 120);
    assert_eq!(panel.style().get_property_value("position").unwrap(), "fixed");
    assert!((px(&panel, "left") - (start.left() + 30.0)).abs() < 1.0, "left {} от {}", px(&panel, "left"), start.left());
    assert!((px(&panel, "top") - (start.top() + 20.0)).abs() < 1.0, "top {} от {}", px(&panel, "top"), start.top());
    let moved = px(&panel, "left");
    mouse(&window(), "mousemove", -4000, 120);
    assert_eq!(px(&panel, "left"), moved, "keep_in_viewport: левее экрана не уходит");
    mouse(&window(), "mouseup", 130, 120);
    assert_eq!(*log.lock().unwrap(), ["dragend"]);
    mouse(&window(), "mousemove", 300, 300);
    assert_eq!(px(&panel, "left"), moved, "после отпускания окно стоит");
    drop(handle);
    settle().await;
    host.remove();
}

async fn draggable_false_and_keep_in_viewport_false() {
    let (host, visible) = (host(), RwSignal::new(true));
    let handle = leptos::mount::mount_to(host.clone(), move || view! { <Dialog visible=visible draggable=false><p>"Body"</p></Dialog> });
    settle().await;
    mouse(&q(".p-dialog-header"), "mousedown", 100, 100);
    mouse(&window(), "mousemove", 150, 150);
    assert_eq!(q(".p-dialog").style().get_property_value("position").unwrap(), "", "draggable=false");
    mouse(&window(), "mouseup", 150, 150);
    drop(handle);
    settle().await;

    let handle = leptos::mount::mount_to(host.clone(), move || {
        view! { <Dialog visible=visible keep_in_viewport=false><p>"Body"</p></Dialog> }
    });
    settle().await;
    let panel = q(".p-dialog");
    mouse(&q(".p-dialog-header"), "mousedown", 100, 100);
    mouse(&window(), "mousemove", -2000, 100);
    assert!(px(&panel, "left") < -1000.0, "без keep_in_viewport окно уходит за экран: {}", px(&panel, "left"));
    mouse(&window(), "mouseup", -2000, 100);
    drop(handle);
    settle().await;
    host.remove();
}

async fn resizing_the_handle_changes_the_size_and_reports_events() {
    let (host, visible) = (host(), RwSignal::new(true));
    let (log, cb) = recorder();
    let (l1, l2) = (log.clone(), log.clone());
    let on_init = Callback::new(move |_: MouseEvent| l1.lock().unwrap().push("init".to_owned()));
    let on_end = Callback::new(move |_: MouseEvent| l2.lock().unwrap().push("end".to_owned()));
    let _ = cb;
    let handle = leptos::mount::mount_to(host.clone(), move || {
        view! { <Dialog visible=visible style="width: 300px" on_resize_init=on_init on_resize_end=on_end><p>"Body"</p></Dialog> }
    });
    settle().await;
    let panel = q(".p-dialog");
    let before = f64::from(panel.offset_width());
    mouse(&q(".p-resizable-handle"), "mousedown", 200, 200);
    mouse(&window(), "mousemove", 220, 210);
    let after = px(&panel, "width");
    assert!(after > before, "ширина выросла: {before} → {after}");
    assert!(!q(".p-dialog-content").style().get_property_value("height").unwrap().is_empty(), "высота содержимого обновлена");
    mouse(&window(), "mouseup", 220, 210);
    assert_eq!(*log.lock().unwrap(), ["init", "end"]);
    mouse(&window(), "mousemove", 400, 400);
    assert_eq!(px(&panel, "width"), after, "после отпускания размер не меняется");
    drop(handle);
    settle().await;

    let handle = leptos::mount::mount_to(host.clone(), move || view! { <Dialog visible=visible resizable=false><p>"Body"</p></Dialog> });
    settle().await;
    assert_eq!(count(".p-resizable-handle"), 0, "resizable=false убирает ручку");
    drop(handle);
    settle().await;
    host.remove();
}

async fn maximize_toggles_class_labels_events_and_scroll_lock() {
    let (host, visible) = (host(), RwSignal::new(true));
    let log: Arc<Mutex<Vec<bool>>> = Arc::default();
    let l = log.clone();
    let on_maximize = Callback::new(move |m: bool| l.lock().unwrap().push(m));
    let handle = leptos::mount::mount_to(host.clone(), move || {
        view! { <Dialog visible=visible maximizable=true on_maximize=on_maximize><p>"Body"</p></Dialog> }
    });
    settle().await;
    let (panel, button) = (q(".p-dialog"), q(".p-dialog-maximize-button"));
    assert_eq!(button.get_attribute("aria-label").as_deref(), Some("Maximize"));
    assert!(!locked());
    button.click();
    settle().await;
    assert!(panel.class_name().contains("p-dialog-maximized"), "{}", panel.class_name());
    assert_eq!(button.get_attribute("aria-label").as_deref(), Some("Minimize"));
    assert!(locked(), "немодальный максимизированный блокирует прокрутку");
    button.click();
    settle().await;
    assert!(!panel.class_name().contains("p-dialog-maximized"));
    assert!(!locked());
    assert_eq!(*log.lock().unwrap(), [true, false]);

    button.click();
    settle().await;
    assert!(locked());
    visible.set(false);
    sleep(800).await;
    assert!(!locked(), "закрытие снимает блокировку максимизации");
    drop(handle);
    settle().await;
    host.remove();
}

async fn breakpoints_add_media_rules_and_remove_them_on_close() {
    let (host, visible) = (host(), RwSignal::new(true));
    let handle = leptos::mount::mount_to(host.clone(), move || {
        view! { <Dialog visible=visible breakpoints=vec![("575px".to_owned(), "90vw".to_owned())]><p>"Body"</p></Dialog> }
    });
    settle().await;
    let panel = q(".p-dialog");
    let attr = panel.get_attribute_names().iter().filter_map(|n| n.as_string()).find(|n| n.starts_with("data-dlg-")).expect("атрибут окна");
    let rules = doc().head().unwrap().inner_html();
    assert!(
        rules.contains(&format!(".p-dialog[{attr}]:not(.p-dialog-maximized)"))
            && rules.contains("max-width: 575px")
            && rules.contains("width: 90vw"),
        "{rules}"
    );
    visible.set(false);
    sleep(800).await;
    assert!(!doc().head().unwrap().inner_html().contains(&attr), "правила убраны после закрытия");
    drop(handle);
    host.remove();
}

async fn focus_goes_to_the_first_focusable_in_the_content() {
    let (host, visible) = (host(), RwSignal::new(true));
    let handle = leptos::mount::mount_to(host.clone(), move || view! { <Dialog visible=visible><input id="field" /></Dialog> });
    settle().await;
    assert!(q("#field").is_same_node(doc().active_element().as_deref()), "фокус на поле содержимого");
    assert!(q(".p-dialog").query_selector("[data-pc-section=firstfocusableelement]").unwrap().is_some(), "ловушка фокуса включена");
    drop(handle);
    settle().await;

    let handle = leptos::mount::mount_to(host.clone(), move || {
        view! { <Dialog visible=visible focus_trap=false focus_on_show=false><input id="field2" /></Dialog> }
    });
    settle().await;
    assert!(!q("#field2").is_same_node(doc().active_element().as_deref()), "focus_on_show=false");
    assert!(q(".p-dialog").query_selector("[data-pc-section=firstfocusableelement]").unwrap().is_none(), "focus_trap=false");
    drop(handle);
    settle().await;
    host.remove();
}

async fn two_dialogs_stack_and_escape_closes_the_top_one() {
    let (host, first, second) = (host(), RwSignal::new(true), RwSignal::new(false));
    let handle = leptos::mount::mount_to(host.clone(), move || {
        view! {
            <Dialog visible=first modal=true header="First" position=DialogPosition::Left><p>"One"</p></Dialog>
            <Dialog visible=second modal=true header="Second" position=DialogPosition::Right><p>"Two"</p></Dialog>
        }
    });
    settle().await;
    second.set(true);
    settle().await;
    let z = |sel: &str| q(sel).style().get_property_value("z-index").unwrap().parse::<i32>().unwrap();
    let (z1, z2) = (z(".p-dialog-left .p-dialog"), z(".p-dialog-right .p-dialog"));
    assert!(z2 > z1, "поздний выше: {z1} {z2}");
    assert_eq!((z(".p-dialog-left"), z(".p-dialog-right")), (z1 - 1, z2 - 1), "маска на единицу ниже окна");
    escape();
    settle().await;
    assert!(!second.get_untracked() && first.get_untracked(), "Escape закрыл только верхний");
    sleep(800).await;
    assert!(locked(), "первый модальный ещё держит блокировку");
    drop(handle);
    settle().await;
    assert!(!locked());
    host.remove();
}

async fn reopening_during_the_leave_animation_keeps_the_dialog() {
    let (host, visible) = (host(), RwSignal::new(true));
    let (log, cb) = recorder();
    let (on_show, on_hide) = (cb("show"), cb("hide"));
    let handle = leptos::mount::mount_to(host.clone(), move || {
        view! { <Dialog visible=visible on_show=on_show on_hide=on_hide><p>"Body"</p></Dialog> }
    });
    settle().await;
    visible.set(false);
    settle().await;
    visible.set(true);
    settle().await;
    sleep(800).await;
    let panel = q(".p-dialog");
    assert!(!panel.class_name().contains("p-dialog-leave-active"), "{}", panel.class_name());
    assert_eq!(*log.lock().unwrap(), ["show", "hide", "show"]);
    drop(handle);
    settle().await;
    host.remove();
}

async fn header_content_footer_headless_and_hidden_header() {
    let (host, visible) = (host(), RwSignal::new(true));
    let handle = leptos::mount::mount_to(host.clone(), move || {
        view! {
            <Dialog
                visible=visible
                header_content=ViewFn::from(|| view! { <span id="custom-title">"Custom"</span> })
                footer=ViewFn::from(|| view! { <button id="ok">"OK"</button> })
            >
                <p>"Body"</p>
            </Dialog>
        }
    });
    settle().await;
    assert_eq!(q(".p-dialog-header #custom-title").text_content().as_deref(), Some("Custom"));
    assert_eq!(count(".p-dialog-title"), 0, "своё содержимое заменяет название");
    assert_eq!(q(".p-dialog-footer #ok").text_content().as_deref(), Some("OK"));
    drop(handle);
    settle().await;

    let handle = leptos::mount::mount_to(host.clone(), move || {
        view! { <Dialog visible=visible headless=ViewFn::from(|| view! { <span id="bare">"Bare"</span> })><p>"Hidden"</p></Dialog> }
    });
    settle().await;
    assert!(q(".p-dialog #bare").text_content().as_deref() == Some("Bare"));
    assert_eq!(count(".p-dialog-header") + count(".p-dialog-content") + count(".p-resizable-handle"), 0);
    drop(handle);
    settle().await;

    let handle = leptos::mount::mount_to(host.clone(), move || view! { <Dialog visible=visible show_header=false><p>"Body"</p></Dialog> });
    settle().await;
    assert_eq!(count(".p-dialog-header"), 0, "show_header=false");
    assert_eq!(count(".p-dialog-content"), 1);
    drop(handle);
    settle().await;
    host.remove();
}

/// Сценарии идут по очереди: диалоги выводятся в общий `body`, а слушатели окна общие.
#[wasm_bindgen_test]
async fn dialog_behaviour() {
    shows_in_body_with_mask_header_and_close_button().await;
    close_button_plays_leave_animation_and_reports_events().await;
    modal_blocks_scroll_and_mask_closes_only_when_dismissable().await;
    escape_respects_close_on_escape_and_closable().await;
    position_sets_mask_class_and_alignment().await;
    dragging_the_header_moves_the_window_within_the_viewport().await;
    draggable_false_and_keep_in_viewport_false().await;
    resizing_the_handle_changes_the_size_and_reports_events().await;
    maximize_toggles_class_labels_events_and_scroll_lock().await;
    breakpoints_add_media_rules_and_remove_them_on_close().await;
    focus_goes_to_the_first_focusable_in_the_content().await;
    two_dialogs_stack_and_escape_closes_the_top_one().await;
    reopening_during_the_leave_animation_keeps_the_dialog().await;
    header_content_footer_headless_and_hidden_header().await;
}
