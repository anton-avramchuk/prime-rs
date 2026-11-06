//! OrderList — порт `p-orderlist` из PrimeNG 21.1.10.
//!
//! Список с изменяемым порядком: выбранные элементы двигаются кнопками вверх, в начало, вниз и в конец. Сам список,
//! выбор и поиск — готовый [`Listbox`](prime_listbox::Listbox); перестановка — чистая логика в [`reorder`].
//!
//! ```
//! #![recursion_limit = "512"]
//! use leptos::prelude::*;
//! use prime_listbox::ListboxOption;
//! use prime_orderlist::OrderList;
//!
//! let items = RwSignal::new(vec![ListboxOption::new("Apple", "a"), ListboxOption::new("Pear", "p")]);
//! let html = view! { <OrderList value=items /> }.to_html();
//! assert!(html.contains("p-orderlist-controls") && html.contains("Apple"));
//! ```

#![recursion_limit = "512"]

pub mod reorder;

use leptos::prelude::*;
use prime_button::{Button, ButtonPt, ButtonSeverity};
use prime_core::{OptProp, Prop, Pt};
use prime_icons::{AngleDoubleDownIcon, AngleDoubleUpIcon, AngleDownIcon, AngleUpIcon};
use prime_listbox::{Listbox, ListboxOption, ListboxPt};
use std::sync::Arc;

/// Положение кнопок относительно списка.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ControlsPosition {
    /// Слева (по умолчанию).
    #[default]
    Left,
    /// Справа.
    Right,
}

impl ControlsPosition {
    fn class(self) -> &'static str {
        match self {
            Self::Left => "p-orderlist p-component p-orderlist-controls-left",
            Self::Right => "p-orderlist p-component p-orderlist-controls-right",
        }
    }
}

/// Pass-through частей [`OrderList`].
#[derive(Clone, Debug, Default)]
pub struct OrderListPt {
    /// Корень.
    pub root: Pt,
    /// Панель кнопок.
    pub controls: Pt,
    /// Кнопка «вверх».
    pub move_up_button: Pt,
    /// Кнопка «в начало».
    pub move_top_button: Pt,
    /// Кнопка «вниз».
    pub move_down_button: Pt,
    /// Кнопка «в конец».
    pub move_bottom_button: Pt,
    /// Части списка.
    pub listbox: ListboxPt,
}

#[derive(Clone, Copy)]
enum Move {
    Up,
    Top,
    Down,
    Bottom,
}

/// Список с изменяемым порядком элементов.
///
/// Каждое свойство принимает значение или сигнал; `value` и `selection` — двусторонние сигналы.
#[component]
pub fn OrderList(
    /// Элементы в текущем порядке.
    value: RwSignal<Vec<ListboxOption>>,
    /// Значения выбранных элементов; без сигнала хранятся внутри.
    #[prop(optional)]
    selection: Option<RwSignal<Vec<String>>>,
    /// Положение кнопок.
    #[prop(optional, into)]
    controls_position: Prop<ControlsPosition>,
    /// Поле поиска над списком.
    #[prop(optional, into)]
    filter: Prop<bool>,
    /// Подсказка поля поиска.
    #[prop(optional, into)]
    filter_placeholder: OptProp<String>,
    /// Чередующиеся цвета строк.
    #[prop(optional, into)]
    striped_rows: Prop<bool>,
    /// Для выбора нужна клавиша Ctrl/Meta.
    #[prop(optional, into)]
    meta_key_selection: Prop<bool>,
    /// Недоступен.
    #[prop(optional, into)]
    disabled: Prop<bool>,
    /// Высота списка.
    #[prop(optional, default = Prop::Static(Some("14rem".to_owned())), into)]
    scroll_height: OptProp<String>,
    /// Атрибут `tabindex` списка.
    #[prop(optional, into)]
    tabindex: OptProp<i32>,
    /// Атрибут `aria-label` списка.
    #[prop(optional, into)]
    aria_label: OptProp<String>,
    /// Встроенный стиль списка.
    #[prop(optional, into)]
    list_style: OptProp<String>,
    /// Встроенный стиль корня.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Дополнительный класс корня.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Свой вид элемента (вариант и признак выбора).
    #[prop(optional, into)]
    option: Option<Callback<(ListboxOption, bool), AnyView>>,
    /// Вызывается после перестановки; получает значения выбранных элементов.
    #[prop(optional, into)]
    on_reorder: Option<Callback<Vec<String>>>,
    /// Вызывается при изменении выбора.
    #[prop(optional, into)]
    on_selection_change: Option<Callback<Vec<String>>>,
    /// Pass-through частей.
    #[prop(optional)]
    pt: Option<OrderListPt>,
) -> impl IntoView {
    let pt = Arc::new(pt.unwrap_or_default());
    let selection = selection.unwrap_or_default();
    let locale = prime_core::use_prime_config().locale;
    let labels = Arc::new([
        locale.text("aria.moveUp").to_owned(),
        locale.text("aria.moveTop").to_owned(),
        locale.text("aria.moveDown").to_owned(),
        locale.text("aria.moveBottom").to_owned(),
    ]);

    let apply = move |action: Move| {
        let selected = selection.get_untracked();
        let keys: Vec<String> = value.with_untracked(|v| v.iter().map(|o| o.value.clone()).collect());
        let order = match action {
            Move::Up => reorder::move_up(&keys, &selected),
            Move::Top => reorder::move_top(&keys, &selected),
            Move::Down => reorder::move_down(&keys, &selected),
            Move::Bottom => reorder::move_bottom(&keys, &selected),
        };
        value.update(|items| {
            let mut sorted = Vec::with_capacity(items.len());
            for key in &order {
                if let Some(pos) = items.iter().position(|o| &o.value == key) {
                    sorted.push(items.remove(pos));
                }
            }
            *items = sorted;
        });
        if let Some(cb) = on_reorder {
            cb.run(selected);
        }
    };

    let button = |action: Move, index: usize, button_pt: Pt, class: &'static str, icon: ViewFn| {
        let labels = labels.clone();
        let disabled = disabled.clone();
        let disabled_signal = Signal::derive(move || disabled.get() || selection.with(Vec::is_empty));
        view! {
            <Button
                button_type="button"
                severity=ButtonSeverity::Secondary
                icon=icon
                disabled=disabled_signal
                aria_label=Some(labels[index].clone())
                style_class=class
                on_click=Callback::new(move |_| apply(action))
                pt=ButtonPt { root: button_pt, ..Default::default() }
            />
        }
    };
    let up = button(Move::Up, 0, pt.move_up_button.clone(), "p-orderlist-move-up-button", ViewFn::from(|| view! { <AngleUpIcon /> }));
    let top =
        button(Move::Top, 1, pt.move_top_button.clone(), "p-orderlist-move-top-button", ViewFn::from(|| view! { <AngleDoubleUpIcon /> }));
    let down =
        button(Move::Down, 2, pt.move_down_button.clone(), "p-orderlist-move-down-button", ViewFn::from(|| view! { <AngleDownIcon /> }));
    let bottom = button(
        Move::Bottom,
        3,
        pt.move_bottom_button.clone(),
        "p-orderlist-move-bottom-button",
        ViewFn::from(|| view! { <AngleDoubleDownIcon /> }),
    );

    let root_class = {
        let pt = pt.clone();
        move || {
            let mut cn = controls_position.get().class().to_owned();
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
    let options = Signal::derive(move || value.get());
    let on_change = Callback::new(move |change: prime_listbox::ListboxChange| {
        if let Some(cb) = on_selection_change {
            cb.run(change.values);
        }
    });
    let list = match option {
        Some(option) => view! {
            <Listbox
                multiple=true
                options=options
                values=selection
                filter=filter
                filter_placeholder=filter_placeholder
                striped=striped_rows
                meta_key_selection=meta_key_selection
                disabled=disabled
                scroll_height=scroll_height
                tabindex=tabindex
                aria_label=aria_label
                list_style=list_style
                option=option
                on_change=on_change
                pt=pt.listbox.clone()
            />
        }
        .into_any(),
        None => view! {
            <Listbox
                multiple=true
                options=options
                values=selection
                filter=filter
                filter_placeholder=filter_placeholder
                striped=striped_rows
                meta_key_selection=meta_key_selection
                disabled=disabled
                scroll_height=scroll_height
                tabindex=tabindex
                aria_label=aria_label
                list_style=list_style
                on_change=on_change
                pt=pt.listbox.clone()
            />
        }
        .into_any(),
    };
    view! {
        <div class={root_class} style={root_style} data-pc-name="orderlist" data-pc-section="root" {..pt.root.attrs()}>
            <div class={pt.controls.merge_class("p-orderlist-controls")} {..pt.controls.attrs()}>
                {up}
                {top}
                {down}
                {bottom}
            </div>
            {list}
        </div>
    }
}
