//! Страницы showcase: по странице на компонент и общая оболочка демонстрации.

mod avatar;
mod badge;
mod button;
mod chip;
mod home;
mod tag;

pub use avatar::AvatarPage;
pub use badge::BadgePage;
pub use button::ButtonPage;
pub use chip::ChipPage;
pub use home::HomePage;
pub use tag::TagPage;

use leptos::prelude::*;

/// Одна демонстрация. Разметка повторяет showcase PrimeNG (`<имя-doc> .card`), чтобы страницы можно было сравнивать скриншотами:
/// у нас `[data-demo="имя"]`, у PrimeNG `имя-doc .card`.
#[component]
pub fn Demo(
    /// Имя демонстрации (как у PrimeNG: `basic`, `severity`, …).
    name: &'static str,
    /// Заголовок.
    title: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <section style="padding: 1rem 0">
            <h3 id=name>{title}</h3>
            <div
                class="card"
                data-demo=name
                style="display: flex; flex-wrap: wrap; align-items: center; justify-content: center; gap: 1rem; padding: 2rem; margin-bottom: 1rem; border: 1px solid var(--p-content-border-color); border-radius: 10px; background: var(--p-content-background)"
            >
                {children()}
            </div>
        </section>
    }
}
