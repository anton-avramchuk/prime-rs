//! Структура DOM обоих режимов, ARIA, ширина, подпись, `color`, шаблон, `pt` и паритет ветвей ProgressBar (SSR).
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_core::Pt;
use prime_progressbar::{ProgressBar, ProgressBarMode, ProgressBarPt};

fn strip(s: &str) -> String {
    let mut out = s.replace("<!>", "");
    while let Some(start) = out.find("<!--") {
        let end = out[start..].find("-->").map_or(out.len(), |e| start + e + 3);
        out.replace_range(start..end, "");
    }
    out
}

#[test]
fn determinate_structure_aria_and_width() {
    let html = view! { <ProgressBar value=50.0 /> }.to_html();
    for want in [
        r#"role="progressbar""#,
        r#"aria-valuemin="0""#,
        r#"aria-valuenow="50""#,
        r#"aria-valuemax="100""#,
        r#" aria-level="50%""#,
        "p-progressbar p-component p-progressbar-determinate",
        r#"data-p="determinate""#,
        "width: 50%",
        "display: flex",
        ">50%<",
    ] {
        assert!(html.contains(want), "{want}: {html}");
    }
    let value = html.find("p-progressbar-value").expect("value");
    let label = html.find("p-progressbar-label").expect("label");
    assert!(value < label, "{html}");
}

#[test]
fn without_value_aria_attributes_are_absent() {
    let html = view! { <ProgressBar /> }.to_html();
    assert!(!html.contains("aria-valuenow") && !html.contains("aria-level") && !html.contains("width:"), "{html}");
    assert!(html.contains("display: none"), "подпись скрыта: {html}");
}

#[test]
fn zero_value_hides_the_label_text() {
    let html = view! { <ProgressBar value=0.0 /> }.to_html();
    assert!(html.contains("width: 0%") && html.contains("display: none") && html.contains(">0%<"), "{html}");
}

#[test]
fn fractional_value_and_custom_unit() {
    let html = view! { <ProgressBar value=33.5 unit="pts" /> }.to_html();
    assert!(html.contains(">33.5pts<") && html.contains(r#"aria-level="33.5pts""#) && html.contains("width: 33.5%"), "{html}");
}

#[test]
fn show_value_false_leaves_the_label_empty() {
    let html = view! { <ProgressBar value=40.0 show_value=false /> }.to_html();
    assert!(html.contains("p-progressbar-label") && !html.contains(">40%<"), "{html}");
}

#[test]
fn color_and_value_style_class() {
    let html = view! { <ProgressBar value=10.0 color="red" value_style_class="mine" /> }.to_html();
    assert!(html.contains("background: red") && html.contains("p-progressbar-value mine"), "{html}");
}

#[test]
fn indeterminate_has_only_the_value_element() {
    let html = view! { <ProgressBar mode=ProgressBarMode::Indeterminate color="blue" /> }.to_html();
    for want in ["p-progressbar-indeterminate", r#"data-p="indeterminate""#, "p-progressbar-value", "background: blue"] {
        assert!(html.contains(want), "{want}: {html}");
    }
    for unwanted in ["p-progressbar-label", "width:", "display: flex"] {
        assert!(!html.contains(unwanted), "{unwanted}: {html}");
    }
}

#[test]
fn content_template_replaces_the_value_text() {
    let tpl = Callback::new(|v: Option<f64>| view! { <i data-testid="t">{format!("{} done", v.unwrap_or_default())}</i> }.into_any());
    let html = view! { <ProgressBar value=70.0 content_template=tpl /> }.to_html();
    assert!(html.contains("data-testid=\"t\"") && html.contains("70 done") && !html.contains(">70%<"), "{html}");
}

#[test]
fn pt_parts_are_added() {
    let pt = ProgressBarPt { root: Pt::new().class("r"), value: Pt::new().class("v"), label: Pt::new().class("l") };
    let html = view! { <ProgressBar value=1.0 style_class="mine" pt=pt /> }.to_html();
    for want in ["p-progressbar-determinate mine r", "p-progressbar-value v", "p-progressbar-label l"] {
        assert!(html.contains(want), "{want}: {html}");
    }
}

#[test]
fn static_and_reactive_branches_match() {
    let stat = view! { <ProgressBar value=25.0 color="red" style_class="x" /> }.to_html();
    let reactive = view! {
        <ProgressBar
            value=RwSignal::new(Some(25.0))
            color=RwSignal::new(Some("red".to_owned()))
            style_class=RwSignal::new(Some("x".to_owned()))
            mode=RwSignal::new(ProgressBarMode::Determinate)
        />
    }
    .to_html();
    assert_eq!(strip(&stat), strip(&reactive));
}
