//! Страница PickList: демонстрации `basic` и `filter` из showcase PrimeNG
//! (`apps/showcase/doc/picklist`; перетаскивание и шаблоны не переносятся).

use super::Demo;
use leptos::prelude::*;
use prime_listbox::ListboxOption;
use prime_picklist::PickList;

fn products() -> RwSignal<Vec<ListboxOption>> {
    let names = ["Bamboo Watch", "Black Watch", "Blue Band", "Blue T-Shirt", "Bracelet", "Brown Purse", "Chakra Bracelet"];
    RwSignal::new(names.iter().map(|n| ListboxOption::new(*n, n.to_lowercase().replace(' ', "-"))).collect())
}

fn names(value: RwSignal<Vec<ListboxOption>>) -> impl IntoView {
    view! { <span>{move || value.get().iter().map(|o| o.label.clone()).collect::<Vec<_>>().join(", ")}</span> }
}

#[component]
fn Basic() -> impl IntoView {
    let (source, target) = (products(), RwSignal::new(Vec::new()));
    view! {
        <div id="basic_picklist" style="width: 100%">
            <PickList source=source target=target source_header="Available" target_header="Selected" scroll_height="18rem" />
            <p data-testid="basic-target" style="margin: 0.5rem 0 0">{names(target)}</p>
        </div>
    }
}

#[component]
fn Filter() -> impl IntoView {
    let (source, target) = (products(), RwSignal::new(Vec::new()));
    view! {
        <div id="filter_picklist" style="width: 100%">
            <PickList
                source=source
                target=target
                source_header="Available"
                target_header="Selected"
                filter=true
                source_filter_placeholder="Search by name"
                target_filter_placeholder="Search by name"
                scroll_height="18rem"
            />
        </div>
    }
}

/// Страница `/picklist`.
#[component]
pub fn PickListPage() -> impl IntoView {
    view! {
        <Demo name="basic" title="Basic">
            <Basic />
        </Demo>
        <Demo name="filter" title="Filter">
            <Filter />
        </Demo>
    }
}
