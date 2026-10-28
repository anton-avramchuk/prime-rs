//! Поведение Button в реальном браузере (wasm-bindgen-test).
//!
//! Каждый сценарий выполняется в обеих ветвях отрисовки: статической (все свойства — значения) и реактивной
//! (одно свойство передано сигналом, чего достаточно для выбора реактивного пути).
#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use prime_button::Button;
use prime_core::{OptProp, PrimeConfig, Prop, provide_prime_config};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use web_sys::{HtmlElement, MouseEvent, MouseEventInit};

wasm_bindgen_test_configure!(run_in_browser);

#[derive(Clone, Copy, Debug)]
enum Mode {
    Static,
    Live,
}

const MODES: [Mode; 2] = [Mode::Static, Mode::Live];

/// Свойство-«переключатель»: значение по умолчанию (статика) или сигнал (реактивная ветвь).
fn path(mode: Mode) -> OptProp<String> {
    match mode {
        Mode::Static => Prop::default(),
        Mode::Live => Prop::Reactive(RwSignal::new(None).into()),
    }
}

fn doc() -> web_sys::Document {
    web_sys::window().unwrap().document().unwrap()
}

fn host() -> HtmlElement {
    let el = doc().create_element("div").unwrap().dyn_into::<HtmlElement>().unwrap();
    doc().body().unwrap().append_child(&el).unwrap();
    el
}

fn button_in(host: &HtmlElement) -> HtmlElement {
    host.query_selector("button").unwrap().expect("button в host").dyn_into().unwrap()
}

fn counter() -> (Arc<AtomicUsize>, Callback<MouseEvent>) {
    let n = Arc::new(AtomicUsize::new(0));
    let n2 = n.clone();
    (
        n,
        Callback::new(move |_: MouseEvent| {
            n2.fetch_add(1, Ordering::SeqCst);
        }),
    )
}

#[wasm_bindgen_test]
async fn click_calls_on_click_once() {
    for mode in MODES {
        let (n, cb) = counter();
        let host = host();
        let _h = leptos::mount::mount_to(host.clone(), move || view! { <Button label="x" on_click=cb aria_label=path(mode) /> });
        leptos::task::tick().await;
        button_in(&host).click();
        assert_eq!(n.load(Ordering::SeqCst), 1, "{mode:?}");
        host.remove();
    }
}

#[wasm_bindgen_test]
async fn disabled_and_loading_buttons_do_not_call_on_click() {
    for mode in MODES {
        let (n, cb) = counter();
        let host = host();
        let _h = leptos::mount::mount_to(host.clone(), move || {
            view! {
                <Button label="a" disabled=true on_click=cb aria_label=path(mode) />
                <Button label="b" loading=true on_click=cb aria_label=path(mode) />
            }
        });
        leptos::task::tick().await;
        let list = host.query_selector_all("button").unwrap();
        assert_eq!(list.length(), 2);
        for i in 0..2 {
            let b = list.item(i).unwrap().dyn_into::<HtmlElement>().unwrap();
            b.click();
            // нативный click() на disabled не срабатывает; синтетическое событие тоже не должно вызывать колбэк
            let ev = MouseEvent::new_with_mouse_event_init_dict("click", &MouseEventInit::new()).unwrap();
            b.dispatch_event(&ev).unwrap();
        }
        assert_eq!(n.load(Ordering::SeqCst), 0, "{mode:?}");
        host.remove();
    }
}

#[wasm_bindgen_test]
async fn focus_and_blur_callbacks() {
    for mode in MODES {
        let focused = Arc::new(AtomicUsize::new(0));
        let blurred = Arc::new(AtomicUsize::new(0));
        let (f, b) = (focused.clone(), blurred.clone());
        let host = host();
        let _h = leptos::mount::mount_to(host.clone(), move || {
            view! {
                <Button
                    label="x"
                    aria_label=path(mode)
                    on_focus=Callback::new(move |_: web_sys::FocusEvent| {
                        f.fetch_add(1, Ordering::SeqCst);
                    })
                    on_blur=Callback::new(move |_: web_sys::FocusEvent| {
                        b.fetch_add(1, Ordering::SeqCst);
                    })
                />
            }
        });
        leptos::task::tick().await;
        let btn = button_in(&host);
        btn.focus().unwrap();
        assert_eq!(focused.load(Ordering::SeqCst), 1, "{mode:?}");
        btn.blur().unwrap();
        assert_eq!(blurred.load(Ordering::SeqCst), 1, "{mode:?}");
        host.remove();
    }
}

#[wasm_bindgen_test]
async fn autofocus_focuses_the_button_after_mount() {
    for mode in MODES {
        let host = host();
        let _h = leptos::mount::mount_to(host.clone(), move || view! { <Button label="x" autofocus=true aria_label=path(mode) /> });
        leptos::task::tick().await;
        let active = doc().active_element().expect("есть активный элемент");
        assert!(active.is_same_node(Some(&button_in(&host))), "{mode:?}");
        host.remove();
    }
}

fn mousedown(el: &HtmlElement) {
    let ev = MouseEvent::new_with_mouse_event_init_dict("mousedown", &MouseEventInit::new()).unwrap();
    el.dispatch_event(&ev).unwrap();
}

#[wasm_bindgen_test]
async fn ripple_enabled_creates_ink_and_activates_on_mousedown() {
    for mode in MODES {
        let host = host();
        let _h = leptos::mount::mount_to(host.clone(), move || {
            provide_prime_config(PrimeConfig { ripple: true, ..Default::default() });
            view! { <Button label="x" aria_label=path(mode) /> }
        });
        leptos::task::tick().await;
        let btn = button_in(&host);
        let ink = btn.query_selector(".p-ink").unwrap().expect("span.p-ink");
        assert_eq!(ink.get_attribute("data-p-ink-active").as_deref(), Some("false"), "{mode:?}");
        mousedown(&btn);
        assert_eq!(ink.get_attribute("data-p-ink-active").as_deref(), Some("true"), "{mode:?}");
        host.remove();
    }
}

#[wasm_bindgen_test]
async fn ripple_disabled_has_no_ink() {
    for mode in MODES {
        let host = host();
        let _h = leptos::mount::mount_to(host.clone(), move || view! { <Button label="x" aria_label=path(mode) /> });
        leptos::task::tick().await;
        let btn = button_in(&host);
        assert!(btn.class_list().contains("p-ripple"), "{mode:?}");
        assert!(btn.query_selector(".p-ink").unwrap().is_none(), "{mode:?}");
        host.remove();
    }
}

#[wasm_bindgen_test]
async fn unmount_removes_button_and_ink() {
    for mode in MODES {
        let host = host();
        let handle = leptos::mount::mount_to(host.clone(), move || {
            provide_prime_config(PrimeConfig { ripple: true, ..Default::default() });
            view! { <Button label="x" aria_label=path(mode) /> }
        });
        leptos::task::tick().await;
        assert!(host.query_selector(".p-ink").unwrap().is_some(), "{mode:?}");
        drop(handle);
        leptos::task::tick().await;
        assert!(host.query_selector("button").unwrap().is_none(), "{mode:?}");
        assert!(doc().query_selector(".p-ink").unwrap().is_none(), "{mode:?}");
        host.remove();
    }
}

#[wasm_bindgen_test]
async fn class_is_rewritten_only_when_class_inputs_change() {
    let aria = RwSignal::new(Some("a".to_owned()));
    let loading = RwSignal::new(false);
    let host = host();
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <Button label="x" aria_label=aria loading=loading /> });
    leptos::task::tick().await;
    let btn = button_in(&host);

    // «Метка»: если атрибут class перезапишут, она исчезнет.
    btn.set_attribute("class", &format!("{} sentinel", btn.class_name())).unwrap();

    aria.set(Some("b".into())); // не влияет на классы
    leptos::task::tick().await;
    assert_eq!(btn.get_attribute("aria-label").as_deref(), Some("b"));
    assert!(btn.class_name().contains("sentinel"), "class перезаписан без причины: {}", btn.class_name());

    loading.set(true); // влияет на классы
    leptos::task::tick().await;
    assert!(btn.class_name().contains("p-button-loading"));
    assert!(!btn.class_name().contains("sentinel"), "class должен быть пересчитан: {}", btn.class_name());
    host.remove();
}

#[wasm_bindgen_test]
async fn static_path_has_no_reactive_updates_but_reactive_path_has() {
    let host = host();
    let signal = RwSignal::new(Some("1".to_owned()));
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! {
            <div data-testid="s"><Button label="static" /></div>
            <div data-testid="l"><Button label=signal /></div>
        }
    });
    leptos::task::tick().await;
    let text = |id: &str| host.query_selector(&format!("[data-testid={id}] .p-button-label")).unwrap().unwrap().text_content().unwrap();
    assert_eq!((text("s"), text("l")), ("static".to_owned(), "1".to_owned()));
    signal.set(Some("2".into()));
    leptos::task::tick().await;
    assert_eq!(text("l"), "2");
    assert_eq!(text("s"), "static");
    host.remove();
}
