//! Страница IftaLabel: демонстрации `basic`, `invalid` по `apps/showcase/doc/iftalabel` PrimeNG.
//!
//! Демонстрации с Angular Forms не переносятся; в `invalid` состояние следует за пустотой поля.

use super::Demo;
use leptos::prelude::*;
use prime_iftalabel::IftaLabel;
use prime_inputtext::InputText;

/// Страница `/iftalabel`.
#[component]
pub fn IftaLabelPage() -> impl IntoView {
    let invalid_value = RwSignal::new(String::new());
    view! {
        <h2>"IftaLabel"</h2>
        <Demo name="basic" title="Basic">
            <IftaLabel>
                <InputText id="username" autocomplete="off" />
                <label for="username">"Username"</label>
            </IftaLabel>
        </Demo>
        <Demo name="invalid" title="Invalid">
            <IftaLabel>
                <InputText id="value" value=invalid_value invalid=Signal::derive(move || invalid_value.get().is_empty()) autocomplete="off" />
                <label for="value">"Value"</label>
            </IftaLabel>
        </Demo>
    }
}
