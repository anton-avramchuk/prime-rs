//! InputMask behaviour in a real browser (wasm-bindgen-test): typing by the mask, deletion with shifts, paste, blur,
//! `unmask`, external value, clearing, `readonly`.
//!
//! The scenarios run as one sequential test: focus is shared by the page. Theme styles are not connected.
#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use prime_inputmask::InputMask;
use std::sync::{Arc, Mutex};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;
use web_sys::{HtmlElement, HtmlInputElement, KeyboardEvent, KeyboardEventInit, MouseEvent, MouseEventInit};

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

fn input(host: &HtmlElement) -> HtmlInputElement {
    host.query_selector("input").unwrap().unwrap_or_else(|| panic!("no input: {}", host.outer_html())).dyn_into().unwrap()
}

async fn sleep(ms: i32) {
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms).unwrap();
    });
    JsFuture::from(promise).await.unwrap();
}

/// Gives effects and the focus timer (10 ms) time to apply.
async fn settle() {
    sleep(40).await;
}

fn press(el: &HtmlInputElement, key: &str) -> bool {
    let init = KeyboardEventInit::new();
    init.set_key(key);
    init.set_bubbles(true);
    init.set_cancelable(true);
    el.dispatch_event(&KeyboardEvent::new_with_keyboard_event_init_dict("keydown", &init).unwrap()).unwrap()
}

fn type_str(el: &HtmlInputElement, s: &str) {
    for ch in s.chars() {
        press(el, &ch.to_string());
    }
}

fn caret(el: &HtmlInputElement) -> (u32, u32) {
    (el.selection_start().unwrap().unwrap(), el.selection_end().unwrap().unwrap())
}

async fn typing_fills_the_mask() {
    let (host, log) = (host(), Log::default());
    let value = RwSignal::new(String::new());
    let (l1, l2, l3) = (log.clone(), log.clone(), log.clone());
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! {
            <InputMask
                mask="99-99"
                value=value
                on_complete=Callback::new(move |_| l1.lock().unwrap().push("complete".into()))
                on_input=Callback::new(move |_| l2.lock().unwrap().push("input".into()))
                on_unmasked_change=Callback::new(move |v: String| l3.lock().unwrap().push(format!("unmasked:{v}")))
            />
        }
    });
    settle().await;
    let i = input(&host);
    assert_eq!(i.value(), "");
    i.focus().unwrap();
    settle().await;
    assert_eq!(i.value(), "__-__", "the mask buffer is visible on focus");
    assert_eq!(caret(&i), (0, 0));
    assert!(!press(&i, "x"), "an unsuitable character is swallowed");
    assert_eq!(i.value(), "__-__");
    press(&i, "1");
    assert_eq!((i.value().as_str(), caret(&i)), ("1_-__", (1, 1)));
    press(&i, "2");
    assert_eq!((i.value().as_str(), caret(&i)), ("12-__", (3, 3)), "the constant character is skipped");
    assert_eq!(value.get_untracked(), "12-__", "the value follows the field");
    assert!(log.lock().unwrap().iter().all(|l| l != "complete"));
    press(&i, "3");
    press(&i, "4");
    assert_eq!(i.value(), "12-34");
    assert_eq!(value.get_untracked(), "12-34");
    assert_eq!(log.lock().unwrap().iter().filter(|l| *l == "complete").count(), 1, "on_complete when filled");
    assert!(log.lock().unwrap().contains(&"unmasked:1234".to_owned()));
    press(&i, "5");
    assert_eq!(i.value(), "12-34", "input beyond the mask is ignored");
    i.blur().unwrap();
    settle().await;
    assert_eq!(i.value(), "12-34", "a full value stays");
    host.remove();
}

async fn deleting_shifts_left() {
    let host = host();
    let value = RwSignal::new("1234".to_owned());
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <InputMask mask="99-99" value=value /> });
    settle().await;
    let i = input(&host);
    assert_eq!(i.value(), "12-34", "the initial value is laid out by the mask");
    i.focus().unwrap();
    settle().await;
    assert_eq!(caret(&i), (0, 5), "a full mask is selected entirely");
    i.set_selection_range(5, 5).unwrap();
    press(&i, "Backspace");
    assert_eq!((i.value().as_str(), caret(&i)), ("12-3_", (4, 4)), "Backspace removes the character on the left");
    i.set_selection_range(1, 1).unwrap();
    press(&i, "Delete");
    assert_eq!(i.value(), "13-__", "Delete removes the character on the right with a shift");
    i.set_selection_range(0, 5).unwrap();
    press(&i, "Backspace");
    assert_eq!(i.value(), "__-__", "the selection is removed entirely");
    assert_eq!(value.get_untracked(), "__-__");
    i.blur().unwrap();
    settle().await;
    assert_eq!(i.value(), "", "an empty mask is cleared on blur");
    assert_eq!(value.get_untracked(), "");
    host.remove();
}

async fn blur_clears_incomplete_and_keeps_with_auto_clear_off() {
    let host = host();
    let value = RwSignal::new(String::new());
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <InputMask mask="99-99" value=value /> });
    settle().await;
    let i = input(&host);
    i.focus().unwrap();
    settle().await;
    type_str(&i, "12");
    i.blur().unwrap();
    settle().await;
    assert_eq!((i.value().as_str(), value.get_untracked().as_str()), ("", ""), "an incomplete value is cleared");
    host.remove();

    let host = self::host();
    let value = RwSignal::new(String::new());
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <InputMask mask="99-99" value=value auto_clear=false slot_char="#" /> });
    settle().await;
    let i = input(&host);
    i.focus().unwrap();
    settle().await;
    assert_eq!(i.value(), "##-##");
    type_str(&i, "12");
    i.blur().unwrap();
    settle().await;
    assert_eq!(i.value(), "12-##", "without auto_clear it stays");
    assert_eq!(value.get_untracked(), "12-##");
    host.remove();
}

async fn paste_and_input_are_laid_out_by_the_mask() {
    let host = host();
    let value = RwSignal::new(String::new());
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <InputMask mask="(999) 999-9999" value=value /> });
    settle().await;
    let i = input(&host);
    i.focus().unwrap();
    settle().await;
    i.set_value("555-123-4567");
    i.dispatch_event(&web_sys::Event::new("input").unwrap()).unwrap();
    sleep(60).await;
    assert_eq!(i.value(), "(555) 123-4567", "the pasted text is distributed over the mask");
    assert_eq!(value.get_untracked(), "(555) 123-4567");
    host.remove();
}

async fn unmask_escape_readonly_and_external_value() {
    let host = host();
    let value = RwSignal::new(String::new());
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <InputMask mask="(999) 99a" value=value unmask=true /> });
    settle().await;
    let i = input(&host);
    i.focus().unwrap();
    settle().await;
    type_str(&i, "12z");
    assert_eq!(value.get_untracked(), "12", "with unmask the value is the entered characters");
    assert_eq!(i.value(), "(12_) __a".replace('a', "_"));
    press(&i, "3");
    press(&i, "4");
    press(&i, "5");
    press(&i, "q");
    assert_eq!(value.get_untracked(), "12345q");
    assert_eq!(i.value(), "(123) 45q");
    // Escape returns the text as it was at focus.
    i.blur().unwrap();
    settle().await;
    i.focus().unwrap();
    settle().await;
    press(&i, "Backspace");
    press(&i, "Escape");
    assert_eq!(i.value(), "(123) 45q", "Escape restores the text");
    // External value.
    value.set("98765z".to_owned());
    settle().await;
    assert_eq!(i.value(), "(987) 65z", "the external value is recalculated by the mask");
    host.remove();

    let host = self::host();
    let value = RwSignal::new(String::new());
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <InputMask mask="99" value=value readonly=true /> });
    settle().await;
    let i = input(&host);
    i.focus().unwrap();
    settle().await;
    assert_eq!(i.value(), "", "in read-only mode the buffer is not shown");
    press(&i, "1");
    assert_eq!(i.value(), "");
    host.remove();
}

async fn clear_icon_resets() {
    let (host, log) = (host(), Log::default());
    let value = RwSignal::new("1234".to_owned());
    let l = log.clone();
    let _h = leptos::mount::mount_to(host.clone(), move || {
        view! { <InputMask mask="99-99" value=value show_clear=true on_clear=Callback::new(move |_| l.lock().unwrap().push("clear".into())) /> }
    });
    settle().await;
    let icon = host.query_selector(".p-inputmask-clear-icon").unwrap().expect("clear icon");
    let init = MouseEventInit::new();
    init.set_bubbles(true);
    icon.dispatch_event(&MouseEvent::new_with_mouse_event_init_dict("click", &init).unwrap()).unwrap();
    settle().await;
    assert_eq!((input(&host).value().as_str(), value.get_untracked().as_str()), ("", ""));
    assert!(host.query_selector(".p-inputmask-clear-icon").unwrap().is_none(), "the icon disappears");
    assert_eq!(log.lock().unwrap().as_slice(), ["clear"]);
    let i = input(&host);
    i.focus().unwrap();
    settle().await;
    assert_eq!(i.value(), "__-__", "after clearing the buffer is empty again");
    host.remove();
}

async fn typing_right_after_focus_is_not_reset_by_the_delayed_caret() {
    let host = host();
    let value = RwSignal::new(String::new());
    let _h = leptos::mount::mount_to(host.clone(), move || view! { <InputMask mask="99-99" value=value /> });
    settle().await;
    let i = input(&host);
    i.focus().unwrap();
    type_str(&i, "1234");
    sleep(60).await;
    assert_eq!(i.value(), "12-34", "the deferred buffer write (10 ms) does not overwrite what was already typed");
    host.remove();
}

#[wasm_bindgen_test]
async fn inputmask_in_the_browser() {
    typing_fills_the_mask().await;
    deleting_shifts_left().await;
    blur_clears_incomplete_and_keeps_with_auto_clear_off().await;
    paste_and_input_are_laid_out_by_the_mask().await;
    unmask_escape_readonly_and_external_value().await;
    clear_icon_resets().await;
    typing_right_after_focus_is_not_reset_by_the_delayed_caret().await;
}
