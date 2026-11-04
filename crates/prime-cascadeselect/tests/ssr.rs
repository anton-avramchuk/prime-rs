//! Разметка CascadeSelect (SSR): корень, скрытое поле, метка, классы состояния, очистка, загрузка, `pt`.
//! Панель выводится только в открытом состоянии и проверяется в браузерных тестах.
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_cascadeselect::{CascadeOption, CascadeSelect, CascadeSelectPt};
use prime_core::{Pt, Size, Variant};

fn options() -> Vec<CascadeOption> {
    vec![
        CascadeOption::new("Australia")
            .children(vec![CascadeOption::new("New South Wales").children(vec![CascadeOption::new("Sydney").value("syd")])]),
        CascadeOption::new("Canada"),
    ]
}

fn render(f: impl FnOnce() -> String) -> String {
    Owner::new().with(f)
}

#[test]
fn empty_field_shows_placeholder() {
    let html = render(|| view! { <CascadeSelect options=options() placeholder="Select a City" /> }.to_html());
    assert!(html.contains("p-cascadeselect p-component p-inputwrapper"), "{html}");
    assert!(html.contains("p-cascadeselect-label p-placeholder") && html.contains("Select a City"), "{html}");
    assert!(
        html.contains("p-hidden-accessible")
            && html.contains("role=\"combobox\"")
            && html.contains("readonly")
            && html.contains("aria-haspopup=\"tree\""),
        "{html}"
    );
    assert!(html.contains("aria-expanded=\"false\"") && !html.contains("aria-controls"), "{html}");
    assert!(html.contains("p-cascadeselect-dropdown") && html.contains("p-cascadeselect-dropdown-icon") && html.contains("<svg"), "{html}");
    assert!(
        !html.contains("p-cascadeselect-overlay") && !html.contains("p-cascadeselect-open") && !html.contains("p-inputwrapper-filled"),
        "{html}"
    );
}

#[test]
fn empty_field_without_placeholder_is_hidden_label() {
    let html = render(|| view! { <CascadeSelect options=options() /> }.to_html());
    assert!(
        html.contains("p-cascadeselect-label p-cascadeselect-label-empty")
            && html.contains("p-emptylabel")
            && !html.contains("p-placeholder"),
        "{html}"
    );
}

#[test]
fn selected_leaf_shows_its_label_by_value() {
    let html = render(|| {
        let value = RwSignal::new(Some("syd".to_owned()));
        view! { <CascadeSelect options=options() value=value placeholder="Pick" /> }.to_html()
    });
    assert!(html.contains(">Sydney<") && !html.contains(">Pick<") && !html.contains("p-placeholder"), "{html}");
    assert!(html.contains("p-inputwrapper-filled"), "{html}");
    let html = render(|| {
        let value = RwSignal::new(Some("Sydney".to_owned()));
        view! { <CascadeSelect options=options() value=value placeholder="Pick" /> }.to_html()
    });
    assert!(html.contains("p-placeholder") && html.contains("Pick"), "неизвестное значение показывает подсказку: {html}");
}

#[test]
fn clear_icon_requires_value_show_clear_and_enabled() {
    let with = |show: bool, selected: bool, disabled: bool| {
        render(move || {
            let value = RwSignal::new(selected.then(|| "Canada".to_owned()));
            view! { <CascadeSelect options=options() value=value show_clear=show disabled=disabled /> }.to_html()
        })
    };
    assert!(
        with(true, true, false).contains("p-cascadeselect-clear-icon") && with(true, true, false).contains("p-cascadeselect-clearable")
    );
    assert!(!with(true, false, false).contains("p-cascadeselect-clear-icon"));
    assert!(!with(false, true, false).contains("p-cascadeselect-clear-icon"));
    assert!(!with(true, true, true).contains("p-cascadeselect-clear-icon"));
}

#[test]
fn state_classes_and_attributes() {
    let html = render(|| {
        view! {
            <CascadeSelect options=options() disabled=true invalid=true variant=Variant::Filled size=Size::Small fluid=true tabindex=3 input_id="cs" aria_label="Pick" />
        }
        .to_html()
    });
    for class in ["p-disabled", "p-invalid", "p-variant-filled", "p-cascadeselect-fluid", "p-cascadeselect-sm p-inputfield-sm"] {
        assert!(html.contains(class), "{class}: {html}");
    }
    assert!(html.contains("tabindex=\"-1\"") && html.contains("id=\"cs\"") && html.contains("aria-label=\"Pick\""), "{html}");
    let html = render(|| view! { <CascadeSelect options=options() size=Size::Large tabindex=3 /> }.to_html());
    assert!(html.contains("p-cascadeselect-lg p-inputfield-lg") && html.contains("tabindex=\"3\""), "{html}");
}

#[test]
fn loading_replaces_the_arrow_with_a_spinner() {
    let html = render(|| view! { <CascadeSelect options=options() loading=true /> }.to_html());
    assert!(html.contains("p-cascadeselect-loading-icon") && !html.contains("p-cascadeselect-dropdown-icon"), "{html}");
}

#[test]
fn style_class_and_pt() {
    let pt = CascadeSelectPt { root: Pt::new().class("r").attr("data-x", "1"), label: Pt::new().class("l"), ..CascadeSelectPt::default() };
    let html = render(|| view! { <CascadeSelect options=options() style="width: 14rem" style_class="mine" pt=pt /> }.to_html());
    assert!(html.contains("mine") && html.contains(" r") && html.contains("data-x=\"1\"") && html.contains("width: 14rem"), "{html}");
    assert!(html.contains("p-cascadeselect-label") && html.contains(" l"), "{html}");
}

#[test]
fn value_view_replaces_the_label() {
    let view_cb = Callback::new(|label: Option<String>| view! { <b class="custom">{label.unwrap_or_default()}</b> }.into_any());
    let html = render(|| {
        let value = RwSignal::new(Some("syd".to_owned()));
        view! { <CascadeSelect options=options() value=value value_view=view_cb /> }.to_html()
    });
    assert!(html.contains("class=\"custom\"") && html.contains(">Sydney<"), "{html}");
}
