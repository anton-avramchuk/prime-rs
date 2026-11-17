//! IftaLabel page: the `basic`, `invalid` demos from `apps/showcase/doc/iftalabel` in PrimeNG.
//!
//! Demos with Angular Forms are not ported; in `invalid` the state follows whether the field is empty.

use super::Demo;
use leptos::prelude::*;
use prime_iftalabel::IftaLabel;
use prime_inputtext::InputText;

/// `/iftalabel` page.
#[component]
pub fn IftaLabelPage() -> impl IntoView {
    let invalid_value = RwSignal::new(String::new());
    view! {
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
