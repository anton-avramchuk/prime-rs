//! Страница DataView: демонстрации `basic`, `layout`, `pagination`, `sorting` и `loading` из showcase PrimeNG
//! (`apps/showcase/doc/dataview`; сортировка и фильтр задаются сравнением и предикатом, шаблоны — функциями).

use super::Demo;
use leptos::prelude::*;
use prime_button::Button;
use prime_dataview::{DataView, DataViewLayout};

#[derive(Clone, Debug)]
struct Product {
    name: &'static str,
    category: &'static str,
    price: u32,
}

fn products() -> Vec<Product> {
    let data = [
        ("Bamboo Watch", "Accessories", 65),
        ("Black Watch", "Accessories", 72),
        ("Blue Band", "Fitness", 79),
        ("Blue T-Shirt", "Clothing", 29),
        ("Bracelet", "Accessories", 15),
        ("Brown Purse", "Accessories", 120),
        ("Chakra Bracelet", "Accessories", 32),
        ("Galaxy Earrings", "Accessories", 34),
        ("Game Controller", "Electronics", 99),
        ("Gaming Set", "Electronics", 299),
        ("Gold Phone Case", "Accessories", 24),
        ("Green Earbuds", "Electronics", 89),
    ];
    data.into_iter().map(|(name, category, price)| Product { name, category, price }).collect()
}

fn basic_items() -> Vec<Product> {
    products().into_iter().take(5).collect()
}

fn layout_items() -> Vec<Product> {
    products().into_iter().take(6).collect()
}

fn list_template() -> Callback<Vec<Product>, AnyView> {
    Callback::new(|page: Vec<Product>| {
        view! {
            <div class="products-list">
                {page
                    .into_iter()
                    .enumerate()
                    .map(|(i, p)| {
                        let border = if i > 0 { "border-top: 1px solid var(--p-content-border-color)" } else { "" };
                        view! {
                            <div class="product-row" style={format!("display: flex; justify-content: space-between; padding: 1rem; {border}")}>
                                <div>
                                    <div style="font-weight: 600">{p.name}</div>
                                    <div style="color: var(--p-text-muted-color)">{p.category}</div>
                                </div>
                                <span class="product-price">{format!("${}", p.price)}</span>
                            </div>
                        }
                    })
                    .collect_view()}
            </div>
        }
        .into_any()
    })
}

fn grid_template() -> Callback<Vec<Product>, AnyView> {
    Callback::new(|page: Vec<Product>| {
        view! {
            <div class="products-grid" style="display: grid; grid-template-columns: repeat(3, 1fr); gap: 1rem">
                {page
                    .into_iter()
                    .map(|p| {
                        view! {
                            <div class="product-card" style="border: 1px solid var(--p-content-border-color); border-radius: 6px; padding: 1rem">
                                <div style="font-weight: 600">{p.name}</div>
                                <div style="color: var(--p-text-muted-color)">{p.category}</div>
                                <div class="product-price" style="margin-top: 0.5rem">{format!("${}", p.price)}</div>
                            </div>
                        }
                    })
                    .collect_view()}
            </div>
        }
        .into_any()
    })
}

#[component]
fn Basic() -> impl IntoView {
    view! {
        <div id="basic_dataview">
            <DataView value=basic_items() list=list_template() />
        </div>
    }
}

#[component]
fn Layout() -> impl IntoView {
    let layout = RwSignal::new(DataViewLayout::List);
    view! {
        <div id="layout_dataview">
            <DataView
                value=layout_items()
                list=list_template()
                grid=grid_template()
                layout=layout
                header=ViewFn::from(move || {
                    view! {
                        <div style="display: flex; gap: 0.5rem; justify-content: flex-end">
                            <Button label="List" on_click=Callback::new(move |_| layout.set(DataViewLayout::List)) />
                            <Button label="Grid" on_click=Callback::new(move |_| layout.set(DataViewLayout::Grid)) />
                        </div>
                    }
                })
            />
        </div>
    }
}

#[component]
fn Pagination() -> impl IntoView {
    view! {
        <div id="pagination_dataview">
            <DataView value=products() list=list_template() paginator=true default_rows=5usize />
        </div>
    }
}

#[component]
fn Sorting() -> impl IntoView {
    let order = RwSignal::new(0);
    let compare = Callback::new(|(a, b): (Product, Product)| a.price.cmp(&b.price));
    view! {
        <div id="sorting_dataview">
            <DataView
                value=products()
                list=list_template()
                compare=compare
                sort_order=order
                paginator=true
                default_rows=4usize
                header=ViewFn::from(move || {
                    view! {
                        <div style="display: flex; gap: 0.5rem">
                            <Button label="Price low to high" on_click=Callback::new(move |_| order.set(1)) />
                            <Button label="Price high to low" on_click=Callback::new(move |_| order.set(-1)) />
                        </div>
                    }
                })
            />
        </div>
    }
}

#[component]
fn Loading() -> impl IntoView {
    view! {
        <div id="loading_dataview">
            <DataView value=basic_items() list=list_template() loading=true />
        </div>
    }
}

/// Страница `/dataview`.
#[component]
pub fn DataViewPage() -> impl IntoView {
    view! {
        <h2>"DataView"</h2>
        <Demo name="basic" title="Basic">
            <Basic />
        </Demo>
        <Demo name="layout" title="Layout">
            <Layout />
        </Demo>
        <Demo name="pagination" title="Pagination">
            <Pagination />
        </Demo>
        <Demo name="sorting" title="Sorting">
            <Sorting />
        </Demo>
        <Demo name="loading" title="Loading">
            <Loading />
        </Demo>
    }
}
