//! Разметка Listbox (SSR): список, группы, выбор, флажки, отметка, поиск, пустые сообщения, свойства.
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_core::Pt;
use prime_listbox::{Listbox, ListboxGroup, ListboxOption, ListboxPt};

fn render(f: impl FnOnce() -> String) -> String {
    Owner::new().with(f)
}

fn cities() -> Vec<ListboxOption> {
    vec![ListboxOption::new("New York", "NY"), ListboxOption::new("Rome", "RM").disabled(true), ListboxOption::new("London", "LDN")]
}

fn groups() -> Vec<ListboxGroup> {
    vec![
        ListboxGroup::new("Germany", vec![ListboxOption::new("Berlin", "B"), ListboxOption::new("Frankfurt", "F")]),
        ListboxGroup::new("Japan", vec![ListboxOption::new("Kyoto", "K"), ListboxOption::new("Tokyo", "T")]),
    ]
}

#[test]
fn flat_list_markup() {
    let html = render(|| view! { <Listbox id="lb" options=cities() /> }.to_html());
    assert!(html.contains("p-listbox p-component"), "{html}");
    assert!(html.contains("id=\"lb\"") && html.contains("id=\"lb_list\""), "{html}");
    assert!(html.contains("p-listbox-list-container") && html.contains("max-height: 14rem"), "{html}");
    assert!(html.contains("role=\"listbox\"") && html.contains("tabindex=\"0\"") && html.contains("aria-disabled=\"false\""), "{html}");
    assert_eq!(html.matches("role=\"option\"").count(), 3, "{html}");
    assert!(html.contains("id=\"lb_0\"") && html.contains("id=\"lb_2\""), "{html}");
    assert!(
        html.contains("aria-label=\"London\"") && html.contains("aria-setsize=\"3\"") && html.contains("aria-posinset=\"3\""),
        "{html}"
    );
    assert!(html.contains("aria-selected=\"false\"") && !html.contains("aria-selected=\"true\""), "{html}");
    assert!(!html.contains("aria-multiselectable") && !html.contains("p-listbox-header") && !html.contains("p-checkbox"), "{html}");
    assert!(!html.contains("p-listbox-option-selected") && !html.contains("aria-activedescendant"), "{html}");
}

#[test]
fn disabled_option_is_marked() {
    let html = render(|| view! { <Listbox options=cities() /> }.to_html());
    assert!(html.contains("p-listbox-option p-disabled") && html.contains("data-p-disabled=\"true\""), "{html}");
    assert!(html.contains("aria-disabled=\"true\""), "{html}");
}

#[test]
fn single_value_is_selected() {
    let value = RwSignal::new(Some("LDN".to_owned()));
    let html = render(|| view! { <Listbox options=cities() value=value /> }.to_html());
    assert_eq!(html.matches("p-listbox-option-selected").count(), 1, "{html}");
    assert!(html.contains("aria-selected=\"true\"") && html.contains("data-p-selected=\"true\""), "{html}");
    let off = render(|| view! { <Listbox options=cities() value=value highlight_on_select=false /> }.to_html());
    assert!(!off.contains("p-listbox-option-selected") && off.contains("aria-selected=\"true\""), "{off}");
}

#[test]
fn multiple_values_are_selected() {
    let values = RwSignal::new(vec!["NY".to_owned(), "LDN".to_owned()]);
    let html = render(|| view! { <Listbox options=cities() multiple=true values=values /> }.to_html());
    assert!(html.contains("aria-multiselectable=\"true\""), "{html}");
    assert_eq!(html.matches("p-listbox-option-selected").count(), 2, "{html}");
}

#[test]
fn unknown_value_selects_nothing() {
    let value = RwSignal::new(Some("zzz".to_owned()));
    let html = render(|| view! { <Listbox options=cities() value=value /> }.to_html());
    assert!(!html.contains("p-listbox-option-selected"), "{html}");
}

#[test]
fn groups_markup_and_aria_ignore_headers() {
    let html = render(|| view! { <Listbox id="g" groups=groups() /> }.to_html());
    assert_eq!(html.matches("p-listbox-option-group").count(), 2, "{html}");
    assert!(html.contains("id=\"g_0\"") && html.contains("id=\"g_3\"") && html.contains("Germany") && html.contains("Japan"), "{html}");
    assert!(html.contains("aria-setsize=\"4\"") && html.contains("aria-posinset=\"4\""), "{html}");
}

#[test]
fn empty_messages() {
    let html = render(|| view! { <Listbox /> }.to_html());
    assert!(html.contains("p-listbox-empty-message") && !html.contains("<ul"), "{html}");
    let custom = render(|| view! { <Listbox empty_message="Nothing here" /> }.to_html());
    assert!(custom.contains("Nothing here"), "{custom}");
}

#[test]
fn filter_header_markup() {
    let html =
        render(|| view! { <Listbox id="f" options=cities() filter=true filter_placeholder="Search" aria_filter_label="Find" /> }.to_html());
    assert!(html.contains("p-listbox-header") && html.contains("p-iconfield") && html.contains("p-inputicon"), "{html}");
    assert!(html.contains("p-inputtext p-component p-listbox-filter") && html.contains("role=\"searchbox\""), "{html}");
    assert!(
        html.contains("placeholder=\"Search\"") && html.contains("aria-label=\"Find\"") && html.contains("aria-owns=\"f_list\""),
        "{html}"
    );
    assert!(!html.contains("p-checkbox"), "{html}");
}

#[test]
fn checkbox_mode_has_toggle_all_and_option_checkboxes() {
    let values = RwSignal::new(vec!["NY".to_owned()]);
    let html = render(|| view! { <Listbox options=cities() multiple=true checkbox=true values=values /> }.to_html());
    assert!(html.contains("p-listbox-header"), "{html}");
    assert_eq!(html.matches("p-checkbox p-component").count(), 1 + 3, "{html}");
    assert_eq!(html.matches("p-checkbox-checked p-highlight").count(), 1, "только выбранный вариант: {html}");
    assert!(html.contains("p-listbox-option-check-icon") && html.contains("p-checkbox-icon"), "{html}");
    let no_toggle = render(|| view! { <Listbox options=cities() multiple=true checkbox=true show_toggle_all=false /> }.to_html());
    assert_eq!(no_toggle.matches("p-checkbox p-component").count(), 3, "{no_toggle}");
    assert!(!no_toggle.contains("p-listbox-header"), "{no_toggle}");
}

#[test]
fn toggle_all_is_checked_when_everything_is_selected() {
    let values = RwSignal::new(vec!["NY".to_owned(), "LDN".to_owned()]);
    let html = render(|| view! { <Listbox options=cities() multiple=true checkbox=true values=values /> }.to_html());
    assert_eq!(html.matches("p-checkbox-checked p-highlight").count(), 3, "отключённый Rome не мешает: {html}");
}

#[test]
fn checkbox_without_multiple_is_ignored() {
    let html = render(|| view! { <Listbox options=cities() checkbox=true /> }.to_html());
    assert!(!html.contains("p-checkbox") && !html.contains("p-listbox-header"), "{html}");
}

#[test]
fn checkmark_shows_check_or_blank_icon() {
    let value = RwSignal::new(Some("NY".to_owned()));
    let html = render(|| view! { <Listbox options=cities() checkmark=true highlight_on_select=false value=value /> }.to_html());
    assert_eq!(html.matches("p-listbox-option-check-icon").count(), 1, "{html}");
    assert_eq!(html.matches("p-listbox-option-blank-icon").count(), 2, "{html}");
}

#[test]
fn root_flags_classes_and_data_p() {
    let html = render(|| view! { <Listbox options=cities() striped=true disabled=true invalid=true fluid=true /> }.to_html());
    assert!(html.contains("p-listbox p-component p-listbox-striped p-disabled p-invalid p-listbox-fluid"), "{html}");
    assert!(html.contains("data-p=\"invalid disabled\"") && html.contains("tabindex=\"-1\""), "{html}");
}

#[test]
fn style_scroll_height_and_list_props() {
    let html = render(|| {
        view! {
            <Listbox
                options=cities()
                style="width: 10rem"
                style_class="mine"
                scroll_height="5rem"
                list_style="color: red"
                list_style_class="inner"
                aria_label="Cities"
                tabindex=3
            />
        }
        .to_html()
    });
    assert!(html.contains("width: 10rem") && html.contains("p-listbox p-component mine"), "{html}");
    assert!(html.contains("max-height: 5rem") && html.contains("p-listbox-list-container inner"), "{html}");
    assert!(html.contains("color: red") && html.contains("aria-label=\"Cities\"") && html.contains("tabindex=\"3\""), "{html}");
}

#[test]
fn pt_applies_to_parts() {
    let pt = ListboxPt {
        root: Pt::new().class("r").attr("data-x", "1"),
        list: Pt::new().class("l"),
        option: Pt::new().class("o"),
        ..ListboxPt::default()
    };
    let html = render(|| view! { <Listbox options=cities() pt=pt /> }.to_html());
    assert!(html.contains("data-x=\"1\"") && html.contains("p-listbox p-component r"), "{html}");
    assert!(html.contains("p-listbox-list l") && html.contains("p-listbox-option o"), "{html}");
}

#[test]
fn option_and_group_templates() {
    let html = render(|| {
        view! {
            <Listbox
                groups=groups()
                option=Callback::new(|(o, selected): (ListboxOption, bool)| view! { <b class="t">{o.label}{selected.to_string()}</b> }.into_any())
                group=Callback::new(|g: ListboxGroup| view! { <i class="g">{g.label}</i> }.into_any())
            />
        }
        .to_html()
    });
    assert_eq!(html.matches("class=\"t\"").count(), 4, "{html}");
    assert_eq!(html.matches("class=\"g\"").count(), 2, "{html}");
    assert!(html.contains("Berlin") && html.contains("false"), "{html}");
}
