//! Страница ButtonGroup: демонстрации `basic` и `icons` по `apps/showcase/doc/buttongroup` PrimeNG.
//!
//! Нет иконок `bookmark`, `heart`, `time` в `prime-icons`, поэтому во втором примере взяты `plus`, `pencil`, `trash`.

use super::Demo;
use leptos::prelude::*;
use prime_button::Button;
use prime_buttongroup::ButtonGroup;
use prime_icons::{CheckIcon, PencilIcon, PlusIcon, TimesIcon, TrashIcon};

/// Страница `/buttongroup`.
#[component]
pub fn ButtonGroupPage() -> impl IntoView {
    view! {
        <h2>"ButtonGroup"</h2>
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
