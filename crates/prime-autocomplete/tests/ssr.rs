//! Разметка AutoComplete (SSR): корень, одиночное поле, множественный режим с чипами, очистка, кнопка, состояния, `pt`.
//! Панель выводится только в открытом состоянии и проверяется в браузерных тестах.
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_autocomplete::{AutoComplete, AutoCompleteOption, AutoCompletePt};
use prime_core::{Pt, Size, Variant};

fn options() -> Vec<AutoCompleteOption> {
    vec![AutoCompleteOption::new("Apple"), AutoCompleteOption::new("Banana").value("b")]
}

fn render(f: impl FnOnce() -> String) -> String {
    Owner::new().with(f)
}

#[test]
fn single_input_markup() {
    let html = render(|| view! { <AutoComplete suggestions=options() placeholder="Search" /> }.to_html());
    assert!(html.contains("p-autocomplete p-component p-inputwrapper"), "{html}");
    assert!(
        html.contains("p-inputtext p-component p-autocomplete-input")
            && html.contains("role=\"combobox\"")
            && html.contains("aria-autocomplete=\"list\""),
        "{html}"
    );
    assert!(
        html.contains("placeholder=\"Search\"")
            && html.contains("type=\"text\"")
            && html.contains("aria-expanded=\"false\"")
            && html.contains("autocomplete=\"off\""),
        "{html}"
    );
    assert!(!html.contains("aria-controls") && !html.contains("p-autocomplete-overlay") && !html.contains("p-autocomplete-open"), "{html}");
    assert!(!html.contains("p-autocomplete-dropdown") && !html.contains("p-inputwrapper-filled") && !html.contains("p-filled"), "{html}");
    assert!(html.contains("position: relative"), "{html}");
}

#[test]
fn value_shows_the_option_label() {
    let html = render(|| {
        let value = RwSignal::new("b".to_owned());
        view! { <AutoComplete suggestions=options() value=value /> }.to_html()
    });
    assert!(html.contains("value=\"Banana\""), "{html}");
    assert!(html.contains("p-inputwrapper-filled") && html.contains("p-filled"), "{html}");
    let html = render(|| {
        let value = RwSignal::new("typed".to_owned());
        view! { <AutoComplete suggestions=options() value=value /> }.to_html()
    });
    assert!(html.contains("value=\"typed\""), "несовпавшее значение показывается как есть: {html}");
}

#[test]
fn multiple_markup_has_chips_and_an_input() {
    let html = render(|| {
        let values = RwSignal::new(vec!["Apple".to_owned(), "b".to_owned()]);
        view! { <AutoComplete suggestions=options() values=values multiple=true placeholder="Add" /> }.to_html()
    });
    assert!(
        html.contains("p-autocomplete-input-multiple")
            && html.contains("role=\"listbox\"")
            && html.contains("aria-orientation=\"horizontal\""),
        "{html}"
    );
    assert_eq!(html.matches("p-autocomplete-chip-item").count(), 2, "{html}");
    assert!(html.contains("p-autocomplete-chip") && html.contains("Apple") && html.contains("Banana"), "{html}");
    assert!(html.contains("p-autocomplete-input-chip") && html.contains("p-inputwrapper-filled"), "{html}");
    assert!(!html.contains("placeholder=\"Add\""), "при выбранных чипах подсказка скрыта: {html}");
    assert!(
        html.contains("aria-setsize=\"2\"")
            && html.contains("aria-posinset=\"2\"")
            && html.contains("id=\"")
            && html.contains("_multiple_option_1"),
        "{html}"
    );
    let html = render(|| view! { <AutoComplete suggestions=options() multiple=true placeholder="Add" /> }.to_html());
    assert!(html.contains("placeholder=\"Add\"") && html.matches("p-autocomplete-chip-item").count() == 0, "{html}");
}

#[test]
fn dropdown_clear_and_state_classes() {
    let html = render(|| view! { <AutoComplete suggestions=options() dropdown=true /> }.to_html());
    assert!(html.contains("<button") && html.contains("p-autocomplete-dropdown") && html.contains("<svg"), "{html}");
    let with = |show: bool, selected: bool, disabled: bool| {
        render(move || {
            let value = RwSignal::new(if selected { "Apple".to_owned() } else { String::new() });
            view! { <AutoComplete suggestions=options() value=value show_clear=show disabled=disabled /> }.to_html()
        })
    };
    assert!(with(true, true, false).contains("p-autocomplete-clear-icon") && with(true, true, false).contains("p-autocomplete-clearable"));
    assert!(!with(true, false, false).contains("p-autocomplete-clear-icon"));
    assert!(!with(false, true, false).contains("p-autocomplete-clear-icon"));
    assert!(!with(true, true, true).contains("p-autocomplete-clear-icon"));
    let html = render(|| {
        view! { <AutoComplete suggestions=options() disabled=true invalid=true variant=Variant::Filled size=Size::Small fluid=true tabindex=4 input_id="ac" /> }.to_html()
    });
    for class in ["p-invalid", "p-variant-filled", "p-inputtext-sm", "p-autocomplete-fluid", "p-inputtext-fluid"] {
        assert!(html.contains(class), "{class}: {html}");
    }
    assert!(html.contains("disabled") && html.contains("tabindex=\"-1\"") && html.contains("id=\"ac\""), "{html}");
    let html = render(|| view! { <AutoComplete suggestions=options() tabindex=4 readonly=true input_type="search" /> }.to_html());
    assert!(html.contains("tabindex=\"4\"") && html.contains("readonly") && html.contains("type=\"search\""), "{html}");
}

#[test]
fn style_class_and_pt() {
    let pt = AutoCompletePt { root: Pt::new().class("r").attr("data-x", "1"), input: Pt::new().class("in"), ..AutoCompletePt::default() };
    let html = render(|| {
        view! { <AutoComplete suggestions=options() style="width: 12rem" style_class="mine" input_style_class="mi" pt=pt /> }.to_html()
    });
    assert!(html.contains("mine") && html.contains(" r") && html.contains("data-x=\"1\"") && html.contains("width: 12rem"), "{html}");
    assert!(html.contains("p-autocomplete-input mi") && html.contains(" in"), "{html}");
}
