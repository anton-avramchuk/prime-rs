//! Главная: иконки и сценарий для замеров памяти.

use leptos::prelude::*;
use prime_badge::{Badge, BadgeSeverity};
use prime_button::{Button, ButtonSeverity};
use prime_icons::AllIcons;

/// Сценарий для браузерных замеров памяти: монтирует и убирает заданное число компонентов.
#[component]
fn Bench() -> impl IntoView {
    let count = RwSignal::new(0usize);
    let buttons = RwSignal::new(0usize);
    let badges = RwSignal::new(0usize);
    let badges_live = RwSignal::new(0usize);
    // значение бейджа передаётся сигналом: реактивный путь
    let live_value = RwSignal::new(Some("5".to_owned()));
    view! {
        <section>
            <h2>"Замеры"</h2>
            <button type="button" data-testid="bench-mount" on:click=move |_| count.set(1000)>
                "Показать 1000 иконок"
            </button>
            <button type="button" data-testid="bench-clear" on:click=move |_| count.set(0)>
                "Убрать"
            </button>
            <div data-testid="bench-area" data-count=move || count.get().to_string()>
                {move || (0..count.get()).map(|_| view! { <prime_icons::CheckIcon /> }).collect_view()}
            </div>
            <button type="button" data-testid="bench-buttons-mount" on:click=move |_| buttons.set(1000)>
                "Показать 1000 кнопок"
            </button>
            <button type="button" data-testid="bench-buttons-clear" on:click=move |_| buttons.set(0)>
                "Убрать кнопки"
            </button>
            <div data-testid="bench-buttons-area" data-count=move || buttons.get().to_string()>
                {move || {
                    (0..buttons.get()).map(|_| view! { <Button label="Сохранить" severity=ButtonSeverity::Success /> }).collect_view()
                }}
            </div>
            <button type="button" data-testid="bench-badges-mount" on:click=move |_| badges.set(1000)>
                "Показать 1000 бейджей"
            </button>
            <button type="button" data-testid="bench-badges-clear" on:click=move |_| badges.set(0)>
                "Убрать бейджи"
            </button>
            <div data-testid="bench-badges-area" data-count=move || badges.get().to_string()>
                {move || (0..badges.get()).map(|_| view! { <Badge value="5" severity=BadgeSeverity::Danger /> }).collect_view()}
            </div>
            <button type="button" data-testid="bench-badges-live-mount" on:click=move |_| badges_live.set(1000)>
                "Показать 1000 бейджей (сигнал)"
            </button>
            <button type="button" data-testid="bench-badges-live-clear" on:click=move |_| badges_live.set(0)>
                "Убрать бейджи (сигнал)"
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

/// Главная страница.
#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <p>"Порт компонентов PrimeNG на Leptos. Компоненты появляются здесь по мере портирования."</p>
        <Bench />
        <h2>"Иконки"</h2>
        <div data-testid="icons" style="display: flex; flex-wrap: wrap; gap: 1rem; font-size: 1.5rem">
            <AllIcons />
        </div>
    }
}
