//! Страница ToggleSwitch: демонстрации `basic`, `preselection`, `template`, `invalid`, `disabled` из showcase PrimeNG
//! (`apps/showcase/doc/toggleswitch`).

use super::Demo;
use leptos::prelude::*;
use prime_icons::{CheckIcon, TimesIcon};
use prime_toggleswitch::ToggleSwitch;

/// Страница `/toggleswitch`.
#[component]
pub fn ToggleSwitchPage() -> impl IntoView {
    let basic = RwSignal::new(false);
    let preselected = RwSignal::new(true);
    let templated = RwSignal::new(false);
    let handle =
        Callback::new(
            |checked: bool| {
                if checked { view! { <CheckIcon size=10u32 /> }.into_any() } else { view! { <TimesIcon size=10u32 /> }.into_any() }
            },
        );
    view! {
        <h2>"ToggleSwitch"</h2>
        <Demo name="basic" title="Basic">
            <ToggleSwitch checked=basic />
            <span data-testid="echo">{move || basic.get().to_string()}</span>
        </Demo>
        <Demo name="preselection" title="Preselection">
            <ToggleSwitch checked=preselected />
        </Demo>
        <Demo name="template" title="Template">
            <ToggleSwitch checked=templated handle_template=handle />
        </Demo>
        <Demo name="invalid" title="Invalid">
            <ToggleSwitch invalid=true />
        </Demo>
        <Demo name="disabled" title="Disabled">
            <ToggleSwitch disabled=true />
        </Demo>
    }
}
