//! Оболочка страницы, меню и маршруты.

use crate::doc::{DOC_CSS, DocNav, DocPage};
use crate::pages::{
    AccordionPage, AutoCompletePage, AvatarPage, BadgePage, BlockUiPage, BreadcrumbPage, ButtonGroupPage, ButtonPage, CardPage,
    CarouselPage, CascadeSelectPage, CheckboxPage, ChipPage, ColorPickerPage, ConfirmDialogPage, ConfirmPopupPage, ContextMenuPage,
    DataViewPage, DatePickerPage, DialogPage, DividerPage, DockPage, DrawerPage, FieldsetPage, FileUploadPage, FloatLabelPage, FluidPage,
    GalleriaPage, HomePage, IconFieldPage, IftaLabelPage, ImageComparePage, ImagePage, InplacePage, InputGroupPage, InputMaskPage,
    InputNumberPage, InputOtpPage, InputTextPage, KnobPage, ListboxPage, MegaMenuPage, MenuPage, MenubarPage, MessagePage, MeterGroupPage,
    MultiSelectPage, OrderListPage, OverlayBadgePage, PaginatorPage, PanelMenuPage, PanelPage, PasswordPage, PickListPage, PopoverPage,
    ProgressBarPage, ProgressSpinnerPage, RadioButtonPage, RatingPage, ScrollPanelPage, ScrollTopPage, SelectButtonPage, SelectPage,
    SkeletonPage, SliderPage, SpeedDialPage, SplitButtonPage, SplitterPage, StepperPage, StepsPage, TablePage, TabsPage, TagPage,
    TextareaPage, TieredMenuPage, TimelinePage, ToastPage, ToggleButtonPage, ToggleSwitchPage, ToolbarPage, TooltipPage, TreePage,
    TreeSelectPage, TreeTablePage,
};
use leptos::hydration::{AutoReload, HydrationScripts};
use leptos::prelude::*;
use leptos_router::StaticSegment;
use leptos_router::components::{Route, Router, Routes};

const DARK_CLASS: &str = "p-dark";

/// HTML-оболочка: тема подключается встроенным `<style>` (базовый CSS и CSS компонентов).
pub fn shell(options: LeptosOptions) -> impl IntoView {
    let css = prime_theme::stylesheet(&[
        "button",
        "badge",
        "tag",
        "chip",
        "avatar",
        "divider",
        "card",
        "inputtext",
        "checkbox",
        "radiobutton",
        "togglebutton",
        "selectbutton",
        "rating",
        "slider",
        "knob",
        "inputnumber",
        "select",
        "iconfield",
        "toggleswitch",
        "message",
        "progressbar",
        "skeleton",
        "textarea",
        "panel",
        "fieldset",
        "accordion",
        "toolbar",
        "scrollpanel",
        "progressspinner",
        "inputgroup",
        "buttongroup",
        "floatlabel",
        "iftalabel",
        "overlaybadge",
        "timeline",
        "metergroup",
        "breadcrumb",
        "inputotp",
        "scrolltop",
        "password",
        "inplace",
        "splitter",
        "tabs",
        "popover",
        "drawer",
        "dialog",
        "tooltip",
        "toast",
        "menu",
        "listbox",
        "multiselect",
        "paginator",
        "stepper",
        "imagecompare",
        "blockui",
        "splitbutton",
        "dock",
        "image",
        "confirmpopup",
        "confirmdialog",
        "speeddial",
        "carousel",
        "tieredmenu",
        "steps",
        "contextmenu",
        "menubar",
        "panelmenu",
        "megamenu",
        "tree",
        "treeselect",
        "cascadeselect",
        "autocomplete",
        "inputmask",
        "colorpicker",
        "galleria",
        "fileupload",
        "orderlist",
        "picklist",
        "datepicker",
        "dataview",
        "datatable",
        "treetable",
    ]);
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
    <style>{DOC_CSS}</style>
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

/// Корневой компонент showcase.
#[component]
pub fn App() -> impl IntoView {
    view! {
            <Router>
                <div data-testid="app" style="font-family: sans-serif; padding: 1.5rem; background: var(--p-content-background); color: var(--p-text-color); min-height: 100vh">
                    <header style="display: flex; align-items: center; justify-content: space-between">
                        <h1>"prime-rs"</h1>
                                            <button type="button" data-testid="toggle-dark" on:click=|_| toggle_dark()>
                            "Светлая / тёмная схема"
                        </button>
                    </header>
                    <div class="doc-shell">
    <DocNav />
    <main>
                        <Routes fallback=|| "Страница не найдена">
                            <Route path=StaticSegment("") view=HomePage />
                            <Route path=StaticSegment("button") view=|| view! { <DocPage name="button"><ButtonPage /></DocPage> } />
                            <Route path=StaticSegment("badge") view=|| view! { <DocPage name="badge"><BadgePage /></DocPage> } />
                            <Route path=StaticSegment("tag") view=|| view! { <DocPage name="tag"><TagPage /></DocPage> } />
                            <Route path=StaticSegment("chip") view=|| view! { <DocPage name="chip"><ChipPage /></DocPage> } />
                            <Route path=StaticSegment("avatar") view=|| view! { <DocPage name="avatar"><AvatarPage /></DocPage> } />
                            <Route path=StaticSegment("divider") view=|| view! { <DocPage name="divider"><DividerPage /></DocPage> } />
                            <Route path=StaticSegment("card") view=|| view! { <DocPage name="card"><CardPage /></DocPage> } />
                            <Route path=StaticSegment("inputtext") view=|| view! { <DocPage name="inputtext"><InputTextPage /></DocPage> } />
                            <Route path=StaticSegment("checkbox") view=|| view! { <DocPage name="checkbox"><CheckboxPage /></DocPage> } />
                            <Route path=StaticSegment("radiobutton") view=|| view! { <DocPage name="radiobutton"><RadioButtonPage /></DocPage> } />
                            <Route path=StaticSegment("togglebutton") view=|| view! { <DocPage name="togglebutton"><ToggleButtonPage /></DocPage> } />
                            <Route path=StaticSegment("selectbutton") view=|| view! { <DocPage name="selectbutton"><SelectButtonPage /></DocPage> } />
                            <Route path=StaticSegment("rating") view=|| view! { <DocPage name="rating"><RatingPage /></DocPage> } />
                            <Route path=StaticSegment("slider") view=|| view! { <DocPage name="slider"><SliderPage /></DocPage> } />
                            <Route path=StaticSegment("knob") view=|| view! { <DocPage name="knob"><KnobPage /></DocPage> } />
                            <Route path=StaticSegment("inputnumber") view=|| view! { <DocPage name="inputnumber"><InputNumberPage /></DocPage> } />
                            <Route path=StaticSegment("select") view=|| view! { <DocPage name="select"><SelectPage /></DocPage> } />
                            <Route path=StaticSegment("toggleswitch") view=|| view! { <DocPage name="toggleswitch"><ToggleSwitchPage /></DocPage> } />
                            <Route path=StaticSegment("message") view=|| view! { <DocPage name="message"><MessagePage /></DocPage> } />
                            <Route path=StaticSegment("progressbar") view=|| view! { <DocPage name="progressbar"><ProgressBarPage /></DocPage> } />
                            <Route path=StaticSegment("skeleton") view=|| view! { <DocPage name="skeleton"><SkeletonPage /></DocPage> } />
                            <Route path=StaticSegment("textarea") view=|| view! { <DocPage name="textarea"><TextareaPage /></DocPage> } />
                            <Route path=StaticSegment("panel") view=|| view! { <DocPage name="panel"><PanelPage /></DocPage> } />
                            <Route path=StaticSegment("fieldset") view=|| view! { <DocPage name="fieldset"><FieldsetPage /></DocPage> } />
                            <Route path=StaticSegment("accordion") view=|| view! { <DocPage name="accordion"><AccordionPage /></DocPage> } />
                            <Route path=StaticSegment("toolbar") view=|| view! { <DocPage name="toolbar"><ToolbarPage /></DocPage> } />
                            <Route path=StaticSegment("scrollpanel") view=|| view! { <DocPage name="scrollpanel"><ScrollPanelPage /></DocPage> } />
                            <Route path=StaticSegment("progressspinner") view=|| view! { <DocPage name="progressspinner"><ProgressSpinnerPage /></DocPage> } />
                            <Route path=StaticSegment("inputgroup") view=|| view! { <DocPage name="inputgroup"><InputGroupPage /></DocPage> } />
                            <Route path=StaticSegment("buttongroup") view=|| view! { <DocPage name="buttongroup"><ButtonGroupPage /></DocPage> } />
                            <Route path=StaticSegment("fluid") view=|| view! { <DocPage name="fluid"><FluidPage /></DocPage> } />
                            <Route path=StaticSegment("iconfield") view=|| view! { <DocPage name="iconfield"><IconFieldPage /></DocPage> } />
                            <Route path=StaticSegment("floatlabel") view=|| view! { <DocPage name="floatlabel"><FloatLabelPage /></DocPage> } />
                            <Route path=StaticSegment("iftalabel") view=|| view! { <DocPage name="iftalabel"><IftaLabelPage /></DocPage> } />
                            <Route path=StaticSegment("overlaybadge") view=|| view! { <DocPage name="overlaybadge"><OverlayBadgePage /></DocPage> } />
                            <Route path=StaticSegment("timeline") view=|| view! { <DocPage name="timeline"><TimelinePage /></DocPage> } />
                            <Route path=StaticSegment("metergroup") view=|| view! { <DocPage name="metergroup"><MeterGroupPage /></DocPage> } />
                            <Route path=StaticSegment("breadcrumb") view=|| view! { <DocPage name="breadcrumb"><BreadcrumbPage /></DocPage> } />
                            <Route path=StaticSegment("inputotp") view=|| view! { <DocPage name="inputotp"><InputOtpPage /></DocPage> } />
                            <Route path=StaticSegment("scrolltop") view=|| view! { <DocPage name="scrolltop"><ScrollTopPage /></DocPage> } />
                            <Route path=StaticSegment("password") view=|| view! { <DocPage name="password"><PasswordPage /></DocPage> } />
                            <Route path=StaticSegment("inplace") view=|| view! { <DocPage name="inplace"><InplacePage /></DocPage> } />
                            <Route path=StaticSegment("splitter") view=|| view! { <DocPage name="splitter"><SplitterPage /></DocPage> } />
                            <Route path=StaticSegment("tabs") view=|| view! { <DocPage name="tabs"><TabsPage /></DocPage> } />
                            <Route path=StaticSegment("popover") view=|| view! { <DocPage name="popover"><PopoverPage /></DocPage> } />
                            <Route path=StaticSegment("drawer") view=|| view! { <DocPage name="drawer"><DrawerPage /></DocPage> } />
                            <Route path=StaticSegment("dialog") view=|| view! { <DocPage name="dialog"><DialogPage /></DocPage> } />
                            <Route path=StaticSegment("tooltip") view=|| view! { <DocPage name="tooltip"><TooltipPage /></DocPage> } />
                            <Route path=StaticSegment("toast") view=|| view! { <DocPage name="toast"><ToastPage /></DocPage> } />
                            <Route path=StaticSegment("menu") view=|| view! { <DocPage name="menu"><MenuPage /></DocPage> } />
                            <Route path=StaticSegment("listbox") view=|| view! { <DocPage name="listbox"><ListboxPage /></DocPage> } />
                            <Route path=StaticSegment("multiselect") view=|| view! { <DocPage name="multiselect"><MultiSelectPage /></DocPage> } />
                            <Route path=StaticSegment("paginator") view=|| view! { <DocPage name="paginator"><PaginatorPage /></DocPage> } />
                            <Route path=StaticSegment("stepper") view=|| view! { <DocPage name="stepper"><StepperPage /></DocPage> } />
                            <Route path=StaticSegment("imagecompare") view=|| view! { <DocPage name="imagecompare"><ImageComparePage /></DocPage> } />
                            <Route path=StaticSegment("blockui") view=|| view! { <DocPage name="blockui"><BlockUiPage /></DocPage> } />
                            <Route path=StaticSegment("splitbutton") view=|| view! { <DocPage name="splitbutton"><SplitButtonPage /></DocPage> } />
                            <Route path=StaticSegment("dock") view=|| view! { <DocPage name="dock"><DockPage /></DocPage> } />
                            <Route path=StaticSegment("image") view=|| view! { <DocPage name="image"><ImagePage /></DocPage> } />
                            <Route path=StaticSegment("confirmpopup") view=|| view! { <DocPage name="confirmpopup"><ConfirmPopupPage /></DocPage> } />
                            <Route path=StaticSegment("confirmdialog") view=|| view! { <DocPage name="confirmdialog"><ConfirmDialogPage /></DocPage> } />
                            <Route path=StaticSegment("speeddial") view=|| view! { <DocPage name="speeddial"><SpeedDialPage /></DocPage> } />
                            <Route path=StaticSegment("carousel") view=|| view! { <DocPage name="carousel"><CarouselPage /></DocPage> } />
                            <Route path=StaticSegment("tieredmenu") view=|| view! { <DocPage name="tieredmenu"><TieredMenuPage /></DocPage> } />
                            <Route path=StaticSegment("steps") view=|| view! { <DocPage name="steps"><StepsPage /></DocPage> } />
                            <Route path=StaticSegment("contextmenu") view=|| view! { <DocPage name="contextmenu"><ContextMenuPage /></DocPage> } />
                            <Route path=StaticSegment("menubar") view=|| view! { <DocPage name="menubar"><MenubarPage /></DocPage> } />
                            <Route path=StaticSegment("panelmenu") view=|| view! { <DocPage name="panelmenu"><PanelMenuPage /></DocPage> } />
                            <Route path=StaticSegment("megamenu") view=|| view! { <DocPage name="megamenu"><MegaMenuPage /></DocPage> } />
                            <Route path=StaticSegment("tree") view=|| view! { <DocPage name="tree"><TreePage /></DocPage> } />
                            <Route path=StaticSegment("treeselect") view=|| view! { <DocPage name="treeselect"><TreeSelectPage /></DocPage> } />
                            <Route path=StaticSegment("cascadeselect") view=|| view! { <DocPage name="cascadeselect"><CascadeSelectPage /></DocPage> } />
                            <Route path=StaticSegment("autocomplete") view=|| view! { <DocPage name="autocomplete"><AutoCompletePage /></DocPage> } />
                            <Route path=StaticSegment("inputmask") view=|| view! { <DocPage name="inputmask"><InputMaskPage /></DocPage> } />
                            <Route path=StaticSegment("colorpicker") view=|| view! { <DocPage name="colorpicker"><ColorPickerPage /></DocPage> } />
                            <Route path=StaticSegment("galleria") view=|| view! { <DocPage name="galleria"><GalleriaPage /></DocPage> } />
                            <Route path=StaticSegment("fileupload") view=|| view! { <DocPage name="fileupload"><FileUploadPage /></DocPage> } />
                            <Route path=StaticSegment("orderlist") view=|| view! { <DocPage name="orderlist"><OrderListPage /></DocPage> } />
                            <Route path=StaticSegment("picklist") view=|| view! { <DocPage name="picklist"><PickListPage /></DocPage> } />
                            <Route path=StaticSegment("datepicker") view=|| view! { <DocPage name="datepicker"><DatePickerPage /></DocPage> } />
                            <Route path=StaticSegment("dataview") view=|| view! { <DocPage name="dataview"><DataViewPage /></DocPage> } />
                            <Route path=StaticSegment("table") view=|| view! { <DocPage name="table"><TablePage /></DocPage> } />
                            <Route path=StaticSegment("treetable") view=|| view! { <DocPage name="treetable"><TreeTablePage /></DocPage> } />
                        </Routes>
                    </main>
    </div>
                </div>
            </Router>
        }
}
