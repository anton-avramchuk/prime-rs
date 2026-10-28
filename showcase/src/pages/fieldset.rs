//! Страница Fieldset: демонстрации `basic`, `toggleable` и `template` из showcase PrimeNG (`apps/showcase/doc/fieldset`).

use super::Demo;
use leptos::prelude::*;
use prime_avatar::{Avatar, AvatarShape};
use prime_fieldset::Fieldset;

const LOREM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

/// Страница `/fieldset`.
#[component]
pub fn FieldsetPage() -> impl IntoView {
    let collapsed = RwSignal::new(false);
    view! {
        <h2>"Fieldset"</h2>
        <Demo name="basic" title="Basic">
            <Fieldset legend="Header">
                <p style="margin: 0">{LOREM}</p>
            </Fieldset>
        </Demo>
        <Demo name="toggleable" title="Toggleable">
            <Fieldset legend="Header" toggleable=true collapsed=collapsed>
                <p style="margin: 0">{LOREM}</p>
            </Fieldset>
            <p data-testid="collapsed-state" style="width: 100%; text-align: center; margin: 0">
                {move || if collapsed.get() { "collapsed" } else { "expanded" }}
            </p>
        </Demo>
        <Demo name="template" title="Template">
            <Fieldset header_template=ViewFn::from(|| {
                view! {
                    <div style="display: flex; align-items: center; gap: 0.5rem; padding: 0 0.5rem">
                        <Avatar image="https://primefaces.org/cdn/primeng/images/demo/avatar/amyelsner.png" shape=AvatarShape::Circle />
                        <span style="font-weight: bold">"Amy Elsner"</span>
                    </div>
                }
            })>
                <p style="margin: 0">{LOREM}</p>
            </Fieldset>
        </Demo>
    }
}
