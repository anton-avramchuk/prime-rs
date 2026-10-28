//! Страница Panel: демонстрации `basic`, `toggleable` и `template` из showcase PrimeNG (`apps/showcase/doc/panel`)
//! и `header-toggler` (`toggler = "header"`, описан в тексте `toggleable-doc`).
//!
//! Меню шестерёнки из `template` не переносится (Menu ещё не портирован): кнопка остаётся без выпадающего списка;
//! вместо шрифта PrimeIcons используются SVG-иконки `prime-icons`.

use super::Demo;
use leptos::prelude::*;
use prime_avatar::{Avatar, AvatarShape};
use prime_button::{Button, ButtonSeverity};
use prime_icons::{PencilIcon, RefreshIcon, StarIcon};
use prime_panel::{Panel, Toggler};

const LOREM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

/// Страница `/panel`.
#[component]
pub fn PanelPage() -> impl IntoView {
    let collapsed = RwSignal::new(false);
    view! {
        <h2>"Panel"</h2>
        <Demo name="basic" title="Basic">
            <Panel header="Header">
                <p style="margin: 0">{LOREM}</p>
            </Panel>
        </Demo>
        <Demo name="toggleable" title="Toggleable">
            <Panel header="Header" toggleable=true collapsed=collapsed>
                <p style="margin: 0">{LOREM}</p>
            </Panel>
            <p data-testid="collapsed-state" style="width: 100%; text-align: center; margin: 0">
                {move || if collapsed.get() { "collapsed" } else { "expanded" }}
            </p>
        </Demo>
        <Demo name="header-toggler" title="Header toggler">
            <Panel header="Header" toggleable=true toggler=Toggler::Header default_collapsed=true>
                <p style="margin: 0">{LOREM}</p>
            </Panel>
        </Demo>
        <Demo name="template" title="Template">
            <Panel
                toggleable=true
                header_template=ViewFn::from(|| {
                    view! {
                        <div style="display: flex; align-items: center; gap: 0.5rem">
                            <Avatar image="https://primefaces.org/cdn/primevue/images/avatar/amyelsner.png" shape=AvatarShape::Circle />
                            <span style="font-weight: bold">"Amy Elsner"</span>
                        </div>
                    }
                })
                icons_template=ViewFn::from(|| {
                    view! {
                        <Button
                            icon=ViewFn::from(|| view! { <RefreshIcon /> })
                            severity=ButtonSeverity::Secondary
                            rounded=true
                            text=true
                            aria_label="Refresh"
                        />
                    }
                })
                footer_template=ViewFn::from(|| {
                    view! {
                        <div style="display: flex; flex-wrap: wrap; align-items: center; justify-content: space-between; gap: 1rem">
                            <div style="display: flex; align-items: center; gap: 0.5rem">
                                <Button icon=ViewFn::from(|| view! { <PencilIcon /> }) rounded=true text=true aria_label="Edit" />
                                <Button
                                    icon=ViewFn::from(|| view! { <StarIcon /> })
                                    severity=ButtonSeverity::Secondary
                                    rounded=true
                                    text=true
                                    aria_label="Favorite"
                                />
                            </div>
                            <span style="color: var(--p-text-muted-color)">"Updated 2 hours ago"</span>
                        </div>
                    }
                })
            >
                <p style="margin: 0">{LOREM}</p>
            </Panel>
        </Demo>
    }
}
