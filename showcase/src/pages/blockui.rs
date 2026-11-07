//! Страница BlockUI: демонстрации `basic` (маска над панелью) и `document` (маска над документом на три секунды) из showcase
//! PrimeNG (`apps/showcase/doc/blockui`). Блокируемая панель оборачивается в `div`, на который указывает `NodeRef`.

use super::Demo;
use leptos::prelude::*;
use prime_blockui::BlockUI;
use prime_button::{Button, ButtonSeverity};
use prime_panel::Panel;
use prime_progressspinner::ProgressSpinner;
use std::time::Duration;

const LOREM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna \
                     aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. \
                     Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.";

/// Страница `/blockui`.
#[component]
pub fn BlockUiPage() -> impl IntoView {
    let panel_blocked = RwSignal::new(false);
    let document_blocked = RwSignal::new(false);
    let target = NodeRef::<leptos::html::Div>::new();
    let block_document = move |_| {
        document_blocked.set(true);
        set_timeout(move || document_blocked.set(false), Duration::from_secs(3));
    };
    view! {
        <Demo name="basic" title="Basic">
            <div style="width: 100%">
                <div style="display: flex; gap: 0.5rem">
                    <Button label="Block" severity=ButtonSeverity::Secondary on_click=Callback::new(move |_| panel_blocked.set(true)) />
                    <Button label="Unblock" severity=ButtonSeverity::Secondary on_click=Callback::new(move |_| panel_blocked.set(false)) />
                </div>
                <div node_ref=target data-testid="target" style="margin-top: 1.5rem">
                    <Panel header="Header">
                        <p style="margin: 0">{LOREM}</p>
                    </Panel>
                </div>
                <BlockUI blocked=panel_blocked target=target>
                    <ProgressSpinner style="width: 3rem; height: 3rem" />
                </BlockUI>
            </div>
        </Demo>
        <Demo name="document" title="Document">
            <BlockUI blocked=document_blocked />
            <Button label="Block" on_click=Callback::new(block_document) />
        </Demo>
    }
}
