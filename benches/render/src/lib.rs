//! Сценарии бенчмарков: каждый рендерит компонент(ы) в HTML-строку через SSR.
//!
//! Новый компонент добавляет сюда свои сценарии (один экземпляр и 1000 экземпляров) — они автоматически попадают
//! в `render` (divan, время), `alloc` (dhat, аллокации) и `time` (медиана в JSON для сравнения с базовой линией).

use leptos::prelude::*;
use prime_avatar::{Avatar, AvatarShape, AvatarSize};
use prime_badge::{Badge, BadgeSeverity};
use prime_button::{Button, ButtonSeverity};
use prime_chip::Chip;
use prime_icons::{AllIcons, CheckIcon, SpinnerIcon};
use prime_tag::{Tag, TagSeverity};

/// Сценарий: имя и функция, возвращающая HTML.
pub struct Scenario {
    /// Имя вида `<компонент>-<число экземпляров>`.
    pub name: &'static str,
    /// Рендер в строку.
    pub run: fn() -> String,
}

fn render(view: impl IntoView + 'static) -> String {
    Owner::new().with(|| view.into_view().to_html())
}

fn check_n(n: usize) -> String {
    render((0..n).map(|_| view! { <CheckIcon /> }).collect_view())
}

fn spinner_n(n: usize) -> String {
    render((0..n).map(|_| view! { <SpinnerIcon spin=true /> }).collect_view())
}

fn badge_n(n: usize) -> String {
    render((0..n).map(|_| view! { <Badge value="5" severity=BadgeSeverity::Danger /> }).collect_view())
}

fn button_n(n: usize) -> String {
    render((0..n).map(|_| view! { <Button label="Сохранить" severity=ButtonSeverity::Success /> }).collect_view())
}

fn button_full_n(n: usize) -> String {
    let icon = ViewFn::from(|| view! { <CheckIcon /> });
    render((0..n).map(|_| view! { <Button label="Сохранить" icon=icon.clone() badge="2" /> }).collect_view())
}

fn tag_n(n: usize) -> String {
    render((0..n).map(|_| view! { <Tag value="New" severity=TagSeverity::Success rounded=true /> }).collect_view())
}

fn chip_n(n: usize) -> String {
    render((0..n).map(|_| view! { <Chip label="Action" /> }).collect_view())
}

fn chip_removable_n(n: usize) -> String {
    render((0..n).map(|_| view! { <Chip label="Thriller" removable=true /> }).collect_view())
}

fn avatar_n(n: usize) -> String {
    render((0..n).map(|_| view! { <Avatar label="P" shape=AvatarShape::Circle size=AvatarSize::Large /> }).collect_view())
}

/// Все сценарии.
pub const SCENARIOS: &[Scenario] = &[
    Scenario { name: "check-icon-1", run: || check_n(1) },
    Scenario { name: "check-icon-1000", run: || check_n(1000) },
    Scenario { name: "spinner-icon-1000", run: || spinner_n(1000) },
    Scenario { name: "badge-1", run: || badge_n(1) },
    Scenario { name: "badge-1000", run: || badge_n(1000) },
    Scenario { name: "button-1", run: || button_n(1) },
    Scenario { name: "button-1000", run: || button_n(1000) },
    Scenario { name: "button-full-1000", run: || button_full_n(1000) },
    Scenario { name: "tag-1", run: || tag_n(1) },
    Scenario { name: "tag-1000", run: || tag_n(1000) },
    Scenario { name: "chip-1", run: || chip_n(1) },
    Scenario { name: "chip-1000", run: || chip_n(1000) },
    Scenario { name: "chip-removable-1000", run: || chip_removable_n(1000) },
    Scenario { name: "avatar-1", run: || avatar_n(1) },
    Scenario { name: "avatar-1000", run: || avatar_n(1000) },
    Scenario { name: "all-icons-1", run: || render(view! { <AllIcons /> }) },
    Scenario { name: "all-icons-20", run: || render((0..20).map(|_| view! { <AllIcons /> }).collect_view()) },
];

/// Каталог для JSON-отчётов (`target/bench`).
pub fn out_dir() -> std::path::PathBuf {
    let target = std::env::var_os("CARGO_TARGET_DIR")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target"));
    let dir = target.join("bench");
    std::fs::create_dir_all(&dir).expect("не удалось создать target/bench");
    dir
}
