//! Страница SpeedDial: демонстрации `linear`, `circle`, `semicircle`, `quartercircle`, `mask`, `tooltip` и `template` из showcase
//! PrimeNG (`apps/showcase/doc/speeddial`). У каждой демонстрации свой `MessageService`. Значки PrimeIcons заменены значками
//! `prime-icons` (`pi-external-link` — стрелкой вправо); пункты с `routerLink` в PrimeNG здесь просто без команды.

use super::Demo;
use leptos::prelude::*;
use prime_button::{Button, ButtonSeverity};
use prime_icons::{ArrowRightIcon, PencilIcon, RefreshIcon, TrashIcon, UploadIcon};
use prime_speeddial::{SpeedDial, SpeedDialDirection, SpeedDialItem, SpeedDialItemContext, SpeedDialType};
use prime_toast::{MessageService, Toast, ToastMessage, ToastSeverity, provide_message_service};
use prime_tooltip::{TooltipOptions, TooltipPosition};

fn command(
    toast: MessageService,
    severity: ToastSeverity,
    summary: &'static str,
    detail: &'static str,
) -> Callback<prime_speeddial::SpeedDialCommand> {
    Callback::new(move |_| toast.add(ToastMessage::new(severity.clone(), summary, detail).life(3000)))
}

fn items(toast: MessageService) -> Vec<SpeedDialItem> {
    vec![
        SpeedDialItem::new().label("Add").icon(ViewFn::from(|| view! { <PencilIcon /> })).command(command(
            toast,
            ToastSeverity::Info,
            "Add",
            "Data Added",
        )),
        SpeedDialItem::new().label("Update").icon(ViewFn::from(|| view! { <RefreshIcon /> })).command(command(
            toast,
            ToastSeverity::Success,
            "Update",
            "Data Updated",
        )),
        SpeedDialItem::new().label("Delete").icon(ViewFn::from(|| view! { <TrashIcon /> })).command(command(
            toast,
            ToastSeverity::Error,
            "Delete",
            "Data Deleted",
        )),
        SpeedDialItem::new().label("Upload").icon(ViewFn::from(|| view! { <UploadIcon /> })),
        SpeedDialItem::new().label("Website").icon(ViewFn::from(|| view! { <ArrowRightIcon /> })),
    ]
}

#[component]
fn LinearDemo() -> impl IntoView {
    let toast = provide_message_service();
    view! {
        <div style="height: 500px; position: relative; width: 100%">
            <Toast />
            <SpeedDial model=items(toast) direction=SpeedDialDirection::Up style="position: absolute; left: calc(50% - 2rem); bottom: 0" />
            <SpeedDial model=items(toast) direction=SpeedDialDirection::Down style="position: absolute; left: calc(50% - 2rem); top: 0" />
            <SpeedDial model=items(toast) direction=SpeedDialDirection::Left style="position: absolute; top: calc(50% - 2rem); right: 0" />
            <SpeedDial model=items(toast) direction=SpeedDialDirection::Right style="position: absolute; top: calc(50% - 2rem); left: 0" />
        </div>
    }
}

#[component]
fn CircleDemo() -> impl IntoView {
    let toast = provide_message_service();
    view! {
        <div style="position: relative; height: 500px; width: 100%; display: flex; align-items: center; justify-content: center">
            <Toast />
            <SpeedDial model=items(toast) r#type=SpeedDialType::Circle radius=80.0 button_severity=ButtonSeverity::Warn style="position: absolute" />
        </div>
    }
}

#[component]
fn SemiCircleDemo() -> impl IntoView {
    let toast = provide_message_service();
    let semi = |direction: SpeedDialDirection, style: &'static str, severity: ButtonSeverity| {
        view! { <SpeedDial model=items(toast) r#type=SpeedDialType::SemiCircle direction=direction radius=80.0 button_severity=severity style=style /> }
    };
    view! {
        <div style="position: relative; height: 500px; width: 100%">
            <Toast />
            {semi(SpeedDialDirection::Up, "position: absolute; left: calc(50% - 2rem); bottom: 0", ButtonSeverity::Help)}
            {semi(SpeedDialDirection::Down, "position: absolute; left: calc(50% - 2rem); top: 0", ButtonSeverity::Help)}
            {semi(SpeedDialDirection::Left, "position: absolute; top: calc(50% - 2rem); right: 0", ButtonSeverity::Success)}
            {semi(SpeedDialDirection::Right, "position: absolute; top: calc(50% - 2rem); left: 0", ButtonSeverity::Success)}
        </div>
    }
}

#[component]
fn QuarterCircleDemo() -> impl IntoView {
    let toast = provide_message_service();
    let quarter = |direction: SpeedDialDirection, style: &'static str| {
        view! { <SpeedDial model=items(toast) r#type=SpeedDialType::QuarterCircle direction=direction radius=80.0 button_severity=ButtonSeverity::Danger style=style /> }
    };
    view! {
        <div style="position: relative; height: 500px; width: 100%">
            <Toast />
            {quarter(SpeedDialDirection::UpLeft, "position: absolute; right: 0; bottom: 0")}
            {quarter(SpeedDialDirection::UpRight, "position: absolute; left: 0; bottom: 0")}
            {quarter(SpeedDialDirection::DownRight, "position: absolute; left: 0; top: 0")}
            {quarter(SpeedDialDirection::DownLeft, "position: absolute; right: 0; top: 0")}
        </div>
    }
}

#[component]
fn MaskDemo() -> impl IntoView {
    let toast = provide_message_service();
    view! {
        <div style="position: relative; height: 350px; width: 100%">
            <Toast />
            <SpeedDial model=items(toast) direction=SpeedDialDirection::Up mask=true style="position: absolute; right: 1rem; bottom: 1rem" />
        </div>
    }
}

#[component]
fn TooltipDemo() -> impl IntoView {
    let toast = provide_message_service();
    view! {
        <div style="position: relative; height: 350px; width: 100%">
            <Toast />
            <SpeedDial
                model=items(toast)
                direction=SpeedDialDirection::Up
                button_severity=ButtonSeverity::Help
                tooltip_options=TooltipOptions::text("").position(TooltipPosition::Left)
                style="position: absolute; right: 0; bottom: 0"
            />
            <SpeedDial
                model=items(toast)
                direction=SpeedDialDirection::Up
                button_severity=ButtonSeverity::Danger
                tooltip_options=TooltipOptions::text("").position(TooltipPosition::Right)
                style="position: absolute; left: 0; bottom: 0"
            />
        </div>
    }
}

#[component]
fn TemplateDemo() -> impl IntoView {
    let toast = provide_message_service();
    view! {
        <div style="height: 400px; width: 100%; position: relative; display: flex; align-items: flex-end; justify-content: center">
            <Toast />
            <SpeedDial
                model=items(toast)
                direction=SpeedDialDirection::Up
                transition_delay=80
                style="position: absolute"
                button_content=Callback::new(|toggle: Callback<web_sys::MouseEvent>| {
                    view! {
                        <span data-testid="custom-toggler">
                            <Button outlined=true label="Menu" on_click=toggle />
                        </span>
                    }
                    .into_any()
                })
                item_content=Callback::new(|ctx: SpeedDialItemContext| {
                    let click = ctx.click;
                    view! {
                        <div
                            data-testid="custom-item"
                            style="display: flex; flex-direction: column; align-items: center; gap: 0.5rem; padding: 0.5rem; width: 5rem; cursor: pointer; border: 1px solid var(--p-content-border-color); border-radius: 0.375rem"
                            on:click=move |e| click.run(e.into())
                        >
                            {ctx.item.icon.map(|i| i.run())}
                            <span>{ctx.item.label.unwrap_or_default()}</span>
                        </div>
                    }
                    .into_any()
                })
            />
        </div>
    }
}

/// Страница `/speeddial`.
#[component]
pub fn SpeedDialPage() -> impl IntoView {
    view! {
        <h2>"SpeedDial"</h2>
        <Demo name="linear" title="Linear">
            <LinearDemo />
        </Demo>
        <Demo name="circle" title="Circle">
            <CircleDemo />
        </Demo>
        <Demo name="semicircle" title="Semi Circle">
            <SemiCircleDemo />
        </Demo>
        <Demo name="quartercircle" title="Quarter Circle">
            <QuarterCircleDemo />
        </Demo>
        <Demo name="mask" title="Mask">
            <MaskDemo />
        </Demo>
        <Demo name="tooltip" title="Tooltip">
            <TooltipDemo />
        </Demo>
        <Demo name="template" title="Template">
            <TemplateDemo />
        </Demo>
    }
}
