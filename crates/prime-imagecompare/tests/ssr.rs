//! Разметка ImageCompare (SSR): состав и порядок, атрибуты ползунка, доступность, `style`/`pt`.
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_core::Pt;
use prime_imagecompare::{ImageCompare, ImageComparePt};

fn render(f: impl FnOnce() -> String) -> String {
    Owner::new().with(f)
}

fn two_images() -> (ViewFn, ViewFn) {
    (ViewFn::from(|| view! { <img src="left.jpg" /> }), ViewFn::from(|| view! { <img src="right.jpg" /> }))
}

#[test]
fn markup_has_both_views_and_a_range_slider_in_order() {
    let (left, right) = two_images();
    let html = render(|| view! { <ImageCompare left=left right=right /> }.to_html());
    assert!(html.contains("class=\"p-imagecompare\""), "{html}");
    let (l, r, s) = (html.find("left.jpg").unwrap(), html.find("right.jpg").unwrap(), html.find("p-imagecompare-slider").unwrap());
    assert!(l < r && r < s, "порядок: левый, правый, ползунок: {html}");
    assert!(
        html.contains("type=\"range\"") && html.contains("min=\"0\"") && html.contains("max=\"100\"") && html.contains("value=\"50\""),
        "{html}"
    );
    // представления без обёрток: оба <img> — непосредственные дети корня
    let inner = html.split_once("p-imagecompare\">").unwrap().1;
    assert!(inner.starts_with("<img src=\"left.jpg\"><img src=\"right.jpg\"><input"), "{html}");
}

#[test]
fn accessibility_attributes_are_on_the_root() {
    let (left, right) = two_images();
    let html = render(|| view! { <ImageCompare left=left right=right tabindex=0 aria_label="Compare" aria_labelledby="lbl" /> }.to_html());
    assert!(
        html.contains("tabindex=\"0\"") && html.contains("aria-label=\"Compare\"") && html.contains("aria-labelledby=\"lbl\""),
        "{html}"
    );
    let (left, right) = two_images();
    let plain = render(|| view! { <ImageCompare left=left right=right /> }.to_html());
    assert!(!plain.contains("tabindex") && !plain.contains("aria-label"), "{plain}");
}

#[test]
fn style_class_and_pt() {
    let (left, right) = two_images();
    let pt = ImageComparePt { root: Pt::new().class("r").attr("data-x", "1"), slider: Pt::new().class("s") };
    let html = render(|| view! { <ImageCompare left=left right=right style="width: 20rem" style_class="mine" pt=pt /> }.to_html());
    assert!(html.contains("p-imagecompare mine r") && html.contains("data-x=\"1\"") && html.contains("width: 20rem"), "{html}");
    assert!(html.contains("p-imagecompare-slider s"), "{html}");
}

#[test]
fn no_clip_path_is_rendered_before_the_first_input() {
    let (left, right) = two_images();
    let html = render(|| view! { <ImageCompare left=left right=right /> }.to_html());
    assert!(!html.contains("clip-path"), "до ввода обрезку задаёт только тема: {html}");
}
