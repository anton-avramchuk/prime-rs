//! Home: icons and a scenario for memory measurements.

use leptos::prelude::*;
use prime_badge::{Badge, BadgeSeverity};
use prime_button::{Button, ButtonSeverity};
use prime_icons::AllIcons;

/// Scenario for in-browser memory measurements: mounts and removes the given number of components.
#[component]
fn Bench() -> impl IntoView {
    let count = RwSignal::new(0usize);
    let buttons = RwSignal::new(0usize);
    let buttons_live = RwSignal::new(0usize);
    let live_loading = RwSignal::new(false);
    let badges = RwSignal::new(0usize);
    let badges_live = RwSignal::new(0usize);
    // the badge value is passed as a signal: the reactive path
    let live_value = RwSignal::new(Some("5".to_owned()));
    view! {
        <section>
            <h2>"Measurements"</h2>
            <button type="button" data-testid="bench-mount" on:click=move |_| count.set(1000)>
                "Show 1000 icons"
            </button>
            <button type="button" data-testid="bench-clear" on:click=move |_| count.set(0)>
                "Remove"
            </button>
            <div data-testid="bench-area" data-count=move || count.get().to_string()>
                {move || (0..count.get()).map(|_| view! { <prime_icons::CheckIcon /> }).collect_view()}
            </div>
            <button type="button" data-testid="bench-buttons-mount" on:click=move |_| buttons.set(1000)>
                "Show 1000 buttons"
            </button>
            <button type="button" data-testid="bench-buttons-clear" on:click=move |_| buttons.set(0)>
                "Remove buttons"
            </button>
            <div data-testid="bench-buttons-area" data-count=move || buttons.get().to_string()>
                {move || {
                    (0..buttons.get()).map(|_| view! { <Button label="Save" severity=ButtonSeverity::Success /> }).collect_view()
                }}
            </div>
            <button type="button" data-testid="bench-buttons-live-mount" on:click=move |_| buttons_live.set(1000)>
                "Show 1000 buttons (signal)"
            </button>
            <button type="button" data-testid="bench-buttons-live-clear" on:click=move |_| buttons_live.set(0)>
                "Remove buttons (signal)"
            </button>
            <div data-testid="bench-buttons-live-area" data-count=move || buttons_live.get().to_string()>
                {move || {
                    (0..buttons_live.get())
                        .map(|_| view! { <Button label="Save" severity=ButtonSeverity::Success loading=live_loading /> })
                        .collect_view()
                }}
            </div>
            <button type="button" data-testid="bench-badges-mount" on:click=move |_| badges.set(1000)>
                "Show 1000 badges"
            </button>
            <button type="button" data-testid="bench-badges-clear" on:click=move |_| badges.set(0)>
                "Remove badges"
            </button>
            <div data-testid="bench-badges-area" data-count=move || badges.get().to_string()>
                {move || (0..badges.get()).map(|_| view! { <Badge value="5" severity=BadgeSeverity::Danger /> }).collect_view()}
            </div>
            <button type="button" data-testid="bench-badges-live-mount" on:click=move |_| badges_live.set(1000)>
                "Show 1000 badges (signal)"
            </button>
            <button type="button" data-testid="bench-badges-live-clear" on:click=move |_| badges_live.set(0)>
                "Remove badges (signal)"
            </button>
            <div data-testid="bench-badges-live-area" data-count=move || badges_live.get().to_string()>
                {move || {
                    (0..badges_live.get())
                        .map(|_| view! { <Badge value=live_value severity=BadgeSeverity::Danger /> })
                        .collect_view()
                }}
            </div>
        </section>
    }
}

/// Home page.
#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <p>"A port of PrimeNG components to Leptos. Components appear here as they are ported."</p>
        <Bench />
        <h2>"Icons"</h2>
        <div data-testid="icons" style="display: flex; flex-wrap: wrap; gap: 1rem; font-size: 1.5rem">
            <AllIcons />
        </div>
    }
}
