//! Страница DatePicker (этап 1): демонстрации `basic`, `format`, `inline`, `multiple`, `range`, `minmax`, `buttonbar`,
//! `month`, `year`, `icon`, `weeks`, `multiplemonths`, `disabled` и `invalid` из showcase PrimeNG
//! (`apps/showcase/doc/datepicker`; время, шаблоны и формы Angular не переносятся).
//! «Сегодня» зафиксировано на 5 ноября 2026, чтобы страница и e2e не зависели от системной даты.

use super::Demo;
use leptos::prelude::*;
use prime_datepicker::calendar::{CalDate, SelectionMode};
use prime_datepicker::{DatePicker, DatePickerView, IconDisplay};

const TODAY: CalDate = CalDate { year: 2026, month: 10, day: 5 };
const WIDTH: &str = "width: 100%; max-width: 14rem";

fn echo(id: &'static str, value: RwSignal<Vec<CalDate>>) -> impl IntoView {
    view! {
        <span data-testid=id style="margin-left: 0.5rem">
            {move || value.get().iter().map(|d| format!("{}-{}-{}", d.year, d.month + 1, d.day)).collect::<Vec<_>>().join(" | ")}
        </span>
    }
}

#[component]
fn Basic() -> impl IntoView {
    let value = RwSignal::new(Vec::new());
    view! {
        <div style="display: flex; align-items: center">
            <DatePicker input_id="basic_dp" value=value today=TODAY style=WIDTH />
            {echo("basic-echo", value)}
        </div>
    }
}

#[component]
fn Format() -> impl IntoView {
    let value = RwSignal::new(vec![TODAY]);
    view! { <DatePicker input_id="format_dp" value=value today=TODAY date_format="DD, d MM yy" style=WIDTH /> }
}

#[component]
fn Inline() -> impl IntoView {
    let value = RwSignal::new(Vec::new());
    view! {
        <div id="inline_dp">
            <DatePicker inline=true value=value today=TODAY />
            {echo("inline-echo", value)}
        </div>
    }
}

#[component]
fn Multiple() -> impl IntoView {
    let value = RwSignal::new(Vec::new());
    view! { <DatePicker input_id="multiple_dp" value=value today=TODAY selection_mode=SelectionMode::Multiple style=WIDTH /> }
}

#[component]
fn Range() -> impl IntoView {
    let value = RwSignal::new(Vec::new());
    view! { <DatePicker input_id="range_dp" value=value today=TODAY selection_mode=SelectionMode::Range style=WIDTH /> }
}

#[component]
fn MinMax() -> impl IntoView {
    let value = RwSignal::new(Vec::new());
    view! {
        <DatePicker
            input_id="minmax_dp"
            value=value
            today=TODAY
            min_date=CalDate::new(2026, 10, 10)
            max_date=CalDate::new(2026, 10, 25)
            disabled_days=vec![0u32, 6]
            style=WIDTH
        />
    }
}

#[component]
fn ButtonBar() -> impl IntoView {
    let value = RwSignal::new(Vec::new());
    view! { <DatePicker input_id="buttonbar_dp" value=value today=TODAY show_button_bar=true style=WIDTH /> }
}

#[component]
fn MonthPicker() -> impl IntoView {
    let value = RwSignal::new(Vec::new());
    view! { <DatePicker input_id="month_dp" value=value today=TODAY view=DatePickerView::Month date_format="mm/yy" style=WIDTH /> }
}

#[component]
fn YearPicker() -> impl IntoView {
    let value = RwSignal::new(Vec::new());
    view! { <DatePicker input_id="year_dp" value=value today=TODAY view=DatePickerView::Year date_format="yy" style=WIDTH /> }
}

#[component]
fn Icon() -> impl IntoView {
    let (a, b) = (RwSignal::new(Vec::new()), RwSignal::new(Vec::new()));
    view! {
        <div style="display: flex; gap: 1rem; flex-wrap: wrap">
            <DatePicker input_id="icon_button_dp" value=a today=TODAY show_icon=true show_on_focus=false style=WIDTH />
            <DatePicker input_id="icon_input_dp" value=b today=TODAY show_icon=true icon_display=IconDisplay::Input style=WIDTH />
        </div>
    }
}

#[component]
fn Weeks() -> impl IntoView {
    let value = RwSignal::new(Vec::new());
    view! { <DatePicker input_id="weeks_dp" value=value today=TODAY show_week=true first_day_of_week=1u32 style=WIDTH /> }
}

#[component]
fn MultipleMonths() -> impl IntoView {
    let value = RwSignal::new(Vec::new());
    view! { <DatePicker input_id="months_dp" value=value today=TODAY number_of_months=2usize style=WIDTH /> }
}

/// Страница `/datepicker`.
#[component]
pub fn DatePickerPage() -> impl IntoView {
    view! {
        <h2>"DatePicker"</h2>
        <Demo name="basic" title="Basic">
            <Basic />
        </Demo>
        <Demo name="format" title="Format">
            <Format />
        </Demo>
        <Demo name="inline" title="Inline">
            <Inline />
        </Demo>
        <Demo name="multiple" title="Multiple">
            <Multiple />
        </Demo>
        <Demo name="range" title="Range">
            <Range />
        </Demo>
        <Demo name="minmax" title="Min / Max">
            <MinMax />
        </Demo>
        <Demo name="buttonbar" title="Button Bar">
            <ButtonBar />
        </Demo>
        <Demo name="month" title="Month Picker">
            <MonthPicker />
        </Demo>
        <Demo name="year" title="Year Picker">
            <YearPicker />
        </Demo>
        <Demo name="icon" title="Icon">
            <Icon />
        </Demo>
        <Demo name="weeks" title="Week Numbers">
            <Weeks />
        </Demo>
        <Demo name="multiplemonths" title="Multiple Months">
            <MultipleMonths />
        </Demo>
        <Demo name="disabled" title="Disabled">
            <DatePicker input_id="disabled_dp" today=TODAY disabled=true style=WIDTH />
        </Demo>
        <Demo name="invalid" title="Invalid">
            <DatePicker input_id="invalid_dp" today=TODAY invalid=true style=WIDTH />
        </Demo>
    }
}
