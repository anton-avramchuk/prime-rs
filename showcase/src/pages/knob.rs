//! Страница Knob: демонстрации `basic`, `minmax`, `reactive`, `size`, `step`, `stroke`, `template`, `color`, `readonly`, `disabled`
//! из showcase PrimeNG (`apps/showcase/doc/knob`). Демонстрации с Angular Forms не переносятся.

use super::Demo;
use leptos::prelude::*;
use prime_button::Button;
use prime_icons::{MinusIcon, PlusIcon};
use prime_knob::Knob;

/// Страница `/knob`.
#[component]
pub fn KnobPage() -> impl IntoView {
    let basic = RwSignal::new(0.0);
    let minmax = RwSignal::new(10.0);
    let controlled = RwSignal::new(0.0_f64);
    let sized = RwSignal::new(60.0);
    let stepped = RwSignal::new(0.0);
    let stroked = RwSignal::new(40.0);
    let templated = RwSignal::new(60.0);
    let colored = RwSignal::new(50.0);
    let readonly = RwSignal::new(50.0);
    let disabled = RwSignal::new(75.0);
    let plus = Callback::new(move |_| controlled.update(|v| *v = (*v + 1.0).min(100.0)));
    let minus = Callback::new(move |_| controlled.update(|v| *v = (*v - 1.0).max(0.0)));
    let at_max = Signal::derive(move || controlled.get() >= 100.0);
    let at_min = Signal::derive(move || controlled.get() <= 0.0);
    view! {
        <h2>"Knob"</h2>
        <Demo name="basic" title="Basic">
            <Knob value=basic aria_label="Basic" />
            <span data-testid="echo">{move || basic.get().to_string()}</span>
        </Demo>
        <Demo name="minmax" title="Min / Max">
            <Knob value=minmax min=-50.0 max=50.0 />
        </Demo>
        <Demo name="reactive" title="Reactive">
            <div style="display: flex; flex-direction: column; align-items: center; gap: 0.5rem">
                <Knob value=controlled size=150 readonly=true />
                <div style="display: flex; gap: 0.5rem">
                    <Button icon=ViewFn::from(|| view! { <PlusIcon /> }) on_click=plus disabled=at_max aria_label="Plus" />
                    <Button icon=ViewFn::from(|| view! { <MinusIcon /> }) on_click=minus disabled=at_min aria_label="Minus" />
                </div>
            </div>
        </Demo>
        <Demo name="size" title="Size">
            <Knob value=sized size=200 />
        </Demo>
        <Demo name="step" title="Step">
            <Knob value=stepped step=10.0 />
        </Demo>
        <Demo name="stroke" title="Stroke">
            <Knob value=stroked stroke_width=5.0 />
        </Demo>
        <Demo name="template" title="Template">
            <Knob value=templated value_template="{value}%" />
        </Demo>
        <Demo name="color" title="Color">
            <Knob value=colored value_color="SlateGray" range_color="MediumTurquoise" />
        </Demo>
        <Demo name="readonly" title="Readonly">
            <Knob value=readonly readonly=true />
        </Demo>
        <Demo name="disabled" title="Disabled">
            <Knob value=disabled disabled=true />
        </Demo>
    }
}
