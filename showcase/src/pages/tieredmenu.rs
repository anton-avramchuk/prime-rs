//! Страница TieredMenu: демонстрации `basic`, `popup`, `command` и `template` из showcase PrimeNG
//! (`apps/showcase/doc/tieredmenu`; `router` не переносится). Значки PrimeIcons заменены значками `prime-icons` там, где аналог есть
//! (`pi-plus`, `pi-search`, `pi-trash`, `pi-upload`, `pi-pencil`, `pi-refresh`), остальные пункты — без значков.

use super::Demo;
use leptos::prelude::*;
use prime_button::Button;
use prime_icons::{PencilIcon, PlusIcon, RefreshIcon, SearchIcon, TrashIcon, UploadIcon};
use prime_menu::{MenuCommand, MenuItem};
use prime_tieredmenu::{TieredMenu, TieredMenuItemContext, TieredMenuRef};
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
        MenuItem::new().label("File").icon(icon(|| view! { <UploadIcon /> })).items(vec![
            MenuItem::new().label("New").icon(icon(|| view! { <PlusIcon /> })).items(vec![
                MenuItem::new().label("Document"),
                MenuItem::new().label("Image"),
                MenuItem::new().label("Video"),
            ]),
            MenuItem::new().label("Open"),
            MenuItem::new().label("Print"),
        ]),
        MenuItem::new()
            .label("Edit")
            .icon(icon(|| view! { <PencilIcon /> }))
            .items(vec![MenuItem::new().label("Copy"), MenuItem::new().label("Delete").icon(icon(|| view! { <TrashIcon /> }))]),
        MenuItem::new().label("Search").icon(icon(|| view! { <SearchIcon /> })),
        MenuItem::new().label("Share").items(vec![MenuItem::new().label("Slack"), MenuItem::new().label("Whatsapp")]),
        MenuItem::separator(),
        MenuItem::new().label("Quit"),
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
        MenuItem::separator(),
        MenuItem::new().label("Quit").command(say(toast, ToastSeverity::Error, "Quit", "Quit requested")),
    ]
}

#[component]
fn PopupDemo() -> impl IntoView {
    let popup_ref = TieredMenuRef::new();
    view! {
        <div style="position: relative">
            <Button label="Toggle" on_click=Callback::new(move |e: web_sys::MouseEvent| popup_ref.toggle(&e)) />
            <TieredMenu id="popup_menu" model=items() popup=true popup_ref=popup_ref />
        </div>
    }
}

#[component]
fn CommandDemo() -> impl IntoView {
    let toast = provide_message_service();
    view! {
        <Toast />
        <TieredMenu id="command_menu" model=command_items(toast) />
    }
}

fn template_item(ctx: TieredMenuItemContext) -> AnyView {
    let TieredMenuItemContext { item, has_submenu } = ctx;
    view! {
        <a style="display: flex; align-items: center; padding: 0.75rem 1rem; cursor: pointer; gap: 0.5rem" data-testid="template-item">
            {item.icon.clone().map(|i| i.run())}
            <span>{item.label.clone().unwrap_or_default()}</span>
            {has_submenu.then(|| view! { <span style="margin-left: auto">"›"</span> })}
        </a>
    }
    .into_any()
}

/// Страница `/tieredmenu`.
#[component]
pub fn TieredMenuPage() -> impl IntoView {
    view! {
        <h2>"TieredMenu"</h2>
        <Demo name="basic" title="Basic">
            <TieredMenu id="basic_menu" model=items() style="width: 250px" />
        </Demo>
        <Demo name="popup" title="Popup">
            <PopupDemo />
        </Demo>
        <Demo name="command" title="Command">
            <CommandDemo />
        </Demo>
        <Demo name="template" title="Template">
            <TieredMenu id="template_menu" model=items() item=Callback::new(template_item) style="width: 250px" />
        </Demo>
    }
}
