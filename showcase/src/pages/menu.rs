//! Menu page: the `basic`, `group`, `command`, `popup`, `template` demos after PrimeNG `apps/showcase/doc/menu`.
//!
//! The `router` demo is not ported (`MenuItem` has no router fields). `prime-icons` has no `cog`, `sign-out` or
//! `ellipsis-v` icons — `Refresh`, `Upload` and `Bars` are used instead; in `template` there is text instead of a logo,
//! initials instead of an avatar, and the key-combination label is taken from a table by the item label. The popup menu sits
//! in a wrapper with `position: relative`: the panel is rendered inside the root, not in `body`.

use super::Demo;
use leptos::prelude::*;
use prime_button::Button;
use prime_icons::{BarsIcon, PlusIcon, RefreshIcon, SearchIcon, UploadIcon};
use prime_menu::{Menu, MenuCommand, MenuItem, MenuRef};
use prime_toast::{Toast, ToastMessage, ToastSeverity, provide_message_service};

fn icon<V: IntoView + 'static>(f: impl Fn() -> V + Send + Sync + 'static) -> ViewFn {
    ViewFn::from(move || f().into_any())
}

fn plus() -> ViewFn {
    icon(|| view! { <PlusIcon /> })
}

fn search() -> ViewFn {
    icon(|| view! { <SearchIcon /> })
}

fn refresh() -> ViewFn {
    icon(|| view! { <RefreshIcon /> })
}

fn upload() -> ViewFn {
    icon(|| view! { <UploadIcon /> })
}

fn shortcut(label: &str) -> Option<&'static str> {
    match label {
        "New" => Some("⌘+N"),
        "Search" => Some("⌘+S"),
        _ => None,
    }
}

#[component]
fn BasicDemo() -> impl IntoView {
    view! {
        <Demo name="basic" title="Basic">
            <Menu model=vec![MenuItem::new().label("New").icon(plus()), MenuItem::new().label("Search").icon(search())] />
        </Demo>
    }
}

#[component]
fn GroupDemo() -> impl IntoView {
    let model = vec![
        MenuItem::new()
            .label("Documents")
            .items(vec![MenuItem::new().label("New").icon(plus()), MenuItem::new().label("Search").icon(search())]),
        MenuItem::new()
            .label("Profile")
            .items(vec![MenuItem::new().label("Settings").icon(refresh()), MenuItem::new().label("Logout").icon(upload())]),
    ];
    view! {
        <Demo name="group" title="Group">
            <Menu model=model />
        </Demo>
    }
}

#[component]
fn CommandDemo() -> impl IntoView {
    let service = provide_message_service();
    let model = vec![
        MenuItem::new().label("New").icon(plus()).command(Callback::new(move |_: MenuCommand| {
            service.add(ToastMessage::new(ToastSeverity::Success, "Success", "File created").life(3000));
        })),
        MenuItem::new().label("Search").icon(search()).command(Callback::new(move |_: MenuCommand| {
            service.add(ToastMessage::new(ToastSeverity::Warn, "Search Completed", "No results found").life(3000));
        })),
    ];
    view! {
        <Demo name="command" title="Command">
            <Toast />
            <Menu model=model />
        </Demo>
    }
}

#[component]
fn PopupDemo() -> impl IntoView {
    let menu = MenuRef::new();
    let model = vec![
        MenuItem::new()
            .label("Options")
            .items(vec![MenuItem::new().label("Refresh").icon(refresh()), MenuItem::new().label("Export").icon(upload())]),
    ];
    view! {
        <Demo name="popup" title="Popup">
            <div style="position: relative">
                <Menu popup=true menu_ref=menu model=model />
                <Button icon=icon(|| view! { <BarsIcon /> }) aria_label="Options" on_click=Callback::new(move |e: web_sys::MouseEvent| menu.toggle(&e)) />
            </div>
        </Demo>
    }
}

#[component]
fn TemplateDemo() -> impl IntoView {
    let model = vec![
        MenuItem::separator(),
        MenuItem::new()
            .label("Documents")
            .items(vec![MenuItem::new().label("New").icon(plus()), MenuItem::new().label("Search").icon(search())]),
        MenuItem::new()
            .label("Profile")
            .items(vec![MenuItem::new().label("Settings").icon(refresh()), MenuItem::new().label("Messages").icon(upload())]),
        MenuItem::separator(),
    ];
    let item = Callback::new(|item: MenuItem| {
        let label = item.label.clone().unwrap_or_default();
        let hint = shortcut(&label);
        view! {
            <a class="p-menu-item-link" style="padding: 0.5rem 0.75rem">
                {item.icon.clone().map(|i| i.run())}
                <span style="margin-inline-start: 0.5rem">{label}</span>
                {hint.map(|h| view! { <span style="margin-inline-start: auto; font-size: 0.75rem; padding: 0.25rem; border: 1px solid var(--p-content-border-color); border-radius: 4px">{h}</span> })}
            </a>
        }
        .into_any()
    });
    let header = Callback::new(|item: MenuItem| {
        view! { <span style="font-weight: 700; color: var(--p-primary-color)">{item.label}</span> }.into_any()
    });
    view! {
        <Demo name="template" title="Template">
            <Menu
                model=model
                item=item
                submenu_header=header
                style="width: 15rem"
                start=|| view! { <span style="display: inline-flex; padding: 0.5rem; font-size: 1.25rem; font-weight: 600">"PRIME" <span style="color: var(--p-primary-color)">"APP"</span></span> }
                end=|| view! { <div style="display: flex; align-items: center; gap: 0.5rem; padding: 0.5rem 1rem"><span style="font-weight: 700">"AE"</span><span>"Amy Elsner" <br /> <small>"Admin"</small></span></div> }
            />
        </Demo>
    }
}

/// Page `/menu`.
#[component]
pub fn MenuPage() -> impl IntoView {
    view! {
        <BasicDemo />
        <GroupDemo />
        <CommandDemo />
        <PopupDemo />
        <TemplateDemo />
    }
}
