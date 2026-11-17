//! FloatLabel page: the `basic`, `variants`, `invalid` demos from `apps/showcase/doc/floatlabel` in PrimeNG.
//!
//! Demos with Angular Forms are not ported; in `invalid` the state follows whether the field is empty.

use super::Demo;
use leptos::prelude::*;
use prime_floatlabel::{FloatLabel, FloatLabelVariant};
use prime_inputtext::InputText;

/// `/floatlabel` page.
#[component]
pub fn FloatLabelPage() -> impl IntoView {
    let invalid_value = RwSignal::new(String::new());
    view! {
        <Demo name="basic" title="Basic">
            <FloatLabel>
                <InputText id="username" autocomplete="off" />
                <label for="username">"Username"</label>
            </FloatLabel>
        </Demo>
        <Demo name="variants" title="Variants">
            <FloatLabel variant=FloatLabelVariant::In>
                <InputText id="in_label" autocomplete="off" />
                <label for="in_label">"In Label"</label>
            </FloatLabel>
            <FloatLabel variant=FloatLabelVariant::On>
                <InputText id="on_label" autocomplete="off" />
                <label for="on_label">"On Label"</label>
            </FloatLabel>
            <FloatLabel variant=FloatLabelVariant::Over>
                <InputText id="over_label" autocomplete="off" />
                <label for="over_label">"Over Label"</label>
            </FloatLabel>
        </Demo>
        <Demo name="invalid" title="Invalid">
            <FloatLabel>
                <InputText id="value" value=invalid_value invalid=Signal::derive(move || invalid_value.get().is_empty()) autocomplete="off" />
                <label for="value">"Value"</label>
            </FloatLabel>
        </Demo>
    }
}
