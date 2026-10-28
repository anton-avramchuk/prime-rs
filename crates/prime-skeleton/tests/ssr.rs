//! Структура DOM, классы, стиль, `size`, `pt` и паритет ветвей Skeleton (SSR).
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_core::Pt;
use prime_skeleton::{Skeleton, SkeletonAnimation, SkeletonShape};

fn strip(s: &str) -> String {
    let mut out = s.replace("<!>", "");
    while let Some(start) = out.find("<!--") {
        let end = out[start..].find("-->").map_or(out.len(), |e| start + e + 3);
        out.replace_range(start..end, "");
    }
    out
}

#[test]
fn defaults_render_rectangle_wave_with_default_size() {
    let html = view! { <Skeleton /> }.to_html();
    assert!(html.starts_with("<div"), "{html}");
    for want in [
        r#"aria-hidden="true""#,
        r#"class="p-skeleton p-component""#,
        r#"data-p="rectangle""#,
        r#"style="position: relative; width: 100%; height: 1rem;""#,
    ] {
        assert!(html.contains(want), "{want}: {html}");
    }
    for unwanted in ["p-skeleton-circle", "p-skeleton-animation-none", "border-radius"] {
        assert!(!html.contains(unwanted), "{unwanted}: {html}");
    }
}

#[test]
fn circle_and_no_animation_classes() {
    let html = view! { <Skeleton shape=SkeletonShape::Circle animation=SkeletonAnimation::None /> }.to_html();
    assert!(html.contains("p-skeleton p-component p-skeleton-circle p-skeleton-animation-none"), "{html}");
    assert!(html.contains(r#"data-p="circle""#), "{html}");
}

#[test]
fn width_height_and_border_radius() {
    let html = view! { <Skeleton width="10rem" height="4rem" border_radius="16px" /> }.to_html();
    assert!(html.contains("position: relative; width: 10rem; height: 4rem; border-radius: 16px;"), "{html}");
}

#[test]
fn size_overrides_width_and_height() {
    let html = view! { <Skeleton size="3rem" width="9rem" height="2rem" /> }.to_html();
    assert!(html.contains("width: 3rem; height: 3rem;") && !html.contains("9rem") && !html.contains("2rem"), "{html}");
}

#[test]
fn style_class_and_pt_are_added() {
    let pt = Pt::new().class("custom").style("margin: 0").attr("data-x", "y");
    let html = view! { <Skeleton style_class="mine" pt=pt /> }.to_html();
    assert!(html.contains("p-skeleton p-component mine custom"), "{html}");
    assert!(html.contains("height: 1rem; margin: 0") && html.contains(r#"data-x="y""#), "{html}");
}

#[test]
fn static_and_reactive_branches_match() {
    let stat = view! { <Skeleton shape=SkeletonShape::Circle size="4rem" border_radius="50%" style_class="x" /> }.to_html();
    let reactive = view! {
        <Skeleton
            shape=RwSignal::new(SkeletonShape::Circle)
            size=RwSignal::new(Some("4rem".to_owned()))
            border_radius=RwSignal::new(Some("50%".to_owned()))
            style_class=RwSignal::new(Some("x".to_owned()))
        />
    }
    .to_html();
    assert_eq!(strip(&stat), strip(&reactive));
}
