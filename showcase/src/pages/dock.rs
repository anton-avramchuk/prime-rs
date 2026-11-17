//! Dock page: the `basic` demo (position selection) from the PrimeNG showcase (`apps/showcase/doc/dock`). Images from the CDN are replaced
//! with colored tiles with emoji, the hint is the `use:tooltip` directive; the `advanced` demo (Terminal, Galleria) is not ported.

use super::Demo;
use leptos::prelude::*;
use prime_dock::{Dock, DockItem, DockPosition};
use prime_radiobutton::RadioButton;
use prime_tooltip::{TooltipOptions, TooltipPosition, tooltip};

/// Dock items: label, emoji and tile color.
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

/// `/dock` page.
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
