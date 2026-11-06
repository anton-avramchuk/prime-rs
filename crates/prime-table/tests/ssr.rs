//! Разметка Table (SSR): столбцы и строки, сортировка, пагинатор, выбор, раскрытие, модификаторы, пустое состояние, `pt`.
#![cfg(feature = "ssr")]
#![recursion_limit = "512"]

use leptos::prelude::*;
use prime_core::{Pt, Size};
use prime_table::model::{SelectionMode, SortMeta, SortMode};
use prime_table::{Column, PaginatorPosition, Table, TablePt};

#[derive(Clone, Debug)]
struct Product {
    code: &'static str,
    name: &'static str,
    quantity: u32,
}

fn products() -> Vec<Product> {
    vec![
        Product { code: "c1", name: "Watch", quantity: 24 },
        Product { code: "c2", name: "Band", quantity: 61 },
        Product { code: "c3", name: "Shirt", quantity: 2 },
        Product { code: "c4", name: "Purse", quantity: 61 },
    ]
}

fn render(f: impl FnOnce() -> String) -> String {
    Owner::new().with(f)
}

fn key() -> Callback<Product, String> {
    Callback::new(|p: Product| p.code.to_owned())
}

fn plain_columns() -> Vec<Column<Product>> {
    vec![Column::text("code", "Code", |p: &Product| p.code.to_owned()), Column::text("name", "Name", |p: &Product| p.name.to_owned())]
}

fn sortable_columns() -> Vec<Column<Product>> {
    vec![
        Column::text("code", "Code", |p: &Product| p.code.to_owned()).sortable(|a, b| a.code.cmp(b.code)),
        Column::text("name", "Name", |p: &Product| p.name.to_owned()).sortable(|a, b| a.name.cmp(b.name)),
        Column::text("quantity", "Quantity", |p: &Product| p.quantity.to_string()).sortable(|a, b| a.quantity.cmp(&b.quantity)),
    ]
}

/// Открывающий тег элемента, в котором встречается подстрока.
fn tag_of<'a>(html: &'a str, needle: &str) -> &'a str {
    let at = html.find(needle).unwrap_or_else(|| panic!("нет {needle}: {html}"));
    let start = html[..at].rfind('<').unwrap();
    let end = at + html[at..].find('>').unwrap();
    &html[start..=end]
}

fn pos(html: &str, needle: &str) -> usize {
    html.find(needle).unwrap_or_else(|| panic!("нет {needle}: {html}"))
}

#[test]
fn basic_markup() {
    let html = render(|| view! { <Table value=products() columns=plain_columns() row_key=key() /> }.to_html());
    assert!(html.contains("p-datatable p-component"), "{html}");
    assert!(
        html.contains("p-datatable-table-container")
            && html.contains("p-datatable-table")
            && html.contains("p-datatable-thead")
            && html.contains("p-datatable-tbody"),
        "{html}"
    );
    assert!(html.contains(">Code<") && html.contains(">Name<"), "{html}");
    assert_eq!(html.matches("<tr role=\"row\"").count(), 5, "заголовок и четыре строки: {html}");
    assert!(html.contains(">Watch<") && html.contains(">c4<"), "{html}");
    assert!(!html.contains("p-datatable-tfoot") && !html.contains("p-datatable-mask") && !html.contains("p-paginator"), "{html}");
    assert!(tag_of(&html, "p-datatable-table-container").contains("overflow: auto"), "{html}");
}

#[test]
fn modifiers() {
    let html = render(|| {
        view! { <Table value=products() columns=plain_columns() row_key=key() striped_rows=true show_gridlines=true size=Size::Small row_hover=true /> }.to_html()
    });
    assert!(html.contains("p-datatable-hoverable p-datatable-striped p-datatable-gridlines p-datatable-sm"), "{html}");
    let html = render(|| view! { <Table value=products() columns=plain_columns() row_key=key() size=Size::Large /> }.to_html());
    assert!(html.contains("p-datatable-lg") && !html.contains("p-datatable-hoverable"), "{html}");
    let html = render(|| {
        view! { <Table value=products() columns=plain_columns() row_key=key() scrollable=true scroll_height="200px" /> }.to_html()
    });
    assert!(html.contains("p-datatable-scrollable ") || html.contains("p-datatable-scrollable\""), "{html}");
    assert!(html.contains("p-datatable-scrollable-table") && html.contains("max-height: 200px"), "{html}");
    let html = render(|| {
        view! { <Table value=products() columns=plain_columns() row_key=key() scrollable=true scroll_height="flex" /> }.to_html()
    });
    assert!(html.contains("p-datatable-flex-scrollable") && !html.contains("max-height"), "{html}");
}

#[test]
fn sortable_headers_and_order() {
    let html = render(|| view! { <Table value=products() columns=sortable_columns() row_key=key() /> }.to_html());
    let code = tag_of(&html, ">Code<");
    let th = &html[html.find("<th").unwrap()..];
    assert!(th.starts_with("<th") && th.contains("p-datatable-sortable-column"), "{html}");
    assert!(html.contains("aria-sort=\"none\"") && html.contains("tabindex=\"0\""), "{html} {code}");
    assert!(!html.contains("p-datatable-column-sorted"), "{html}");
    assert_eq!(html.matches("p-datatable-sort-icon").count(), 3, "по значку в каждом заголовке: {html}");

    let html = render(|| {
        let sort = RwSignal::new(vec![SortMeta::new("name", -1)]);
        view! { <Table value=products() columns=sortable_columns() row_key=key() sort=sort /> }.to_html()
    });
    assert!(html.contains("aria-sort=\"descending\"") && html.contains("p-datatable-column-sorted"), "{html}");
    let order: Vec<usize> = ["Watch", "Shirt", "Purse", "Band"].iter().map(|n| pos(&html, &format!(">{n}<"))).collect();
    assert!(order.windows(2).all(|w| w[0] < w[1]), "по убыванию имени: {html}");

    let html = render(|| {
        let sort = RwSignal::new(vec![SortMeta::new("quantity", 1), SortMeta::new("name", 1)]);
        view! { <Table value=products() columns=sortable_columns() row_key=key() sort_mode=SortMode::Multiple sort=sort /> }.to_html()
    });
    let order: Vec<usize> = ["Shirt", "Watch", "Band", "Purse"].iter().map(|n| pos(&html, &format!(">{n}<"))).collect();
    assert!(order.windows(2).all(|w| w[0] < w[1]), "по количеству, затем по имени: {html}");
    assert!(html.contains("p-sortable-column-badge"), "в многоуровневой сортировке есть счётчики: {html}");
}

#[test]
fn paginator_slices_rows() {
    let html =
        render(|| view! { <Table value=products() columns=plain_columns() row_key=key() paginator=true default_rows=3usize /> }.to_html());
    assert!(html.contains("p-datatable-paginator-bottom") && html.contains("p-paginator"), "{html}");
    assert!(html.contains(">c3<") && !html.contains(">c4<"), "{html}");
    let html = render(|| {
        view! { <Table value=products() columns=plain_columns() row_key=key() paginator=true default_rows=3usize paginator_position=PaginatorPosition::Top /> }.to_html()
    });
    assert!(html.contains("p-datatable-paginator-top") && !html.contains("p-datatable-paginator-bottom"), "{html}");
    let html = render(|| {
        let first = RwSignal::new(3usize);
        view! { <Table value=products() columns=plain_columns() row_key=key() paginator=true default_rows=3usize first=first /> }.to_html()
    });
    assert!(html.contains(">c4<") && !html.contains(">c1<"), "вторая страница: {html}");
}

#[test]
fn row_selection_classes() {
    let html = render(|| {
        let selection = RwSignal::new(vec!["c2".to_owned()]);
        view! { <Table value=products() columns=plain_columns() row_key=key() selection_mode=Some(SelectionMode::Single) selection=selection /> }.to_html()
    });
    assert_eq!(html.matches("p-datatable-selectable-row").count(), 4, "{html}");
    assert_eq!(html.matches("p-datatable-row-selected").count(), 1, "{html}");
    assert_eq!(html.matches("aria-selected=\"true\"").count(), 1, "{html}");
    assert_eq!(html.matches("aria-selected=\"false\"").count(), 3, "{html}");
    assert!(html.contains("p-datatable-hoverable"), "{html}");
}

#[test]
fn checkbox_and_radio_columns() {
    let mut columns = vec![Column::selection_checkbox()];
    columns.extend(plain_columns());
    let html = render(|| {
        let selection = RwSignal::new(vec!["c1".to_owned(), "c2".to_owned()]);
        view! { <Table value=products() columns=columns.clone() row_key=key() selection=selection /> }.to_html()
    });
    assert_eq!(html.matches("p-checkbox ").count() + html.matches("class=\"p-checkbox\"").count(), 5, "заголовок и четыре строки: {html}");
    assert_eq!(html.matches("p-checkbox-checked").count(), 2, "{html}");
    assert!(html.contains("aria-label=\"Select All\"") || html.contains("p-checkbox"), "{html}");
    let mut columns = vec![Column::selection_radio()];
    columns.extend(plain_columns());
    let html = render(|| {
        let selection = RwSignal::new(vec!["c3".to_owned()]);
        view! { <Table value=products() columns=columns.clone() row_key=key() selection=selection /> }.to_html()
    });
    assert_eq!(html.matches("p-radiobutton p-component").count(), 4, "{html}");
    assert_eq!(html.matches("p-radiobutton-checked").count(), 1, "{html}");
}

#[test]
fn row_expansion() {
    let mut columns = vec![Column::expander()];
    columns.extend(plain_columns());
    let expansion = Callback::new(|p: Product| view! { <span class="detail">{format!("Details of {}", p.name)}</span> }.into_any());
    let html = render(|| {
        let expanded = RwSignal::new(vec!["c2".to_owned()]);
        view! { <Table value=products() columns=columns.clone() row_key=key() expanded_keys=expanded row_expansion=expansion /> }.to_html()
    });
    assert_eq!(html.matches("p-datatable-row-toggle-button").count(), 4, "{html}");
    assert!(html.contains("p-datatable-row-expansion") && html.contains("Details of Band") && !html.contains("Details of Watch"), "{html}");
    assert_eq!(html.matches("p-datatable-row-expansion").count(), 1, "{html}");
    let expansion_at = pos(&html, "Details of Band");
    assert!(pos(&html, ">Band<") < expansion_at && expansion_at < pos(&html, ">Shirt<"), "раскрытие сразу после своей строки: {html}");
    assert!(html.contains("colspan=\"3\""), "{html}");
}

#[test]
fn empty_loading_caption_footer() {
    let html = render(|| view! { <Table value={Vec::<Product>::new()} columns=plain_columns() row_key=key() /> }.to_html());
    assert!(html.contains("p-datatable-empty-message") && html.contains("No results found") && html.contains("colspan=\"2\""), "{html}");
    let html = render(|| {
        view! { <Table value={Vec::<Product>::new()} columns=plain_columns() row_key=key() empty_message="Nothing" /> }.to_html()
    });
    assert!(html.contains("Nothing"), "{html}");
    let html = render(|| view! { <Table value=products() columns=plain_columns() row_key=key() loading=true /> }.to_html());
    assert!(html.contains("p-datatable-mask p-overlay-mask") && html.contains("p-datatable-loading-icon"), "{html}");
    let html = render(|| view! { <Table value={Vec::<Product>::new()} columns=plain_columns() row_key=key() loading=true /> }.to_html());
    assert!(!html.contains("p-datatable-empty-message"), "при загрузке сообщения нет: {html}");
    let columns = vec![
        Column::text("code", "Code", |p: &Product| p.code.to_owned()).footer("Total"),
        Column::text("name", "Name", |p: &Product| p.name.to_owned()).footer("4 items"),
    ];
    let html = render(|| {
        view! {
            <Table
                value=products()
                columns=columns.clone()
                row_key=key()
                caption=ViewFn::from(|| view! { <b>"Products"</b> })
                summary=ViewFn::from(|| view! { <i>"In total there are 4"</i> })
            />
        }
        .to_html()
    });
    assert!(html.contains("p-datatable-header") && html.contains("<b>Products</b>"), "{html}");
    assert!(html.contains("p-datatable-tfoot") && html.contains(">Total<") && html.contains(">4 items<"), "{html}");
    assert!(html.contains("p-datatable-footer") && html.contains("In total there are 4"), "{html}");
    assert!(
        pos(&html, "p-datatable-header") < pos(&html, "p-datatable-table-container")
            && pos(&html, "p-datatable-table-container") < pos(&html, "p-datatable-footer\""),
        "{html}"
    );
}

#[test]
fn style_and_pass_through() {
    let pt = TablePt { root: Pt::new().class("root-x"), thead: Pt::new().class("thead-x"), ..Default::default() };
    let html = render(|| {
        view! { <Table value=products() columns=plain_columns() row_key=key() style="width: 40rem" style_class="mine" table_style="min-width: 30rem" pt=pt /> }.to_html()
    });
    assert!(
        html.contains("root-x") && html.contains("mine") && html.contains("width: 40rem") && html.contains("min-width: 30rem"),
        "{html}"
    );
    assert!(html.contains("p-datatable-thead thead-x"), "{html}");
}
