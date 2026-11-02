//! Страница Paginator: демонстрации `basic`, `currentpagereport`, `template`, `images` из showcase PrimeNG
//! (`apps/showcase/doc/paginator`). В `images` вместо фотографий с CDN — цветные блоки с номером страницы.

use super::Demo;
use leptos::prelude::*;
use prime_button::{Button, ButtonVariant};
use prime_divider::Divider;
use prime_icons::{SearchIcon, StarIcon};
use prime_paginator::{Paginator, RowsPerPage};
use prime_select::{Select, SelectOption};
use prime_slider::Slider;

fn icon<V: IntoView + 'static>(f: impl Fn() -> V + Send + Sync + 'static) -> ViewFn {
    ViewFn::from(f)
}

fn rows_options() -> Vec<RowsPerPage> {
    vec![RowsPerPage::Rows(10), RowsPerPage::Rows(20), RowsPerPage::Rows(30)]
}

/// Страница `/paginator`.
#[component]
pub fn PaginatorPage() -> impl IntoView {
    let (first, rows) = (RwSignal::new(0usize), RwSignal::new(10usize));
    let (report_first, report_rows) = (RwSignal::new(0usize), RwSignal::new(10usize));
    let (first1, rows1) = (RwSignal::new(0usize), RwSignal::new(10usize));
    let (first2, rows2) = (RwSignal::new(0usize), RwSignal::new(10usize));
    let rows2_value = RwSignal::new(Some("10".to_owned()));
    let (first3, rows3_slider) = (RwSignal::new(0usize), RwSignal::new(10.0f64));
    let rows3 = RwSignal::new(10usize);
    let image_first = RwSignal::new(0usize);
    let items_per_page = Callback::new(move |e: prime_select::SelectChange| {
        if let Some(n) = e.value.and_then(|v| v.parse::<usize>().ok()) {
            rows2.set(n);
            first2.set(0);
        }
    });
    let slider_change = Callback::new(move |e: prime_slider::SliderChange| {
        if let prime_slider::SliderValue::Single(v) = e.value {
            rows3.set(v as usize);
            first3.set(0);
        }
    });
    view! {
        <h2>"Paginator"</h2>
        <Demo name="basic" title="Basic">
            <Paginator first=first rows=rows total_records=120 rows_per_page_options=rows_options() />
            <span data-testid="echo">{move || format!("first={} rows={}", first.get(), rows.get())}</span>
        </Demo>
        <Demo name="currentpagereport" title="Current Page Report">
            <Paginator
                first=report_first
                rows=report_rows
                total_records=120
                show_current_page_report=true
                show_page_links=false
                show_jump_to_page_dropdown=false
                current_page_report_template="Showing {first} to {last} of {totalRecords}"
            />
        </Demo>
        <Demo name="template" title="Template">
            <div style="display: flex; flex-direction: column; gap: 1rem; width: 100%">
                <div style="display: flex; align-items: center; justify-content: center">
                    <div><Button icon=icon(|| view! { <StarIcon /> }) variant=ButtonVariant::Outlined aria_label="Star" /></div>
                    <div style="flex: 1">
                        <Paginator first=first1 rows=rows1 total_records=120 rows_per_page_options=rows_options() show_first_last_icon=false />
                    </div>
                    <div><Button icon=icon(|| view! { <SearchIcon /> }) aria_label="Search" /></div>
                </div>
                <Divider />
                <div style="display: flex; align-items: center; justify-content: flex-end; gap: 0.25rem">
                    <span>"Items per page: "</span>
                    <Select
                        options=vec![SelectOption::new("10", "10"), SelectOption::new("20", "20"), SelectOption::new("30", "30")]
                        value=rows2_value
                        on_change=items_per_page
                    />
                    <Paginator
                        first=first2
                        rows=rows2
                        total_records=120
                        show_current_page_report=true
                        current_page_report_template="{first} - {last} of {totalRecords}"
                        show_page_links=false
                        show_first_last_icon=false
                    />
                </div>
                <Divider />
                <div style="display: flex; align-items: center; justify-content: flex-start">
                    <div style="display: flex; align-items: center; gap: 1rem">
                        <span>"Items per page: "</span>
                        <Slider value=rows3_slider min=10.0 max=120.0 step=30.0 on_change=slider_change style="width: 10rem" aria_label="Items per page" />
                    </div>
                    <Paginator
                        first=first3
                        rows=rows3
                        total_records=120
                        show_first_last_icon=false
                        show_current_page_report=true
                        current_page_report_template="{first} - {last} of {totalRecords}"
                    />
                </div>
            </div>
        </Demo>
        <Demo name="images" title="Images">
            <div style="display: flex; flex-direction: column; align-items: center; gap: 1rem">
                <Paginator first=image_first rows=RwSignal::new(1) total_records=12 show_jump_to_page_dropdown=true show_page_links=false />
                <div
                    data-testid="image"
                    style="width: 20rem; max-width: 100%; height: 12rem; border-radius: 12px; display: flex; align-items: center; justify-content: center; color: white; font-size: 2rem"
                    style:background=move || format!("hsl({}, 55%, 45%)", image_first.get() * 30)
                >
                    {move || format!("Image {}", image_first.get() + 1)}
                </div>
            </div>
        </Demo>
    }
}
