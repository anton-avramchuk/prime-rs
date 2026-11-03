//! Разметка Image (SSR): корень, `img`, кнопка просмотра, представления, `pt`. Просмотр на сервере не выводится.
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_core::Pt;
use prime_image::{Image, ImageLoading, ImagePt};

fn render(f: impl FnOnce() -> String) -> String {
    Owner::new().with(f)
}

#[test]
fn plain_image_has_root_and_img_only() {
    let html = render(|| view! { <Image src="a.jpg" alt="Photo" /> }.to_html());
    assert!(html.contains("class=\"p-image p-component\""), "{html}");
    assert!(html.contains("<img") && html.contains("src=\"a.jpg\"") && html.contains("alt=\"Photo\""), "{html}");
    assert!(!html.contains("p-image-preview") && !html.contains("<button"), "{html}");
}

#[test]
fn img_attributes_are_forwarded_and_absent_ones_omitted() {
    let html = render(|| {
        view! {
            <Image
                src="a.jpg"
                srcset="a.jpg 1x, b.jpg 2x"
                sizes="100vw"
                alt="x"
                width="250"
                height="100"
                loading=ImageLoading::Lazy
                image_class="pic"
                image_style="border: 0"
            />
        }
        .to_html()
    });
    for needle in [
        "srcset=\"a.jpg 1x, b.jpg 2x\"",
        "sizes=\"100vw\"",
        "width=\"250\"",
        "height=\"100\"",
        "loading=\"lazy\"",
        "class=\"pic\"",
        "border: 0",
    ] {
        assert!(html.contains(needle), "нет {needle}: {html}");
    }
    let bare = render(|| view! { <Image src="a.jpg" /> }.to_html());
    for absent in ["srcset", "sizes", "alt=", "width", "height", "loading"] {
        assert!(!bare.contains(absent), "лишний {absent}: {bare}");
    }
    let eager = render(|| view! { <Image src="a.jpg" loading=ImageLoading::Eager /> }.to_html());
    assert!(eager.contains("loading=\"eager\""), "{eager}");
}

#[test]
fn preview_adds_class_and_button_with_icon_and_size() {
    let html = render(|| view! { <Image src="a.jpg" width="250" height="100" preview=true /> }.to_html());
    assert!(html.contains("p-image p-component p-image-preview"), "{html}");
    assert!(html.contains("class=\"p-image-preview-mask\"") && html.contains("type=\"button\""), "{html}");
    assert!(html.contains("aria-label=\"Zoom Image\""), "{html}");
    assert!(html.contains("height: 100px; width: 250px"), "размер кнопки: {html}");
    assert!(html.contains("p-image-preview-icon") && html.contains("<svg"), "{html}");
    assert!(html.find("<img").unwrap() < html.find("<button").unwrap(), "кнопка после картинки: {html}");
    assert!(!html.contains("p-image-mask p-overlay-mask") && !html.contains("role=\"dialog\""), "просмотр закрыт: {html}");
}

#[test]
fn indicator_replaces_the_eye() {
    let html = render(|| {
        view! { <Image src="a.jpg" preview=true indicator=ViewFn::from(|| view! { <i class="custom-ind">"?"</i> }) /> }.to_html()
    });
    assert!(html.contains("custom-ind") && !html.contains("p-image-preview-icon"), "{html}");
}

#[test]
fn image_template_replaces_img_and_receives_error_callback() {
    let html = render(|| {
        view! {
            <Image
                src="a.jpg"
                image=Callback::new(|_err: Callback<web_sys::Event>| view! { <picture class="tpl">"pic"</picture> }.into_any())
            />
        }
        .to_html()
    });
    assert!(html.contains("class=\"tpl\"") && !html.contains("<img"), "{html}");
}

#[test]
fn style_class_style_and_pt() {
    let pt = ImagePt {
        root: Pt::new().class("r").attr("data-x", "1"),
        image: Pt::new().class("i"),
        preview_mask: Pt::new().class("m"),
        preview_icon: Pt::new().class("ico"),
        ..ImagePt::default()
    };
    let html = render(|| view! { <Image src="a.jpg" preview=true style_class="mine" style="margin: 1px" pt=pt /> }.to_html());
    assert!(html.contains("p-image p-component p-image-preview mine r") && html.contains("data-x=\"1\""), "{html}");
    assert!(html.contains("margin: 1px"), "{html}");
    assert!(html.contains("class=\"i\"") && html.contains("p-image-preview-mask m") && html.contains("p-image-preview-icon ico"), "{html}");
}

#[test]
fn reactive_props_follow_signals() {
    let html = render(|| {
        let src = RwSignal::new(Some("one.jpg".to_owned()));
        let preview = RwSignal::new(false);
        let out = view! { <Image src=src preview=preview /> }.to_html();
        assert!(out.contains("one.jpg") && !out.contains("<button"), "{out}");
        out
    });
    assert!(html.contains("p-image"), "{html}");
}
