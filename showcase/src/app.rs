//! Оболочка страницы и приложение.

use leptos::hydration::{AutoReload, HydrationScripts};
use leptos::prelude::*;
use prime_icons::AllIcons;

const DARK_CLASS: &str = "p-dark";

/// HTML-оболочка: тема подключается встроенным `<style>` (базовый CSS и CSS компонентов).
pub fn shell(options: LeptosOptions) -> impl IntoView {
    let css = prime_theme::stylesheet(&["button", "badge", "card"]);
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <title>"prime-rs"</title>
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <style>{css}</style>
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

/// Переключает класс тёмной схемы на `<html>`; на сервере ничего не делает.
fn toggle_dark() {
    if let Some(root) = web_sys::window().and_then(|w| w.document()).and_then(|d| d.document_element()) {
        let _ = root.class_list().toggle(DARK_CLASS);
    }
}

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

/// Корневой компонент showcase.
#[component]
pub fn App() -> impl IntoView {
    view! {
        <main style="font-family: sans-serif; padding: 1.5rem; background: var(--p-content-background); color: var(--p-text-color); min-height: 100vh">
            <header style="display: flex; align-items: center; justify-content: space-between">
                <h1>"prime-rs"</h1>
                <button type="button" data-testid="toggle-dark" on:click=|_| toggle_dark()>
                    "Светлая / тёмная схема"
                </button>
            </header>
            <p>"Порт компонентов PrimeNG на Leptos. Компоненты появятся здесь по мере портирования."</p>
            <Bench />
            <h2>"Иконки"</h2>
            <div data-testid="icons" style="display: flex; flex-wrap: wrap; gap: 1rem; font-size: 1.5rem">
                <AllIcons />
            </div>
        </main>
    }
}
