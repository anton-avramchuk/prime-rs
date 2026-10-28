//! Страница Chip: демонстрации `basic`, `icon`, `image`, `template` из showcase PrimeNG (`apps/showcase/doc/chip`).
//!
//! Иконки брендов (`pi-apple`, `pi-facebook`, …) заменены иконками из `prime-icons`; изображения берутся с CDN PrimeNG, как в оригинале.

use super::Demo;
use leptos::prelude::*;
use prime_chip::Chip;
use prime_icons::{HomeIcon, InfoCircleIcon, SearchIcon, StarIcon};

fn icon<V: IntoView + 'static>(f: impl Fn() -> V + Send + Sync + 'static) -> ViewFn {
    ViewFn::from(f)
}

const AVATARS: &str = "https://primefaces.org/cdn/primeng/images/demo/avatar";

/// Страница `/chip`.
#[component]
pub fn ChipPage() -> impl IntoView {
    view! {
        <h2>"Chip"</h2>
        <Demo name="basic" title="Basic">
            <Chip label="Action" />
            <Chip label="Comedy" />
            <Chip label="Mystery" />
            <Chip label="Thriller" removable=true />
        </Demo>
        <Demo name="icon" title="Icon">
            <Chip label="Apple" icon=icon(|| view! { <HomeIcon /> }) />
            <Chip label="Facebook" icon=icon(|| view! { <InfoCircleIcon /> }) />
            <Chip label="Google" icon=icon(|| view! { <SearchIcon /> }) />
            <Chip label="Microsoft" icon=icon(|| view! { <StarIcon /> }) removable=true />
        </Demo>
        <Demo name="image" title="Image">
            <Chip label="Amy Elsner" image=format!("{AVATARS}/amyelsner.png") alt="Avatar image" />
            <Chip label="Asiya Javayant" image=format!("{AVATARS}/asiyajavayant.png") alt="Avatar image" />
            <Chip label="Onyama Limba" image=format!("{AVATARS}/onyamalimba.png") alt="Avatar image" />
            <Chip label="Xuxue Feng" image=format!("{AVATARS}/xuxuefeng.png") alt="Avatar image" removable=true />
        </Demo>
        <Demo name="template" title="Template">
            <Chip style="padding: 0 1rem 0 0">
                <span style="background: var(--p-primary-color); color: var(--p-primary-contrast-color); border-radius: 9999px; width: 2rem; height: 2rem; display: flex; align-items: center; justify-content: center">
                    "P"
                </span>
                <span style="margin-left: 0.5rem; font-weight: 500">"PRIME"</span>
            </Chip>
        </Demo>
    }
}
