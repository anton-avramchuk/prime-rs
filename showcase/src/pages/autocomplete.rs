//! Страница AutoComplete: демонстрации `basic`, `dropdown`, `multiple`, `forceselection`, `group`, `clearicon`, `disabled`,
//! `invalid` и `template` из showcase PrimeNG (`apps/showcase/doc/autocomplete`; формы Angular и виртуальная прокрутка не переносятся).

use super::Demo;
use leptos::prelude::*;
use prime_autocomplete::{AutoComplete, AutoCompleteCompleteEvent, AutoCompleteDropdownMode, AutoCompleteOption};

const COUNTRIES: &[&str] = &[
    "Afghanistan",
    "Albania",
    "Algeria",
    "Argentina",
    "Australia",
    "Austria",
    "Belgium",
    "Brazil",
    "Canada",
    "Chile",
    "China",
    "Denmark",
    "Egypt",
    "France",
    "Germany",
    "India",
    "Italy",
    "Japan",
    "Spain",
    "Sweden",
];

const WIDTH: &str = "width: 100%; max-width: 20rem";

fn filtered(query: &str) -> Vec<AutoCompleteOption> {
    let q = query.to_lowercase();
    COUNTRIES.iter().filter(|c| c.to_lowercase().contains(&q)).map(|c| AutoCompleteOption::new(*c)).collect()
}

fn completer(suggestions: RwSignal<Vec<AutoCompleteOption>>) -> Callback<AutoCompleteCompleteEvent> {
    Callback::new(move |e: AutoCompleteCompleteEvent| suggestions.set(filtered(&e.query)))
}

#[component]
fn Basic() -> impl IntoView {
    let (value, suggestions) = (RwSignal::new(String::new()), RwSignal::new(Vec::new()));
    view! {
        <AutoComplete id="basic_ac" suggestions=suggestions value=value delay=100 on_complete=completer(suggestions) placeholder="Search" style=WIDTH />
    }
}

#[component]
fn Dropdown() -> impl IntoView {
    let (value, suggestions) = (RwSignal::new(String::new()), RwSignal::new(Vec::new()));
    view! {
        <AutoComplete
            id="dropdown_ac"
            suggestions=suggestions
            value=value
            dropdown=true
            dropdown_mode=AutoCompleteDropdownMode::Blank
            delay=100
            on_complete=completer(suggestions)
            placeholder="Search"
            style=WIDTH
        />
    }
}

#[component]
fn Multiple() -> impl IntoView {
    let (values, suggestions) = (RwSignal::new(Vec::<String>::new()), RwSignal::new(Vec::new()));
    view! { <AutoComplete id="multiple_ac" suggestions=suggestions values=values multiple=true delay=100 on_complete=completer(suggestions) style=WIDTH /> }
}

#[component]
fn ForceSelection() -> impl IntoView {
    let (value, suggestions) = (RwSignal::new(String::new()), RwSignal::new(Vec::new()));
    view! {
        <AutoComplete
            id="force_ac"
            suggestions=suggestions
            value=value
            force_selection=true
            delay=100
            on_complete=completer(suggestions)
            placeholder="Search"
            style=WIDTH
        />
    }
}

#[component]
fn Grouped() -> impl IntoView {
    let (value, suggestions) = (RwSignal::new(String::new()), RwSignal::new(Vec::new()));
    let on_complete = Callback::new(move |e: AutoCompleteCompleteEvent| {
        let q = e.query.to_lowercase();
        let groups = [("Europe", ["France", "Germany", "Italy", "Spain"]), ("Asia", ["China", "India", "Japan", "Thailand"])];
        suggestions.set(
            groups
                .iter()
                .map(|(label, items)| {
                    AutoCompleteOption::new(*label)
                        .items(items.iter().filter(|c| c.to_lowercase().contains(&q)).map(|c| AutoCompleteOption::new(*c)).collect())
                })
                .filter(|g| !g.items.is_empty())
                .collect(),
        );
    });
    view! { <AutoComplete id="group_ac" suggestions=suggestions value=value group=true dropdown=true delay=100 on_complete=on_complete style=WIDTH /> }
}

#[component]
fn Clear() -> impl IntoView {
    let (value, suggestions) = (RwSignal::new("Canada".to_owned()), RwSignal::new(Vec::new()));
    view! { <AutoComplete id="clear_ac" suggestions=suggestions value=value show_clear=true delay=100 on_complete=completer(suggestions) style=WIDTH /> }
}

#[component]
fn Template() -> impl IntoView {
    let (value, suggestions) = (RwSignal::new(String::new()), RwSignal::new(Vec::new()));
    let item = Callback::new(|o: AutoCompleteOption| view! { <b data-testid="item-tpl">{o.label}</b> }.into_any());
    view! { <AutoComplete id="template_ac" suggestions=suggestions value=value delay=100 on_complete=completer(suggestions) item=item style=WIDTH /> }
}

/// Страница `/autocomplete`.
#[component]
pub fn AutoCompletePage() -> impl IntoView {
    view! {
        <Demo name="basic" title="Basic">
            <Basic />
        </Demo>
        <Demo name="dropdown" title="Dropdown">
            <Dropdown />
        </Demo>
        <Demo name="multiple" title="Multiple">
            <Multiple />
        </Demo>
        <Demo name="forceselection" title="Force Selection">
            <ForceSelection />
        </Demo>
        <Demo name="group" title="Group">
            <Grouped />
        </Demo>
        <Demo name="clearicon" title="Clear Icon">
            <Clear />
        </Demo>
        <Demo name="disabled" title="Disabled">
            <AutoComplete id="disabled_ac" suggestions={Vec::<AutoCompleteOption>::new()} disabled=true placeholder="Disabled" style=WIDTH />
        </Demo>
        <Demo name="invalid" title="Invalid">
            <AutoComplete id="invalid_ac" suggestions={Vec::<AutoCompleteOption>::new()} invalid=true placeholder="Search" style=WIDTH />
        </Demo>
        <Demo name="template" title="Template">
            <Template />
        </Demo>
    }
}
