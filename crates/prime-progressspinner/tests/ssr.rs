//! Структура DOM, значения по умолчанию, ARIA, `style`, `pt` и паритет ветвей ProgressSpinner (SSR).
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_core::Pt;
use prime_progressspinner::{ProgressSpinner, ProgressSpinnerPt};

fn has(html: &str, wants: &[&str]) {
    for want in wants {
        assert!(html.contains(want), "{want}: {html}");
    }
}

fn lacks(html: &str, unwanted: &[&str]) {
    for u in unwanted {
        assert!(!html.contains(u), "{u}: {html}");
    }
}

#[test]
fn default_markup() {
    let html = view! { <ProgressSpinner /> }.to_html();
    assert!(html.starts_with("<div"), "{html}");
    has(
        &html,
        &[
            r#"class="p-progressspinner""#,
            r#"role="progressbar""#,
            r#"aria-busy="true""#,
            "p-progressspinner-spin",
            "viewBox=\"25 25 50 50\"",
            "p-progressspinner-circle",
            r#"cx="50""#,
            r#"cy="50""#,
            r#"r="20""#,
            r#"fill="none""#,
            r#"stroke-width="2""#,
            r#"stroke-miterlimit="10""#,
            "animation-duration: 2s",
        ],
    );
    lacks(&html, &["aria-label", "data-p"]);
}

#[test]
fn custom_values() {
    let html = view! { <ProgressSpinner stroke_width="8" fill="transparent" animation_duration=".5s" style="width: 50px; height: 50px" /> }
        .to_html();
    has(&html, &[r#"stroke-width="8""#, r#"fill="transparent""#, "animation-duration: .5s", "width: 50px; height: 50px"]);
}

#[test]
fn aria_label_and_class() {
    let html = view! { <ProgressSpinner aria_label="loading" style_class="mine" /> }.to_html();
    has(&html, &[r#"aria-label="loading""#, "p-progressspinner mine"]);
}

#[test]
fn pt_parts_extend_own_classes_and_attrs() {
    let pt = ProgressSpinnerPt {
        root: Pt::new().class("r").attr("data-x", "1"),
        spin: Pt::new().class("s").style("color: red"),
        circle: Pt::new().class("custom"),
    };
    let html = view! { <ProgressSpinner pt=pt /> }.to_html();
    has(
        &html,
        &[
            "p-progressspinner r",
            r#"data-x="1""#,
            "p-progressspinner-spin s",
            "color: red",
            "animation-duration: 2s",
            "p-progressspinner-circle custom",
        ],
    );
}

#[test]
fn static_and_reactive_branches_render_the_same() {
    let fixed =
        view! { <ProgressSpinner stroke_width="8" fill="red" animation_duration="1s" aria_label="a" style="color: red" style_class="m" /> }
            .to_html();
    let live = view! {
        <ProgressSpinner
            stroke_width=RwSignal::new(Some("8".to_owned()))
            fill=RwSignal::new(Some("red".to_owned()))
            animation_duration=RwSignal::new(Some("1s".to_owned()))
            aria_label=RwSignal::new(Some("a".to_owned()))
            style=RwSignal::new(Some("color: red".to_owned()))
            style_class=RwSignal::new(Some("m".to_owned()))
        />
    }
    .to_html();
    let norm = |s: &str| s.replace("<!>", "").replace("<!---->", "");
    assert_eq!(norm(&fixed), norm(&live), "{fixed}\n{live}");
}

#[test]
fn reactive_stroke_width_updates() {
    let owner = Owner::new();
    owner.with(|| {
        let width = RwSignal::new(Some("4".to_owned()));
        let html = view! { <ProgressSpinner stroke_width=width /> }.to_html();
        has(&html, &[r#"stroke-width="4""#]);
        width.set(Some("9".to_owned()));
        let html = view! { <ProgressSpinner stroke_width=width /> }.to_html();
        has(&html, &[r#"stroke-width="9""#]);
        lacks(&html, &[r#"stroke-width="4""#]);
    });
}
