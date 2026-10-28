//! Страница Message: демонстрации `basic`, `closable`, `severity`, `outlined`, `simple`, `sizes`, `life` из showcase PrimeNG
//! (`apps/showcase/doc/message`). Иконки `pi pi-*` заменены SVG-иконками prime-icons.

use super::Demo;
use leptos::prelude::*;
use prime_button::Button;
use prime_core::Size;
use prime_icons::{CheckIcon, HomeIcon, StarIcon};
use prime_message::{Message, MessageSeverity, MessageVariant};
use std::time::Duration;

const SEVERITIES: [(MessageSeverity, &str); 6] = [
    (MessageSeverity::Success, "Success Message"),
    (MessageSeverity::Info, "Info Message"),
    (MessageSeverity::Warn, "Warn Message"),
    (MessageSeverity::Error, "Error Message"),
    (MessageSeverity::Secondary, "Secondary Message"),
    (MessageSeverity::Contrast, "Contrast Message"),
];

fn severities(variant: Option<MessageVariant>) -> impl IntoView {
    SEVERITIES.iter().map(|(severity, text)| view! { <Message severity=*severity variant=variant>{*text}</Message> }).collect_view()
}

/// Страница `/message`.
#[component]
pub fn MessagePage() -> impl IntoView {
    let visible = RwSignal::new(false);
    let show = move || {
        visible.set(true);
        set_timeout(move || visible.set(false), Duration::from_millis(3000));
    };
    view! {
        <h2>"Message"</h2>
        <Demo name="basic" title="Basic">
            <Message>"Message Content"</Message>
        </Demo>
        <Demo name="closable" title="Closable">
            <Message closable=true>"Closable Message"</Message>
        </Demo>
        <Demo name="severity" title="Severity">
            <div style="display: flex; flex-wrap: wrap; gap: 1rem">{severities(None)}</div>
        </Demo>
        <Demo name="outlined" title="Outlined">
            <div style="display: flex; flex-wrap: wrap; gap: 1rem">{severities(Some(MessageVariant::Outlined))}</div>
        </Demo>
        <Demo name="simple" title="Simple">
            <div style="display: flex; flex-wrap: wrap; gap: 1rem">{severities(Some(MessageVariant::Simple))}</div>
        </Demo>
        <Demo name="sizes" title="Sizes">
            <div style="display: flex; flex-direction: column; align-items: center; gap: 1rem">
                <Message size=Size::Small icon_template=ViewFn::from(|| view! { <StarIcon /> })>"Small Message"</Message>
                <Message icon_template=ViewFn::from(|| view! { <HomeIcon /> })>"Normal Message"</Message>
                <Message size=Size::Large icon_template=ViewFn::from(|| view! { <CheckIcon /> })>"Large Message"</Message>
            </div>
        </Demo>
        <Demo name="life" title="Life">
            <div style="display: flex; flex-direction: column; align-items: center; gap: 1rem">
                <Button label="Show" disabled=visible on_click=Callback::new(move |_| show()) />
                <Show when=move || visible.get()>
                    <Message life=3000u32 severity=MessageSeverity::Success>"Auto disappear message"</Message>
                </Show>
            </div>
        </Demo>
    }
}
