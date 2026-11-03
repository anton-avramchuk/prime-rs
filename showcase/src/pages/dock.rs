//! Страница Dock: демонстрация `basic` (выбор положения) из showcase PrimeNG (`apps/showcase/doc/dock`). Картинки с CDN заменены
//! цветными плитками с эмодзи, подсказка — директивой `use:tooltip`; демонстрация `advanced` (Terminal, Galleria) не переносится.

use super::Demo;
use leptos::prelude::*;
use prime_dock::{Dock, DockItem, DockPosition};
use prime_radiobutton::RadioButton;
use prime_tooltip::{TooltipOptions, TooltipPosition, tooltip};

/// Пункты дока: подпись, эмодзи и цвет плитки.
const APPS: [(&str, &str, &str); 6] = [
    ("Finder", "📁", "#3b82f6"),
    ("Terminal", "⌨️", "#334155"),
    ("Photos", "🖼️", "#ec4899"),
    ("Music", "🎵", "#ef4444"),
    ("Mail", "✉️", "#06b6d4"),
    ("Trash", "🗑️", "#64748b"),
];

fn model() -> Vec<DockItem> {
    APPS.iter().map(|(label, _, _)| DockItem::new().label(*label)).collect()
}

/// Страница `/dock`.
#[component]
pub fn DockPage() -> impl IntoView {
    let choice = RwSignal::new(Some("bottom".to_owned()));
    let position = Signal::derive(move || match choice.get().as_deref() {
        Some("top") => DockPosition::Top,
        Some("left") => DockPosition::Left,
        Some("right") => DockPosition::Right,
        _ => DockPosition::Bottom,
    });
    let tile = Callback::new(|item: DockItem| {
        let label = item.label.unwrap_or_default();
        let (emoji, color) = APPS.iter().find(|(l, _, _)| *l == label).map(|(_, e, c)| (*e, *c)).unwrap_or(("•", "#64748b"));
        view! {
            <span
                style=format!("display: flex; align-items: center; justify-content: center; width: 100%; height: 100%; font-size: 1.75rem; border-radius: 0.75rem; background: {color}")
                use:tooltip=TooltipOptions::text(label).position(TooltipPosition::Top)
            >
                {emoji}
            </span>
        }
        .into_any()
    });
    let option = move |value: &'static str, label: &'static str| {
        view! {
            <div style="display: flex; align-items: center; gap: 0.5rem">
                <RadioButton input_id=value name="dock" value=value model=choice />
                <label for=value>{label}</label>
            </div>
        }
    };
    view! {
        <h2>"Dock"</h2>
        <Demo name="basic" title="Basic">
            <div style="width: 100%">
                <div style="display: flex; flex-wrap: wrap; gap: 1rem; margin-bottom: 2rem">
                    {option("bottom", "Bottom")}
                    {option("top", "Top")}
                    {option("left", "Left")}
                    {option("right", "Right")}
                </div>
                <div
                    data-testid="window"
                    style="position: relative; height: 450px; overflow: hidden; border-radius: 0.75rem; background: linear-gradient(135deg, #1e293b, #475569)"
                >
                    <Dock model=model() position=position item=tile aria_label="Applications" />
                </div>
            </div>
        </Demo>
    }
}
