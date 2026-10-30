//! Структура DOM, классы, шаблоны и `pt` Timeline (SSR).
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_core::Pt;
use prime_timeline::{Timeline, TimelineAlign, TimelineLayout, TimelinePt};

fn strip(s: &str) -> String {
    let mut out = s.replace("<!>", "");
    while let Some(start) = out.find("<!--") {
        let end = out[start..].find("-->").map_or(out.len(), |e| start + e + 3);
        out.replace_range(start..end, "");
    }
    out
}

fn count(html: &str, needle: &str) -> usize {
    html.matches(needle).count()
}

#[test]
fn defaults() {
    let html = strip(&view! { <Timeline value={vec![1, 2, 3]} /> }.to_html());
    assert!(html.contains("class=\"p-timeline p-component p-timeline-left p-timeline-vertical\""), "{html}");
    assert!(html.contains("data-p=\"vertical left\""), "{html}");
}

#[test]
fn three_events_have_three_markers_and_two_connectors() {
    let html = strip(&view! { <Timeline value={vec![1, 2, 3]} /> }.to_html());
    assert_eq!(count(&html, "class=\"p-timeline-event \""), 0);
    assert_eq!(count(&html, "p-timeline-event-opposite"), 3, "{html}");
    assert_eq!(count(&html, "p-timeline-event-separator"), 3, "{html}");
    assert_eq!(count(&html, "p-timeline-event-content"), 3, "{html}");
    assert_eq!(count(&html, "p-timeline-event-marker"), 3, "{html}");
    assert_eq!(count(&html, "p-timeline-event-connector"), 2, "{html}");
}

#[test]
fn empty_list_renders_only_root() {
    let html = strip(&view! { <Timeline value={Vec::<i32>::new()} /> }.to_html());
    assert!(!html.contains("p-timeline-event"), "{html}");
}

#[test]
fn horizontal_top() {
    let html = strip(&view! { <Timeline value={vec![1]} align=TimelineAlign::Top layout=TimelineLayout::Horizontal /> }.to_html());
    assert!(html.contains("p-timeline-top") && html.contains("p-timeline-horizontal"), "{html}");
}

#[test]
fn reactive_align_replaces_class() {
    let align = RwSignal::new(TimelineAlign::Left);
    let html = strip(&view! { <Timeline value={vec![1]} align=align /> }.to_html());
    assert!(html.contains("p-timeline-left"), "{html}");
}

#[test]
fn templates_render_in_parts_and_marker_replaces_default() {
    let content = Callback::new(|e: i32| view! { <b>{format!("c{e}")}</b> }.into_any());
    let opposite = Callback::new(|e: i32| view! { <u>{format!("o{e}")}</u> }.into_any());
    let marker = Callback::new(|e: i32| view! { <s>{format!("m{e}")}</s> }.into_any());
    let html = strip(&view! { <Timeline value={vec![1, 2]} content=content opposite=opposite marker=marker /> }.to_html());
    for needle in ["<b>c1</b>", "<b>c2</b>", "<u>o1</u>", "<s>m2</s>"] {
        assert!(html.contains(needle), "{needle}: {html}");
    }
    assert_eq!(count(&html, "p-timeline-event-marker"), 0, "{html}");
    assert_eq!(count(&html, "p-timeline-event-connector"), 1, "{html}");
}

#[test]
fn style_class_style_and_pt() {
    let pt = TimelinePt { root: Pt::new().attr("data-x", "1"), event_marker: Pt::new().class("custom"), ..Default::default() };
    let html = strip(&view! { <Timeline value={vec![1, 2]} style_class="mine" style="width: 5px" pt=pt /> }.to_html());
    assert!(html.contains("p-timeline-vertical mine"), "{html}");
    assert!(html.contains("width: 5px") && html.contains("data-x=\"1\""), "{html}");
    assert_eq!(count(&html, "p-timeline-event-marker custom"), 2, "{html}");
}
