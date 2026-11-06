//! Разметка DataView (SSR): список и сетка, пагинатор, пустое состояние, загрузка, заголовок и подвал, сортировка, фильтр, `pt`.
#![cfg(feature = "ssr")]
#![recursion_limit = "512"]

use leptos::prelude::*;
use prime_core::Pt;
use prime_dataview::{DataView, DataViewLayout, DataViewPt, PaginatorPosition};

fn render(f: impl FnOnce() -> String) -> String {
    Owner::new().with(f)
}

fn items(n: i32) -> Vec<i32> {
    (1..=n).collect()
}

fn list() -> Callback<Vec<i32>, AnyView> {
    Callback::new(|page: Vec<i32>| {
        view! { <ul>{page.into_iter().map(|i| view! { <li class="item">{i}</li> }).collect_view()}</ul> }.into_any()
    })
}

fn grid() -> Callback<Vec<i32>, AnyView> {
    Callback::new(|page: Vec<i32>| {
        view! { <div class="cells">{page.into_iter().map(|i| view! { <b class="cell">{i}</b> }).collect_view()}</div> }.into_any()
    })
}

#[test]
fn list_markup() {
    let html = render(|| view! { <DataView value=items(3) list=list() /> }.to_html());
    assert!(html.contains("p-dataview p-component p-dataview-list"), "{html}");
    assert!(html.contains("p-dataview-content") && html.matches("class=\"item\"").count() == 3, "{html}");
    assert!(!html.contains("p-paginator") && !html.contains("p-dataview-empty-message") && !html.contains("p-dataview-loading"), "{html}");
}

#[test]
fn grid_layout_uses_grid_template() {
    let html = render(|| view! { <DataView value=items(4) list=list() grid=grid() layout=DataViewLayout::Grid /> }.to_html());
    assert!(html.contains("p-dataview-grid") && !html.contains("p-dataview-list"), "{html}");
    assert_eq!(html.matches("class=\"cell\"").count(), 4, "{html}");
    assert!(!html.contains("class=\"item\""), "{html}");
}

#[test]
fn paginator_slices_the_page() {
    let html = render(|| view! { <DataView value=items(25) list=list() paginator=true default_rows=10usize /> }.to_html());
    assert_eq!(html.matches("class=\"item\"").count(), 10, "{html}");
    assert!(html.contains("p-dataview-paginator-bottom") && html.contains("p-paginator"), "{html}");
    assert!(!html.contains("p-dataview-paginator-top"), "{html}");
    assert!(html.contains(">1<") && html.contains(">10<") && !html.contains(">11<"), "{html}");
    let html = render(|| {
        let first = RwSignal::new(20usize);
        view! { <DataView value=items(25) list=list() paginator=true default_rows=10usize first=first /> }.to_html()
    });
    assert_eq!(html.matches("class=\"item\"").count(), 5, "последняя страница: {html}");
    let html = render(|| {
        let first = RwSignal::new(90usize);
        view! { <DataView value=items(25) list=list() paginator=true default_rows=10usize first=first /> }.to_html()
    });
    assert_eq!(html.matches("class=\"item\"").count(), 5, "first за пределами возвращается на последнюю страницу: {html}");
}

#[test]
fn paginator_positions() {
    let html = render(|| {
        view! { <DataView value=items(25) list=list() paginator=true default_rows=10usize paginator_position=PaginatorPosition::Top /> }
            .to_html()
    });
    assert!(html.contains("p-dataview-paginator-top") && !html.contains("p-dataview-paginator-bottom"), "{html}");
    let html = render(|| {
        view! { <DataView value=items(25) list=list() paginator=true default_rows=10usize paginator_position=PaginatorPosition::Both /> }
            .to_html()
    });
    assert!(html.contains("p-dataview-paginator-top") && html.contains("p-dataview-paginator-bottom"), "{html}");
    let html = render(|| {
        view! { <DataView value=items(25) list=list() paginator=true default_rows=10usize paginator_style_class="mine" /> }.to_html()
    });
    assert!(html.contains("p-dataview-paginator-bottom mine"), "{html}");
}

#[test]
fn sort_and_filter() {
    let cmp = Callback::new(|(a, b): (i32, i32)| a.cmp(&b));
    let html = render(|| view! { <DataView value=vec![3, 1, 2] list=list() compare=cmp sort_order=-1 /> }.to_html());
    let order: Vec<usize> = ["class=\"item\">3", "class=\"item\">2", "class=\"item\">1"].iter().map(|n| html.find(n).unwrap()).collect();
    assert!(order.windows(2).all(|w| w[0] < w[1]), "по убыванию: {html}");
    let only_odd = Callback::new(|n: i32| n % 2 == 1);
    let html = render(|| view! { <DataView value=items(6) list=list() filter=only_odd /> }.to_html());
    assert_eq!(html.matches("class=\"item\"").count(), 3, "{html}");
}

#[test]
fn empty_loading_header_footer() {
    let html = render(|| view! { <DataView value={Vec::<i32>::new()} list=list() /> }.to_html());
    assert!(html.contains("p-dataview-empty-message") && html.contains("No results found"), "{html}");
    let html = render(|| view! { <DataView value={Vec::<i32>::new()} list=list() empty_message="Nothing here" /> }.to_html());
    assert!(html.contains("Nothing here"), "{html}");
    let html =
        render(|| view! { <DataView value={Vec::<i32>::new()} list=list() empty=ViewFn::from(|| view! { <i>"Custom"</i> }) /> }.to_html());
    assert!(html.contains("<i>Custom</i>"), "{html}");
    let html = render(|| view! { <DataView value={Vec::<i32>::new()} list=list() loading=true /> }.to_html());
    assert!(html.contains("p-dataview-loading-overlay p-overlay-mask") && html.contains("p-dataview-loading-icon"), "{html}");
    assert!(!html.contains("p-dataview-empty-message"), "при загрузке сообщения нет: {html}");
    let html = render(|| {
        view! {
            <DataView
                value=items(1)
                list=list()
                header=ViewFn::from(|| view! { <h3>"Head"</h3> })
                footer=ViewFn::from(|| view! { <em>"Foot"</em> })
            />
        }
        .to_html()
    });
    assert!(
        html.contains("p-dataview-header") && html.contains("Head") && html.contains("p-dataview-footer") && html.contains("Foot"),
        "{html}"
    );
}

#[test]
fn lazy_shows_value_as_is() {
    let html = render(|| {
        view! { <DataView value=items(10) list=list() paginator=true lazy=true total_records=100usize default_rows=10usize /> }.to_html()
    });
    assert_eq!(html.matches("class=\"item\"").count(), 10, "{html}");
    assert!(html.contains(">10<"), "страницы считаются по total_records: {html}");
}

#[test]
fn style_and_pass_through() {
    let pt = DataViewPt { root: Pt::new().class("root-x"), content: Pt::new().class("content-x"), ..Default::default() };
    let html = render(|| view! { <DataView value=items(1) list=list() style="width: 30rem" style_class="mine" pt=pt /> }.to_html());
    assert!(html.contains("root-x") && html.contains("mine") && html.contains("width: 30rem"), "{html}");
    assert!(html.contains("p-dataview-content content-x"), "{html}");
}
