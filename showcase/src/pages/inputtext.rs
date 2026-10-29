//! Страница InputText: демонстрации `basic`, `filled`, `sizes`, `invalid`, `disabled`, `fluid`, `floatlabel`, `iftalabel` из showcase PrimeNG (`apps/showcase/doc/inputtext`).

use super::Demo;
use leptos::prelude::*;
use prime_core::{Size, Variant};
use prime_floatlabel::FloatLabel;
use prime_iftalabel::IftaLabel;
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
        <Demo name="floatlabel" title="Float Label">
            <FloatLabel>
                <InputText id="float_username" autocomplete="off" />
                <label for="float_username">"Username"</label>
            </FloatLabel>
        </Demo>
        <Demo name="iftalabel" title="Ifta Label">
            <IftaLabel>
                <InputText id="ifta_username" autocomplete="off" />
                <label for="ifta_username">"Username"</label>
            </IftaLabel>
        </Demo>
    }
}
