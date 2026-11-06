//! Table — порт `p-table` (DataTable) из PrimeNG 21.1.10, этап 2 (пакет 1).
//!
//! Таблица данных: столбцы и строки, сортировка по одному и нескольким столбцам, фильтры (по столбцам и глобальный),
//! пагинатор, выбор строк (кликом, флажками, радиокнопками), раскрытие строк, правка ячеек и строк, ленивая загрузка,
//! полосатость, сетка, размер, вертикальная прокрутка и индикатор загрузки.
//! Столбцы описываются данными ([`Column`]), логика — чистая [`model`]. Перестановка и изменение размера столбцов,
//! заморозка, виртуальная прокрутка и группировка — следующие пакеты этапа 2.
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
    FilterMeta, MatchMode, SelectionMode, SortMeta, SortMode, all_selected, badge_of, filter_rows, filter_value, order_of, row_click,
    set_filter, sort_rows, toggle_all, toggle_expanded, toggle_sort,
};
use prime_badge::{Badge, BadgeSize};
use prime_button::{Button, ButtonSeverity, ButtonVariant};
use prime_checkbox::Checkbox;
use prime_core::{OptProp, Prop, Pt, Size, use_prime_config};
use prime_dataview::data::{clamp_first, page};
use prime_icons::{
    CheckIcon, ChevronDownIcon, ChevronRightIcon, PencilIcon, SortAltIcon, SortAmountDownIcon, SortAmountUpAltIcon, SpinnerIcon, TimesIcon,
};
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
    /// Кнопки правки строки (режим [`EditMode::Row`]).
    RowEditor,
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
    /// В строке фильтров есть поле ввода.
    pub filterable: bool,
    /// Текст поля строки для фильтров (общий и столбцовый).
    pub filter_text: Option<Callback<T, String>>,
    /// Режим сравнения фильтра столбца.
    pub filter_match_mode: MatchMode,
    /// Редактор ячейки: получает черновик строки.
    pub editor: Option<Callback<RwSignal<T>, AnyView>>,
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
            filterable: false,
            filter_text: None,
            filter_match_mode: MatchMode::StartsWith,
            editor: None,
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

    /// Столбец кнопок правки строки.
    pub fn row_editor() -> Self {
        let mut column = Self::new("");
        column.kind = ColumnKind::RowEditor;
        column.style = Some("width: 8rem".to_owned());
        column
    }

    /// Включает фильтр столбца; `text` даёт текст поля строки.
    pub fn filterable(mut self, text: impl Fn(&T) -> String + Send + Sync + 'static) -> Self {
        self.filterable = true;
        self.filter_text = Some(Callback::new(move |row: T| text(&row)));
        self
    }

    /// Только текст поля для общего фильтра (без поля ввода в строке фильтров).
    pub fn searchable(mut self, text: impl Fn(&T) -> String + Send + Sync + 'static) -> Self {
        self.filter_text = Some(Callback::new(move |row: T| text(&row)));
        self
    }

    /// Режим сравнения фильтра столбца.
    pub fn filter_match_mode(mut self, mode: MatchMode) -> Self {
        self.filter_match_mode = mode;
        self
    }

    /// Редактор ячейки: получает черновик строки и меняет его.
    pub fn editable(mut self, editor: Callback<RwSignal<T>, AnyView>) -> Self {
        self.editor = Some(editor);
        self
    }

    /// Текстовый редактор: `get` читает значение из строки, `set` возвращает строку с новым значением.
    pub fn edit_text(
        self,
        get: impl Fn(&T) -> String + Send + Sync + 'static,
        set: impl Fn(&T, String) -> T + Send + Sync + 'static,
    ) -> Self {
        let (get, set) = (Arc::new(get), Arc::new(set));
        self.editable(Callback::new(move |draft: RwSignal<T>| {
            let (get, set) = (get.clone(), set.clone());
            let input = NodeRef::<leptos::html::Input>::new();
            #[cfg(target_arch = "wasm32")]
            Effect::new(move |_| {
                if let Some(el) = input.get() {
                    let _ = el.focus();
                }
            });
            view! {
                <input
                    node_ref=input
                    type="text"
                    class="p-inputtext p-component"
                    value={let get = get.clone(); move || draft.with_untracked(|r| get(r))}
                    prop:value=move || draft.with(|r| get(r))
                    on:input=move |e| {
                        let value = event_target_value(&e);
                        draft.update(|r| *r = set(r, value));
                    }
                />
            }
            .into_any()
        }))
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

/// Режим правки.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EditMode {
    /// Правка отдельной ячейки.
    Cell,
    /// Правка строки целиком.
    Row,
}

/// Событие правки.
#[derive(Clone, Debug)]
pub struct EditEvent<T> {
    /// Строка до правки.
    pub original: T,
    /// Строка с изменениями (для `init` и `cancel` совпадает с черновиком на этот момент).
    pub row: T,
    /// Поле ячейки (пусто при правке строки).
    pub field: String,
}

/// Параметры загрузки при `lazy`.
#[derive(Clone, Debug, PartialEq)]
pub struct LazyLoadEvent {
    /// Индекс первой записи страницы.
    pub first: usize,
    /// Число строк на странице.
    pub rows: usize,
    /// Сортировка.
    pub sort: Vec<SortMeta>,
    /// Фильтры столбцов.
    pub filters: Vec<FilterMeta>,
    /// Глобальный фильтр.
    pub global_filter: String,
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
    filters: RwSignal<Vec<FilterMeta>>,
    global_filter: Prop<String>,
    global_filter_fields: Prop<Vec<String>>,
    lazy: Prop<bool>,
    total_records: OptProp<usize>,
    edit_mode: Prop<Option<EditMode>>,
    editing_keys: RwSignal<Vec<String>>,
    cell_edit: RwSignal<Option<(String, String)>>,
    drafts: StoredValue<Vec<(String, RwSignal<T>)>>,
    /// Владелец таблицы: черновики живут дольше кнопок, из обработчиков которых создаются.
    owner: Option<Owner>,
    on_filter: Option<Callback<Vec<FilterMeta>>>,
    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    on_lazy_load: Option<Callback<LazyLoadEvent>>,
    on_edit_init: Option<Callback<EditEvent<T>>>,
    on_edit_complete: Option<Callback<EditEvent<T>>>,
    on_edit_cancel: Option<Callback<EditEvent<T>>>,
    on_row_edit_init: Option<Callback<EditEvent<T>>>,
    on_row_edit_save: Option<Callback<EditEvent<T>>>,
    on_row_edit_cancel: Option<Callback<EditEvent<T>>>,
}

impl<T: Clone + Send + Sync + 'static> Core<T> {
    /// Строки после фильтрации и сортировки (при `lazy` — как есть).
    fn processed(&self) -> Vec<T> {
        if self.lazy.get() {
            return self.value.get();
        }
        let columns = self.columns.get();
        let meta = self.sort.get();
        let filters = self.filters.get();
        let global = self.global_filter.get();
        let mut fields = self.global_filter_fields.get();
        if fields.is_empty() {
            fields = columns.iter().filter(|c| c.filter_text.is_some()).map(|c| c.field.clone()).collect();
        }
        let text_of = |field: &str, row: &T| {
            columns.iter().find(|c| c.field == field).and_then(|c| c.filter_text).map_or_else(String::new, |cb| cb.run(row.clone()))
        };
        let filtered = self.value.with(|rows| filter_rows(rows, &filters, &global, &fields, &text_of));
        let compare = |field: &str, a: &T, b: &T| {
            columns.iter().find(|c| c.field == field).and_then(|c| c.compare).map_or(Ordering::Equal, |cb| cb.run((a.clone(), b.clone())))
        };
        sort_rows(&filtered, &meta, &compare)
    }

    /// Число записей для пагинатора.
    fn total(&self) -> usize {
        if self.lazy.get() { self.total_records.get().unwrap_or_else(|| self.value.with(Vec::len)) } else { self.processed().len() }
    }

    fn set_column_filter(&self, field: &str, value: &str, mode: MatchMode) {
        let next = set_filter(&self.filters.get_untracked(), field, value, mode);
        self.first.set(0);
        self.filters.set(next.clone());
        if let Some(cb) = self.on_filter {
            cb.run(next);
        }
    }

    /// Черновик строки: существующий или новый из `row`.
    fn draft(&self, key: &str, row: &T) -> RwSignal<T> {
        if let Some(d) = self.drafts.with_value(|d| d.iter().find(|(k, _)| k == key).map(|(_, s)| *s)) {
            return d;
        }
        let value = row.clone();
        let draft = match &self.owner {
            Some(owner) => owner.with(|| RwSignal::new(value)),
            None => RwSignal::new(value),
        };
        self.drafts.update_value(|d| d.push((key.to_owned(), draft)));
        draft
    }

    /// Забирает черновик строки, удаляя его из хранилища.
    fn take_draft(&self, key: &str, row: &T) -> T {
        let draft = self.drafts.with_value(|d| d.iter().find(|(k, _)| k == key).map(|(_, s)| *s));
        self.drafts.update_value(|d| d.retain(|(k, _)| k != key));
        draft.map_or_else(
            || row.clone(),
            |d| {
                let value = d.get_untracked();
                d.dispose();
                value
            },
        )
    }

    fn event(&self, original: &T, row: T, field: &str) -> EditEvent<T> {
        EditEvent { original: original.clone(), row, field: field.to_owned() }
    }

    fn start_cell_edit(&self, row: &T, field: &str) {
        let key = self.row_key.run(row.clone());
        let current = (key.clone(), field.to_owned());
        if self.cell_edit.with_untracked(|c| c.as_ref() == Some(&current)) {
            return;
        }
        self.drafts.update_value(|d| d.retain(|(k, _)| *k != key));
        self.draft(&key, row);
        self.cell_edit.set(Some(current));
        if let Some(cb) = self.on_edit_init {
            cb.run(self.event(row, row.clone(), field));
        }
    }

    fn end_cell_edit(&self, row: &T, field: &str, save: bool) {
        let key = self.row_key.run(row.clone());
        if !self.cell_edit.with_untracked(|c| c.as_ref() == Some(&(key.clone(), field.to_owned()))) {
            return;
        }
        self.cell_edit.set(None);
        let draft = self.take_draft(&key, row);
        let callback = if save { self.on_edit_complete } else { self.on_edit_cancel };
        if let Some(cb) = callback {
            cb.run(self.event(row, if save { draft } else { row.clone() }, field));
        }
    }

    fn start_row_edit(&self, row: &T) {
        let key = self.row_key.run(row.clone());
        self.editing_keys.update(|k| {
            if !k.contains(&key) {
                k.push(key.clone());
            }
        });
        self.draft(&key, row);
        if let Some(cb) = self.on_row_edit_init {
            cb.run(self.event(row, row.clone(), ""));
        }
    }

    fn end_row_edit(&self, row: &T, save: bool) {
        let key = self.row_key.run(row.clone());
        self.editing_keys.update(|k| k.retain(|x| *x != key));
        let draft = self.take_draft(&key, row);
        let callback = if save { self.on_row_edit_save } else { self.on_row_edit_cancel };
        if let Some(cb) = callback {
            cb.run(self.event(row, if save { draft } else { row.clone() }, ""));
        }
    }

    fn keys(&self, rows: &[T]) -> Vec<String> {
        rows.iter().map(|r| self.row_key.run(r.clone())).collect()
    }

    /// Строки текущей страницы.
    fn page_rows(&self, processed: &[T]) -> Vec<T> {
        let rows = self.rows.get();
        if self.lazy.get() {
            return processed.to_vec();
        }
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
    /// Двусторонний сигнал фильтров столбцов.
    #[prop(optional)]
    filters: Option<RwSignal<Vec<FilterMeta>>>,
    /// Значение глобального фильтра (режим «содержит» по полям `global_filter_fields`).
    #[prop(optional, into)]
    global_filter: Prop<String>,
    /// Поля глобального фильтра; по умолчанию — все столбцы с текстом для фильтра.
    #[prop(optional, into)]
    global_filter_fields: Prop<Vec<String>>,
    /// Ленивая загрузка: сортировку, фильтры и страницы выполняет приложение по `on_lazy_load`.
    #[prop(optional, into)]
    lazy: Prop<bool>,
    /// Общее число записей при `lazy`.
    #[prop(optional, into)]
    total_records: OptProp<usize>,
    /// Режим правки.
    #[prop(optional, into)]
    edit_mode: Prop<Option<EditMode>>,
    /// Двусторонний сигнал ключей строк в правке (режим строки).
    #[prop(optional)]
    editing_keys: Option<RwSignal<Vec<String>>>,
    /// Вызывается после изменения фильтров столбцов.
    #[prop(optional, into)]
    on_filter: Option<Callback<Vec<FilterMeta>>>,
    /// Вызывается при `lazy` при монтировании и смене страницы, сортировки или фильтров.
    #[prop(optional, into)]
    on_lazy_load: Option<Callback<LazyLoadEvent>>,
    /// Вызывается при открытии редактора ячейки.
    #[prop(optional, into)]
    on_edit_init: Option<Callback<EditEvent<T>>>,
    /// Вызывается при завершении правки ячейки.
    #[prop(optional, into)]
    on_edit_complete: Option<Callback<EditEvent<T>>>,
    /// Вызывается при отмене правки ячейки.
    #[prop(optional, into)]
    on_edit_cancel: Option<Callback<EditEvent<T>>>,
    /// Вызывается при начале правки строки.
    #[prop(optional, into)]
    on_row_edit_init: Option<Callback<EditEvent<T>>>,
    /// Вызывается при сохранении строки.
    #[prop(optional, into)]
    on_row_edit_save: Option<Callback<EditEvent<T>>>,
    /// Вызывается при отмене правки строки.
    #[prop(optional, into)]
    on_row_edit_cancel: Option<Callback<EditEvent<T>>>,
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
        filters: filters.unwrap_or_default(),
        global_filter,
        global_filter_fields,
        lazy,
        total_records,
        edit_mode,
        editing_keys: editing_keys.unwrap_or_default(),
        cell_edit: RwSignal::new(None),
        drafts: StoredValue::new(Vec::new()),
        owner: Owner::current(),
        on_filter,
        on_lazy_load,
        on_edit_init,
        on_edit_complete,
        on_edit_cancel,
        on_row_edit_init,
        on_row_edit_save,
        on_row_edit_cancel,
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
            let total = c.total();
            let (first, rows) = (c.first.get_untracked(), c.rows.get());
            let clamped = clamp_first(first, total, rows);
            if clamped != first && !c.lazy.get_untracked() {
                c.first.set(clamped);
            }
        });
        // Новое значение глобального фильтра возвращает на первую страницу.
        let c = core.clone();
        Effect::new(move |previous: Option<()>| {
            c.global_filter.get();
            if previous.is_some() {
                c.first.set(0);
            }
        });
        // Ленивая загрузка: параметры собираются при монтировании и после каждого изменения.
        let c = core.clone();
        Effect::new(move |_| {
            let event = LazyLoadEvent {
                first: c.first.get(),
                rows: c.rows.get(),
                sort: c.sort.get(),
                filters: c.filters.get(),
                global_filter: c.global_filter.get(),
            };
            if c.lazy.get_untracked()
                && let Some(cb) = c.on_lazy_load
            {
                cb.run(event);
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
                    Signal::derive(move || core.total())
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
                ColumnKind::SelectionRadio | ColumnKind::Expander | ColumnKind::RowEditor => {
                    view! { <th class={extra_class} style={style} role="columnheader"></th> }.into_any()
                }
                ColumnKind::Data => sortable_header(core, column, extra_class, style),
            }
        })
        .collect_view();
    let filter_row = columns.iter().any(|c| c.filterable).then(|| {
        let cells = columns.iter().map(|column| filter_cell(core, column)).collect_view();
        view! { <tr role="row">{cells}</tr> }
    });
    view! {
        <tr role="row">{cells}</tr>
        {filter_row}
    }
}

/// Ячейка строки фильтров: поле ввода и кнопка очистки.
fn filter_cell<T: Clone + Send + Sync + 'static>(core: &Arc<Core<T>>, column: &Column<T>) -> AnyView {
    let (extra_class, style) = cell_attrs(column);
    if !column.filterable {
        return view! { <th class={extra_class} style={style} role="columnheader"></th> }.into_any();
    }
    let (field, mode) = (column.field.clone(), column.filter_match_mode);
    let (value_core, value_field) = (core.clone(), field.clone());
    let (input_core, input_field) = (core.clone(), field.clone());
    let (show_core, show_field) = (core.clone(), field.clone());
    let (clear_core, clear_field) = (core.clone(), field);
    let value = move || value_core.filters.with(|f| filter_value(f, &value_field).to_owned());
    let has_value = move || show_core.filters.with(|f| !filter_value(f, &show_field).is_empty());
    let label = column.header.clone().unwrap_or_default();
    view! {
        <th class={extra_class} style={style} role="columnheader">
            <div class="p-datatable-filter">
                <input
                    type="text"
                    class="p-inputtext p-component"
                    aria-label={format!("{label} filter")}
                    value={value.clone()}
                    prop:value={value}
                    on:input=move |e| input_core.set_column_filter(&input_field, &event_target_value(&e), mode)
                />
                {move || {
                    let (core, field) = (clear_core.clone(), clear_field.clone());
                    has_value()
                        .then(|| {
                            view! {
                                <button
                                    type="button"
                                    class="p-datatable-filter-clear-button p-button p-component p-button-icon-only p-button-text p-button-rounded p-button-secondary"
                                    aria-label="Clear"
                                    on:click=move |_| core.set_column_filter(&field, "", mode)
                                >
                                    <TimesIcon class="p-button-icon" />
                                </button>
                            }
                        })
                }}
            </div>
        </th>
    }
    .into_any()
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
    if column.kind == ColumnKind::Data
        && let Some(editor) = column.editor
    {
        return editable_cell(core, column, editor, row, key, extra_class, style).into_any();
    }
    let content = match column.kind {
        ColumnKind::Data => column.body.map(|body| body.run(row.clone())),
        ColumnKind::RowEditor => Some(row_editor_buttons(core, row, key)),
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
    view! { <td class={extra_class} style={style} role="cell">{content}</td> }.into_any()
}

/// Ячейка с редактором: показывает значение или редактор (правка ячейки или строки).
fn editable_cell<T: Clone + Send + Sync + 'static>(
    core: &Arc<Core<T>>,
    column: &Column<T>,
    editor: Callback<RwSignal<T>, AnyView>,
    row: &T,
    key: &str,
    extra_class: String,
    style: Option<String>,
) -> AnyView {
    let (field, body) = (column.field.clone(), column.body);
    let editing = {
        let (core, key, field) = (core.clone(), key.to_owned(), field.clone());
        move || match core.edit_mode.get() {
            Some(EditMode::Cell) => core.cell_edit.with(|e| e.as_ref().is_some_and(|(k, f)| *k == key && *f == field)),
            Some(EditMode::Row) => core.editing_keys.with(|e| e.contains(&key)),
            None => false,
        }
    };
    let class = {
        let (core, editing) = (core.clone(), editing.clone());
        move || {
            let mut cn = extra_class.clone();
            if core.edit_mode.get().is_some() {
                cn.push_str(" p-editable-column");
            }
            if editing() {
                cn.push_str(" p-cell-editing");
            }
            cn
        }
    };
    let content = {
        let (core, key, row, editing) = (core.clone(), key.to_owned(), row.clone(), editing.clone());
        move || {
            if editing() { editor.run(core.draft(&key, &row)) } else { body.map_or_else(|| ().into_any(), |b| b.run(row.clone())) }
        }
    };
    let (click_core, click_row, click_field) = (core.clone(), row.clone(), field.clone());
    let (key_core, key_row, key_field) = (core.clone(), row.clone(), field.clone());
    let (out_core, out_row, out_field) = (core.clone(), row.clone(), field);
    let (click_editing, key_editing, out_editing) = (editing.clone(), editing.clone(), editing);
    view! {
        <td
            class={class}
            style={style}
            role="cell"
            on:click=move |_| {
                if click_core.edit_mode.get_untracked() == Some(EditMode::Cell) && !click_editing() {
                    click_core.start_cell_edit(&click_row, &click_field);
                }
            }
            on:keydown=move |e: web_sys::KeyboardEvent| {
                if key_core.edit_mode.get_untracked() == Some(EditMode::Cell) && key_editing() {
                    match e.key().as_str() {
                        "Enter" => key_core.end_cell_edit(&key_row, &key_field, true),
                        "Escape" => key_core.end_cell_edit(&key_row, &key_field, false),
                        _ => {}
                    }
                }
            }
            on:focusout=move |_: web_sys::FocusEvent| {
                if out_core.edit_mode.get_untracked() == Some(EditMode::Cell) && out_editing() {
                    out_core.end_cell_edit(&out_row, &out_field, true);
                }
            }
        >
            {content}
        </td>
    }
    .into_any()
}

/// Кнопки правки строки: «изменить» либо «сохранить» и «отменить».
fn row_editor_buttons<T: Clone + Send + Sync + 'static>(core: &Arc<Core<T>>, row: &T, key: &str) -> AnyView {
    let (core, row, key) = (core.clone(), row.clone(), key.to_owned());
    let button = |class: &'static str, icon: ViewFn, label: &'static str, on_click: Callback<()>| {
        view! {
            <Button
                text=true
                rounded=true
                plain=true
                variant=ButtonVariant::Text
                severity=ButtonSeverity::Secondary
                button_type="button"
                icon=icon
                aria_label=label
                style_class=class
                on_click=Callback::new(move |_| on_click.run(()))
            />
        }
    };
    (move || {
        if core.editing_keys.with(|e| e.contains(&key)) {
            let (save_core, save_row, cancel_core, cancel_row) = (core.clone(), row.clone(), core.clone(), row.clone());
            let save = button(
                "p-datatable-row-editor-save",
                ViewFn::from(|| view! { <CheckIcon /> }),
                "Save",
                Callback::new(move |()| save_core.end_row_edit(&save_row, true)),
            );
            let cancel = button(
                "p-datatable-row-editor-cancel",
                ViewFn::from(|| view! { <TimesIcon /> }),
                "Cancel",
                Callback::new(move |()| cancel_core.end_row_edit(&cancel_row, false)),
            );
            view! { <div style="display: flex; gap: 0.25rem">{save}{cancel}</div> }.into_any()
        } else {
            let (init_core, init_row) = (core.clone(), row.clone());
            button(
                "p-datatable-row-editor-init",
                ViewFn::from(|| view! { <PencilIcon /> }),
                "Edit",
                Callback::new(move |()| init_core.start_row_edit(&init_row)),
            )
            .into_any()
        }
    })
    .into_any()
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
