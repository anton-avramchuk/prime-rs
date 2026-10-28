//! RadioButton — порт `p-radiobutton` из PrimeNG 21.1.10.
//!
//! Разметка и классы как в PrimeNG: корень `div.p-radiobutton` (вместо хоста `<p-radiobutton>`), внутри `input.p-radiobutton-input`
//! и `div.p-radiobutton-box` с `div.p-radiobutton-icon`.
//!
//! Группа — это общий сигнал `model: RwSignal<Option<String>>`: кнопка отмечена, когда в нём лежит её `value`. Без `model` кнопка
//! неконтролируемая: начальное состояние из `default_checked`, дальше его хранит DOM (нативная группировка по `name`), а классы корня
//! не отслеживаются.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_radiobutton::RadioButton;
//!
//! let model = RwSignal::new(Some("Cheese".to_owned()));
//! let html = view! { <RadioButton value="Cheese" model=model name="pizza" /> }.to_html();
//! assert!(html.contains("p-radiobutton-checked") && html.contains("p-radiobutton-input") && html.contains(r#"name="pizza""#));
//! ```

mod state;

use leptos::either::Either;
use leptos::prelude::*;
use prime_core::{ClassNames, OptProp, Prop, Pt, Size, Variant, all_static, use_prime_config};
use std::sync::Arc;

pub use state::{RadioButtonState, data_p, write_classes};

/// Событие выбора: значение кнопки и исходное событие браузера.
#[derive(Clone, Debug)]
pub struct RadioButtonClick {
    /// Значение выбранной кнопки.
    pub value: Option<String>,
    /// Исходное событие `change`.
    pub original_event: web_sys::Event,
}

/// Pass-through частей RadioButton.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RadioButtonPt {
    /// Корневой элемент.
    pub root: Pt,
    /// Нативный `<input>`.
    pub input: Pt,
    /// Рамка (`div.p-radiobutton-box`).
    pub box_: Pt,
    /// Точка (`div.p-radiobutton-icon`).
    pub icon: Pt,
}

/// Переключатель выбора одного значения из группы.
///
/// Каждое свойство принимает значение или сигнал; `model` — двусторонний сигнал группы.
#[component]
pub fn RadioButton(
    /// Значение этой кнопки (и атрибут `value` у `<input>`).
    #[prop(optional, into)]
    value: OptProp<String>,
    /// Двусторонний сигнал выбранного значения группы.
    #[prop(optional)]
    model: Option<RwSignal<Option<String>>>,
    /// Начальное состояние неконтролируемой кнопки.
    #[prop(optional)]
    default_checked: bool,
    /// Атрибут `id` у `<input>`.
    #[prop(optional, into)]
    input_id: OptProp<String>,
    /// Атрибут `name`.
    #[prop(optional, into)]
    name: OptProp<String>,
    /// Атрибут `tabindex`.
    #[prop(optional, into)]
    tabindex: OptProp<i32>,
    /// Атрибут `aria-label`.
    #[prop(optional, into)]
    aria_label: OptProp<String>,
    /// Атрибут `aria-labelledby`.
    #[prop(optional, into)]
    aria_labelledby: OptProp<String>,
    /// Недоступна.
    #[prop(optional, into)]
    disabled: Prop<bool>,
    /// Обязательна.
    #[prop(optional, into)]
    required: Prop<bool>,
    /// Недопустимое значение.
    #[prop(optional, into)]
    invalid: Prop<bool>,
    /// Размер.
    #[prop(optional, into)]
    size: OptProp<Size>,
    /// Вариант; если не задан, берётся `PrimeConfig.input_variant`.
    #[prop(optional, into)]
    variant: OptProp<Variant>,
    /// Встроенный стиль корня.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Дополнительный класс корня.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Вызывается после выбора пользователем.
    #[prop(optional, into)]
    on_click: Option<Callback<RadioButtonClick>>,
    /// Вызывается при получении фокуса.
    #[prop(optional, into)]
    on_focus: Option<Callback<web_sys::FocusEvent>>,
    /// Вызывается при потере фокуса.
    #[prop(optional, into)]
    on_blur: Option<Callback<web_sys::FocusEvent>>,
    /// Pass-through частей.
    #[prop(optional)]
    pt: Option<RadioButtonPt>,
) -> impl IntoView {
    let pt = Arc::new(pt.unwrap_or_default());
    let is_static = all_static!(
        value,
        input_id,
        name,
        tabindex,
        aria_label,
        aria_labelledby,
        disabled,
        required,
        invalid,
        size,
        variant,
        style,
        style_class
    ) && model.is_none();
    let props = Arc::new(Props {
        value,
        model,
        default_checked,
        input_id,
        name,
        tabindex,
        aria_label,
        aria_labelledby,
        disabled,
        required,
        invalid,
        size,
        variant,
        style,
        style_class,
        config_variant: use_prime_config().input_variant,
    });
    let (root_attrs, input_attrs, box_attrs, icon_attrs) = (pt.root.attrs(), pt.input.attrs(), pt.box_.attrs(), pt.icon.attrs());

    let change = {
        let props = props.clone();
        move |e: web_sys::Event| {
            let input = event_target::<web_sys::HtmlInputElement>(&e);
            if props.disabled.get() {
                input.set_checked(checked_of(&props));
                return;
            }
            let value = props.value.get();
            if let Some(model) = props.model {
                model.set(value.clone());
            }
            if let Some(cb) = on_click {
                cb.run(RadioButtonClick { value, original_event: e });
            }
        }
    };
    let focus = move |e: web_sys::FocusEvent| {
        if let Some(cb) = on_focus {
            cb.run(e);
        }
    };
    let blur = move |e: web_sys::FocusEvent| {
        if let Some(cb) = on_blur {
            cb.run(e);
        }
    };

    if is_static {
        let (p, t) = (&*props, &*pt);
        let root_class = root_class(p, t);
        let root_style = root_style(p, t);
        let (checked_attr, disabled_attr, data) = (bool_attr(checked_of(p)), bool_attr(p.disabled.get()), data_of(p));
        let (id, value, name, tabindex) = (p.input_id.get(), p.value.get(), p.name.get(), p.tabindex.get());
        let (required, disabled) = (p.required.get(), p.disabled.get());
        let (aria_label, aria_labelledby) = (p.aria_label.get(), p.aria_labelledby.get());
        let (is_checked, input_class) = (checked_of(p), t.input.merge_class("p-radiobutton-input"));
        let (box_class, icon_class) = (t.box_.merge_class("p-radiobutton-box"), t.icon.merge_class("p-radiobutton-icon"));
        Either::Left(radio_markup!(
            root_class,
            root_style,
            checked_attr,
            disabled_attr,
            data,
            id,
            value,
            name,
            tabindex,
            required,
            disabled,
            aria_label,
            aria_labelledby,
            input_class,
            is_checked,
            box_class,
            icon_class,
            change,
            focus,
            blur,
            root_attrs,
            input_attrs,
            box_attrs,
            icon_attrs
        ))
    } else {
        let (p, t) = (&props, &pt);
        let root_class = rx(p, t, root_class);
        let root_style = rx(p, t, root_style);
        let checked_attr = rx(p, t, |p, _| bool_attr(checked_of(p)));
        let disabled_attr = rx(p, t, |p, _| bool_attr(p.disabled.get()));
        let data = rx(p, t, |p, _| data_of(p));
        let id = rx(p, t, |p, _| p.input_id.get());
        let value = rx(p, t, |p, _| p.value.get());
        let name = rx(p, t, |p, _| p.name.get());
        let tabindex = rx(p, t, |p, _| p.tabindex.get());
        let required = rx(p, t, |p, _| p.required.get());
        let disabled = rx(p, t, |p, _| p.disabled.get());
        let aria_label = rx(p, t, |p, _| p.aria_label.get());
        let aria_labelledby = rx(p, t, |p, _| p.aria_labelledby.get());
        let input_class = rx(p, t, |_, t| t.input.merge_class("p-radiobutton-input"));
        let is_checked = rx(p, t, |p, _| checked_of(p));
        let box_class = rx(p, t, |_, t| t.box_.merge_class("p-radiobutton-box"));
        let icon_class = rx(p, t, |_, t| t.icon.merge_class("p-radiobutton-icon"));
        Either::Right(radio_markup!(
            root_class,
            root_style,
            checked_attr,
            disabled_attr,
            data,
            id,
            value,
            name,
            tabindex,
            required,
            disabled,
            aria_label,
            aria_labelledby,
            input_class,
            is_checked,
            box_class,
            icon_class,
            change,
            focus,
            blur,
            root_attrs,
            input_attrs,
            box_attrs,
            icon_attrs
        ))
    }
}

/// Разметка RadioButton: одна для статической и реактивной ветвей (меняются только выражения атрибутов).
macro_rules! radio_markup {
    (
        $root_class:expr, $root_style:expr, $checked_attr:expr, $disabled_attr:expr, $data:expr, $id:expr, $value:expr, $name:expr,
        $tabindex:expr, $required:expr, $disabled:expr, $aria_label:expr, $aria_labelledby:expr, $input_class:expr, $is_checked:expr,
        $box_class:expr, $icon_class:expr, $change:expr, $focus:expr, $blur:expr, $root_attrs:expr, $input_attrs:expr, $box_attrs:expr,
        $icon_attrs:expr
    ) => {{
        let checked_root = $checked_attr.clone();
        let checked_aria = $checked_attr.clone();
        view! {
            <div class={$root_class} style={$root_style} data-p-disabled={$disabled_attr} data-p-checked={checked_root} data-p={$data} {..$root_attrs}>
                <input
                    id={$id}
                    type="radio"
                    class={$input_class}
                    name={$name}
                    required={$required}
                    disabled={$disabled}
                    value={$value}
                    aria-labelledby={$aria_labelledby}
                    aria-label={$aria_label}
                    aria-checked={checked_aria}
                    tabindex={$tabindex}
                    checked={$is_checked.clone()}
                    prop:checked={$is_checked}
                    on:focus={$focus}
                    on:blur={$blur}
                    on:change={$change}
                    {..$input_attrs}
                />
                <div class={$box_class} {..$box_attrs}>
                    <div class={$icon_class} {..$icon_attrs}></div>
                </div>
            </div>
        }
    }};
}
use radio_markup;

/// Замыкание над свойствами экземпляра: клоны `Arc` и указатель на функцию без захвата.
fn rx<T>(
    props: &Arc<Props>,
    pt: &Arc<RadioButtonPt>,
    f: fn(&Props, &RadioButtonPt) -> T,
) -> impl Fn() -> T + Clone + Send + Sync + 'static + use<T>
where
    T: 'static,
{
    let (props, pt) = (props.clone(), pt.clone());
    move || f(&props, &pt)
}

fn bool_attr(v: bool) -> &'static str {
    if v { "true" } else { "false" }
}

fn checked_of(props: &Props) -> bool {
    match props.model {
        Some(model) => props.value.with(|v| model.with(|m| v.is_some() && v == m)),
        None => props.default_checked,
    }
}

fn state_of(props: &Props) -> RadioButtonState {
    RadioButtonState {
        checked: checked_of(props),
        disabled: props.disabled.get(),
        invalid: props.invalid.get(),
        variant: props.variant.get().or(props.config_variant),
        size: props.size.get(),
    }
}

fn data_of(props: &Props) -> String {
    data_p(&state_of(props))
}

fn root_class(props: &Props, pt: &RadioButtonPt) -> String {
    let mut cn = ClassNames::new();
    write_classes(&mut cn, &state_of(props));
    props.style_class.with(|c| cn.add_opt(c.as_deref()));
    cn.add_opt(pt.root.extra_class());
    cn.finish()
}

fn root_style(props: &Props, pt: &RadioButtonPt) -> Option<String> {
    props.style.with(|st| (st.is_some() || pt.root.has_style()).then(|| pt.root.merge_style(st.as_deref().unwrap_or(""))))
}

/// Свойства экземпляра.
struct Props {
    value: OptProp<String>,
    model: Option<RwSignal<Option<String>>>,
    default_checked: bool,
    input_id: OptProp<String>,
    name: OptProp<String>,
    tabindex: OptProp<i32>,
    aria_label: OptProp<String>,
    aria_labelledby: OptProp<String>,
    disabled: Prop<bool>,
    required: Prop<bool>,
    invalid: Prop<bool>,
    size: OptProp<Size>,
    variant: OptProp<Variant>,
    style: OptProp<String>,
    style_class: OptProp<String>,
    config_variant: Option<Variant>,
}
