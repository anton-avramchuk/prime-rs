//! Страница OverlayBadge: демонстрация `basic` по `apps/showcase/doc/badge/overlay-doc.ts` PrimeNG.
//!
//! В `prime-icons` нет иконок `bell` и `envelope`, поэтому взяты `calendar`, `info-circle` и `exclamation-triangle`.

use super::Demo;
use leptos::prelude::*;
use prime_icons::{CalendarIcon, ExclamationTriangleIcon, InfoCircleIcon};
use prime_overlaybadge::{BadgeSeverity, OverlayBadge};

/// Страница `/overlaybadge`.
#[component]
pub fn OverlayBadgePage() -> impl IntoView {
    view! {
        <h2>"OverlayBadge"</h2>
        <Demo name="basic" title="Basic">
            <OverlayBadge value="2">
                <span style="font-size: 2rem; display: inline-flex"><InfoCircleIcon /></span>
            </OverlayBadge>
            <OverlayBadge value="4" severity=BadgeSeverity::Danger>
                <span style="font-size: 2rem; display: inline-flex"><CalendarIcon /></span>
            </OverlayBadge>
            <OverlayBadge severity=BadgeSeverity::Danger>
                <span style="font-size: 2rem; display: inline-flex"><ExclamationTriangleIcon /></span>
            </OverlayBadge>
        </Demo>
    }
}
