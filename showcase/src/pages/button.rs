//! Страница Button: те же демонстрации, что в showcase PrimeNG (`apps/showcase/doc/button`).
//!
//! Иконки PrimeIcons (`pi pi-user`, `pi-bell`, …) заменены ближайшими из `prime-icons`: в иконках PrimeNG нет
//! пользователя, закладки, колокольчика, сердца; соответствие описано в `docs/button.md`.

use super::Demo;
use leptos::prelude::*;
use prime_badge::BadgeSeverity;
use prime_button::{Button, ButtonSeverity, ButtonVariant, IconPos};
use prime_core::{PrimeConfig, Size, provide_prime_config};
use prime_icons::{
    CheckIcon, ExclamationTriangleIcon, HomeIcon, InfoCircleIcon, PlusIcon, RefreshIcon, SearchIcon, StarFillIcon, StarIcon, TimesIcon,
};
use std::time::Duration;

fn icon<V: IntoView + 'static>(f: impl Fn() -> V + Send + Sync + 'static) -> ViewFn {
    ViewFn::from(f)
}

/// Кнопки всех серьёзностей с общей настройкой.
macro_rules! severities {
    ($($extra:tt)*) => {
        view! {
            <Button label="Primary" $($extra)* />
            <Button label="Secondary" severity=ButtonSeverity::Secondary $($extra)* />
            <Button label="Success" severity=ButtonSeverity::Success $($extra)* />
            <Button label="Info" severity=ButtonSeverity::Info $($extra)* />
            <Button label="Warn" severity=ButtonSeverity::Warn $($extra)* />
            <Button label="Help" severity=ButtonSeverity::Help $($extra)* />
            <Button label="Danger" severity=ButtonSeverity::Danger $($extra)* />
            <Button label="Contrast" severity=ButtonSeverity::Contrast $($extra)* />
        }
    };
}

/// Кнопка с включённым ripple: конфигурация действует только внутри этого компонента.
#[component]
fn RippleDemo() -> impl IntoView {
    provide_prime_config(PrimeConfig { ripple: true, ..Default::default() });
    view! { <Button label="Ripple" /> }
}

/// Страница `/button`.
#[component]
pub fn ButtonPage() -> impl IntoView {
    let loading = RwSignal::new(false);
    let load = Callback::new(move |_: web_sys::MouseEvent| {
        loading.set(true);
        set_timeout(move || loading.set(false), Duration::from_secs(2));
    });
    view! {
        <h2>"Button"</h2>
        <Demo name="basic" title="Basic">
            <Button label="Submit" />
        </Demo>
        <Demo name="icons" title="Icons">
            <Button icon=icon(|| view! { <HomeIcon /> }) aria_label="Save" />
            <Button label="Profile" icon=icon(|| view! { <InfoCircleIcon /> }) />
            <Button label="Save" icon=icon(|| view! { <CheckIcon /> }) icon_pos=IconPos::Right />
            <Button label="Search" icon=icon(|| view! { <SearchIcon /> }) icon_pos=IconPos::Top />
            <Button label="Update" icon=icon(|| view! { <RefreshIcon /> }) icon_pos=IconPos::Bottom />
        </Demo>
        <Demo name="iconsonly" title="Icons Only">
            <Button icon=icon(|| view! { <CheckIcon /> }) />
            <Button icon=icon(|| view! { <StarIcon /> }) severity=ButtonSeverity::Secondary />
            <Button icon=icon(|| view! { <SearchIcon /> }) severity=ButtonSeverity::Success />
            <Button icon=icon(|| view! { <InfoCircleIcon /> }) severity=ButtonSeverity::Info />
            <Button icon=icon(|| view! { <ExclamationTriangleIcon /> }) severity=ButtonSeverity::Warn />
            <Button icon=icon(|| view! { <StarFillIcon /> }) severity=ButtonSeverity::Help />
            <Button icon=icon(|| view! { <TimesIcon /> }) severity=ButtonSeverity::Danger />
            <Button icon=icon(|| view! { <CheckIcon /> }) rounded=true />
            <Button icon=icon(|| view! { <PlusIcon /> }) rounded=true variant=ButtonVariant::Outlined />
        </Demo>
        <Demo name="loading" title="Loading">
            <Button label="Search" icon=icon(|| view! { <CheckIcon /> }) loading=loading on_click=load />
        </Demo>
        <Demo name="severity" title="Severity">{severities!()}</Demo>
        <Demo name="sizes" title="Sizes">
            <Button label="Small" icon=icon(|| view! { <CheckIcon /> }) size=Size::Small />
            <Button label="Normal" icon=icon(|| view! { <CheckIcon /> }) />
            <Button label="Large" icon=icon(|| view! { <CheckIcon /> }) size=Size::Large />
        </Demo>
        <Demo name="badge" title="Badge">
            <Button label="Emails" badge="2" />
            <Button label="Messages" icon=icon(|| view! { <InfoCircleIcon /> }) badge="2" badge_severity=BadgeSeverity::Contrast outlined=true />
        </Demo>
        <Demo name="raised" title="Raised">{severities!(raised=true)}</Demo>
        <Demo name="rounded" title="Rounded">{severities!(rounded=true)}</Demo>
        <Demo name="text" title="Text">{severities!(variant=ButtonVariant::Text)}</Demo>
        <Demo name="outlined" title="Outlined">{severities!(variant=ButtonVariant::Outlined)}</Demo>
        <Demo name="link" title="Link">
            <Button label="Link" link=true />
        </Demo>
        <Demo name="disabled" title="Disabled">
            <Button label="Submit" disabled=true />
        </Demo>
        <Demo name="ripple" title="Ripple (нет в PrimeNG showcase)">
            <RippleDemo />
        </Demo>
        <Demo name="template" title="Template">
            <Button variant=ButtonVariant::Outlined>
                <span style="font-weight: 700">"Template"</span>
            </Button>
        </Demo>
    }
}
