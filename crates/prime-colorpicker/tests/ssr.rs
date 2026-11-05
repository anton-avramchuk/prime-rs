//! Разметка ColorPicker (SSR): образец, закрытая и встроенная панель, положение указателей, формат, блокировка, `pt`.
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_colorpicker::color::Hsb;
use prime_colorpicker::{ColorFormat, ColorPicker, ColorPickerPt, ColorValue};
use prime_core::Pt;

fn render(f: impl FnOnce() -> String) -> String {
    Owner::new().with(f)
}

#[test]
fn closed_picker_has_preview_and_no_panel() {
    let html = render(|| view! { <ColorPicker input_id="cp" tabindex="2" /> }.to_html());
    assert!(html.contains("p-colorpicker p-component p-colorpicker-overlay"), "{html}");
    assert!(html.contains("class=\"p-colorpicker-preview\"") && html.contains("readonly") && html.contains("id=\"cp\""), "{html}");
    assert!(html.contains("aria-label=\"Select Color\"") && html.contains("tabindex=\"2\""), "{html}");
    assert!(html.contains("background-color:#ff0000"), "по умолчанию красный: {html}");
    assert!(!html.contains("p-colorpicker-panel"), "{html}");
}

#[test]
fn inline_picker_has_panel_and_no_preview() {
    let html = render(|| view! { <ColorPicker inline=true /> }.to_html());
    assert!(!html.contains("p-colorpicker-overlay") && !html.contains("p-colorpicker-preview"), "{html}");
    assert!(html.contains("p-colorpicker-panel p-colorpicker-panel-inline"), "{html}");
    for part in [
        "p-colorpicker-content",
        "p-colorpicker-color-selector",
        "p-colorpicker-color-background",
        "p-colorpicker-color-handle",
        "p-colorpicker-hue",
        "p-colorpicker-hue-handle",
    ] {
        assert!(html.contains(part), "нет {part}: {html}");
    }
}

#[test]
fn handles_follow_the_value() {
    let html = render(|| {
        let value = RwSignal::new(Some(ColorValue::Hex("#00ff00".into())));
        view! { <ColorPicker inline=true value=value /> }.to_html()
    });
    // зелёный: h = 120, s = 100 (с точностью до округления), b = 100
    assert!(html.contains("top: 0px") && html.contains("top: 100px"), "указатель шкалы на 120° → 100px: {html}");
    assert!(html.contains("background-color:#00ff00"), "фон поля — чистый оттенок: {html}");
}

#[test]
fn value_in_every_format_is_accepted() {
    let html = render(|| {
        let value = RwSignal::new(Some(ColorValue::Hsb(Hsb { h: 0.0, s: 0.0, b: 0.0 })));
        view! { <ColorPicker format=ColorFormat::Hsb value=value /> }.to_html()
    });
    assert!(html.contains("background-color:#000000"), "{html}");
    let html = render(|| {
        let value = RwSignal::new(Some(ColorValue::Rgb(prime_colorpicker::color::Rgb { r: 0, g: 0, b: 255 })));
        view! { <ColorPicker format=ColorFormat::Rgb value=value /> }.to_html()
    });
    assert!(html.contains("background-color:#0000ff"), "{html}");
}

#[test]
fn default_color_is_used_without_value() {
    let html = render(|| view! { <ColorPicker default_color="0000ff" /> }.to_html());
    assert!(html.contains("background-color:#0000ff"), "{html}");
}

#[test]
fn disabled_marks_preview_and_panel() {
    let html = render(|| view! { <ColorPicker disabled=true /> }.to_html());
    assert!(html.contains("p-colorpicker-preview p-disabled") && html.contains("disabled"), "{html}");
    let html = render(|| view! { <ColorPicker disabled=true inline=true /> }.to_html());
    assert!(html.contains("p-colorpicker-panel p-colorpicker-panel-inline p-disabled"), "{html}");
}

#[test]
fn style_and_pass_through() {
    let pt = ColorPickerPt { root: Pt::new().class("root-x"), preview: Pt::new().class("preview-x"), ..Default::default() };
    let html = render(|| view! { <ColorPicker id="r" style="width: 3rem" style_class="mine" pt=pt /> }.to_html());
    assert!(html.contains("root-x") && html.contains("mine") && html.contains("id=\"r\"") && html.contains("width: 3rem"), "{html}");
    assert!(html.contains("p-colorpicker-preview preview-x"), "{html}");
}
