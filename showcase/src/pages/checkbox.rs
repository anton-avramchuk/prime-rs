//! Страница Checkbox: демонстрации `basic`, `multiple`, `filled`, `sizes`, `indeterminate`, `invalid`, `disabled` из showcase PrimeNG
//! (`apps/showcase/doc/checkbox`).

use super::Demo;
use leptos::prelude::*;
use prime_checkbox::Checkbox;
use prime_core::{Size, Variant};

/// Страница `/checkbox`.
#[component]
pub fn CheckboxPage() -> impl IntoView {
    let checked = RwSignal::new(false);
    let cities = RwSignal::new(Vec::<String>::new());
    let indeterminate = RwSignal::new(true);
    let partial = RwSignal::new(false);
    let city = move |value: &'static str, id: &'static str| {
        view! {
            <div style="display: flex; align-items: center; gap: 0.5rem">
                <Checkbox input_id=id name="city" value=value model=cities />
                <label for=id>{value}</label>
            </div>
        }
    };
    view! {
        <h2>"Checkbox"</h2>
        <Demo name="basic" title="Basic">
            <Checkbox binary=true checked=checked />
            <span data-testid="echo">{move || checked.get().to_string()}</span>
        </Demo>
        <Demo name="multiple" title="Multiple">
            <div style="display: flex; gap: 1rem">
                {city("Chicago", "ny1")} {city("Los Angeles", "ny2")} {city("New York", "ny3")} {city("San Francisco", "ny4")}
            </div>
            <span data-testid="selected">{move || cities.get().join(",")}</span>
        </Demo>
        <Demo name="filled" title="Filled">
            <Checkbox binary=true variant=Variant::Filled default_checked=true />
        </Demo>
        <Demo name="sizes" title="Sizes">
            <Checkbox binary=true size=Size::Small default_checked=true />
            <Checkbox binary=true default_checked=true />
            <Checkbox binary=true size=Size::Large default_checked=true />
        </Demo>
        <Demo name="indeterminate" title="Indeterminate">
            <Checkbox binary=true checked=partial indeterminate=indeterminate />
        </Demo>
        <Demo name="invalid" title="Invalid">
            <Checkbox binary=true invalid=true />
        </Demo>
        <Demo name="disabled" title="Disabled">
            <Checkbox binary=true disabled=true />
            <Checkbox binary=true disabled=true default_checked=true />
        </Demo>
    }
}
