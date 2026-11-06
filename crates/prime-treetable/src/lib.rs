//! TreeTable — порт `p-treetable` из PrimeNG 21.1.10, этап 1.
//!
//! Иерархические данные в таблице: узлы с потомками, кнопка раскрытия с отступом по уровню, сортировка на каждом уровне,
//! пагинатор по корневым узлам, выбор (клик, Ctrl/⌘, флажки с распространением на потомков и родителей), индикатор
//! загрузки. Столбцы — [`prime_table::Column`], логика — чистая [`tree`]. Фильтры, правка, изменение и перестановка
//! столбцов, заморозка и ленивая загрузка — следующие этапы.
//!
//! ```
//! #![recursion_limit = "512"]
//! use leptos::prelude::*;
//! use prime_table::Column;
//! use prime_treetable::{TreeNode, TreeTable};
//!
//! let nodes = vec![TreeNode::new("docs", "Documents").expanded(true).children(vec![TreeNode::new("cv", "Resume.pdf")])];
//! let columns = vec![Column::text("name", "Name", |n: &&str| (*n).to_owned())];
//! let html = view! { <TreeTable value=nodes columns=columns /> }.to_html();
//! assert!(html.contains("p-treetable") && html.contains("Resume.pdf") && html.contains("p-treetable-toggler"));
//! ```

#![recursion_limit = "512"]

pub mod tree;

use leptos::attr::any_attribute::{AnyAttribute, IntoAnyAttribute};
use leptos::attr::custom::custom_attribute;
use leptos::prelude::*;
use prime_badge::{Badge, BadgeSize};
use prime_checkbox::Checkbox;
use prime_core::{OptProp, Prop, Pt, Size, use_prime_config};
use prime_dataview::data::clamp_first;
use prime_icons::{ChevronDownIcon, ChevronRightIcon, SortAltIcon, SortAmountDownIcon, SortAmountUpAltIcon, SpinnerIcon};
use prime_paginator::{Paginator, PaginatorPt, PaginatorState, RowsPerPage};
use prime_table::model::{SelectionMode, SortMeta, SortMode, badge_of, order_of, row_click, toggle_expanded, toggle_sort};
use std::cmp::Ordering;
use std::sync::Arc;
use tree::{CheckState, RowNode, check_state, checkbox_toggle, find, flatten, initial_expanded, page_roots, sort_nodes};

pub use prime_table::{Column, PageEvent, PaginatorPosition};
pub use tree::TreeNode;

/// Режим выбора.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TreeSelectionMode {
    /// Один узел кликом.
    Single,
    /// Несколько узлов кликом.
    Multiple,
    /// Флажки с распространением на потомков и родителей.
    Checkbox,
}

/// Pass-through частей [`TreeTable`].
#[derive(Clone, Debug, Default)]
pub struct TreeTablePt {
    /// Корень.
    pub root: Pt,
    /// Контейнер таблицы.
    pub table_container: Pt,
    /// Таблица.
    pub table: Pt,
    /// Заголовок таблицы (`thead`).
    pub thead: Pt,
    /// Тело (`tbody`).
    pub tbody: Pt,
    /// Подвал (`tfoot`).
    pub tfoot: Pt,
    /// Части пагинатора.
    pub paginator: PaginatorPt,
}

/// Состояние, общее для обработчиков.
struct Core<T: Send + Sync + 'static> {
    value: Prop<Vec<TreeNode<T>>>,
    columns: Prop<Vec<Column<T>>>,
    selection: RwSignal<Vec<String>>,
    expanded: RwSignal<Vec<String>>,
    sort: RwSignal<Vec<SortMeta>>,
    first: RwSignal<usize>,
    rows: RwSignal<usize>,
    selection_mode: Prop<Option<TreeSelectionMode>>,
    meta_key_selection: Prop<bool>,
    sort_mode: Prop<SortMode>,
    default_sort_order: Prop<i32>,
    reset_page_on_sort: Prop<bool>,
    paginator: Prop<bool>,
    toggler_column: usize,
    on_sort: Option<Callback<Vec<SortMeta>>>,
    on_page: Option<Callback<PageEvent>>,
    on_node_select: Option<Callback<T>>,
    on_node_unselect: Option<Callback<T>>,
    on_node_expand: Option<Callback<T>>,
    on_node_collapse: Option<Callback<T>>,
}

impl<T: Clone + Send + Sync + 'static> Core<T> {
    /// Дерево после сортировки на каждом уровне.
    fn sorted(&self) -> Vec<TreeNode<T>> {
        let columns = self.columns.get();
        let meta = self.sort.get();
        let compare = |field: &str, a: &T, b: &T| {
            columns.iter().find(|c| c.field == field).and_then(|c| c.compare).map_or(Ordering::Equal, |cb| cb.run((a.clone(), b.clone())))
        };
        self.value.with(|roots| sort_nodes(roots, &meta, &compare))
    }

    /// Видимые строки текущей страницы.
    fn visible_rows(&self) -> Vec<RowNode<T>> {
        let sorted = self.sorted();
        let rows = self.rows.get();
        let first = clamp_first(self.first.get(), sorted.len(), rows);
        let page = page_roots(&sorted, self.paginator.get(), first, rows);
        self.expanded.with(|expanded| flatten(&page, expanded))
    }

    fn click_sort(&self, field: &str, meta_key: bool) {
        let next = toggle_sort(
            self.sort_mode.get_untracked(),
            &self.sort.get_untracked(),
            field,
            meta_key,
            self.default_sort_order.get_untracked(),
        );
        if self.reset_page_on_sort.get_untracked() {
            self.first.set(0);
        }
        self.sort.set(next.clone());
        if let Some(cb) = self.on_sort {
            cb.run(next);
        }
    }

    fn toggle_node(&self, row: &RowNode<T>) {
        let (next, open) = toggle_expanded(&self.expanded.get_untracked(), &row.key, false);
        self.expanded.set(next);
        let callback = if open { self.on_node_expand } else { self.on_node_collapse };
        if let Some(cb) = callback {
            cb.run(row.data.clone());
        }
    }

    fn row_clicked(&self, row: &RowNode<T>, meta_key: bool) {
        let mode = match self.selection_mode.get_untracked() {
            Some(TreeSelectionMode::Single) => SelectionMode::Single,
            Some(TreeSelectionMode::Multiple) => SelectionMode::Multiple,
            _ => return,
        };
        let result = row_click(mode, &self.selection.get_untracked(), &row.key, self.meta_key_selection.get_untracked(), meta_key);
        self.selection.set(result.selection);
        let callback = if result.selected { self.on_node_select } else { self.on_node_unselect };
        if let Some(cb) = callback {
            cb.run(row.data.clone());
        }
    }

    fn check_node(&self, row: &RowNode<T>, select: bool) {
        let next = checkbox_toggle(&self.value.get_untracked(), &self.selection.get_untracked(), &row.key, select);
        self.selection.set(next);
        let callback = if select { self.on_node_select } else { self.on_node_unselect };
        if let Some(cb) = callback {
            cb.run(row.data.clone());
        }
    }

    fn paginate(&self, state: PaginatorState) {
        self.first.set(state.first);
        self.rows.set(state.rows);
        if let Some(cb) = self.on_page {
            cb.run(PageEvent { first: state.first, rows: state.rows });
        }
    }
}

/// Иерархическая таблица данных.
///
/// Каждое свойство принимает значение или сигнал; `selection`, `expanded_keys`, `sort`, `first` и `rows` — двусторонние сигналы.
#[component]
pub fn TreeTable<T>(
    /// Корневые узлы.
    #[prop(optional, into)]
    value: Prop<Vec<TreeNode<T>>>,
    /// Столбцы (используются столбцы данных).
    #[prop(optional, into)]
    columns: Prop<Vec<Column<T>>>,
    /// Номер столбца, в котором выводятся кнопка раскрытия и флажок.
    #[prop(optional)]
    toggler_column: usize,
    /// Режим выбора.
    #[prop(optional, into)]
    selection_mode: Prop<Option<TreeSelectionMode>>,
    /// Двусторонний сигнал ключей выбранных узлов.
    #[prop(optional)]
    selection: Option<RwSignal<Vec<String>>>,
    /// Для выбора нужна клавиша Ctrl/⌘.
    #[prop(optional, into)]
    meta_key_selection: Prop<bool>,
    /// Двусторонний сигнал ключей раскрытых узлов; без него начальное раскрытие берётся из `TreeNode::expanded`.
    #[prop(optional)]
    expanded_keys: Option<RwSignal<Vec<String>>>,
    /// Режим сортировки.
    #[prop(optional, into)]
    sort_mode: Prop<SortMode>,
    /// Двусторонний сигнал сортировки.
    #[prop(optional)]
    sort: Option<RwSignal<Vec<SortMeta>>>,
    /// Порядок при первой сортировке столбца (1 или -1).
    #[prop(optional, default = Prop::Static(1), into)]
    default_sort_order: Prop<i32>,
    /// Возвращаться на первую страницу при сортировке.
    #[prop(optional, default = Prop::Static(true), into)]
    reset_page_on_sort: Prop<bool>,
    /// Включить пагинатор (по корневым узлам).
    #[prop(optional, into)]
    paginator: Prop<bool>,
    /// Двусторонний сигнал индекса первого корня страницы.
    #[prop(optional)]
    first: Option<RwSignal<usize>>,
    /// Двусторонний сигнал числа корней на странице.
    #[prop(optional)]
    rows: Option<RwSignal<usize>>,
    /// Число корней по умолчанию (когда `rows` не передан).
    #[prop(optional, default = 10)]
    default_rows: usize,
    /// Число ссылок на страницы.
    #[prop(optional, default = Prop::Static(5), into)]
    page_links: Prop<usize>,
    /// Варианты числа корней на странице.
    #[prop(optional, into)]
    rows_per_page_options: Prop<Vec<RowsPerPage>>,
    /// Положение пагинатора.
    #[prop(optional, into)]
    paginator_position: Prop<PaginatorPosition>,
    /// Показывать пагинатор при единственной странице.
    #[prop(optional, default = Prop::Static(true), into)]
    always_show_paginator: Prop<bool>,
    /// Подсвечивать строку под курсором.
    #[prop(optional, into)]
    row_hover: Prop<bool>,
    /// Линии сетки.
    #[prop(optional, into)]
    show_gridlines: Prop<bool>,
    /// Размер.
    #[prop(optional, into)]
    size: OptProp<Size>,
    /// Прокручиваемое тело.
    #[prop(optional, into)]
    scrollable: Prop<bool>,
    /// Высота области прокрутки.
    #[prop(optional, into)]
    scroll_height: OptProp<String>,
    /// Показывать индикатор загрузки.
    #[prop(optional, into)]
    loading: Prop<bool>,
    /// Текст при отсутствии узлов.
    #[prop(optional, into)]
    empty_message: OptProp<String>,
    /// Заголовок над таблицей.
    #[prop(optional, into)]
    caption: Option<ViewFn>,
    /// Подвал под таблицей.
    #[prop(optional, into)]
    summary: Option<ViewFn>,
    /// Встроенный стиль таблицы.
    #[prop(optional, into)]
    table_style: OptProp<String>,
    /// Атрибут `aria-label` таблицы.
    #[prop(optional, into)]
    aria_label: OptProp<String>,
    /// Встроенный стиль корня.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Дополнительный класс корня.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Вызывается после смены сортировки.
    #[prop(optional, into)]
    on_sort: Option<Callback<Vec<SortMeta>>>,
    /// Вызывается при смене страницы.
    #[prop(optional, into)]
    on_page: Option<Callback<PageEvent>>,
    /// Вызывается при выборе узла.
    #[prop(optional, into)]
    on_node_select: Option<Callback<T>>,
    /// Вызывается при снятии выбора узла.
    #[prop(optional, into)]
    on_node_unselect: Option<Callback<T>>,
    /// Вызывается при раскрытии узла.
    #[prop(optional, into)]
    on_node_expand: Option<Callback<T>>,
    /// Вызывается при сворачивании узла.
    #[prop(optional, into)]
    on_node_collapse: Option<Callback<T>>,
    /// Pass-through частей.
    #[prop(optional)]
    pt: Option<TreeTablePt>,
) -> impl IntoView
where
    T: Clone + Send + Sync + 'static,
{
    let pt = Arc::new(pt.unwrap_or_default());
    let expanded = expanded_keys.unwrap_or_else(|| RwSignal::new(initial_expanded(&value.get_untracked())));
    let core = Arc::new(Core {
        value,
        columns,
        selection: selection.unwrap_or_default(),
        expanded,
        sort: sort.unwrap_or_default(),
        first: first.unwrap_or_default(),
        rows: rows.unwrap_or_else(|| RwSignal::new(default_rows)),
        selection_mode: selection_mode.clone(),
        meta_key_selection,
        sort_mode,
        default_sort_order,
        reset_page_on_sort,
        paginator: paginator.clone(),
        toggler_column,
        on_sort,
        on_page,
        on_node_select,
        on_node_unselect,
        on_node_expand,
        on_node_collapse,
    });

    // Возврат `first` на допустимую страницу — только в браузере.
    #[cfg(target_arch = "wasm32")]
    {
        let c = core.clone();
        Effect::new(move |_| {
            let total = c.value.with(Vec::len);
            let (first, rows) = (c.first.get_untracked(), c.rows.get());
            let clamped = clamp_first(first, total, rows);
            if clamped != first {
                c.first.set(clamped);
            }
        });
    }

    let root_class = {
        let (pt, selection_mode, size) = (pt.clone(), selection_mode.clone(), size.clone());
        let (row_hover, show_gridlines, scrollable, scroll_height) =
            (row_hover.clone(), show_gridlines.clone(), scrollable.clone(), scroll_height.clone());
        move || {
            let mut cn = String::from("p-treetable p-component");
            if show_gridlines.get() {
                cn.push_str(" p-treetable-gridlines");
            }
            if row_hover.get() || matches!(selection_mode.get(), Some(TreeSelectionMode::Single | TreeSelectionMode::Multiple)) {
                cn.push_str(" p-treetable-hoverable-rows");
            }
            if scrollable.get() && scroll_height.get().as_deref() == Some("flex") {
                cn.push_str(" p-treetable-flex-scrollable");
            }
            match size.get() {
                Some(Size::Small) => cn.push_str(" p-treetable-sm"),
                Some(Size::Large) => cn.push_str(" p-treetable-lg"),
                None => {}
            }
            style_class.with(|c| {
                if let Some(c) = c.as_deref().filter(|c| !c.is_empty()) {
                    cn.push(' ');
                    cn.push_str(c);
                }
            });
            pt.root.merge_class(&cn).into_owned()
        }
    };
    let root_style = {
        let pt = pt.clone();
        move || {
            let merged = style.with(|st| pt.root.merge_style(st.as_deref().unwrap_or_default()));
            (!merged.is_empty()).then_some(merged)
        }
    };
    let mask_loading = loading.clone();
    let mask = move || {
        mask_loading.get().then(
            || view! { <div class="p-treetable-mask p-overlay-mask"><SpinnerIcon class="p-treetable-loading-icon" spin=true /></div> },
        )
    };
    let caption_view = caption.map(|c| view! { <div class="p-treetable-header">{c.run()}</div> });
    let summary_view = summary.map(|s| view! { <div class="p-treetable-footer">{s.run()}</div> });

    let paginator_view = |position: &'static str| {
        let (core, pt) = (core.clone(), pt.clone());
        let (paginator_position, rows_options, page_links, always_show) =
            (paginator_position.clone(), rows_per_page_options.clone(), page_links.clone(), always_show_paginator.clone());
        move || {
            let wanted = match paginator_position.get() {
                PaginatorPosition::Top => position == "top",
                PaginatorPosition::Bottom => position == "bottom",
                PaginatorPosition::Both => true,
            };
            (core.paginator.get() && wanted).then(|| {
                let total = {
                    let core = core.clone();
                    Signal::derive(move || core.value.with(Vec::len))
                };
                let on_page_change = {
                    let core = core.clone();
                    Callback::new(move |state: PaginatorState| core.paginate(state))
                };
                view! {
                    <Paginator
                        first=core.first
                        rows=core.rows
                        total_records=total
                        rows_per_page_options=rows_options.clone()
                        page_link_size=Signal::derive({
                            let page_links = page_links.clone();
                            move || Some(page_links.get())
                        })
                        always_show=Signal::derive({
                            let always_show = always_show.clone();
                            move || Some(always_show.get())
                        })
                        style_class=format!("p-treetable-paginator-{position}")
                        on_page_change=on_page_change
                        pt=pt.paginator.clone()
                    />
                }
            })
        }
    };
    let top = paginator_view("top");
    let bottom = paginator_view("bottom");

    let container_style = {
        let scroll_height = scroll_height.clone();
        move || scroll_height.get().filter(|h| !h.is_empty() && h != "flex").map(|h| format!("max-height: {h}; overflow: auto"))
    };
    let table_class = {
        let (pt, scrollable) = (pt.clone(), scrollable.clone());
        move || {
            let base = if scrollable.get() { "p-treetable-table p-treetable-scrollable-table" } else { "p-treetable-table" };
            pt.table.merge_class(base).into_owned()
        }
    };
    let head = {
        let core = core.clone();
        move || head_view(&core)
    };
    let body = {
        let (core, empty_message) = (core.clone(), empty_message.clone());
        move || body_view(&core, &empty_message, loading.get())
    };
    let foot = {
        let (core, pt) = (core.clone(), pt.clone());
        move || foot_view(&core, &pt)
    };

    view! {
        <div class={root_class} style={root_style} data-pc-name="treetable" data-pc-section="root" {..pt.root.attrs()}>
            {mask}
            {caption_view}
            {top}
            <div class={pt.table_container.merge_class("p-treetable-table-container")} style={container_style} {..pt.table_container.attrs()}>
                <table role="treegrid" class={table_class} style={move || table_style.get()} aria-label={move || aria_label.get()} {..pt.table.attrs()}>
                    <thead role="rowgroup" class={pt.thead.merge_class("p-treetable-thead")} {..pt.thead.attrs()}>
                        {head}
                    </thead>
                    <tbody role="rowgroup" class={pt.tbody.merge_class("p-treetable-tbody")} {..pt.tbody.attrs()}>
                        {body}
                    </tbody>
                    {foot}
                </table>
            </div>
            {bottom}
            {summary_view}
        </div>
    }
}

fn cell_attrs<T: Clone + Send + Sync + 'static>(column: &Column<T>) -> (String, Option<String>) {
    (column.style_class.clone().unwrap_or_default(), column.style.clone())
}

/// Строка заголовков.
fn head_view<T: Clone + Send + Sync + 'static>(core: &Arc<Core<T>>) -> impl IntoView + use<T> {
    let columns = core.columns.get();
    let cells = columns.iter().map(|column| header_cell(core, column)).collect_view();
    view! { <tr role="row">{cells}</tr> }
}

fn header_cell<T: Clone + Send + Sync + 'static>(core: &Arc<Core<T>>, column: &Column<T>) -> AnyView {
    let (extra_class, style) = cell_attrs(column);
    let title = column.header.clone().unwrap_or_default();
    if !(column.sortable && column.compare.is_some()) {
        return view! { <th class={extra_class} style={style} role="columnheader">{title}</th> }.into_any();
    }
    let field = column.field.clone();
    let (class_core, aria_core, icon_core, badge_core, click_core, key_core) =
        (core.clone(), core.clone(), core.clone(), core.clone(), core.clone(), core.clone());
    let (class_field, aria_field, icon_field, badge_field, click_field, key_field) =
        (field.clone(), field.clone(), field.clone(), field.clone(), field.clone(), field);
    let class = move || {
        let mut cn = String::from("p-sortable-column");
        if order_of(&class_core.sort.get(), &class_field) != 0 {
            cn.push_str(" p-treetable-column-sorted");
        }
        if !extra_class.is_empty() {
            cn.push(' ');
            cn.push_str(&extra_class);
        }
        cn
    };
    let aria_sort = move || match order_of(&aria_core.sort.get(), &aria_field) {
        1 => "ascending",
        -1 => "descending",
        _ => "none",
    };
    let icon = move || match order_of(&icon_core.sort.get(), &icon_field) {
        1 => view! { <SortAmountUpAltIcon class="p-treetable-sort-icon" /> }.into_any(),
        -1 => view! { <SortAmountDownIcon class="p-treetable-sort-icon" /> }.into_any(),
        _ => view! { <SortAltIcon class="p-treetable-sort-icon" /> }.into_any(),
    };
    let badge = move || {
        badge_of(badge_core.sort_mode.get(), &badge_core.sort.get(), &badge_field)
            .map(|n| view! { <Badge value=n.to_string() size=BadgeSize::Small style_class="p-sortable-column-badge" /> })
    };
    view! {
        <th
            class={class}
            style={style}
            role="columnheader"
            tabindex="0"
            aria-sort={aria_sort}
            on:click=move |e: web_sys::MouseEvent| click_core.click_sort(&click_field, e.meta_key() || e.ctrl_key())
            on:keydown=move |e: web_sys::KeyboardEvent| {
                if e.key() == "Enter" || e.key() == " " {
                    key_core.click_sort(&key_field, e.meta_key() || e.ctrl_key());
                    e.prevent_default();
                }
            }
        >
            <div class="p-treetable-column-header-content">
                <span class="p-treetable-column-title">{title}</span>
                {icon}
                {badge}
            </div>
        </th>
    }
    .into_any()
}

fn body_view<T: Clone + Send + Sync + 'static>(
    core: &Arc<Core<T>>,
    empty_message: &OptProp<String>,
    loading: bool,
) -> impl IntoView + use<T> {
    let rows = core.visible_rows();
    let columns = core.columns.get();
    if rows.is_empty() {
        let text =
            empty_message.get().filter(|m| !m.is_empty()).unwrap_or_else(|| use_prime_config().locale.text("emptyMessage").to_owned());
        let colspan = columns.len().max(1).to_string();
        return (!loading)
            .then(|| view! { <tr class="p-treetable-empty-message" role="row"><td colspan=colspan>{text}</td></tr> })
            .into_any();
    }
    rows.into_iter().map(|row| row_view(core, &columns, row)).collect_view().into_any()
}

fn row_view<T: Clone + Send + Sync + 'static>(core: &Arc<Core<T>>, columns: &[Column<T>], row: RowNode<T>) -> impl IntoView + use<T> {
    let (class_core, aria_core, click_core) = (core.clone(), core.clone(), core.clone());
    let (class_key, aria_key) = (row.key.clone(), row.key.clone());
    let click_row = row.clone();
    let class = move || {
        let mode = class_core.selection_mode.get();
        let selected = mode.is_some() && class_core.selection.with(|s| s.contains(&class_key));
        selected.then_some("p-treetable-row-selected")
    };
    let aria_selected = move || aria_core.selection_mode.get().map(|_| aria_core.selection.with(|s| s.contains(&aria_key)).to_string());
    let aria_expanded = row.expandable.then(|| row.expanded.to_string());
    let level_attrs: Vec<AnyAttribute> = [("aria-level", row.level + 1), ("aria-posinset", row.position), ("aria-setsize", row.set_size)]
        .into_iter()
        .map(|(name, value)| custom_attribute(name, value.to_string()).into_any_attr())
        .collect();
    let cells = columns.iter().enumerate().map(|(i, column)| body_cell(core, column, &row, i == core.toggler_column)).collect_view();
    view! {
        <tr
            class={class}
            role="row"
            aria-expanded={aria_expanded}
            aria-selected={aria_selected}
            on:click=move |e: web_sys::MouseEvent| {
                use leptos::wasm_bindgen::JsCast;
                let interactive = e
                    .target()
                    .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
                    .is_some_and(|el| el.closest("input, button, a, .p-checkbox").ok().flatten().is_some());
                if !interactive {
                    click_core.row_clicked(&click_row, e.meta_key() || e.ctrl_key());
                }
            }
            {..level_attrs}
        >
            {cells}
        </tr>
    }
}

fn body_cell<T: Clone + Send + Sync + 'static>(
    core: &Arc<Core<T>>,
    column: &Column<T>,
    row: &RowNode<T>,
    with_toggler: bool,
) -> impl IntoView + use<T> {
    let (extra_class, style) = cell_attrs(column);
    let content = column.body.map(|body| body.run(row.data.clone()));
    let leading = with_toggler.then(|| {
        let toggle = toggler(core, row);
        let checkbox = (core.selection_mode.get_untracked() == Some(TreeSelectionMode::Checkbox)).then(|| node_checkbox(core, row));
        view! { {toggle}{checkbox} }
    });
    view! { <td class={extra_class} style={style} role="gridcell">{leading}{content}</td> }
}

/// Кнопка раскрытия с отступом по уровню.
fn toggler<T: Clone + Send + Sync + 'static>(core: &Arc<Core<T>>, row: &RowNode<T>) -> impl IntoView + use<T> {
    let (toggle_core, toggle_row) = (core.clone(), row.clone());
    let visibility = if row.expandable { "visible" } else { "hidden" };
    let margin = row.level * 16;
    let icon = if row.expanded {
        view! { <ChevronDownIcon class="p-treetable-node-toggle-icon" /> }.into_any()
    } else {
        view! { <ChevronRightIcon class="p-treetable-node-toggle-icon" /> }.into_any()
    };
    view! {
        <button
            type="button"
            class="p-treetable-toggler"
            tabindex="-1"
            style=format!("visibility: {visibility}; margin-inline-start: {margin}px")
            aria-label=use_prime_config().locale.text(if row.expanded { "aria.collapseRow" } else { "aria.expandRow" }).to_owned()
            on:click=move |e: web_sys::MouseEvent| {
                e.stop_propagation();
                toggle_core.toggle_node(&toggle_row);
            }
        >
            {icon}
        </button>
    }
}

/// Флажок узла с частичным выбором.
fn node_checkbox<T: Clone + Send + Sync + 'static>(core: &Arc<Core<T>>, row: &RowNode<T>) -> impl IntoView + use<T> {
    let state = find(&core.value.get_untracked(), &row.key)
        .map(|n| core.selection.with_untracked(|s| check_state(n, s)))
        .unwrap_or(CheckState::Unchecked);
    let checked = RwSignal::new(state == CheckState::Checked);
    let indeterminate = RwSignal::new(state == CheckState::Partial);
    #[cfg(target_arch = "wasm32")]
    {
        let (sync_core, key) = (core.clone(), row.key.clone());
        Effect::new(move |_| {
            let state = sync_core
                .value
                .with(|roots| find(roots, &key).map(|n| sync_core.selection.with(|s| check_state(n, s))))
                .unwrap_or(CheckState::Unchecked);
            checked.set(state == CheckState::Checked);
            indeterminate.set(state == CheckState::Partial);
        });
    }
    let (change_core, change_row) = (core.clone(), row.clone());
    let on_change = Callback::new(move |change: prime_checkbox::CheckboxChange| change_core.check_node(&change_row, change.checked));
    view! { <Checkbox binary=true checked=checked indeterminate=indeterminate style_class="p-treetable-node-checkbox" on_change=on_change /> }
}

/// Подвал: подписи столбцов.
fn foot_view<T: Clone + Send + Sync + 'static>(core: &Arc<Core<T>>, pt: &Arc<TreeTablePt>) -> impl IntoView + use<T> {
    let columns = core.columns.get();
    columns.iter().any(|c| c.footer.is_some()).then(|| {
        let cells = columns
            .iter()
            .map(|c| {
                let (extra_class, style) = cell_attrs(c);
                view! { <td class={extra_class} style={style}>{c.footer.clone()}</td> }
            })
            .collect_view();
        view! {
            <tfoot role="rowgroup" class={pt.tfoot.merge_class("p-treetable-tfoot")} {..pt.tfoot.attrs()}>
                <tr role="row">{cells}</tr>
            </tfoot>
        }
    })
}
