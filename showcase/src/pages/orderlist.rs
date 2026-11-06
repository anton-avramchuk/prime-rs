//! Страница OrderList: демонстрации `basic`, `filter` и `controls` (кнопки справа) по showcase PrimeNG
//! (`apps/showcase/doc/orderlist`; перетаскивание и шаблоны не переносятся).

use super::Demo;
use leptos::prelude::*;
use prime_listbox::ListboxOption;
use prime_orderlist::{ControlsPosition, OrderList};

fn products() -> RwSignal<Vec<ListboxOption>> {
    let names =
        ["Bamboo Watch", "Black Watch", "Blue Band", "Blue T-Shirt", "Bracelet", "Brown Purse", "Chakra Bracelet", "Galaxy Earrings"];
    RwSignal::new(names.iter().map(|n| ListboxOption::new(*n, n.to_lowercase().replace(' ', "-"))).collect())
}

fn order_text(value: RwSignal<Vec<ListboxOption>>) -> impl IntoView {
    view! { <span>{move || value.get().iter().map(|o| o.label.clone()).collect::<Vec<_>>().join(", ")}</span> }
}

#[component]
fn Basic() -> impl IntoView {
    let value = products();
    view! {
        <div id="basic_orderlist" style="width: 100%; max-width: 30rem">
            <OrderList value=value scroll_height="18rem" />
            <p data-testid="basic-order" style="margin: 0.5rem 0 0">{order_text(value)}</p>
        </div>
    }
}

#[component]
fn Filter() -> impl IntoView {
    let value = products();
    view! {
        <div id="filter_orderlist" style="width: 100%; max-width: 30rem">
            <OrderList value=value filter=true filter_placeholder="Filter by name" scroll_height="18rem" />
        </div>
    }
}

#[component]
fn Controls() -> impl IntoView {
    let value = products();
    view! {
        <div id="controls_orderlist" style="width: 100%; max-width: 30rem">
            <OrderList value=value controls_position=ControlsPosition::Right striped_rows=true scroll_height="12rem" />
        </div>
    }
}

/// Страница `/orderlist`.
#[component]
pub fn OrderListPage() -> impl IntoView {
    view! {
        <h2>"OrderList"</h2>
        <Demo name="basic" title="Basic">
            <Basic />
        </Demo>
        <Demo name="filter" title="Filter">
            <Filter />
        </Demo>
        <Demo name="controls" title="Controls">
            <Controls />
        </Demo>
    }
}
