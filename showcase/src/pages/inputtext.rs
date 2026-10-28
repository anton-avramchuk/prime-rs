//! Страница InputText: демонстрации `basic`, `filled`, `sizes`, `invalid`, `disabled`, `fluid` из showcase PrimeNG (`apps/showcase/doc/inputtext`).

use super::Demo;
use leptos::prelude::*;
use prime_core::{Size, Variant};
use prime_inputtext::InputText;

/// Страница `/inputtext`.
#[component]
pub fn InputTextPage() -> impl IntoView {
    let value = RwSignal::new(String::new());
    view! {
        <h2>"InputText"</h2>
        <Demo name="basic" title="Basic">
            <InputText value=value placeholder="Введите текст" />
            <span data-testid="echo">{move || value.get()}</span>
        </Demo>
        <Demo name="filled" title="Filled">
            <InputText variant=Variant::Filled placeholder="Filled" />
        </Demo>
        <Demo name="sizes" title="Sizes">
            <InputText size=Size::Small placeholder="Small" />
            <InputText placeholder="Normal" />
            <InputText size=Size::Large placeholder="Large" />
        </Demo>
        <Demo name="invalid" title="Invalid">
            <InputText invalid=true placeholder="Invalid" />
        </Demo>
        <Demo name="disabled" title="Disabled">
            <InputText disabled=true placeholder="Disabled" />
        </Demo>
        <Demo name="fluid" title="Fluid">
            <InputText fluid=true placeholder="Fluid" />
        </Demo>
    }
}
