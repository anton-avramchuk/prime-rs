//! Страница Toast: демонстрации `basic`, `severity`, `multiple`, `position`, `sticky`, `life`, `clear`, `target`, `responsive`, `template`,
//! `headless` по `apps/showcase/doc/toast` PrimeNG.
//!
//! У каждой демонстрации свой `MessageService` (как `providers: [MessageService]` в PrimeNG), поэтому безключевые `Toast` разных
//! демонстраций не получают чужих сообщений. `Ripple` из `multiple`, градиентный фон и загрузка файла из `headless` не
//! переносятся: `headless` упрощена до карточки с двумя кнопками, закрывающими сообщение.

use super::Demo;
use leptos::prelude::*;
use prime_avatar::{Avatar, AvatarShape};
use prime_button::{Button, ButtonSeverity};
use prime_toast::{Toast, ToastMessage, ToastPosition, ToastSeverity, provide_message_service};

fn click(f: impl Fn() + Send + Sync + 'static) -> Callback<web_sys::MouseEvent> {
    Callback::new(move |_| f())
}

#[component]
fn BasicDemo() -> impl IntoView {
    let service = provide_message_service();
    view! {
        <Demo name="basic" title="Basic">
            <Toast />
            <Button label="Show" on_click=click(move || service.add(ToastMessage::new(ToastSeverity::Success, "Success Message", "Message Content"))) />
        </Demo>
    }
}

#[component]
fn SeverityDemo() -> impl IntoView {
    let service = provide_message_service();
    let show = move |severity: ToastSeverity, summary: &'static str| {
        click(move || service.add(ToastMessage::new(severity.clone(), summary, "Message Content")))
    };
    view! {
        <Demo name="severity" title="Severity">
            <Toast />
            <div style="display: flex; flex-wrap: wrap; gap: 0.5rem; justify-content: center">
                <Button label="Success" severity=ButtonSeverity::Success on_click=show(ToastSeverity::Success, "Success") />
                <Button label="Info" severity=ButtonSeverity::Info on_click=show(ToastSeverity::Info, "Info") />
                <Button label="Warn" severity=ButtonSeverity::Warn on_click=show(ToastSeverity::Warn, "Warn") />
                <Button label="Error" severity=ButtonSeverity::Danger on_click=show(ToastSeverity::Error, "Error") />
                <Button label="Secondary" severity=ButtonSeverity::Secondary on_click=show(ToastSeverity::Secondary, "Secondary") />
                <Button label="Contrast" severity=ButtonSeverity::Contrast on_click=show(ToastSeverity::Contrast, "Contrast") />
            </div>
        </Demo>
    }
}

#[component]
fn MultipleDemo() -> impl IntoView {
    let service = provide_message_service();
    view! {
        <Demo name="multiple" title="Multiple">
            <Toast />
            <Button
                label="Multiple"
                severity=ButtonSeverity::Warn
                on_click=click(move || {
                    service.add_all(vec![
                        ToastMessage::new(ToastSeverity::Success, "Message 1", "Message Content"),
                        ToastMessage::new(ToastSeverity::Info, "Message 2", "Message Content"),
                        ToastMessage::new(ToastSeverity::Warn, "Message 3", "Message Content"),
                    ]);
                })
            />
        </Demo>
    }
}

#[component]
fn PositionDemo() -> impl IntoView {
    let service = provide_message_service();
    let position = RwSignal::new(ToastPosition::TopRight);
    let show = move |p: ToastPosition, label: &'static str| {
        view! {
            <Button
                label=label
                severity=ButtonSeverity::Secondary
                on_click=click(move || {
                    position.set(p);
                    service.add(ToastMessage::new(ToastSeverity::Info, "Success", "Message Content"));
                })
            />
        }
    };
    view! {
        <Demo name="position" title="Position">
            <Toast position=Signal::derive(move || position.get()) />
            <div style="display: flex; flex-wrap: wrap; gap: 0.5rem; justify-content: center">
                {show(ToastPosition::TopLeft, "Top Left")} {show(ToastPosition::TopCenter, "Top Center")}
                {show(ToastPosition::TopRight, "Top Right")} {show(ToastPosition::BottomLeft, "Bottom Left")}
                {show(ToastPosition::BottomCenter, "Bottom Center")} {show(ToastPosition::BottomRight, "Bottom Right")}
                {show(ToastPosition::Center, "Center")}
            </div>
        </Demo>
    }
}

#[component]
fn StickyDemo() -> impl IntoView {
    let service = provide_message_service();
    view! {
        <Demo name="sticky" title="Sticky">
            <Toast />
            <Button
                label="Sticky"
                on_click=click(move || service.add(ToastMessage::new(ToastSeverity::Info, "Sticky Message", "Message Content").sticky(true)))
            />
        </Demo>
    }
}

#[component]
fn LifeDemo() -> impl IntoView {
    let service = provide_message_service();
    view! {
        <Demo name="life" title="Life">
            <Toast life=10000 />
            <Button label="Life" on_click=click(move || service.add(ToastMessage::new(ToastSeverity::Info, "Life", "Message Content"))) />
        </Demo>
    }
}

#[component]
fn ClearDemo() -> impl IntoView {
    let service = provide_message_service();
    view! {
        <Demo name="clear" title="Clear">
            <Toast key="myKey" />
            <Button
                label="Show"
                on_click=click(move || service.add(ToastMessage::new(ToastSeverity::Info, "Message Content", "Message Content").key("myKey").sticky(true)))
            />
            <Button label="Clear" severity=ButtonSeverity::Secondary on_click=click(move || service.clear_key("myKey")) />
        </Demo>
    }
}

#[component]
fn TargetDemo() -> impl IntoView {
    let service = provide_message_service();
    view! {
        <Demo name="target" title="Target">
            <Toast key="toast1" position=ToastPosition::TopLeft />
            <Toast key="toast2" position=ToastPosition::BottomRight />
            <Button
                label="Show Success"
                on_click=click(move || {
                    service.clear();
                    service.add(ToastMessage::new(ToastSeverity::Success, "Success", "key: toast1").key("toast1"));
                })
            />
            <Button
                label="Show Warning"
                severity=ButtonSeverity::Warn
                on_click=click(move || {
                    service.clear();
                    service.add(ToastMessage::new(ToastSeverity::Warn, "Warning", "key: toast2").key("toast2"));
                })
            />
        </Demo>
    }
}

#[component]
fn ResponsiveDemo() -> impl IntoView {
    let service = provide_message_service();
    let breakpoints = vec![("920px".to_owned(), vec![("width".to_owned(), "50%".to_owned()), ("right".to_owned(), "auto".to_owned())])];
    view! {
        <Demo name="responsive" title="Responsive">
            <Toast breakpoints=breakpoints />
            <Button label="Show" on_click=click(move || service.add(ToastMessage::new(ToastSeverity::Contrast, "Success", "Message Content"))) />
        </Demo>
    }
}

#[component]
fn TemplateDemo() -> impl IntoView {
    let service = provide_message_service();
    view! {
        <Demo name="template" title="Template">
            <Toast
                key="confirm"
                position=ToastPosition::BottomCenter
                base_z_index=5000
                message=Callback::new(move |m: ToastMessage| {
                    view! {
                        <div style="display: flex; flex-direction: column; align-items: flex-start; flex: 1 1 auto">
                            <div style="display: flex; align-items: center; gap: 0.5rem">
                                <Avatar label="AE" shape=AvatarShape::Circle />
                                <span style="font-weight: 700">"Amy Elsner"</span>
                            </div>
                            <div style="font-weight: 500; font-size: 1.125rem; margin: 1rem 0">{m.summary.unwrap_or_default()}</div>
                            <Button
                                severity=ButtonSeverity::Success
                                label="Reply"
                                on_click=click(move || service.clear_key("confirm"))
                            />
                        </div>
                    }
                    .into_any()
                })
            />
            <Button
                label="View"
                on_click=click(move || {
                    service.add(ToastMessage::new(ToastSeverity::Success, "Need to reply?", "").key("confirm").sticky(true));
                })
            />
        </Demo>
    }
}

#[component]
fn HeadlessDemo() -> impl IntoView {
    let service = provide_message_service();
    view! {
        <Demo name="headless" title="Headless">
            <Toast
                key="headless"
                position=ToastPosition::BottomCenter
                headless=Callback::new(|(m, close): (ToastMessage, Callback<()>)| {
                    view! {
                        <section style="display: flex; flex-direction: column; gap: 1rem; padding: 1.5rem; width: 100%; border-radius: 1rem; background: var(--p-primary-color); color: var(--p-primary-contrast-color)">
                            <span style="font-weight: 600">{m.summary.unwrap_or_default()}</span>
                            <div style="display: flex; gap: 1rem; justify-content: flex-end">
                                <Button label="Another Upload?" on_click=click(move || close.run(())) />
                                <Button label="Cancel" on_click=click(move || close.run(())) />
                            </div>
                        </section>
                    }
                    .into_any()
                })
            />
            <Button
                label="Confirm"
                on_click=click(move || {
                    service.add(ToastMessage::new(ToastSeverity::Custom("custom".into()), "Uploading your files.", "").key("headless").sticky(true));
                })
            />
        </Demo>
    }
}

/// Страница `/toast`.
#[component]
pub fn ToastPage() -> impl IntoView {
    view! {
        <h2>"Toast"</h2>
        <BasicDemo />
        <SeverityDemo />
        <MultipleDemo />
        <PositionDemo />
        <StickyDemo />
        <LifeDemo />
        <ClearDemo />
        <TargetDemo />
        <ResponsiveDemo />
        <TemplateDemo />
        <HeadlessDemo />
    }
}
