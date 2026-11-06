//! Разметка DatePicker (SSR): поле, значки, панель, шапка, сетка дней, выбранные дни, ограничения, режимы, виды, `pt`.
#![cfg(feature = "ssr")]
#![recursion_limit = "512"]

use leptos::prelude::*;
use prime_core::Pt;
use prime_datepicker::calendar::{CalDate, SelectionMode};
use prime_datepicker::{DatePicker, DatePickerPt, DatePickerView, IconDisplay};

fn render(f: impl FnOnce() -> String) -> String {
    Owner::new().with(f)
}

fn d(y: i32, m: u32, day: u32) -> CalDate {
    CalDate::new(y, m, day)
}

const TODAY: CalDate = CalDate { year: 2026, month: 10, day: 5 };

/// Открывающий тег элемента, в котором встречается подстрока.
fn tag_of<'a>(html: &'a str, needle: &str) -> &'a str {
    let at = html.find(needle).unwrap_or_else(|| panic!("нет {needle}: {html}"));
    let start = html[..at].rfind('<').unwrap();
    let end = at + html[at..].find('>').unwrap();
    &html[start..=end]
}

#[test]
fn closed_picker_has_field_and_no_panel() {
    let html = render(|| view! { <DatePicker today=TODAY placeholder="Date" input_id="dp" /> }.to_html());
    assert!(html.contains("p-datepicker p-component p-inputwrapper"), "{html}");
    let input = tag_of(&html, "p-datepicker-input");
    assert!(input.contains("role=\"combobox\"") && input.contains("placeholder=\"Date\"") && input.contains("id=\"dp\""), "{input}");
    assert!(input.contains("p-inputtext p-component"), "поле на классах InputText: {input}");
    assert!(!html.contains("p-datepicker-panel") && !html.contains("p-datepicker-dropdown"), "{html}");
}

#[test]
fn value_is_formatted() {
    let html = render(|| {
        let value = RwSignal::new(vec![d(2026, 10, 5)]);
        view! { <DatePicker value=value today=TODAY /> }.to_html()
    });
    assert!(html.contains("11/05/2026") && html.contains("p-inputwrapper-filled") && html.contains("p-filled"), "{html}");
    let html = render(|| {
        let value = RwSignal::new(vec![d(2026, 10, 5)]);
        view! { <DatePicker value=value date_format="dd.mm.yy" today=TODAY /> }.to_html()
    });
    assert!(html.contains("05.11.2026"), "{html}");
    let html = render(|| {
        let value = RwSignal::new(vec![d(2026, 10, 5), d(2026, 10, 9)]);
        view! { <DatePicker value=value selection_mode=SelectionMode::Multiple today=TODAY /> }.to_html()
    });
    assert!(html.contains("11/05/2026, 11/09/2026"), "{html}");
    let html = render(|| {
        let value = RwSignal::new(vec![d(2026, 10, 5), d(2026, 10, 9)]);
        view! { <DatePicker value=value selection_mode=SelectionMode::Range today=TODAY /> }.to_html()
    });
    assert!(html.contains("11/05/2026 - 11/09/2026"), "{html}");
}

#[test]
fn inline_calendar_grid() {
    let html = render(|| view! { <DatePicker inline=true today=TODAY /> }.to_html());
    assert!(!html.contains("p-datepicker-input"), "{html}");
    assert!(html.contains("p-datepicker-panel p-component p-datepicker-panel-inline"), "{html}");
    assert!(html.contains("p-datepicker-select-month") && html.contains(">November<"), "{html}");
    assert!(html.contains("p-datepicker-select-year") && html.contains(">2026<"), "{html}");
    for day in ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"] {
        assert!(html.contains(&format!(">{day}<")), "нет {day}: {html}");
    }
    assert_eq!(html.matches("data-date=\"2026-10-").count(), 30, "30 дней ноября: {html}");
    assert!(html.contains("data-date=\"2026-10-5\""), "{html}");
    assert!(html.contains("p-datepicker-today"), "сегодняшний день отмечен: {html}");
    assert_eq!(html.matches("p-datepicker-other-month").count(), 5, "пять дней декабря: {html}");
    assert!(html.contains("p-datepicker-prev-button") && html.contains("p-datepicker-next-button"), "{html}");
}

#[test]
fn selected_and_range_classes() {
    let html = render(|| {
        let value = RwSignal::new(vec![d(2026, 10, 5)]);
        view! { <DatePicker inline=true value=value today=TODAY /> }.to_html()
    });
    assert_eq!(html.matches("p-datepicker-day-selected").count(), 1, "{html}");
    let html = render(|| {
        let value = RwSignal::new(vec![d(2026, 10, 5), d(2026, 10, 9)]);
        view! { <DatePicker inline=true value=value selection_mode=SelectionMode::Range today=TODAY /> }.to_html()
    });
    assert_eq!(html.matches("p-datepicker-day-selected-range").count(), 3, "дни 6–8: {html}");
    assert_eq!(html.matches("p-datepicker-day p-datepicker-day-selected\"").count(), 2, "края диапазона: {html}");
}

#[test]
fn limits_disable_days() {
    let html = render(|| {
        view! { <DatePicker inline=true today=TODAY min_date=d(2026, 10, 10) max_date=d(2026, 10, 20) disabled_dates=vec![d(2026, 10, 15)] disabled_days=vec![0u32] /> }
            .to_html()
    });
    let disabled = |day: u32| tag_of(&html, &format!("data-date=\"2026-10-{day}\"")).contains("p-disabled");
    assert!(disabled(9) && disabled(21) && disabled(15), "{html}");
    assert!(!disabled(10) && !disabled(20) && !disabled(16), "{html}");
    assert!(disabled(22), "воскресенье 22? нет, 22 ноября — воскресенье и вне диапазона: {html}");
}

#[test]
fn first_day_of_week_and_weeks() {
    let html = render(|| view! { <DatePicker inline=true today=TODAY first_day_of_week=1u32 show_week=true /> }.to_html());
    let mo = html.find(">Mo<").unwrap();
    let su = html.find(">Su<").unwrap();
    assert!(mo < su, "неделя с понедельника: {html}");
    assert!(html.contains("p-datepicker-weekheader") && html.contains("p-datepicker-weeknumber") && html.contains(">Wk<"), "{html}");
    assert!(html.contains("data-date=\"2026-9-26\""), "первая неделя начинается с 26 октября: {html}");
}

#[test]
fn other_months_can_be_hidden() {
    let html = render(|| view! { <DatePicker inline=true today=TODAY show_other_months=false /> }.to_html());
    assert!(!html.contains("data-date=\"2026-11-"), "{html}");
    assert!(html.contains("p-datepicker-other-month"), "ячейки остаются: {html}");
}

#[test]
fn month_and_year_views() {
    let html = render(|| view! { <DatePicker inline=true today=TODAY view=DatePickerView::Month /> }.to_html());
    assert!(html.contains("p-datepicker-month-view") && !html.contains("p-datepicker-day-view"), "{html}");
    assert_eq!(html.matches("<span class=\"p-datepicker-month").count(), 12, "{html}");
    assert!(html.contains(">Nov<") && html.contains(">2026<"), "{html}");
    let html = render(|| view! { <DatePicker inline=true today=TODAY view=DatePickerView::Year /> }.to_html());
    assert!(html.contains("p-datepicker-year-view") && html.contains("p-datepicker-decade"), "{html}");
    assert!(html.contains("2020 - 2029") && html.contains(">2026<"), "{html}");
}

#[test]
fn multiple_months_show_two_headers() {
    let html = render(|| view! { <DatePicker inline=true today=TODAY number_of_months=2usize /> }.to_html());
    assert_eq!(html.matches("p-datepicker-calendar\"").count(), 2, "{html}");
    assert!(html.contains(">November<") && html.contains(">December<"), "{html}");
}

#[test]
fn icons_button_bar_and_clear() {
    let html = render(|| view! { <DatePicker today=TODAY show_icon=true /> }.to_html());
    assert!(html.contains("p-datepicker-dropdown") && html.contains("aria-label=\"Choose Date\""), "{html}");
    let html = render(|| view! { <DatePicker today=TODAY show_icon=true icon_display=IconDisplay::Input /> }.to_html());
    assert!(
        html.contains("p-datepicker-input-icon-container")
            && html.contains("p-datepicker-input-icon")
            && !html.contains("p-datepicker-dropdown"),
        "{html}"
    );
    let html = render(|| {
        let value = RwSignal::new(vec![d(2026, 10, 5)]);
        view! { <DatePicker today=TODAY value=value show_clear=true /> }.to_html()
    });
    assert!(html.contains("p-datepicker-clear-icon"), "{html}");
    let html = render(|| view! { <DatePicker today=TODAY show_clear=true /> }.to_html());
    assert!(!html.contains("p-datepicker-clear-icon"), "без значения значка нет: {html}");
    let html = render(|| view! { <DatePicker inline=true today=TODAY show_button_bar=true /> }.to_html());
    assert!(
        html.contains("p-datepicker-buttonbar") && html.contains("p-datepicker-today-button") && html.contains("p-datepicker-clear-button"),
        "{html}"
    );
    assert!(html.contains(">Today<") && html.contains(">Clear<"), "{html}");
}

#[test]
fn state_classes_and_disabled() {
    let html = render(|| view! { <DatePicker today=TODAY invalid=true fluid=true disabled=true /> }.to_html());
    assert!(html.contains("p-invalid") && html.contains("p-datepicker-fluid"), "{html}");
    assert!(tag_of(&html, "p-datepicker-input").contains("disabled"), "{html}");
    let html = render(|| view! { <DatePicker inline=true today=TODAY disabled=true /> }.to_html());
    assert!(html.contains("p-datepicker-panel p-component p-datepicker-panel-inline p-disabled"), "{html}");
}

#[test]
fn style_and_pass_through() {
    let pt = DatePickerPt { root: Pt::new().class("root-x"), input: Pt::new().class("input-x"), ..Default::default() };
    let html = render(|| view! { <DatePicker today=TODAY style="width: 14rem" style_class="mine" pt=pt /> }.to_html());
    assert!(html.contains("root-x") && html.contains("mine") && html.contains("width: 14rem") && html.contains("input-x"), "{html}");
}
