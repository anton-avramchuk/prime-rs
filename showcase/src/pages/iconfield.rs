//! Страница IconField: демонстрации `basic`, `sizes`, `template`, `floatlabel`, `iftalabel` по `apps/showcase/doc/iconfield` PrimeNG.
//!
//! В `template` вместо логотипа PrimeVue — простой SVG; значки `user` и `lock` в `prime-icons` отсутствуют, взяты `search`, `eye`, `check`.

use super::Demo;
use leptos::prelude::*;
use prime_core::Size;
use prime_floatlabel::FloatLabel;
use prime_iconfield::{IconField, IconPosition, InputIcon};
use prime_icons::{CheckIcon, EyeIcon, SearchIcon, SpinnerIcon};
use prime_iftalabel::IftaLabel;
use prime_inputtext::InputText;

/// Страница `/iconfield`.
#[component]
pub fn IconFieldPage() -> impl IntoView {
    view! {
        <h2>"IconField"</h2>
        <Demo name="basic" title="Basic">
            <IconField>
                <InputIcon>
                    <SearchIcon />
                </InputIcon>
                <InputText placeholder="Search" />
            </IconField>
            <IconField icon_position=IconPosition::Right>
                <InputText placeholder="Search" />
                <InputIcon>
                    <SpinnerIcon />
                </InputIcon>
            </IconField>
        </Demo>
        <Demo name="sizes" title="Sizes">
            <IconField>
                <InputIcon>
                    <SearchIcon />
                </InputIcon>
                <InputText size=Size::Small placeholder="Small" />
            </IconField>
            <IconField>
                <InputIcon>
                    <SearchIcon />
                </InputIcon>
                <InputText placeholder="Normal" />
            </IconField>
            <IconField>
                <InputIcon>
                    <SearchIcon />
                </InputIcon>
                <InputText size=Size::Large placeholder="Large" />
            </IconField>
        </Demo>
        <Demo name="template" title="Template">
            <IconField>
                <InputIcon>
                    <svg data-testid="custom-icon" width="14" height="14" viewBox="0 0 14 14" fill="none" xmlns="http://www.w3.org/2000/svg">
                        <circle cx="7" cy="7" r="6" stroke="currentColor" stroke-width="1.5" />
                        <path d="M4 7.5L6.2 9.5L10 5" stroke="currentColor" stroke-width="1.5" />
                    </svg>
                </InputIcon>
                <InputText placeholder="Custom icon" />
            </IconField>
            <IconField icon_position=IconPosition::Right>
                <InputText placeholder="Password" />
                <InputIcon>
                    <EyeIcon />
                </InputIcon>
            </IconField>
        </Demo>
        <Demo name="floatlabel" title="Float Label">
            <FloatLabel>
                <IconField>
                    <InputIcon>
                        <SearchIcon />
                    </InputIcon>
                    <InputText id="if_search" autocomplete="off" />
                </IconField>
                <label for="if_search">"Search"</label>
            </FloatLabel>
            <FloatLabel>
                <IconField icon_position=IconPosition::Right>
                    <InputText id="if_done" autocomplete="off" />
                    <InputIcon>
                        <CheckIcon />
                    </InputIcon>
                </IconField>
                <label for="if_done">"Done"</label>
            </FloatLabel>
        </Demo>
        <Demo name="iftalabel" title="Ifta Label">
            <IftaLabel>
                <IconField>
                    <InputIcon>
                        <SearchIcon />
                    </InputIcon>
                    <InputText id="ifta_search" autocomplete="off" />
                </IconField>
                <label for="ifta_search">"Search"</label>
            </IftaLabel>
        </Demo>
    }
}
