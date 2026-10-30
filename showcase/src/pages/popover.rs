//! Страница Popover: демонстрации `basic`, `target`, `selectdata`, `template` по `apps/showcase/doc/popover` PrimeNG.
//!
//! Демонстрация `datatable` не переносится (нет DataTable). Значков `share-alt`, `copy`, `users`, `image` в `prime-icons` нет,
//! поэтому кнопки без значков, а аватары — инициалы вместо картинок. Панель выводится внутри корня, поэтому триггер и
//! `Popover` лежат в обёртке с `position: relative`.

use super::Demo;
use leptos::prelude::*;
use prime_button::Button;
use prime_inputtext::InputText;
use prime_popover::{Popover, PopoverRef};

const MEMBERS: [(&str, &str, &str, &str); 3] = [
    ("Amy Elsner", "amy@email.com", "Owner", "AE"),
    ("Bernardo Dominic", "bernardo@email.com", "Editor", "BD"),
    ("Ioni Bowcher", "ioni@email.com", "Viewer", "IB"),
];

fn avatar(initials: &'static str) -> impl IntoView {
    view! {
        <span style="display: inline-flex; align-items: center; justify-content: center; width: 32px; height: 32px; border-radius: 50%; background: var(--p-content-hover-background); font-size: 0.75rem">
            {initials}
        </span>
    }
}

/// Страница `/popover`.
#[component]
pub fn PopoverPage() -> impl IntoView {
    let basic = PopoverRef::new();
    let target = PopoverRef::new();
    let select = PopoverRef::new();
    let template = PopoverRef::new();
    let target_el = NodeRef::<leptos::html::Div>::new();
    let selected = RwSignal::new(None::<&'static str>);

    view! {
        <h2>"Popover"</h2>
        <Demo name="basic" title="Basic">
            <div style="position: relative">
                <Button label="Share" on_click=Callback::new(move |e: web_sys::MouseEvent| basic.toggle(&e)) />
                <Popover popover_ref=basic>
                    <div style="display: flex; flex-direction: column; gap: 1rem; width: 25rem">
                        <div>
                            <span style="font-weight: 500; display: block; margin-bottom: 0.5rem">"Share this document"</span>
                            <InputText value=RwSignal::new("https://primeng.org/12323ff26t2g243g423g234gg52hy25XADXAG3".to_owned()) readonly=true />
                        </div>
                        <div>
                            <span style="font-weight: 500; display: block; margin-bottom: 0.5rem">"Team Members"</span>
                            <ul style="list-style: none; padding: 0; margin: 0; display: flex; flex-direction: column; gap: 1rem">
                                {MEMBERS
                                    .iter()
                                    .map(|(name, email, role, initials)| {
                                        view! {
                                            <li style="display: flex; align-items: center; gap: 0.5rem">
                                                {avatar(initials)}
                                                <div>
                                                    <span style="font-weight: 500">{*name}</span>
                                                    <div style="font-size: 0.875rem; color: var(--p-text-muted-color)">{*email}</div>
                                                </div>
                                                <span style="margin-left: auto; font-size: 0.875rem; color: var(--p-text-muted-color)">{*role}</span>
                                            </li>
                                        }
                                    })
                                    .collect_view()}
                            </ul>
                        </div>
                    </div>
                </Popover>
            </div>
        </Demo>
        <Demo name="target" title="Target">
            <div style="position: relative; display: flex; flex-direction: column; align-items: center; gap: 1rem">
                <Button
                    label="Show"
                    on_click=Callback::new(move |_: web_sys::MouseEvent| {
                        if let Some(el) = target_el.get_untracked() {
                            target.toggle_at(&el);
                        }
                    })
                />
                <div
                    node_ref=target_el
                    style="margin-top: 2rem; width: 10rem; height: 5rem; border: 1px solid var(--p-content-border-color); border-radius: var(--p-content-border-radius); display: flex; align-items: center; justify-content: center"
                >
                    <span>"Target Element"</span>
                </div>
                <Popover popover_ref=target>
                    <div style="width: 12rem">"Aligned to the target element, not the button."</div>
                </Popover>
            </div>
        </Demo>
        <Demo name="selectdata" title="Select Data">
            <div style="position: relative">
                <Button
                    label=Signal::derive(move || Some(selected.get().unwrap_or("Select Member").to_owned()))
                    style="min-width: 12rem"
                    on_click=Callback::new(move |e: web_sys::MouseEvent| select.toggle(&e))
                />
                <Popover popover_ref=select>
                    <div>
                        <span style="font-weight: 500; display: block; margin-bottom: 0.5rem">"Team Members"</span>
                        <ul style="list-style: none; padding: 0; margin: 0; display: flex; flex-direction: column">
                            {MEMBERS
                                .iter()
                                .map(|(name, email, _, initials)| {
                                    view! {
                                        <li
                                            style="display: flex; align-items: center; gap: 0.5rem; padding: 0.75rem 0.5rem; cursor: pointer"
                                            on:click=move |_| {
                                                selected.set(Some(name));
                                                select.hide();
                                            }
                                        >
                                            {avatar(initials)}
                                            <div>
                                                <span style="font-weight: 500">{*name}</span>
                                                <div style="font-size: 0.875rem; color: var(--p-text-muted-color)">{*email}</div>
                                            </div>
                                        </li>
                                    }
                                })
                                .collect_view()}
                        </ul>
                    </div>
                </Popover>
            </div>
        </Demo>
        <Demo name="template" title="Template">
            <div style="position: relative">
                <Button label="Show" on_click=Callback::new(move |e: web_sys::MouseEvent| template.toggle(&e)) />
                <Popover popover_ref=template>
                    <h4 style="margin: 0">"Custom Content"</h4>
                </Popover>
            </div>
        </Demo>
    }
}
