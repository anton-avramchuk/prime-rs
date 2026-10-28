//! Страница ScrollPanel: демонстрации `basic` и `custom` из showcase PrimeNG (`apps/showcase/doc/scrollpanel`).

use super::Demo;
use leptos::prelude::*;
use prime_scrollpanel::ScrollPanel;

const LOREM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";
const SED: &str = "Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo. Nemo enim ipsam voluptatem quia voluptas sit aspernatur aut odit aut fugit, sed quia consequuntur magni dolores eos qui ratione voluptatem sequi nesciunt. Consectetur, adipisci velit, sed quia non numquam eius modi.";
const AT_VERO: &str = "At vero eos et accusamus et iusto odio dignissimos ducimus qui blanditiis praesentium voluptatum deleniti atque corrupti quos dolores et quas molestias excepturi sint occaecati cupiditate non provident, similique sunt in culpa qui officia deserunt mollitia animi, id est laborum et dolorum fuga. Et harum quidem rerum facilis est et expedita distinctio. Nam libero tempore, cum soluta nobis est eligendi optio cumque nihil impedit quo minus.";

/// Страница `/scrollpanel`.
#[component]
pub fn ScrollPanelPage() -> impl IntoView {
    view! {
        <h2>"ScrollPanel"</h2>
        <Demo name="basic" title="Basic">
            <div style="width: 100%">
                <ScrollPanel style="width: 100%; height: 150px">
                    <p>{LOREM}</p>
                    <p>{SED}</p>
                    <p class="m-0" style="margin: 0">{AT_VERO}</p>
                </ScrollPanel>
            </div>
        </Demo>
        <Demo name="custom" title="Custom">
            <style>".p-scrollpanel.custombar .p-scrollpanel-bar { background-color: var(--p-primary-color); }"</style>
            <div style="width: 100%">
                <ScrollPanel style="width: 100%; height: 200px" style_class="custombar">
                    <p>{LOREM}</p>
                    <p>{SED}</p>
                    <p>{AT_VERO}</p>
                    <p>{LOREM}</p>
                    <p>{SED}</p>
                    <p>{AT_VERO}</p>
                </ScrollPanel>
            </div>
        </Demo>
    }
}
