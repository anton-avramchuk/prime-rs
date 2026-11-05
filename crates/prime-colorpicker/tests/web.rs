//! Поведение ColorPicker в реальном браузере (wasm-bindgen-test): открытие и закрытие, выбор цвета по полю и шкале,
//! перетаскивание, форматы, внешнее значение, блокировка, встроенная панель.
//!
//! Сценарии идут одним последовательным тестом: слушатели `window` общие на страницу. Стили темы не подключены,
//! поэтому размеры областей задаёт локальный `<style>`.
#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use prime_colorpicker::color::Rgb;
use prime_colorpicker::{ColorFormat, ColorPicker, ColorValue};
use std::sync::{Arc, Mutex};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;
use web_sys::{HtmlElement, KeyboardEvent, KeyboardEventInit, MouseEvent, MouseEventInit};

wasm_bindgen_test_configure!(run_in_browser);

type Log = Arc<Mutex<Vec<String>>>;

const CSS: &str = ".p-colorpicker{display:inline-block;position:relative}\
.p-colorpicker-preview{width:24px;height:24px;padding:0;border:0}\
.p-colorpicker-panel{position:absolute;top:0;left:0;width:193px;height:166px}\
.p-colorpicker-panel-inline{position:static}\
.p-colorpicker-content{position:relative}\
.p-colorpicker-color-selector{position:absolute;left:8px;top:8px;width:150px;height:150px}\
.p-colorpicker-hue{position:absolute;left:167px;top:8px;width:17px;height:150px}";

fn doc() -> web_sys::Document {
    web_sys::window().unwrap().document().unwrap()
}

fn host() -> HtmlElement {
    let el = doc().create_element("div").unwrap().dyn_into::<HtmlElement>().unwrap();
    el.set_attribute("style", "margin: 40px; height: 400px").unwrap();
    doc().body().unwrap().append_child(&el).unwrap();
    el
}

fn add_style() {
    let style = doc().create_element("style").unwrap();
    style.set_text_content(Some(CSS));
    doc().head().unwrap().append_child(&style).unwrap();
}

async fn sleep(ms: i32) {
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms).unwrap();
    });
    JsFuture::from(promise).await.unwrap();
}

/// Даёт эффектам и таймерам (подписки окна вешаются через `set_timeout(0)`) примениться.
async fn settle() {
    sleep(40).await;
}

fn q(host: &HtmlElement, selector: &str) -> Option<HtmlElement> {
    host.query_selector(selector).unwrap().map(|e| e.dyn_into().unwrap())
}

fn must(host: &HtmlElement, selector: &str) -> HtmlElement {
    q(host, selector).unwrap_or_else(|| panic!("нет {selector}: {}", host.outer_html()))
}

fn mouse(target: &web_sys::EventTarget, kind: &str, x: f64, y: f64) {
    let init = MouseEventInit::new();
    init.set_bubbles(true);
    init.set_cancelable(true);
    init.set_client_x(x as i32);
    init.set_client_y(y as i32);
    target.dispatch_event(&MouseEvent::new_with_mouse_event_init_dict(kind, &init).unwrap()).unwrap();
}

/// Точка внутри области: смещение от её левого верхнего угла.
fn at(el: &HtmlElement, dx: f64, dy: f64) -> (f64, f64) {
    let r = el.get_bounding_client_rect();
    (r.left() + dx, r.top() + dy)
}

fn click_at(el: &HtmlElement, dx: f64, dy: f64) {
    let (x, y) = at(el, dx, dy);
    mouse(el, "mousedown", x, y);
    mouse(el, "mouseup", x, y);
}

fn key(el: &HtmlElement, code: &str) -> bool {
    let init = KeyboardEventInit::new();
    init.set_code(code);
    init.set_bubbles(true);
    init.set_cancelable(true);
    el.dispatch_event(&KeyboardEvent::new_with_keyboard_event_init_dict("keydown", &init).unwrap()).unwrap()
}

fn hex(value: RwSignal<Option<ColorValue>>) -> String {
    match value.get_untracked() {
        Some(ColorValue::Hex(h)) => h,
        other => panic!("ожидался hex: {other:?}"),
    }
}

async fn open_close_and_events() {
    let (host, log) = (host(), Log::default());
    let (l1, l2) = (log.clone(), log.clone());
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! {
            <ColorPicker
                on_show=Callback::new(move |_| l1.lock().unwrap().push("show".into()))
                on_hide=Callback::new(move |_| l2.lock().unwrap().push("hide".into()))
            />
        }
    });
    settle().await;
    let preview = must(&host, ".p-colorpicker-preview");
    assert!(q(&host, ".p-colorpicker-panel").is_none(), "закрыт");
    preview.click();
    settle().await;
    assert!(q(&host, ".p-colorpicker-panel").is_some(), "клик открывает");
    assert_eq!(log.lock().unwrap().as_slice(), ["show"]);
    preview.click();
    settle().await;
    assert!(q(&host, ".p-colorpicker-panel").is_none(), "повторный клик закрывает");
    assert_eq!(log.lock().unwrap().as_slice(), ["show", "hide"]);

    assert!(!key(&preview, "Space"), "Space поглощается (preventDefault)");
    settle().await;
    assert!(q(&host, ".p-colorpicker-panel").is_some(), "Space открывает");
    key(&preview, "Space");
    settle().await;
    assert!(q(&host, ".p-colorpicker-panel").is_none(), "Space закрывает");
    key(&preview, "Space");
    settle().await;
    key(&preview, "Escape");
    settle().await;
    assert!(q(&host, ".p-colorpicker-panel").is_none(), "Escape закрывает");
    key(&preview, "Space");
    settle().await;
    key(&preview, "Tab");
    settle().await;
    assert!(q(&host, ".p-colorpicker-panel").is_none(), "Tab закрывает");
    let events = log.lock().unwrap().clone();
    assert_eq!(events.iter().filter(|l| *l == "show").count(), events.iter().filter(|l| *l == "hide").count());

    // клик снаружи
    preview.click();
    settle().await;
    assert!(q(&host, ".p-colorpicker-panel").is_some());
    let body = doc().body().unwrap();
    mouse(&body, "click", 5.0, 5.0);
    settle().await;
    assert!(q(&host, ".p-colorpicker-panel").is_none(), "клик вне корня закрывает");
    // клик внутри корня не закрывает
    preview.click();
    settle().await;
    let panel = must(&host, ".p-colorpicker-panel");
    let (x, y) = at(&panel, 5.0, 5.0);
    mouse(&panel, "click", x, y);
    settle().await;
    assert!(q(&host, ".p-colorpicker-panel").is_some(), "клик по панели не закрывает");
    mouse(&body, "click", 5.0, 5.0);
    settle().await;
    host.remove();
}

async fn picking_dragging_and_formats() {
    let (host, log) = (host(), Log::default());
    let value = RwSignal::new(None);
    let l = log.clone();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! {
            <ColorPicker
                inline=true
                value=value
                on_change=Callback::new(move |v: ColorValue| l.lock().unwrap().push(format!("{v:?}")))
            />
        }
    });
    settle().await;
    assert!(q(&host, ".p-colorpicker-preview").is_none() && q(&host, ".p-colorpicker-panel-inline").is_some());
    let field = must(&host, ".p-colorpicker-color-selector");
    let hue = must(&host, ".p-colorpicker-hue");
    let handle = must(&host, ".p-colorpicker-color-handle");
    let hue_handle = must(&host, ".p-colorpicker-hue-handle");

    click_at(&field, 150.0, 0.0);
    settle().await;
    assert_eq!(hex(value), "#ff0000", "правый верхний угол: s = 100, b = 100");
    click_at(&field, 75.0, 75.0);
    settle().await;
    assert_eq!(hex(value), "#804040", "s = 50, b = 50");
    assert_eq!(handle.style().get_property_value("left").unwrap(), "75px");
    assert_eq!(handle.style().get_property_value("top").unwrap(), "75px");
    click_at(&hue, 8.0, 75.0);
    settle().await;
    assert_eq!(hex(value), "#408080", "оттенок 180");
    assert_eq!(hue_handle.style().get_property_value("top").unwrap(), "75px");
    assert!(field.style().get_property_value("background-color").unwrap().contains("0, 255, 255"), "фон поля — чистый оттенок");
    assert_eq!(log.lock().unwrap().len(), 3, "on_change на каждое изменение");

    // перетаскивание: нажатие, выход за границы, отпускание
    let root = must(&host, ".p-colorpicker");
    let (x, y) = at(&field, 10.0, 10.0);
    mouse(&field, "mousedown", x, y);
    settle().await;
    assert!(root.class_name().contains("p-colorpicker-dragging"), "во время жеста: {}", root.class_name());
    let window: web_sys::EventTarget = web_sys::window().unwrap().into();
    let (fx, fy) = at(&field, 500.0, 500.0);
    mouse(&window, "mousemove", fx, fy);
    settle().await;
    assert_eq!(hex(value), "#000000", "за правым нижним углом: s = 100, b = 0");
    mouse(&window, "mouseup", fx, fy);
    settle().await;
    assert!(!root.class_name().contains("p-colorpicker-dragging"));
    let (bx, by) = at(&field, 0.0, 0.0);
    mouse(&window, "mousemove", bx, by);
    settle().await;
    assert_eq!(hex(value), "#000000", "после отпускания движение значение не меняет");

    // шкала: ниже нижней границы оттенок 0
    let (x, y) = at(&hue, 8.0, 10.0);
    mouse(&hue, "mousedown", x, y);
    let (lx, ly) = at(&hue, 8.0, 900.0);
    mouse(&window, "mousemove", lx, ly);
    mouse(&window, "mouseup", lx, ly);
    settle().await;
    assert_eq!(hue_handle.style().get_property_value("top").unwrap(), "150px", "оттенок ограничен 0");
    host.remove();

    // форматы rgb и hsb
    let host = host_for_format();
    let rgb = RwSignal::new(None);
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <ColorPicker inline=true format=ColorFormat::Rgb value=rgb /> });
    settle().await;
    click_at(&must(&host, ".p-colorpicker-color-selector"), 150.0, 0.0);
    settle().await;
    assert_eq!(rgb.get_untracked(), Some(ColorValue::Rgb(Rgb { r: 255, g: 0, b: 0 })));
    host.remove();
    let host = host_for_format();
    let hsb = RwSignal::new(None);
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <ColorPicker inline=true format=ColorFormat::Hsb value=hsb /> });
    settle().await;
    click_at(&must(&host, ".p-colorpicker-color-selector"), 75.0, 75.0);
    settle().await;
    match hsb.get_untracked() {
        Some(ColorValue::Hsb(c)) => assert_eq!((c.h, c.s, c.b), (0.0, 50.0, 50.0)),
        other => panic!("{other:?}"),
    }
    host.remove();
}

fn host_for_format() -> HtmlElement {
    host()
}

async fn external_value_and_disabled() {
    let host = host();
    let value = RwSignal::new(Some(ColorValue::Hex("#00ff00".into())));
    let disabled = RwSignal::new(false);
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <ColorPicker inline=true value=value disabled=disabled /> });
    settle().await;
    let hue_handle = must(&host, ".p-colorpicker-hue-handle");
    assert_eq!(hue_handle.style().get_property_value("top").unwrap(), "100px", "зелёный: оттенок 120");
    value.set(Some(ColorValue::Hex("#0000ff".into())));
    settle().await;
    assert_eq!(hue_handle.style().get_property_value("top").unwrap(), "50px", "внешнее значение сдвигает указатель (синий: 240)");
    value.set(None);
    settle().await;
    assert_eq!(hue_handle.style().get_property_value("top").unwrap(), "150px", "пустое значение — цвет по умолчанию (красный)");

    value.set(Some(ColorValue::Hex("#00ff00".into())));
    settle().await;
    disabled.set(true);
    settle().await;
    let field = must(&host, ".p-colorpicker-color-selector");
    click_at(&field, 150.0, 150.0);
    settle().await;
    assert_eq!(hex(value), "#00ff00", "заблокированный не реагирует");
    host.remove();
}

async fn disabled_preview_does_not_open() {
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <ColorPicker disabled=true /> });
    settle().await;
    let preview = must(&host, ".p-colorpicker-preview");
    assert!(preview.has_attribute("disabled"));
    preview.click();
    settle().await;
    assert!(q(&host, ".p-colorpicker-panel").is_none());
    host.remove();
}

#[wasm_bindgen_test]
async fn colorpicker_in_the_browser() {
    add_style();
    open_close_and_events().await;
    picking_dragging_and_formats().await;
    external_value_and_disabled().await;
    disabled_preview_does_not_open().await;
}
