//! Структура DOM, классы, `data-p*`, стиль корня, слот, `pt` и паритет ветвей ToggleSwitch (SSR).
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_core::Pt;
use prime_toggleswitch::{ToggleSwitch, ToggleSwitchPt};

fn strip(s: &str) -> String {
    s.split("<!--").next().unwrap_or(s).to_owned()
}

#[test]
fn structure_is_input_slider_handle() {
    let html = view! { <ToggleSwitch /> }.to_html();
    assert!(html.starts_with("<div"), "{html}");
    assert!(
        html.contains("p-toggleswitch p-component") && html.contains(r#"role="switch""#) && html.contains(r#"type="checkbox""#),
        "{html}"
    );
    let input = html.find("p-toggleswitch-input").expect("input");
    let slider = html.find("p-toggleswitch-slider").expect("slider");
    let handle = html.find("p-toggleswitch-handle").expect("handle");
    assert!(input < slider && slider < handle, "{html}");
    for unwanted in ["p-toggleswitch-checked", " p-disabled", "p-invalid", "required", "name=", "tabindex"] {
        assert!(!html.contains(unwanted), "{unwanted}: {html}");
    }
    assert!(html.contains(r#"style="position: relative;""#), "{html}");
    assert!(html.contains(r#"data-p-checked="false""#) && html.contains(r#"aria-checked="false""#), "{html}");
}

#[test]
fn checked_state_is_reflected_everywhere() {
    let html = view! { <ToggleSwitch checked=RwSignal::new(true) /> }.to_html();
    for want in ["p-toggleswitch-checked", r#"aria-checked="true""#, r#"data-p-checked="true""#, r#"data-p="checked""#] {
        assert!(html.contains(want), "{want}: {html}");
    }
    assert_eq!(html.matches(r#"data-p="checked""#).count(), 3, "root, slider, handle: {html}");
}

#[test]
fn modifiers_and_data_p() {
    let html = view! { <ToggleSwitch disabled=true invalid=true default_checked=true /> }.to_html();
    for want in [" p-disabled", "p-invalid", r#"data-p-disabled="true""#, r#"data-p="checked disabled invalid""#] {
        assert!(html.contains(want), "{want}: {html}");
    }
}

#[test]
fn input_attributes_are_passed() {
    let html = view! { <ToggleSwitch input_id="i" name="n" tabindex=1 aria_label="L" aria_labelledby="B" required=true /> }.to_html();
    for want in [r#"id="i""#, r#"name="n""#, r#"tabindex="1""#, r#"aria-label="L""#, r#"aria-labelledby="B""#, "required"] {
        assert!(html.contains(want), "{want}: {html}");
    }
}

#[test]
fn root_style_is_merged_after_position() {
    let html = view! { <ToggleSwitch style="color: red" pt=ToggleSwitchPt { root: Pt::new().style("margin: 0"), ..Default::default() } /> }
        .to_html();
    assert!(html.contains("position: relative"), "{html}");
    assert!(html.contains("color: red") && html.contains("margin: 0"), "{html}");
}

#[test]
fn handle_template_gets_checked() {
    let tpl = Callback::new(|checked: bool| view! { <i data-testid="h">{checked.to_string()}</i> }.into_any());
    let html = view! { <ToggleSwitch default_checked=true handle_template=tpl /> }.to_html();
    assert!(html.contains("data-testid=\"h\"") && html.contains(">true<"), "{html}");
}

#[test]
fn pt_parts_are_added() {
    let pt = ToggleSwitchPt {
        root: Pt::new().class("r"),
        input: Pt::new().class("i"),
        slider: Pt::new().class("s"),
        handle: Pt::new().class("h"),
    };
    let html = view! { <ToggleSwitch style_class="mine" pt=pt /> }.to_html();
    for want in ["p-toggleswitch p-component mine r", "p-toggleswitch-input i", "p-toggleswitch-slider s", "p-toggleswitch-handle h"] {
        assert!(html.contains(want), "{want}: {html}");
    }
}

#[test]
fn static_and_reactive_branches_match() {
    let stat = view! { <ToggleSwitch default_checked=true name="n" invalid=true style_class="x" style="color: red" /> }.to_html();
    let reactive = view! {
        <ToggleSwitch
            checked=RwSignal::new(true)
            name=RwSignal::new(Some("n".to_owned()))
            invalid=RwSignal::new(true)
            style_class=RwSignal::new(Some("x".to_owned()))
            style=RwSignal::new(Some("color: red".to_owned()))
        />
    }
    .to_html();
    assert_eq!(strip(&stat), strip(&reactive));
}
