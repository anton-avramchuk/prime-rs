//! Закрытая и открытая (через `open`) разметка Select: метка, подсказка, очистка, загрузка, варианты, группы, поиск, `aria-*`, `pt` (SSR).
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_core::{Pt, Size, Variant};
use prime_select::{Select, SelectGroup, SelectOption, SelectPt};

fn cities() -> Vec<SelectOption> {
    vec![
        SelectOption::new("New York", "NY"),
        SelectOption::new("Rome", "RM").disabled(true),
        SelectOption::new("London", "LDN"),
        SelectOption::new("Istanbul", "IST"),
        SelectOption::new("Paris", "PRS"),
    ]
}

fn groups() -> Vec<SelectGroup> {
    vec![
        SelectGroup::new("Germany", vec![SelectOption::new("Berlin", "B"), SelectOption::new("Frankfurt", "F")]),
        SelectGroup::new("USA", vec![SelectOption::new("Chicago", "C"), SelectOption::new("Los Angeles", "LA")]),
    ]
}

fn count(html: &str, needle: &str) -> usize {
    html.matches(needle).count()
}

fn open() -> RwSignal<bool> {
    RwSignal::new(true)
}

#[test]
fn closed_structure_and_placeholder() {
    let html = view! { <Select options=cities() placeholder="Select a City" /> }.to_html();
    assert!(html.starts_with("<div"), "{html}");
    for expected in [
        r#"class="p-select p-component p-inputwrapper""#,
        r#"role="combobox""#,
        r#"aria-haspopup="listbox""#,
        r#"aria-expanded="false""#,
        r#"aria-disabled="false""#,
        r#"tabindex="0""#,
        r#"class="p-select-label p-placeholder""#,
        "Select a City",
        r#"class="p-select-dropdown""#,
        r#"role="button""#,
        r#"aria-label="dropdown trigger""#,
        r#"data-pc-section="trigger""#,
        "p-select-dropdown-icon",
        r#"data-p="placeholder""#,
    ] {
        assert!(html.contains(expected), "{expected}: {html}");
    }
    for unwanted in ["p-select-overlay", "p-select-open", "aria-controls", "p-select-clear-icon", "p-disabled", "p-invalid", "<li"] {
        assert!(!html.contains(unwanted), "{unwanted}: {html}");
    }
}

#[test]
fn selected_value_shows_its_label_and_unknown_value_falls_back() {
    let html = view! { <Select options=cities() value=RwSignal::new(Some("LDN".to_owned())) placeholder="Select a City" /> }.to_html();
    assert!(html.contains(">London<") && !html.contains("p-placeholder") && html.contains("p-inputwrapper-filled"), "{html}");
    assert!(html.contains(r#"aria-label="London""#), "{html}");
    let html = view! { <Select options=cities() default_value="nope" placeholder="Select a City" /> }.to_html();
    assert!(html.contains("p-placeholder") && html.contains("Select a City") && !html.contains("p-inputwrapper-filled"), "{html}");
    // значение из группы
    let html = view! { <Select groups=groups() default_value="C" /> }.to_html();
    assert!(html.contains(">Chicago<"), "{html}");
}

#[test]
fn empty_label_without_placeholder() {
    let html = view! { <Select options=cities() /> }.to_html();
    assert!(
        html.contains("p-select-label p-select-label-empty") && html.contains("\u{a0}") && html.contains(r#"data-p="empty""#),
        "{html}"
    );
    assert!(!html.contains(r#"aria-label="#.trim_end_matches('"')) || !html.contains(r#"aria-label="""#), "{html}");
}

#[test]
fn selected_item_template_replaces_the_label() {
    let tpl = Callback::new(|o: SelectOption| view! { <b class="tpl">{format!("sel:{}", o.label)}</b> }.into_any());
    let html = view! { <Select options=cities() default_value="PRS" selected_item_template=tpl /> }.to_html();
    assert!(html.contains("sel:Paris") && html.contains(r#"class="tpl""#), "{html}");
}

#[test]
fn clear_icon_and_loading_icon() {
    let html = view! { <Select options=cities() default_value="NY" show_clear=true /> }.to_html();
    assert_eq!(count(&html, "p-select-clear-icon"), 1, "{html}");
    assert!(html.contains(r#"data-pc-section="clearicon""#) && html.contains(r#"data-p="clearable""#), "{html}");
    assert!(!view! { <Select options=cities() show_clear=true /> }.to_html().contains("clear-icon"));
    assert!(!view! { <Select options=cities() default_value="NY" show_clear=true disabled=true /> }.to_html().contains("clear-icon"));
    let html = view! { <Select options=cities() loading=true /> }.to_html();
    assert!(html.contains("p-select-loading-icon") && !html.contains("p-select-dropdown-icon") && html.contains("p-icon-spin"), "{html}");
    let icon = ViewFn::from(|| view! { <i class="my-clear" /> });
    let html = view! { <Select options=cities() default_value="NY" show_clear=true clear_icon=icon /> }.to_html();
    assert!(html.contains("my-clear") && html.contains("p-select-clear-icon"), "{html}");
}

#[test]
fn root_modifiers_and_data_p() {
    let html =
        view! { <Select options=cities() disabled=true invalid=true fluid=true size=Size::Small variant=Variant::Filled /> }.to_html();
    for expected in [
        "p-select p-component p-inputwrapper p-disabled p-variant-filled p-invalid p-select-fluid p-select-sm p-inputfield-sm",
        r#"data-p="invalid disabled fluid filled small""#,
        r#"tabindex="-1""#,
        r#"aria-disabled="true""#,
        "disabled",
    ] {
        assert!(html.contains(expected), "{expected}: {html}");
    }
    assert!(!html.contains("p-select-lg p-inputfield-lg"));
    let html = view! { <Select options=cities() size=Size::Large /> }.to_html();
    assert!(html.contains("p-select-lg p-inputfield-lg"), "{html}");
}

#[test]
fn attributes_and_ids() {
    let html = view! {
        <Select options=cities() input_id="city" aria_label="City" aria_labelledby="lbl" required=true tabindex=3 style="width: 14rem" style_class="mine" />
    }
    .to_html();
    for expected in [
        r#"id="city""#,
        r#"aria-label="City""#,
        r#"aria-labelledby="lbl""#,
        r#"aria-required="true""#,
        "required",
        r#"tabindex="3""#,
        r#"style="width: 14rem;""#,
        "p-inputwrapper mine",
    ] {
        assert!(html.contains(expected), "{expected}: {html}");
    }
    assert!(html.contains(r#"id="pn_id_"#), "корень должен иметь id: {html}");
}

#[test]
fn open_panel_lists_options_with_aria() {
    let html = view! { <Select options=cities() value=RwSignal::new(Some("LDN".to_owned())) open=open() /> }.to_html();
    for expected in [
        "p-inputwrapper-focus p-select-open",
        r#"aria-expanded="true""#,
        "p-select-overlay p-component-overlay p-component",
        r#"data-p="overlay-self""#,
        "p-select-list-container",
        "max-height: 200px",
        r#"role="listbox""#,
        r#"aria-label="Option List""#,
        r#"aria-controls=""#,
    ] {
        assert!(html.contains(expected), "{expected}: {html}");
    }
    assert_eq!(count(&html, r#"class="p-hidden-accessible p-hidden-focusable""#), 2, "{html}");
    assert_eq!(count(&html, "<li"), 5, "{html}");
    for label in ["New York", "Rome", "London", "Istanbul", "Paris"] {
        assert!(html.contains(&format!(r#"aria-label="{label}""#)), "{label}: {html}");
    }
    // выбранный вариант
    assert_eq!(count(&html, r#"aria-selected="true""#), 1, "{html}");
    assert!(html.contains("p-select-option p-select-option-selected"), "{html}");
    assert_eq!(count(&html, r#"data-p-disabled="true""#), 1, "отключённый вариант: {html}");
    assert!(html.contains("p-select-option p-disabled"), "{html}");
    assert!(html.contains(r#"aria-setsize="5""#) && html.contains(r#"aria-posinset="3""#), "{html}");
    // идентификаторы вариантов {id}_{индекс}
    assert!(html.contains("_0\"") && html.contains("_4\""), "{html}");
}

#[test]
fn scroll_height_and_panel_attributes() {
    let html =
        view! { <Select options=cities() open=open() scroll_height="120px" panel_style="width: 30rem" panel_style_class="panel-extra" /> }
            .to_html();
    assert!(
        html.contains("max-height: 120px") && html.contains(r#"style="width: 30rem;""#) && html.contains("p-component panel-extra"),
        "{html}"
    );
}

#[test]
fn checkmark_icons() {
    let html = view! { <Select options=cities() default_value="LDN" checkmark=true open=open() /> }.to_html();
    assert_eq!(count(&html, "p-select-option-check-icon"), 1, "{html}");
    assert_eq!(count(&html, "p-select-option-blank-icon"), 4, "{html}");
    assert!(!html.contains("p-select-option-selected"), "с галочкой выделения классом нет: {html}");
}

#[test]
fn item_template_replaces_option_content() {
    let tpl = Callback::new(|o: SelectOption| view! { <b class="tpl">{format!("opt:{}", o.value)}</b> }.into_any());
    let html = view! { <Select options=cities() open=open() item_template=tpl /> }.to_html();
    assert_eq!(count(&html, r#"class="tpl""#), 5, "{html}");
    assert!(html.contains("opt:LDN") && !html.contains("p-select-option-label"), "{html}");
}

#[test]
fn groups_render_headers_and_options_in_order() {
    let html = view! { <Select groups=groups() open=open() /> }.to_html();
    assert_eq!(count(&html, "p-select-option-group-label"), 2, "{html}");
    assert_eq!(count(&html, "p-select-option-group\""), 2, "{html}");
    assert_eq!(count(&html, "<li"), 6, "{html}");
    let order: Vec<usize> =
        ["Germany", "Berlin", "Frankfurt", "USA", "Chicago", "Los Angeles"].iter().map(|l| html.find(l).unwrap()).collect();
    assert!(order.windows(2).all(|w| w[0] < w[1]), "{html}");
    assert!(html.contains(r#"aria-setsize="4""#) && html.contains(r#"aria-posinset="3""#), "{html}");
    // идентификаторы считаются вместе с заголовками: у «Chicago» индекс 4
    let chicago = html.find("Chicago").unwrap();
    assert!(html[..chicago].rfind("_4\"").is_some(), "{html}");
    let tpl = Callback::new(|g: SelectGroup| view! { <i class="gtpl">{g.label}</i> }.into_any());
    let html = view! { <Select groups=groups() open=open() group_template=tpl /> }.to_html();
    assert_eq!(count(&html, r#"class="gtpl""#), 2, "{html}");
}

#[test]
fn empty_messages() {
    let html = view! { <Select open=open() /> }.to_html();
    assert!(html.contains("p-select-empty-message") && html.contains("No results found"), "{html}");
    let html = view! { <Select open=open() empty_message="Nothing here" /> }.to_html();
    assert!(html.contains("Nothing here"), "{html}");
    assert_eq!(count(&html, "<li"), 1, "{html}");
}

#[test]
fn filter_input_in_the_header() {
    let html = view! { <Select options=cities() open=open() filter=true filter_placeholder="Search" aria_filter_label="Find" /> }.to_html();
    for expected in [
        "p-select-header",
        "p-iconfield",
        "p-inputtext p-component p-select-filter",
        r#"role="searchbox""#,
        r#"placeholder="Search""#,
        r#"aria-label="Find""#,
        "p-inputicon",
        "<svg",
    ] {
        assert!(html.contains(expected), "{expected}: {html}");
    }
    assert!(!view! { <Select options=cities() open=open() /> }.to_html().contains("p-select-filter"));
}

#[test]
fn pt_parts_are_added() {
    let pt = SelectPt {
        root: Pt::new().class("r").attr("data-r", "1"),
        label: Pt::new().class("lb"),
        dropdown: Pt::new().class("dd"),
        dropdown_icon: Pt::new().class("ddi"),
        overlay: Pt::new().class("ov"),
        list_container: Pt::new().class("lc"),
        list: Pt::new().class("ls"),
        option: Pt::new().class("custom"),
        option_label: Pt::new().class("ol"),
        ..SelectPt::default()
    };
    let html = view! { <Select options=cities() open=open() pt=pt /> }.to_html();
    for expected in [
        "p-inputwrapper-focus p-select-open r",
        r#"data-r="1""#,
        "p-select-label p-select-label-empty lb",
        "p-select-dropdown dd",
        "p-select-dropdown-icon ddi",
        "p-component-overlay p-component ov",
        "p-select-list-container lc",
        "p-select-list ls",
        "p-select-option custom",
        r#"class="ol""#,
    ] {
        assert!(html.contains(expected), "{expected}: {html}");
    }
}

#[test]
fn reactive_props_render_like_static() {
    let placeholder = RwSignal::new(Some("Pick".to_owned()));
    let reactive = view! { <Select options=cities() placeholder=placeholder /> }.to_html();
    let fixed = view! { <Select options=cities() placeholder="Pick" /> }.to_html();
    let strip = |s: &str| {
        let mut out = s.replace("<!>", "");
        // идентификаторы экземпляров отличаются
        while let Some(i) = out.find("pn_id_") {
            let end = out[i..].find('"').map_or(out.len(), |e| i + e);
            out.replace_range(i..end, "ID");
        }
        out
    };
    assert_eq!(strip(&reactive), strip(&fixed));
    placeholder.set(Some("Other".to_owned()));
    assert!(view! { <Select options=cities() placeholder=placeholder /> }.to_html().contains("Other"));
}

#[test]
fn reactive_options_render_the_current_list() {
    let options = RwSignal::new(cities());
    options.update(|o| o.truncate(2));
    let html = view! { <Select options=options open=open() /> }.to_html();
    assert_eq!(count(&html, "<li"), 2, "{html}");
}

#[test]
fn fluid_inherited_from_context() {
    let outside = view! { <Select options=cities()/> }.to_html();
    assert!(!outside.contains("p-select-fluid"), "{outside}");
    Owner::new().with(|| {
        prime_core::fluid::provide_fluid();
        let inherited = view! { <Select options=cities()/> }.to_html();
        assert!(inherited.contains("p-select-fluid"), "{inherited}");
        let forced_off = view! { <Select options=cities() fluid=false/> }.to_html();
        assert!(!forced_off.contains("p-select-fluid"), "{forced_off}");
        let live = RwSignal::new(None::<bool>);
        let reactive = view! { <Select options=cities() fluid=live/> }.to_html();
        assert!(reactive.contains("p-select-fluid"), "{reactive}");
        live.set(Some(false));
        let reactive_off = view! { <Select options=cities() fluid=live/> }.to_html();
        assert!(!reactive_off.contains("p-select-fluid"), "{reactive_off}");
    });
}
