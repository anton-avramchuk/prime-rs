//! Страницы showcase: по странице на компонент и общая оболочка демонстрации.

mod accordion;
mod avatar;
mod badge;
mod blockui;
mod breadcrumb;
mod button;
mod buttongroup;
mod card;
mod checkbox;
mod chip;
mod confirmdialog;
mod confirmpopup;
mod dialog;
mod divider;
mod dock;
mod drawer;
mod fieldset;
mod floatlabel;
mod fluid;
mod home;
mod iconfield;
mod iftalabel;
mod image;
mod imagecompare;
mod inplace;
mod inputgroup;
mod inputnumber;
mod inputotp;
mod inputtext;
mod knob;
mod listbox;
mod menu;
mod message;
mod metergroup;
mod multiselect;
mod overlaybadge;
mod paginator;
mod panel;
mod password;
mod popover;
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
mod splitbutton;
mod splitter;
mod stepper;
mod tabs;
mod tag;
mod textarea;
mod timeline;
mod toast;
mod togglebutton;
mod toggleswitch;
mod toolbar;
mod tooltip;

pub use accordion::AccordionPage;
pub use avatar::AvatarPage;
pub use badge::BadgePage;
pub use blockui::BlockUiPage;
pub use breadcrumb::BreadcrumbPage;
pub use button::ButtonPage;
pub use buttongroup::ButtonGroupPage;
pub use card::CardPage;
pub use checkbox::CheckboxPage;
pub use chip::ChipPage;
pub use confirmdialog::ConfirmDialogPage;
pub use confirmpopup::ConfirmPopupPage;
pub use dialog::DialogPage;
pub use divider::DividerPage;
pub use dock::DockPage;
pub use drawer::DrawerPage;
pub use fieldset::FieldsetPage;
pub use floatlabel::FloatLabelPage;
pub use fluid::FluidPage;
pub use home::HomePage;
pub use iconfield::IconFieldPage;
pub use iftalabel::IftaLabelPage;
pub use image::ImagePage;
pub use imagecompare::ImageComparePage;
pub use inplace::InplacePage;
pub use inputgroup::InputGroupPage;
pub use inputnumber::InputNumberPage;
pub use inputotp::InputOtpPage;
pub use inputtext::InputTextPage;
pub use knob::KnobPage;
pub use listbox::ListboxPage;
pub use menu::MenuPage;
pub use message::MessagePage;
pub use metergroup::MeterGroupPage;
pub use multiselect::MultiSelectPage;
pub use overlaybadge::OverlayBadgePage;
pub use paginator::PaginatorPage;
pub use panel::PanelPage;
pub use password::PasswordPage;
pub use popover::PopoverPage;
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
pub use splitbutton::SplitButtonPage;
pub use splitter::SplitterPage;
pub use stepper::StepperPage;
pub use tabs::TabsPage;
pub use tag::TagPage;
pub use textarea::TextareaPage;
pub use timeline::TimelinePage;
pub use toast::ToastPage;
pub use togglebutton::ToggleButtonPage;
pub use toggleswitch::ToggleSwitchPage;
pub use toolbar::ToolbarPage;
pub use tooltip::TooltipPage;

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
