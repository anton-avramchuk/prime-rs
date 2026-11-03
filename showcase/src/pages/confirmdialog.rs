//! Страница ConfirmDialog: демонстрации `basic`, `position`, `template` и `headless` из showcase PrimeNG
//! (`apps/showcase/doc/confirmdialog`). У каждой демонстрации свои `ConfirmationService` и `MessageService` (как `providers` в
//! PrimeNG), у `position` — отдельный `key`. Значки PrimeIcons заменены значками `prime-icons`.

use super::Demo;
use leptos::prelude::*;
use prime_button::{Button, ButtonSeverity};
use prime_confirmation::{ConfirmButtonProps, Confirmation, provide_confirmation_service};
use prime_confirmdialog::{ConfirmDialog, ConfirmDialogHeadless};
use prime_icons::{ExclamationTriangleIcon, InfoCircleIcon};
use prime_toast::{Toast, ToastMessage, ToastSeverity, provide_message_service};

fn say(toast: prime_toast::MessageService, severity: ToastSeverity, summary: &'static str, detail: &'static str) -> Callback<()> {
    Callback::new(move |()| toast.add(ToastMessage::new(severity.clone(), summary, detail).life(3000)))
}

#[component]
fn BasicDemo() -> impl IntoView {
    let confirm = provide_confirmation_service();
    let toast = provide_message_service();
    let save = move |_| {
        confirm.confirm(
            Confirmation::new()
                .header("Confirmation")
                .message("Please confirm to proceed.")
                .reject_button_props(ConfirmButtonProps::new().label("Cancel").severity(ButtonSeverity::Secondary).outlined(true))
                .accept_button_props(ConfirmButtonProps::new().label("Save"))
                .accept(say(toast, ToastSeverity::Info, "Confirmed", "You have accepted"))
                .reject(say(toast, ToastSeverity::Error, "Rejected", "You have rejected")),
        );
    };
    let delete = move |_| {
        confirm.confirm(
            Confirmation::new()
                .header("Danger Zone")
                .message("Do you want to delete this record?")
                .icon(ViewFn::from(|| view! { <InfoCircleIcon size=32u32 /> }))
                .reject_button_props(ConfirmButtonProps::new().label("Cancel").severity(ButtonSeverity::Secondary).outlined(true))
                .accept_button_props(ConfirmButtonProps::new().label("Delete").severity(ButtonSeverity::Danger))
                .accept(say(toast, ToastSeverity::Info, "Confirmed", "Record deleted"))
                .reject(say(toast, ToastSeverity::Error, "Rejected", "You have rejected")),
        );
    };
    view! {
        <Toast />
        <ConfirmDialog />
        <div style="display: flex; gap: 0.5rem">
            <Button label="Save" outlined=true on_click=Callback::new(save) />
            <Button label="Delete" severity=ButtonSeverity::Danger outlined=true on_click=Callback::new(delete) />
        </div>
    }
}

#[component]
fn PositionDemo() -> impl IntoView {
    let confirm = provide_confirmation_service();
    let toast = provide_message_service();
    let ask = move |position: &'static str| {
        confirm.confirm(
            Confirmation::new()
                .key("positionDialog")
                .header("Confirmation")
                .message("Are you sure you want to proceed?")
                .icon(ViewFn::from(|| view! { <InfoCircleIcon size=32u32 /> }))
                .position(position)
                .reject_button_props(ConfirmButtonProps::new().label("Cancel").severity(ButtonSeverity::Secondary).text(true))
                .accept_button_props(ConfirmButtonProps::new().label("Save").text(true))
                .accept(say(toast, ToastSeverity::Info, "Confirmed", "Request submitted"))
                .reject(say(toast, ToastSeverity::Warn, "Rejected", "Process incomplete")),
        );
    };
    let button = move |label: &'static str, position: &'static str| {
        view! { <Button label=label severity=ButtonSeverity::Secondary on_click=Callback::new(move |_| ask(position)) /> }
    };
    view! {
        <Toast />
        <ConfirmDialog key="positionDialog" />
        <div style="display: flex; flex-direction: column; gap: 1rem; align-items: center">
            <div style="display: flex; gap: 0.5rem">{button("Left", "left")}{button("Right", "right")}</div>
            <div style="display: flex; gap: 0.5rem">{button("TopLeft", "topleft")}{button("Top", "top")}{button("TopRight", "topright")}</div>
            <div style="display: flex; gap: 0.5rem">
                {button("BottomLeft", "bottomleft")}{button("Bottom", "bottom")}{button("BottomRight", "bottomright")}
            </div>
        </div>
    }
}

#[component]
fn TemplateDemo() -> impl IntoView {
    let confirm = provide_confirmation_service();
    let toast = provide_message_service();
    let save = move |_| {
        confirm.confirm(
            Confirmation::new()
                .header("Confirmation")
                .message("Please confirm to proceed moving forward.")
                .icon(ViewFn::from(|| view! { <ExclamationTriangleIcon size=48u32 /> }))
                .accept(say(toast, ToastSeverity::Info, "Confirmed", "You have accepted")),
        );
    };
    view! {
        <Toast />
        <ConfirmDialog message_content=Callback::new(|c: Confirmation| {
            let icon = c.icon.clone();
            view! {
                <div data-testid="custom-message" style="display: flex; flex-direction: column; align-items: center; gap: 1rem; width: 100%">
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
    let save = move |_| {
        confirm.confirm(
            Confirmation::new()
                .message("Are you sure you want to proceed?")
                .header("Are you sure?")
                .accept(say(toast, ToastSeverity::Info, "Confirmed", "You have accepted"))
                .reject(say(toast, ToastSeverity::Error, "Rejected", "You have rejected")),
        );
    };
    view! {
        <Toast />
        <ConfirmDialog headless=Callback::new(|ctx: ConfirmDialogHeadless| {
            let (accept, reject) = (ctx.accept, ctx.reject);
            view! {
                <div data-testid="headless" style="display: flex; flex-direction: column; align-items: center; padding: 2rem">
                    <span style="font-weight: 700; font-size: 1.25rem">{ctx.confirmation.header.unwrap_or_default()}</span>
                    <p style="margin: 1rem 0">{ctx.confirmation.message.unwrap_or_default()}</p>
                    <div style="display: flex; align-items: center; gap: 0.5rem">
                        <Button label="Save" on_click=Callback::new(move |_| accept.run(())) />
                        <Button label="Cancel" outlined=true on_click=Callback::new(move |_| reject.run(())) />
                    </div>
                </div>
            }
            .into_any()
        }) />
        <Button label="Save" on_click=Callback::new(save) />
    }
}

/// Страница `/confirmdialog`.
#[component]
pub fn ConfirmDialogPage() -> impl IntoView {
    view! {
        <h2>"ConfirmDialog"</h2>
        <Demo name="basic" title="Basic">
            <BasicDemo />
        </Demo>
        <Demo name="position" title="Position">
            <PositionDemo />
        </Demo>
        <Demo name="template" title="Template">
            <TemplateDemo />
        </Demo>
        <Demo name="headless" title="Headless">
            <HeadlessDemo />
        </Demo>
    }
}
