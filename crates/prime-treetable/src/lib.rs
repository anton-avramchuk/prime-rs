//! TreeTable — port of `p-treetable` from PrimeNG 21.1.10, stage 1.
//!
//! Hierarchical data in a table: nodes with descendants, an expand button indented by level, sorting at every level,
//! a paginator over root nodes, selection (click, Ctrl/⌘, checkboxes propagating to descendants and parents), a loading
//! indicator. Columns are [`prime_table::Column`], the logic is the pure [`tree`]. Filters, editing, resizing and reordering of
//! columns, freezing and lazy loading are later stages.
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
use prime_dataview::data::clamp_first;
use prime_icons::{ChevronDownIcon, ChevronRightIcon, SortAltIcon, SortAmountDownIcon, SortAmountUpAltIcon, SpinnerIcon};
use prime_paginator::{Paginator, PaginatorPt, PaginatorState, RowsPerPage};
use prime_rs_core::{OptProp, Prop, Pt, Size, use_prime_config};
use prime_table::model::{SelectionMode, SortMeta, SortMode, badge_of, order_of, row_click, toggle_expanded, toggle_sort};
use std::cmp::Ordering;
use std::sync::Arc;
use tree::{CheckState, RowNode, check_state, checkbox_toggle, find, flatten, initial_expanded, page_roots, sort_nodes};

pub use prime_table::{Column, PageEvent, PaginatorPosition};
pub use tree::TreeNode;

/// Selection mode.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TreeSelectionMode {
    /// A single node by click.
    Single,
    /// Several nodes by click.
    Multiple,
    /// Checkboxes propagating to descendants and parents.
    Checkbox,
}

/// Pass-through parts of [`TreeTable`].
#[derive(Clone, Debug, Default)]
pub struct TreeTablePt {
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

/// State shared between handlers.
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
    /// Tree after sorting at every level.
    fn sorted(&self) -> Vec<TreeNode<T>> {
        let columns = self.columns.get();
        let meta = self.sort.get();
        let compare = |field: &str, a: &T, b: &T| {
            columns.iter().find(|c| c.field == field).and_then(|c| c.compare).map_or(Ordering::Equal, |cb| cb.run((a.clone(), b.clone())))
        };
        self.value.with(|roots| sort_nodes(roots, &meta, &compare))
    }

    /// Visible rows of the current page.
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

/// Hierarchical data table.
///
/// Every property accepts a value or a signal; `selection`, `expanded_keys`, `sort`, `first` and `rows` are two-way signals.
#[component]
pub fn TreeTable<T>(
    /// Root nodes.
    #[prop(optional, into)]
    value: Prop<Vec<TreeNode<T>>>,
    /// Columns (data columns are used).
    #[prop(optional, into)]
    columns: Prop<Vec<Column<T>>>,
    /// Index of the column that shows the expand button and the checkbox.
    #[prop(optional)]
    toggler_column: usize,
    /// Selection mode.
    #[prop(optional, into)]
    selection_mode: Prop<Option<TreeSelectionMode>>,
    /// Two-way signal of the selected node keys.
    #[prop(optional)]
    selection: Option<RwSignal<Vec<String>>>,
    /// Selection requires the Ctrl/⌘ key.
    #[prop(optional, into)]
    meta_key_selection: Prop<bool>,
    /// Two-way signal of the expanded node keys; without it the initial expansion comes from `TreeNode::expanded`.
    #[prop(optional)]
    expanded_keys: Option<RwSignal<Vec<String>>>,
    /// Sort mode.
    #[prop(optional, into)]
    sort_mode: Prop<SortMode>,
    /// Two-way sort signal.
    #[prop(optional)]
    sort: Option<RwSignal<Vec<SortMeta>>>,
    /// Order on the first sort of a column (1 or -1).
    #[prop(optional, default = Prop::Static(1), into)]
    default_sort_order: Prop<i32>,
    /// Return to the first page on sorting.
    #[prop(optional, default = Prop::Static(true), into)]
    reset_page_on_sort: Prop<bool>,
    /// Enable the paginator (over root nodes).
    #[prop(optional, into)]
    paginator: Prop<bool>,
    /// Two-way signal of the index of the first root of the page.
    #[prop(optional)]
    first: Option<RwSignal<usize>>,
    /// Two-way signal of the number of roots per page.
    #[prop(optional)]
    rows: Option<RwSignal<usize>>,
    /// Default number of roots (when `rows` is not passed).
    #[prop(optional, default = 10)]
    default_rows: usize,
    /// Number of page links.
    #[prop(optional, default = Prop::Static(5), into)]
    page_links: Prop<usize>,
    /// Options for the number of roots per page.
    #[prop(optional, into)]
    rows_per_page_options: Prop<Vec<RowsPerPage>>,
    /// Paginator position.
    #[prop(optional, into)]
    paginator_position: Prop<PaginatorPosition>,
    /// Show the paginator when there is a single page.
    #[prop(optional, default = Prop::Static(true), into)]
    always_show_paginator: Prop<bool>,
    /// Highlight the row under the cursor.
    #[prop(optional, into)]
    row_hover: Prop<bool>,
    /// Grid lines.
    #[prop(optional, into)]
    show_gridlines: Prop<bool>,
    /// Size.
    #[prop(optional, into)]
    size: OptProp<Size>,
    /// Scrollable body.
    #[prop(optional, into)]
    scrollable: Prop<bool>,
    /// Height of the scroll area.
    #[prop(optional, into)]
    scroll_height: OptProp<String>,
    /// Show the loading indicator.
    #[prop(optional, into)]
    loading: Prop<bool>,
    /// Text shown when there are no nodes.
    #[prop(optional, into)]
    empty_message: OptProp<String>,
    /// Caption above the table.
    #[prop(optional, into)]
    caption: Option<ViewFn>,
    /// Footer below the table.
    #[prop(optional, into)]
    summary: Option<ViewFn>,
    /// Inline style of the table.
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
    /// Called when a node is selected.
    #[prop(optional, into)]
    on_node_select: Option<Callback<T>>,
    /// Called when a node is unselected.
    #[prop(optional, into)]
    on_node_unselect: Option<Callback<T>>,
    /// Called when a node is expanded.
    #[prop(optional, into)]
    on_node_expand: Option<Callback<T>>,
    /// Called when a node is collapsed.
    #[prop(optional, into)]
    on_node_collapse: Option<Callback<T>>,
    /// Pass-through parts.
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

    // Resetting `first` to a valid page — browser only.
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
            pt.table.merge_class(base)
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

/// Header row.
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

/// Expand button indented by level.
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

/// Node checkbox with partial selection.
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

/// Footer: column labels.
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
