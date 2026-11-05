//! Страница ColorPicker: демонстрации `basic`, `inline`, `format` и `disabled` из showcase PrimeNG
//! (`apps/showcase/doc/colorpicker`; формы Angular не переносятся).

use super::Demo;
use leptos::prelude::*;
use prime_colorpicker::{ColorFormat, ColorPicker, ColorValue};

fn hex_text(value: Option<ColorValue>) -> String {
    match value {
        Some(ColorValue::Hex(h)) => h,
        _ => String::new(),
    }
}

#[component]
fn Basic() -> impl IntoView {
    let value = RwSignal::new(None);
    view! {
        <div style="display: flex; flex-direction: column; align-items: center; gap: 0.5rem">
            <ColorPicker id="basic_cp" value=value />
            <span data-testid="basic-echo">{move || hex_text(value.get())}</span>
        </div>
    }
}

#[component]
fn Inline() -> impl IntoView {
    let value = RwSignal::new(None);
    view! {
        <div style="display: flex; flex-direction: column; align-items: center; gap: 0.5rem">
            <ColorPicker id="inline_cp" inline=true value=value />
            <span data-testid="inline-echo">{move || hex_text(value.get())}</span>
        </div>
    }
}

#[component]
fn Formats() -> impl IntoView {
    let hex = RwSignal::new(None);
    let rgb = RwSignal::new(None);
    let hsb = RwSignal::new(None);
    view! {
        <div style="display: flex; flex-wrap: wrap; gap: 2rem; justify-content: center">
            <div style="display: flex; flex-direction: column; align-items: center; gap: 0.5rem">
                <label for="cp-hex" style="font-weight: bold">"HEX"</label>
                <ColorPicker input_id="cp-hex" value=hex />
                <span data-testid="hex-echo">{move || hex_text(hex.get())}</span>
            </div>
            <div style="display: flex; flex-direction: column; align-items: center; gap: 0.5rem">
                <label for="cp-rgb" style="font-weight: bold">"RGB"</label>
                <ColorPicker input_id="cp-rgb" format=ColorFormat::Rgb value=rgb />
                <span data-testid="rgb-echo">
                    {move || match rgb.get() {
                        Some(ColorValue::Rgb(c)) => format!("r:{} g:{} b:{}", c.r, c.g, c.b),
                        _ => String::new(),
                    }}
                </span>
            </div>
            <div style="display: flex; flex-direction: column; align-items: center; gap: 0.5rem">
                <label for="cp-hsb" style="font-weight: bold">"HSB"</label>
                <ColorPicker input_id="cp-hsb" format=ColorFormat::Hsb value=hsb />
                <span data-testid="hsb-echo">
                    {move || match hsb.get() {
                        Some(ColorValue::Hsb(c)) => format!("h:{} s:{} b:{}", c.h, c.s, c.b),
                        _ => String::new(),
                    }}
                </span>
            </div>
        </div>
    }
}

/// Страница `/colorpicker`.
#[component]
pub fn ColorPickerPage() -> impl IntoView {
    view! {
        <h2>"ColorPicker"</h2>
        <Demo name="basic" title="Basic">
            <Basic />
        </Demo>
        <Demo name="inline" title="Inline">
            <Inline />
        </Demo>
        <Demo name="format" title="Format">
            <Formats />
        </Demo>
        <Demo name="disabled" title="Disabled">
            <ColorPicker id="disabled_cp" disabled=true />
        </Demo>
    }
}
