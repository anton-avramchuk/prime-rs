//! Разметка InputMask (SSR): корень, поле, начальное значение по маске, значок очистки, классы состояния, `pt`.
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_core::{Pt, Size, Variant};
use prime_inputmask::{InputMask, InputMaskPt};

fn render(f: impl FnOnce() -> String) -> String {
    Owner::new().with(f)
}

#[test]
fn empty_field_markup() {
    let html = render(|| view! { <InputMask mask="(999) 999-9999" placeholder="(999) 999-9999" /> }.to_html());
    assert!(html.contains("p-inputmask p-component p-inputwrapper") && html.contains("position: relative"), "{html}");
    assert!(html.contains("p-inputtext p-component") && !html.contains("p-filled") && html.contains("type=\"text\""), "{html}");
    assert!(html.contains("placeholder=\"(999) 999-9999\"") && html.contains("value=\"\""), "{html}");
    assert!(!html.contains("p-inputmask-clear-icon") && !html.contains("p-variant-filled"), "{html}");
}

#[test]
fn initial_value_is_laid_out_by_the_mask() {
    let html = render(|| {
        let value = RwSignal::new("5551234567".to_owned());
        view! { <InputMask mask="(999) 999-9999" value=value /> }.to_html()
    });
    assert!(html.contains("value=\"(555) 123-4567\"") && html.contains("p-filled"), "{html}");
    let html = render(|| {
        let value = RwSignal::new("12".to_owned());
        view! { <InputMask mask="99-99" value=value /> }.to_html()
    });
    assert!(html.contains("value=\"\""), "неполное значение с auto_clear очищается: {html}");
    let html = render(|| {
        let value = RwSignal::new("12".to_owned());
        view! { <InputMask mask="99-99" value=value auto_clear=false slot_char="#" /> }.to_html()
    });
    assert!(html.contains("value=\"12-##\""), "без auto_clear остаётся с заполнителями: {html}");
    let html = render(|| {
        let value = RwSignal::new("ab12".to_owned());
        view! { <InputMask mask="aa-99" value=value /> }.to_html()
    });
    assert!(html.contains("value=\"ab-12\""), "{html}");
}

#[test]
fn clear_icon_requires_value_show_clear_and_enabled() {
    let with = |show: bool, filled: bool, disabled: bool| {
        render(move || {
            let value = RwSignal::new(if filled { "1234".to_owned() } else { String::new() });
            view! { <InputMask mask="99-99" value=value show_clear=show disabled=disabled /> }.to_html()
        })
    };
    assert!(with(true, true, false).contains("p-inputmask-clear-icon"));
    assert!(!with(true, false, false).contains("p-inputmask-clear-icon"));
    assert!(!with(false, true, false).contains("p-inputmask-clear-icon"));
    assert!(!with(true, true, true).contains("p-inputmask-clear-icon"));
}

#[test]
fn state_classes_and_attributes() {
    let html = render(|| {
        view! {
            <InputMask
                mask="99-99"
                invalid=true
                variant=Variant::Filled
                size=Size::Large
                fluid=true
                readonly=true
                input_id="im"
                tabindex=2
                title="Mask"
                aria_label="Phone"
                aria_required=true
                autocomplete="off"
                input_type="tel"
            />
        }
        .to_html()
    });
    for class in ["p-invalid", "p-variant-filled", "p-inputtext-lg", "p-inputtext-fluid"] {
        assert!(html.contains(class), "{class}: {html}");
    }
    for attr in [
        "readonly",
        "id=\"im\"",
        "tabindex=\"2\"",
        "title=\"Mask\"",
        "aria-label=\"Phone\"",
        "aria-required=\"true\"",
        "autocomplete=\"off\"",
        "type=\"tel\"",
        "width: 100%",
    ] {
        assert!(html.contains(attr), "{attr}: {html}");
    }
    let html = render(|| view! { <InputMask mask="99-99" disabled=true style="width: 8rem" style_class="mine" id="root" /> }.to_html());
    assert!(html.contains("disabled") && html.contains("width: 8rem") && html.contains("mine") && html.contains("id=\"root\""), "{html}");
}

#[test]
fn pt_classes_and_attributes() {
    let pt = InputMaskPt { root: Pt::new().class("r").attr("data-x", "1"), input: Pt::new().class("in"), ..InputMaskPt::default() };
    let html = render(|| view! { <InputMask mask="99" pt=pt /> }.to_html());
    assert!(html.contains(" r") && html.contains("data-x=\"1\"") && html.contains(" in"), "{html}");
}
