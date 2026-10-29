//! Структура DOM, классы, `data-p*`, атрибуты, `tabindex`, слоты и `pt` ToggleButton (SSR).
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_core::{Pt, Size};
use prime_togglebutton::{IconPos, ToggleButton, ToggleButtonPt};

fn icon() -> ViewFn {
    ViewFn::from(|| view! { <i class="ic" /> })
}

#[test]
fn structure_and_default_labels() {
    let html = view! { <ToggleButton /> }.to_html();
    assert!(html.starts_with("<div"), "{html}");
    for expected in [
        r#"class="p-togglebutton p-component""#,
        r#"role="button""#,
        r#"aria-pressed="false""#,
        r#"tabindex="0""#,
        r#"data-pc-name="togglebutton""#,
        r#"data-p-checked="false""#,
        r#"data-p-disabled="false""#,
        "p-togglebutton-content",
    ] {
        assert!(html.contains(expected), "{expected}: {html}");
    }
    let content = html.find("p-togglebutton-content").unwrap();
    let label = html.find("p-togglebutton-label").unwrap();
    assert!(content < label, "{html}");
    for unwanted in ["p-togglebutton-icon", "p-togglebutton-checked", " p-disabled", "p-invalid", "aria-label", "style=", "<input"] {
        assert!(!html.contains(unwanted), "{unwanted}: {html}");
    }
    assert!(html.contains(">No<"), "{html}");
    assert!(view! { <ToggleButton default_checked=true /> }.to_html().contains(">Yes<"));
}

#[test]
fn checked_state_is_reflected_everywhere() {
    let html = view! { <ToggleButton checked=RwSignal::new(true) on_label="On" off_label="Off" /> }.to_html();
    for expected in [
        "p-togglebutton p-component p-togglebutton-checked",
        r#"aria-pressed="true""#,
        r#"data-p-checked="true""#,
        r#"data-p="checked""#,
        ">On<",
    ] {
        assert!(html.contains(expected), "{expected}: {html}");
    }
    assert!(!html.contains(">Off<"), "{html}");
}

#[test]
fn empty_label_is_a_space() {
    let html = view! { <ToggleButton on_label="" off_label="" /> }.to_html();
    assert!(html.contains("p-togglebutton-label") && html.contains("> <"), "{html}");
}

#[test]
fn modifiers_and_data_p() {
    let html = view! { <ToggleButton invalid=true size=Size::Small fluid=true /> }.to_html();
    assert!(html.contains("p-togglebutton p-component p-invalid p-togglebutton-sm p-inputfield-sm p-togglebutton-fluid"), "{html}");
    assert!(html.contains(r#"data-p="invalid small""#), "{html}");
    let html = view! { <ToggleButton size=Size::Large disabled=true /> }.to_html();
    assert!(html.contains("p-togglebutton p-component p-disabled p-togglebutton-lg p-inputfield-lg"), "{html}");
}

#[test]
fn accessibility_attributes() {
    let html = view! { <ToggleButton checked=RwSignal::new(true) aria_label="Confirm" aria_labelledby="lbl" /> }.to_html();
    for expected in
        [r#"role="button""#, r#"aria-pressed="true""#, r#"aria-label="Confirm""#, r#"aria-labelledby="lbl""#, r#"data-p="checked""#]
    {
        assert!(html.contains(expected), "{expected}: {html}");
    }
}

#[test]
fn tabindex_rules() {
    assert!(view! { <ToggleButton disabled=true /> }.to_html().contains(r#"tabindex="-1""#));
    assert!(view! { <ToggleButton disabled=true /> }.to_html().contains(r#"data-p-disabled="true""#));
    assert!(view! { <ToggleButton tabindex=3 /> }.to_html().contains(r#"tabindex="3""#));
    assert!(view! { <ToggleButton tabindex=3 disabled=true /> }.to_html().contains(r#"tabindex="3""#));
}

#[test]
fn icon_follows_state_and_position() {
    let on = ViewFn::from(|| view! { <i class="on-ic" /> });
    let off = ViewFn::from(|| view! { <i class="off-ic" /> });
    let html = view! { <ToggleButton on_icon=on.clone() off_icon=off.clone() /> }.to_html();
    assert!(
        html.contains("p-togglebutton-icon p-togglebutton-icon-left")
            && html.contains(r#"class="off-ic""#)
            && !html.contains(r#"class="on-ic""#),
        "{html}"
    );
    assert!(html.find("p-togglebutton-icon").unwrap() < html.find("p-togglebutton-label").unwrap(), "{html}");
    let html = view! { <ToggleButton default_checked=true on_icon=on off_icon=off icon_pos=IconPos::Right /> }.to_html();
    assert!(
        html.contains("p-togglebutton-icon p-togglebutton-icon-right")
            && html.contains(r#"class="on-ic""#)
            && !html.contains(r#"class="off-ic""#),
        "{html}"
    );
    let html = view! { <ToggleButton on_icon=icon() /> }.to_html();
    assert!(html.contains("p-togglebutton-icon"), "span без иконки выключенного состояния всё равно выводится: {html}");
}

#[test]
fn icon_template_replaces_default_icon() {
    let tpl = Callback::new(|c: bool| view! { <b class="tpl">{if c { "1" } else { "0" }}</b> }.into_any());
    let html = view! { <ToggleButton icon_template=tpl on_icon=icon() /> }.to_html();
    assert!(html.contains(r#"class="tpl""#) && !html.contains("p-togglebutton-icon") && !html.contains("ic\""), "{html}");
    assert!(html.contains("p-togglebutton-label"), "{html}");
}

#[test]
fn content_template_replaces_everything() {
    let tpl = Callback::new(|c: bool| view! { <b class="tpl">{if c { "yes!" } else { "no!" }}</b> }.into_any());
    let html = view! { <ToggleButton content_template=tpl default_checked=true on_icon=icon() /> }.to_html();
    assert!(html.contains("yes!") && html.contains("p-togglebutton-content"), "{html}");
    assert!(!html.contains("p-togglebutton-label") && !html.contains("p-togglebutton-icon"), "{html}");
}

#[test]
fn style_and_style_class() {
    let html = view! { <ToggleButton style="width: 9rem" style_class="mine" /> }.to_html();
    assert!(html.contains(r#"style="width: 9rem;""#) && html.contains("p-togglebutton p-component mine"), "{html}");
}

#[test]
fn pt_parts_are_added() {
    let pt = ToggleButtonPt {
        root: Pt::new().class("r").style("color: red").attr("data-r", "1"),
        content: Pt::new().class("c"),
        icon: Pt::new().class("i"),
        label: Pt::new().class("custom").attr("data-l", "2"),
    };
    let html = view! { <ToggleButton pt=pt on_icon=icon() off_icon=icon() /> }.to_html();
    for expected in [
        "p-togglebutton p-component r",
        r#"style="color: red;""#,
        r#"data-r="1""#,
        "p-togglebutton-content c",
        "p-togglebutton-icon p-togglebutton-icon-left i",
        "p-togglebutton-label custom",
        r#"data-l="2""#,
    ] {
        assert!(html.contains(expected), "{expected}: {html}");
    }
}

#[test]
fn reactive_props_render_like_static() {
    let on_label = RwSignal::new(Some("A".to_owned()));
    let disabled = RwSignal::new(false);
    let reactive = view! { <ToggleButton default_checked=true on_label=on_label disabled=disabled /> }.to_html();
    let fixed = view! { <ToggleButton default_checked=true on_label="A" /> }.to_html();
    let strip = |s: &str| s.replace("<!>", "");
    assert_eq!(strip(&reactive), strip(&fixed));
    on_label.set(Some("B".to_owned()));
    disabled.set(true);
    let html = view! { <ToggleButton default_checked=true on_label=on_label disabled=disabled /> }.to_html();
    assert!(html.contains(">B<") && html.contains("p-disabled"), "{html}");
}
