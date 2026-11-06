//! PickList — порт `p-picklist` из PrimeNG 21.1.10.
//!
//! Два списка (источник и цель) с кнопками переноса и изменения порядка. Списки, выбор и поиск — готовый
//! [`Listbox`](prime_listbox::Listbox); перенос — чистая логика в [`transfer`], перестановка — `prime_orderlist::reorder`.
//!
//! ```
//! #![recursion_limit = "512"]
//! use leptos::prelude::*;
//! use prime_listbox::ListboxOption;
//! use prime_picklist::PickList;
//!
//! let source = RwSignal::new(vec![ListboxOption::new("Apple", "a")]);
//! let target = RwSignal::new(Vec::<ListboxOption>::new());
//! let html = view! { <PickList source=source target=target /> }.to_html();
//! assert!(html.contains("p-picklist-transfer-controls") && html.contains("Apple"));
//! ```

#![recursion_limit = "512"]

pub mod transfer;

use leptos::prelude::*;
use prime_button::{Button, ButtonPt, ButtonSeverity};
use prime_core::{OptProp, Prop, Pt};
use prime_icons::{
    AngleDoubleDownIcon, AngleDoubleLeftIcon, AngleDoubleRightIcon, AngleDoubleUpIcon, AngleDownIcon, AngleLeftIcon, AngleRightIcon,
    AngleUpIcon,
};
use prime_listbox::{Listbox, ListboxChange, ListboxFilter, ListboxOption, ListboxPt, matches_filter};
use prime_orderlist::reorder;
use std::sync::Arc;

/// Pass-through частей [`PickList`].
#[derive(Clone, Debug, Default)]
pub struct PickListPt {
    /// Корень.
    pub root: Pt,
    /// Кнопки перестановки источника.
    pub source_controls: Pt,
    /// Контейнер списка источника.
    pub source_list_container: Pt,
    /// Кнопки переноса.
    pub transfer_controls: Pt,
    /// Контейнер списка цели.
    pub target_list_container: Pt,
    /// Кнопки перестановки цели.
    pub target_controls: Pt,
    /// Части списка источника.
    pub source_listbox: ListboxPt,
    /// Части списка цели.
    pub target_listbox: ListboxPt,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Side {
    Source,
    Target,
}

#[derive(Clone, Copy)]
enum Move {
    Up,
    Top,
    Down,
    Bottom,
}

/// События и состояние, общие для обработчиков.
struct Core {
    source: RwSignal<Vec<ListboxOption>>,
    target: RwSignal<Vec<ListboxOption>>,
    sel_source: RwSignal<Vec<String>>,
    sel_target: RwSignal<Vec<String>>,
    filter_source: RwSignal<String>,
    filter_target: RwSignal<String>,
    keep_selection: Prop<bool>,
    on_move_to_target: Option<Callback<Vec<ListboxOption>>>,
    on_move_all_to_target: Option<Callback<Vec<ListboxOption>>>,
    on_move_to_source: Option<Callback<Vec<ListboxOption>>>,
    on_move_all_to_source: Option<Callback<Vec<ListboxOption>>>,
    on_source_reorder: Option<Callback<Vec<String>>>,
    on_target_reorder: Option<Callback<Vec<String>>>,
}

impl Core {
    fn lists(&self, side: Side) -> (RwSignal<Vec<ListboxOption>>, RwSignal<Vec<ListboxOption>>) {
        if side == Side::Source { (self.source, self.target) } else { (self.target, self.source) }
    }

    fn selections(&self, side: Side) -> (RwSignal<Vec<String>>, RwSignal<Vec<String>>) {
        if side == Side::Source { (self.sel_source, self.sel_target) } else { (self.sel_target, self.sel_source) }
    }

    /// Выбранные элементы `from` переходят в `to`.
    fn transfer_selected(&self, from: Side) {
        let (from_list, to_list) = self.lists(from);
        let (from_sel, to_sel) = self.selections(from);
        let selection = from_sel.get_untracked();
        if selection.is_empty() {
            return;
        }
        let (mut a, mut b) = (from_list.get_untracked(), to_list.get_untracked());
        let moved = transfer::move_selected(&mut a, &mut b, &selection);
        from_list.set(a);
        to_list.set(b);
        if self.keep_selection.get_untracked() {
            to_sel.update(|s| s.extend(moved.iter().map(|o| o.value.clone())));
        }
        from_sel.set(Vec::new());
        let callback = if from == Side::Source { self.on_move_to_target } else { self.on_move_to_source };
        if let Some(cb) = callback {
            cb.run(moved);
        }
    }

    /// Все видимые при поиске элементы `from` переходят в `to`.
    fn transfer_all(&self, from: Side) {
        let (from_list, to_list) = self.lists(from);
        let (from_sel, to_sel) = self.selections(from);
        if from_list.with_untracked(Vec::is_empty) {
            return;
        }
        let text = if from == Side::Source { self.filter_source } else { self.filter_target }.get_untracked();
        let (mut a, mut b) = (from_list.get_untracked(), to_list.get_untracked());
        let moved = transfer::move_all(&mut a, &mut b, |o| matches_filter(&o.label, &text));
        from_list.set(a);
        to_list.set(b);
        if self.keep_selection.get_untracked() {
            to_sel.update(|s| s.extend(moved.iter().map(|o| o.value.clone())));
        }
        from_sel.set(Vec::new());
        let callback = if from == Side::Source { self.on_move_all_to_target } else { self.on_move_all_to_source };
        if let Some(cb) = callback {
            cb.run(moved);
        }
    }

    fn reorder(&self, side: Side, action: Move) {
        let (list, _) = self.lists(side);
        let (selection, _) = self.selections(side);
        let selected = selection.get_untracked();
        let keys: Vec<String> = list.with_untracked(|v| v.iter().map(|o| o.value.clone()).collect());
        let order = match action {
            Move::Up => reorder::move_up(&keys, &selected),
            Move::Top => reorder::move_top(&keys, &selected),
            Move::Down => reorder::move_down(&keys, &selected),
            Move::Bottom => reorder::move_bottom(&keys, &selected),
        };
        list.update(|items| {
            let mut sorted = Vec::with_capacity(items.len());
            for key in &order {
                if let Some(pos) = items.iter().position(|o| &o.value == key) {
                    sorted.push(items.remove(pos));
                }
            }
            *items = sorted;
        });
        let callback = if side == Side::Source { self.on_source_reorder } else { self.on_target_reorder };
        if let Some(cb) = callback {
            cb.run(selected);
        }
    }
}

/// Два списка с переносом элементов между ними.
///
/// Каждое свойство принимает значение или сигнал; `source`, `target` и выбор — двусторонние сигналы.
#[component]
pub fn PickList(
    /// Элементы источника.
    source: RwSignal<Vec<ListboxOption>>,
    /// Элементы цели.
    target: RwSignal<Vec<ListboxOption>>,
    /// Выбранные значения источника.
    #[prop(optional)]
    source_selection: Option<RwSignal<Vec<String>>>,
    /// Выбранные значения цели.
    #[prop(optional)]
    target_selection: Option<RwSignal<Vec<String>>>,
    /// Заголовок источника.
    #[prop(optional, into)]
    source_header: OptProp<String>,
    /// Заголовок цели.
    #[prop(optional, into)]
    target_header: OptProp<String>,
    /// Поле поиска над списками.
    #[prop(optional, into)]
    filter: Prop<bool>,
    /// Показывать поиск источника (при `filter`).
    #[prop(optional, default = Prop::Static(true), into)]
    show_source_filter: Prop<bool>,
    /// Показывать поиск цели (при `filter`).
    #[prop(optional, default = Prop::Static(true), into)]
    show_target_filter: Prop<bool>,
    /// Подсказка поиска источника.
    #[prop(optional, into)]
    source_filter_placeholder: OptProp<String>,
    /// Подсказка поиска цели.
    #[prop(optional, into)]
    target_filter_placeholder: OptProp<String>,
    /// Чередующиеся цвета строк.
    #[prop(optional, into)]
    striped_rows: Prop<bool>,
    /// Для выбора нужна клавиша Ctrl/Meta.
    #[prop(optional, into)]
    meta_key_selection: Prop<bool>,
    /// Недоступен.
    #[prop(optional, into)]
    disabled: Prop<bool>,
    /// Кнопки перестановки источника.
    #[prop(optional, default = Prop::Static(true), into)]
    show_source_controls: Prop<bool>,
    /// Кнопки перестановки цели.
    #[prop(optional, default = Prop::Static(true), into)]
    show_target_controls: Prop<bool>,
    /// Сохранять выбор перенесённых элементов в списке назначения.
    #[prop(optional, into)]
    keep_selection: Prop<bool>,
    /// Высота списков.
    #[prop(optional, default = Prop::Static(Some("14rem".to_owned())), into)]
    scroll_height: OptProp<String>,
    /// Атрибут `tabindex` списков.
    #[prop(optional, default = Prop::Static(Some(0)), into)]
    tabindex: OptProp<i32>,
    /// Атрибут `aria-label` списка источника.
    #[prop(optional, into)]
    source_aria_label: OptProp<String>,
    /// Атрибут `aria-label` списка цели.
    #[prop(optional, into)]
    target_aria_label: OptProp<String>,
    /// Встроенный стиль списка источника.
    #[prop(optional, into)]
    source_style: OptProp<String>,
    /// Встроенный стиль списка цели.
    #[prop(optional, into)]
    target_style: OptProp<String>,
    /// Встроенный стиль корня.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Дополнительный класс корня.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Вызывается после переноса выбранных в цель.
    #[prop(optional, into)]
    on_move_to_target: Option<Callback<Vec<ListboxOption>>>,
    /// Вызывается после переноса всех в цель.
    #[prop(optional, into)]
    on_move_all_to_target: Option<Callback<Vec<ListboxOption>>>,
    /// Вызывается после переноса выбранных в источник.
    #[prop(optional, into)]
    on_move_to_source: Option<Callback<Vec<ListboxOption>>>,
    /// Вызывается после переноса всех в источник.
    #[prop(optional, into)]
    on_move_all_to_source: Option<Callback<Vec<ListboxOption>>>,
    /// Вызывается после перестановки в источнике.
    #[prop(optional, into)]
    on_source_reorder: Option<Callback<Vec<String>>>,
    /// Вызывается после перестановки в цели.
    #[prop(optional, into)]
    on_target_reorder: Option<Callback<Vec<String>>>,
    /// Вызывается при изменении выбора в источнике.
    #[prop(optional, into)]
    on_source_select: Option<Callback<Vec<String>>>,
    /// Вызывается при изменении выбора в цели.
    #[prop(optional, into)]
    on_target_select: Option<Callback<Vec<String>>>,
    /// Pass-through частей.
    #[prop(optional)]
    pt: Option<PickListPt>,
) -> impl IntoView {
    let pt = Arc::new(pt.unwrap_or_default());
    let core = Arc::new(Core {
        source,
        target,
        sel_source: source_selection.unwrap_or_default(),
        sel_target: target_selection.unwrap_or_default(),
        filter_source: RwSignal::new(String::new()),
        filter_target: RwSignal::new(String::new()),
        keep_selection,
        on_move_to_target,
        on_move_all_to_target,
        on_move_to_source,
        on_move_all_to_source,
        on_source_reorder,
        on_target_reorder,
    });
    let locale = prime_core::use_prime_config().locale;
    let label = |key: &str| locale.text(key).to_owned();

    // Кнопка: подпись, значок, условие недоступности, действие.
    let button = |aria: String, icon: ViewFn, off: Signal<bool>, action: Callback<()>, button_pt: Pt| {
        view! {
            <Button
                button_type="button"
                severity=ButtonSeverity::Secondary
                icon=icon
                disabled=off
                aria_label=Some(aria)
                on_click=Callback::new(move |_| action.run(()))
                pt=ButtonPt { root: button_pt, ..Default::default() }
            />
        }
    };
    let reorder_buttons = |side: Side| {
        let (sel, _) = core.selections(side);
        let off = {
            let disabled = disabled.clone();
            Signal::derive(move || disabled.get() || sel.with(Vec::is_empty))
        };
        let act = |m: Move| {
            let core = core.clone();
            Callback::new(move |()| core.reorder(side, m))
        };
        view! {
            {button(label("aria.moveUp"), ViewFn::from(|| view! { <AngleUpIcon /> }), off, act(Move::Up), Pt::default())}
            {button(label("aria.moveTop"), ViewFn::from(|| view! { <AngleDoubleUpIcon /> }), off, act(Move::Top), Pt::default())}
            {button(label("aria.moveDown"), ViewFn::from(|| view! { <AngleDownIcon /> }), off, act(Move::Down), Pt::default())}
            {button(label("aria.moveBottom"), ViewFn::from(|| view! { <AngleDoubleDownIcon /> }), off, act(Move::Bottom), Pt::default())}
        }
    };
    // Наличие кнопок перестановки определяется при создании: представление нельзя собрать повторно из сигнала.
    let source_controls = show_source_controls.get_untracked().then(|| {
        let buttons = reorder_buttons(Side::Source);
        view! {
            <div class={pt.source_controls.merge_class("p-picklist-controls p-picklist-source-controls")} data-pc-group-section="controls" {..pt.source_controls.attrs()}>
                {buttons}
            </div>
        }
    });
    let target_controls = show_target_controls.get_untracked().then(|| {
        let buttons = reorder_buttons(Side::Target);
        view! {
            <div class={pt.target_controls.merge_class("p-picklist-controls p-picklist-target-controls")} data-pc-group-section="controls" {..pt.target_controls.attrs()}>
                {buttons}
            </div>
        }
    });

    let transfer_buttons = {
        let off_selected = |sel: RwSignal<Vec<String>>| {
            let disabled = disabled.clone();
            Signal::derive(move || disabled.get() || sel.with(Vec::is_empty))
        };
        let off_all = |list: RwSignal<Vec<ListboxOption>>| {
            let disabled = disabled.clone();
            Signal::derive(move || disabled.get() || list.with(Vec::is_empty))
        };
        let (c1, c2, c3, c4) = (core.clone(), core.clone(), core.clone(), core.clone());
        view! {
            {button(
                label("aria.moveToTarget"),
                ViewFn::from(|| view! { <AngleRightIcon /> }),
                off_selected(core.sel_source),
                Callback::new(move |()| c1.transfer_selected(Side::Source)),
                Pt::default(),
            )}
            {button(
                label("aria.moveAllToTarget"),
                ViewFn::from(|| view! { <AngleDoubleRightIcon /> }),
                off_all(core.source),
                Callback::new(move |()| c2.transfer_all(Side::Source)),
                Pt::default(),
            )}
            {button(
                label("aria.moveToSource"),
                ViewFn::from(|| view! { <AngleLeftIcon /> }),
                off_selected(core.sel_target),
                Callback::new(move |()| c3.transfer_selected(Side::Target)),
                Pt::default(),
            )}
            {button(
                label("aria.moveAllToSource"),
                ViewFn::from(|| view! { <AngleDoubleLeftIcon /> }),
                off_all(core.target),
                Callback::new(move |()| c4.transfer_all(Side::Target)),
                Pt::default(),
            )}
        }
    };

    let list_for = |side: Side| {
        let (list, _) = core.lists(side);
        let (selection, _) = core.selections(side);
        let (header, placeholder, style, aria, show_filter, select_event, listbox_pt) = if side == Side::Source {
            (
                source_header.clone(),
                source_filter_placeholder.clone(),
                source_style.clone(),
                source_aria_label.clone(),
                show_source_filter.clone(),
                on_source_select,
                pt.source_listbox.clone(),
            )
        } else {
            (
                target_header.clone(),
                target_filter_placeholder.clone(),
                target_style.clone(),
                target_aria_label.clone(),
                show_target_filter.clone(),
                on_target_select,
                pt.target_listbox.clone(),
            )
        };
        let filter_signal = if side == Side::Source { core.filter_source } else { core.filter_target };
        let filter = filter.clone();
        let filter_on = Signal::derive(move || filter.get() && show_filter.get());
        let header_content = header.get_untracked().map(|text| ViewFn::from(move || view! { <div>{text.clone()}</div> }));
        let options = Signal::derive(move || list.get());
        let on_change = Callback::new(move |change: ListboxChange| {
            if let Some(cb) = select_event {
                cb.run(change.values);
            }
        });
        let on_filter = Callback::new(move |f: ListboxFilter| filter_signal.set(f.filter));
        let dbl_core = core.clone();
        let on_dbl_click = Callback::new(move |_| dbl_core.transfer_selected(side));
        match header_content {
            Some(header_content) => view! {
                <Listbox
                    multiple=true
                    options=options
                    values=selection
                    filter=filter_on
                    filter_placeholder=placeholder
                    striped=striped_rows.clone()
                    meta_key_selection=meta_key_selection.clone()
                    disabled=disabled.clone()
                    scroll_height=scroll_height.clone()
                    tabindex=tabindex.clone()
                    aria_label=aria
                    list_style=style
                    header_content=header_content
                    on_change=on_change
                    on_filter=on_filter
                    on_dbl_click=on_dbl_click
                    pt=listbox_pt
                />
            }
            .into_any(),
            None => view! {
                <Listbox
                    multiple=true
                    options=options
                    values=selection
                    filter=filter_on
                    filter_placeholder=placeholder
                    striped=striped_rows.clone()
                    meta_key_selection=meta_key_selection.clone()
                    disabled=disabled.clone()
                    scroll_height=scroll_height.clone()
                    tabindex=tabindex.clone()
                    aria_label=aria
                    list_style=style
                    on_change=on_change
                    on_filter=on_filter
                    on_dbl_click=on_dbl_click
                    pt=listbox_pt
                />
            }
            .into_any(),
        }
    };
    let source_list = list_for(Side::Source);
    let target_list = list_for(Side::Target);

    let root_class = {
        let pt = pt.clone();
        move || {
            let mut cn = String::from("p-picklist p-component");
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
    view! {
        <div class={root_class} style={root_style} data-pc-name="picklist" data-pc-section="root" {..pt.root.attrs()}>
            {source_controls}
            <div
                class={pt.source_list_container.merge_class("p-picklist-list-container p-picklist-source-list-container")}
                data-pc-group-section="listcontainer"
                {..pt.source_list_container.attrs()}
            >
                {source_list}
            </div>
            <div class={pt.transfer_controls.merge_class("p-picklist-controls p-picklist-transfer-controls")} data-pc-group-section="controls" {..pt.transfer_controls.attrs()}>
                {transfer_buttons}
            </div>
            <div
                class={pt.target_list_container.merge_class("p-picklist-list-container p-picklist-target-list-container")}
                data-pc-group-section="listcontainer"
                {..pt.target_list_container.attrs()}
            >
                {target_list}
            </div>
            {target_controls}
        </div>
    }
}
