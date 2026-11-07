//! Страница MultiSelect: демонстрации `basic`, `chips`, `clearicon`, `disabled`, `filled`, `filter`, `fluid`, `group`, `invalid`,
//! `loadingstate`, `sizes`, `template`, `floatlabel`, `iftalabel` из showcase PrimeNG (`apps/showcase/doc/multiselect`).
//! Демонстрации с Angular Forms и виртуальной прокруткой не переносятся; в `template` вместо флагов с CDN — коды стран, без
//! шаблонов `header`/`footer`.

use super::Demo;
use leptos::prelude::*;
use prime_core::{Size, Variant};
use prime_floatlabel::FloatLabel;
use prime_iftalabel::IftaLabel;
use prime_multiselect::{MultiSelect, MultiSelectDisplay, MultiSelectGroup, MultiSelectOption};

const FIELD: &str = "width: 20rem";

fn cities() -> Vec<MultiSelectOption> {
    [("New York", "NY"), ("Rome", "RM"), ("London", "LDN"), ("Istanbul", "IST"), ("Paris", "PRS")]
        .into_iter()
        .map(|(label, value)| MultiSelectOption::new(label, value))
        .collect()
}

fn groups() -> Vec<MultiSelectGroup> {
    let items = |cities: &[&str]| cities.iter().map(|c| MultiSelectOption::new(*c, *c)).collect::<Vec<_>>();
    vec![
        MultiSelectGroup::new("Germany", items(&["Berlin", "Frankfurt", "Hamburg", "Munich"])),
        MultiSelectGroup::new("USA", items(&["Chicago", "Los Angeles", "New York", "San Francisco"])),
        MultiSelectGroup::new("Japan", items(&["Kyoto", "Osaka", "Tokyo", "Yokohama"])),
    ]
}

fn countries() -> Vec<MultiSelectOption> {
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
    .map(|(label, value)| MultiSelectOption::new(label, value))
    .collect()
}

/// Страница `/multiselect`.
#[component]
pub fn MultiSelectPage() -> impl IntoView {
    let signal = || RwSignal::new(Vec::<String>::new());
    let (basic, chips, filter, grouped, filled, fluid) = (signal(), signal(), signal(), signal(), signal(), signal());
    let (disabled, loading, templated, small, normal, large) = (signal(), signal(), signal(), signal(), signal(), signal());
    let (invalid, invalid_filled) = (signal(), signal());
    let clearable = RwSignal::new(vec!["PRS".to_owned()]);
    let (float, ifta) = (signal(), signal());
    let invalid_flag = Signal::derive(move || invalid.get().is_empty());
    let invalid_filled_flag = Signal::derive(move || invalid_filled.get().is_empty());
    let item = Callback::new(|(o, _): (MultiSelectOption, bool)| {
        view! { <div style="display: flex; align-items: center; gap: 0.5rem"><b>{o.value}</b><span>{o.label}</span></div> }.into_any()
    });
    view! {
        <Demo name="basic" title="Basic">
            <MultiSelect options=cities() values=basic placeholder="Select Cities" max_selected_labels=3 style=FIELD />
            <span data-testid="echo">{move || basic.get().join(",")}</span>
        </Demo>
        <Demo name="chips" title="Chips">
            <MultiSelect options=cities() values=chips placeholder="Select Cities" display=MultiSelectDisplay::Chip style=FIELD />
            <span data-testid="echo-chips">{move || chips.get().join(",")}</span>
        </Demo>
        <Demo name="clearicon" title="Clear Icon">
            <MultiSelect options=cities() values=clearable placeholder="Select Cities" show_clear=true style=FIELD />
        </Demo>
        <Demo name="disabled" title="Disabled">
            <MultiSelect options=cities() values=disabled placeholder="Select Cities" disabled=true style=FIELD />
        </Demo>
        <Demo name="filled" title="Filled">
            <MultiSelect options=cities() values=filled placeholder="Select Cities" variant=Variant::Filled style=FIELD />
        </Demo>
        <Demo name="filter" title="Filter">
            <MultiSelect options=cities() values=filter placeholder="Select Cities" filter_placeholder="Search" max_selected_labels=3 style=FIELD />
        </Demo>
        <Demo name="fluid" title="Fluid">
            <MultiSelect options=cities() values=fluid placeholder="Select Cities" fluid=true />
        </Demo>
        <Demo name="group" title="Group">
            <MultiSelect groups=groups() values=grouped placeholder="Select Cities" style=FIELD />
        </Demo>
        <Demo name="invalid" title="Invalid">
            <div style="display: flex; flex-wrap: wrap; gap: 1rem">
                <MultiSelect options=cities() values=invalid invalid=invalid_flag placeholder="Select Cities" style=FIELD />
                <MultiSelect options=cities() values=invalid_filled invalid=invalid_filled_flag variant=Variant::Filled placeholder="Select Cities" style=FIELD />
            </div>
        </Demo>
        <Demo name="loadingstate" title="Loading State">
            <MultiSelect options=cities() values=loading placeholder="Loading..." loading=true style=FIELD />
        </Demo>
        <Demo name="sizes" title="Sizes">
            <div style="display: flex; flex-direction: column; align-items: center; gap: 1rem">
                <MultiSelect options=cities() values=small size=Size::Small placeholder="Small" style=FIELD />
                <MultiSelect options=cities() values=normal placeholder="Normal" style=FIELD />
                <MultiSelect options=cities() values=large size=Size::Large placeholder="Large" style=FIELD />
            </div>
        </Demo>
        <Demo name="template" title="Template">
            <MultiSelect options=countries() values=templated placeholder="Select Countries" display=MultiSelectDisplay::Chip item=item style=FIELD />
        </Demo>
        <Demo name="floatlabel" title="Float Label">
            <FloatLabel>
                <MultiSelect options=cities() values=float input_id="float_ms" style=FIELD />
                <label for="float_ms">"Select Cities"</label>
            </FloatLabel>
        </Demo>
        <Demo name="iftalabel" title="Ifta Label">
            <IftaLabel>
                <MultiSelect options=cities() values=ifta input_id="ifta_ms" style=FIELD />
                <label for="ifta_ms">"Select Cities"</label>
            </IftaLabel>
        </Demo>
    }
}
