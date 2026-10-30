//! Страница InputOtp: демонстрации `basic`, `mask`, `integeronly`, `sizes`, `template` по `apps/showcase/doc/inputotp` PrimeNG.
//!
//! Демонстрации `reactiveforms`, `templatedrivenforms` и `sample` не переносятся (Angular Forms). В `template` вместо
//! значка `pi-minus` разделитель — тире.

use super::Demo;
use leptos::prelude::*;
use prime_core::Size;
use prime_inputotp::{InputOtp, InputOtpSlot};

/// Страница `/inputotp`.
#[component]
pub fn InputOtpPage() -> impl IntoView {
    let basic = RwSignal::new(String::new());
    let mask = RwSignal::new(String::new());
    let integer = RwSignal::new(String::new());
    let small = RwSignal::new(String::new());
    let normal = RwSignal::new(String::new());
    let large = RwSignal::new(String::new());
    let custom = RwSignal::new(String::new());
    let template = Callback::new(|slot: InputOtpSlot| {
        let InputOtpSlot { token, index, events } = slot;
        let separator = (index == 2).then(|| view! { <div style="padding: 0 1rem">"–"</div> });
        view! {
            <input
                type="text"
                maxlength="1"
                class="custom-otp-input"
                style="width: 48px; height: 48px; text-align: center; font-size: 1.5rem; border: 0 none; border-bottom: 2px solid var(--p-inputtext-border-color); background: transparent; color: inherit; outline: none"
                value=token.clone()
                prop:value=token
                on:input=move |e| events.on_input.run(e)
                on:keydown=move |e| events.on_keydown.run(e)
                on:focus=move |e| events.on_focus.run(e)
                on:blur=move |e| events.on_blur.run(e)
                on:paste=move |e| events.on_paste.run(e)
            />
            {separator}
        }
        .into_any()
    });
    view! {
        <h2>"InputOtp"</h2>
        <Demo name="basic" title="Basic">
            <InputOtp value=basic />
            <span data-testid="echo-basic">{move || basic.get()}</span>
        </Demo>
        <Demo name="mask" title="Mask">
            <InputOtp value=mask mask=true />
        </Demo>
        <Demo name="integeronly" title="Integer Only">
            <InputOtp value=integer integer_only=true />
            <span data-testid="echo-integer">{move || integer.get()}</span>
        </Demo>
        <Demo name="sizes" title="Sizes">
            <InputOtp value=small size=Size::Small />
            <InputOtp value=normal />
            <InputOtp value=large size=Size::Large />
        </Demo>
        <Demo name="template" title="Template">
            <InputOtp value=custom integer_only=true length=6 input=template />
        </Demo>
    }
}
