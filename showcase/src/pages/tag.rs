//! Страница Tag: демонстрации `basic`, `icon`, `pill`, `severity`, `template` из showcase PrimeNG (`apps/showcase/doc/tag`).
//!
//! Иконки PrimeIcons заменены ближайшими из `prime-icons` (`pi-user` → `HomeIcon`, `pi-cog` → `RefreshIcon`); флаг из демонстрации
//! `template` не воспроизводится (внешняя картинка).

use super::Demo;
use leptos::prelude::*;
use prime_icons::{CheckIcon, ExclamationTriangleIcon, HomeIcon, InfoCircleIcon, RefreshIcon, SearchIcon, TimesIcon};
use prime_tag::{Tag, TagSeverity};

fn icon<V: IntoView + 'static>(f: impl Fn() -> V + Send + Sync + 'static) -> ViewFn {
    ViewFn::from(f)
}

/// Теги всех серьёзностей с общей настройкой.
macro_rules! severities {
    ($($extra:tt)*) => {
        view! {
            <Tag value="Primary" $($extra)* />
            <Tag value="Secondary" severity=TagSeverity::Secondary $($extra)* />
            <Tag value="Success" severity=TagSeverity::Success $($extra)* />
            <Tag value="Info" severity=TagSeverity::Info $($extra)* />
            <Tag value="Warn" severity=TagSeverity::Warn $($extra)* />
            <Tag value="Danger" severity=TagSeverity::Danger $($extra)* />
            <Tag value="Contrast" severity=TagSeverity::Contrast $($extra)* />
        }
    };
}

/// Страница `/tag`.
#[component]
pub fn TagPage() -> impl IntoView {
    view! {
        <h2>"Tag"</h2>
        <Demo name="basic" title="Basic">
            <Tag value="New" />
        </Demo>
        <Demo name="icon" title="Icon">
            <Tag icon=icon(|| view! { <HomeIcon /> }) value="Primary" />
            <Tag icon=icon(|| view! { <SearchIcon /> }) severity=TagSeverity::Secondary value="Secondary" />
            <Tag icon=icon(|| view! { <CheckIcon /> }) severity=TagSeverity::Success value="Success" />
            <Tag icon=icon(|| view! { <InfoCircleIcon /> }) severity=TagSeverity::Info value="Info" />
            <Tag icon=icon(|| view! { <ExclamationTriangleIcon /> }) severity=TagSeverity::Warn value="Warn" />
            <Tag icon=icon(|| view! { <TimesIcon /> }) severity=TagSeverity::Danger value="Danger" />
            <Tag icon=icon(|| view! { <RefreshIcon /> }) severity=TagSeverity::Contrast value="Contrast" />
        </Demo>
        <Demo name="pill" title="Pill">{severities!(rounded=true)}</Demo>
        <Demo name="severity" title="Severity">{severities!()}</Demo>
        <Demo name="template" title="Template">
            <Tag style="border: 2px solid var(--p-content-border-color); background: transparent; color: var(--p-text-color)">
                <span style="font-size: 1rem">"Italy"</span>
            </Tag>
        </Demo>
    }
}
