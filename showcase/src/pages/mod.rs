//! Страницы showcase: по странице на компонент и общая оболочка демонстрации.

mod accordion;
mod avatar;
mod badge;
mod breadcrumb;
mod button;
mod buttongroup;
mod card;
mod checkbox;
mod chip;
mod divider;
mod fieldset;
mod floatlabel;
mod fluid;
mod home;
mod iconfield;
mod iftalabel;
mod inputgroup;
mod inputnumber;
mod inputotp;
mod inputtext;
mod knob;
mod message;
mod metergroup;
mod overlaybadge;
mod panel;
mod progressbar;
mod progressspinner;
mod radiobutton;
mod rating;
mod scrollpanel;
mod scrolltop;
mod select;
mod selectbutton;
mod skeleton;
mod slider;
mod tag;
mod textarea;
mod timeline;
mod togglebutton;
mod toggleswitch;
mod toolbar;

pub use accordion::AccordionPage;
pub use avatar::AvatarPage;
pub use badge::BadgePage;
pub use breadcrumb::BreadcrumbPage;
pub use button::ButtonPage;
pub use buttongroup::ButtonGroupPage;
pub use card::CardPage;
pub use checkbox::CheckboxPage;
pub use chip::ChipPage;
pub use divider::DividerPage;
pub use fieldset::FieldsetPage;
pub use floatlabel::FloatLabelPage;
pub use fluid::FluidPage;
pub use home::HomePage;
pub use iconfield::IconFieldPage;
pub use iftalabel::IftaLabelPage;
pub use inputgroup::InputGroupPage;
pub use inputnumber::InputNumberPage;
pub use inputotp::InputOtpPage;
pub use inputtext::InputTextPage;
pub use knob::KnobPage;
pub use message::MessagePage;
pub use metergroup::MeterGroupPage;
pub use overlaybadge::OverlayBadgePage;
pub use panel::PanelPage;
pub use progressbar::ProgressBarPage;
pub use progressspinner::ProgressSpinnerPage;
pub use radiobutton::RadioButtonPage;
pub use rating::RatingPage;
pub use scrollpanel::ScrollPanelPage;
pub use scrolltop::ScrollTopPage;
pub use select::SelectPage;
pub use selectbutton::SelectButtonPage;
pub use skeleton::SkeletonPage;
pub use slider::SliderPage;
pub use tag::TagPage;
pub use textarea::TextareaPage;
pub use timeline::TimelinePage;
pub use togglebutton::ToggleButtonPage;
pub use toggleswitch::ToggleSwitchPage;
pub use toolbar::ToolbarPage;

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
