//! Listbox — порт `p-listbox` из PrimeNG 21.1.10.
//!
//! Разметка и классы как в PrimeNG: корень `div.p-listbox`, шапка `p-listbox-header` (флажок «выбрать все» и поле поиска в
//! `p-iconfield`), контейнер `p-listbox-list-container` и список `ul.p-listbox-list[role=listbox]` с вариантами
//! `li.p-listbox-option`, заголовками групп `li.p-listbox-option-group` и сообщением пустого списка.
//!
//! Значение — двусторонний сигнал: `value: RwSignal<Option<String>>` в одиночном режиме и
//! `values: RwSignal<Vec<String>>` при `multiple`; без них компонент ведёт внутреннее состояние.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_listbox::{Listbox, ListboxOption};
//!
//! let value = RwSignal::new(Some("LDN".to_owned()));
//! let options = vec![ListboxOption::new("New York", "NY"), ListboxOption::new("London", "LDN")];
//! let html = view! { <Listbox options=options value=value /> }.to_html();
//! assert!(html.contains("p-listbox-list") && html.contains("p-listbox-option-selected"));
//! ```

mod logic;

use leptos::attr::any_attribute::{AnyAttribute, IntoAnyAttribute};
use leptos::attr::custom::custom_attribute;
use leptos::either::Either;
use leptos::html;
use leptos::leptos_dom::helpers::TimeoutHandle;
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use prime_checkbox::CheckboxState;
use prime_core::ids::next_unique;
use prime_core::{ClassNames, OptProp, Prop, Pt, Variant, use_prime_config};
use prime_icons::{BlankIcon, CheckIcon, SearchIcon};
use std::sync::Arc;
use std::time::Duration;

pub use logic::{
    Entry, ListboxGroup, ListboxOption, all_selected, data_p, find_option, first_focused, first_valid, last_focused, last_valid,
    matches_filter, nearest_selected, next_valid, pos_in_set, prev_valid, search_match_from, select_all, select_multiple, select_range,
    select_single, selected_index, set_size, visible_entries, write_option_classes, write_root_classes,
};

/// Сколько миллисекунд копятся символы поиска набором.
const SEARCH_RESET_MS: u64 = 500;

/// Событие изменения выбора: новое значение (одиночный режим) или значения (`multiple`) и исходное событие.
#[derive(Clone, Debug)]
pub struct ListboxChange {
    /// Новое значение в одиночном режиме (`None` — сброшено или `multiple`).
    pub value: Option<String>,
    /// Новые значения в режиме `multiple` (в одиночном — одно или ни одного).
    pub values: Vec<String>,
    /// Исходное событие.
    pub original_event: web_sys::Event,
}

/// Событие клика или двойного клика по варианту.
#[derive(Clone, Debug)]
pub struct ListboxEvent {
    /// Вариант.
    pub option: ListboxOption,
    /// Исходное событие.
    pub original_event: web_sys::Event,
}

/// Событие поиска: строка и исходное событие.
#[derive(Clone, Debug)]
pub struct ListboxFilter {
    /// Строка поиска.
    pub filter: String,
    /// Исходное событие `input`.
    pub original_event: web_sys::Event,
}

/// Событие флажка «выбрать все».
#[derive(Clone, Debug)]
pub struct ListboxSelectAll {
    /// Будут ли выбраны все варианты после клика.
    pub checked: bool,
    /// Исходное событие.
    pub original_event: web_sys::Event,
}

/// Pass-through частей Listbox.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ListboxPt {
    /// Корневой элемент.
    pub root: Pt,
    /// Шапка.
    pub header: Pt,
    /// Поле поиска.
    pub filter: Pt,
    /// Контейнер списка.
    pub list_container: Pt,
    /// Список.
    pub list: Pt,
    /// Заголовок группы.
    pub option_group: Pt,
    /// Вариант.
    pub option: Pt,
    /// Галочка выбранного варианта.
    pub option_check_icon: Pt,
    /// Пустой значок невыбранного варианта.
    pub option_blank_icon: Pt,
    /// Сообщение пустого списка.
    pub empty_message: Pt,
}

/// Список вариантов с выбором одного или нескольких значений.
///
/// Каждое свойство принимает значение или сигнал; `value` и `values` — двусторонние сигналы.
#[component]
pub fn Listbox(
    /// Варианты (без групп).
    #[prop(optional, into)]
    options: Prop<Vec<ListboxOption>>,
    /// Группы вариантов; если заданы, `options` не используется.
    #[prop(optional, into)]
    groups: Prop<Vec<ListboxGroup>>,
    /// Двусторонний сигнал значения (одиночный режим).
    #[prop(optional)]
    value: Option<RwSignal<Option<String>>>,
    /// Начальное значение, если `value` не передан.
    #[prop(optional, into)]
    default_value: OptProp<String>,
    /// Двусторонний сигнал значений (`multiple`).
    #[prop(optional)]
    values: Option<RwSignal<Vec<String>>>,
    /// Начальные значения, если `values` не передан.
    #[prop(optional)]
    default_values: Vec<String>,
    /// Выбор нескольких значений.
    #[prop(optional, into)]
    multiple: Prop<bool>,
    /// Флажки у вариантов и «выбрать все» (только с `multiple`).
    #[prop(optional, into)]
    checkbox: Prop<bool>,
    /// Поле поиска в шапке.
    #[prop(optional, into)]
    filter: Prop<bool>,
    /// Подсказка поля поиска.
    #[prop(optional, into)]
    filter_placeholder: OptProp<String>,
    /// Галочка у выбранного варианта.
    #[prop(optional, into)]
    checkmark: Prop<bool>,
    /// Чередующийся фон вариантов.
    #[prop(optional, into)]
    striped: Prop<bool>,
    /// Подсвечивать выбранные варианты (по умолчанию `true`).
    #[prop(optional, into)]
    highlight_on_select: OptProp<bool>,
    /// Выбор как в списках ОС: без `Ctrl`/`⌘` клик заменяет выбор.
    #[prop(optional, into)]
    meta_key_selection: Prop<bool>,
    /// Выбирать вариант при переносе на него фокуса стрелками.
    #[prop(optional, into)]
    select_on_focus: Prop<bool>,
    /// Фокусировать первый вариант при фокусе списка (по умолчанию `true`).
    #[prop(optional, into)]
    auto_option_focus: OptProp<bool>,
    /// Переносить фокус на вариант под указателем (по умолчанию `true`).
    #[prop(optional, into)]
    focus_on_hover: OptProp<bool>,
    /// Флажок «выбрать все» в шапке (по умолчанию `true`).
    #[prop(optional, into)]
    show_toggle_all: OptProp<bool>,
    /// Только чтение.
    #[prop(optional, into)]
    readonly: Prop<bool>,
    /// Недоступен.
    #[prop(optional, into)]
    disabled: Prop<bool>,
    /// Недопустимое значение.
    #[prop(optional, into)]
    invalid: Prop<bool>,
    /// На всю ширину.
    #[prop(optional, into)]
    fluid: OptProp<bool>,
    /// Высота списка, при превышении которой появляется прокрутка (по умолчанию `14rem`).
    #[prop(optional, into)]
    scroll_height: OptProp<String>,
    /// Атрибут `tabindex` списка (по умолчанию 0).
    #[prop(optional, into)]
    tabindex: OptProp<i32>,
    /// Атрибут `id` корня; по нему строятся идентификаторы вариантов.
    #[prop(optional, into)]
    id: OptProp<String>,
    /// Встроенный стиль корня.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Дополнительный класс корня.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Встроенный стиль списка.
    #[prop(optional, into)]
    list_style: OptProp<String>,
    /// Дополнительный класс контейнера списка.
    #[prop(optional, into)]
    list_style_class: OptProp<String>,
    /// Сообщение пустого списка.
    #[prop(optional, into)]
    empty_message: OptProp<String>,
    /// Сообщение пустого результата поиска.
    #[prop(optional, into)]
    empty_filter_message: OptProp<String>,
    /// Атрибут `aria-label` списка.
    #[prop(optional, into)]
    aria_label: OptProp<String>,
    /// Атрибут `aria-label` поля поиска.
    #[prop(optional, into)]
    aria_filter_label: OptProp<String>,
    /// Содержимое варианта; получает вариант и признак выбора.
    #[prop(optional, into)]
    option: Option<Callback<(ListboxOption, bool), AnyView>>,
    /// Содержимое заголовка группы.
    #[prop(optional, into)]
    group: Option<Callback<ListboxGroup, AnyView>>,
    /// Содержимое над шапкой списка (слот `header` PrimeNG: заголовок списка).
    #[prop(optional, into)]
    header_content: Option<ViewFn>,
    /// Вызывается после смены выбора пользователем.
    #[prop(optional, into)]
    on_change: Option<Callback<ListboxChange>>,
    /// Вызывается при клике по доступному варианту.
    #[prop(optional, into)]
    on_click: Option<Callback<ListboxEvent>>,
    /// Вызывается при двойном клике по доступному варианту.
    #[prop(optional, into)]
    on_dbl_click: Option<Callback<ListboxEvent>>,
    /// Вызывается при вводе в поле поиска.
    #[prop(optional, into)]
    on_filter: Option<Callback<ListboxFilter>>,
    /// Вызывается при получении фокуса списком.
    #[prop(optional, into)]
    on_focus: Option<Callback<web_sys::FocusEvent>>,
    /// Вызывается при потере фокуса списком.
    #[prop(optional, into)]
    on_blur: Option<Callback<web_sys::FocusEvent>>,
    /// Вызывается при клике по флажку «выбрать все».
    #[prop(optional, into)]
    on_select_all_change: Option<Callback<ListboxSelectAll>>,
    /// Pass-through частей.
    #[prop(optional)]
    pt: Option<ListboxPt>,
) -> impl IntoView {
    let pt = Arc::new(pt.unwrap_or_default());
    let locale = use_prime_config().locale;
    let fluid = prime_core::fluid::fluid_prop(fluid);
    let dom_id = id.get_untracked().unwrap_or_else(|| format!("pn_id_{}", next_unique()));
    let props = Arc::new(Props {
        options,
        groups,
        value: value.unwrap_or_else(|| RwSignal::new(default_value.get_untracked())),
        values: values.unwrap_or_else(|| RwSignal::new(default_values)),
        filter_text: RwSignal::new(String::new()),
        focused_index: RwSignal::new(None),
        start_range: RwSignal::new(None),
        focused: RwSignal::new(false),
        id: dom_id,
        multiple,
        checkbox,
        filter,
        filter_placeholder,
        checkmark,
        striped,
        highlight_on_select,
        meta_key_selection,
        select_on_focus,
        auto_option_focus,
        focus_on_hover,
        show_toggle_all,
        readonly,
        disabled,
        invalid,
        fluid,
        scroll_height,
        tabindex,
        style,
        style_class,
        list_style,
        list_style_class,
        empty_message,
        empty_filter_message,
        aria_label,
        aria_filter_label,
        option,
        group,
        on_change,
        on_click,
        on_dbl_click,
        on_filter,
        on_focus,
        on_blur,
        on_select_all_change,
        default_empty: locale.text("emptyMessage").to_owned(),
        default_empty_filter: locale.text("emptyFilterMessage").to_owned(),
        select_all_label: locale.text("aria.selectAll").to_owned(),
        unselect_all_label: locale.text("aria.unselectAll").to_owned(),
        config_variant: use_prime_config().input_variant,
        container_ref: NodeRef::new(),
        list_ref: NodeRef::new(),
        filter_ref: NodeRef::new(),
        search: StoredValue::new(String::new()),
        search_timer: StoredValue::new_local(None),
        touched: StoredValue::new(false),
    });
    {
        let props = props.clone();
        on_cleanup(move || props.release());
    }

    let root_class = {
        let (props, pt) = (props.clone(), pt.clone());
        move || {
            let mut cn = ClassNames::new();
            write_root_classes(&mut cn, props.striped.get(), props.disabled.get(), props.invalid.get(), props.fluid.get());
            props.style_class.with(|c| cn.add_user(c.as_deref()));
            cn.add_user(pt.root.extra_class());
            cn.finish_cow()
        }
    };
    let root_style = {
        let (props, pt) = (props.clone(), pt.clone());
        move || {
            let style = props.style.with(|st| pt.root.merge_style(st.as_deref().unwrap_or_default()));
            (!style.is_empty()).then_some(style)
        }
    };
    let data = {
        let props = props.clone();
        move || data_p(props.invalid.get(), props.disabled.get())
    };
    let header = {
        let (props, pt) = (props.clone(), pt.clone());
        move || props.has_header().then(|| header_view(&props, &pt))
    };
    let container_class = {
        let (props, pt) = (props.clone(), pt.clone());
        move || {
            let mut cn = ClassNames::new();
            cn.add("p-listbox-list-container");
            props.list_style_class.with(|c| cn.add_user(c.as_deref()));
            cn.add_user(pt.list_container.extra_class());
            cn.finish_cow()
        }
    };
    let container_style = {
        let props = props.clone();
        move || format!("max-height: {}", props.scroll_height.get().unwrap_or_else(|| "14rem".to_owned()))
    };
    let content = {
        let (props, pt) = (props.clone(), pt.clone());
        move || content_view(&props, &pt)
    };
    let (root_attrs, container_attrs) = (pt.root.attrs(), pt.list_container.attrs());
    let (root_id, container_ref) = (props.id.clone(), props.container_ref);

    view! {
        <div id={root_id} class={root_class} style={root_style} data-p={data} {..root_attrs}>
            {header_content.map(|h| h.run())}
            {header}
            <div node_ref={container_ref} class={container_class} style={container_style} {..container_attrs}>
                {content}
            </div>
        </div>
    }
}

/// Булев атрибут `data-p-*`: `"true"` при истине, иначе атрибута нет.
fn flag(f: impl Fn() -> bool + Send + Sync + 'static) -> impl Fn() -> Option<&'static str> + Send + Sync + 'static {
    move || f().then_some("true")
}

/// Реактивный атрибут с произвольным именем; `None` — атрибута нет.
fn reactive_attr(name: &'static str, f: impl Fn() -> Option<String> + Send + Sync + 'static) -> AnyAttribute {
    custom_attribute(name, f).into_any_attr()
}

/// Нажат ли `Ctrl`/`⌘` в событии.
fn meta_key(event: &web_sys::Event) -> bool {
    if let Some(m) = event.dyn_ref::<web_sys::MouseEvent>() {
        m.meta_key() || m.ctrl_key()
    } else if let Some(k) = event.dyn_ref::<web_sys::KeyboardEvent>() {
        k.meta_key() || k.ctrl_key()
    } else {
        false
    }
}

/// Шапка: флажок «выбрать все» и поле поиска.
fn header_view(props: &Arc<Props>, pt: &Arc<ListboxPt>) -> impl IntoView + use<> {
    let (class, attrs) = (pt.header.merge_class("p-listbox-header"), pt.header.attrs());
    let toggle = {
        let props = props.clone();
        move || {
            props.toggle_all_visible().then(|| {
                let (checked, disabled, click, label) = (props.clone(), props.clone(), props.clone(), props.clone());
                let label: Arc<dyn Fn() -> String + Send + Sync> = Arc::new(move || label.toggle_all_text());
                checkbox_view(
                    &props,
                    Arc::new(move || checked.all_selected_now()),
                    Arc::new(move || disabled.disabled.get()),
                    Some(Callback::new(move |e: web_sys::MouseEvent| click.toggle_all(e.into()))),
                    Some(label),
                    true,
                )
            })
        }
    };
    let search = {
        let (props, pt) = (props.clone(), pt.clone());
        move || props.filter.get().then(|| filter_view(&props, &pt))
    };
    view! { <div class={class} {..attrs}>{toggle}{search}</div> }
}

/// Поле поиска со значком.
fn filter_view(props: &Arc<Props>, pt: &Arc<ListboxPt>) -> impl IntoView + use<> {
    let class = pt.filter.merge_class("p-inputtext p-component p-listbox-filter");
    let attrs = pt.filter.attrs();
    let value = {
        let props = props.clone();
        move || props.filter_text.get()
    };
    let value_attr = value.clone();
    let placeholder = {
        let props = props.clone();
        move || props.filter_placeholder.get()
    };
    let aria_label = {
        let props = props.clone();
        move || props.aria_filter_label.get()
    };
    let active = {
        let props = props.clone();
        move || props.focused_index.get().map(|i| format!("{}_{}", props.id, i))
    };
    let disabled = {
        let props = props.clone();
        move || props.disabled.get().then_some("")
    };
    let tabindex = {
        let props = props.clone();
        move || if props.disabled.get() || props.focused.get() { -1 } else { props.tabindex.get().unwrap_or(0) }
    };
    let owns = format!("{}_list", props.id);
    let (input, key_down, blur) = {
        let (a, b, c) = (props.clone(), props.clone(), props.clone());
        (
            move |e: web_sys::Event| a.filter_input(e),
            move |e: web_sys::KeyboardEvent| b.key_down(e, true),
            move |_: web_sys::FocusEvent| c.filter_blur(),
        )
    };
    let filter_ref = props.filter_ref;
    view! {
        <span class="p-iconfield">
            <input
                node_ref={filter_ref}
                type="text"
                role="searchbox"
                autocomplete="off"
                class={class}
                value={value_attr}
                prop:value={value}
                placeholder={placeholder}
                aria-owns={owns}
                aria-label={aria_label}
                aria-activedescendant={active}
                disabled={disabled}
                tabindex={tabindex}
                on:input={input}
                on:keydown={key_down}
                on:blur={blur}
                {..attrs}
            />
            <span class="p-inputicon"><SearchIcon /></span>
        </span>
    }
}

/// Флажок как в `Checkbox`: разметка и классы те же, состояние берётся из замыканий. Клик по полю не меняет его сам.
fn checkbox_view(
    props: &Arc<Props>,
    checked: Arc<dyn Fn() -> bool + Send + Sync>,
    disabled: Arc<dyn Fn() -> bool + Send + Sync>,
    click: Option<Callback<web_sys::MouseEvent>>,
    aria_label: Option<Arc<dyn Fn() -> String + Send + Sync>>,
    header: bool,
) -> AnyView {
    let variant = props.config_variant;
    let state = {
        let (checked, disabled) = (checked.clone(), disabled.clone());
        move || CheckboxState { checked: checked(), disabled: disabled(), invalid: false, variant, size: None }
    };
    let root_class = {
        let state = state.clone();
        move || {
            let mut cn = ClassNames::new();
            prime_checkbox::write_classes(&mut cn, &state());
            cn.add("p-listbox-option-check-icon");
            cn.finish_cow()
        }
    };
    let data = {
        let state = state.clone();
        move || prime_checkbox::data_p(&state())
    };
    let data_box = data.clone();
    let (highlight, highlight_checked) = (
        flag({
            let c = checked.clone();
            move || c()
        }),
        flag({
            let c = checked.clone();
            move || c()
        }),
    );
    let disabled_attr = {
        let disabled = disabled.clone();
        move || disabled().then_some("true")
    };
    let is_checked = {
        let checked = checked.clone();
        move || checked()
    };
    let is_checked_attr = is_checked.clone();
    let is_disabled = {
        let disabled = disabled.clone();
        move || disabled()
    };
    let icon = {
        let checked = checked.clone();
        move || checked().then(|| view! { <CheckIcon class="p-checkbox-icon".to_owned() /> })
    };
    let on_click = move |e: web_sys::MouseEvent| {
        e.prevent_default();
        if let Some(cb) = click {
            cb.run(e);
        }
    };
    let on_key = {
        let props = props.clone();
        move |e: web_sys::KeyboardEvent| {
            if header {
                props.header_checkbox_key(e);
            }
        }
    };
    view! {
        <div class={root_class} data-p-highlight={highlight} data-p-checked={highlight_checked} data-p-disabled={disabled_attr} data-p={data}>
            <input
                type="checkbox"
                class="p-checkbox-input"
                tabindex="-1"
                aria-label={aria_label.map(|f| move || f())}
                checked={is_checked_attr}
                prop:checked={is_checked}
                disabled={move || is_disabled().then_some("")}
                on:click={on_click}
                on:keydown={on_key}
            />
            <div class="p-checkbox-box" data-p={data_box}>{icon}</div>
        </div>
    }
    .into_any()
}

/// Содержимое контейнера: сообщение пустого списка или список. Читает только варианты, группы и строку поиска.
fn content_view(props: &Arc<Props>, pt: &Arc<ListboxPt>) -> impl IntoView + use<> {
    let entries = props.entries();
    if entries.is_empty() {
        let (class, attrs) = (pt.empty_message.merge_class("p-listbox-empty-message"), pt.empty_message.attrs());
        let text = props.empty_text();
        return Either::Left(view! { <div class={class} {..attrs}>{text}</div> });
    }
    let size = set_size(&entries);
    let views: Vec<AnyView> = entries
        .iter()
        .enumerate()
        .map(|(index, entry)| match entry {
            Entry::Group { index: group, label } => group_view(props, pt, index, *group, label),
            Entry::Option(option) => option_view(props, pt, index, option, size, pos_in_set(&entries, index)),
        })
        .collect();
    Either::Right(list_view(props, pt, views))
}

/// Список `ul` с вариантами.
fn list_view(props: &Arc<Props>, pt: &Arc<ListboxPt>, items: Vec<AnyView>) -> impl IntoView + use<> {
    let class = pt.list.merge_class("p-listbox-list");
    let attrs = pt.list.attrs();
    let style = {
        let props = props.clone();
        move || props.list_style.get().filter(|s| !s.is_empty())
    };
    let list_attrs: Vec<AnyAttribute> = {
        macro_rules! attr {
            ($name:literal, |$p:ident| $body:expr) => {{
                let $p = props.clone();
                reactive_attr($name, move || $body)
            }};
        }
        vec![
            attr!("tabindex", |p| Some(if p.disabled.get() { -1 } else { p.tabindex.get().unwrap_or(0) }.to_string())),
            attr!("aria-multiselectable", |p| p.multiple.get().then(|| "true".to_owned())),
            attr!("aria-disabled", |p| Some(p.disabled.get().to_string())),
            attr!("aria-label", |p| p.aria_label.get()),
            attr!("aria-activedescendant", |p| {
                if p.focused.get() { p.focused_index.get().map(|i| format!("{}_{}", p.id, i)) } else { None }
            }),
        ]
    };
    let (focus, blur, key_down) = {
        let (a, b, c) = (props.clone(), props.clone(), props.clone());
        (
            move |e: web_sys::FocusEvent| a.list_focus(e),
            move |e: web_sys::FocusEvent| b.list_blur(e),
            move |e: web_sys::KeyboardEvent| c.key_down(e, false),
        )
    };
    let list_id = format!("{}_list", props.id);
    let list_ref = props.list_ref;
    view! {
        <ul
            node_ref={list_ref}
            id={list_id}
            class={class}
            style={style}
            role="listbox"
            on:focus={focus}
            on:blur={blur}
            on:keydown={key_down}
            {..list_attrs}
            {..attrs}
        >
            {items}
        </ul>
    }
}

/// Заголовок группы.
fn group_view(props: &Arc<Props>, pt: &Arc<ListboxPt>, index: usize, group: usize, label: &str) -> AnyView {
    let id = format!("{}_{}", props.id, index);
    let (class, attrs) = (pt.option_group.merge_class("p-listbox-option-group"), pt.option_group.attrs());
    let content = match (props.group.as_ref(), props.groups.get_untracked().get(group)) {
        (Some(template), Some(g)) => template.run(g.clone()),
        _ => label.to_owned().into_any(),
    };
    view! { <li id={id} class={class} role="option" {..attrs}>{content}</li> }.into_any()
}

/// Вариант.
fn option_view(props: &Arc<Props>, pt: &Arc<ListboxPt>, index: usize, option: &ListboxOption, size: usize, pos: usize) -> AnyView {
    let id = format!("{}_{}", props.id, index);
    let value = option.value.clone();
    let selected = {
        let (props, value) = (props.clone(), value.clone());
        move || props.is_selected(&value)
    };
    let focused = {
        let props = props.clone();
        move || props.focused_index.get() == Some(index)
    };
    let disabled = option.disabled;
    let class = {
        let (props, pt, selected, focused) = (props.clone(), pt.clone(), selected.clone(), focused.clone());
        move || {
            let mut cn = ClassNames::new();
            write_option_classes(&mut cn, selected() && props.highlight_on_select.get().unwrap_or(true), focused(), disabled);
            cn.add_user(pt.option.extra_class());
            cn.finish_cow()
        }
    };
    let selected_attr = flag(selected.clone());
    let aria_selected = {
        let selected = selected.clone();
        move || selected().to_string()
    };
    let focused_attr = flag(focused);
    let disabled_attr = disabled.then_some("true");
    let aria_disabled = disabled.to_string();
    let content = {
        let (props, pt, option, selected) = (props.clone(), pt.clone(), option.clone(), selected.clone());
        move || option_content(&props, &pt, &option, selected())
    };
    let (click, dbl, down, enter, touch) = {
        let (a, b, c, d, f, option) = (props.clone(), props.clone(), props.clone(), props.clone(), props.clone(), option.clone());
        let option_dbl = option.clone();
        (
            move |e: web_sys::MouseEvent| a.select_option(&option, e.into(), Some(index)),
            move |e: web_sys::MouseEvent| b.option_dbl_click(&option_dbl, e.into()),
            move |_: web_sys::MouseEvent| c.set_focused(Some(index)),
            move |_: web_sys::MouseEvent| d.option_mouse_enter(index),
            move |_: web_sys::TouchEvent| f.touched.set_value(true),
        )
    };
    let attrs = pt.option.attrs();
    let (label, size, pos) = (option.label.clone(), size.to_string(), pos.to_string());
    view! {
        <li
            id={id}
            class={class}
            role="option"
            aria-label={label}
            aria-selected={aria_selected}
            aria-disabled={aria_disabled}
            aria-setsize={size}
            aria-posinset={pos}
            data-p-selected={selected_attr}
            data-p-focused={focused_attr}
            data-p-disabled={disabled_attr}
            on:click={click}
            on:dblclick={dbl}
            on:mousedown={down}
            on:mouseenter={enter}
            on:touchend={touch}
            {..attrs}
        >
            {content}
        </li>
    }
    .into_any()
}

/// Содержимое варианта: флажок или отметка, затем метка или шаблон.
fn option_content(props: &Arc<Props>, pt: &Arc<ListboxPt>, option: &ListboxOption, selected: bool) -> AnyView {
    let checkbox = (props.checkbox.get() && props.multiple.get()).then(|| {
        let (checked, disabled) = (props.clone(), props.clone());
        let value = option.value.clone();
        let own_disabled = option.disabled;
        checkbox_view(
            props,
            Arc::new(move || checked.is_selected(&value)),
            Arc::new(move || disabled.disabled.get() || own_disabled),
            None,
            None,
            false,
        )
    });
    let mark = props.checkmark.get().then(|| {
        if selected {
            let (class, attrs) = (pt.option_check_icon.merge_class("p-listbox-option-check-icon"), pt.option_check_icon.attrs());
            view! { <CheckIcon class={class.into_owned()} {..attrs} /> }.into_any()
        } else {
            let (class, attrs) = (pt.option_blank_icon.merge_class("p-listbox-option-blank-icon"), pt.option_blank_icon.attrs());
            view! { <BlankIcon class={class.into_owned()} {..attrs} /> }.into_any()
        }
    });
    let body = match props.option {
        Some(template) => template.run((option.clone(), selected)),
        None => view! { <span>{option.label.clone()}</span> }.into_any(),
    };
    view! { {checkbox}{mark}{body} }.into_any()
}

/// Свойства и состояние экземпляра.
struct Props {
    options: Prop<Vec<ListboxOption>>,
    groups: Prop<Vec<ListboxGroup>>,
    value: RwSignal<Option<String>>,
    values: RwSignal<Vec<String>>,
    /// Строка поиска.
    filter_text: RwSignal<String>,
    /// Индекс элемента с фокусом в плоском списке.
    focused_index: RwSignal<Option<usize>>,
    /// Опорный индекс диапазона (`startRangeIndex`).
    start_range: RwSignal<Option<usize>>,
    /// Список в фокусе.
    focused: RwSignal<bool>,
    id: String,
    multiple: Prop<bool>,
    checkbox: Prop<bool>,
    filter: Prop<bool>,
    filter_placeholder: OptProp<String>,
    checkmark: Prop<bool>,
    striped: Prop<bool>,
    highlight_on_select: OptProp<bool>,
    meta_key_selection: Prop<bool>,
    select_on_focus: Prop<bool>,
    auto_option_focus: OptProp<bool>,
    focus_on_hover: OptProp<bool>,
    show_toggle_all: OptProp<bool>,
    readonly: Prop<bool>,
    disabled: Prop<bool>,
    invalid: Prop<bool>,
    fluid: Prop<bool>,
    scroll_height: OptProp<String>,
    tabindex: OptProp<i32>,
    style: OptProp<String>,
    style_class: OptProp<String>,
    list_style: OptProp<String>,
    list_style_class: OptProp<String>,
    empty_message: OptProp<String>,
    empty_filter_message: OptProp<String>,
    aria_label: OptProp<String>,
    aria_filter_label: OptProp<String>,
    option: Option<Callback<(ListboxOption, bool), AnyView>>,
    group: Option<Callback<ListboxGroup, AnyView>>,
    on_change: Option<Callback<ListboxChange>>,
    on_click: Option<Callback<ListboxEvent>>,
    on_dbl_click: Option<Callback<ListboxEvent>>,
    on_filter: Option<Callback<ListboxFilter>>,
    on_focus: Option<Callback<web_sys::FocusEvent>>,
    on_blur: Option<Callback<web_sys::FocusEvent>>,
    on_select_all_change: Option<Callback<ListboxSelectAll>>,
    default_empty: String,
    default_empty_filter: String,
    select_all_label: String,
    unselect_all_label: String,
    config_variant: Option<Variant>,
    container_ref: NodeRef<html::Div>,
    list_ref: NodeRef<html::Ul>,
    filter_ref: NodeRef<html::Input>,
    /// Накопленные символы поиска набором.
    search: StoredValue<String>,
    search_timer: StoredValue<Option<TimeoutHandle>, LocalStorage>,
    /// Последний выбор был после касания: мета-режим отключён (`optionTouched`).
    touched: StoredValue<bool>,
}

impl Props {
    /// Плоский список видимых элементов (с подпиской).
    fn entries(&self) -> Vec<Entry> {
        visible_entries(&self.options.get(), &self.groups.get(), &self.filter_text.get())
    }

    fn entries_untracked(&self) -> Vec<Entry> {
        untrack(|| self.entries())
    }

    /// Выбранные значения (с подпиской).
    fn selected(&self) -> Vec<String> {
        if self.multiple.get() { self.values.get() } else { self.value.get().into_iter().collect() }
    }

    fn selected_untracked(&self) -> Vec<String> {
        untrack(|| self.selected())
    }

    /// Выбрано ли значение (с подпиской).
    fn is_selected(&self, value: &str) -> bool {
        if self.multiple.get() {
            self.values.with(|vs| vs.iter().any(|v| v == value))
        } else {
            self.value.with(|v| v.as_deref() == Some(value))
        }
    }

    /// Выбраны ли все видимые варианты (с подпиской).
    fn all_selected_now(&self) -> bool {
        all_selected(&self.entries(), &self.selected())
    }

    fn has_header(&self) -> bool {
        self.toggle_all_visible() || self.filter.get()
    }

    /// Показывать ли флажок «выбрать все».
    fn toggle_all_visible(&self) -> bool {
        self.checkbox.get() && self.multiple.get() && self.show_toggle_all.get().unwrap_or(true)
    }

    fn empty_text(&self) -> String {
        if self.filter_text.get().trim().is_empty() {
            self.empty_message.get().filter(|m| !m.is_empty()).unwrap_or_else(|| self.default_empty.clone())
        } else {
            self.empty_filter_message.get().filter(|m| !m.is_empty()).unwrap_or_else(|| self.default_empty_filter.clone())
        }
    }

    /// Подпись флажка «выбрать все» для чтения с экрана.
    fn toggle_all_text(&self) -> String {
        if self.all_selected_now() { self.select_all_label.clone() } else { self.unselect_all_label.clone() }
    }

    /// Снимает таймер поиска набором.
    fn release(&self) {
        if let Some(handle) = self.search_timer.try_update_value(Option::take).flatten() {
            handle.clear();
        }
    }

    fn option_id(&self, index: usize) -> String {
        format!("{}_{}", self.id, index)
    }

    // ---- выбор ----

    /// `updateModel`: записывает значения в модель и вызывает `on_change`.
    fn update_model(&self, values: Vec<String>, event: web_sys::Event) {
        let value = if self.multiple.get_untracked() {
            self.values.set(values.clone());
            None
        } else {
            let value = values.first().cloned();
            self.value.set(value.clone());
            value
        };
        if let Some(cb) = self.on_change {
            cb.run(ListboxChange { value, values, original_event: event });
        }
    }

    /// `onOptionSelect`: клик, Enter или Space по варианту.
    fn select_option(&self, option: &ListboxOption, event: web_sys::Event, index: Option<usize>) {
        if self.disabled.get_untracked() || option.disabled || self.readonly.get_untracked() {
            return;
        }
        if let Some(cb) = self.on_click {
            cb.run(ListboxEvent { option: option.clone(), original_event: event.clone() });
        }
        let meta = self.meta_key_selection.get_untracked() && !self.touched.get_value();
        let meta_key = meta_key(&event);
        if self.multiple.get_untracked() {
            let new = select_multiple(&self.selected_untracked(), &option.value, meta, meta_key);
            self.update_model(new, event);
        } else if let Some(new) = select_single(self.value.get_untracked().as_deref(), &option.value, meta, meta_key) {
            self.update_model(new.into_iter().collect(), event);
        }
        self.touched.set_value(false);
        if index.is_some() {
            self.focused_index.set(index);
        }
    }

    fn option_dbl_click(&self, option: &ListboxOption, event: web_sys::Event) {
        if self.disabled.get_untracked() || option.disabled || self.readonly.get_untracked() {
            return;
        }
        if let Some(cb) = self.on_dbl_click {
            cb.run(ListboxEvent { option: option.clone(), original_event: event });
        }
    }

    /// `onOptionSelectRange`: выбор диапазона между опорным индексом и фокусным.
    fn select_range_event(&self, event: &web_sys::Event, start: Option<usize>, end: Option<usize>) {
        let entries = self.entries_untracked();
        let selected = self.selected_untracked();
        let (start, end) = match (start, end) {
            (Some(s), Some(e)) => (s, e),
            (None, Some(e)) => (nearest_selected(&entries, &selected, e, true), e),
            (Some(s), None) => (s, nearest_selected(&entries, &selected, s, false)),
            (None, None) => return,
        };
        self.update_model(select_range(&entries, start, end), event.clone());
    }

    /// `onToggleAll`: клик по флажку «выбрать все».
    fn toggle_all(&self, event: web_sys::Event) {
        if self.disabled.get_untracked() || self.readonly.get_untracked() {
            return;
        }
        let entries = self.entries_untracked();
        let all = all_selected(&entries, &self.selected_untracked());
        if let Some(cb) = self.on_select_all_change {
            cb.run(ListboxSelectAll { checked: !all, original_event: event.clone() });
        }
        self.update_model(if all { Vec::new() } else { select_all(&entries) }, event);
    }

    /// `onHeaderCheckboxKeyDown`: Enter переключает, Tab уводит фокус в список.
    fn header_checkbox_key(&self, e: web_sys::KeyboardEvent) {
        if self.disabled.get_untracked() {
            e.prevent_default();
            return;
        }
        match e.code().as_str() {
            "Enter" | "NumpadEnter" => {
                self.toggle_all(e.into());
            }
            "Tab" => {
                if let Some(list) = self.list_ref.get_untracked() {
                    let _ = list.unchecked_into::<web_sys::HtmlElement>().focus();
                }
                e.prevent_default();
            }
            _ => {}
        }
    }

    // ---- фокус ----

    /// Переносит подсветку на элемент без выбора и без прокрутки (мышь).
    fn set_focused(&self, index: Option<usize>) {
        if self.disabled.get_untracked() {
            return;
        }
        self.focused_index.set(index);
    }

    fn option_mouse_enter(&self, index: usize) {
        if self.focus_on_hover.get_untracked().unwrap_or(true) && self.focused.get_untracked() {
            self.set_focused(Some(index));
        }
    }

    fn scroll_to(&self, index: usize) {
        if let Some(el) = leptos::prelude::document().get_element_by_id(&self.option_id(index)) {
            let options = web_sys::ScrollIntoViewOptions::new();
            options.set_block(web_sys::ScrollLogicalPosition::Nearest);
            options.set_inline(web_sys::ScrollLogicalPosition::Nearest);
            el.scroll_into_view_with_scroll_into_view_options(&options);
        }
    }

    /// `changeFocusedOptionIndex`: перенос фокуса клавишами; при `select_on_focus` — с выбором.
    fn change_focused(&self, event: &web_sys::Event, index: Option<usize>, shift: bool) {
        if self.focused_index.get_untracked() == index {
            return;
        }
        self.focused_index.set(index);
        if let Some(i) = index {
            self.scroll_to(i);
            let range = shift && self.multiple.get_untracked();
            if self.select_on_focus.get_untracked()
                && !range
                && let Some(Entry::Option(option)) = self.entries_untracked().get(i)
            {
                self.select_option(&option.clone(), event.clone(), None);
            }
        }
    }

    fn list_focus(&self, e: web_sys::FocusEvent) {
        if self.disabled.get_untracked() {
            return;
        }
        self.focused.set(true);
        let index = match self.focused_index.get_untracked() {
            Some(i) => Some(i),
            None => {
                let (entries, selected) = (self.entries_untracked(), self.selected_untracked());
                if self.auto_option_focus.get_untracked().unwrap_or(true) {
                    first_focused(&entries, &selected)
                } else {
                    selected_index(&entries, &selected)
                }
            }
        };
        self.focused_index.set(index);
        if let Some(i) = index {
            self.scroll_to(i);
        }
        if let Some(cb) = self.on_focus {
            cb.run(e);
        }
    }

    fn list_blur(&self, e: web_sys::FocusEvent) {
        self.focused.set(false);
        self.focused_index.set(None);
        self.start_range.set(None);
        self.search.set_value(String::new());
        if let Some(cb) = self.on_blur {
            cb.run(e);
        }
    }

    fn filter_blur(&self) {
        self.focused_index.set(None);
        self.start_range.set(None);
    }

    // ---- поиск ----

    fn filter_input(&self, e: web_sys::Event) {
        let Some(input) = self.filter_ref.get_untracked() else { return };
        let text = input.value();
        self.filter_text.set(text.clone());
        self.focused_index.set(None);
        self.start_range.set(None);
        if let Some(cb) = self.on_filter {
            cb.run(ListboxFilter { filter: text, original_event: e });
        }
    }

    /// Поиск набором (`searchOptions`): символы копятся 500 мс.
    fn search_by_typing(self: &Arc<Self>, event: &web_sys::Event, key: &str) {
        let typed = {
            let mut typed = self.search.get_value();
            typed.push_str(key);
            self.search.set_value(typed.clone());
            typed
        };
        let entries = self.entries_untracked();
        let focused = self.focused_index.get_untracked();
        let mut index = search_match_from(&entries, &typed, focused);
        if index.is_none() && focused.is_none() {
            index = first_focused(&entries, &self.selected_untracked());
        }
        if index.is_some() {
            self.change_focused(event, index, false);
        }
        if let Some(handle) = self.search_timer.try_update_value(Option::take).flatten() {
            handle.clear();
        }
        let this = self.clone();
        let handle = set_timeout_with_handle(
            move || {
                if !this.focused.is_disposed() {
                    this.search.set_value(String::new());
                }
            },
            Duration::from_millis(SEARCH_RESET_MS),
        )
        .ok();
        self.search_timer.set_value(handle);
    }

    // ---- клавиатура ----

    /// `onListKeyDown` списка и `onFilterKeyDown` поля поиска (`in_filter`).
    fn key_down(self: &Arc<Self>, e: web_sys::KeyboardEvent, in_filter: bool) {
        if self.disabled.get_untracked() {
            return;
        }
        let event: web_sys::Event = e.clone().into();
        let meta = e.meta_key() || e.ctrl_key();
        match e.code().as_str() {
            "ArrowDown" => self.arrow_down(&e),
            "ArrowUp" => self.arrow_up(&e),
            "ArrowLeft" | "ArrowRight" if in_filter => self.focused_index.set(None),
            "Home" => self.home_end(&e, in_filter, true),
            "End" => self.home_end(&e, in_filter, false),
            "PageDown" if !in_filter => self.page(&e, true),
            "PageUp" if !in_filter => self.page(&e, false),
            "Enter" | "NumpadEnter" => self.enter_key(&e),
            "Space" if !in_filter => self.enter_key(&e),
            "ShiftLeft" | "ShiftRight" => self.start_range.set(self.focused_index.get_untracked()),
            _ if in_filter => {}
            "Tab" => {}
            "KeyA" if self.multiple.get_untracked() && meta => {
                self.update_model(select_all(&self.entries_untracked()), event);
                e.prevent_default();
            }
            _ => {
                let key = e.key();
                if !meta && key.chars().count() == 1 {
                    self.search_by_typing(&event, &key);
                    e.prevent_default();
                }
            }
        }
    }

    fn arrow_down(&self, e: &web_sys::KeyboardEvent) {
        let entries = self.entries_untracked();
        let index = match self.focused_index.get_untracked() {
            Some(i) => Some(next_valid(&entries, i)),
            None => first_focused(&entries, &self.selected_untracked()),
        };
        let event: web_sys::Event = e.clone().into();
        if self.multiple.get_untracked() && e.shift_key() {
            self.select_range_event(&event, self.start_range.get_untracked(), index);
        }
        self.change_focused(&event, index, e.shift_key());
        e.prevent_default();
    }

    fn arrow_up(&self, e: &web_sys::KeyboardEvent) {
        let entries = self.entries_untracked();
        let index = match self.focused_index.get_untracked() {
            Some(i) => Some(prev_valid(&entries, i)),
            None => last_focused(&entries, &self.selected_untracked()),
        };
        let event: web_sys::Event = e.clone().into();
        if self.multiple.get_untracked() && e.shift_key() {
            self.select_range_event(&event, index, self.start_range.get_untracked());
        }
        self.change_focused(&event, index, e.shift_key());
        e.prevent_default();
    }

    /// `Home` и `End`: в поле поиска двигают курсор, иначе фокус на первый или последний вариант.
    fn home_end(&self, e: &web_sys::KeyboardEvent, in_filter: bool, home: bool) {
        if in_filter {
            if let Some(input) = self.filter_ref.get_untracked() {
                let len = input.value().chars().count() as u32;
                let caret = if home { 0 } else { len };
                let _ = input.set_selection_range(caret, caret);
            }
            self.focused_index.set(None);
        } else {
            let entries = self.entries_untracked();
            let index = if home { first_valid(&entries) } else { last_valid(&entries) };
            let event: web_sys::Event = e.clone().into();
            if self.multiple.get_untracked() && e.shift_key() && (e.ctrl_key() || e.meta_key()) {
                if home {
                    self.select_range_event(&event, index, self.start_range.get_untracked());
                } else {
                    self.select_range_event(&event, self.start_range.get_untracked(), index);
                }
            }
            self.change_focused(&event, index, e.shift_key());
        }
        e.prevent_default();
    }

    /// `PageDown`/`PageUp`: прокрутка контейнера на высоту видимой области.
    fn page(&self, e: &web_sys::KeyboardEvent, down: bool) {
        if let Some(container) = self.container_ref.get_untracked() {
            let el: web_sys::Element = container.unchecked_into();
            let step = el.client_height();
            el.set_scroll_top(el.scroll_top() + if down { step } else { -step });
        }
        e.prevent_default();
    }

    fn enter_key(&self, e: &web_sys::KeyboardEvent) {
        if let Some(i) = self.focused_index.get_untracked() {
            let event: web_sys::Event = e.clone().into();
            if self.multiple.get_untracked() && e.shift_key() {
                self.select_range_event(&event, Some(i), None);
            } else if let Some(Entry::Option(option)) = self.entries_untracked().get(i).cloned() {
                self.select_option(&option, event, None);
            }
        }
        e.prevent_default();
    }
}
