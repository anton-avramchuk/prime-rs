//! Страница Select: демонстрации `basic`, `checkmark`, `clearicon`, `disabled`, `filled`, `filter`, `fluid`, `group`, `invalid`,
//! `loadingstate`, `sizes`, `template` из showcase PrimeNG (`apps/showcase/doc/select`). Демонстрации с Angular Forms, FloatLabel,
//! IftaLabel, виртуальной прокруткой, `editable` и `customfilter` не переносятся; в `template` вместо флагов с CDN — коды стран.

use super::Demo;
use leptos::prelude::*;
use prime_core::{Size, Variant};
use prime_select::{Select, SelectGroup, SelectOption};

const FIELD: &str = "width: 14rem";

fn cities() -> Vec<SelectOption> {
    [("New York", "NY"), ("Rome", "RM"), ("London", "LDN"), ("Istanbul", "IST"), ("Paris", "PRS")]
        .into_iter()
        .map(|(label, value)| SelectOption::new(label, value))
        .collect()
}

fn groups() -> Vec<SelectGroup> {
    let items = |cities: &[(&str, &str)]| cities.iter().map(|(l, v)| SelectOption::new(*l, *v)).collect::<Vec<_>>();
    vec![
        SelectGroup::new(
            "Germany",
            items(&[("Berlin", "Berlin"), ("Frankfurt", "Frankfurt"), ("Hamburg", "Hamburg"), ("Munich", "Munich")]),
        ),
        SelectGroup::new(
            "USA",
            items(&[("Chicago", "Chicago"), ("Los Angeles", "Los Angeles"), ("New York", "New York"), ("San Francisco", "San Francisco")]),
        ),
        SelectGroup::new("Japan", items(&[("Kyoto", "Kyoto"), ("Osaka", "Osaka"), ("Tokyo", "Tokyo"), ("Yokohama", "Yokohama")])),
    ]
}

fn countries() -> Vec<SelectOption> {
    [
        ("Australia", "AU"),
        ("Brazil", "BR"),
        ("China", "CN"),
        ("Egypt", "EG"),
        ("France", "FR"),
        ("Germany", "DE"),
        ("India", "IN"),
        ("Japan", "JP"),
    ]
    .into_iter()
    .map(|(label, value)| SelectOption::new(label, value))
    .collect()
}

/// Страница `/select`.
#[component]
pub fn SelectPage() -> impl IntoView {
    let signal = || RwSignal::new(None::<String>);
    let (basic, checkmark, clearable, filter, grouped) = (signal(), signal(), RwSignal::new(Some("PRS".to_owned())), signal(), signal());
    let (filled, fluid, invalid, invalid_filled, templated) = (signal(), signal(), signal(), signal(), signal());
    let (small, normal, large, disabled, loading) = (signal(), signal(), signal(), signal(), signal());
    let (invalid_flag, invalid_filled_flag) =
        (Signal::derive(move || invalid.get().is_none()), Signal::derive(move || invalid_filled.get().is_none()));
    let item = Callback::new(|o: SelectOption| {
        view! { <div style="display: flex; align-items: center; gap: 0.5rem"><b>{o.value}</b><span>{o.label}</span></div> }.into_any()
    });
    let selected = Callback::new(|o: SelectOption| {
        view! { <div style="display: flex; align-items: center; gap: 0.5rem"><b>{o.value}</b><span>{o.label}</span></div> }.into_any()
    });
    view! {
        <h2>"Select"</h2>
        <Demo name="basic" title="Basic">
            <Select options=cities() value=basic placeholder="Select a City" style=FIELD />
            <span data-testid="echo">{move || basic.get().unwrap_or_default()}</span>
        </Demo>
        <Demo name="checkmark" title="Checkmark">
            <Select options=cities() value=checkmark placeholder="Select a City" checkmark=true style=FIELD />
        </Demo>
        <Demo name="clearicon" title="Clear Icon">
            <Select options=cities() value=clearable placeholder="Select a City" show_clear=true style=FIELD />
        </Demo>
        <Demo name="disabled" title="Disabled">
            <Select options=cities() value=disabled placeholder="Select a City" disabled=true style=FIELD />
        </Demo>
        <Demo name="filled" title="Filled">
            <Select options=cities() value=filled placeholder="Select a City" variant=Variant::Filled style=FIELD />
        </Demo>
        <Demo name="filter" title="Filter">
            <Select options=cities() value=filter placeholder="Select a City" filter=true filter_placeholder="Search" style=FIELD />
        </Demo>
        <Demo name="fluid" title="Fluid">
            <Select options=cities() value=fluid placeholder="Select a City" fluid=true />
        </Demo>
        <Demo name="group" title="Group">
            <Select groups=groups() value=grouped placeholder="Select a City" style=FIELD />
        </Demo>
        <Demo name="invalid" title="Invalid">
            <div style="display: flex; flex-wrap: wrap; gap: 1rem">
                <Select options=cities() value=invalid invalid=invalid_flag placeholder="Select a City" style=FIELD />
                <Select options=cities() value=invalid_filled invalid=invalid_filled_flag variant=Variant::Filled placeholder="Select a City" style=FIELD />
            </div>
        </Demo>
        <Demo name="loadingstate" title="Loading State">
            <Select options=cities() value=loading placeholder="Loading..." loading=true style=FIELD />
        </Demo>
        <Demo name="sizes" title="Sizes">
            <div style="display: flex; flex-direction: column; align-items: center; gap: 1rem">
                <Select options=cities() value=small size=Size::Small placeholder="Small" style=FIELD />
                <Select options=cities() value=normal placeholder="Normal" style=FIELD />
                <Select options=cities() value=large size=Size::Large placeholder="Large" style=FIELD />
            </div>
        </Demo>
        <Demo name="template" title="Template">
            <Select options=countries() value=templated placeholder="Select a Country" filter=true item_template=item selected_item_template=selected style=FIELD />
        </Demo>
    }
}
