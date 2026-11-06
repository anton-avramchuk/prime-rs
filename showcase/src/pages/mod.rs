//! Страницы showcase: по странице на компонент и общая оболочка демонстрации.

mod accordion;
mod autocomplete;
mod avatar;
mod badge;
mod blockui;
mod breadcrumb;
mod button;
mod buttongroup;
mod card;
mod carousel;
mod cascadeselect;
mod checkbox;
mod chip;
mod colorpicker;
mod confirmdialog;
mod confirmpopup;
mod contextmenu;
mod dataview;
mod datepicker;
mod dialog;
mod divider;
mod dock;
mod drawer;
mod fieldset;
mod fileupload;
mod floatlabel;
mod fluid;
mod galleria;
mod home;
mod iconfield;
mod iftalabel;
mod image;
mod imagecompare;
mod inplace;
mod inputgroup;
mod inputmask;
mod inputnumber;
mod inputotp;
mod inputtext;
mod knob;
mod listbox;
mod megamenu;
mod menu;
mod menubar;
mod message;
mod metergroup;
mod multiselect;
mod orderlist;
mod overlaybadge;
mod paginator;
mod panel;
mod panelmenu;
mod password;
mod picklist;
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
mod speeddial;
mod splitbutton;
mod splitter;
mod stepper;
mod steps;
mod table;
mod tabs;
mod tag;
mod textarea;
mod tieredmenu;
mod timeline;
mod toast;
mod togglebutton;
mod toggleswitch;
mod toolbar;
mod tooltip;
mod tree;
mod treeselect;
mod treetable;

pub use accordion::AccordionPage;
pub use autocomplete::AutoCompletePage;
pub use avatar::AvatarPage;
pub use badge::BadgePage;
pub use blockui::BlockUiPage;
pub use breadcrumb::BreadcrumbPage;
pub use button::ButtonPage;
pub use buttongroup::ButtonGroupPage;
pub use card::CardPage;
pub use carousel::CarouselPage;
pub use cascadeselect::CascadeSelectPage;
pub use checkbox::CheckboxPage;
pub use chip::ChipPage;
pub use colorpicker::ColorPickerPage;
pub use confirmdialog::ConfirmDialogPage;
pub use confirmpopup::ConfirmPopupPage;
pub use contextmenu::ContextMenuPage;
pub use dataview::DataViewPage;
pub use datepicker::DatePickerPage;
pub use dialog::DialogPage;
pub use divider::DividerPage;
pub use dock::DockPage;
pub use drawer::DrawerPage;
pub use fieldset::FieldsetPage;
pub use fileupload::FileUploadPage;
pub use floatlabel::FloatLabelPage;
pub use fluid::FluidPage;
pub use galleria::GalleriaPage;
pub use home::HomePage;
pub use iconfield::IconFieldPage;
pub use iftalabel::IftaLabelPage;
pub use image::ImagePage;
pub use imagecompare::ImageComparePage;
pub use inplace::InplacePage;
pub use inputgroup::InputGroupPage;
pub use inputmask::InputMaskPage;
pub use inputnumber::InputNumberPage;
pub use inputotp::InputOtpPage;
pub use inputtext::InputTextPage;
pub use knob::KnobPage;
pub use listbox::ListboxPage;
pub use megamenu::MegaMenuPage;
pub use menu::MenuPage;
pub use menubar::MenubarPage;
pub use message::MessagePage;
pub use metergroup::MeterGroupPage;
pub use multiselect::MultiSelectPage;
pub use orderlist::OrderListPage;
pub use overlaybadge::OverlayBadgePage;
pub use paginator::PaginatorPage;
pub use panel::PanelPage;
pub use panelmenu::PanelMenuPage;
pub use password::PasswordPage;
pub use picklist::PickListPage;
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
pub use speeddial::SpeedDialPage;
pub use splitbutton::SplitButtonPage;
pub use splitter::SplitterPage;
pub use stepper::StepperPage;
pub use steps::StepsPage;
pub use table::TablePage;
pub use tabs::TabsPage;
pub use tag::TagPage;
pub use textarea::TextareaPage;
pub use tieredmenu::TieredMenuPage;
pub use timeline::TimelinePage;
pub use toast::ToastPage;
pub use togglebutton::ToggleButtonPage;
pub use toggleswitch::ToggleSwitchPage;
pub use toolbar::ToolbarPage;
pub use tooltip::TooltipPage;
pub use tree::TreePage;
pub use treeselect::TreeSelectPage;
pub use treetable::TreeTablePage;

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
