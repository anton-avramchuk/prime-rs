//! Разметка MultiSelect (SSR): корень, скрытое поле, метка в режимах `comma` и `chip`, значки, классы по состояниям, `pt`.
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_core::{Pt, Size, Variant};
use prime_multiselect::{MultiSelect, MultiSelectDisplay, MultiSelectGroup, MultiSelectOption, MultiSelectPt};

fn render(f: impl FnOnce() -> String) -> String {
    Owner::new().with(f)
}

fn cities() -> Vec<MultiSelectOption> {
    vec![
        MultiSelectOption::new("New York", "NY"),
        MultiSelectOption::new("Rome", "RM"),
        MultiSelectOption::new("London", "LDN"),
        MultiSelectOption::new("Istanbul", "IST"),
        MultiSelectOption::new("Paris", "PRS"),
    ]
}

fn picked(values: &[&str]) -> RwSignal<Vec<String>> {
    RwSignal::new(values.iter().map(|v| (*v).to_owned()).collect())
}

#[test]
fn closed_markup_has_root_hidden_input_label_and_dropdown() {
    let html = render(|| view! { <MultiSelect id="ms" options=cities() placeholder="Select Cities" /> }.to_html());
    assert!(html.contains("p-multiselect p-component p-inputwrapper"), "{html}");
    assert!(html.contains("id=\"ms\"") && html.contains("data-p-hidden-accessible"), "{html}");
    assert!(html.contains("role=\"combobox\"") && html.contains("aria-haspopup=\"listbox\"") && html.contains("readonly"), "{html}");
    assert!(
        html.contains("aria-expanded=\"false\"") && html.contains("tabindex=\"0\"") && html.contains("aria-disabled=\"false\""),
        "{html}"
    );
    assert!(html.contains("p-multiselect-label-container") && html.contains("p-multiselect-label p-placeholder"), "{html}");
    assert!(
        html.contains("Select Cities") && html.contains("p-multiselect-dropdown") && html.contains("p-multiselect-dropdown-icon"),
        "{html}"
    );
    assert!(!html.contains("p-multiselect-overlay") && !html.contains("aria-controls") && !html.contains("clear-icon"), "{html}");
}

#[test]
fn comma_label_lists_selected_labels_in_selection_order() {
    let values = picked(&["LDN", "NY"]);
    let html = render(|| view! { <MultiSelect options=cities() values=values placeholder="Select Cities" /> }.to_html());
    assert!(html.contains("London, New York"), "{html}");
    assert!(!html.contains("p-placeholder") && !html.contains("Select Cities"), "{html}");
    assert!(html.contains("p-inputwrapper-filled"), "{html}");
}

#[test]
fn comma_label_switches_to_summary_over_the_limit() {
    let values = picked(&["NY", "RM", "LDN", "IST"]);
    let html = render(|| view! { <MultiSelect options=cities() values=values /> }.to_html());
    assert!(html.contains("4 items selected"), "{html}");
    let wide = render(|| view! { <MultiSelect options=cities() values=values max_selected_labels=4 /> }.to_html());
    assert!(wide.contains("New York, Rome, London, Istanbul"), "{wide}");
    let custom = render(|| {
        view! { <MultiSelect options=cities() values=values max_selected_labels=2 selected_items_label="{0} cities" /> }.to_html()
    });
    assert!(custom.contains("4 cities"), "{custom}");
}

#[test]
fn display_selected_label_off_shows_the_placeholder() {
    let values = picked(&["NY"]);
    let html =
        render(|| view! { <MultiSelect options=cities() values=values placeholder="Pick" display_selected_label=false /> }.to_html());
    assert!(html.contains("Pick") && html.contains("p-placeholder") && !html.contains("New York"), "{html}");
}

#[test]
fn empty_label_without_placeholder_is_marked_empty() {
    let html = render(|| view! { <MultiSelect options=cities() /> }.to_html());
    assert!(html.contains("p-multiselect-label p-multiselect-label-empty") && html.contains("empty"), "{html}");
    assert!(html.contains("data-p=\"empty\""), "{html}");
}

#[test]
fn unknown_values_are_ignored_by_the_label() {
    let values = picked(&["zzz", "PRS"]);
    let html = render(|| view! { <MultiSelect options=cities() values=values /> }.to_html());
    assert!(html.contains("Paris") && !html.contains("zzz"), "{html}");
}

#[test]
fn chip_mode_renders_one_chip_per_value() {
    let values = picked(&["NY", "LDN"]);
    let html = render(|| view! { <MultiSelect options=cities() values=values display=MultiSelectDisplay::Chip /> }.to_html());
    assert!(html.contains("p-multiselect-display-chip"), "{html}");
    assert_eq!(html.matches("p-multiselect-chip-item").count(), 2, "{html}");
    assert_eq!(html.matches("p-chip-remove-icon").count(), 2, "{html}");
    assert!(html.contains("p-multiselect-chip") && html.contains("New York") && html.contains("London"), "{html}");
    assert!(html.contains("has-chip"), "{html}");
}

#[test]
fn chip_mode_without_removal_when_readonly_or_disabled() {
    let values = picked(&["NY"]);
    let ro = render(|| view! { <MultiSelect options=cities() values=values display=MultiSelectDisplay::Chip readonly=true /> }.to_html());
    assert!(ro.contains("p-multiselect-chip-item") && !ro.contains("p-chip-remove-icon"), "{ro}");
    let dis = render(|| view! { <MultiSelect options=cities() values=values display=MultiSelectDisplay::Chip disabled=true /> }.to_html());
    assert!(!dis.contains("p-chip-remove-icon"), "{dis}");
}

#[test]
fn chip_mode_switches_to_summary_at_the_limit() {
    let values = picked(&["NY", "RM", "LDN"]);
    let html = render(|| view! { <MultiSelect options=cities() values=values display=MultiSelectDisplay::Chip /> }.to_html());
    assert!(html.contains("3 items selected") && !html.contains("p-multiselect-chip-item"), "{html}");
    let wide = render(|| {
        view! { <MultiSelect options=cities() values=values display=MultiSelectDisplay::Chip max_selected_labels=5 /> }.to_html()
    });
    assert_eq!(wide.matches("p-multiselect-chip-item").count(), 3, "{wide}");
}

#[test]
fn clear_icon_only_with_selection_and_enabled() {
    let values = picked(&["NY"]);
    let shown = render(|| view! { <MultiSelect options=cities() values=values show_clear=true /> }.to_html());
    assert!(shown.contains("p-multiselect-clear-icon") && shown.contains("clearable"), "{shown}");
    let empty = render(|| view! { <MultiSelect options=cities() show_clear=true /> }.to_html());
    assert!(!empty.contains("p-multiselect-clear-icon"), "{empty}");
    let disabled = render(|| view! { <MultiSelect options=cities() values=values show_clear=true disabled=true /> }.to_html());
    assert!(!disabled.contains("p-multiselect-clear-icon"), "{disabled}");
}

#[test]
fn loading_shows_the_spinner_instead_of_the_arrow() {
    let html = render(|| view! { <MultiSelect options=cities() loading=true /> }.to_html());
    assert!(html.contains("p-multiselect-loading-icon") && !html.contains("p-multiselect-dropdown-icon"), "{html}");
}

#[test]
fn root_classes_follow_the_state() {
    let values = picked(&["NY"]);
    let html = render(|| {
        view! {
            <MultiSelect options=cities() values=values disabled=true invalid=true variant=Variant::Filled fluid=true size=Size::Small />
        }
        .to_html()
    });
    assert!(
        html.contains("p-multiselect p-component p-inputwrapper p-disabled p-invalid p-variant-filled p-inputwrapper-filled p-multiselect-fluid p-multiselect-sm p-inputfield-sm"),
        "{html}"
    );
    assert!(html.contains("data-p=\"invalid disabled fluid filled small\"") && html.contains("tabindex=\"-1\""), "{html}");
}

#[test]
fn open_panel_is_rendered_from_the_open_signal() {
    let open = RwSignal::new(true);
    let values = picked(&["LDN"]);
    let html = render(|| view! { <MultiSelect id="o" options=cities() values=values open=open filter_placeholder="Search" /> }.to_html());
    assert!(
        html.contains("p-multiselect-open") && html.contains("aria-expanded=\"true\"") && html.contains("aria-controls=\"o_list\""),
        "{html}"
    );
    assert!(html.contains("p-multiselect-overlay") && html.contains("id=\"o_list\"") && html.contains("data-p=\"overlay-self\""), "{html}");
    assert!(
        html.contains("p-multiselect-header") && html.contains("p-multiselect-filter-container") && html.contains("role=\"searchbox\""),
        "{html}"
    );
    assert!(html.contains("placeholder=\"Search\"") && html.contains("p-inputicon"), "{html}");
    assert!(html.contains("p-multiselect-list-container") && html.contains("max-height: 200px"), "{html}");
    assert!(html.contains("role=\"listbox\"") && html.contains("aria-multiselectable=\"true\""), "{html}");
    assert_eq!(html.matches("role=\"option\"").count(), 5, "{html}");
    assert_eq!(html.matches("class=\"p-hidden-accessible p-hidden-focusable\"").count(), 2, "две скрытые границы: {html}");
    assert!(html.contains("id=\"o_0\"") && html.contains("id=\"o_4\""), "{html}");
}

#[test]
fn options_carry_checkbox_and_selection_markup() {
    let open = RwSignal::new(true);
    let values = picked(&["LDN"]);
    let html = render(|| view! { <MultiSelect options=cities() values=values open=open /> }.to_html());
    assert_eq!(html.matches("p-multiselect-option-selected").count(), 1, "{html}");
    assert!(
        html.contains("aria-selected=\"true\"") && html.contains("aria-checked=\"true\"") && html.contains("data-p-selected=\"true\""),
        "{html}"
    );
    assert!(html.contains("aria-setsize=\"5\"") && html.contains("aria-posinset=\"3\""), "{html}");
    // флажок у каждого из пяти вариантов и один в шапке
    assert_eq!(html.matches("p-checkbox p-component").count(), 6, "{html}");
    assert_eq!(html.matches("p-checkbox-checked p-highlight").count(), 1, "выбран только London; шапка не отмечена: {html}");
    let off = render(|| view! { <MultiSelect options=cities() values=values open=open highlight_on_select=false /> }.to_html());
    assert!(!off.contains("p-multiselect-option-selected"), "{off}");
}

#[test]
fn selection_limit_disables_unselected_options_and_hides_toggle_all() {
    let open = RwSignal::new(true);
    let values = picked(&["NY", "LDN"]);
    let html = render(|| view! { <MultiSelect options=cities() values=values open=open selection_limit=2 /> }.to_html());
    assert_eq!(html.matches("p-multiselect-option p-disabled").count(), 3, "{html}");
    assert_eq!(html.matches("p-checkbox p-component").count(), 5, "флажка «выбрать все» нет: {html}");
    let below = render(|| view! { <MultiSelect options=cities() values=values open=open selection_limit=3 /> }.to_html());
    assert!(!below.contains("p-multiselect-option p-disabled"), "{below}");
}

#[test]
fn header_options_toggle_all_filter_and_header_flags() {
    let open = RwSignal::new(true);
    let no_toggle = render(|| view! { <MultiSelect options=cities() open=open show_toggle_all=false /> }.to_html());
    assert_eq!(no_toggle.matches("p-checkbox p-component").count(), 5, "{no_toggle}");
    let no_filter = render(|| view! { <MultiSelect options=cities() open=open filter=false /> }.to_html());
    assert!(no_filter.contains("p-multiselect-header") && !no_filter.contains("p-multiselect-filter"), "{no_filter}");
    let no_header = render(|| view! { <MultiSelect options=cities() open=open show_header=false /> }.to_html());
    assert!(!no_header.contains("p-multiselect-header"), "{no_header}");
}

#[test]
fn toggle_all_is_checked_when_every_option_is_selected() {
    let open = RwSignal::new(true);
    let values = picked(&["NY", "RM", "LDN", "IST", "PRS"]);
    let html = render(|| view! { <MultiSelect options=cities() values=values open=open /> }.to_html());
    assert_eq!(html.matches("p-checkbox-checked p-highlight").count(), 6, "{html}");
}

#[test]
fn groups_and_empty_messages() {
    let open = RwSignal::new(true);
    let groups = vec![
        MultiSelectGroup::new("Germany", vec![MultiSelectOption::new("Berlin", "B")]),
        MultiSelectGroup::new("Japan", vec![MultiSelectOption::new("Kyoto", "K"), MultiSelectOption::new("Tokyo", "T").disabled(true)]),
    ];
    let html = render(|| view! { <MultiSelect id="g" groups=groups open=open /> }.to_html());
    assert_eq!(html.matches("p-multiselect-option-group").count(), 2, "{html}");
    assert!(html.contains("id=\"g_0\"") && html.contains("id=\"g_4\"") && html.contains("Germany") && html.contains("Japan"), "{html}");
    assert!(html.contains("aria-setsize=\"3\"") && html.contains("aria-posinset=\"3\""), "{html}");
    let empty = render(|| view! { <MultiSelect open=open empty_message="Nothing" /> }.to_html());
    assert!(empty.contains("p-multiselect-empty-message") && empty.contains("Nothing"), "{empty}");
}

#[test]
fn props_style_and_pt() {
    let open = RwSignal::new(true);
    let pt = MultiSelectPt {
        root: Pt::new().class("r").attr("data-x", "1"),
        label: Pt::new().class("l"),
        option: Pt::new().class("o"),
        overlay: Pt::new().class("ov"),
        ..MultiSelectPt::default()
    };
    let html = render(|| {
        view! {
            <MultiSelect
                options=cities()
                open=open
                style="width: 12rem"
                style_class="mine"
                panel_style="color: red"
                panel_style_class="panel"
                scroll_height="5rem"
                aria_label="Cities"
                input_id="field"
                tabindex=3
                pt=pt
            />
        }
        .to_html()
    });
    assert!(html.contains("width: 12rem") && html.contains("p-multiselect-open mine r") && html.contains("data-x=\"1\""), "{html}");
    assert!(html.contains("p-multiselect-label-empty l") && html.contains("p-multiselect-option o"), "{html}");
    assert!(html.contains("p-multiselect-overlay p-component-overlay p-component panel ov") && html.contains("color: red"), "{html}");
    assert!(
        html.contains("max-height: 5rem")
            && html.contains("aria-label=\"Cities\"")
            && html.contains("id=\"field\"")
            && html.contains("tabindex=\"3\""),
        "{html}"
    );
}

#[test]
fn templates_replace_option_group_and_label_content() {
    let open = RwSignal::new(true);
    let values = picked(&["B"]);
    let groups = vec![MultiSelectGroup::new("Germany", vec![MultiSelectOption::new("Berlin", "B"), MultiSelectOption::new("Munich", "M")])];
    let html = render(|| {
        view! {
            <MultiSelect
                groups=groups
                values=values
                open=open
                item=Callback::new(|(o, selected): (MultiSelectOption, bool)| view! { <b class="t">{o.value}{selected.to_string()}</b> }.into_any())
                group=Callback::new(|g: MultiSelectGroup| view! { <i class="g">{g.label}</i> }.into_any())
                selected_items=Callback::new(|v: Vec<MultiSelectOption>| view! { <u class="s">{v.len()}</u> }.into_any())
            />
        }
        .to_html()
    });
    assert_eq!(html.matches("class=\"t\"").count(), 2, "{html}");
    assert!(html.contains("class=\"g\"") && html.contains("class=\"s\""), "{html}");
}
