//! Table - port of `p-table` (DataTable) from PrimeNG 21.1.10, stage 2 (package 1).
//!
//! Data table: columns and rows, sorting by one or several columns, filters (per-column and global),
//! paginator, row selection (click, checkboxes, radio buttons), row expansion, cell and row editing, lazy loading,
//! stripes, grid lines, size, vertical scrolling and a loading indicator.
//! Columns are described by data ([`Column`]), the logic is the pure [`model`]. Column reordering and resizing,
//! freezing, virtual scrolling and grouping are the next packages of stage 2.
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
use prime_dataview::data::{clamp_first, page};
use prime_icons::{
    CheckIcon, ChevronDownIcon, ChevronRightIcon, PencilIcon, SortAltIcon, SortAmountDownIcon, SortAmountUpAltIcon, SpinnerIcon, TimesIcon,
};
use prime_paginator::{Paginator, PaginatorPt, PaginatorState, RowsPerPage};
use prime_radiobutton::{RadioButton, RadioButtonClick};
use prime_rs_core::{OptProp, Prop, Pt, Size, use_prime_config};
use std::cmp::Ordering;
use std::sync::Arc;

/// Column kind.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ColumnKind {
    /// Data: the cell is built by `body`.
    #[default]
    Data,
    /// Row selection checkbox (in the header, "select all").
    SelectionCheckbox,
    /// Row selection radio button.
    SelectionRadio,
    /// Row expansion button.
    Expander,
    /// Row editing buttons ([`EditMode::Row`] mode).
    RowEditor,
}

/// Table column.
#[derive(Clone)]
pub struct Column<T: 'static> {
    /// Field: the column identifier in sorting.
    pub field: String,
    /// Header.
    pub header: Option<String>,
    /// Table footer label.
    pub footer: Option<String>,
    /// Cell builder.
    pub body: Option<Callback<T, AnyView>>,
    /// Can be sorted (needs `compare`).
    pub sortable: bool,
    /// Row comparison for sorting.
    pub compare: Option<Callback<(T, T), Ordering>>,
    /// Inline style of header and body cells.
    pub style: Option<String>,
    /// Extra class of the cells.
    pub style_class: Option<String>,
    /// Column kind.
    pub kind: ColumnKind,
    /// The filter row has an input.
    pub filterable: bool,
    /// Row field text for filters (global and per-column).
    pub filter_text: Option<Callback<T, String>>,
    /// Column filter match mode.
    pub filter_match_mode: MatchMode,
    /// Cell editor: receives the row draft.
    pub editor: Option<Callback<RwSignal<T>, AnyView>>,
}

impl<T: Clone + Send + Sync + 'static> Column<T> {
    /// Empty column for the `field` field.
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

    /// Text column: the cell is the string returned by `cell`.
    pub fn text(field: impl Into<String>, header: impl Into<String>, cell: impl Fn(&T) -> String + Send + Sync + 'static) -> Self {
        let mut column = Self::new(field);
        column.header = Some(header.into());
        column.body = Some(Callback::new(move |row: T| cell(&row).into_any()));
        column
    }

    /// Column with an arbitrary cell.
    pub fn custom(field: impl Into<String>, header: impl Into<String>, body: Callback<T, AnyView>) -> Self {
        let mut column = Self::new(field);
        column.header = Some(header.into());
        column.body = Some(body);
        column
    }

    /// Selection checkbox column.
    pub fn selection_checkbox() -> Self {
        let mut column = Self::new("");
        column.kind = ColumnKind::SelectionCheckbox;
        column.style = Some("width: 3rem".to_owned());
        column
    }

    /// Selection radio button column.
    pub fn selection_radio() -> Self {
        let mut column = Self::new("");
        column.kind = ColumnKind::SelectionRadio;
        column.style = Some("width: 3rem".to_owned());
        column
    }

    /// Row expansion button column.
    pub fn expander() -> Self {
        let mut column = Self::new("");
        column.kind = ColumnKind::Expander;
        column.style = Some("width: 5rem".to_owned());
        column
    }

    /// Row editing button column.
    pub fn row_editor() -> Self {
        let mut column = Self::new("");
        column.kind = ColumnKind::RowEditor;
        column.style = Some("width: 8rem".to_owned());
        column
    }

    /// Enables the column filter; `text` gives the text of the row field.
    pub fn filterable(mut self, text: impl Fn(&T) -> String + Send + Sync + 'static) -> Self {
        self.filterable = true;
        self.filter_text = Some(Callback::new(move |row: T| text(&row)));
        self
    }

    /// Only the field text for the global filter (no input in the filter row).
    pub fn searchable(mut self, text: impl Fn(&T) -> String + Send + Sync + 'static) -> Self {
        self.filter_text = Some(Callback::new(move |row: T| text(&row)));
        self
    }

    /// Column filter match mode.
    pub fn filter_match_mode(mut self, mode: MatchMode) -> Self {
        self.filter_match_mode = mode;
        self
    }

    /// Cell editor: receives the row draft and changes it.
    pub fn editable(mut self, editor: Callback<RwSignal<T>, AnyView>) -> Self {
        self.editor = Some(editor);
        self
    }

    /// Text editor: `get` reads the value from the row, `set` returns the row with the new value.
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

    /// Enables sorting with the given comparison.
    pub fn sortable(mut self, compare: impl Fn(&T, &T) -> Ordering + Send + Sync + 'static) -> Self {
        self.sortable = true;
        self.compare = Some(Callback::new(move |(a, b): (T, T)| compare(&a, &b)));
        self
    }

    /// Footer label.
    pub fn footer(mut self, footer: impl Into<String>) -> Self {
        self.footer = Some(footer.into());
        self
    }

    /// Inline style of the cells.
    pub fn style(mut self, style: impl Into<String>) -> Self {
        self.style = Some(style.into());
        self
    }

    /// Extra class of the cells.
    pub fn style_class(mut self, class: impl Into<String>) -> Self {
        self.style_class = Some(class.into());
        self
    }
}

/// Paginator position.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum PaginatorPosition {
    /// Top.
    Top,
    /// Bottom (default).
    #[default]
    Bottom,
    /// Top and bottom.
    Both,
}

/// Edit mode.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EditMode {
    /// Editing a single cell.
    Cell,
    /// Editing a whole row.
    Row,
}

/// Edit event.
#[derive(Clone, Debug)]
pub struct EditEvent<T> {
    /// Row before editing.
    pub original: T,
    /// Row with changes (for `init` and `cancel` it matches the draft at that moment).
    pub row: T,
    /// Cell field (empty when editing a row).
    pub field: String,
}

/// Load parameters for `lazy`.
#[derive(Clone, Debug, PartialEq)]
pub struct LazyLoadEvent {
    /// Index of the first record of the page.
    pub first: usize,
    /// Number of rows per page.
    pub rows: usize,
    /// Sorting.
    pub sort: Vec<SortMeta>,
    /// Column filters.
    pub filters: Vec<FilterMeta>,
    /// Global filter.
    pub global_filter: String,
}

/// Page change event.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PageEvent {
    /// Index of the first record.
    pub first: usize,
    /// Number of rows per page.
    pub rows: usize,
}

/// Pass-through of the [`Table`] parts.
#[derive(Clone, Debug, Default)]
pub struct TablePt {
    /// Root.
    pub root: Pt,
    /// Table container.
    pub table_container: Pt,
    /// Table.
    pub table: Pt,
    /// Table header (`thead`).
    pub thead: Pt,
    /// Body (`tbody`).
    pub tbody: Pt,
    /// Footer (`tfoot`).
    pub tfoot: Pt,
    /// Paginator parts.
    pub paginator: PaginatorPt,
}

/// State shared by the handlers.
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
    /// Table owner: drafts outlive the buttons whose handlers create them.
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
    /// Rows after filtering and sorting (as is when `lazy`).
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

    /// Number of records for the paginator.
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

    /// Row draft: an existing one or a new one from `row`.
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

    /// Takes the row draft, removing it from the store.
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

    /// Rows of the current page.
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

    /// Keys of the rows the header checkbox acts on.
    fn header_keys(&self) -> Vec<String> {
        let processed = self.processed();
        if self.selection_page_only.get_untracked() { self.keys(&self.page_rows(&processed)) } else { self.keys(&processed) }
    }
}

/// Data table.
///
/// Every property accepts a value or a signal; `selection`, `expanded_keys`, `sort`, `first` and `rows` are two-way signals.
#[component]
pub fn Table<T>(
    /// Rows.
    #[prop(optional, into)]
    value: Prop<Vec<T>>,
    /// Columns.
    #[prop(optional, into)]
    columns: Prop<Vec<Column<T>>>,
    /// Row key (like `dataKey`): needed for selection and expansion.
    row_key: Callback<T, String>,
    /// Row selection mode on click.
    #[prop(optional, into)]
    selection_mode: Prop<Option<SelectionMode>>,
    /// Two-way signal of the selected row keys.
    #[prop(optional)]
    selection: Option<RwSignal<Vec<String>>>,
    /// Selection needs the Ctrl/⌘ key (otherwise it is a toggle).
    #[prop(optional, into)]
    meta_key_selection: Prop<bool>,
    /// The header checkbox acts only on the current page.
    #[prop(optional, into)]
    selection_page_only: Prop<bool>,
    /// Sort mode.
    #[prop(optional, into)]
    sort_mode: Prop<SortMode>,
    /// Two-way signal of the sort.
    #[prop(optional)]
    sort: Option<RwSignal<Vec<SortMeta>>>,
    /// Order on the first sort of a column (1 or -1).
    #[prop(optional, default = Prop::Static(1), into)]
    default_sort_order: Prop<i32>,
    /// Return to the first page on sort.
    #[prop(optional, default = Prop::Static(true), into)]
    reset_page_on_sort: Prop<bool>,
    /// Enable the paginator.
    #[prop(optional, into)]
    paginator: Prop<bool>,
    /// Two-way signal of the index of the first record of the page.
    #[prop(optional)]
    first: Option<RwSignal<usize>>,
    /// Two-way signal of the number of rows per page.
    #[prop(optional)]
    rows: Option<RwSignal<usize>>,
    /// Default number of rows (when `rows` is not passed).
    #[prop(optional, default = 10)]
    default_rows: usize,
    /// Number of page links.
    #[prop(optional, default = Prop::Static(5), into)]
    page_links: Prop<usize>,
    /// Options for the number of rows per page.
    #[prop(optional, into)]
    rows_per_page_options: Prop<Vec<RowsPerPage>>,
    /// Paginator position.
    #[prop(optional, into)]
    paginator_position: Prop<PaginatorPosition>,
    /// Show the paginator when there is a single page.
    #[prop(optional, default = Prop::Static(true), into)]
    always_show_paginator: Prop<bool>,
    /// Current page report template.
    #[prop(optional, default = Prop::Static(Some("{currentPage} of {totalPages}".to_owned())), into)]
    current_page_report_template: OptProp<String>,
    /// Show the current page report.
    #[prop(optional, into)]
    show_current_page_report: Prop<bool>,
    /// Two-way signal of the expanded row keys.
    #[prop(optional)]
    expanded_keys: Option<RwSignal<Vec<String>>>,
    /// Content of an expanded row.
    #[prop(optional, into)]
    row_expansion: Option<Callback<T, AnyView>>,
    /// Only one row can be expanded.
    #[prop(optional, into)]
    row_expand_single: Prop<bool>,
    /// Highlight the row under the cursor.
    #[prop(optional, into)]
    row_hover: Prop<bool>,
    /// Striped rows.
    #[prop(optional, into)]
    striped_rows: Prop<bool>,
    /// Grid lines.
    #[prop(optional, into)]
    show_gridlines: Prop<bool>,
    /// Size.
    #[prop(optional, into)]
    size: OptProp<Size>,
    /// Scrollable body with a pinned header.
    #[prop(optional, into)]
    scrollable: Prop<bool>,
    /// Height of the scroll area (for example `400px`).
    #[prop(optional, into)]
    scroll_height: OptProp<String>,
    /// Show the loading indicator.
    #[prop(optional, into)]
    loading: Prop<bool>,
    /// Text when there are no rows.
    #[prop(optional, into)]
    empty_message: OptProp<String>,
    /// Custom content when there are no rows.
    #[prop(optional, into)]
    empty: Option<ViewFn>,
    /// Heading above the table (`caption`).
    #[prop(optional, into)]
    caption: Option<ViewFn>,
    /// Footer under the table (`summary`).
    #[prop(optional, into)]
    summary: Option<ViewFn>,
    /// Inline style of the table (`table`).
    #[prop(optional, into)]
    table_style: OptProp<String>,
    /// `aria-label` attribute of the table.
    #[prop(optional, into)]
    aria_label: OptProp<String>,
    /// Inline style of the root.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Extra class of the root.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Called after the sort changes.
    #[prop(optional, into)]
    on_sort: Option<Callback<Vec<SortMeta>>>,
    /// Called when the page changes.
    #[prop(optional, into)]
    on_page: Option<Callback<PageEvent>>,
    /// Called when a row is selected.
    #[prop(optional, into)]
    on_row_select: Option<Callback<T>>,
    /// Called when a row is unselected.
    #[prop(optional, into)]
    on_row_unselect: Option<Callback<T>>,
    /// Called when a row is expanded.
    #[prop(optional, into)]
    on_row_expand: Option<Callback<T>>,
    /// Called when a row is collapsed.
    #[prop(optional, into)]
    on_row_collapse: Option<Callback<T>>,
    /// Called when the header checkbox is toggled.
    #[prop(optional, into)]
    on_header_checkbox_toggle: Option<Callback<bool>>,
    /// Two-way signal of the column filters.
    #[prop(optional)]
    filters: Option<RwSignal<Vec<FilterMeta>>>,
    /// Global filter value ("contains" mode over the `global_filter_fields` fields).
    #[prop(optional, into)]
    global_filter: Prop<String>,
    /// Global filter fields; by default all columns with filter text.
    #[prop(optional, into)]
    global_filter_fields: Prop<Vec<String>>,
    /// Lazy loading: sorting, filters and pages are handled by the application via `on_lazy_load`.
    #[prop(optional, into)]
    lazy: Prop<bool>,
    /// Total number of records when `lazy`.
    #[prop(optional, into)]
    total_records: OptProp<usize>,
    /// Edit mode.
    #[prop(optional, into)]
    edit_mode: Prop<Option<EditMode>>,
    /// Two-way signal of the keys of the rows being edited (row mode).
    #[prop(optional)]
    editing_keys: Option<RwSignal<Vec<String>>>,
    /// Called after the column filters change.
    #[prop(optional, into)]
    on_filter: Option<Callback<Vec<FilterMeta>>>,
    /// Called when `lazy` on mount and on page, sort or filter change.
    #[prop(optional, into)]
    on_lazy_load: Option<Callback<LazyLoadEvent>>,
    /// Called when a cell editor opens.
    #[prop(optional, into)]
    on_edit_init: Option<Callback<EditEvent<T>>>,
    /// Called when cell editing completes.
    #[prop(optional, into)]
    on_edit_complete: Option<Callback<EditEvent<T>>>,
    /// Called when cell editing is cancelled.
    #[prop(optional, into)]
    on_edit_cancel: Option<Callback<EditEvent<T>>>,
    /// Called when row editing starts.
    #[prop(optional, into)]
    on_row_edit_init: Option<Callback<EditEvent<T>>>,
    /// Called when a row is saved.
    #[prop(optional, into)]
    on_row_edit_save: Option<Callback<EditEvent<T>>>,
    /// Called when row editing is cancelled.
    #[prop(optional, into)]
    on_row_edit_cancel: Option<Callback<EditEvent<T>>>,
    /// Pass-through of the parts.
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

    // Syncing the radio buttons and header checkbox with the selection, returning `first` to a valid page - browser only.
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
        // A new global filter value returns to the first page.
        let c = core.clone();
        Effect::new(move |previous: Option<()>| {
            c.global_filter.get();
            if previous.is_some() {
                c.first.set(0);
            }
        });
        // Lazy loading: parameters are collected on mount and after every change.
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
            pt.root.merge_class(&cn)
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
            pt.table.merge_class(base)
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

/// Header row.
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

/// Filter row cell: an input and a clear button.
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

/// Body: page rows, expansions and the empty table message.
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

/// Cell with an editor: shows the value or the editor (cell or row editing).
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

/// Row editing buttons: "edit" or "save" and "cancel".
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

/// Footer: column labels.
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
