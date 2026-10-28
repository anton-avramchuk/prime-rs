//! Страница Toolbar: демонстрации `basic` и `custom` из showcase PrimeNG (`apps/showcase/doc/toolbar`).
//!
//! В `basic` вместо SplitButton и IconField (их ещё нет в порте) — обычная кнопка «Save» и поле InputText с иконкой поиска;
//! иконки — SVG из `prime-icons` вместо шрифта PrimeIcons.

use super::Demo;
use leptos::prelude::*;
use prime_avatar::Avatar;
use prime_button::{Button, ButtonSeverity};
use prime_core::Size;
use prime_icons::{PlusIcon, SearchIcon, TrashIcon, UploadIcon};
use prime_inputtext::InputText;
use prime_toolbar::Toolbar;

/// Страница `/toolbar`.
#[component]
pub fn ToolbarPage() -> impl IntoView {
    view! {
        <h2>"Toolbar"</h2>
        <Demo name="basic" title="Basic">
            <div style="width: 100%">
                <Toolbar
                    start_template=ViewFn::from(|| {
                        view! {
                            <Button
                                icon=ViewFn::from(|| view! { <PlusIcon /> })
                                text=true
                                severity=ButtonSeverity::Secondary
                                style="margin-right: 0.5rem"
                                aria_label="Add"
                            />
                            <Button
                                icon=ViewFn::from(|| view! { <TrashIcon /> })
                                text=true
                                severity=ButtonSeverity::Secondary
                                style="margin-right: 0.5rem"
                                aria_label="Delete"
                            />
                            <Button icon=ViewFn::from(|| view! { <UploadIcon /> }) text=true severity=ButtonSeverity::Secondary aria_label="Upload" />
                        }
                    })
                    center_template=ViewFn::from(|| {
                        view! {
                            <span style="display: inline-flex; align-items: center; gap: 0.5rem">
                                <SearchIcon />
                                <InputText placeholder="Search" />
                            </span>
                        }
                    })
                    end_template=ViewFn::from(|| view! { <Button label="Save" /> })
                />
            </div>
        </Demo>
        <Demo name="custom" title="Custom">
            <div style="width: 100%">
                <Toolbar
                    style="border-radius: 3rem; padding: 1rem 1rem 1rem 1.5rem"
                    start_template=ViewFn::from(|| {
                        view! {
                            <div style="display: flex; align-items: center; gap: 0.5rem">
                                <svg width="31" height="33" viewBox="0 0 31 33" fill="none" xmlns="http://www.w3.org/2000/svg" style="width: 2rem; margin-right: 1rem">
                                    <path d="M15.1934 0L0.0391235 5.38288L2.35052 25.3417L15.1934 32.427L28.0364 25.3417L30.3478 5.38288L15.1934 0Z" fill="var(--p-primary-color)" />
                                    <path d="M13.0167 5.68861L11.7244 8.7568L13.8244 14.8932H14.7936V5.68861H13.0167ZM15.4397 5.68861V14.8932H16.5706L18.5091 8.7568L17.2167 5.68861H15.4397Z" fill="var(--p-content-background)" />
                                </svg>
                                <Button label="Files" text=true plain=true />
                                <Button label="Edit" text=true plain=true />
                                <Button label="View" text=true plain=true />
                            </div>
                        }
                    })
                    end_template=ViewFn::from(|| {
                        view! {
                            <div style="display: flex; align-items: center; gap: 0.5rem">
                                <Button label="Share" severity=ButtonSeverity::Contrast size=Size::Small />
                                <Avatar image="https://primefaces.org/cdn/primeng/images/demo/avatar/amyelsner.png" style="width: 32px; height: 32px" />
                            </div>
                        }
                    })
                />
            </div>
        </Demo>
    }
}
