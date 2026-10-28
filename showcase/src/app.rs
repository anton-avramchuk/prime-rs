//! Оболочка страницы, меню и маршруты.

use crate::pages::{
    AccordionPage, AvatarPage, BadgePage, ButtonPage, CardPage, CheckboxPage, ChipPage, DividerPage, FieldsetPage, HomePage, InputTextPage,
    MessagePage, PanelPage, ProgressBarPage, RadioButtonPage, SkeletonPage, TagPage, TextareaPage, ToggleSwitchPage,
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
        "toggleswitch",
        "message",
        "progressbar",
        "skeleton",
        "textarea",
        "panel",
        "fieldset",
        "accordion",
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
                        <A href="/toggleswitch">"ToggleSwitch"</A>
                        <A href="/message">"Message"</A>
                        <A href="/progressbar">"ProgressBar"</A>
                        <A href="/skeleton">"Skeleton"</A>
                        <A href="/textarea">"Textarea"</A>
                        <A href="/panel">"Panel"</A>
                        <A href="/fieldset">"Fieldset"</A>
                        <A href="/accordion">"Accordion"</A>
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
                        <Route path=StaticSegment("toggleswitch") view=ToggleSwitchPage />
                        <Route path=StaticSegment("message") view=MessagePage />
                        <Route path=StaticSegment("progressbar") view=ProgressBarPage />
                        <Route path=StaticSegment("skeleton") view=SkeletonPage />
                        <Route path=StaticSegment("textarea") view=TextareaPage />
                        <Route path=StaticSegment("panel") view=PanelPage />
                        <Route path=StaticSegment("fieldset") view=FieldsetPage />
                        <Route path=StaticSegment("accordion") view=AccordionPage />
                    </Routes>
                </main>
            </div>
        </Router>
    }
}
