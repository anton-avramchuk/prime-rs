//! Страница PanelMenu: демонстрации `basic`, `multiple`, `controlled`, `command` и `template` из showcase PrimeNG
//! (`apps/showcase/doc/panelmenu`; `router` не переносится). Значки PrimeIcons заменены значками `prime-icons` там, где аналог есть.

use super::Demo;
use leptos::prelude::*;
use prime_button::{Button, ButtonVariant};
use prime_icons::{CalendarIcon, CheckIcon, HomeIcon, PlusIcon, RefreshIcon, SearchIcon, StarIcon, TrashIcon, UploadIcon};
use prime_menu::{MenuCommand, MenuItem};
use prime_panelmenu::{PanelMenu, PanelMenuItemContext, PanelMenuRef};
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
        MenuItem::new().label("Files").icon(icon(|| view! { <StarIcon /> })).items(vec![
            MenuItem::new().label("Documents").icon(icon(|| view! { <StarIcon /> })).items(vec![
                MenuItem::new().label("Invoices").icon(icon(|| view! { <CalendarIcon /> })).items(vec![
                    MenuItem::new().label("Pending").icon(icon(|| view! { <RefreshIcon /> })),
                    MenuItem::new().label("Paid").icon(icon(|| view! { <CheckIcon /> })),
                ]),
                MenuItem::new().label("Clients").icon(icon(|| view! { <HomeIcon /> })),
            ]),
            MenuItem::new()
                .label("Images")
                .icon(icon(|| view! { <SearchIcon /> }))
                .items(vec![MenuItem::new().label("Logos").icon(icon(|| view! { <SearchIcon /> }))]),
        ]),
        MenuItem::new().label("Cloud").icon(icon(|| view! { <UploadIcon /> })).items(vec![
            MenuItem::new().label("Upload").icon(icon(|| view! { <UploadIcon /> })),
            MenuItem::new().label("Download").icon(icon(|| view! { <UploadIcon /> })),
            MenuItem::new().label("Sync").icon(icon(|| view! { <RefreshIcon /> })),
        ]),
        MenuItem::new().label("Devices").icon(icon(|| view! { <HomeIcon /> })).items(vec![
            MenuItem::new().label("Phone"),
            MenuItem::new().label("Desktop"),
            MenuItem::new().label("Tablet"),
        ]),
    ]
}

fn say(toast: MessageService, severity: ToastSeverity, summary: &'static str, detail: &'static str) -> Callback<MenuCommand> {
    Callback::new(move |_| toast.add(ToastMessage::new(severity.clone(), summary, detail).life(3000)))
}

fn command_items(toast: MessageService) -> Vec<MenuItem> {
    vec![
        MenuItem::new().label("Files").icon(icon(|| view! { <StarIcon /> })).items(vec![
            MenuItem::new().label("New").icon(icon(|| view! { <PlusIcon /> })).command(say(
                toast,
                ToastSeverity::Success,
                "File created",
                "File created successfully",
            )),
            MenuItem::new().label("Search").icon(icon(|| view! { <SearchIcon /> })).command(say(
                toast,
                ToastSeverity::Info,
                "Search Results",
                "No results found",
            )),
            MenuItem::new().label("Print").command(say(toast, ToastSeverity::Warn, "Printing", "Preparing to print")),
        ]),
        MenuItem::new().label("Sync").icon(icon(|| view! { <RefreshIcon /> })).items(vec![
            MenuItem::new().label("Import").icon(icon(|| view! { <UploadIcon /> })).command(say(
                toast,
                ToastSeverity::Success,
                "Imported",
                "Data imported successfully",
            )),
            MenuItem::new().label("Export").icon(icon(|| view! { <UploadIcon /> })).command(say(
                toast,
                ToastSeverity::Success,
                "Exported",
                "Data exported successfully",
            )),
        ]),
        MenuItem::new().label("Sign Out").icon(icon(|| view! { <TrashIcon /> })).command(say(
            toast,
            ToastSeverity::Error,
            "Signed out",
            "User logged out",
        )),
    ]
}

fn controlled_items() -> Vec<MenuItem> {
    vec![
        MenuItem::new().label("Users").icon(icon(|| view! { <StarIcon /> })).items(vec![
            MenuItem::new().label("New").items(vec![MenuItem::new().label("Member"), MenuItem::new().label("Group")]),
            MenuItem::new().label("Search"),
        ]),
        MenuItem::new().label("Tasks").icon(icon(|| view! { <CheckIcon /> })).items(vec![
            MenuItem::new().label("Add New"),
            MenuItem::new().label("Pending"),
            MenuItem::new().label("Overdue"),
        ]),
        MenuItem::new().label("Calendar").icon(icon(|| view! { <CalendarIcon /> })).items(vec![
            MenuItem::new().label("New Event"),
            MenuItem::new().label("Today"),
            MenuItem::new().label("This Week"),
        ]),
    ]
}

fn template_items() -> Vec<MenuItem> {
    vec![
        MenuItem::new().label("Mail").icon(icon(|| view! { <HomeIcon /> })).items(vec![
            MenuItem::new().label("Compose").icon(icon(|| view! { <PlusIcon /> })),
            MenuItem::new().label("Inbox").icon(icon(|| view! { <UploadIcon /> })),
            MenuItem::new().label("Sent").icon(icon(|| view! { <UploadIcon /> })),
            MenuItem::new().label("Trash").icon(icon(|| view! { <TrashIcon /> })),
        ]),
        MenuItem::new().label("Reports").icon(icon(|| view! { <StarIcon /> })).items(vec![
            MenuItem::new().label("Sales").icon(icon(|| view! { <StarIcon /> })),
            MenuItem::new().label("Products").icon(icon(|| view! { <StarIcon /> })),
        ]),
        MenuItem::new().label("Profile").icon(icon(|| view! { <HomeIcon /> })).items(vec![
            MenuItem::new().label("Settings").icon(icon(|| view! { <CheckIcon /> })),
            MenuItem::new().label("Privacy").icon(icon(|| view! { <CheckIcon /> })),
        ]),
    ]
}

fn template_item(ctx: PanelMenuItemContext) -> AnyView {
    let PanelMenuItemContext { item, .. } = ctx;
    view! {
        <a style="display: flex; align-items: center; padding: 0.5rem 1rem; cursor: pointer; gap: 0.5rem" data-testid="template-item">
            {item.icon.clone().map(|i| i.run())}
            <span>{item.label.clone().unwrap_or_default()}</span>
        </a>
    }
    .into_any()
}

#[component]
fn CommandDemo() -> impl IntoView {
    let toast = provide_message_service();
    view! {
        <Toast />
        <PanelMenu id="command_menu" style="width: 100%; max-width: 20rem" model=command_items(toast) />
    }
}

#[component]
fn ControlledDemo() -> impl IntoView {
    let panel_ref = PanelMenuRef::new();
    view! {
        <div style="display: flex; flex-direction: column; align-items: center; gap: 1rem; width: 100%">
            <Button label="Toggle All" variant=ButtonVariant::Text on_click=Callback::new(move |_| panel_ref.toggle_all()) />
            <PanelMenu id="controlled_menu" style="width: 100%; max-width: 20rem" model=controlled_items() multiple=true panel_ref=panel_ref />
        </div>
    }
}

/// Страница `/panelmenu`.
#[component]
pub fn PanelMenuPage() -> impl IntoView {
    view! {
        <Demo name="basic" title="Basic">
            <PanelMenu id="basic_menu" style="width: 100%; max-width: 20rem" model=items() />
        </Demo>
        <Demo name="multiple" title="Multiple">
            <PanelMenu id="multiple_menu" style="width: 100%; max-width: 20rem" model=items() multiple=true />
        </Demo>
        <Demo name="controlled" title="Controlled">
            <ControlledDemo />
        </Demo>
        <Demo name="command" title="Command">
            <CommandDemo />
        </Demo>
        <Demo name="template" title="Template">
            <PanelMenu id="template_menu" style="width: 100%; max-width: 20rem" model=template_items() item=Callback::new(template_item) />
        </Demo>
    }
}
