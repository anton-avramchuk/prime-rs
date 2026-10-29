//! Select — порт `p-select` из PrimeNG 21.1.10.
//!
//! Разметка и классы как в PrimeNG: корень `div.p-select`, метка `span.p-select-label[role=combobox]`, значок очистки, кнопка
//! `div.p-select-dropdown` и панель `div.p-select-overlay` с полем поиска, списком вариантов (`ul.p-select-list`, группы,
//! пустое сообщение) и скрытыми фокусируемыми границами. Панель лежит внутри корня и позиционируется через `prime-dom`.
//!
//! Значение — двусторонний сигнал `value: RwSignal<Option<String>>`, открытость панели — `open: RwSignal<bool>`; без них
//! компонент ведёт внутреннее состояние. Варианты задаются списком `options` или группами `groups`.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_select::{Select, SelectOption};
//!
//! let value = RwSignal::new(Some("LDN".to_owned()));
//! let options = vec![SelectOption::new("New York", "NY"), SelectOption::new("London", "LDN")];
//! let html = view! { <Select options=options value=value placeholder="Select a City" /> }.to_html();
//! assert!(html.contains("p-select-label") && html.contains("London") && !html.contains("p-placeholder"));
//! ```

mod options;
mod state;

use leptos::attr::any_attribute::{AnyAttribute, IntoAnyAttribute};
use leptos::attr::custom::custom_attribute;
use leptos::either::Either;
use leptos::ev;
use leptos::html;
use leptos::leptos_dom::helpers::{TimeoutHandle, WindowListenerHandle};
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use prime_core::ids::next_unique;
use prime_core::{ClassNames, OptProp, Prop, Pt, Size, Variant, ZGroup, use_prime_config};
use prime_dom::dom::{AlignMode, align_overlay};
use prime_icons::{BlankIcon, CheckIcon, ChevronDownIcon, SearchIcon, SpinnerIcon, TimesIcon};
use std::sync::Arc;
use std::time::Duration;

pub use options::{
    Entry, SelectGroup, SelectOption, find_option, first_focused, first_valid, last_focused, last_valid, matches_filter, next_valid,
    pos_in_set, prev_valid, search_match, selected_index, set_size, visible_entries,
};
pub use state::{LabelKind, SelectState, data_p, label_data_p, write_classes, write_label_classes, write_option_classes};

/// Сколько миллисекунд копятся символы поиска набором.
const SEARCH_RESET_MS: u64 = 500;

/// Событие изменения значения: новое значение и исходное событие.
#[derive(Clone, Debug)]
pub struct SelectChange {
    /// Новое значение (`None` — очищено).
    pub value: Option<String>,
    /// Исходное событие.
    pub original_event: web_sys::Event,
}

/// Событие поиска: строка и исходное событие.
#[derive(Clone, Debug)]
pub struct SelectFilter {
    /// Строка поиска.
    pub filter: String,
    /// Исходное событие `input`.
    pub original_event: web_sys::Event,
}

/// Pass-through частей Select.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SelectPt {
    /// Корневой элемент.
    pub root: Pt,
    /// Метка.
    pub label: Pt,
    /// Значок очистки.
    pub clear_icon: Pt,
    /// Кнопка раскрытия.
    pub dropdown: Pt,
    /// Значок кнопки раскрытия.
    pub dropdown_icon: Pt,
    /// Значок загрузки.
    pub loading_icon: Pt,
    /// Панель.
    pub overlay: Pt,
    /// Шапка панели с полем поиска.
    pub header: Pt,
    /// Поле поиска.
    pub filter: Pt,
    /// Контейнер списка.
    pub list_container: Pt,
    /// Список.
    pub list: Pt,
    /// Заголовок группы.
    pub option_group: Pt,
    /// Текст заголовка группы.
    pub option_group_label: Pt,
    /// Вариант.
    pub option: Pt,
    /// Метка варианта.
    pub option_label: Pt,
    /// Галочка выбранного варианта.
    pub option_check_icon: Pt,
    /// Пустой значок невыбранного варианта.
    pub option_blank_icon: Pt,
    /// Сообщение пустого списка.
    pub empty_message: Pt,
}

/// Выбор значения из списка в выпадающей панели.
///
/// Каждое свойство принимает значение или сигнал; `value` и `open` — двусторонние сигналы.
#[component]
pub fn Select(
    /// Варианты (без групп).
    #[prop(optional, into)]
    options: Prop<Vec<SelectOption>>,
    /// Группы вариантов; если заданы, `options` не используется.
    #[prop(optional, into)]
    groups: Prop<Vec<SelectGroup>>,
    /// Двусторонний сигнал значения.
    #[prop(optional)]
    value: Option<RwSignal<Option<String>>>,
    /// Начальное значение, если `value` не передан.
    #[prop(optional, into)]
    default_value: OptProp<String>,
    /// Двусторонний сигнал открытости панели.
    #[prop(optional)]
    open: Option<RwSignal<bool>>,
    /// Подсказка при отсутствии выбора.
    #[prop(optional, into)]
    placeholder: OptProp<String>,
    /// Показывать значок очистки.
    #[prop(optional, into)]
    show_clear: Prop<bool>,
    /// Галочка у выбранного варианта.
    #[prop(optional, into)]
    checkmark: Prop<bool>,
    /// Поле поиска в панели.
    #[prop(optional, into)]
    filter: Prop<bool>,
    /// Подсказка поля поиска.
    #[prop(optional, into)]
    filter_placeholder: OptProp<String>,
    /// Сбрасывать строку поиска при закрытии.
    #[prop(optional, into)]
    reset_filter_on_hide: Prop<bool>,
    /// Ставить фокус в поле поиска при открытии (по умолчанию `true`).
    #[prop(optional, into)]
    autofocus_filter: OptProp<bool>,
    /// Сообщение пустого списка.
    #[prop(optional, into)]
    empty_message: OptProp<String>,
    /// Сообщение пустого результата поиска.
    #[prop(optional, into)]
    empty_filter_message: OptProp<String>,
    /// Идёт загрузка: вместо стрелки значок загрузки, ввод заблокирован.
    #[prop(optional, into)]
    loading: Prop<bool>,
    /// Переносить фокус на вариант под указателем (по умолчанию `true`).
    #[prop(optional, into)]
    focus_on_hover: OptProp<bool>,
    /// Выбирать вариант при переносе на него фокуса клавишами.
    #[prop(optional, into)]
    select_on_focus: Prop<bool>,
    /// Фокусировать вариант при открытии панели.
    #[prop(optional, into)]
    auto_option_focus: Prop<bool>,
    /// Высота списка, при превышении которой появляется прокрутка (по умолчанию `200px`).
    #[prop(optional, into)]
    scroll_height: OptProp<String>,
    /// Недоступен.
    #[prop(optional, into)]
    disabled: Prop<bool>,
    /// Только чтение.
    #[prop(optional, into)]
    readonly: Prop<bool>,
    /// Недопустимое значение.
    #[prop(optional, into)]
    invalid: Prop<bool>,
    /// Обязателен.
    #[prop(optional, into)]
    required: Prop<bool>,
    /// Вариант оформления.
    #[prop(optional, into)]
    variant: OptProp<Variant>,
    /// Размер.
    #[prop(optional, into)]
    size: OptProp<Size>,
    /// На всю ширину.
    #[prop(optional, into)]
    fluid: Prop<bool>,
    /// Атрибут `id` метки.
    #[prop(optional, into)]
    input_id: OptProp<String>,
    /// Атрибут `tabindex` метки (по умолчанию 0).
    #[prop(optional, into)]
    tabindex: OptProp<i32>,
    /// Атрибут `aria-label` метки.
    #[prop(optional, into)]
    aria_label: OptProp<String>,
    /// Атрибут `aria-labelledby` метки.
    #[prop(optional, into)]
    aria_labelledby: OptProp<String>,
    /// Атрибут `aria-label` поля поиска.
    #[prop(optional, into)]
    aria_filter_label: OptProp<String>,
    /// Встроенный стиль корня.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Дополнительный класс корня.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Встроенный стиль панели.
    #[prop(optional, into)]
    panel_style: OptProp<String>,
    /// Дополнительный класс панели.
    #[prop(optional, into)]
    panel_style_class: OptProp<String>,
    /// Значок кнопки раскрытия вместо стрелки.
    #[prop(optional, into)]
    dropdown_icon: Option<ViewFn>,
    /// Значок очистки вместо крестика.
    #[prop(optional, into)]
    clear_icon: Option<ViewFn>,
    /// Значок загрузки вместо спиннера.
    #[prop(optional, into)]
    loading_icon: Option<ViewFn>,
    /// Содержимое варианта; получает вариант.
    #[prop(optional, into)]
    item_template: Option<Callback<SelectOption, AnyView>>,
    /// Содержимое метки для выбранного варианта.
    #[prop(optional, into)]
    selected_item_template: Option<Callback<SelectOption, AnyView>>,
    /// Содержимое заголовка группы.
    #[prop(optional, into)]
    group_template: Option<Callback<SelectGroup, AnyView>>,
    /// Вызывается после смены значения пользователем.
    #[prop(optional, into)]
    on_change: Option<Callback<SelectChange>>,
    /// Вызывается при вводе в поле поиска.
    #[prop(optional, into)]
    on_filter: Option<Callback<SelectFilter>>,
    /// Вызывается при получении фокуса меткой.
    #[prop(optional, into)]
    on_focus: Option<Callback<web_sys::FocusEvent>>,
    /// Вызывается при потере фокуса меткой.
    #[prop(optional, into)]
    on_blur: Option<Callback<web_sys::FocusEvent>>,
    /// Вызывается при клике по полю.
    #[prop(optional, into)]
    on_click: Option<Callback<web_sys::MouseEvent>>,
    /// Вызывается после открытия панели.
    #[prop(optional, into)]
    on_show: Option<Callback<()>>,
    /// Вызывается после закрытия панели.
    #[prop(optional, into)]
    on_hide: Option<Callback<()>>,
    /// Вызывается при очистке.
    #[prop(optional, into)]
    on_clear: Option<Callback<web_sys::Event>>,
    /// Pass-through частей.
    #[prop(optional)]
    pt: Option<SelectPt>,
) -> impl IntoView {
    let pt = Arc::new(pt.unwrap_or_default());
    let locale = use_prime_config().locale;
    let props = Arc::new(Props {
        options,
        groups,
        value: value.unwrap_or_else(|| RwSignal::new(default_value.get_untracked())),
        open: open.unwrap_or_else(|| RwSignal::new(false)),
        filter_text: RwSignal::new(String::new()),
        focused_index: RwSignal::new(None),
        focused: RwSignal::new(false),
        clicked: RwSignal::new(false),
        id: format!("pn_id_{}", next_unique()),
        placeholder,
        show_clear,
        checkmark,
        filter,
        filter_placeholder,
        reset_filter_on_hide,
        autofocus_filter,
        empty_message,
        empty_filter_message,
        loading,
        focus_on_hover,
        select_on_focus,
        auto_option_focus,
        scroll_height,
        disabled,
        readonly,
        invalid,
        required,
        variant,
        config_variant: use_prime_config().input_variant,
        size,
        fluid,
        input_id,
        tabindex,
        aria_label,
        aria_labelledby,
        aria_filter_label,
        style,
        style_class,
        panel_style,
        panel_style_class,
        dropdown_icon,
        clear_icon,
        loading_icon,
        item_template,
        selected_item_template,
        group_template,
        on_change,
        on_filter,
        on_focus,
        on_blur,
        on_click,
        on_show,
        on_hide,
        on_clear,
        default_empty: locale.text("emptyMessage").to_owned(),
        default_empty_filter: locale.text("emptyFilterMessage").to_owned(),
        list_label: locale.text("aria.listLabel").to_owned(),
        root_ref: NodeRef::new(),
        label_ref: NodeRef::new(),
        overlay_ref: NodeRef::new(),
        filter_ref: NodeRef::new(),
        search: StoredValue::new(String::new()),
        search_timer: StoredValue::new_local(None),
        listeners: StoredValue::new_local(Vec::new()),
        z_index: StoredValue::new(None),
        was_open: StoredValue::new(false),
    });
    // Подписки на `window`, таймер поиска и z-index живут только пока панель открыта; при размонтировании снимаются.
    {
        let props = props.clone();
        on_cleanup(move || props.release());
    }
    // Побочные эффекты открытия и закрытия панели (положение, z-index, слушатели, прокрутка, фокус) — только в браузере.
    #[cfg(target_arch = "wasm32")]
    {
        let props = props.clone();
        Effect::new(move |_| {
            let is_open = props.open.get();
            if is_open != props.was_open.get_value() {
                props.was_open.set_value(is_open);
                if is_open {
                    props.opened();
                } else {
                    props.closed();
                }
            }
        });
    }

    let root_class = {
        let (props, pt) = (props.clone(), pt.clone());
        move || {
            let mut cn = ClassNames::new();
            write_classes(&mut cn, &props.state());
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
        move || data_p(&props.state())
    };
    let label_class = {
        let (props, pt) = (props.clone(), pt.clone());
        move || {
            let mut cn = ClassNames::new();
            write_label_classes(&mut cn, props.label_kind());
            cn.add_user(pt.label.extra_class());
            cn.finish_cow()
        }
    };
    let label_data = {
        let props = props.clone();
        move || {
            let clearable = props.show_clear.get();
            label_data_p(props.label_kind(), clearable, props.disabled.get(), props.size.get())
        }
    };
    let label_content = {
        let props = props.clone();
        move || props.label_content()
    };
    macro_rules! attr {
        ($name:literal, |$p:ident| $body:expr) => {{
            let $p = props.clone();
            reactive_attr($name, move || $body)
        }};
    }
    let label_attrs: Vec<AnyAttribute> = vec![
        attr!("id", |p| p.input_id.get()),
        attr!("aria-label", |p| p.aria_label.get().or_else(|| p.label_text())),
        attr!("aria-labelledby", |p| p.aria_labelledby.get()),
        attr!("aria-expanded", |p| Some(p.open.get().to_string())),
        attr!("aria-controls", |p| p.open.get().then(|| format!("{}_list", p.id))),
        attr!("aria-disabled", |p| Some(p.disabled.get().to_string())),
        attr!("aria-required", |p| Some(p.required.get().to_string())),
        attr!("required", |p| p.required.get().then(String::new)),
        attr!("disabled", |p| p.disabled.get().then(String::new)),
        attr!("tabindex", |p| Some(if p.disabled.get() { -1 } else { p.tabindex.get().unwrap_or(0) }.to_string())),
        attr!("aria-activedescendant", |p| p.active_descendant(p.focused.get())),
    ];
    let (focus, blur, key_down, container_click) = {
        let (a, b, c, d) = (props.clone(), props.clone(), props.clone(), props.clone());
        (
            move |e: web_sys::FocusEvent| a.label_focus(e),
            move |e: web_sys::FocusEvent| b.label_blur(e),
            move |e: web_sys::KeyboardEvent| c.key_down(e, false),
            move |e: web_sys::MouseEvent| d.container_click(e),
        )
    };
    let clear = {
        let (props, pt) = (props.clone(), pt.clone());
        move || props.clear_visible().then(|| clear_view(&props, &pt))
    };
    let dropdown_class = pt.dropdown.merge_class("p-select-dropdown");
    let dropdown_expanded = {
        let props = props.clone();
        move || props.open.get().to_string()
    };
    let dropdown_content = {
        let (props, pt) = (props.clone(), pt.clone());
        move || dropdown_view(&props, &pt)
    };
    let panel = {
        let (props, pt) = (props.clone(), pt.clone());
        move || props.open.get().then(|| panel_view(&props, &pt))
    };
    let (root_attrs, label_pt_attrs, dropdown_attrs) = (pt.root.attrs(), pt.label.attrs(), pt.dropdown.attrs());
    let (root_id, root_ref, label_ref) = (props.id.clone(), props.root_ref, props.label_ref);

    view! {
        <div node_ref={root_ref} id={root_id} class={root_class} style={root_style} data-p={data} on:click={container_click} {..root_attrs}>
            <span
                node_ref={label_ref}
                role="combobox"
                aria-haspopup="listbox"
                class={label_class}
                data-p={label_data}
                on:focus={focus}
                on:blur={blur}
                on:keydown={key_down}
                {..label_attrs}
                {..label_pt_attrs}
            >
                {label_content}
            </span>
            {clear}
            <div
                class={dropdown_class}
                role="button"
                aria-label="dropdown trigger"
                aria-haspopup="listbox"
                aria-expanded={dropdown_expanded}
                data-pc-section="trigger"
                {..dropdown_attrs}
            >
                {dropdown_content}
            </div>
            {panel}
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

/// Значок очистки.
fn clear_view(props: &Arc<Props>, pt: &Arc<SelectPt>) -> impl IntoView + use<> {
    let class = pt.clear_icon.merge_class("p-select-clear-icon");
    let attrs = pt.clear_icon.clone().attr("data-pc-section", "clearicon").attrs();
    let click = {
        let props = props.clone();
        move |e: web_sys::MouseEvent| props.clear(e.into())
    };
    match props.clear_icon.clone() {
        Some(icon) => Either::Left(view! { <span class={class} on:click={click} {..attrs}>{icon.run()}</span> }),
        None => Either::Right(view! { <TimesIcon class={class.into_owned()} on:click={click} {..attrs} /> }),
    }
}

/// Содержимое кнопки раскрытия: загрузка или стрелка.
fn dropdown_view(props: &Arc<Props>, pt: &Arc<SelectPt>) -> AnyView {
    if props.loading.get() {
        let (class, attrs) = (pt.loading_icon.merge_class("p-select-loading-icon"), pt.loading_icon.attrs());
        return match props.loading_icon.clone() {
            Some(icon) => view! { <span class={class} aria-hidden="true" {..attrs}>{icon.run()}</span> }.into_any(),
            None => view! { <SpinnerIcon spin=true class={class.into_owned()} {..attrs} /> }.into_any(),
        };
    }
    let (class, attrs) = (pt.dropdown_icon.merge_class("p-select-dropdown-icon"), pt.dropdown_icon.attrs());
    match props.dropdown_icon.clone() {
        Some(icon) => view! { <span class={class} {..attrs}>{icon.run()}</span> }.into_any(),
        None => view! { <ChevronDownIcon class={class.into_owned()} {..attrs} /> }.into_any(),
    }
}

/// Панель: скрытые границы, поле поиска, список вариантов.
fn panel_view(props: &Arc<Props>, pt: &Arc<SelectPt>) -> impl IntoView + use<> {
    let class = {
        let (props, pt) = (props.clone(), pt.clone());
        move || {
            let mut cn = ClassNames::new();
            cn.add("p-select-overlay p-component-overlay p-component");
            props.panel_style_class.with(|c| cn.add_user(c.as_deref()));
            cn.add_user(pt.overlay.extra_class());
            cn.finish_cow()
        }
    };
    let style = {
        let (props, pt) = (props.clone(), pt.clone());
        move || {
            let style = props.panel_style.with(|st| pt.overlay.merge_style(st.as_deref().unwrap_or_default()));
            (!style.is_empty()).then_some(style)
        }
    };
    let header = {
        let (props, pt) = (props.clone(), pt.clone());
        move || props.filter.get().then(|| header_view(&props, &pt))
    };
    let container_style = {
        let props = props.clone();
        move || {
            let height = props.scroll_height.get().unwrap_or_else(|| "200px".to_owned());
            format!("max-height: {height}")
        }
    };
    let list_class = pt.list.merge_class("p-select-list");
    let (container_class, list_attrs, container_attrs, overlay_attrs) =
        (pt.list_container.merge_class("p-select-list-container"), pt.list.attrs(), pt.list_container.attrs(), pt.overlay.attrs());
    let (list_id, list_label) = (format!("{}_list", props.id), props.list_label.clone());
    let items = {
        let (props, pt) = (props.clone(), pt.clone());
        move || items_view(&props, &pt)
    };
    let (first, last) = {
        let (a, b) = (props.clone(), props.clone());
        (move |e: web_sys::FocusEvent| a.first_hidden_focus(e), move |_: web_sys::FocusEvent| b.last_hidden_focus())
    };
    let overlay_ref = props.overlay_ref;

    view! {
        <div node_ref={overlay_ref} class={class} style={style} data-p="overlay-self" {..overlay_attrs}>
            <span
                role="presentation"
                class="p-hidden-accessible p-hidden-focusable"
                tabindex="0"
                data-p-hidden-accessible="true"
                data-p-hidden-focusable="true"
                on:focus={first}
            ></span>
            {header}
            <div class={container_class} style={container_style} {..container_attrs}>
                <ul id={list_id} aria-label={list_label} class={list_class} role="listbox" {..list_attrs}>
                    {items}
                </ul>
            </div>
            <span
                role="presentation"
                class="p-hidden-accessible p-hidden-focusable"
                tabindex="0"
                data-p-hidden-accessible="true"
                data-p-hidden-focusable="true"
                on:focus={last}
            ></span>
        </div>
    }
}

/// Шапка панели: поле поиска со значком.
fn header_view(props: &Arc<Props>, pt: &Arc<SelectPt>) -> impl IntoView + use<> {
    let (header_class, header_attrs) = (pt.header.merge_class("p-select-header"), pt.header.attrs());
    let (filter_class, filter_attrs) = (pt.filter.merge_class("p-inputtext p-component p-select-filter"), pt.filter.attrs());
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
        move || props.active_descendant(true)
    };
    let owns = format!("{}_list", props.id);
    let (input, key_down, blur) = {
        let (a, b, c) = (props.clone(), props.clone(), props.clone());
        (
            move |e: web_sys::Event| a.filter_input(e),
            move |e: web_sys::KeyboardEvent| b.key_down(e, true),
            move |_: web_sys::FocusEvent| c.focused_index.set(None),
        )
    };
    let filter_ref = props.filter_ref;
    view! {
        <div class={header_class} {..header_attrs}>
            <span class="p-iconfield">
                <input
                    node_ref={filter_ref}
                    type="text"
                    role="searchbox"
                    autocomplete="off"
                    class={filter_class}
                    value={value_attr}
                    prop:value={value}
                    placeholder={placeholder}
                    aria-owns={owns}
                    aria-label={aria_label}
                    aria-activedescendant={active}
                    on:input={input}
                    on:keydown={key_down}
                    on:blur={blur}
                    {..filter_attrs}
                />
                <span class="p-inputicon"><SearchIcon /></span>
            </span>
        </div>
    }
}

/// Список элементов: варианты, заголовки групп, сообщение пустого списка. Читает только варианты, группы и строку поиска.
fn items_view(props: &Arc<Props>, pt: &Arc<SelectPt>) -> impl IntoView + use<> {
    let entries = props.entries();
    if entries.is_empty() {
        return Either::Left(empty_view(props, pt));
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
    Either::Right(views)
}

/// Сообщение пустого списка или пустого результата.
fn empty_view(props: &Arc<Props>, pt: &Arc<SelectPt>) -> impl IntoView + use<> {
    let class = pt.empty_message.merge_class("p-select-empty-message");
    let attrs = pt.empty_message.attrs();
    let text = props.empty_text();
    view! { <li class={class} role="option" {..attrs}>{text}</li> }
}

/// Заголовок группы.
fn group_view(props: &Arc<Props>, pt: &Arc<SelectPt>, index: usize, group: usize, label: &str) -> AnyView {
    let id = format!("{}_{}", props.id, index);
    let (class, attrs) = (pt.option_group.merge_class("p-select-option-group"), pt.option_group.attrs());
    let (label_class, label_attrs) = (pt.option_group_label.merge_class("p-select-option-group-label"), pt.option_group_label.attrs());
    let content = match (props.group_template, props.groups.get_untracked().get(group)) {
        (Some(template), Some(g)) => template.run(g.clone()),
        _ => view! { <span class={label_class} {..label_attrs}>{label.to_owned()}</span> }.into_any(),
    };
    view! { <li id={id} class={class} role="option" {..attrs}>{content}</li> }.into_any()
}

/// Вариант.
fn option_view(props: &Arc<Props>, pt: &Arc<SelectPt>, index: usize, option: &SelectOption, size: usize, pos: usize) -> AnyView {
    let id = format!("{}_{}", props.id, index);
    let value = option.value.clone();
    let selected = {
        let (props, value) = (props.clone(), value.clone());
        move || props.value.get().as_deref() == Some(value.as_str())
    };
    let focused = {
        let props = props.clone();
        move || props.focused_index.get() == Some(index)
    };
    let class = {
        let (props, pt, selected, focused, disabled) = (props.clone(), pt.clone(), selected.clone(), focused.clone(), option.disabled);
        move || {
            let mut cn = ClassNames::new();
            write_option_classes(&mut cn, selected(), props.checkmark.get(), disabled, focused());
            cn.add_user(pt.option.extra_class());
            cn.finish_cow()
        }
    };
    let (selected_attr, highlight_attr) = (flag(selected.clone()), flag(selected.clone()));
    let aria_selected = {
        let selected = selected.clone();
        move || selected().to_string()
    };
    let focused_attr = flag(focused);
    let disabled_attr = option.disabled.then_some("true");
    let content = {
        let (props, pt, option, selected) = (props.clone(), pt.clone(), option.clone(), selected);
        move || option_content(&props, &pt, &option, selected())
    };
    let (click, enter) = {
        let (a, b, option) = (props.clone(), props.clone(), option.clone());
        (
            move |e: web_sys::MouseEvent| a.select_option(&option, e.into(), true),
            move |e: web_sys::MouseEvent| b.option_mouse_enter(e, index),
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
            aria-setsize={size}
            aria-posinset={pos}
            aria-selected={aria_selected}
            data-p-focused={focused_attr}
            data-p-highlight={highlight_attr}
            data-p-selected={selected_attr}
            data-p-disabled={disabled_attr}
            on:click={click}
            on:mouseenter={enter}
            {..attrs}
        >
            {content}
        </li>
    }
    .into_any()
}

/// Содержимое варианта: галочка или пустой значок, затем метка или шаблон.
fn option_content(props: &Arc<Props>, pt: &Arc<SelectPt>, option: &SelectOption, selected: bool) -> AnyView {
    let mark = props.checkmark.get().then(|| {
        if selected {
            let (class, attrs) = (pt.option_check_icon.merge_class("p-select-option-check-icon"), pt.option_check_icon.attrs());
            view! { <CheckIcon class={class.into_owned()} {..attrs} /> }.into_any()
        } else {
            let (class, attrs) = (pt.option_blank_icon.merge_class("p-select-option-blank-icon"), pt.option_blank_icon.attrs());
            view! { <BlankIcon class={class.into_owned()} {..attrs} /> }.into_any()
        }
    });
    let body = match props.item_template {
        Some(template) => template.run(option.clone()),
        None => {
            let (class, attrs) = (pt.option_label.merge_class(""), pt.option_label.attrs());
            let class = (!class.is_empty()).then(|| class.into_owned());
            view! { <span class={class} {..attrs}>{option.label.clone()}</span> }.into_any()
        }
    };
    view! { {mark}{body} }.into_any()
}

/// Свойства и состояние экземпляра.
struct Props {
    options: Prop<Vec<SelectOption>>,
    groups: Prop<Vec<SelectGroup>>,
    value: RwSignal<Option<String>>,
    open: RwSignal<bool>,
    /// Строка поиска.
    filter_text: RwSignal<String>,
    /// Индекс элемента с фокусом в плоском списке.
    focused_index: RwSignal<Option<usize>>,
    /// Метка в фокусе.
    focused: RwSignal<bool>,
    /// Панель открыта мышью: первая стрелка идёт на первый или последний вариант.
    clicked: RwSignal<bool>,
    id: String,
    placeholder: OptProp<String>,
    show_clear: Prop<bool>,
    checkmark: Prop<bool>,
    filter: Prop<bool>,
    filter_placeholder: OptProp<String>,
    reset_filter_on_hide: Prop<bool>,
    autofocus_filter: OptProp<bool>,
    empty_message: OptProp<String>,
    empty_filter_message: OptProp<String>,
    loading: Prop<bool>,
    focus_on_hover: OptProp<bool>,
    select_on_focus: Prop<bool>,
    auto_option_focus: Prop<bool>,
    scroll_height: OptProp<String>,
    disabled: Prop<bool>,
    readonly: Prop<bool>,
    invalid: Prop<bool>,
    required: Prop<bool>,
    variant: OptProp<Variant>,
    config_variant: Option<Variant>,
    size: OptProp<Size>,
    fluid: Prop<bool>,
    input_id: OptProp<String>,
    tabindex: OptProp<i32>,
    aria_label: OptProp<String>,
    aria_labelledby: OptProp<String>,
    aria_filter_label: OptProp<String>,
    style: OptProp<String>,
    style_class: OptProp<String>,
    panel_style: OptProp<String>,
    panel_style_class: OptProp<String>,
    dropdown_icon: Option<ViewFn>,
    clear_icon: Option<ViewFn>,
    loading_icon: Option<ViewFn>,
    item_template: Option<Callback<SelectOption, AnyView>>,
    selected_item_template: Option<Callback<SelectOption, AnyView>>,
    group_template: Option<Callback<SelectGroup, AnyView>>,
    on_change: Option<Callback<SelectChange>>,
    on_filter: Option<Callback<SelectFilter>>,
    on_focus: Option<Callback<web_sys::FocusEvent>>,
    on_blur: Option<Callback<web_sys::FocusEvent>>,
    on_click: Option<Callback<web_sys::MouseEvent>>,
    on_show: Option<Callback<()>>,
    on_hide: Option<Callback<()>>,
    on_clear: Option<Callback<web_sys::Event>>,
    default_empty: String,
    default_empty_filter: String,
    list_label: String,
    root_ref: NodeRef<html::Div>,
    label_ref: NodeRef<html::Span>,
    overlay_ref: NodeRef<html::Div>,
    filter_ref: NodeRef<html::Input>,
    /// Накопленные символы поиска набором.
    search: StoredValue<String>,
    search_timer: StoredValue<Option<TimeoutHandle>, LocalStorage>,
    /// Подписки на `window`, пока панель открыта.
    listeners: StoredValue<Vec<WindowListenerHandle>, LocalStorage>,
    /// Выданный панели z-index.
    z_index: StoredValue<Option<i32>>,
    /// Предыдущее состояние открытости (для побочных эффектов только при смене).
    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    was_open: StoredValue<bool>,
}

impl Props {
    /// Плоский список видимых элементов (с подпиской).
    fn entries(&self) -> Vec<Entry> {
        visible_entries(&self.options.get(), &self.groups.get(), &self.filter_text.get())
    }

    fn entries_untracked(&self) -> Vec<Entry> {
        untrack(|| self.entries())
    }

    /// Выбранный вариант среди всех вариантов (с подпиской); значение вне списка — невыбранное.
    fn selected_option(&self) -> Option<SelectOption> {
        let value = self.value.get()?;
        find_option(&self.options.get(), &self.groups.get(), &value)
    }

    fn label_kind(&self) -> LabelKind {
        if self.selected_option().is_some() {
            LabelKind::Selected
        } else if self.placeholder.get().is_some_and(|p| !p.is_empty()) {
            LabelKind::Placeholder
        } else {
            LabelKind::Empty
        }
    }

    /// Текст метки: метка выбранного варианта или подсказка.
    fn label_text(&self) -> Option<String> {
        match self.selected_option() {
            Some(o) => Some(o.label),
            None => self.placeholder.get().filter(|p| !p.is_empty()),
        }
    }

    fn label_content(&self) -> AnyView {
        match self.selected_option() {
            Some(option) => match self.selected_item_template {
                Some(template) => template.run(option),
                None => option.label.into_any(),
            },
            None => self.placeholder.get().filter(|p| !p.is_empty()).unwrap_or_else(|| "\u{a0}".to_owned()).into_any(),
        }
    }

    fn state(&self) -> SelectState {
        SelectState {
            disabled: self.disabled.get(),
            variant_filled: self.variant.get().or(self.config_variant) == Some(Variant::Filled),
            focused: self.focused.get(),
            invalid: self.invalid.get(),
            filled: self.selected_option().is_some(),
            open: self.open.get(),
            fluid: self.fluid.get(),
            size: self.size.get(),
        }
    }

    fn clear_visible(&self) -> bool {
        self.show_clear.get() && !self.disabled.get() && self.selected_option().is_some()
    }

    fn empty_text(&self) -> String {
        if self.filter_text.get().is_empty() {
            self.empty_message.get().filter(|m| !m.is_empty()).unwrap_or_else(|| self.default_empty.clone())
        } else {
            self.empty_filter_message.get().filter(|m| !m.is_empty()).unwrap_or_else(|| self.default_empty_filter.clone())
        }
    }

    /// Идентификатор варианта под фокусом (для `aria-activedescendant`); `enabled` — элемент в фокусе.
    fn active_descendant(&self, enabled: bool) -> Option<String> {
        let index = self.focused_index.get()?;
        enabled.then(|| format!("{}_{}", self.id, index))
    }

    /// Только чтение, недоступно или идёт загрузка: ввод заблокирован.
    fn blocked(&self) -> bool {
        self.disabled.get_untracked() || self.readonly.get_untracked() || self.loading.get_untracked()
    }

    fn focus_label(&self) {
        if let Some(label) = self.label_ref.get_untracked() {
            let _ = label.focus();
        }
    }

    // ---- открытие и закрытие ----

    /// `show()`: открывает панель; фокус варианта — прежний, иначе первый (`auto_option_focus`) или выбранный.
    fn show(&self, focus: bool) {
        self.open.set(true);
        if self.focused_index.get_untracked().is_none() {
            let entries = self.entries_untracked();
            let value = self.value.get_untracked();
            let index = if self.auto_option_focus.get_untracked() {
                first_focused(&entries, value.as_deref())
            } else {
                selected_index(&entries, value.as_deref())
            };
            self.focused_index.set(index);
        }
        if focus {
            self.focus_label();
        }
    }

    /// `hide()`: закрывает панель и сбрасывает состояние списка.
    fn hide(&self, focus: bool) {
        self.open.set(false);
        self.focused_index.set(None);
        self.clicked.set(false);
        self.search.set_value(String::new());
        if self.filter.get_untracked() && self.reset_filter_on_hide.get_untracked() {
            self.filter_text.set(String::new());
        }
        if focus {
            self.focus_label();
        }
    }

    /// Панель только что открылась: z-index, положение, подписки, прокрутка и фокус поля поиска, `on_show`.
    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    fn opened(self: &Arc<Self>) {
        if let (Some(overlay), Some(root)) = (self.overlay_ref.get_untracked(), self.root_ref.get_untracked()) {
            let base = use_prime_config().z_index.base(ZGroup::Overlay);
            let z = prime_core::zindex::generate(ZGroup::Overlay, base);
            self.z_index.set_value(Some(z));
            let overlay: web_sys::HtmlElement = overlay.unchecked_into();
            let _ = overlay.style().set_property("z-index", &z.to_string());
            align_overlay(&overlay, &root.unchecked_into::<web_sys::HtmlElement>(), AlignMode::Relative, true);
        }
        self.bind_window();
        self.scroll_selected_into_view();
        if self.filter.get_untracked()
            && self.autofocus_filter.get_untracked().unwrap_or(true)
            && let Some(input) = self.filter_ref.get_untracked()
        {
            let _ = input.focus();
        }
        if let Some(cb) = self.on_show {
            cb.run(());
        }
    }

    /// Панель только что закрылась: подписки и z-index освобождаются, `on_hide`.
    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    fn closed(&self) {
        self.release();
        if let Some(cb) = self.on_hide {
            cb.run(());
        }
    }

    /// Снимает подписки на `window`, таймер поиска и z-index.
    fn release(&self) {
        for handle in self.listeners.try_update_value(std::mem::take).unwrap_or_default() {
            handle.remove();
        }
        if let Some(z) = self.z_index.try_update_value(Option::take).flatten() {
            prime_core::zindex::revert(z);
        }
        if let Some(handle) = self.search_timer.try_update_value(Option::take).flatten() {
            handle.clear();
        }
    }

    /// Закрытие по клику вне компонента и по изменению размера окна.
    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    fn bind_window(self: &Arc<Self>) {
        let this = self.clone();
        let outside = window_event_listener(ev::click, move |e: web_sys::MouseEvent| {
            let inside = this.root_ref.get_untracked().is_some_and(|root| {
                e.target().and_then(|t| t.dyn_into::<web_sys::Node>().ok()).is_some_and(|node| root.contains(Some(&node)))
            });
            if !inside && this.open.get_untracked() {
                this.hide(false);
            }
        });
        let this = self.clone();
        let resize = window_event_listener(ev::resize, move |_| {
            if this.open.get_untracked() {
                this.hide(false);
            }
        });
        self.listeners.update_value(|l| l.extend([outside, resize]));
    }

    /// Пересчитывает положение панели после перерисовки списка.
    fn realign(self: &Arc<Self>) {
        let this = self.clone();
        set_timeout(
            move || {
                if this.open.is_disposed() {
                    return;
                }
                if let (Some(overlay), Some(root)) = (this.overlay_ref.get_untracked(), this.root_ref.get_untracked()) {
                    align_overlay(
                        &overlay.unchecked_into::<web_sys::HtmlElement>(),
                        &root.unchecked_into::<web_sys::HtmlElement>(),
                        AlignMode::Relative,
                        true,
                    );
                }
            },
            Duration::ZERO,
        );
    }

    fn scroll_to(&self, selector_id: &str) {
        let element = leptos::prelude::document().get_element_by_id(selector_id);
        if let Some(el) = element {
            let options = web_sys::ScrollIntoViewOptions::new();
            options.set_block(web_sys::ScrollLogicalPosition::Nearest);
            options.set_inline(web_sys::ScrollLogicalPosition::Nearest);
            el.scroll_into_view_with_scroll_into_view_options(&options);
        }
    }

    fn scroll_selected_into_view(&self) {
        if let Some(index) = selected_index(&self.entries_untracked(), self.value.get_untracked().as_deref()) {
            self.scroll_to(&format!("{}_{}", self.id, index));
        }
    }

    // ---- выбор ----

    /// `onOptionSelect`: выбирает вариант, вызывает `on_change` и при `hide` закрывает панель.
    fn select_option(&self, option: &SelectOption, event: web_sys::Event, hide: bool) {
        if option.disabled {
            return;
        }
        if self.value.get_untracked().as_deref() != Some(option.value.as_str()) {
            self.value.set(Some(option.value.clone()));
            self.focused_index.set(selected_index(&self.entries_untracked(), Some(&option.value)));
            if let Some(cb) = self.on_change {
                cb.run(SelectChange { value: Some(option.value.clone()), original_event: event });
            }
        }
        if hide {
            self.hide(true);
        }
    }

    /// Перенос фокуса на элемент `index` (`changeFocusedOptionIndex`).
    fn change_focused(&self, event: &web_sys::Event, index: Option<usize>) {
        if self.focused_index.get_untracked() == index {
            return;
        }
        self.focused_index.set(index);
        if let Some(i) = index {
            self.scroll_to(&format!("{}_{}", self.id, i));
            if self.select_on_focus.get_untracked()
                && let Some(Entry::Option(option)) = self.entries_untracked().get(i)
            {
                self.select_option(option, event.clone(), false);
            }
        }
    }

    fn option_mouse_enter(&self, event: web_sys::MouseEvent, index: usize) {
        if self.focus_on_hover.get_untracked().unwrap_or(true) {
            self.change_focused(&event.into(), Some(index));
        }
    }

    /// Очистка: значение `None`, `on_change`, `on_clear`, сброс строки поиска.
    fn clear(&self, event: web_sys::Event) {
        self.value.set(None);
        self.filter_text.set(String::new());
        if let Some(cb) = self.on_change {
            cb.run(SelectChange { value: None, original_event: event.clone() });
        }
        if let Some(cb) = self.on_clear {
            cb.run(event);
        }
    }

    // ---- события метки и контейнера ----

    fn container_click(&self, e: web_sys::MouseEvent) {
        if self.blocked() {
            return;
        }
        let Some(target) = e.target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()) else { return };
        if target.tag_name().eq_ignore_ascii_case("INPUT") || target.closest("[data-pc-section=\"clearicon\"]").ok().flatten().is_some() {
            return;
        }
        let in_overlay = self.overlay_ref.get_untracked().is_some_and(|overlay| overlay.contains(Some(&target)));
        if !in_overlay {
            if self.open.get_untracked() {
                self.hide(true);
            } else {
                self.show(true);
            }
        }
        self.focus_label();
        if let Some(cb) = self.on_click {
            cb.run(e);
        }
        self.clicked.set(true);
    }

    fn label_focus(&self, e: web_sys::FocusEvent) {
        if self.disabled.get_untracked() {
            return;
        }
        self.focused.set(true);
        if self.focused_index.get_untracked().is_none() && self.open.get_untracked() && self.auto_option_focus.get_untracked() {
            let entries = self.entries_untracked();
            self.focused_index.set(first_focused(&entries, self.value.get_untracked().as_deref()));
        }
        if self.open.get_untracked()
            && let Some(i) = self.focused_index.get_untracked()
        {
            self.scroll_to(&format!("{}_{}", self.id, i));
        }
        if let Some(cb) = self.on_focus {
            cb.run(e);
        }
    }

    fn label_blur(&self, e: web_sys::FocusEvent) {
        self.focused.set(false);
        if let Some(cb) = self.on_blur {
            cb.run(e);
        }
    }

    /// Первая скрытая граница получила фокус: из метки — в поле поиска, иначе — назад на метку.
    fn first_hidden_focus(&self, e: web_sys::FocusEvent) {
        let from_label = e
            .related_target()
            .and_then(|t| t.dyn_into::<web_sys::Node>().ok())
            .is_some_and(|node| self.label_ref.get_untracked().is_some_and(|label| label.is_same_node(Some(&node))));
        match (from_label, self.filter_ref.get_untracked()) {
            (true, Some(input)) => {
                let _ = input.focus();
            }
            _ => self.focus_label(),
        }
    }

    /// Последняя скрытая граница получила фокус: панель закрывается, фокус на метке.
    fn last_hidden_focus(&self) {
        self.hide(true);
    }

    // ---- поиск ----

    fn filter_input(self: &Arc<Self>, e: web_sys::Event) {
        let Some(input) = self.filter_ref.get_untracked() else { return };
        let text = input.value();
        self.filter_text.set(text.clone());
        self.focused_index.set(None);
        if let Some(cb) = self.on_filter {
            cb.run(SelectFilter { filter: text, original_event: e });
        }
        self.realign();
    }

    /// Поиск набором (`searchOptions`): символы копятся 500 мс.
    fn search_by_typing(self: &Arc<Self>, event: web_sys::Event, key: &str) {
        let typed = {
            let mut typed = self.search.get_value();
            typed.push_str(key);
            self.search.set_value(typed.clone());
            typed
        };
        let entries = self.entries_untracked();
        let mut index = search_match(&entries, &typed);
        if index.is_none() && self.focused_index.get_untracked().is_none() {
            index = first_focused(&entries, self.value.get_untracked().as_deref());
        }
        if let Some(i) = index {
            let this = self.clone();
            set_timeout(
                move || {
                    if !this.open.is_disposed() {
                        this.change_focused(&event, Some(i));
                    }
                },
                Duration::ZERO,
            );
        }
        if let Some(handle) = self.search_timer.try_update_value(Option::take).flatten() {
            handle.clear();
        }
        let this = self.clone();
        let handle = set_timeout_with_handle(
            move || {
                if !this.open.is_disposed() {
                    this.search.set_value(String::new());
                }
            },
            Duration::from_millis(SEARCH_RESET_MS),
        )
        .ok();
        self.search_timer.set_value(handle);
    }

    // ---- клавиатура ----

    /// `onKeyDown` метки и `onFilterKeyDown` поля поиска (`in_filter`).
    fn key_down(self: &Arc<Self>, e: web_sys::KeyboardEvent, in_filter: bool) {
        if self.blocked() {
            return;
        }
        let event: web_sys::Event = e.clone().into();
        match e.code().as_str() {
            "ArrowDown" => self.arrow_down(&e),
            "ArrowUp" => self.arrow_up(&e, in_filter),
            "ArrowLeft" | "ArrowRight" => {
                if in_filter {
                    self.focused_index.set(None);
                }
            }
            "Delete" if !in_filter => {
                if self.show_clear.get_untracked() {
                    self.clear(event);
                    e.prevent_default();
                }
            }
            "Home" => self.home_end(&e, in_filter, true),
            "End" => self.home_end(&e, in_filter, false),
            "PageDown" if !in_filter => {
                if let Some(last) = self.entries_untracked().len().checked_sub(1) {
                    self.scroll_to(&format!("{}_{}", self.id, last));
                }
                e.prevent_default();
            }
            "PageUp" if !in_filter => {
                self.scroll_to(&format!("{}_0", self.id));
                e.prevent_default();
            }
            "Space" if !in_filter => self.enter_key(&e, false),
            "Enter" | "NumpadEnter" => self.enter_key(&e, in_filter),
            "Escape" => {
                if self.open.get_untracked() {
                    self.hide(true);
                    e.prevent_default();
                    e.stop_propagation();
                }
            }
            "Tab" => self.tab_key(&e, in_filter),
            "ShiftLeft" | "ShiftRight" | "Backspace" => {}
            _ if !in_filter => {
                let key = e.key();
                if !e.meta_key() && !e.ctrl_key() && key.chars().count() == 1 {
                    if !self.open.get_untracked() {
                        self.show(false);
                    }
                    self.search_by_typing(event, &key);
                }
            }
            _ => {}
        }
        if !in_filter {
            self.clicked.set(false);
        }
    }

    fn arrow_down(&self, e: &web_sys::KeyboardEvent) {
        if !self.open.get_untracked() {
            self.show(false);
        } else {
            let entries = self.entries_untracked();
            let index = match self.focused_index.get_untracked() {
                Some(i) => Some(next_valid(&entries, i)),
                None if self.clicked.get_untracked() => first_valid(&entries),
                None => first_focused(&entries, self.value.get_untracked().as_deref()),
            };
            self.change_focused(e, index);
        }
        e.prevent_default();
        e.stop_propagation();
    }

    fn arrow_up(&self, e: &web_sys::KeyboardEvent, in_filter: bool) {
        if e.alt_key() && !in_filter {
            if let Some(Entry::Option(option)) = self.focused_index.get_untracked().and_then(|i| self.entries_untracked().get(i).cloned()) {
                self.select_option(&option, e.clone().into(), true);
            }
            if self.open.get_untracked() {
                self.hide(false);
            }
        } else {
            let entries = self.entries_untracked();
            let index = match self.focused_index.get_untracked() {
                Some(i) => Some(prev_valid(&entries, i)),
                None if self.clicked.get_untracked() => last_valid(&entries),
                None => last_focused(&entries, self.value.get_untracked().as_deref()),
            };
            self.change_focused(e, index);
            if !self.open.get_untracked() {
                self.show(false);
            }
        }
        e.prevent_default();
        e.stop_propagation();
    }

    /// `Home` и `End`: в поле поиска двигают курсор, иначе фокус на первый или последний вариант.
    fn home_end(&self, e: &web_sys::KeyboardEvent, in_filter: bool, home: bool) {
        if in_filter && let Some(input) = self.filter_ref.get_untracked() {
            let len = input.value().chars().count() as u32;
            if e.shift_key() {
                let _ = input.set_selection_range(0, len);
            } else {
                let caret = if home { 0 } else { len };
                let _ = input.set_selection_range(caret, caret);
                self.focused_index.set(None);
            }
        } else {
            let entries = self.entries_untracked();
            self.change_focused(e, if home { first_valid(&entries) } else { last_valid(&entries) });
            if !self.open.get_untracked() {
                self.show(false);
            }
        }
        e.prevent_default();
    }

    fn enter_key(self: &Arc<Self>, e: &web_sys::KeyboardEvent, in_input: bool) {
        if !self.open.get_untracked() {
            self.focused_index.set(None);
            self.arrow_down(e);
        } else {
            if let Some(Entry::Option(option)) = self.focused_index.get_untracked().and_then(|i| self.entries_untracked().get(i).cloned()) {
                self.select_option(&option, e.clone().into(), true);
            }
            if !in_input {
                self.hide(false);
            }
        }
        e.prevent_default();
    }

    fn tab_key(&self, e: &web_sys::KeyboardEvent, in_filter: bool) {
        if !in_filter {
            // с полем поиска Tab из метки ведёт в него через первую скрытую границу панели, без него — выбирает и закрывает
            let goes_to_filter = self.open.get_untracked() && self.filter.get_untracked();
            if !goes_to_filter {
                if self.open.get_untracked()
                    && let Some(Entry::Option(option)) =
                        self.focused_index.get_untracked().and_then(|i| self.entries_untracked().get(i).cloned())
                {
                    self.select_option(&option, e.clone().into(), true);
                }
                if self.open.get_untracked() {
                    self.hide(false);
                }
            }
        }
        e.stop_propagation();
    }
}
