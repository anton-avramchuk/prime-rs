//! Оболочка страницы, меню и маршруты.

use crate::pages::{
    AccordionPage, AvatarPage, BadgePage, BreadcrumbPage, ButtonGroupPage, ButtonPage, CardPage, CheckboxPage, ChipPage, DividerPage,
    FieldsetPage, FloatLabelPage, FluidPage, HomePage, IconFieldPage, IftaLabelPage, InputGroupPage, InputNumberPage, InputOtpPage,
    InputTextPage, KnobPage, MessagePage, MeterGroupPage, OverlayBadgePage, PanelPage, PasswordPage, ProgressBarPage, ProgressSpinnerPage,
    RadioButtonPage, RatingPage, ScrollPanelPage, ScrollTopPage, SelectButtonPage, SelectPage, SkeletonPage, SliderPage, TagPage,
    TextareaPage, TimelinePage, ToggleButtonPage, ToggleSwitchPage, ToolbarPage,
};
use leptos::hydration::{AutoReload, HydrationScripts};
use leptos::prelude::*;
use leptos_router::StaticSegment;
use leptos_router::components::{A, Route, Router, Routes};

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
                    <nav style="display: flex; gap: 1rem" data-testid="nav">
                        <A href="/">"Home"</A>
                        <A href="/button">"Button"</A>
                        <A href="/badge">"Badge"</A>
                        <A href="/tag">"Tag"</A>
                        <A href="/chip">"Chip"</A>
                        <A href="/avatar">"Avatar"</A>
                        <A href="/divider">"Divider"</A>
                        <A href="/card">"Card"</A>
                        <A href="/inputtext">"InputText"</A>
                        <A href="/checkbox">"Checkbox"</A>
                        <A href="/radiobutton">"RadioButton"</A>
                        <A href="/togglebutton">"ToggleButton"</A>
                        <A href="/selectbutton">"SelectButton"</A>
                        <A href="/rating">"Rating"</A>
                        <A href="/slider">"Slider"</A>
                        <A href="/knob">"Knob"</A>
                        <A href="/inputnumber">"InputNumber"</A>
                        <A href="/select">"Select"</A>
                        <A href="/toggleswitch">"ToggleSwitch"</A>
                        <A href="/message">"Message"</A>
                        <A href="/progressbar">"ProgressBar"</A>
                        <A href="/skeleton">"Skeleton"</A>
                        <A href="/textarea">"Textarea"</A>
                        <A href="/panel">"Panel"</A>
                        <A href="/fieldset">"Fieldset"</A>
                        <A href="/accordion">"Accordion"</A>
                        <A href="/toolbar">"Toolbar"</A>
                        <A href="/scrollpanel">"ScrollPanel"</A>
                        <A href="/progressspinner">"ProgressSpinner"</A>
                        <A href="/inputgroup">"InputGroup"</A>
                        <A href="/buttongroup">"ButtonGroup"</A>
                        <A href="/fluid">"Fluid"</A>
                        <A href="/iconfield">"IconField"</A>
                        <A href="/floatlabel">"FloatLabel"</A>
                        <A href="/iftalabel">"IftaLabel"</A>
                        <A href="/overlaybadge">"OverlayBadge"</A>
                        <A href="/timeline">"Timeline"</A>
                        <A href="/metergroup">"MeterGroup"</A>
                        <A href="/breadcrumb">"Breadcrumb"</A>
                        <A href="/inputotp">"InputOtp"</A>
                        <A href="/scrolltop">"ScrollTop"</A>
                        <A href="/password">"Password"</A>
                    </nav>
                    <button type="button" data-testid="toggle-dark" on:click=|_| toggle_dark()>
                        "Светлая / тёмная схема"
                    </button>
                </header>
                <main>
                    <Routes fallback=|| "Страница не найдена">
                        <Route path=StaticSegment("") view=HomePage />
                        <Route path=StaticSegment("button") view=ButtonPage />
                        <Route path=StaticSegment("badge") view=BadgePage />
                        <Route path=StaticSegment("tag") view=TagPage />
                        <Route path=StaticSegment("chip") view=ChipPage />
                        <Route path=StaticSegment("avatar") view=AvatarPage />
                        <Route path=StaticSegment("divider") view=DividerPage />
                        <Route path=StaticSegment("card") view=CardPage />
                        <Route path=StaticSegment("inputtext") view=InputTextPage />
                        <Route path=StaticSegment("checkbox") view=CheckboxPage />
                        <Route path=StaticSegment("radiobutton") view=RadioButtonPage />
                        <Route path=StaticSegment("togglebutton") view=ToggleButtonPage />
                        <Route path=StaticSegment("selectbutton") view=SelectButtonPage />
                        <Route path=StaticSegment("rating") view=RatingPage />
                        <Route path=StaticSegment("slider") view=SliderPage />
                        <Route path=StaticSegment("knob") view=KnobPage />
                        <Route path=StaticSegment("inputnumber") view=InputNumberPage />
                        <Route path=StaticSegment("select") view=SelectPage />
                        <Route path=StaticSegment("toggleswitch") view=ToggleSwitchPage />
                        <Route path=StaticSegment("message") view=MessagePage />
                        <Route path=StaticSegment("progressbar") view=ProgressBarPage />
                        <Route path=StaticSegment("skeleton") view=SkeletonPage />
                        <Route path=StaticSegment("textarea") view=TextareaPage />
                        <Route path=StaticSegment("panel") view=PanelPage />
                        <Route path=StaticSegment("fieldset") view=FieldsetPage />
                        <Route path=StaticSegment("accordion") view=AccordionPage />
                        <Route path=StaticSegment("toolbar") view=ToolbarPage />
                        <Route path=StaticSegment("scrollpanel") view=ScrollPanelPage />
                        <Route path=StaticSegment("progressspinner") view=ProgressSpinnerPage />
                        <Route path=StaticSegment("inputgroup") view=InputGroupPage />
                        <Route path=StaticSegment("buttongroup") view=ButtonGroupPage />
                        <Route path=StaticSegment("fluid") view=FluidPage />
                        <Route path=StaticSegment("iconfield") view=IconFieldPage />
                        <Route path=StaticSegment("floatlabel") view=FloatLabelPage />
                        <Route path=StaticSegment("iftalabel") view=IftaLabelPage />
                        <Route path=StaticSegment("overlaybadge") view=OverlayBadgePage />
                        <Route path=StaticSegment("timeline") view=TimelinePage />
                        <Route path=StaticSegment("metergroup") view=MeterGroupPage />
                        <Route path=StaticSegment("breadcrumb") view=BreadcrumbPage />
                        <Route path=StaticSegment("inputotp") view=InputOtpPage />
                        <Route path=StaticSegment("scrolltop") view=ScrollTopPage />
                        <Route path=StaticSegment("password") view=PasswordPage />
                    </Routes>
                </main>
            </div>
        </Router>
    }
}
