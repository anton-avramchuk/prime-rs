//! DataView - port of `p-dataview` from PrimeNG 21.1.10.
//!
//! Data as a list or a grid with a paginator, sorting and a filter. The application defines how items look
//! (`list` and `grid` receive the items of the current page), the filter is a predicate, sorting is a comparison and an order.
//! The selection and page logic is the pure [`data`].
//!
//! ```
//! #![recursion_limit = "512"]
//! use leptos::prelude::*;
//! use prime_dataview::DataView;
//!
//! let items = vec!["Apple".to_owned(), "Pear".to_owned()];
//! let list = Callback::new(|page: Vec<String>| view! { <ul>{page.into_iter().map(|i| view! { <li>{i}</li> }).collect_view()}</ul> }.into_any());
//! let html = view! { <DataView value=items list=list /> }.to_html();
//! assert!(html.contains("p-dataview-list") && html.contains("<li>Apple</li>"));
//! ```

#![recursion_limit = "512"]

pub mod data;

use data::{clamp_first, page, prepare, total};
use leptos::prelude::*;
use prime_core::{OptProp, Prop, Pt, use_prime_config};
use prime_icons::SpinnerIcon;
use prime_paginator::{Paginator, PaginatorPt, PaginatorState, RowsPerPage};
use std::cmp::Ordering;
use std::sync::Arc;

#[cfg(target_arch = "wasm32")]
use std::sync::atomic::{AtomicBool, Ordering as AtomicOrdering};

/// Data layout.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum DataViewLayout {
    /// List (default).
    #[default]
    List,
    /// Grid.
    Grid,
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

/// Load event in `lazy` mode: page, rows, sort order.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LazyLoadEvent {
    /// Index of the first record.
    pub first: usize,
    /// Number of rows per page.
    pub rows: usize,
    /// Sort order: 1, -1 or 0.
    pub sort_order: i32,
}

/// Page change event.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PageEvent {
    /// Index of the first record.
    pub first: usize,
    /// Number of rows per page.
    pub rows: usize,
}

/// Pass-through of the [`DataView`] parts.
#[derive(Clone, Debug, Default)]
pub struct DataViewPt {
    /// Root.
    pub root: Pt,
    /// Header.
    pub header: Pt,
    /// Content.
    pub content: Pt,
    /// Empty data message.
    pub empty_message: Pt,
    /// Footer.
    pub footer: Pt,
    /// Loading indicator.
    pub loading: Pt,
    /// Paginator parts.
    pub paginator: PaginatorPt,
}

/// State shared by the handlers.
struct Core<T: Send + Sync + 'static> {
    value: Prop<Vec<T>>,
    first: RwSignal<usize>,
    rows: RwSignal<usize>,
    total_records: Prop<usize>,
    lazy: Prop<bool>,
    paginator: Prop<bool>,
    filter: Option<Callback<T, bool>>,
    compare: Option<Callback<(T, T), Ordering>>,
    sort_order: Prop<i32>,
    on_page: Option<Callback<PageEvent>>,
    on_lazy_load: Option<Callback<LazyLoadEvent>>,
}

impl<T: Clone + Send + Sync + 'static> Core<T> {
    fn prepared(&self) -> Vec<T> {
        let filter = self.filter.map(|cb| move |item: &T| cb.run(item.clone()));
        let compare = self.compare.map(|cb| move |a: &T, b: &T| cb.run((a.clone(), b.clone())));
        self.value.with(|items| {
            prepare(
                items,
                filter.as_ref().map(|f| f as &dyn Fn(&T) -> bool),
                compare.as_ref().map(|c| c as &dyn Fn(&T, &T) -> Ordering),
                self.sort_order.get(),
            )
        })
    }

    fn total(&self, prepared_len: usize) -> usize {
        total(prepared_len, self.lazy.get(), self.total_records.get())
    }

    fn paginate(&self, state: PaginatorState) {
        self.first.set(state.first);
        self.rows.set(state.rows);
        if self.lazy.get_untracked()
            && let Some(cb) = self.on_lazy_load
        {
            cb.run(self.lazy_event());
        }
        if let Some(cb) = self.on_page {
            cb.run(PageEvent { first: state.first, rows: state.rows });
        }
    }

    fn lazy_event(&self) -> LazyLoadEvent {
        LazyLoadEvent { first: self.first.get_untracked(), rows: self.rows.get_untracked(), sort_order: self.sort_order.get_untracked() }
    }
}

/// Data as a list or a grid.
///
/// Every property accepts a value or a signal; `first` and `rows` are two-way signals.
#[component]
pub fn DataView<T>(
    /// Items.
    #[prop(optional, into)]
    value: Prop<Vec<T>>,
    /// List template: receives the items of the current page.
    #[prop(optional, into)]
    list: Option<Callback<Vec<T>, AnyView>>,
    /// Grid template: receives the items of the current page.
    #[prop(optional, into)]
    grid: Option<Callback<Vec<T>, AnyView>>,
    /// Layout.
    #[prop(optional, into)]
    layout: Prop<DataViewLayout>,
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
    /// Total records in `lazy` mode.
    #[prop(optional, into)]
    total_records: Prop<usize>,
    /// Number of page links.
    #[prop(optional, default = Prop::Static(5), into)]
    page_links: Prop<usize>,
    /// Options for the number of rows per page.
    #[prop(optional, into)]
    rows_per_page_options: Prop<Vec<RowsPerPage>>,
    /// Paginator position.
    #[prop(optional, into)]
    paginator_position: Prop<PaginatorPosition>,
    /// Extra class of the paginator.
    #[prop(optional, into)]
    paginator_style_class: OptProp<String>,
    /// Show the paginator when there is a single page.
    #[prop(optional, default = Prop::Static(true), into)]
    always_show_paginator: Prop<bool>,
    /// Height of the paginator dropdown.
    #[prop(optional, default = Prop::Static(Some("200px".to_owned())), into)]
    paginator_dropdown_scroll_height: OptProp<String>,
    /// Current page report template.
    #[prop(optional, default = Prop::Static(Some("{currentPage} of {totalPages}".to_owned())), into)]
    current_page_report_template: OptProp<String>,
    /// Show the current page report.
    #[prop(optional, into)]
    show_current_page_report: Prop<bool>,
    /// Show the jump-to-page dropdown.
    #[prop(optional, into)]
    show_jump_to_page_dropdown: Prop<bool>,
    /// Show the first and last page buttons.
    #[prop(optional, default = Prop::Static(true), into)]
    show_first_last_icon: Prop<bool>,
    /// Show the page links.
    #[prop(optional, default = Prop::Static(true), into)]
    show_page_links: Prop<bool>,
    /// Data arrives page by page from the application.
    #[prop(optional, into)]
    lazy: Prop<bool>,
    /// Call `on_lazy_load` on creation (in `lazy` mode).
    #[prop(optional, default = true)]
    lazy_load_on_init: bool,
    /// Filter predicate (replaces `filterBy`).
    #[prop(optional, into)]
    filter: Option<Callback<T, bool>>,
    /// Comparison for sorting (replaces `sortField`).
    #[prop(optional, into)]
    compare: Option<Callback<(T, T), Ordering>>,
    /// Sort order: 1, -1 or 0.
    #[prop(optional, into)]
    sort_order: Prop<i32>,
    /// Text when there is no data.
    #[prop(optional, into)]
    empty_message: OptProp<String>,
    /// Show the loading indicator.
    #[prop(optional, into)]
    loading: Prop<bool>,
    /// Header.
    #[prop(optional, into)]
    header: Option<ViewFn>,
    /// Footer.
    #[prop(optional, into)]
    footer: Option<ViewFn>,
    /// Custom content when there is no data.
    #[prop(optional, into)]
    empty: Option<ViewFn>,
    /// Inline style of the root.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Extra class of the root.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Called when the page changes.
    #[prop(optional, into)]
    on_page: Option<Callback<PageEvent>>,
    /// Called in `lazy` mode on page change, sort change and on creation.
    #[prop(optional, into)]
    on_lazy_load: Option<Callback<LazyLoadEvent>>,
    /// Pass-through of the parts.
    #[prop(optional)]
    pt: Option<DataViewPt>,
) -> impl IntoView
where
    T: Clone + Send + Sync + 'static,
{
    let pt = Arc::new(pt.unwrap_or_default());
    let core = Arc::new(Core {
        value,
        first: first.unwrap_or_default(),
        rows: rows.unwrap_or_else(|| RwSignal::new(default_rows)),
        total_records,
        lazy: lazy.clone(),
        paginator: paginator.clone(),
        filter,
        compare,
        sort_order,
        on_page,
        on_lazy_load,
    });

    // Returning to a valid page, initial load and sorting in `lazy` mode - browser only.
    #[cfg(target_arch = "wasm32")]
    {
        let c = core.clone();
        Effect::new(move |_| {
            let prepared_len = c.prepared().len();
            let total = c.total(prepared_len);
            let (first, rows) = (c.first.get_untracked(), c.rows.get());
            let clamped = clamp_first(first, total, rows);
            if clamped != first {
                c.first.set(clamped);
            }
        });
        if lazy.get_untracked()
            && lazy_load_on_init
            && let Some(cb) = on_lazy_load
        {
            let c = core.clone();
            Effect::new(move |previous: Option<()>| {
                let _ = c.sort_order.get();
                if previous.is_none() {
                    cb.run(c.lazy_event());
                }
            });
        }
        // A sort order change returns to the first page and, in `lazy` mode, requests data.
        let c = core.clone();
        let started = Arc::new(AtomicBool::new(false));
        Effect::new(move |_| {
            let _ = c.sort_order.get();
            if started.swap(true, AtomicOrdering::Relaxed) {
                c.first.set(0);
                if c.lazy.get_untracked()
                    && let Some(cb) = c.on_lazy_load
                {
                    cb.run(c.lazy_event());
                }
            }
        });
    }
    #[cfg(not(target_arch = "wasm32"))]
    let _ = lazy_load_on_init;

    let root_class = {
        let (layout, pt) = (layout.clone(), pt.clone());
        move || {
            let mut cn = String::from(match layout.get() {
                DataViewLayout::List => "p-dataview p-component p-dataview-list",
                DataViewLayout::Grid => "p-dataview p-component p-dataview-grid",
            });
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

    let loading_view = {
        let (pt, loading) = (pt.clone(), loading.clone());
        move || {
            loading.get().then(|| {
                view! {
                    <div class={pt.loading.merge_class("p-dataview-loading")} {..pt.loading.attrs()}>
                        <div class="p-dataview-loading-overlay p-overlay-mask">
                            <SpinnerIcon class="p-dataview-loading-icon" spin=true />
                        </div>
                    </div>
                }
            })
        }
    };
    let header_view = header.map(|h| {
        let pt = pt.clone();
        view! { <div class={pt.header.merge_class("p-dataview-header")} {..pt.header.attrs()}>{h.run()}</div> }
    });
    let footer_view = footer.map(|f| {
        let pt = pt.clone();
        view! { <div class={pt.footer.merge_class("p-dataview-footer")} {..pt.footer.attrs()}>{f.run()}</div> }
    });

    let paginator_view = |position: &'static str| {
        let (core, pt) = (core.clone(), pt.clone());
        let (paginator_position, paginator_style_class) = (paginator_position.clone(), paginator_style_class.clone());
        let (rows_options, page_links, always_show, scroll, template, report, jump, first_last, page_links_shown) = (
            rows_per_page_options.clone(),
            page_links.clone(),
            always_show_paginator.clone(),
            paginator_dropdown_scroll_height.clone(),
            current_page_report_template.clone(),
            show_current_page_report.clone(),
            show_jump_to_page_dropdown.clone(),
            show_first_last_icon.clone(),
            show_page_links.clone(),
        );
        move || {
            let wanted = match paginator_position.get() {
                PaginatorPosition::Top => position == "top",
                PaginatorPosition::Bottom => position == "bottom",
                PaginatorPosition::Both => true,
            };
            (core.paginator.get() && wanted).then(|| {
                let class = {
                    let base = format!("p-dataview-paginator-{position}");
                    match paginator_style_class.get() {
                        Some(extra) if !extra.is_empty() => format!("{base} {extra}"),
                        _ => base,
                    }
                };
                let total_records = {
                    let core = core.clone();
                    Signal::derive(move || {
                        let prepared_len = core.prepared().len();
                        core.total(prepared_len)
                    })
                };
                let on_page_change = {
                    let core = core.clone();
                    Callback::new(move |state: PaginatorState| core.paginate(state))
                };
                view! {
                    <Paginator
                        first=core.first
                        rows=core.rows
                        total_records=total_records
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
                        show_first_last_icon=Signal::derive({
                            let first_last = first_last.clone();
                            move || Some(first_last.get())
                        })
                        show_page_links=Signal::derive({
                            let page_links_shown = page_links_shown.clone();
                            move || Some(page_links_shown.get())
                        })
                        show_jump_to_page_dropdown=jump.clone()
                        dropdown_scroll_height=scroll.clone()
                        style_class=class
                        on_page_change=on_page_change
                        pt=pt.paginator.clone()
                    />
                }
            })
        }
    };
    let top = paginator_view("top");
    let bottom = paginator_view("bottom");

    let content = {
        let (core, pt) = (core.clone(), pt.clone());
        move || {
            let prepared = core.prepared();
            let rows = core.rows.get();
            let first = clamp_first(core.first.get(), core.total(prepared.len()), rows);
            let visible = page(&prepared, core.paginator.get(), core.lazy.get(), first, rows);
            let empty_data = prepared.is_empty();
            let body = match layout.get() {
                DataViewLayout::List => list.map(|cb| cb.run(visible)),
                DataViewLayout::Grid => grid.map(|cb| cb.run(visible)),
            };
            let message = (empty_data && !loading.get()).then(|| {
                let text = empty_message
                    .get()
                    .filter(|m| !m.is_empty())
                    .unwrap_or_else(|| use_prime_config().locale.text("emptyMessage").to_owned());
                let custom = empty.clone().map(|e| e.run().into_any());
                view! {
                    <div class={pt.empty_message.merge_class("p-dataview-empty-message")} {..pt.empty_message.attrs()}>
                        {custom.unwrap_or_else(|| text.into_any())}
                    </div>
                }
            });
            view! { {body}{message} }
        }
    };
    view! {
        <div class={root_class} style={root_style} data-pc-name="dataview" data-pc-section="root" {..pt.root.attrs()}>
            {loading_view}
            {header_view}
            {top}
            <div class={pt.content.merge_class("p-dataview-content")} {..pt.content.attrs()}>{content}</div>
            {bottom}
            {footer_view}
        </div>
    }
}
