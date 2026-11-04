//! Страница ContextMenu: демонстрации `basic`, `command` и `template` из showcase PrimeNG (`apps/showcase/doc/contextmenu`;
//! `router` и `table` не переносятся). Картинка из `basic` заменена блоком (без сетевых запросов), значки PrimeIcons — значками
//! `prime-icons` там, где аналог есть.

use super::Demo;
use leptos::prelude::*;
use prime_contextmenu::{ContextMenu, ContextMenuItemContext, ContextMenuRef};
use prime_icons::{PencilIcon, TrashIcon};
use prime_menu::{MenuCommand, MenuItem};
use prime_toast::{MessageService, Toast, ToastMessage, ToastSeverity, provide_message_service};

fn icon<F, V>(f: F) -> ViewFn
where
    F: Fn() -> V + Send + Sync + 'static,
    V: IntoView + 'static,
{
    ViewFn::from(move || f().into_any())
}

fn basic_items() -> Vec<MenuItem> {
    vec![MenuItem::new().label("Copy"), MenuItem::new().label("Rename").icon(icon(|| view! { <PencilIcon /> }))]
}

/// Вложенные пункты для демонстрации подменю и шаблона.
fn nested_items() -> Vec<MenuItem> {
    vec![
        MenuItem::new().label("File").items(vec![
            MenuItem::new().label("New").items(vec![MenuItem::new().label("Document"), MenuItem::new().label("Image")]),
            MenuItem::new().label("Open"),
        ]),
        MenuItem::new()
            .label("Edit")
            .icon(icon(|| view! { <PencilIcon /> }))
            .items(vec![MenuItem::new().label("Copy"), MenuItem::new().label("Delete")]),
        MenuItem::separator(),
        MenuItem::new().label("Quit"),
    ]
}

fn say(toast: MessageService, severity: ToastSeverity, summary: &'static str, detail: &'static str) -> Callback<MenuCommand> {
    Callback::new(move |_| toast.add(ToastMessage::new(severity.clone(), summary, detail).life(3000)))
}

const USERS: [(&str, &str); 3] = [("Amy Elsner", "Admin"), ("Anna Fali", "Owner"), ("Asiya Javayant", "Guest")];

#[component]
fn CommandDemo() -> impl IntoView {
    let toast = provide_message_service();
    let menu = ContextMenuRef::new();
    let selected = RwSignal::new(None::<usize>);
    let items = vec![
        MenuItem::new().label("View").command(say(toast, ToastSeverity::Info, "View", "View requested")),
        MenuItem::new().label("Delete").icon(icon(|| view! { <TrashIcon /> })).command(say(
            toast,
            ToastSeverity::Error,
            "Delete",
            "Delete requested",
        )),
    ];
    view! {
        <Toast />
        <ul data-testid="users" style="margin: 0; list-style: none; width: 24rem; display: flex; flex-direction: column; gap: 0.5rem">
            {USERS
                .iter()
                .enumerate()
                .map(|(i, (name, role))| {
                    view! {
                        <li
                            data-testid="user"
                            style="display: flex; justify-content: space-between; padding: 0.5rem; border: 1px solid transparent; border-radius: 6px"
                            style:border-color=move || (selected.get() == Some(i)).then_some("var(--p-primary-color)")
                            on:contextmenu=move |e| {
                                selected.set(Some(i));
                                menu.show(&e);
                            }
                        >
                            <b>{*name}</b>
                            <span>{*role}</span>
                        </li>
                    }
                })
                .collect_view()}
        </ul>
        <ContextMenu id="command_menu" model=items popup_ref=menu on_hide=Callback::new(move |()| selected.set(None)) />
    }
}

fn template_item(ctx: ContextMenuItemContext) -> AnyView {
    let ContextMenuItemContext { item, has_submenu } = ctx;
    view! {
        <a style="display: flex; align-items: center; padding: 0.5rem 0.75rem; cursor: pointer; gap: 0.5rem" data-testid="template-item">
            {item.icon.clone().map(|i| i.run())}
            <span>{item.label.clone().unwrap_or_default()}</span>
            {has_submenu.then(|| view! { <span style="margin-left: auto">"›"</span> })}
        </a>
    }
    .into_any()
}

/// Страница `/contextmenu`.
#[component]
pub fn ContextMenuPage() -> impl IntoView {
    view! {
        <h2>"ContextMenu"</h2>
        <Demo name="basic" title="Basic">
            <div
                id="basic_target"
                aria-haspopup="true"
                style="width: 30rem; max-width: 100%; height: 12rem; display: flex; align-items: center; justify-content: center; border-radius: 6px; background: var(--p-content-hover-background)"
            >
                "Right click here"
            </div>
            <ContextMenu id="basic_menu" target="#basic_target" model=basic_items() />
        </Demo>
        <Demo name="nested" title="Nested">
            <div
                id="nested_target"
                style="width: 30rem; max-width: 100%; height: 8rem; display: flex; align-items: center; justify-content: center; border-radius: 6px; background: var(--p-content-hover-background)"
            >
                "Right click here"
            </div>
            <ContextMenu id="nested_menu" target="#nested_target" model=nested_items() />
        </Demo>
        <Demo name="command" title="Command">
            <CommandDemo />
        </Demo>
        <Demo name="template" title="Template">
            <div
                id="template_target"
                style="width: 30rem; max-width: 100%; height: 8rem; display: flex; align-items: center; justify-content: center; border-radius: 6px; background: var(--p-content-hover-background)"
            >
                "Right click here"
            </div>
            <ContextMenu id="template_menu" target="#template_target" model=nested_items() item=Callback::new(template_item) />
        </Demo>
    }
}
