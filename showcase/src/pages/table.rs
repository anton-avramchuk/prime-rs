//! Страница Table (этап 1): демонстрации `basic`, `singlecolumnsort`, `multiplecolumnssort`, `paginatorbasic`,
//! `singleselection`, `multipleselection`, `checkboxselection`, `radiobuttonselection`, `rowexpansion`, `striped`,
//! `gridlines`, `size`, `verticalscroll`, `loadingmask` и `style` из showcase PrimeNG (`apps/showcase/doc/table`;
//! фильтры, правка, изменение столбцов, заморозка, виртуальная прокрутка и группировка — следующие этапы).

use super::Demo;
use leptos::prelude::*;
use prime_core::Size;
use prime_table::model::{SelectionMode, SortMode};
use prime_table::{Column, Table};

#[derive(Clone, Debug)]
struct Product {
    code: &'static str,
    name: &'static str,
    category: &'static str,
    quantity: u32,
}

fn products() -> Vec<Product> {
    let data = [
        ("f230fh0g3", "Bamboo Watch", "Accessories", 24),
        ("nvklal433", "Black Watch", "Accessories", 61),
        ("zz21cz3c1", "Blue Band", "Fitness", 2),
        ("244wgerg2", "Blue T-Shirt", "Clothing", 25),
        ("h456wer53", "Bracelet", "Accessories", 73),
        ("av2231fwg", "Brown Purse", "Accessories", 0),
        ("bib36pfvm", "Chakra Bracelet", "Accessories", 5),
        ("mbvjkgip5", "Galaxy Earrings", "Accessories", 23),
        ("vbb124btr", "Game Controller", "Electronics", 2),
        ("cm230f032", "Gaming Set", "Electronics", 63),
    ];
    data.into_iter().map(|(code, name, category, quantity)| Product { code, name, category, quantity }).collect()
}

fn five() -> Vec<Product> {
    products().into_iter().take(5).collect()
}

fn three() -> Vec<Product> {
    products().into_iter().take(3).collect()
}

fn key() -> Callback<Product, String> {
    Callback::new(|p: Product| p.code.to_owned())
}

fn columns() -> Vec<Column<Product>> {
    vec![
        Column::text("code", "Code", |p: &Product| p.code.to_owned()),
        Column::text("name", "Name", |p: &Product| p.name.to_owned()),
        Column::text("category", "Category", |p: &Product| p.category.to_owned()),
        Column::text("quantity", "Quantity", |p: &Product| p.quantity.to_string()),
    ]
}

fn sortable_columns() -> Vec<Column<Product>> {
    vec![
        Column::text("code", "Code", |p: &Product| p.code.to_owned()).sortable(|a, b| a.code.cmp(b.code)),
        Column::text("name", "Name", |p: &Product| p.name.to_owned()).sortable(|a, b| a.name.cmp(b.name)),
        Column::text("category", "Category", |p: &Product| p.category.to_owned()).sortable(|a, b| a.category.cmp(b.category)),
        Column::text("quantity", "Quantity", |p: &Product| p.quantity.to_string()).sortable(|a, b| a.quantity.cmp(&b.quantity)),
    ]
}

const TABLE_STYLE: &str = "min-width: 50rem";

#[component]
fn Basic() -> impl IntoView {
    view! {
        <div id="basic_table">
            <Table value=five() columns=columns() row_key=key() table_style=TABLE_STYLE />
        </div>
    }
}

#[component]
fn SingleSort() -> impl IntoView {
    view! {
        <div id="sort_table">
            <Table value=five() columns=sortable_columns() row_key=key() table_style=TABLE_STYLE />
        </div>
    }
}

#[component]
fn MultiSort() -> impl IntoView {
    view! {
        <div id="multisort_table">
            <Table
                value=products()
                columns=sortable_columns()
                row_key=key()
                sort_mode=SortMode::Multiple
                table_style=TABLE_STYLE
                caption=ViewFn::from(|| view! { <span>"Hold Ctrl to sort by several columns"</span> })
            />
        </div>
    }
}

#[component]
fn Paginated() -> impl IntoView {
    view! {
        <div id="paginator_table">
            <Table
                value=products()
                columns=sortable_columns()
                row_key=key()
                paginator=true
                default_rows=4usize
                table_style=TABLE_STYLE
                show_current_page_report=true
                current_page_report_template="Showing {first} to {last} of {totalRecords}"
            />
        </div>
    }
}

#[component]
fn SingleSelection() -> impl IntoView {
    let selection = RwSignal::new(Vec::new());
    view! {
        <div id="single_table">
            <Table
                value=five()
                columns=columns()
                row_key=key()
                selection_mode=Some(SelectionMode::Single)
                selection=selection
                table_style=TABLE_STYLE
            />
            <p data-testid="single-selection" style="margin: 0.5rem 0 0">{move || selection.get().join(", ")}</p>
        </div>
    }
}

#[component]
fn MultipleSelection() -> impl IntoView {
    let selection = RwSignal::new(Vec::new());
    view! {
        <div id="multiple_table">
            <Table
                value=five()
                columns=columns()
                row_key=key()
                selection_mode=Some(SelectionMode::Multiple)
                meta_key_selection=true
                selection=selection
                table_style=TABLE_STYLE
            />
            <p data-testid="multiple-selection" style="margin: 0.5rem 0 0">{move || selection.get().join(", ")}</p>
        </div>
    }
}

#[component]
fn CheckboxSelection() -> impl IntoView {
    let selection = RwSignal::new(Vec::new());
    let mut cols = vec![Column::selection_checkbox()];
    cols.extend(columns());
    view! {
        <div id="checkbox_table">
            <Table value=five() columns=cols row_key=key() selection=selection table_style=TABLE_STYLE />
            <p data-testid="checkbox-selection" style="margin: 0.5rem 0 0">{move || selection.get().len()}</p>
        </div>
    }
}

#[component]
fn RadioSelection() -> impl IntoView {
    let selection = RwSignal::new(Vec::new());
    let mut cols = vec![Column::selection_radio()];
    cols.extend(columns());
    view! {
        <div id="radio_table">
            <Table value=five() columns=cols row_key=key() selection=selection table_style=TABLE_STYLE />
            <p data-testid="radio-selection" style="margin: 0.5rem 0 0">{move || selection.get().join(", ")}</p>
        </div>
    }
}

#[component]
fn RowExpansion() -> impl IntoView {
    let mut cols = vec![Column::expander()];
    cols.extend(columns());
    let expansion = Callback::new(|p: Product| view! { <div class="expansion-body">{format!("Orders for {}", p.name)}</div> }.into_any());
    view! {
        <div id="expansion_table">
            <Table
                value=five()
                columns=cols
                row_key=key()
                row_expansion=expansion
                table_style=TABLE_STYLE
            />
        </div>
    }
}

#[component]
fn Looks() -> impl IntoView {
    view! {
        <div id="looks_table" style="display: flex; flex-direction: column; gap: 1.5rem">
            <Table value=three() columns=columns() row_key=key() striped_rows=true table_style=TABLE_STYLE />
            <Table value=three() columns=columns() row_key=key() show_gridlines=true table_style=TABLE_STYLE />
            <Table value=three() columns=columns() row_key=key() size=Size::Small table_style=TABLE_STYLE />
            <Table value=three() columns=columns() row_key=key() size=Size::Large table_style=TABLE_STYLE />
        </div>
    }
}

#[component]
fn VerticalScroll() -> impl IntoView {
    view! {
        <div id="scroll_table">
            <Table value=products() columns=columns() row_key=key() scrollable=true scroll_height="220px" table_style=TABLE_STYLE />
        </div>
    }
}

#[component]
fn Loading() -> impl IntoView {
    view! {
        <div id="loading_table" style="position: relative">
            <Table value=three() columns=columns() row_key=key() loading=true table_style=TABLE_STYLE />
        </div>
    }
}

/// Страница `/table`.
#[component]
pub fn TablePage() -> impl IntoView {
    view! {
        <h2>"Table"</h2>
        <Demo name="basic" title="Basic">
            <Basic />
        </Demo>
        <Demo name="singlecolumnsort" title="Single Column Sort">
            <SingleSort />
        </Demo>
        <Demo name="multiplecolumnssort" title="Multiple Column Sort">
            <MultiSort />
        </Demo>
        <Demo name="paginatorbasic" title="Pagination">
            <Paginated />
        </Demo>
        <Demo name="singleselection" title="Single Selection">
            <SingleSelection />
        </Demo>
        <Demo name="multipleselection" title="Multiple Selection">
            <MultipleSelection />
        </Demo>
        <Demo name="checkboxselection" title="Checkbox Selection">
            <CheckboxSelection />
        </Demo>
        <Demo name="radiobuttonselection" title="RadioButton Selection">
            <RadioSelection />
        </Demo>
        <Demo name="rowexpansion" title="Row Expansion">
            <RowExpansion />
        </Demo>
        <Demo name="looks" title="Striped, Gridlines and Size">
            <Looks />
        </Demo>
        <Demo name="verticalscroll" title="Vertical Scroll">
            <VerticalScroll />
        </Demo>
        <Demo name="loadingmask" title="Loading Mask">
            <Loading />
        </Demo>
    }
}
