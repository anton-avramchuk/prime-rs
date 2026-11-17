//! ToggleButton page: demos of `basic`, `customized`, `disabled`, `fluid`, `invalid`, `sizes` from the PrimeNG showcase
//! (`apps/showcase/doc/togglebutton`). Demos with Angular Forms are not ported; instead of `pi-lock` and `pi-lock-open` the icons
//! `CheckIcon` and `TimesIcon` are used (`prime-icons` has no locks).

use super::Demo;
use leptos::prelude::*;
use prime_icons::{CheckIcon, TimesIcon};
use prime_rs_core::Size;
use prime_togglebutton::ToggleButton;

fn on_icon() -> ViewFn {
    ViewFn::from(|| view! { <CheckIcon /> })
}

fn off_icon() -> ViewFn {
    ViewFn::from(|| view! { <TimesIcon /> })
}

/// The `/togglebutton` page.
#[component]
pub fn ToggleButtonPage() -> impl IntoView {
    let basic = RwSignal::new(false);
    let customized = RwSignal::new(false);
    let invalid = RwSignal::new(false);
    let not_checked = Signal::derive(move || !invalid.get());
    view! {
        <Demo name="basic" title="Basic">
            <ToggleButton checked=basic on_label="On" off_label="Off" style="width: 6rem" />
            <span data-testid="echo">{move || basic.get().to_string()}</span>
        </Demo>
        <Demo name="customized" title="Customized">
            <ToggleButton
                checked=customized
                on_label="Locked"
                off_label="Unlocked"
                on_icon=on_icon()
                off_icon=off_icon()
                aria_label="Do you confirm"
                style="width: 9rem"
            />
        </Demo>
        <Demo name="disabled" title="Disabled">
            <ToggleButton
                disabled=true
                on_icon=on_icon()
                off_icon=off_icon()
                on_label="Yes"
                off_label="No"
                aria_label="Confirmation"
                style="width: 10rem"
            />
        </Demo>
        <Demo name="fluid" title="Fluid">
            <ToggleButton on_label="On" off_label="Off" fluid=true />
        </Demo>
        <Demo name="invalid" title="Invalid">
            <ToggleButton checked=invalid on_icon=on_icon() off_icon=off_icon() invalid=not_checked aria_label="Confirmation" style="width: 10rem" />
        </Demo>
        <Demo name="sizes" title="Sizes">
            <div style="display: flex; flex-direction: column; align-items: center; gap: 1rem">
                <ToggleButton on_label="On" off_label="Off" size=Size::Small style="min-width: 4rem" />
                <ToggleButton on_label="On" off_label="Off" style="min-width: 5rem" />
                <ToggleButton on_label="On" off_label="Off" size=Size::Large style="min-width: 6rem" />
            </div>
        </Demo>
    }
}
