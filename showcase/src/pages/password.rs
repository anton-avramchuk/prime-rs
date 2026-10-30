//! Страница Password: демонстрации `basic`, `meter`, `locale`, `template`, `floatlabel`, `iftalabel`, `togglemask`, `clearicon`,
//! `sizes`, `fluid`, `filled`, `invalid`, `disabled` по `apps/showcase/doc/password` PrimeNG.
//!
//! Демонстрации с Angular Forms (`reactiveforms`, `templatedrivenforms`) не переносятся. В `sizes` у поля нет `type="text"`.

use super::Demo;
use leptos::prelude::*;
use prime_core::{Size, Variant};
use prime_divider::Divider;
use prime_floatlabel::{FloatLabel, FloatLabelVariant};
use prime_iftalabel::IftaLabel;
use prime_password::Password;

/// Страница `/password`.
#[component]
pub fn PasswordPage() -> impl IntoView {
    let basic = RwSignal::new(String::new());
    let meter = RwSignal::new(String::new());
    let locale = RwSignal::new(String::new());
    let template = RwSignal::new(String::new());
    let float_over = RwSignal::new(String::new());
    let float_in = RwSignal::new(String::new());
    let float_on = RwSignal::new(String::new());
    let ifta = RwSignal::new(String::new());
    let mask = RwSignal::new(String::new());
    let clear = RwSignal::new("secret".to_owned());
    let (small, normal, large) = (RwSignal::new(String::new()), RwSignal::new(String::new()), RwSignal::new(String::new()));
    let fluid = RwSignal::new(String::new());
    let filled = RwSignal::new(String::new());
    let (invalid1, invalid2) = (RwSignal::new(String::new()), RwSignal::new(String::new()));
    let disabled = RwSignal::new(String::new());
    view! {
        <h2>"Password"</h2>
        <Demo name="basic" title="Basic">
            <Password value=basic feedback=false autocomplete="off" />
        </Demo>
        <Demo name="meter" title="Meter">
            <Password value=meter autocomplete="off" />
        </Demo>
        <Demo name="locale" title="Locale">
            <Password
                value=locale
                prompt_label="Choose a password"
                weak_label="Too simple"
                medium_label="Average complexity"
                strong_label="Complex password"
                autocomplete="off"
            />
        </Demo>
        <Demo name="template" title="Template">
            <Password
                value=template
                autocomplete="off"
                header=ViewFn::from(|| view! { <div style="font-weight: 600; margin-bottom: 1rem">"Reset Password"</div> })
                footer=ViewFn::from(|| {
                    view! {
                        <Divider />
                        <ul style="padding-left: 0.5rem; margin: 0; font-size: 0.875rem">
                            <li>"At least one lowercase"</li>
                            <li>"At least one uppercase"</li>
                            <li>"At least one numeric"</li>
                            <li>"Minimum 8 characters"</li>
                        </ul>
                    }
                })
            />
        </Demo>
        <Demo name="floatlabel" title="Float Label">
            <FloatLabel>
                <Password value=float_over input_id="over_label" autocomplete="off" />
                <label for="over_label">"Over Label"</label>
            </FloatLabel>
            <FloatLabel variant=FloatLabelVariant::In>
                <Password value=float_in input_id="in_label" autocomplete="off" />
                <label for="in_label">"In Label"</label>
            </FloatLabel>
            <FloatLabel variant=FloatLabelVariant::On>
                <Password value=float_on input_id="on_label" autocomplete="off" />
                <label for="on_label">"On Label"</label>
            </FloatLabel>
        </Demo>
        <Demo name="iftalabel" title="Ifta Label">
            <IftaLabel>
                <Password value=ifta input_id="password" autocomplete="off" />
                <label for="password">"Password"</label>
            </IftaLabel>
        </Demo>
        <Demo name="togglemask" title="Toggle Mask">
            <Password value=mask toggle_mask=true autocomplete="off" />
        </Demo>
        <Demo name="clearicon" title="Clear Icon">
            <Password value=clear feedback=false show_clear=true autocomplete="off" input_style="width: 14rem" />
        </Demo>
        <Demo name="sizes" title="Sizes">
            <Password value=small size=Size::Small placeholder="Small" />
            <Password value=normal placeholder="Normal" />
            <Password value=large size=Size::Large placeholder="Large" />
        </Demo>
        <Demo name="fluid" title="Fluid">
            <Password value=fluid feedback=false autocomplete="off" fluid=true />
        </Demo>
        <Demo name="filled" title="Filled">
            <Password value=filled feedback=false variant=Variant::Filled autocomplete="off" />
        </Demo>
        <Demo name="invalid" title="Invalid">
            <Password value=invalid1 invalid=Signal::derive(move || invalid1.get().is_empty()) placeholder="Password" />
            <Password
                value=invalid2
                invalid=Signal::derive(move || invalid2.get().is_empty())
                variant=Variant::Filled
                placeholder="Password"
            />
        </Demo>
        <Demo name="disabled" title="Disabled">
            <Password value=disabled disabled=true placeholder="Disabled" autocomplete="off" />
        </Demo>
    }
}
