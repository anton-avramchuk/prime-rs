//! ButtonGroup page: the `basic` and `icons` demos from `apps/showcase/doc/buttongroup` in PrimeNG.
//!
//! There are no `bookmark`, `heart`, `time` icons in `prime-icons`, so the second example uses `plus`, `pencil`, `trash`.

use super::Demo;
use leptos::prelude::*;
use prime_button::Button;
use prime_buttongroup::ButtonGroup;
use prime_icons::{CheckIcon, PencilIcon, PlusIcon, TimesIcon, TrashIcon};

/// `/buttongroup` page.
#[component]
pub fn ButtonGroupPage() -> impl IntoView {
    view! {
        <Demo name="basic" title="Basic">
            <ButtonGroup>
                <Button label="Save" icon=ViewFn::from(|| view! { <CheckIcon /> }) />
                <Button label="Delete" icon=ViewFn::from(|| view! { <TrashIcon /> }) />
                <Button label="Cancel" icon=ViewFn::from(|| view! { <TimesIcon /> }) />
            </ButtonGroup>
        </Demo>
        <Demo name="icons" title="Icons">
            <ButtonGroup>
                <Button icon=ViewFn::from(|| view! { <PlusIcon /> }) aria_label="Add" />
                <Button icon=ViewFn::from(|| view! { <PencilIcon /> }) aria_label="Edit" />
                <Button icon=ViewFn::from(|| view! { <TrashIcon /> }) aria_label="Delete" />
            </ButtonGroup>
        </Demo>
    }
}
