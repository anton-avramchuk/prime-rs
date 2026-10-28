//! Рендер иконок (SSR) и a11y.
#![cfg(all(feature = "ssr", feature = "check", feature = "spinner", feature = "blank"))]

use leptos::prelude::*;
use prime_icons::{BlankIcon, CheckIcon, SpinnerIcon};

#[test]
fn decorative_icon_is_aria_hidden_without_role() {
    let html = view! { <CheckIcon /> }.to_html();
    assert!(html.contains(r#"aria-hidden="true""#), "{html}");
    assert!(!html.contains("role="), "{html}");
    assert!(html.contains(r#"viewBox="0 0 14 14""#), "{html}");
    assert!(html.contains(r#"class="p-icon""#), "{html}");
    assert!(html.contains(r#"width="14""#) && html.contains(r#"height="14""#), "{html}");
}

#[test]
fn labelled_icon_has_role_img_and_no_aria_hidden() {
    let html = view! { <CheckIcon aria_label="Готово" /> }.to_html();
    assert!(html.contains(r#"role="img""#), "{html}");
    assert!(html.contains(r#"aria-label="Готово""#), "{html}");
    assert!(!html.contains("aria-hidden"), "{html}");
}

#[test]
fn spin_size_and_class() {
    let html = view! { <SpinnerIcon spin=true size=20u32 class="custom" /> }.to_html();
    assert!(html.contains(r#"class="p-icon p-icon-spin custom""#), "{html}");
    assert!(html.contains(r#"width="20""#), "{html}");
}

#[test]
fn blank_icon_renders_transparent_rect() {
    let html = view! { <BlankIcon /> }.to_html();
    assert!(html.contains("<rect"), "{html}");
    assert!(html.contains(r#"fill-opacity="0""#), "{html}");
}
