//! Table — порт `p-table` (DataTable) из PrimeNG 21.1.10, этап 1.
//!
//! Таблица данных: столбцы и строки, сортировка по одному и нескольким столбцам, пагинатор, выбор строк (кликом,
//! флажками, радиокнопками), раскрытие строк, полосатость, сетка, размер, вертикальная прокрутка и индикатор загрузки.
//! Столбцы описываются данными ([`Column`]), логика — чистая [`model`]. Фильтры, правка ячеек, перестановка и
//! изменение размера столбцов, заморозка, виртуальная прокрутка и группировка — следующие этапы.
//!
//! ```
//! #![recursion_limit = "512"]
//! use leptos::prelude::*;
//! use prime_table::{Column, Table};
//!
//! #[derive(Clone)]
//! struct Row { code: &'static str, name: &'static str }
//! let rows = vec![Row { code: "f230", name: "Bamboo Watch" }];
//! let columns = vec![Column::text("code", "Code", |r: &Row| r.code.to_owned()), Column::text("name", "Name", |r: &Row| r.name.to_owned())];
//! let html = view! { <Table value=rows columns=columns row_key=Callback::new(|r: Row| r.code.to_owned()) /> }.to_html();
//! assert!(html.contains("p-datatable") && html.contains("Bamboo Watch") && html.contains(">Code<"));
//! ```

#![recursion_limit = "512"]

pub mod model;

use leptos::prelude::*;
use model::{
    SelectionMode, SortMeta, SortMode, all_selected, badge_of, order_of, row_click, sort_rows, toggle_all, toggle_expanded, toggle_sort,
};
use prime_badge::{Badge, BadgeSize};
use prime_button::{Button, ButtonSeverity, ButtonVariant};
use prime_checkbox::Checkbox;
use prime_core::{OptProp, Prop, Pt, Size, use_prime_config};
use prime_dataview::data::{clamp_first, page};
use prime_icons::{ChevronDownIcon, ChevronRightIcon, SortAltIcon, SortAmountDownIcon, SortAmountUpAltIcon, SpinnerIcon};
use prime_paginator::{Paginator, PaginatorPt, PaginatorState, RowsPerPage};
use prime_radiobutton::{RadioButton, RadioButtonClick};
use std::cmp::Ordering;
use std::sync::Arc;

/// Вид столбца.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ColumnKind {
    /// Данные: ячейка строится `body`.
    #[default]
    Data,
    /// Флажок выбора строки (в заголовке — «выбрать все»).
    SelectionCheckbox,
    /// Радиокнопка выбора строки.
    SelectionRadio,
    /// Кнопка раскрытия строки.
    Expander,
}

/// Столбец таблицы.
#[derive(Clone)]
pub struct Column<T: 'static> {
    /// Поле: идентификатор столбца в сортировке.
    pub field: String,
    /// Заголовок.
    pub header: Option<String>,
    /// Подпись в подвале таблицы.
    pub footer: Option<String>,
    /// Построение ячейки.
    pub body: Option<Callback<T, AnyView>>,
    /// Можно сортировать (нужен `compare`).
    pub sortable: bool,
    /// Сравнение строк для сортировки.
    pub compare: Option<Callback<(T, T), Ordering>>,
    /// Встроенный стиль ячеек заголовка и тела.
    pub style: Option<String>,
    /// Дополнительный класс ячеек.
    pub style_class: Option<String>,
    /// Вид столбца.
    pub kind: ColumnKind,
}

impl<T: Clone + Send + Sync + 'static> Column<T> {
    /// Пустой столбец поля `field`.
    pub fn new(field: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            header: None,
            footer: None,
            body: None,
            sortable: false,
            compare: None,
            style: None,
            style_class: None,
            kind: ColumnKind::Data,
        }
    }

    /// Текстовый столбец: ячейка — строка, возвращаемая `cell`.
    pub fn text(field: impl Into<String>, header: impl Into<String>, cell: impl Fn(&T) -> String + Send + Sync + 'static) -> Self {
        let mut column = Self::new(field);
        column.header = Some(header.into());
        column.body = Some(Callback::new(move |row: T| cell(&row).into_any()));
        column
    }

    /// Столбец с произвольной ячейкой.
    pub fn custom(field: impl Into<String>, header: impl Into<String>, body: Callback<T, AnyView>) -> Self {
        let mut column = Self::new(field);
        column.header = Some(header.into());
        column.body = Some(body);
        column
    }

    /// Столбец флажков выбора.
    pub fn selection_checkbox() -> Self {
        let mut column = Self::new("");
        column.kind = ColumnKind::SelectionCheckbox;
        column.style = Some("width: 3rem".to_owned());
        column
    }

    /// Столбец радиокнопок выбора.
    pub fn selection_radio() -> Self {
        let mut column = Self::new("");
        column.kind = ColumnKind::SelectionRadio;
        column.style = Some("width: 3rem".to_owned());
        column
    }

    /// Столбец кнопок раскрытия строки.
    pub fn expander() -> Self {
        let mut column = Self::new("");
        column.kind = ColumnKind::Expander;
        column.style = Some("width: 5rem".to_owned());
        column
    }

    /// Включает сортировку с заданным сравнением.
    pub fn sortable(mut self, compare: impl Fn(&T, &T) -> Ordering + Send + Sync + 'static) -> Self {
        self.sortable = true;
        self.compare = Some(Callback::new(move |(a, b): (T, T)| compare(&a, &b)));
        self
    }

    /// Подпись в подвале.
    pub fn footer(mut self, footer: impl Into<String>) -> Self {
        self.footer = Some(footer.into());
        self
    }

    /// Встроенный стиль ячеек.
    pub fn style(mut self, style: impl Into<String>) -> Self {
        self.style = Some(style.into());
        self
    }

    /// Дополнительный класс ячеек.
    pub fn style_class(mut self, class: impl Into<String>) -> Self {
        self.style_class = Some(class.into());
        self
    }
}

/// Положение пагинатора.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum PaginatorPosition {
    /// Сверху.
    Top,
    /// Снизу (по умолчанию).
    #[default]
    Bottom,
    /// Сверху и снизу.
    Both,
}

/// Событие смены страницы.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PageEvent {
    /// Индекс первой записи.
    pub first: usize,
    /// Число строк на странице.
    pub rows: usize,
}

/// Pass-through частей [`Table`].
#[derive(Clone, Debug, Default)]
pub struct TablePt {
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
    value: Prop<Vec<T>>,
    columns: Prop<Vec<Column<T>>>,
    row_key: Callback<T, String>,
    selection: RwSignal<Vec<String>>,
    radio_model: RwSignal<Option<String>>,
    expanded: RwSignal<Vec<String>>,
    sort: RwSignal<Vec<SortMeta>>,
    first: RwSignal<usize>,
    rows: RwSignal<usize>,
    selection_mode: Prop<Option<SelectionMode>>,
    meta_key_selection: Prop<bool>,
    selection_page_only: Prop<bool>,
    sort_mode: Prop<SortMode>,
    default_sort_order: Prop<i32>,
    reset_page_on_sort: Prop<bool>,
    paginator: Prop<bool>,
    row_expand_single: Prop<bool>,
    on_sort: Option<Callback<Vec<SortMeta>>>,
    on_page: Option<Callback<PageEvent>>,
    on_row_select: Option<Callback<T>>,
    on_row_unselect: Option<Callback<T>>,
    on_row_expand: Option<Callback<T>>,
    on_row_collapse: Option<Callback<T>>,
    on_header_checkbox_toggle: Option<Callback<bool>>,
}

impl<T: Clone + Send + Sync + 'static> Core<T> {
    /// Строки после сортировки.
    fn processed(&self) -> Vec<T> {
        let columns = self.columns.get();
        let meta = self.sort.get();
        let compare = |field: &str, a: &T, b: &T| {
            columns.iter().find(|c| c.field == field).and_then(|c| c.compare).map_or(Ordering::Equal, |cb| cb.run((a.clone(), b.clone())))
        };
        self.value.with(|rows| sort_rows(rows, &meta, &compare))
    }

    fn keys(&self, rows: &[T]) -> Vec<String> {
        rows.iter().map(|r| self.row_key.run(r.clone())).collect()
    }

    /// Строки текущей страницы.
    fn page_rows(&self, processed: &[T]) -> Vec<T> {
        let rows = self.rows.get();
        let first = clamp_first(self.first.get(), processed.len(), rows);
        page(processed, self.paginator.get(), false, first, rows)
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

    fn row_clicked(&self, row: &T, meta_key: bool) {
        let Some(mode) = self.selection_mode.get_untracked() else { return };
        let key = self.row_key.run(row.clone());
        let result = row_click(mode, &self.selection.get_untracked(), &key, self.meta_key_selection.get_untracked(), meta_key);
        self.selection.set(result.selection);
        let callback = if result.selected { self.on_row_select } else { self.on_row_unselect };
        if let Some(cb) = callback {
            cb.run(row.clone());
        }
    }

    fn toggle_row(&self, row: &T) {
        let key = self.row_key.run(row.clone());
        let (next, open) = toggle_expanded(&self.expanded.get_untracked(), &key, self.row_expand_single.get_untracked());
        self.expanded.set(next);
        let callback = if open { self.on_row_expand } else { self.on_row_collapse };
        if let Some(cb) = callback {
            cb.run(row.clone());
        }
    }

    fn paginate(&self, state: PaginatorState) {
        self.first.set(state.first);
        self.rows.set(state.rows);
        if let Some(cb) = self.on_page {
            cb.run(PageEvent { first: state.first, rows: state.rows });
        }
    }

    /// Ключи строк, на которые действует флажок в заголовке.
    fn header_keys(&self) -> Vec<String> {
        let processed = self.processed();
        if self.selection_page_only.get_untracked() { self.keys(&self.page_rows(&processed)) } else { self.keys(&processed) }
    }
}

/// Таблица данных.
///
/// Каждое свойство принимает значение или сигнал; `selection`, `expanded_keys`, `sort`, `first` и `rows` — двусторонние сигналы.
#[component]
pub fn Table<T>(
    /// Строки.
    #[prop(optional, into)]
    value: Prop<Vec<T>>,
    /// Столбцы.
    #[prop(optional, into)]
    columns: Prop<Vec<Column<T>>>,
    /// Ключ строки (аналог `dataKey`): нужен для выбора и раскрытия.
    row_key: Callback<T, String>,
    /// Режим выбора строк кликом.
    #[prop(optional, into)]
    selection_mode: Prop<Option<SelectionMode>>,
    /// Двусторонний сигнал ключей выбранных строк.
    #[prop(optional)]
    selection: Option<RwSignal<Vec<String>>>,
    /// Для выбора нужна клавиша Ctrl/⌘ (иначе — переключатель).
    #[prop(optional, into)]
    meta_key_selection: Prop<bool>,
    /// Флажок в заголовке действует только на текущую страницу.
    #[prop(optional, into)]
    selection_page_only: Prop<bool>,
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
    /// Включить пагинатор.
    #[prop(optional, into)]
    paginator: Prop<bool>,
    /// Двусторонний сигнал индекса первой записи страницы.
    #[prop(optional)]
    first: Option<RwSignal<usize>>,
    /// Двусторонний сигнал числа строк на странице.
    #[prop(optional)]
    rows: Option<RwSignal<usize>>,
    /// Число строк по умолчанию (когда `rows` не передан).
    #[prop(optional, default = 10)]
    default_rows: usize,
    /// Число ссылок на страницы.
    #[prop(optional, default = Prop::Static(5), into)]
    page_links: Prop<usize>,
    /// Варианты числа строк на странице.
    #[prop(optional, into)]
    rows_per_page_options: Prop<Vec<RowsPerPage>>,
    /// Положение пагинатора.
    #[prop(optional, into)]
    paginator_position: Prop<PaginatorPosition>,
    /// Показывать пагинатор при единственной странице.
    #[prop(optional, default = Prop::Static(true), into)]
    always_show_paginator: Prop<bool>,
    /// Шаблон отчёта о текущей странице.
    #[prop(optional, default = Prop::Static(Some("{currentPage} of {totalPages}".to_owned())), into)]
    current_page_report_template: OptProp<String>,
    /// Показывать отчёт о текущей странице.
    #[prop(optional, into)]
    show_current_page_report: Prop<bool>,
    /// Двусторонний сигнал ключей раскрытых строк.
    #[prop(optional)]
    expanded_keys: Option<RwSignal<Vec<String>>>,
    /// Содержимое раскрытой строки.
    #[prop(optional, into)]
    row_expansion: Option<Callback<T, AnyView>>,
    /// Раскрыта может быть только одна строка.
    #[prop(optional, into)]
    row_expand_single: Prop<bool>,
    /// Подсвечивать строку под курсором.
    #[prop(optional, into)]
    row_hover: Prop<bool>,
    /// Полосатые строки.
    #[prop(optional, into)]
    striped_rows: Prop<bool>,
    /// Линии сетки.
    #[prop(optional, into)]
    show_gridlines: Prop<bool>,
    /// Размер.
    #[prop(optional, into)]
    size: OptProp<Size>,
    /// Прокручиваемое тело с закреплённым заголовком.
    #[prop(optional, into)]
    scrollable: Prop<bool>,
    /// Высота области прокрутки (например `400px`).
    #[prop(optional, into)]
    scroll_height: OptProp<String>,
    /// Показывать индикатор загрузки.
    #[prop(optional, into)]
    loading: Prop<bool>,
    /// Текст при отсутствии строк.
    #[prop(optional, into)]
    empty_message: OptProp<String>,
    /// Своё содержимое при отсутствии строк.
    #[prop(optional, into)]
    empty: Option<ViewFn>,
    /// Заголовок над таблицей (`caption`).
    #[prop(optional, into)]
    caption: Option<ViewFn>,
    /// Подвал под таблицей (`summary`).
    #[prop(optional, into)]
    summary: Option<ViewFn>,
    /// Встроенный стиль таблицы (`table`).
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
    /// Вызывается при выборе строки.
    #[prop(optional, into)]
    on_row_select: Option<Callback<T>>,
    /// Вызывается при снятии выбора строки.
    #[prop(optional, into)]
    on_row_unselect: Option<Callback<T>>,
    /// Вызывается при раскрытии строки.
    #[prop(optional, into)]
    on_row_expand: Option<Callback<T>>,
    /// Вызывается при сворачивании строки.
    #[prop(optional, into)]
    on_row_collapse: Option<Callback<T>>,
    /// Вызывается при переключении флажка в заголовке.
    #[prop(optional, into)]
    on_header_checkbox_toggle: Option<Callback<bool>>,
    /// Pass-through частей.
    #[prop(optional)]
    pt: Option<TablePt>,
) -> impl IntoView
where
    T: Clone + Send + Sync + 'static,
{
    let pt = Arc::new(pt.unwrap_or_default());
    let core = Arc::new(Core {
        value,
        columns,
        row_key,
        selection: selection.unwrap_or_default(),
        radio_model: RwSignal::new(None),
        expanded: expanded_keys.unwrap_or_default(),
        sort: sort.unwrap_or_default(),
        first: first.unwrap_or_default(),
        rows: rows.unwrap_or_else(|| RwSignal::new(default_rows)),
        selection_mode: selection_mode.clone(),
        meta_key_selection,
        selection_page_only,
        sort_mode,
        default_sort_order,
        reset_page_on_sort,
        paginator: paginator.clone(),
        row_expand_single,
        on_sort,
        on_page,
        on_row_select,
        on_row_unselect,
        on_row_expand,
        on_row_collapse,
        on_header_checkbox_toggle,
    });
    core.radio_model.set(core.selection.with_untracked(|s| s.first().cloned()));
    let header_checked = RwSignal::new(false);
    header_checked.set({
        let keys = core.header_keys();
        core.selection.with_untracked(|s| all_selected(&keys, s))
    });

    // Согласование радиокнопок и флажка в заголовке с выбором, возврат `first` на допустимую страницу — только в браузере.
    #[cfg(target_arch = "wasm32")]
    {
        let c = core.clone();
        Effect::new(move |_| {
            let selection = c.selection.get();
            c.radio_model.set(selection.first().cloned());
            let keys = {
                let processed = c.processed();
                if c.selection_page_only.get() { c.keys(&c.page_rows(&processed)) } else { c.keys(&processed) }
            };
            header_checked.set(all_selected(&keys, &selection));
        });
        let c = core.clone();
        Effect::new(move |_| {
            let total = c.processed().len();
            let (first, rows) = (c.first.get_untracked(), c.rows.get());
            let clamped = clamp_first(first, total, rows);
            if clamped != first {
                c.first.set(clamped);
            }
        });
    }

    let root_class = {
        let (pt, selection_mode, size) = (pt.clone(), selection_mode.clone(), size.clone());
        let (row_hover, striped_rows, show_gridlines, scrollable) =
            (row_hover.clone(), striped_rows.clone(), show_gridlines.clone(), scrollable.clone());
        let scroll_height = scroll_height.clone();
        move || {
            let mut cn = String::from("p-datatable p-component");
            if row_hover.get() || selection_mode.get().is_some() {
                cn.push_str(" p-datatable-hoverable");
            }
            if scrollable.get() {
                cn.push_str(" p-datatable-scrollable");
                if scroll_height.get().as_deref() == Some("flex") {
                    cn.push_str(" p-datatable-flex-scrollable");
                }
            }
            if striped_rows.get() {
                cn.push_str(" p-datatable-striped");
            }
            if show_gridlines.get() {
                cn.push_str(" p-datatable-gridlines");
            }
            match size.get() {
                Some(Size::Small) => cn.push_str(" p-datatable-sm"),
                Some(Size::Large) => cn.push_str(" p-datatable-lg"),
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
        mask_loading.get().then(|| {
            view! { <div class="p-datatable-mask p-overlay-mask"><SpinnerIcon class="p-datatable-loading-icon" spin=true /></div> }
        })
    };
    let caption_view = caption.map(|c| view! { <div class="p-datatable-header">{c.run()}</div> });
    let summary_view = summary.map(|s| view! { <div class="p-datatable-footer">{s.run()}</div> });

    let paginator_view = |position: &'static str| {
        let (core, pt) = (core.clone(), pt.clone());
        let (paginator_position, rows_options, page_links, always_show, template, report) = (
            paginator_position.clone(),
            rows_per_page_options.clone(),
            page_links.clone(),
            always_show_paginator.clone(),
            current_page_report_template.clone(),
            show_current_page_report.clone(),
        );
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
                        show_current_page_report=report.clone()
                        current_page_report_template=template.clone()
                        style_class=format!("p-datatable-paginator-{position}")
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
        move || match scroll_height.get().filter(|h| !h.is_empty() && h != "flex") {
            Some(h) => format!("max-height: {h}; overflow: auto"),
            None => "overflow: auto".to_owned(),
        }
    };
    let table_class = {
        let (pt, scrollable) = (pt.clone(), scrollable.clone());
        move || {
            let base = if scrollable.get() { "p-datatable-table p-datatable-scrollable-table" } else { "p-datatable-table" };
            pt.table.merge_class(base).into_owned()
        }
    };

    let head = {
        let (core, pt) = (core.clone(), pt.clone());
        move || head_view(&core, &pt, header_checked)
    };
    let body = {
        let (core, pt) = (core.clone(), pt.clone());
        let (empty_message, empty) = (empty_message.clone(), empty.clone());
        move || body_view(&core, &pt, row_expansion, &empty_message, empty.as_ref(), loading.get())
    };
    let foot = {
        let (core, pt) = (core.clone(), pt.clone());
        move || foot_view(&core, &pt)
    };

    view! {
        <div class={root_class} style={root_style} data-pc-name="datatable" data-pc-section="root" {..pt.root.attrs()}>
            {mask}
            {caption_view}
            {top}
            <div class={pt.table_container.merge_class("p-datatable-table-container")} style={container_style} {..pt.table_container.attrs()}>
                <table role="table" class={table_class} style={move || table_style.get()} aria-label={move || aria_label.get()} {..pt.table.attrs()}>
                    <thead role="rowgroup" class={pt.thead.merge_class("p-datatable-thead")} style="position: sticky" {..pt.thead.attrs()}>
                        {head}
                    </thead>
                    <tbody role="rowgroup" class={pt.tbody.merge_class("p-datatable-tbody")} {..pt.tbody.attrs()}>
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

fn cell_attrs(column: &Column<impl Clone + Send + Sync + 'static>) -> (String, Option<String>) {
    (column.style_class.clone().unwrap_or_default(), column.style.clone())
}

/// Строка заголовков.
fn head_view<T: Clone + Send + Sync + 'static>(
    core: &Arc<Core<T>>,
    _pt: &Arc<TablePt>,
    header_checked: RwSignal<bool>,
) -> impl IntoView + use<T> {
    let columns = core.columns.get();
    let cells = columns
        .iter()
        .map(|column| {
            let (extra_class, style) = cell_attrs(column);
            match column.kind {
                ColumnKind::SelectionCheckbox => {
                    let (toggle_core, label) = (core.clone(), use_prime_config().locale.text("aria.selectAll").to_owned());
                    let on_change = Callback::new(move |change: prime_checkbox::CheckboxChange| {
                        let keys = toggle_core.header_keys();
                        let next = toggle_all(
                            &keys,
                            &toggle_core.selection.get_untracked(),
                            change.checked,
                            toggle_core.selection_page_only.get_untracked(),
                        );
                        toggle_core.selection.set(next);
                        if let Some(cb) = toggle_core.on_header_checkbox_toggle {
                            cb.run(change.checked);
                        }
                    });
                    view! {
                        <th class={extra_class} style={style} role="columnheader">
                            <Checkbox binary=true checked=header_checked aria_label=Some(label) on_change=on_change />
                        </th>
                    }
                    .into_any()
                }
                ColumnKind::SelectionRadio | ColumnKind::Expander => {
                    view! { <th class={extra_class} style={style} role="columnheader"></th> }.into_any()
                }
                ColumnKind::Data => sortable_header(core, column, extra_class, style),
            }
        })
        .collect_view();
    view! { <tr role="row">{cells}</tr> }
}

fn sortable_header<T: Clone + Send + Sync + 'static>(
    core: &Arc<Core<T>>,
    column: &Column<T>,
    extra_class: String,
    style: Option<String>,
) -> AnyView {
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
        let sorted = order_of(&class_core.sort.get(), &class_field) != 0;
        let mut cn = String::from("p-datatable-sortable-column");
        if sorted {
            cn.push_str(" p-datatable-column-sorted");
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
        1 => view! { <SortAmountUpAltIcon class="p-datatable-sort-icon" /> }.into_any(),
        -1 => view! { <SortAmountDownIcon class="p-datatable-sort-icon" /> }.into_any(),
        _ => view! { <SortAltIcon class="p-datatable-sort-icon" /> }.into_any(),
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
            <div class="p-datatable-column-header-content">
                <span class="p-datatable-column-title">{title}</span>
                {icon}
                {badge}
            </div>
        </th>
    }
    .into_any()
}

/// Тело: строки страницы, раскрытия и сообщение о пустой таблице.
fn body_view<T: Clone + Send + Sync + 'static>(
    core: &Arc<Core<T>>,
    _pt: &Arc<TablePt>,
    row_expansion: Option<Callback<T, AnyView>>,
    empty_message: &OptProp<String>,
    empty: Option<&ViewFn>,
    loading: bool,
) -> impl IntoView + use<T> {
    let processed = core.processed();
    let columns = core.columns.get();
    let column_count = columns.len().max(1);
    let rows = core.page_rows(&processed);
    if processed.is_empty() {
        let text =
            empty_message.get().filter(|m| !m.is_empty()).unwrap_or_else(|| use_prime_config().locale.text("emptyMessage").to_owned());
        let custom = empty.map(|e| e.run().into_any());
        return (!loading)
            .then(|| view! { <tr class="p-datatable-empty-message" role="row"><td colspan=column_count.to_string()>{custom.unwrap_or_else(|| text.into_any())}</td></tr> })
            .into_any();
    }
    rows.into_iter().enumerate().map(|(index, row)| row_view(core, &columns, row, index, row_expansion)).collect_view().into_any()
}

fn row_view<T: Clone + Send + Sync + 'static>(
    core: &Arc<Core<T>>,
    columns: &[Column<T>],
    row: T,
    index: usize,
    row_expansion: Option<Callback<T, AnyView>>,
) -> impl IntoView + use<T> {
    let key = core.row_key.run(row.clone());
    let (class_core, aria_core, click_core) = (core.clone(), core.clone(), core.clone());
    let (class_key, aria_key) = (key.clone(), key.clone());
    let click_row = row.clone();
    let class = move || {
        let mut cn = String::new();
        if class_core.selection_mode.get().is_some() {
            cn.push_str("p-datatable-selectable-row");
            if class_core.selection.with(|s| s.contains(&class_key)) {
                cn.push_str(" p-datatable-row-selected");
            }
        }
        cn
    };
    let aria_selected =
        move || aria_core.selection_mode.get().is_some().then(|| aria_core.selection.with(|s| s.contains(&aria_key)).to_string());
    let cells = columns.iter().map(|column| body_cell(core, column, &row, &key)).collect_view();
    let expansion = row_expansion.map(|render| {
        let (expand_core, expand_key, expand_row) = (core.clone(), key.clone(), row.clone());
        let colspan = columns.len().max(1).to_string();
        move || {
            expand_core.expanded.with(|e| e.contains(&expand_key)).then(|| {
                view! { <tr class="p-datatable-row-expansion" role="row"><td colspan=colspan.clone()>{render.run(expand_row.clone())}</td></tr> }
            })
        }
    });
    view! {
        <tr
            class={class}
            role="row"
            data-p-index={index.to_string()}
            aria-selected={aria_selected}
            on:click=move |e: web_sys::MouseEvent| {
                use leptos::wasm_bindgen::JsCast;
                let interactive = e
                    .target()
                    .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
                    .is_some_and(|el| el.closest("input, button, a, .p-checkbox, .p-radiobutton").ok().flatten().is_some());
                if !interactive {
                    click_core.row_clicked(&click_row, e.meta_key() || e.ctrl_key());
                }
            }
        >
            {cells}
        </tr>
        {expansion}
    }
}

fn body_cell<T: Clone + Send + Sync + 'static>(core: &Arc<Core<T>>, column: &Column<T>, row: &T, key: &str) -> impl IntoView + use<T> {
    let (extra_class, style) = cell_attrs(column);
    let content = match column.kind {
        ColumnKind::Data => column.body.map(|body| body.run(row.clone())),
        ColumnKind::SelectionCheckbox => {
            let (select_core, select_row) = (core.clone(), row.clone());
            let on_change = Callback::new(move |change: prime_checkbox::CheckboxChange| {
                let callback = if change.checked { select_core.on_row_select } else { select_core.on_row_unselect };
                if let Some(cb) = callback {
                    cb.run(select_row.clone());
                }
            });
            Some(view! { <Checkbox model=core.selection value=key.to_owned() on_change=on_change /> }.into_any())
        }
        ColumnKind::SelectionRadio => {
            let (select_core, select_row, select_key) = (core.clone(), row.clone(), key.to_owned());
            let on_click = Callback::new(move |_: RadioButtonClick| {
                select_core.selection.set(vec![select_key.clone()]);
                if let Some(cb) = select_core.on_row_select {
                    cb.run(select_row.clone());
                }
            });
            Some(view! { <RadioButton model=core.radio_model value=key.to_owned() on_click=on_click /> }.into_any())
        }
        ColumnKind::Expander => {
            let (toggle_core, toggle_row, state_core, state_key) = (core.clone(), row.clone(), core.clone(), key.to_owned());
            let icon = ViewFn::from(move || {
                if state_core.expanded.with(|e| e.contains(&state_key)) {
                    view! { <ChevronDownIcon class="p-datatable-row-toggle-icon" /> }.into_any()
                } else {
                    view! { <ChevronRightIcon class="p-datatable-row-toggle-icon" /> }.into_any()
                }
            });
            Some(
                view! {
                    <Button
                        text=true
                        rounded=true
                        plain=true
                        variant=ButtonVariant::Text
                        severity=ButtonSeverity::Secondary
                        button_type="button"
                        icon=icon
                        style_class="p-datatable-row-toggle-button"
                        on_click=Callback::new(move |_| toggle_core.toggle_row(&toggle_row))
                    />
                }
                .into_any(),
            )
        }
    };
    view! { <td class={extra_class} style={style} role="cell">{content}</td> }
}

/// Подвал: подписи столбцов.
fn foot_view<T: Clone + Send + Sync + 'static>(core: &Arc<Core<T>>, pt: &Arc<TablePt>) -> impl IntoView + use<T> {
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
            <tfoot role="rowgroup" class={pt.tfoot.merge_class("p-datatable-tfoot")} style="position: sticky" {..pt.tfoot.attrs()}>
                <tr role="row">{cells}</tr>
            </tfoot>
        }
    })
}
