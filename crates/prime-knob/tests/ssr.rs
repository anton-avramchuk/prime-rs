//! Структура DOM, `aria-*`, размер, дуги, текст, цвета, `disabled`/`readonly` и `pt` Knob (SSR).
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_core::Pt;
use prime_knob::{Knob, KnobPt};

fn count(html: &str, needle: &str) -> usize {
    html.matches(needle).count()
}

#[test]
fn default_structure() {
    let html = view! { <Knob default_value=60.0 /> }.to_html();
    assert!(html.starts_with("<div"), "{html}");
    for expected in [
        r#"class="p-knob p-component""#,
        r#"viewBox="0 0 100 100""#,
        r#"role="slider""#,
        "width: 100px; height: 100px",
        r#"aria-valuemin="0""#,
        r#"aria-valuemax="100""#,
        r#"aria-valuenow="60""#,
        r#"tabindex="0""#,
        r#"class="p-knob-range""#,
        r#"class="p-knob-value""#,
        r#"stroke-width="14""#,
        r#"stroke="var(--p-knob-range-background)""#,
        r#"stroke="var(--p-knob-value-background)""#,
        r#"class="p-knob-text""#,
        r#"x="50""#,
        r#"y="57""#,
        r#"text-anchor="middle""#,
        r#"fill="var(--p-knob-text-color)""#,
        ">60<",
    ] {
        assert!(html.contains(expected), "{expected}: {html}");
    }
    assert_eq!(count(&html, "<path"), 2, "{html}");
    let (range, value, text) = (html.find("p-knob-range").unwrap(), html.find("p-knob-value").unwrap(), html.find("p-knob-text").unwrap());
    assert!(range < value && value < text, "{html}");
    for unwanted in ["p-disabled", "required", "aria-label", "name="] {
        assert!(!html.contains(unwanted), "{unwanted}: {html}");
    }
}

#[test]
fn paths_follow_the_value() {
    let range = r#"d="M 30 84.641 A 40 40 0 1 1 70 84.641""#;
    assert!(view! { <Knob default_value=50.0 /> }.to_html().contains(range));
    // 50 из 0..100 — верх шкалы; дуга значения идёт от нуля (нижний левый край)
    assert!(view! { <Knob default_value=50.0 /> }.to_html().contains(r#"d="M 30 84.641 A 40 40 0 0 1 50 10""#));
    // 0 из -50..50 — верх шкалы: дуга нулевой длины
    assert!(view! { <Knob default_value=0.0 min=-50.0 max=50.0 /> }.to_html().contains(r#"d="M 50 10 A 40 40 0 0 1 50 10""#));
    // максимум — большая дуга до нижнего правого края
    assert!(view! { <Knob default_value=100.0 /> }.to_html().contains(r#"d="M 30 84.641 A 40 40 0 1 1 70 84.641""#));
}

#[test]
fn out_of_range_value_is_drawn_clamped_but_shown_as_is() {
    let html = view! { <Knob default_value=500.0 /> }.to_html();
    assert!(html.contains(r#"d="M 30 84.641 A 40 40 0 1 1 70 84.641""#), "{html}");
    assert!(html.contains(r#"aria-valuenow="500""#) && html.contains(">500<"), "{html}");
}

#[test]
fn value_defaults_to_min_and_signal_wins() {
    assert!(view! { <Knob min=10.0 max=20.0 /> }.to_html().contains(r#"aria-valuenow="10""#));
    assert!(view! { <Knob value=RwSignal::new(33.0) default_value=1.0 /> }.to_html().contains(r#"aria-valuenow="33""#));
}

#[test]
fn template_and_show_value() {
    let html = view! { <Knob default_value=60.0 value_template="{value}%" /> }.to_html();
    assert!(html.contains(">60%<"), "{html}");
    assert!(view! { <Knob default_value=60.0 show_value=false /> }.to_html().find("<text").is_none());
    assert!(view! { <Knob default_value=5.0 name="vol" /> }.to_html().contains(r#"name="vol""#));
}

#[test]
fn size_stroke_colors_and_labels() {
    let html = view! {
        <Knob
            default_value=40.0
            size=200
            stroke_width=5.0
            value_color="SlateGray"
            range_color="MediumTurquoise"
            text_color="red"
            aria_label="Volume"
            aria_labelledby="lbl"
            required=true
        />
    }
    .to_html();
    for expected in [
        "width: 200px; height: 200px",
        r#"stroke-width="5""#,
        r#"stroke="SlateGray""#,
        r#"stroke="MediumTurquoise""#,
        r#"fill="red""#,
        r#"aria-label="Volume""#,
        r#"aria-labelledby="lbl""#,
        "required",
    ] {
        assert!(html.contains(expected), "{expected}: {html}");
    }
    assert!(!html.contains("--p-knob"), "цвета по умолчанию не должны остаться: {html}");
}

#[test]
fn disabled_readonly_and_tabindex() {
    let html = view! { <Knob disabled=true /> }.to_html();
    assert!(html.contains(r#"class="p-knob p-component p-disabled""#) && html.contains(r#"tabindex="-1""#), "{html}");
    assert!(view! { <Knob readonly=true /> }.to_html().contains(r#"tabindex="-1""#));
    assert!(!view! { <Knob readonly=true /> }.to_html().contains("p-disabled"));
    assert!(view! { <Knob tabindex=4 /> }.to_html().contains(r#"tabindex="4""#));
}

#[test]
fn style_and_style_class() {
    let html = view! { <Knob style="margin: 1px" style_class="mine" /> }.to_html();
    assert!(html.contains(r#"class="p-knob p-component mine""#) && html.contains(r#"style="margin: 1px;""#), "{html}");
}

#[test]
fn pt_parts_are_added() {
    let pt = KnobPt {
        root: Pt::new().class("r").attr("data-r", "1"),
        svg: Pt::new().class("s").style("opacity: 0.5"),
        range: Pt::new().class("rg"),
        value: Pt::new().class("v").attr("data-v", "2"),
        text: Pt::new().class("custom"),
    };
    let html = view! { <Knob pt=pt /> }.to_html();
    for expected in [
        "p-knob p-component r",
        r#"data-r="1""#,
        "width: 100px; height: 100px; opacity: 0.5",
        "p-knob-range rg",
        "p-knob-value v",
        r#"data-v="2""#,
        "p-knob-text custom",
    ] {
        assert!(html.contains(expected), "{expected}: {html}");
    }
    assert!(html.contains(r#"class="s""#), "{html}");
}

#[test]
fn reactive_props_render_like_static() {
    let max = RwSignal::new(Some(200.0));
    let reactive = view! { <Knob default_value=50.0 max=max /> }.to_html();
    let fixed = view! { <Knob default_value=50.0 max=200.0 /> }.to_html();
    let strip = |s: &str| s.replace("<!>", "");
    assert_eq!(strip(&reactive), strip(&fixed));
    max.set(Some(100.0));
    assert!(view! { <Knob default_value=50.0 max=max /> }.to_html().contains(r#"aria-valuemax="100""#));
}
