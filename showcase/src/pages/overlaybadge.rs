//! OverlayBadge page: the `basic` demo from `apps/showcase/doc/badge/overlay-doc.ts` in PrimeNG.
//!
//! `prime-icons` has no `bell` and `envelope` icons, so `calendar`, `info-circle` and `exclamation-triangle` are used.

use super::Demo;
use leptos::prelude::*;
use prime_icons::{CalendarIcon, ExclamationTriangleIcon, InfoCircleIcon};
use prime_overlaybadge::{BadgeSeverity, OverlayBadge};

/// `/overlaybadge` page.
#[component]
pub fn OverlayBadgePage() -> impl IntoView {
    view! {
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
