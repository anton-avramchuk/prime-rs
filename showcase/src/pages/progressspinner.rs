//! ProgressSpinner page: the `basic` and `custom` demos from the PrimeNG showcase (`apps/showcase/doc/progressspinner`).

use super::Demo;
use leptos::prelude::*;
use prime_progressspinner::ProgressSpinner;

/// `/progressspinner` page.
#[component]
pub fn ProgressSpinnerPage() -> impl IntoView {
    view! {
        <Demo name="basic" title="Basic">
            <ProgressSpinner aria_label="loading" />
        </Demo>
        <Demo name="custom" title="Custom">
            <ProgressSpinner stroke_width="8" fill="transparent" animation_duration=".5s" style="width: 50px; height: 50px" />
        </Demo>
    }
}
