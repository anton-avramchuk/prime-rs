//! Страница Listbox: демонстрации `basic`, `multiple`, `group`, `filter`, `checkbox`, `checkmark`, `disabled`, `invalid`,
//! `template` из showcase PrimeNG (`apps/showcase/doc/listbox`). Демонстрации с Angular Forms, виртуальной прокруткой и
//! `dragdrop` не переносятся; в `template` вместо флагов с CDN — коды стран.

use super::Demo;
use leptos::prelude::*;
use prime_listbox::{Listbox, ListboxGroup, ListboxOption};

const FIELD: &str = "width: 14rem";

fn cities() -> Vec<ListboxOption> {
    [("New York", "NY"), ("Rome", "RM"), ("London", "LDN"), ("Istanbul", "IST"), ("Paris", "PRS")]
        .into_iter()
        .map(|(label, value)| ListboxOption::new(label, value))
        .collect()
}

fn groups() -> Vec<ListboxGroup> {
    let items = |cities: &[&str]| cities.iter().map(|c| ListboxOption::new(*c, *c)).collect::<Vec<_>>();
    vec![
        ListboxGroup::new("Germany", items(&["Berlin", "Frankfurt", "Hamburg", "Munich"])),
        ListboxGroup::new("USA", items(&["Chicago", "Los Angeles", "New York", "San Francisco"])),
        ListboxGroup::new("Japan", items(&["Kyoto", "Osaka", "Tokyo", "Yokohama"])),
    ]
}

fn countries() -> Vec<ListboxOption> {
    [
        ("Australia", "AU"),
        ("Brazil", "BR"),
        ("China", "CN"),
        ("Egypt", "EG"),
        ("France", "FR"),
        ("Germany", "DE"),
        ("India", "IN"),
        ("Japan", "JP"),
        ("Spain", "ES"),
        ("United States", "US"),
    ]
    .into_iter()
    .map(|(label, value)| ListboxOption::new(label, value))
    .collect()
}

/// Страница `/listbox`.
#[component]
pub fn ListboxPage() -> impl IntoView {
    let single = || RwSignal::new(None::<String>);
    let many = || RwSignal::new(Vec::<String>::new());
    let (basic, grouped, filtered, checkmark, disabled, invalid, templated) =
        (single(), single(), single(), single(), single(), single(), single());
    let (multiple, checkbox) = (many(), many());
    let invalid_flag = Signal::derive(move || invalid.get().is_none());
    let item = Callback::new(|(o, _): (ListboxOption, bool)| {
        view! { <div style="display: flex; align-items: center; gap: 0.5rem"><b>{o.value}</b><span>{o.label}</span></div> }.into_any()
    });
    view! {
        <h2>"Listbox"</h2>
        <Demo name="basic" title="Basic">
            <Listbox options=cities() value=basic style=FIELD />
            <span data-testid="echo">{move || basic.get().unwrap_or_default()}</span>
        </Demo>
        <Demo name="multiple" title="Multiple">
            <Listbox options=cities() values=multiple multiple=true meta_key_selection=true style=FIELD />
            <span data-testid="echo-multiple">{move || multiple.get().join(",")}</span>
        </Demo>
        <Demo name="group" title="Group">
            <Listbox groups=groups() value=grouped style=FIELD />
        </Demo>
        <Demo name="filter" title="Filter">
            <Listbox options=cities() value=filtered filter=true filter_placeholder="Search" style=FIELD />
        </Demo>
        <Demo name="checkbox" title="Checkbox">
            <Listbox options=cities() values=checkbox multiple=true checkbox=true filter=true style=FIELD />
            <span data-testid="echo-checkbox">{move || checkbox.get().join(",")}</span>
        </Demo>
        <Demo name="checkmark" title="Checkmark">
            <Listbox options=cities() value=checkmark checkmark=true highlight_on_select=false style=FIELD />
        </Demo>
        <Demo name="disabled" title="Disabled">
            <Listbox options=cities() value=disabled disabled=true style=FIELD />
        </Demo>
        <Demo name="invalid" title="Invalid">
            <Listbox options=cities() value=invalid invalid=invalid_flag style=FIELD />
        </Demo>
        <Demo name="template" title="Template">
            <Listbox options=countries() value=templated option=item style=FIELD />
        </Demo>
    }
}
