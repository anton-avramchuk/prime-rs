//! Страница ConfirmPopup: демонстрации `basic`, `template` и `headless` из showcase PrimeNG (`apps/showcase/doc/confirmpopup`).
//! У каждой демонстрации свои `ConfirmationService` и `MessageService` (как `providers` в PrimeNG). Значки PrimeIcons заменены
//! значками `prime-icons`.

use super::Demo;
use leptos::prelude::*;
use prime_button::{Button, ButtonSeverity};
use prime_confirmation::{ConfirmButtonProps, Confirmation, provide_confirmation_service};
use prime_confirmpopup::{ConfirmPopup, ConfirmPopupHeadless};
use prime_core::Size;
use prime_icons::{ExclamationTriangleIcon, InfoCircleIcon};
use prime_toast::{Toast, ToastMessage, ToastSeverity, provide_message_service};

#[component]
fn BasicDemo() -> impl IntoView {
    let confirm = provide_confirmation_service();
    let toast = provide_message_service();
    let say = move |severity: ToastSeverity, summary: &'static str, detail: &'static str| {
        Callback::new(move |()| toast.add(ToastMessage::new(severity.clone(), summary, detail).life(3000)))
    };
    let save = move |e: web_sys::MouseEvent| {
        confirm.confirm(
            Confirmation::new()
                .target_event(&e)
                .message("Are you sure you want to proceed?")
                .icon(ViewFn::from(|| view! { <ExclamationTriangleIcon /> }))
                .reject_button_props(ConfirmButtonProps::new().label("Cancel").severity(ButtonSeverity::Secondary).outlined(true))
                .accept_button_props(ConfirmButtonProps::new().label("Save"))
                .accept(say(ToastSeverity::Info, "Confirmed", "You have accepted"))
                .reject(say(ToastSeverity::Error, "Rejected", "You have rejected")),
        );
    };
    let delete = move |e: web_sys::MouseEvent| {
        confirm.confirm(
            Confirmation::new()
                .target_event(&e)
                .message("Do you want to delete this record?")
                .icon(ViewFn::from(|| view! { <InfoCircleIcon /> }))
                .reject_button_props(ConfirmButtonProps::new().label("Cancel").severity(ButtonSeverity::Secondary).outlined(true))
                .accept_button_props(ConfirmButtonProps::new().label("Delete").severity(ButtonSeverity::Danger))
                .accept(say(ToastSeverity::Info, "Confirmed", "Record deleted"))
                .reject(say(ToastSeverity::Error, "Rejected", "You have rejected")),
        );
    };
    view! {
        <Toast />
        <ConfirmPopup />
        <div style="display: flex; gap: 0.5rem">
            <Button label="Save" outlined=true on_click=Callback::new(save) />
            <Button label="Delete" severity=ButtonSeverity::Danger outlined=true on_click=Callback::new(delete) />
        </div>
    }
}

#[component]
fn TemplateDemo() -> impl IntoView {
    let confirm = provide_confirmation_service();
    let toast = provide_message_service();
    let save = move |e: web_sys::MouseEvent| {
        confirm.confirm(
            Confirmation::new()
                .target_event(&e)
                .message("Are you sure you want to proceed?")
                .icon(ViewFn::from(|| view! { <ExclamationTriangleIcon size=48u32 /> }))
                .accept(Callback::new(move |()| {
                    toast.add(ToastMessage::new(ToastSeverity::Info, "Confirmed", "You have accepted").life(3000))
                })),
        );
    };
    view! {
        <Toast />
        <ConfirmPopup content=Callback::new(|c: Confirmation| {
            let icon = c.icon.clone();
            view! {
                <div data-testid="custom-content" style="display: flex; flex-direction: column; align-items: center; gap: 1rem; padding: 1rem 1rem 0">
                    {icon.map(|i| i.run())}
                    <p>{c.message.unwrap_or_default()}</p>
                </div>
            }
            .into_any()
        }) />
        <Button label="Save" on_click=Callback::new(save) />
    }
}

#[component]
fn HeadlessDemo() -> impl IntoView {
    let confirm = provide_confirmation_service();
    let toast = provide_message_service();
    let save = move |e: web_sys::MouseEvent| {
        confirm.confirm(
            Confirmation::new()
                .target_event(&e)
                .message("Save your current process?")
                .accept(Callback::new(move |()| {
                    toast.add(ToastMessage::new(ToastSeverity::Info, "Confirmed", "You have accepted").life(3000))
                }))
                .reject(Callback::new(move |()| {
                    toast.add(ToastMessage::new(ToastSeverity::Error, "Rejected", "You have rejected").life(3000))
                })),
        );
    };
    view! {
        <Toast />
        <ConfirmPopup headless=Callback::new(|ctx: ConfirmPopupHeadless| {
            let (accept, reject) = (ctx.accept, ctx.reject);
            view! {
                <div data-testid="headless" style="padding: 1rem">
                    <span>{ctx.confirmation.message.unwrap_or_default()}</span>
                    <div style="display: flex; align-items: center; gap: 0.5rem; margin-top: 1rem">
                        <Button label="Save" size=Size::Small on_click=Callback::new(move |_| accept.run(())) />
                        <Button label="Cancel" text=true size=Size::Small severity=ButtonSeverity::Secondary on_click=Callback::new(move |_| reject.run(())) />
                    </div>
                </div>
            }
            .into_any()
        }) />
        <Button label="Save" on_click=Callback::new(save) />
    }
}

/// Страница `/confirmpopup`.
#[component]
pub fn ConfirmPopupPage() -> impl IntoView {
    view! {
        <h2>"ConfirmPopup"</h2>
        <Demo name="basic" title="Basic">
            <BasicDemo />
        </Demo>
        <Demo name="template" title="Template">
            <TemplateDemo />
        </Demo>
        <Demo name="headless" title="Headless">
            <HeadlessDemo />
        </Demo>
    }
}
