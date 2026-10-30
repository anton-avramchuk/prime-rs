//! Разметка Splitter (SSR): корень, панели, разделители, размеры, раскладка, вложенность, `pt`.
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_core::Pt;
use prime_splitter::{Splitter, SplitterLayout, SplitterPt};

fn panels(n: usize) -> Vec<ViewFn> {
    (1..=n).map(|i| ViewFn::from(move || view! { <p>{format!("Panel {i}")}</p> })).collect()
}

fn count(html: &str, needle: &str) -> usize {
    html.matches(needle).count()
}

#[test]
fn two_panels_one_gutter() {
    let html = view! { <Splitter panels=panels(2) /> }.to_html();
    assert!(html.contains("p-splitter p-component p-splitter-horizontal"), "{html}");
    assert!(html.contains("display: flex; flex-wrap: nowrap"), "{html}");
    assert_eq!(count(&html, r#"class="p-splitterpanel""#), 2, "{html}");
    assert_eq!(count(&html, "p-splitter-gutter\""), 1, "разделитель только между панелями: {html}");
    assert!(html.contains("Panel 1") && html.contains("Panel 2"), "{html}");
    assert!(html.contains(r#"role="separator""#) && html.contains(r#"data-p-gutter-resizing="false""#), "{html}");
    assert!(html.contains(r#"tabindex="0""#) && html.contains(r#"data-pc-section="panel""#), "{html}");
}

#[test]
fn default_sizes_are_equal() {
    let html = view! { <Splitter panels=panels(2) /> }.to_html();
    assert_eq!(count(&html, "flex-basis: calc(50% - 4px)"), 2, "{html}");
    assert!(html.contains(r#"aria-valuenow="50.0000""#), "{html}");
    let three = view! { <Splitter panels=panels(3) /> }.to_html();
    assert_eq!(count(&three, "calc(33.333333333333336% - 8px)"), 3, "{three}");
    assert_eq!(count(&three, "p-splitter-gutter\""), 2, "{three}");
}

#[test]
fn given_sizes_and_gutter_size() {
    let html = view! { <Splitter panels=panels(2) panel_sizes=vec![20.0, 80.0] gutter_size=6.0 /> }.to_html();
    assert!(html.contains("flex-basis: calc(20% - 6px)") && html.contains("flex-basis: calc(80% - 6px)"), "{html}");
    assert!(html.contains("width: 6px") && html.contains(r#"aria-valuenow="20.0000""#), "{html}");
}

#[test]
fn vertical_layout() {
    let html = view! { <Splitter panels=panels(2) layout=SplitterLayout::Vertical /> }.to_html();
    assert!(html.contains("p-splitter-vertical") && html.contains("flex-direction: column"), "{html}");
    assert!(html.contains(r#"aria-orientation="vertical""#) && html.contains("height: 4px"), "{html}");
    assert!(html.contains(r#"data-p="vertical""#), "{html}");
}

#[test]
fn nested_flag() {
    let html = view! { <Splitter panels=panels(2) nested=true /> }.to_html();
    assert_eq!(count(&html, "p-splitterpanel p-splitterpanel-nested"), 2, "{html}");
    assert!(html.contains(r#"data-p="horizontal nested""#), "{html}");
}

#[test]
fn style_classes_and_pt() {
    let pt = SplitterPt {
        root: Pt::new().class("from-pt").attr("data-testid", "root"),
        panel: Pt::new().class("panel-pt"),
        gutter: Pt::new().attr("data-gutter", "x"),
        gutter_handle: Pt::new().class("handle-pt"),
    };
    let html = view! {
        <Splitter panels=panels(2) style="height: 300px" style_class="mine" panel_style="color: red" panel_style_class="panel-mine" pt=pt />
    }
    .to_html();
    for needle in [
        "mine",
        "from-pt",
        "height: 300px",
        "color: red",
        "panel-mine",
        "panel-pt",
        "handle-pt",
        r#"data-testid="root""#,
        r#"data-gutter="x""#,
    ] {
        assert!(html.contains(needle), "{needle}: {html}");
    }
}

#[test]
fn single_panel_has_no_gutter() {
    let html = view! { <Splitter panels=panels(1) /> }.to_html();
    assert_eq!(count(&html, "p-splitter-gutter\""), 0, "{html}");
    assert!(html.contains("calc(100% - 0px)"), "{html}");
}
