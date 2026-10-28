//! Главная: иконки и сценарий для замеров памяти.

use leptos::prelude::*;
use prime_icons::AllIcons;

/// Сценарий для браузерных замеров памяти: монтирует и убирает заданное число компонентов.
#[component]
fn Bench() -> impl IntoView {
    let count = RwSignal::new(0usize);
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
