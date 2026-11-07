//! Страница CascadeSelect: демонстрации `basic`, `clearicon`, `disabled`, `invalid`, `loading` и `template` из showcase PrimeNG
//! (`apps/showcase/doc/cascadeselect`; формы Angular не переносятся).

use super::Demo;
use leptos::prelude::*;
use prime_cascadeselect::{CascadeOption, CascadeOptionContext, CascadeSelect};

fn city(label: &str, code: &str) -> CascadeOption {
    CascadeOption::new(label).value(code)
}

fn countries() -> Vec<CascadeOption> {
    vec![
        CascadeOption::new("Australia").children(vec![
            CascadeOption::new("New South Wales").children(vec![
                city("Sydney", "A-SY"),
                city("Newcastle", "A-NE"),
                city("Wollongong", "A-WO"),
            ]),
            CascadeOption::new("Queensland").children(vec![city("Brisbane", "A-BR"), city("Townsville", "A-TO")]),
        ]),
        CascadeOption::new("Canada").children(vec![
            CascadeOption::new("Quebec").children(vec![city("Montreal", "C-MO"), city("Quebec City", "C-QU")]),
            CascadeOption::new("Ontario").children(vec![city("Ottawa", "C-OT"), city("Toronto", "C-TO")]),
        ]),
        CascadeOption::new("United States").children(vec![
            CascadeOption::new("California").children(vec![
                city("Los Angeles", "U-LA"),
                city("San Diego", "U-SD"),
                city("San Francisco", "U-SF"),
            ]),
            CascadeOption::new("Florida").children(vec![
                city("Jacksonville", "U-JA"),
                city("Miami", "U-MI"),
                city("Tampa", "U-TA"),
                city("Orlando", "U-OR"),
            ]),
        ]),
    ]
}

const WIDTH: &str = "width: 100%; max-width: 14rem";

#[component]
fn Basic() -> impl IntoView {
    let value = RwSignal::new(None::<String>);
    view! { <CascadeSelect id="basic_select" options=countries() value=value placeholder="Select a City" style=WIDTH /> }
}

#[component]
fn Clear() -> impl IntoView {
    let value = RwSignal::new(Some("A-SY".to_owned()));
    view! { <CascadeSelect id="clear_select" options=countries() value=value show_clear=true placeholder="Select a City" style=WIDTH /> }
}

#[component]
fn Template() -> impl IntoView {
    let value = RwSignal::new(None::<String>);
    let option = Callback::new(|ctx: CascadeOptionContext| {
        view! { <span data-testid="option-tpl" style="display: flex; gap: 0.5rem">{if ctx.group { "▸ " } else { "• " }}{ctx.label}</span> }
            .into_any()
    });
    view! { <CascadeSelect id="template_select" options=countries() value=value option=option placeholder="Select a City" style=WIDTH /> }
}

/// Страница `/cascadeselect`.
#[component]
pub fn CascadeSelectPage() -> impl IntoView {
    view! {
        <Demo name="basic" title="Basic">
            <Basic />
        </Demo>
        <Demo name="clearicon" title="Clear Icon">
            <Clear />
        </Demo>
        <Demo name="disabled" title="Disabled">
            <CascadeSelect id="disabled_select" options=countries() disabled=true placeholder="Disabled" style=WIDTH />
        </Demo>
        <Demo name="invalid" title="Invalid">
            <CascadeSelect id="invalid_select" options=countries() invalid=true placeholder="Select a City" style=WIDTH />
        </Demo>
        <Demo name="loading" title="Loading">
            <CascadeSelect id="loading_select" options=countries() loading=true placeholder="Loading" style=WIDTH />
        </Demo>
        <Demo name="template" title="Template">
            <Template />
        </Demo>
    }
}
