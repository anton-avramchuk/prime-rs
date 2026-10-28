//! Страницы showcase: по странице на компонент и общая оболочка демонстрации.

mod accordion;
mod avatar;
mod badge;
mod button;
mod card;
mod checkbox;
mod chip;
mod divider;
mod fieldset;
mod home;
mod inputtext;
mod message;
mod panel;
mod progressbar;
mod radiobutton;
mod skeleton;
mod tag;
mod textarea;
mod toggleswitch;

pub use accordion::AccordionPage;
pub use avatar::AvatarPage;
pub use badge::BadgePage;
pub use button::ButtonPage;
pub use card::CardPage;
pub use checkbox::CheckboxPage;
pub use chip::ChipPage;
pub use divider::DividerPage;
pub use fieldset::FieldsetPage;
pub use home::HomePage;
pub use inputtext::InputTextPage;
pub use message::MessagePage;
pub use panel::PanelPage;
pub use progressbar::ProgressBarPage;
pub use radiobutton::RadioButtonPage;
pub use skeleton::SkeletonPage;
pub use tag::TagPage;
pub use textarea::TextareaPage;
pub use toggleswitch::ToggleSwitchPage;

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
