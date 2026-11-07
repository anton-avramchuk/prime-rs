//! Страница SplitButton: демонстрации `basic`, `disabled`, `icons`, `outlined`, `raised`, `raisedtext`, `rounded`, `severity`, `sizes`,
//! `template`, `text` из showcase PrimeNG (`apps/showcase/doc/splitbutton`). Демонстрация `nested` (подменю) не переносится до
//! `TieredMenu`, `reversedkeys` — текст про токены темы. Значки PrimeIcons (`pi-check`, `pi-cog`) заменены значками `prime-icons`,
//! логотип в `template` — текстом.

use super::Demo;
use leptos::prelude::*;
use prime_button::ButtonSeverity;
use prime_core::Size;
use prime_icons::{CheckIcon, RefreshIcon};
use prime_menu::{MenuCommand, MenuItem};
use prime_splitbutton::SplitButton;
use prime_toast::{Toast, ToastMessage, ToastSeverity, provide_message_service};

/// Команды меню (как `items` в демонстрациях PrimeNG): сообщения через общий `Toast`.
fn items(service: prime_toast::MessageService) -> Vec<MenuItem> {
    let say = move |severity: ToastSeverity, summary: &'static str, detail: &'static str| {
        Callback::new(move |_: MenuCommand| service.add(ToastMessage::new(severity.clone(), summary, detail).life(3000)))
    };
    vec![
        MenuItem::new().label("Update").command(say(ToastSeverity::Success, "Updated", "Data Updated")),
        MenuItem::new().label("Delete").command(say(ToastSeverity::Warn, "Delete", "Data Deleted")),
        MenuItem::new().label("Upload").url("#upload"),
        MenuItem::separator(),
        MenuItem::new().label("Home").url("#home"),
    ]
}

macro_rules! severities {
    ($service:expr, $($extra:tt)*) => {
        view! {
            <SplitButton label="Primary" model=items($service) $($extra)* />
            <SplitButton label="Secondary" model=items($service) severity=ButtonSeverity::Secondary $($extra)* />
            <SplitButton label="Success" model=items($service) severity=ButtonSeverity::Success $($extra)* />
            <SplitButton label="Info" model=items($service) severity=ButtonSeverity::Info $($extra)* />
            <SplitButton label="Warn" model=items($service) severity=ButtonSeverity::Warn $($extra)* />
            <SplitButton label="Help" model=items($service) severity=ButtonSeverity::Help $($extra)* />
            <SplitButton label="Danger" model=items($service) severity=ButtonSeverity::Danger $($extra)* />
        }
    };
}

/// Страница `/splitbutton`.
#[component]
pub fn SplitButtonPage() -> impl IntoView {
    let service = provide_message_service();
    let saved = RwSignal::new(0u32);
    let save = Callback::new(move |_| {
        saved.update(|n| *n += 1);
        service.add(ToastMessage::new(ToastSeverity::Info, "Success", "Data Saved").life(3000));
    });
    let check = ViewFn::from(|| view! { <CheckIcon /> });
    let cog = ViewFn::from(|| view! { <RefreshIcon /> });
    view! {
        <Toast />
        <Demo name="basic" title="Basic">
            <SplitButton label="Save" on_click=save model=items(service) />
            <span data-testid="echo">{move || saved.get().to_string()}</span>
        </Demo>
        <Demo name="disabled" title="Disabled">
            <SplitButton label="Save" model=items(service) disabled=true />
        </Demo>
        <Demo name="icons" title="Icons">
            <SplitButton label="Save" icon=check dropdown_icon=cog model=items(service) />
        </Demo>
        <Demo name="outlined" title="Outlined">{severities!(service, outlined=true)}</Demo>
        <Demo name="raised" title="Raised">{severities!(service, raised=true)}</Demo>
        <Demo name="raisedtext" title="Raised Text">{severities!(service, raised=true text=true)}</Demo>
        <Demo name="rounded" title="Rounded">{severities!(service, rounded=true)}</Demo>
        <Demo name="severity" title="Severity">{severities!(service,)}</Demo>
        <Demo name="sizes" title="Sizes">
            <SplitButton label="Small" model=items(service) size=Size::Small />
            <SplitButton label="Normal" model=items(service) />
            <SplitButton label="Large" model=items(service) size=Size::Large />
        </Demo>
        <Demo name="template" title="Template">
            <SplitButton
                label="Save"
                model=items(service)
                severity=ButtonSeverity::Contrast
                content=ViewFn::from(|| view! { <span style="display: flex; align-items: center; font-weight: 700">"PrimeNG"</span> })
            />
        </Demo>
        <Demo name="text" title="Text">{severities!(service, text=true)}</Demo>
    }
}
