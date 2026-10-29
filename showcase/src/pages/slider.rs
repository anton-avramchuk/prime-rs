//! Страница Slider: демонстрации `basic`, `range`, `step`, `vertical`, `filter`, `input` из showcase PrimeNG
//! (`apps/showcase/doc/slider`). Демонстрации с Angular Forms не переносятся; в `filter` вместо внешней картинки градиентный блок.

use super::Demo;
use leptos::prelude::*;
use prime_inputtext::InputText;
use prime_selectbutton::{SelectButton, SelectButtonOption};
use prime_slider::{Slider, SliderOrientation};

/// Страница `/slider`.
#[component]
pub fn SliderPage() -> impl IntoView {
    let basic = RwSignal::new(0.0);
    let range = RwSignal::new([20.0, 80.0]);
    let stepped = RwSignal::new(20.0);
    let vertical = RwSignal::new(50.0);
    let which = RwSignal::new(Some("0".to_owned()));
    let (contrast, brightness, sepia) = (RwSignal::new(100.0), RwSignal::new(100.0), RwSignal::new(0.0));
    let filter_options =
        vec![SelectButtonOption::new("Contrast", "0"), SelectButtonOption::new("Brightness", "1"), SelectButtonOption::new("Sepia", "2")];
    let filter_style = move || {
        format!(
            "width: 14rem; height: 8rem; border-radius: 6px; background: linear-gradient(135deg, #f59e0b, #ef4444 55%, #6366f1); \
             filter: contrast({}%) brightness({}%) sepia({}%)",
            contrast.get(),
            brightness.get(),
            sepia.get()
        )
    };
    let number = RwSignal::new(50.0);
    let text = RwSignal::new("50".to_owned());
    Effect::new(move |_| {
        let n = number.get();
        if text.get_untracked().parse::<f64>().ok() != Some(n) {
            text.set(n.to_string());
        }
    });
    Effect::new(move |_| {
        if let Ok(n) = text.get().parse::<f64>()
            && number.get_untracked() != n
        {
            number.set(n);
        }
    });
    view! {
        <h2>"Slider"</h2>
        <Demo name="basic" title="Basic">
            <Slider value=basic style="width: 14rem" aria_label="Basic" />
            <span data-testid="echo">{move || basic.get().to_string()}</span>
        </Demo>
        <Demo name="range" title="Range">
            <Slider range=true values=range style="width: 14rem" />
            <span data-testid="echo-range">{move || range.get().map(|v| v.to_string()).join(",")}</span>
        </Demo>
        <Demo name="step" title="Step">
            <Slider value=stepped step=20.0 style="width: 14rem" />
            <span data-testid="echo-step">{move || stepped.get().to_string()}</span>
        </Demo>
        <Demo name="vertical" title="Vertical">
            <Slider value=vertical orientation=SliderOrientation::Vertical style="height: 14rem" />
        </Demo>
        <Demo name="filter" title="Filter">
            <div style="display: flex; flex-direction: column; align-items: center; gap: 1rem">
                <div data-testid="filter-target" style={filter_style}></div>
                <SelectButton options=filter_options value=which allow_empty=false />
                {move || match which.get().as_deref() {
                    Some("1") => view! { <Slider value=brightness min=0.0 max=200.0 style="width: 14rem" /> }.into_any(),
                    Some("2") => view! { <Slider value=sepia min=0.0 max=200.0 style="width: 14rem" /> }.into_any(),
                    _ => view! { <Slider value=contrast min=0.0 max=200.0 style="width: 14rem" /> }.into_any(),
                }}
            </div>
        </Demo>
        <Demo name="input" title="Input">
            <div style="width: 14rem">
                <InputText value=text fluid=true />
                <div style="height: 1rem"></div>
                <Slider value=number />
            </div>
        </Demo>
    }
}
