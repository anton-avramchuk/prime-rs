//! Страница Menubar: демонстрации `basic`, `command` и `template` из showcase PrimeNG (`apps/showcase/doc/menubar`;
//! `router` не переносится). Значки PrimeIcons заменены значками `prime-icons` там, где аналог есть.

use super::Demo;
use leptos::prelude::*;
use prime_icons::{HomeIcon, PlusIcon, RefreshIcon, SearchIcon, StarIcon, UploadIcon};
use prime_menu::{MenuCommand, MenuItem};
use prime_menubar::{Menubar, MenubarItemContext};
use prime_toast::{MessageService, Toast, ToastMessage, ToastSeverity, provide_message_service};

fn icon<F, V>(f: F) -> ViewFn
where
    F: Fn() -> V + Send + Sync + 'static,
    V: IntoView + 'static,
{
    ViewFn::from(move || f().into_any())
}

fn items() -> Vec<MenuItem> {
    vec![
        MenuItem::new().label("Home").icon(icon(|| view! { <HomeIcon /> })),
        MenuItem::new().label("Features").icon(icon(|| view! { <StarIcon /> })),
        MenuItem::new().label("Projects").icon(icon(|| view! { <SearchIcon /> })).items(vec![
            MenuItem::new().label("Components").icon(icon(|| view! { <PlusIcon /> })),
            MenuItem::new().label("Blocks"),
            MenuItem::new().label("UI Kit"),
            MenuItem::separator(),
            MenuItem::new().label("Templates").items(vec![MenuItem::new().label("Apollo"), MenuItem::new().label("Ultima")]),
        ]),
        MenuItem::new().label("Contact").icon(icon(|| view! { <UploadIcon /> })),
    ]
}

fn say(toast: MessageService, severity: ToastSeverity, summary: &'static str, detail: &'static str) -> Callback<MenuCommand> {
    Callback::new(move |_| toast.add(ToastMessage::new(severity.clone(), summary, detail).life(3000)))
}

fn command_items(toast: MessageService) -> Vec<MenuItem> {
    vec![
        MenuItem::new().label("File").icon(icon(|| view! { <UploadIcon /> })).items(vec![
            MenuItem::new().label("New").icon(icon(|| view! { <PlusIcon /> })).items(vec![MenuItem::new().label("Document").command(say(
                toast,
                ToastSeverity::Success,
                "File created",
                "File created successfully",
            ))]),
            MenuItem::new().label("Print").command(say(toast, ToastSeverity::Warn, "Printing", "Preparing to print")),
        ]),
        MenuItem::new().label("Search").icon(icon(|| view! { <SearchIcon /> })).command(say(
            toast,
            ToastSeverity::Info,
            "Search Results",
            "No results found",
        )),
        MenuItem::new().label("Refresh").icon(icon(|| view! { <RefreshIcon /> })).command(say(
            toast,
            ToastSeverity::Success,
            "Refreshed",
            "Page refreshed",
        )),
    ]
}

#[component]
fn CommandDemo() -> impl IntoView {
    let toast = provide_message_service();
    view! {
        <Toast />
        <Menubar id="command_menu" model=command_items(toast) />
    }
}

fn template_item(ctx: MenubarItemContext) -> AnyView {
    let MenubarItemContext { item, has_submenu, root } = ctx;
    view! {
        <a style="display: flex; align-items: center; padding: 0.5rem 0.75rem; cursor: pointer; gap: 0.5rem" data-testid="template-item">
            {item.icon.clone().map(|i| i.run())}
            <span>{item.label.clone().unwrap_or_default()}</span>
            {has_submenu.then(|| view! { <span style="margin-left: auto">{if root { "▾" } else { "›" }}</span> })}
        </a>
    }
    .into_any()
}

/// Страница `/menubar`.
#[component]
pub fn MenubarPage() -> impl IntoView {
    view! {
        <h2>"Menubar"</h2>
        <Demo name="basic" title="Basic">
            <Menubar id="basic_menu" model=items() />
        </Demo>
        <Demo name="command" title="Command">
            <CommandDemo />
        </Demo>
        <Demo name="template" title="Template">
            <Menubar
                id="template_menu"
                model=items()
                item=Callback::new(template_item)
                start=ViewFn::from(|| view! { <b data-testid="menubar-start" style="margin-right: 1rem">"PrimeRS"</b> })
                end=ViewFn::from(|| view! { <span data-testid="menubar-end">"v1"</span> })
            />
        </Demo>
    }
}
