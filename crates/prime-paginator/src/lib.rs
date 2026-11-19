//! Paginator — port of PrimeNG 21.1.10 `p-paginator`.
//!
//! Markup and classes as in PrimeNG: root `div.p-paginator`, buttons `p-paginator-first/prev/next/last`, `p-paginator-page`
//! links in `span.p-paginator-pages`, the `p-paginator-current` report, dropdowns for jumping to a page and for the number
//! of rows (`Select`), the jump field (`InputNumber`) and the `p-paginator-content-start/end` areas.
//!
//! Control is via two-way signals `first` (index of the first record) and `rows` (rows per page); a page change is
//! reported by `on_page_change`. Without signals the component keeps internal state.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_paginator::Paginator;
//!
//! let first = RwSignal::new(20);
//! let html = view! { <Paginator first=first rows=RwSignal::new(10) total_records=120 /> }.to_html();
//! assert!(html.contains("p-paginator-page-selected") && html.contains("aria-current=\"page\""));
//! ```

mod state;

use leptos::either::Either;
use leptos::prelude::*;
use prime_icons::{AngleDoubleLeftIcon, AngleDoubleRightIcon, AngleLeftIcon, AngleRightIcon};
use prime_inputnumber::InputNumber;
use prime_rs_core::{ClassNames, OptProp, Prop, Pt, use_prime_config};
use prime_select::{Select, SelectChange, SelectOption};
use std::sync::Arc;

pub use state::{
    PaginatorState, RowsPerPage, current_page, link_bounds, links, page, page_after_rows_change, page_count, page_items, report,
    retreat_page, rows_items, write_nav_classes, write_page_classes,
};

/// Pass-through parts of Paginator.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PaginatorPt {
    /// Root element.
    pub root: Pt,
    /// Left area.
    pub content_start: Pt,
    /// Right area.
    pub content_end: Pt,
    /// Page report.
    pub current: Pt,
    /// "First page" button.
    pub first: Pt,
    /// "First page" button icon.
    pub first_icon: Pt,
    /// "Previous page" button.
    pub prev: Pt,
    /// "Previous page" button icon.
    pub prev_icon: Pt,
    /// Links container.
    pub pages: Pt,
    /// Page link.
    pub page: Pt,
    /// "Next page" button.
    pub next: Pt,
    /// "Next page" button icon.
    pub next_icon: Pt,
    /// "Last page" button.
    pub last: Pt,
    /// "Last page" button icon.
    pub last_icon: Pt,
}

/// Pagination: buttons, page links, jump to a page and rows-per-page selection.
///
/// Each property accepts a value or a signal; `first` and `rows` are two-way signals.
#[component]
pub fn Paginator(
    /// Two-way signal of the first record index.
    #[prop(optional)]
    first: Option<RwSignal<usize>>,
    /// Initial value of `first` if no signal is passed.
    #[prop(optional)]
    default_first: usize,
    /// Two-way signal of rows per page.
    #[prop(optional)]
    rows: Option<RwSignal<usize>>,
    /// Initial rows per page if no signal is passed (10 by default).
    #[prop(optional)]
    default_rows: Option<usize>,
    /// Total records.
    #[prop(optional, into)]
    total_records: Prop<usize>,
    /// Options of the rows-per-page list; an empty list means no dropdown.
    #[prop(optional, into)]
    rows_per_page_options: Prop<Vec<RowsPerPage>>,
    /// How many page links to show (5 by default).
    #[prop(optional, into)]
    page_link_size: OptProp<usize>,
    /// Show even if there is a single page (`true` by default).
    #[prop(optional, into)]
    always_show: OptProp<bool>,
    /// Current page report.
    #[prop(optional, into)]
    show_current_page_report: Prop<bool>,
    /// Report template (`{currentPage} of {totalPages}` by default).
    #[prop(optional, into)]
    current_page_report_template: OptProp<String>,
    /// "First" and "last" buttons (`true` by default).
    #[prop(optional, into)]
    show_first_last_icon: OptProp<bool>,
    /// Page links (`true` by default).
    #[prop(optional, into)]
    show_page_links: OptProp<bool>,
    /// Dropdown for jumping to a page.
    #[prop(optional, into)]
    show_jump_to_page_dropdown: Prop<bool>,
    /// Jump-to-page field.
    #[prop(optional, into)]
    show_jump_to_page_input: Prop<bool>,
    /// Height of the lists (`200px` by default).
    #[prop(optional, into)]
    dropdown_scroll_height: OptProp<String>,
    /// Inline style of the root.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Extra class of the root.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// "First page" button icon.
    #[prop(optional, into)]
    first_page_link_icon: Option<ViewFn>,
    /// "Previous page" button icon.
    #[prop(optional, into)]
    previous_page_link_icon: Option<ViewFn>,
    /// "Next page" button icon.
    #[prop(optional, into)]
    next_page_link_icon: Option<ViewFn>,
    /// "Last page" button icon.
    #[prop(optional, into)]
    last_page_link_icon: Option<ViewFn>,
    /// Dropdowns icon.
    #[prop(optional, into)]
    dropdown_icon: Option<ViewFn>,
    /// Content of the left area; receives the state.
    #[prop(optional, into)]
    template_left: Option<Callback<PaginatorState, AnyView>>,
    /// Content of the right area; receives the state.
    #[prop(optional, into)]
    template_right: Option<Callback<PaginatorState, AnyView>>,
    /// Called when the page changes.
    #[prop(optional, into)]
    on_page_change: Option<Callback<PaginatorState>>,
    /// Pass-through parts.
    #[prop(optional)]
    pt: Option<PaginatorPt>,
) -> impl IntoView {
    let pt = Arc::new(pt.unwrap_or_default());
    let locale = use_prime_config().locale;
    let first = first.unwrap_or_else(|| RwSignal::new(default_first));
    let rows = rows.unwrap_or_else(|| RwSignal::new(default_rows.unwrap_or(10)));
    let props = Arc::new(Props {
        first,
        rows,
        total_records,
        rows_per_page_options,
        page_link_size,
        always_show,
        show_current_page_report,
        current_page_report_template,
        show_first_last_icon,
        show_page_links,
        show_jump_to_page_dropdown,
        show_jump_to_page_input,
        dropdown_scroll_height,
        style,
        style_class,
        first_page_link_icon,
        previous_page_link_icon,
        next_page_link_icon,
        last_page_link_icon,
        dropdown_icon,
        template_left,
        template_right,
        on_page_change,
        first_label: locale.text("aria.firstPageLabel").to_owned(),
        prev_label: locale.text("aria.prevPageLabel").to_owned(),
        next_label: locale.text("aria.nextPageLabel").to_owned(),
        last_label: locale.text("aria.lastPageLabel").to_owned(),
        page_label: locale.text("aria.pageLabel").to_owned(),
        rows_label: locale.text("aria.rowsPerPageLabel").to_owned(),
        jump_label: locale.text("aria.jumpToPageDropdownLabel").to_owned(),
        jtp_value: RwSignal::new(None),
        rpp_value: RwSignal::new(None),
        jump_input: RwSignal::new(None),
    });
    props.jtp_value.set(props.current_page_index().map(|p| p.to_string()));
    props.rpp_value.set(Some(rows.get_untracked().to_string()));
    props.jump_input.set(Some(props.current_page_number() as f64));

    // One-way sync of the internal list and field signals with `first`/`rows`, jump field → page and step back
    // when the record count shrinks — browser only (in native tests without an executor the effects panic).
    #[cfg(target_arch = "wasm32")]
    {
        let p = props.clone();
        Effect::new(move |_| {
            let value = Some(p.rows.get().to_string());
            if p.rpp_value.get_untracked() != value {
                p.rpp_value.set(value);
            }
        });
        let p = props.clone();
        Effect::new(move |_| {
            let value = p.current_page_index().map(|i| i.to_string());
            if p.jtp_value.get_untracked() != value {
                p.jtp_value.set(value);
            }
            let number = Some(p.current_page_number() as f64);
            if p.jump_input.get_untracked() != number {
                p.jump_input.set(number);
            }
        });
        let p = props.clone();
        Effect::new(move |_| {
            if let Some(n) = p.jump_input.get()
                && n >= 1.0
                && n.fract() == 0.0
                && n as usize != untrack(|| p.current_page_number())
            {
                p.change_page(n as usize - 1);
            }
        });
        let p = props.clone();
        Effect::new(move |_| {
            let (first, rows, total) = (p.first.get(), p.rows.get(), p.total_records.get());
            if let Some(back) = retreat_page(first, rows, total) {
                p.change_page(back);
            }
        });
    }

    let root_class = {
        let (props, pt) = (props.clone(), pt.clone());
        move || {
            let mut cn = ClassNames::new();
            cn.add("p-paginator p-component");
            props.style_class.with(|c| cn.add_user(c.as_deref()));
            cn.add_user(pt.root.extra_class());
            cn.finish_cow()
        }
    };
    let root_style = {
        let (props, pt) = (props.clone(), pt.clone());
        move || {
            let mut style = props.style.with(|st| pt.root.merge_style(st.as_deref().unwrap_or_default()));
            if !props.visible() {
                if !style.is_empty() && !style.trim_end().ends_with(';') {
                    style.push(';');
                }
                style.push_str(" display: none;");
            }
            let style = style.trim().to_owned();
            (!style.is_empty()).then_some(style)
        }
    };
    let left = {
        let (props, pt) = (props.clone(), pt.clone());
        move || {
            props.template_left.map(|template| {
                let (class, attrs) = (pt.content_start.merge_class("p-paginator-content-start"), pt.content_start.attrs());
                let props = props.clone();
                view! { <div class={class} {..attrs}>{move || template.run(props.state())}</div> }
            })
        }
    };
    let right = {
        let (props, pt) = (props.clone(), pt.clone());
        move || {
            props.template_right.map(|template| {
                let (class, attrs) = (pt.content_end.merge_class("p-paginator-content-end"), pt.content_end.attrs());
                let props = props.clone();
                view! { <div class={class} {..attrs}>{move || template.run(props.state())}</div> }
            })
        }
    };
    let current = {
        let (props, pt) = (props.clone(), pt.clone());
        move || {
            props.show_current_page_report.get().then(|| {
                let (class, attrs) = (pt.current.merge_class("p-paginator-current"), pt.current.attrs());
                let props = props.clone();
                view! { <span class={class} {..attrs}>{move || props.report_text()}</span> }
            })
        }
    };
    let first_button = {
        let (props, pt) = (props.clone(), pt.clone());
        move || props.show_first_last_icon.get().unwrap_or(true).then(|| nav_button(&props, &pt, Nav::First))
    };
    let prev_button = nav_button(&props, &pt, Nav::Prev);
    let pages = {
        let (props, pt) = (props.clone(), pt.clone());
        move || props.show_page_links.get().unwrap_or(true).then(|| pages_view(&props, &pt))
    };
    let jump_dropdown = {
        let props = props.clone();
        move || props.show_jump_to_page_dropdown.get().then(|| jump_dropdown_view(&props))
    };
    let next_button = nav_button(&props, &pt, Nav::Next);
    let last_button = {
        let (props, pt) = (props.clone(), pt.clone());
        move || props.show_first_last_icon.get().unwrap_or(true).then(|| nav_button(&props, &pt, Nav::Last))
    };
    let jump_input = {
        let props = props.clone();
        move || props.show_jump_to_page_input.get().then(|| jump_input_view(&props))
    };
    let rows_dropdown = {
        let props = props.clone();
        move || (!props.rows_per_page_options.with(Vec::is_empty)).then(|| rows_dropdown_view(&props))
    };
    let root_attrs = pt.root.attrs();

    view! {
        <div class={root_class} style={root_style} {..root_attrs}>
            {left}
            {current}
            {first_button}
            {prev_button}
            {pages}
            {jump_dropdown}
            {next_button}
            {last_button}
            {jump_input}
            {rows_dropdown}
            {right}
        </div>
    }
}

/// Which navigation button.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Nav {
    First,
    Prev,
    Next,
    Last,
}

/// Navigation button: classes, `disabled` (except "first", as in PrimeNG), label, icon.
fn nav_button(props: &Arc<Props>, pt: &Arc<PaginatorPt>, nav: Nav) -> impl IntoView + use<> {
    let (base, icon_base, label, part, icon_part) = match nav {
        Nav::First => ("p-paginator-first", "p-paginator-first-icon", props.first_label.clone(), &pt.first, &pt.first_icon),
        Nav::Prev => ("p-paginator-prev", "p-paginator-prev-icon", props.prev_label.clone(), &pt.prev, &pt.prev_icon),
        Nav::Next => ("p-paginator-next", "p-paginator-next-icon", props.next_label.clone(), &pt.next, &pt.next_icon),
        Nav::Last => ("p-paginator-last", "p-paginator-last-icon", props.last_label.clone(), &pt.last, &pt.last_icon),
    };
    let disabled = {
        let props = props.clone();
        move || match nav {
            Nav::First | Nav::Prev => props.is_first() || props.is_empty(),
            Nav::Next | Nav::Last => props.is_last() || props.is_empty(),
        }
    };
    let class = {
        let (disabled, part) = (disabled.clone(), part.clone());
        move || {
            let mut cn = ClassNames::new();
            write_nav_classes(&mut cn, base, disabled());
            cn.add_user(part.extra_class());
            cn.finish_cow()
        }
    };
    let disabled_attr = {
        let disabled = disabled.clone();
        move || (nav != Nav::First && disabled()).then_some("")
    };
    let click = {
        let props = props.clone();
        move |e: web_sys::MouseEvent| {
            match nav {
                Nav::First => {
                    if !props.is_first() {
                        props.change_page(0);
                    }
                }
                Nav::Prev => {
                    if let Some(p) = props.page_index().checked_sub(1) {
                        props.change_page(p);
                    }
                }
                Nav::Next => props.change_page(props.page_index() + 1),
                Nav::Last => {
                    if !props.is_last()
                        && let Some(p) = props.page_total().checked_sub(1)
                    {
                        props.change_page(p);
                    }
                }
            }
            e.prevent_default();
        }
    };
    let icon_class = icon_part.merge_class(icon_base);
    let icon_attrs = icon_part.attrs();
    let custom = match nav {
        Nav::First => props.first_page_link_icon.clone(),
        Nav::Prev => props.previous_page_link_icon.clone(),
        Nav::Next => props.next_page_link_icon.clone(),
        Nav::Last => props.last_page_link_icon.clone(),
    };
    let icon = match (custom, nav) {
        (Some(icon), _) => view! { <span class={icon_class}>{icon.run()}</span> }.into_any(),
        (None, Nav::First) => view! { <AngleDoubleLeftIcon class={icon_class} {..icon_attrs} /> }.into_any(),
        (None, Nav::Prev) => view! { <AngleLeftIcon class={icon_class} {..icon_attrs} /> }.into_any(),
        (None, Nav::Next) => view! { <AngleRightIcon class={icon_class} {..icon_attrs} /> }.into_any(),
        (None, Nav::Last) => view! { <AngleDoubleRightIcon class={icon_class} {..icon_attrs} /> }.into_any(),
    };
    let attrs = part.attrs();
    view! {
        <button type="button" class={class} disabled={disabled_attr} aria-label={label} on:click={click} {..attrs}>
            {icon}
        </button>
    }
}

/// Page links.
fn pages_view(props: &Arc<Props>, pt: &Arc<PaginatorPt>) -> impl IntoView + use<> {
    let (class, attrs) = (pt.pages.merge_class("p-paginator-pages"), pt.pages.attrs());
    let buttons = {
        let (props, pt) = (props.clone(), pt.clone());
        move || {
            let current = props.page_index();
            links(current, props.page_total(), props.link_size())
                .into_iter()
                .map(|number| {
                    let selected = number - 1 == current;
                    let mut cn = ClassNames::new();
                    write_page_classes(&mut cn, selected);
                    cn.add_user(pt.page.extra_class());
                    let class = cn.finish_cow();
                    let label = props.page_label.replace("{page}", &number.to_string());
                    let click = {
                        let props = props.clone();
                        move |e: web_sys::MouseEvent| {
                            props.change_page(number - 1);
                            e.prevent_default();
                        }
                    };
                    let current_attr = selected.then_some("page");
                    let page_attrs = pt.page.attrs();
                    view! {
                        <button type="button" class={class} aria-label={label} aria-current={current_attr} on:click={click} {..page_attrs}>
                            {number.to_string()}
                        </button>
                    }
                })
                .collect_view()
        }
    };
    view! { <span class={class} {..attrs}>{buttons}</span> }
}

/// Dropdown for jumping to a page: the label is the page report.
fn jump_dropdown_view(props: &Arc<Props>) -> impl IntoView + use<> {
    let options = {
        let props = props.clone();
        Signal::derive(move || {
            page_items(props.page_total()).into_iter().map(|(label, value)| SelectOption::new(label, value)).collect::<Vec<_>>()
        })
    };
    let disabled = {
        let props = props.clone();
        Signal::derive(move || props.is_empty())
    };
    let selected = {
        let props = props.clone();
        Callback::new(move |_: SelectOption| {
            let props = props.clone();
            view! { {move || props.report_text()} }.into_any()
        })
    };
    let change = {
        let props = props.clone();
        Callback::new(move |e: SelectChange| {
            if let Some(index) = e.value.and_then(|v| v.parse::<usize>().ok()) {
                props.change_page(index);
            }
        })
    };
    let scroll = props.dropdown_scroll_height.clone();
    let value = props.jtp_value;
    let label = props.jump_label.clone();
    match props.dropdown_icon.clone() {
        Some(icon) => Either::Left(view! {
            <Select
                options=options
                value=value
                disabled=disabled
                aria_label=label
                style_class="p-paginator-jtp-dropdown"
                scroll_height=scroll
                selected_item_template=selected
                dropdown_icon=icon
                on_change=change
            />
        }),
        None => Either::Right(view! {
            <Select
                options=options
                value=value
                disabled=disabled
                aria_label=label
                style_class="p-paginator-jtp-dropdown"
                scroll_height=scroll
                selected_item_template=selected
                on_change=change
            />
        }),
    }
}

/// Jump-to-page field.
fn jump_input_view(props: &Arc<Props>) -> impl IntoView + use<> {
    let disabled = {
        let props = props.clone();
        Signal::derive(move || props.is_empty())
    };
    view! { <InputNumber value=props.jump_input disabled=disabled style_class="p-paginator-jtp-input" /> }
}

/// Rows-per-page dropdown.
fn rows_dropdown_view(props: &Arc<Props>) -> impl IntoView + use<> {
    let options = {
        let props = props.clone();
        Signal::derive(move || {
            let items = props.rows_per_page_options.with(|o| rows_items(o, props.total_records.get()));
            items.into_iter().map(|(label, value)| SelectOption::new(label, value)).collect::<Vec<_>>()
        })
    };
    let disabled = {
        let props = props.clone();
        Signal::derive(move || props.is_empty())
    };
    let change = {
        let props = props.clone();
        Callback::new(move |e: SelectChange| {
            if let Some(rows) = e.value.and_then(|v| v.parse::<usize>().ok()) {
                props.rows.set(rows);
                if let Some(p) = page_after_rows_change(props.first.get_untracked(), rows, props.total_records.get_untracked()) {
                    props.change_page(p);
                }
            }
        })
    };
    let scroll = props.dropdown_scroll_height.clone();
    let value = props.rpp_value;
    let label = props.rows_label.clone();
    match props.dropdown_icon.clone() {
        Some(icon) => Either::Left(view! {
            <Select
                options=options
                value=value
                disabled=disabled
                aria_label=label
                style_class="p-paginator-rpp-dropdown"
                scroll_height=scroll
                dropdown_icon=icon
                on_change=change
            />
        }),
        None => Either::Right(view! {
            <Select
                options=options
                value=value
                disabled=disabled
                aria_label=label
                style_class="p-paginator-rpp-dropdown"
                scroll_height=scroll
                on_change=change
            />
        }),
    }
}

/// Properties and instance state.
struct Props {
    first: RwSignal<usize>,
    rows: RwSignal<usize>,
    total_records: Prop<usize>,
    rows_per_page_options: Prop<Vec<RowsPerPage>>,
    page_link_size: OptProp<usize>,
    always_show: OptProp<bool>,
    show_current_page_report: Prop<bool>,
    current_page_report_template: OptProp<String>,
    show_first_last_icon: OptProp<bool>,
    show_page_links: OptProp<bool>,
    show_jump_to_page_dropdown: Prop<bool>,
    show_jump_to_page_input: Prop<bool>,
    dropdown_scroll_height: OptProp<String>,
    style: OptProp<String>,
    style_class: OptProp<String>,
    first_page_link_icon: Option<ViewFn>,
    previous_page_link_icon: Option<ViewFn>,
    next_page_link_icon: Option<ViewFn>,
    last_page_link_icon: Option<ViewFn>,
    dropdown_icon: Option<ViewFn>,
    template_left: Option<Callback<PaginatorState, AnyView>>,
    template_right: Option<Callback<PaginatorState, AnyView>>,
    on_page_change: Option<Callback<PaginatorState>>,
    first_label: String,
    prev_label: String,
    next_label: String,
    last_label: String,
    page_label: String,
    rows_label: String,
    jump_label: String,
    /// Value of the jump-to-page list (page index).
    jtp_value: RwSignal<Option<String>>,
    /// Value of the rows list.
    rpp_value: RwSignal<Option<String>>,
    /// Value of the jump field (page counted from one).
    jump_input: RwSignal<Option<f64>>,
}

impl Props {
    // ---- data (subscribed) ----

    fn page_total(&self) -> usize {
        page_count(self.total_records.get(), self.rows.get())
    }

    fn page_index(&self) -> usize {
        page(self.first.get(), self.rows.get())
    }

    fn is_empty(&self) -> bool {
        self.page_total() == 0
    }

    fn is_first(&self) -> bool {
        self.page_index() == 0
    }

    fn is_last(&self) -> bool {
        self.page_index() + 1 == self.page_total()
    }

    /// Index of the current page for the jump list; `None` for an empty list.
    fn current_page_index(&self) -> Option<usize> {
        (!self.is_empty()).then(|| self.page_index())
    }

    /// Current page counted from one (0 for an empty list).
    fn current_page_number(&self) -> usize {
        current_page(self.first.get(), self.rows.get(), self.total_records.get())
    }

    fn link_size(&self) -> usize {
        self.page_link_size.get().unwrap_or(5)
    }

    fn state(&self) -> PaginatorState {
        PaginatorState {
            page: self.page_index(),
            first: self.first.get(),
            rows: self.rows.get(),
            page_count: self.page_total(),
            total_records: self.total_records.get(),
        }
    }

    fn report_text(&self) -> String {
        let template = self.current_page_report_template.get().unwrap_or_else(|| "{currentPage} of {totalPages}".to_owned());
        report(&template, self.first.get(), self.rows.get(), self.total_records.get())
    }

    /// Whether the root is visible: `always_show` or more than one link.
    fn visible(&self) -> bool {
        self.always_show.get().unwrap_or(true) || links(self.page_index(), self.page_total(), self.link_size()).len() > 1
    }

    // ---- actions ----

    /// `changePage`: go to page `p` (zero-based) if it exists; writes `first` and calls `on_page_change`.
    fn change_page(&self, p: usize) {
        let rows = self.rows.get_untracked();
        let count = page_count(self.total_records.get_untracked(), rows);
        if p < count {
            self.first.set(rows * p);
            if let Some(cb) = self.on_page_change {
                cb.run(PaginatorState {
                    page: p,
                    first: rows * p,
                    rows,
                    page_count: count,
                    total_records: self.total_records.get_untracked(),
                });
            }
        }
    }
}
