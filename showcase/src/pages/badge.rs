//! Страница Badge: демонстрации `basic`, `size`, `severity` из showcase PrimeNG (`apps/showcase/doc/badge`).

use super::Demo;
use leptos::prelude::*;
use prime_badge::{Badge, BadgeSeverity, BadgeSize};

/// Страница `/badge`.
#[component]
pub fn BadgePage() -> impl IntoView {
    view! {
        <h2>"Badge"</h2>
        <Demo name="basic" title="Basic">
            <Badge value="2" />
        </Demo>
        <Demo name="size" title="Size">
            <Badge value="8" size=BadgeSize::XLarge severity=BadgeSeverity::Success />
            <Badge value="6" size=BadgeSize::Large severity=BadgeSeverity::Warn />
            <Badge value="4" severity=BadgeSeverity::Info />
            <Badge value="2" size=BadgeSize::Small />
        </Demo>
        <Demo name="severity" title="Severity">
            <Badge value="2" />
            <Badge value="6" severity=BadgeSeverity::Secondary />
            <Badge value="8" severity=BadgeSeverity::Success />
            <Badge value="4" severity=BadgeSeverity::Info />
            <Badge value="9" severity=BadgeSeverity::Warn />
            <Badge value="3" severity=BadgeSeverity::Danger />
            <Badge value="5" severity=BadgeSeverity::Contrast />
        </Demo>
    }
}
