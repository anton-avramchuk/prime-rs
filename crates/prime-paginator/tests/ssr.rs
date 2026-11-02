//! Разметка Paginator (SSR): кнопки, ссылки, отчёт, списки, поле, скрытие, пустой список, шаблоны, `pt`.
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_core::Pt;
use prime_paginator::{Paginator, PaginatorPt, PaginatorState, RowsPerPage};

fn render(f: impl FnOnce() -> String) -> String {
    Owner::new().with(f)
}

/// Число ссылок на страницы (без контейнера `p-paginator-pages`).
fn link_count(html: &str) -> usize {
    html.matches("class=\"p-paginator-page\"").count() + html.matches("class=\"p-paginator-page p-").count()
}

/// Есть ли у открывающего тега атрибут `disabled` (а не только класс `p-disabled`).
fn has_disabled_attr(tag: &str) -> bool {
    tag.split_whitespace().any(|t| t == "disabled" || t.starts_with("disabled="))
}

#[test]
fn default_markup_has_four_buttons_and_five_links() {
    let html = render(|| view! { <Paginator total_records=120 /> }.to_html());
    assert!(html.contains("p-paginator p-component"), "{html}");
    for class in ["p-paginator-first", "p-paginator-prev", "p-paginator-next", "p-paginator-last"] {
        assert!(html.contains(class), "нет {class}: {html}");
    }
    assert_eq!(link_count(&html), 5, "{html}");
    assert!(
        html.contains("p-paginator-pages") && html.contains("aria-label=\"First Page\"") && html.contains("aria-label=\"Last Page\""),
        "{html}"
    );
    assert!(html.contains("aria-label=\"Previous Page\"") && html.contains("aria-label=\"Next Page\""), "{html}");
    assert!(!html.contains("p-paginator-current") && !html.contains("p-select") && !html.contains("p-inputnumber"), "{html}");
}

#[test]
fn first_page_is_selected_and_back_buttons_are_disabled() {
    let html = render(|| view! { <Paginator total_records=120 /> }.to_html());
    assert_eq!(html.matches("p-paginator-page-selected").count(), 1, "{html}");
    assert!(html.contains("aria-current=\"page\""), "{html}");
    assert!(html.contains("p-paginator-first p-disabled") && html.contains("p-paginator-prev p-disabled"), "{html}");
    assert!(!html.contains("p-paginator-next p-disabled") && !html.contains("p-paginator-last p-disabled"), "{html}");
    // «первая» без атрибута disabled, как в PrimeNG; «предыдущая» с ним
    let first_btn = html.split("<button").find(|b| b.contains("p-paginator-first")).unwrap();
    let prev_btn = html.split("<button").find(|b| b.contains("p-paginator-prev")).unwrap();
    assert!(!has_disabled_attr(first_btn.split('>').next().unwrap()), "{first_btn}");
    assert!(has_disabled_attr(prev_btn.split('>').next().unwrap()), "{prev_btn}");
}

#[test]
fn last_page_disables_forward_buttons() {
    let first = RwSignal::new(110);
    let html = render(|| view! { <Paginator first=first rows=RwSignal::new(10) total_records=120 /> }.to_html());
    assert!(html.contains("p-paginator-next p-disabled") && html.contains("p-paginator-last p-disabled"), "{html}");
    assert!(!html.contains("p-paginator-prev p-disabled"), "{html}");
    assert!(html.contains(">12<") || html.contains("12</button>"), "{html}");
}

#[test]
fn link_window_follows_the_current_page() {
    let first = RwSignal::new(50);
    let html = render(|| view! { <Paginator first=first rows=RwSignal::new(10) total_records=120 /> }.to_html());
    for n in 4..=8 {
        assert!(html.contains(&format!("aria-label=\"{n}\"")), "нет страницы {n}: {html}");
    }
    assert!(!html.contains("aria-label=\"3\"") && !html.contains("aria-label=\"9\""), "{html}");
    let narrow = render(|| view! { <Paginator first=first rows=RwSignal::new(10) total_records=120 page_link_size=3 /> }.to_html());
    assert_eq!(link_count(&narrow), 3, "{narrow}");
}

#[test]
fn few_pages_show_fewer_links() {
    let html = render(|| view! { <Paginator total_records=25 /> }.to_html());
    assert_eq!(link_count(&html), 3, "{html}");
}

#[test]
fn current_page_report_uses_the_template() {
    let first = RwSignal::new(10);
    let html = render(|| {
        view! {
            <Paginator
                first=first
                rows=RwSignal::new(10)
                total_records=120
                show_current_page_report=true
                current_page_report_template="Showing {first} to {last} of {totalRecords}"
                show_page_links=false
            />
        }
        .to_html()
    });
    assert!(html.contains("p-paginator-current") && html.contains("Showing 11 to 20 of 120"), "{html}");
    assert!(!html.contains("p-paginator-pages"), "{html}");
    let default =
        render(|| view! { <Paginator first=first rows=RwSignal::new(10) total_records=120 show_current_page_report=true /> }.to_html());
    assert!(default.contains("2 of 12"), "{default}");
}

#[test]
fn first_last_buttons_can_be_hidden() {
    let html = render(|| view! { <Paginator total_records=120 show_first_last_icon=false /> }.to_html());
    assert!(!html.contains("p-paginator-first") && !html.contains("p-paginator-last"), "{html}");
    assert!(html.contains("p-paginator-prev") && html.contains("p-paginator-next"), "{html}");
}

#[test]
fn rows_per_page_dropdown_with_show_all() {
    let html = render(|| {
        view! {
            <Paginator total_records=120 rows_per_page_options=vec![RowsPerPage::Rows(10), RowsPerPage::Rows(20), RowsPerPage::ShowAll("All".into())] />
        }
        .to_html()
    });
    assert!(html.contains("p-select") && html.contains("p-paginator-rpp-dropdown"), "{html}");
    assert!(html.contains("aria-label=\"Rows per page\"") || html.contains("Rows per page"), "{html}");
    assert!(html.contains(">10<") || html.contains("10</span>"), "выбранное число строк на метке: {html}");
}

#[test]
fn jump_to_page_dropdown_shows_the_report_as_label() {
    let first = RwSignal::new(40);
    let html = render(|| {
        view! { <Paginator first=first rows=RwSignal::new(10) total_records=120 show_jump_to_page_dropdown=true show_page_links=false /> }
            .to_html()
    });
    assert!(html.contains("p-paginator-jtp-dropdown") && html.contains("5 of 12"), "{html}");
}

#[test]
fn jump_to_page_input_is_an_inputnumber_with_the_current_page() {
    let first = RwSignal::new(20);
    let html =
        render(|| view! { <Paginator first=first rows=RwSignal::new(10) total_records=120 show_jump_to_page_input=true /> }.to_html());
    assert!(html.contains("p-inputnumber") && html.contains("p-paginator-jtp-input") && html.contains("role=\"spinbutton\""), "{html}");
    assert!(html.contains("value=\"3\""), "{html}");
}

#[test]
fn element_order_matches_primeng() {
    let html = render(|| {
        view! {
            <Paginator
                total_records=120
                show_current_page_report=true
                show_jump_to_page_dropdown=true
                show_jump_to_page_input=true
                rows_per_page_options=vec![RowsPerPage::Rows(10)]
                template_left=Callback::new(|_: PaginatorState| view! { <i class="l">"L"</i> }.into_any())
                template_right=Callback::new(|_: PaginatorState| view! { <i class="r">"R"</i> }.into_any())
            />
        }
        .to_html()
    });
    let pos = |needle: &str| html.find(needle).unwrap_or_else(|| panic!("нет {needle}: {html}"));
    let order = [
        pos("p-paginator-content-start"),
        pos("p-paginator-current"),
        pos("p-paginator-first"),
        pos("p-paginator-prev"),
        pos("p-paginator-pages"),
        pos("p-paginator-jtp-dropdown"),
        pos("p-paginator-next"),
        pos("p-paginator-last"),
        pos("p-paginator-jtp-input"),
        pos("p-paginator-rpp-dropdown"),
        pos("p-paginator-content-end"),
    ];
    assert!(order.windows(2).all(|w| w[0] < w[1]), "порядок элементов: {order:?}");
}

#[test]
fn templates_receive_the_state() {
    let first = RwSignal::new(30);
    let html = render(|| {
        view! {
            <Paginator
                first=first
                rows=RwSignal::new(10)
                total_records=120
                template_left=Callback::new(|s: PaginatorState| view! { <b class="l">{format!("p{}/{}", s.page, s.page_count)}</b> }.into_any())
            />
        }
        .to_html()
    });
    assert!(html.contains("p3/12"), "{html}");
}

#[test]
fn hidden_when_always_show_is_off_and_there_is_one_page() {
    let single = render(|| view! { <Paginator total_records=5 always_show=false /> }.to_html());
    assert!(single.contains("display: none"), "{single}");
    let many = render(|| view! { <Paginator total_records=50 always_show=false /> }.to_html());
    assert!(!many.contains("display: none"), "{many}");
    let always = render(|| view! { <Paginator total_records=5 /> }.to_html());
    assert!(!always.contains("display: none"), "{always}");
}

#[test]
fn empty_and_zero_rows_have_no_links_and_disabled_buttons() {
    let empty = render(|| view! { <Paginator total_records=0 show_current_page_report=true /> }.to_html());
    assert_eq!(link_count(&empty), 0, "{empty}");
    assert!(empty.contains("0 of 0"), "{empty}");
    for class in
        ["p-paginator-first p-disabled", "p-paginator-prev p-disabled", "p-paginator-next p-disabled", "p-paginator-last p-disabled"]
    {
        assert!(empty.contains(class), "нет {class}: {empty}");
    }
    let zero = render(|| view! { <Paginator total_records=50 rows=RwSignal::new(0) /> }.to_html());
    assert_eq!(link_count(&zero), 0, "{zero}");
}

#[test]
fn custom_icons_replace_defaults() {
    let html = render(|| {
        view! {
            <Paginator
                total_records=120
                first_page_link_icon=ViewFn::from(|| view! { <i class="fi">"F"</i> })
                next_page_link_icon=ViewFn::from(|| view! { <i class="ni">"N"</i> })
            />
        }
        .to_html()
    });
    assert!(html.contains("class=\"fi\"") && html.contains("class=\"ni\""), "{html}");
    assert_eq!(html.matches("<svg").count(), 2, "остальные два значка стандартные: {html}");
}

#[test]
fn style_class_and_pt() {
    let pt = PaginatorPt {
        root: Pt::new().class("r").attr("data-x", "1"),
        page: Pt::new().class("pg"),
        next: Pt::new().class("nx"),
        ..PaginatorPt::default()
    };
    let html = render(|| view! { <Paginator total_records=120 style="width: 30rem" style_class="mine" pt=pt /> }.to_html());
    assert!(html.contains("p-paginator p-component mine r") && html.contains("data-x=\"1\"") && html.contains("width: 30rem"), "{html}");
    assert!(html.contains("p-paginator-page p-paginator-page-selected pg") && html.contains("p-paginator-next nx"), "{html}");
}
