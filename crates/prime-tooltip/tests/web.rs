//! Поведение директивы `tooltip` в реальном браузере (wasm-bindgen-test): показ по hover/focus/both, стороны и перенос, задержки,
//! автоскрытие, содержимое, Escape/resize/scroll, реактивные текст и `disabled`, слой, очистка, повторное подключение.
#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use prime_button::Button;
use prime_inputtext::InputText;
use prime_tooltip::{TooltipEvent, TooltipOptions, TooltipPosition, tooltip};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;
use web_sys::{Element, EventTarget, HtmlElement, KeyboardEvent, KeyboardEventInit, MouseEvent, MouseEventInit};

wasm_bindgen_test_configure!(run_in_browser);

fn doc() -> web_sys::Document {
    web_sys::window().unwrap().document().unwrap()
}

/// Минимум стилей темы: подсказка позиционируется абсолютно (остальное тесту не нужно).
fn install_css() {
    if doc().get_element_by_id("tip-test-css").is_none() {
        let style = doc().create_element("style").unwrap();
        style.set_id("tip-test-css");
        style.set_text_content(Some(
            ".p-tooltip{position:absolute;} .host{position:absolute;left:300px;top:250px;width:120px;height:40px;}",
        ));
        doc().head().unwrap().append_child(&style).unwrap();
    }
}

fn container() -> HtmlElement {
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

fn tip() -> Option<HtmlElement> {
    find(".p-tooltip")
}

async fn sleep(ms: i32) {
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms).unwrap();
    });
    JsFuture::from(promise).await.unwrap();
}

async fn settle() {
    leptos::task::tick().await;
    sleep(20).await;
}

fn fire(target: &EventTarget, kind: &str) {
    let init = MouseEventInit::new();
    init.set_bubbles(false);
    target.dispatch_event(&MouseEvent::new_with_mouse_event_init_dict(kind, &init).unwrap()).unwrap();
}

fn leave_to(target: &EventTarget, related: Option<&EventTarget>) {
    let init = MouseEventInit::new();
    init.set_related_target(related);
    target.dispatch_event(&MouseEvent::new_with_mouse_event_init_dict("mouseleave", &init).unwrap()).unwrap();
}

fn escape() {
    let init = KeyboardEventInit::new();
    init.set_key("Escape");
    init.set_bubbles(true);
    let event = KeyboardEvent::new_with_keyboard_event_init_dict("keydown", &init).unwrap();
    web_sys::window().unwrap().dispatch_event(&event).unwrap();
}

fn window() -> EventTarget {
    web_sys::window().unwrap().into()
}

/// Элемент `.host` с директивой; возвращает владельца-размонтирование и сам элемент.
fn mount(options: TooltipOptions) -> (Box<dyn std::any::Any>, HtmlElement, HtmlElement) {
    install_css();
    let c = container();
    let handle = leptos::mount::mount_to(c.clone(), move || view! { <div id="host" class="host" use:tooltip=options.clone()>"Host"</div> });
    let host = c.query_selector("#host").unwrap().unwrap().dyn_into().unwrap();
    (Box::new(handle), host, c)
}

async fn end(handle: impl std::any::Any, c: HtmlElement) {
    drop(handle);
    settle().await;
    c.remove();
}

fn rect(el: &Element) -> (f64, f64, f64, f64) {
    let r = el.get_bounding_client_rect();
    (r.left(), r.top(), r.right(), r.bottom())
}

async fn hover_shows_and_leave_or_click_hide() {
    let (handle, host, c) = mount(TooltipOptions::text("Enter your username"));
    assert!(tip().is_none(), "до наведения подсказки нет");
    fire(&host, "mouseenter");
    settle().await;
    let t = tip().expect("подсказка показана");
    for class in ["p-tooltip", "p-component", "p-tooltip-right"] {
        assert!(t.class_name().contains(class), "{class}: {}", t.class_name());
    }
    assert_eq!(t.get_attribute("role").as_deref(), Some("tooltip"));
    assert_eq!(t.style().get_property_value("display").unwrap(), "inline-block");
    assert_eq!(t.parent_element().unwrap().tag_name(), "BODY");
    assert!(t.query_selector(".p-tooltip-arrow").unwrap().is_some());
    assert_eq!(q(".p-tooltip-text").text_content().as_deref(), Some("Enter your username"));
    assert_eq!(t.style().get_property_value("pointer-events").unwrap(), "none", "auto_hide: подсказка не ловит указатель");
    fire(&host, "mouseleave");
    settle().await;
    assert!(tip().is_none(), "mouseleave скрывает");
    fire(&host, "mouseenter");
    settle().await;
    assert!(tip().is_some());
    host.click();
    settle().await;
    assert!(tip().is_none(), "click скрывает");
    assert_eq!(host.get_attribute("class").as_deref(), Some("host"), "атрибуты элемента не тронуты");
    end(handle, c).await;
}

async fn content_text_html_and_view() {
    let (handle, host, c) = mount(TooltipOptions::text("<b>x</b>"));
    fire(&host, "mouseenter");
    settle().await;
    assert_eq!(q(".p-tooltip-text").text_content().as_deref(), Some("<b>x</b>"));
    assert_eq!(count(".p-tooltip-text b"), 0, "текст не разбирается как разметка");
    end(handle, c).await;

    let (handle, host, c) = mount(TooltipOptions::html("<b id=\"bold\">x</b>"));
    fire(&host, "mouseenter");
    settle().await;
    assert_eq!(q(".p-tooltip-text #bold").text_content().as_deref(), Some("x"));
    end(handle, c).await;

    let (handle, host, c) = mount(TooltipOptions::view(|| view! { <span id="custom">"Custom"</span> }));
    fire(&host, "mouseenter");
    settle().await;
    assert_eq!(q(".p-tooltip-text #custom").text_content().as_deref(), Some("Custom"));
    fire(&host, "mouseleave");
    settle().await;
    assert!(tip().is_none() && find("#custom").is_none());
    end(handle, c).await;

    let (handle, host, c) = mount(TooltipOptions::text(""));
    fire(&host, "mouseenter");
    settle().await;
    assert!(tip().is_none(), "пустое содержимое не показывает подсказку");
    end(handle, c).await;
}

async fn each_side_is_placed_next_to_the_host() {
    let cases = [
        (TooltipPosition::Right, "p-tooltip-right"),
        (TooltipPosition::Left, "p-tooltip-left"),
        (TooltipPosition::Top, "p-tooltip-top"),
        (TooltipPosition::Bottom, "p-tooltip-bottom"),
    ];
    for (position, class) in cases {
        let (handle, host, c) = mount(TooltipOptions::text("Tip").position(position));
        fire(&host, "mouseenter");
        settle().await;
        let t = tip().unwrap();
        assert!(t.class_name().contains(class), "{class}: {}", t.class_name());
        let (h, p) = (rect(&host), rect(&t));
        match position {
            TooltipPosition::Right => assert!(p.0 >= h.2 - 1.0 && (p.1 + p.3 - h.1 - h.3).abs() < 2.0, "справа и по центру: {h:?} {p:?}"),
            TooltipPosition::Left => assert!(p.2 <= h.0 + 1.0, "слева: {h:?} {p:?}"),
            TooltipPosition::Top => assert!(p.3 <= h.1 + 1.0 && (p.0 + p.2 - h.0 - h.2).abs() < 2.0, "сверху и по центру: {h:?} {p:?}"),
            TooltipPosition::Bottom => assert!(p.1 >= h.3 - 1.0, "снизу: {h:?} {p:?}"),
        }
        let arrow = q(".p-tooltip-arrow").style();
        match position {
            TooltipPosition::Right => assert_eq!(
                (arrow.get_property_value("top").unwrap(), arrow.get_property_value("left").unwrap()),
                ("50%".into(), "0px".into())
            ),
            TooltipPosition::Left => assert_eq!(arrow.get_property_value("right").unwrap(), "0px"),
            TooltipPosition::Top => assert_eq!(arrow.get_property_value("bottom").unwrap(), "0px"),
            TooltipPosition::Bottom => assert_eq!(arrow.get_property_value("top").unwrap(), "0px"),
        }
        end(handle, c).await;
    }
}

async fn moves_to_another_side_when_there_is_no_room() {
    install_css();
    let c = container();
    let o = TooltipOptions::text("Tip").position(TooltipPosition::Right);
    let handle = leptos::mount::mount_to(c.clone(), move || {
        view! { <div id="edge" style="position:absolute;right:0;top:250px;width:60px;height:40px" use:tooltip=o.clone()>"E"</div> }
    });
    let host: HtmlElement = c.query_selector("#edge").unwrap().unwrap().dyn_into().unwrap();
    fire(&host, "mouseenter");
    settle().await;
    assert!(tip().unwrap().class_name().contains("p-tooltip-left"), "нет места справа: {}", tip().unwrap().class_name());
    end(handle, c).await;

    let c = container();
    let o = TooltipOptions::text("Tip").position(TooltipPosition::Top);
    let handle = leptos::mount::mount_to(c.clone(), move || {
        view! { <div id="ceil" style="position:absolute;left:300px;top:0;width:100px;height:40px" use:tooltip=o.clone()>"C"</div> }
    });
    let host: HtmlElement = c.query_selector("#ceil").unwrap().unwrap().dyn_into().unwrap();
    fire(&host, "mouseenter");
    settle().await;
    assert!(tip().unwrap().class_name().contains("p-tooltip-bottom"), "нет места сверху: {}", tip().unwrap().class_name());
    end(handle, c).await;
}

async fn offsets_and_style_options_apply() {
    let (handle, host, c) = mount(TooltipOptions::text("Tip"));
    fire(&host, "mouseenter");
    settle().await;
    let base = rect(&tip().unwrap());
    end(handle, c).await;

    let options = TooltipOptions::text("Tip").offset(7, 20).style_class("custom wide").id("my-tip").z_index(77);
    let (handle, host, c) = mount(options);
    fire(&host, "mouseenter");
    settle().await;
    let t = tip().unwrap();
    let shifted = rect(&t);
    assert!((shifted.0 - base.0 - 20.0).abs() < 1.0 && (shifted.1 - base.1 - 7.0).abs() < 1.0, "{base:?} → {shifted:?}");
    assert!(t.class_name().contains("custom wide"), "{}", t.class_name());
    assert_eq!(t.get_attribute("id").as_deref(), Some("my-tip"));
    assert_eq!(t.style().get_property_value("z-index").unwrap(), "77");
    assert_eq!(t.style().get_property_value("width").unwrap(), "fit-content");
    end(handle, c).await;

    let (handle, host, c) = mount(TooltipOptions::text("Tip").fit_content(false).position_style("fixed"));
    fire(&host, "mouseenter");
    settle().await;
    let t = tip().unwrap();
    assert_eq!(t.style().get_property_value("width").unwrap(), "");
    assert_eq!(t.style().get_property_value("position").unwrap(), "fixed");
    assert!(t.style().get_property_value("z-index").unwrap().parse::<i32>().unwrap() > 0, "автоматический z-index");
    end(handle, c).await;
}

async fn delays_and_life() {
    let (handle, host, c) = mount(TooltipOptions::text("Tip").show_delay(150));
    fire(&host, "mouseenter");
    sleep(40).await;
    fire(&host, "mouseleave");
    sleep(250).await;
    assert!(tip().is_none(), "уход курсора до show_delay отменяет показ");
    fire(&host, "mouseenter");
    sleep(60).await;
    assert!(tip().is_none(), "ещё не показана");
    sleep(200).await;
    assert!(tip().is_some(), "показана после show_delay");
    end(handle, c).await;

    let (handle, host, c) = mount(TooltipOptions::text("Tip").hide_delay(150));
    fire(&host, "mouseenter");
    settle().await;
    fire(&host, "mouseleave");
    sleep(40).await;
    assert!(tip().is_some(), "ещё видна до hide_delay");
    sleep(250).await;
    assert!(tip().is_none(), "скрыта после hide_delay");
    end(handle, c).await;

    let (handle, host, c) = mount(TooltipOptions::text("Tip").life(150));
    fire(&host, "mouseenter");
    sleep(40).await;
    assert!(tip().is_some());
    sleep(250).await;
    assert!(tip().is_none(), "life скрывает показанную подсказку");
    end(handle, c).await;
}

async fn focus_event_and_both() {
    install_css();
    let c = container();
    let o = TooltipOptions::text("Focus tip").event(TooltipEvent::Focus);
    let handle = leptos::mount::mount_to(c.clone(), move || view! { <input id="field" class="host" use:tooltip=o.clone() /> });
    let input: HtmlElement = c.query_selector("#field").unwrap().unwrap().dyn_into().unwrap();
    fire(&input, "mouseenter");
    settle().await;
    assert!(tip().is_none(), "Focus: наведение не показывает");
    input.focus().unwrap();
    settle().await;
    assert!(tip().is_some(), "Focus: фокус показывает");
    input.blur().unwrap();
    settle().await;
    assert!(tip().is_none(), "blur скрывает");
    end(handle, c).await;

    let c = container();
    let o = TooltipOptions::text("Both").event(TooltipEvent::Both);
    let handle = leptos::mount::mount_to(c.clone(), move || view! { <input id="field" class="host" use:tooltip=o.clone() /> });
    let input: HtmlElement = c.query_selector("#field").unwrap().unwrap().dyn_into().unwrap();
    fire(&input, "mouseenter");
    settle().await;
    assert!(tip().is_some(), "Both: наведение");
    fire(&input, "mouseleave");
    settle().await;
    assert!(tip().is_none());
    input.focus().unwrap();
    settle().await;
    assert!(tip().is_some(), "Both: фокус");
    input.blur().unwrap();
    settle().await;
    end(handle, c).await;
}

async fn auto_hide_false_lets_the_pointer_reach_the_tooltip() {
    let (handle, host, c) = mount(TooltipOptions::text("Interact").auto_hide(false));
    fire(&host, "mouseenter");
    settle().await;
    let t = tip().unwrap();
    assert_eq!(t.style().get_property_value("pointer-events").unwrap(), "unset");
    leave_to(&host, Some(&t.clone().into()));
    settle().await;
    assert!(tip().is_some(), "курсор перешёл на подсказку — она остаётся");
    fire(&t, "mouseleave");
    settle().await;
    assert!(tip().is_none(), "уход с подсказки скрывает её");

    fire(&host, "mouseenter");
    settle().await;
    leave_to(&host, None);
    settle().await;
    assert!(tip().is_none(), "курсор ушёл не на подсказку — скрыта");
    end(handle, c).await;
}

async fn escape_resize_and_scroll_hide() {
    let (handle, host, c) = mount(TooltipOptions::text("Tip"));
    fire(&host, "mouseenter");
    settle().await;
    escape();
    settle().await;
    assert!(tip().is_none(), "Escape скрывает");
    fire(&host, "mouseleave");
    settle().await;
    fire(&host, "mouseenter");
    settle().await;
    window().dispatch_event(&web_sys::Event::new("resize").unwrap()).unwrap();
    settle().await;
    assert!(tip().is_none(), "resize скрывает");
    end(handle, c).await;

    let (handle, host, c) = mount(TooltipOptions::text("Tip").hide_on_escape(false));
    fire(&host, "mouseenter");
    settle().await;
    escape();
    settle().await;
    assert!(tip().is_some(), "hide_on_escape=false");
    doc().dispatch_event(&web_sys::Event::new("scroll").unwrap()).unwrap();
    settle().await;
    assert!(tip().is_some(), "прокрутка самой страницы не скрывает: подсказка едет вместе со страницей");
    let other = container();
    other.dispatch_event(&web_sys::Event::new("scroll").unwrap()).unwrap();
    settle().await;
    assert!(tip().is_some(), "прокрутка постороннего элемента не скрывает");
    other.remove();
    host.parent_element().unwrap().dispatch_event(&web_sys::Event::new("scroll").unwrap()).unwrap();
    settle().await;
    assert!(tip().is_none(), "прокрутка предка скрывает");
    end(handle, c).await;
}

async fn reactive_text_and_disabled() {
    install_css();
    let c = container();
    let text = RwSignal::new("First".to_owned());
    let disabled = RwSignal::new(false);
    let handle = leptos::mount::mount_to(c.clone(), move || {
        let o = TooltipOptions::text_signal(text).disabled(disabled);
        view! { <div id="host" class="host" use:tooltip=o.clone()>"Host"</div> }
    });
    let host: HtmlElement = c.query_selector("#host").unwrap().unwrap().dyn_into().unwrap();
    fire(&host, "mouseenter");
    settle().await;
    assert_eq!(q(".p-tooltip-text").text_content().as_deref(), Some("First"));
    text.set("Second".to_owned());
    settle().await;
    assert_eq!(q(".p-tooltip-text").text_content().as_deref(), Some("Second"), "текст обновился");
    text.set(String::new());
    settle().await;
    assert!(tip().is_none(), "пустой текст скрывает подсказку");
    text.set("Third".to_owned());
    settle().await;
    assert_eq!(q(".p-tooltip-text").text_content().as_deref(), Some("Third"), "новый текст показывается снова");
    disabled.set(true);
    settle().await;
    assert!(tip().is_none(), "disabled скрывает показанную");
    fire(&host, "mouseleave");
    fire(&host, "mouseenter");
    settle().await;
    assert!(tip().is_none(), "disabled не показывает");
    disabled.set(false);
    fire(&host, "mouseleave");
    fire(&host, "mouseenter");
    settle().await;
    assert!(tip().is_some());
    end(handle, c).await;
}

async fn show_on_ellipsis_needs_overflowing_text() {
    install_css();
    for (text, shown) in [("A very long text that does not fit in twenty pixels", true), ("x", false)] {
        let c = container();
        let o = TooltipOptions::text("Full").show_on_ellipsis(true);
        let handle = leptos::mount::mount_to(c.clone(), move || {
            view! { <div id="clip" style="position:absolute;left:300px;top:250px;width:30px;overflow:hidden;white-space:nowrap" use:tooltip=o.clone()>{text}</div> }
        });
        let host: HtmlElement = c.query_selector("#clip").unwrap().unwrap().dyn_into().unwrap();
        fire(&host, "mouseenter");
        settle().await;
        assert_eq!(tip().is_some(), shown, "{text}");
        end(handle, c).await;
    }
}

async fn component_roots_and_cleanup() {
    install_css();
    let c = container();
    let handle = leptos::mount::mount_to(c.clone(), || {
        view! {
            <Button label="Save" use:tooltip=TooltipOptions::text("Confirm to proceed").position(TooltipPosition::Bottom) />
            <InputText placeholder="name" use:tooltip=TooltipOptions::text("Enter your username").event(TooltipEvent::Focus) />
        }
    });
    leptos::task::tick().await;
    let button: HtmlElement = c.query_selector("button").unwrap().unwrap().dyn_into().unwrap();
    let input: HtmlElement = c.query_selector("input").unwrap().unwrap().dyn_into().unwrap();
    fire(&button, "mouseenter");
    settle().await;
    assert_eq!(q(".p-tooltip-text").text_content().as_deref(), Some("Confirm to proceed"), "директива на корне Button");
    fire(&button, "mouseleave");
    input.focus().unwrap();
    settle().await;
    assert_eq!(q(".p-tooltip-text").text_content().as_deref(), Some("Enter your username"), "директива на корне InputText");
    // размонтирование убирает показанную подсказку
    drop(handle);
    settle().await;
    assert!(tip().is_none(), "после размонтирования подсказки нет");
    c.remove();
}

async fn applying_twice_does_not_duplicate_listeners() {
    install_css();
    let c = container();
    let host = doc().create_element("div").unwrap();
    host.set_class_name("host");
    host.set_text_content(Some("Host"));
    c.append_child(&host).unwrap();
    let owner = Owner::new();
    owner.with(|| {
        tooltip(host.clone(), TooltipOptions::text("One"));
        tooltip(host.clone(), TooltipOptions::text("Two"));
    });
    fire(&host, "mouseenter");
    settle().await;
    assert_eq!(count(".p-tooltip"), 1, "одна подсказка");
    assert_eq!(q(".p-tooltip-text").text_content().as_deref(), Some("Two"), "параметры обновились");
    fire(&host, "mouseleave");
    settle().await;
    assert_eq!(count(".p-tooltip"), 0);
    drop(owner);
    c.remove();
}

/// Сценарии идут по очереди: подсказки выводятся в общий `body`, а слушатели окна общие.
#[wasm_bindgen_test]
async fn tooltip_behaviour() {
    hover_shows_and_leave_or_click_hide().await;
    content_text_html_and_view().await;
    each_side_is_placed_next_to_the_host().await;
    moves_to_another_side_when_there_is_no_room().await;
    offsets_and_style_options_apply().await;
    delays_and_life().await;
    focus_event_and_both().await;
    auto_hide_false_lets_the_pointer_reach_the_tooltip().await;
    escape_resize_and_scroll_hide().await;
    reactive_text_and_disabled().await;
    show_on_ellipsis_needs_overflowing_text().await;
    component_roots_and_cleanup().await;
    applying_twice_does_not_duplicate_listeners().await;
}
