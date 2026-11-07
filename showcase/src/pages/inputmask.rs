//! Страница InputMask: демонстрации `basic`, `mask`, `optional`, `slotchar`, `unmask`, `clearicon`, `disabled` и `invalid`
//! из showcase PrimeNG (`apps/showcase/doc/inputmask`; формы Angular не переносятся).

use super::Demo;
use leptos::prelude::*;
use prime_inputmask::InputMask;

const WIDTH: &str = "width: 100%; max-width: 14rem";

#[component]
fn Basic() -> impl IntoView {
    let value = RwSignal::new(String::new());
    view! { <InputMask id="basic_mask" mask="99-999999" value=value placeholder="99-999999" style=WIDTH /> }
}

#[component]
fn Masks() -> impl IntoView {
    let (date, phone, serial) = (RwSignal::new(String::new()), RwSignal::new(String::new()), RwSignal::new(String::new()));
    view! {
        <div style="display: flex; flex-wrap: wrap; gap: 1rem">
            <InputMask id="date_mask" mask="99/99/9999" value=date slot_char="mm/dd/yyyy" placeholder="99/99/9999" style=WIDTH />
            <InputMask id="phone_mask" mask="(999) 999-9999" value=phone placeholder="(999) 999-9999" style=WIDTH />
            <InputMask id="serial_mask" mask="a*-999-a999" value=serial placeholder="a*-999-a999" style=WIDTH />
        </div>
    }
}

#[component]
fn Optional() -> impl IntoView {
    let value = RwSignal::new(String::new());
    view! { <InputMask id="optional_mask" mask="(999) 999-9999? x99999" value=value placeholder="(999) 999-9999? x99999" style=WIDTH /> }
}

#[component]
fn SlotChar() -> impl IntoView {
    let value = RwSignal::new(String::new());
    view! { <InputMask id="slot_mask" mask="99/99/9999" value=value slot_char="mm/dd/yyyy" placeholder="mm/dd/yyyy" style=WIDTH /> }
}

#[component]
fn Unmask() -> impl IntoView {
    let value = RwSignal::new(String::new());
    view! {
        <div style="display: flex; flex-direction: column; gap: 0.5rem">
            <InputMask id="unmask_mask" mask="(999) 999-9999" value=value unmask=true placeholder="(999) 999-9999" style=WIDTH />
            <span data-testid="unmasked">{move || value.get()}</span>
        </div>
    }
}

#[component]
fn Clear() -> impl IntoView {
    let value = RwSignal::new("5551234567".to_owned());
    view! { <InputMask id="clear_mask" mask="(999) 999-9999" value=value show_clear=true style=WIDTH /> }
}

/// Страница `/inputmask`.
#[component]
pub fn InputMaskPage() -> impl IntoView {
    view! {
        <Demo name="basic" title="Basic">
            <Basic />
        </Demo>
        <Demo name="mask" title="Mask">
            <Masks />
        </Demo>
        <Demo name="optional" title="Optional">
            <Optional />
        </Demo>
        <Demo name="slotchar" title="SlotChar">
            <SlotChar />
        </Demo>
        <Demo name="unmask" title="Unmask">
            <Unmask />
        </Demo>
        <Demo name="clearicon" title="Clear Icon">
            <Clear />
        </Demo>
        <Demo name="disabled" title="Disabled">
            <InputMask id="disabled_mask" mask="99-999999" disabled=true placeholder="99-999999" style=WIDTH />
        </Demo>
        <Demo name="invalid" title="Invalid">
            <InputMask id="invalid_mask" mask="99-999999" invalid=true placeholder="99-999999" style=WIDTH />
        </Demo>
    }
}
