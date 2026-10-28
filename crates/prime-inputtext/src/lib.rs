//! InputText — порт директивы `pInputText` из PrimeNG 21.1.10.
//!
//! В PrimeNG это директива на нативном `<input>`; здесь компонент отрисовывает `<input>` с теми же классами (`p-inputtext p-component`,
//! `p-filled`, `p-inputtext-sm`/`-lg`, `p-invalid`, `p-variant-filled`, `p-inputtext-fluid`) и `data-p`.
//!
//! Значение — необязательное двустороннее `value: RwSignal<String>`: поле показывает сигнал, ввод пользователя пишется обратно.
//! Без `value` поле неконтролируемое: значение живёт в DOM, а `p-filled` не отслеживается.
//!
//! Если все свойства статические и `value` не задан, компонент рисует готовые значения без замыканий и `RenderEffect`.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_core::Size;
//! use prime_inputtext::InputText;
//!
//! let value = RwSignal::new("abc".to_owned());
//! let html = view! { <InputText value=value placeholder="Name" size=Size::Small invalid=true /> }.to_html();
//! assert!(html.contains("p-inputtext-sm") && html.contains("p-invalid") && html.contains("p-filled") && html.contains(r#"placeholder="Name""#));
//! ```

mod state;

use leptos::either::Either;
use leptos::prelude::*;
use prime_core::{ClassNames, OptProp, Prop, Pt, Size, Variant, all_static, use_prime_config};
use std::borrow::Cow;
use std::sync::Arc;

pub use state::{InputTextState, data_p, write_classes};

/// Поле ввода текста.
///
/// Каждое свойство принимает значение или сигнал. `value` — двусторонняя привязка (`RwSignal<String>`).
#[component]
pub fn InputText(
    /// Двусторонне связанное значение; без него поле неконтролируемое.
    #[prop(optional)]
    value: Option<RwSignal<String>>,
    /// Атрибут `type` (по умолчанию `text`).
    #[prop(optional, into)]
    input_type: OptProp<String>,
    /// Атрибут `id`.
    #[prop(optional, into)]
    id: OptProp<String>,
    /// Атрибут `name`.
    #[prop(optional, into)]
    name: OptProp<String>,
    /// Подсказка в пустом поле.
    #[prop(optional, into)]
    placeholder: OptProp<String>,
    /// Атрибут `autocomplete`.
    #[prop(optional, into)]
    autocomplete: OptProp<String>,
    /// Атрибут `aria-label`.
    #[prop(optional, into)]
    aria_label: OptProp<String>,
    /// Атрибут `aria-describedby`.
    #[prop(optional, into)]
    aria_describedby: OptProp<String>,
    /// Недоступно.
    #[prop(optional, into)]
    disabled: Prop<bool>,
    /// Только для чтения.
    #[prop(optional, into)]
    readonly: Prop<bool>,
    /// Обязательное.
    #[prop(optional, into)]
    required: Prop<bool>,
    /// Максимальная длина (`maxlength`).
    #[prop(optional, into)]
    maxlength: OptProp<i32>,
    /// Размер.
    #[prop(optional, into)]
    size: OptProp<Size>,
    /// Вариант; если не задан, берётся `PrimeConfig.input_variant`.
    #[prop(optional, into)]
    variant: OptProp<Variant>,
    /// Недопустимое значение.
    #[prop(optional, into)]
    invalid: Prop<bool>,
    /// На всю ширину контейнера.
    #[prop(optional, into)]
    fluid: Prop<bool>,
    /// Встроенный стиль.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Дополнительный класс.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Вызывается при вводе, после записи значения в `value`.
    #[prop(optional, into)]
    on_input: Option<Callback<web_sys::Event>>,
    /// Вызывается при фиксации значения (потеря фокуса или Enter).
    #[prop(optional, into)]
    on_change: Option<Callback<web_sys::Event>>,
    /// Вызывается при получении фокуса.
    #[prop(optional, into)]
    on_focus: Option<Callback<web_sys::FocusEvent>>,
    /// Вызывается при потере фокуса.
    #[prop(optional, into)]
    on_blur: Option<Callback<web_sys::FocusEvent>>,
    /// Pass-through корневого элемента.
    #[prop(optional)]
    pt: Option<Pt>,
) -> impl IntoView {
    let pt = pt.unwrap_or_default();
    let is_static = all_static!(
        input_type,
        id,
        name,
        placeholder,
        autocomplete,
        aria_label,
        aria_describedby,
        disabled,
        readonly,
        required,
        maxlength,
        size,
        variant,
        invalid,
        fluid,
        style,
        style_class
    ) && value.is_none();
    let config_variant = use_prime_config().input_variant;
    let props = Arc::new(Props {
        value,
        input_type,
        id,
        name,
        placeholder,
        autocomplete,
        aria_label,
        aria_describedby,
        disabled,
        readonly,
        required,
        maxlength,
        size,
        variant,
        invalid,
        fluid,
        style,
        style_class,
        config_variant,
    });
    let attrs = pt.attrs();

    let input = move |e: web_sys::Event| {
        if let Some(sig) = value {
            sig.set(event_target_value(&e));
        }
        if let Some(cb) = on_input {
            cb.run(e);
        }
    };
    let change = move |e: web_sys::Event| {
        if let Some(cb) = on_change {
            cb.run(e);
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
        let (class, style, data, ty) = (class_of(&props, &pt), style_of(&props, &pt), data_of(&props), type_of(&props));
        let (id, name, placeholder) = (props.id.get(), props.name.get(), props.placeholder.get());
        let (autocomplete, aria_label, aria_describedby) = (props.autocomplete.get(), props.aria_label.get(), props.aria_describedby.get());
        let (disabled, readonly, required, maxlength) =
            (props.disabled.get(), props.readonly.get(), props.required.get(), props.maxlength.get());
        let current = None::<String>;
        Either::Left(input_markup!(
            ty,
            class,
            style,
            data,
            id,
            name,
            placeholder,
            autocomplete,
            aria_label,
            aria_describedby,
            disabled,
            readonly,
            required,
            maxlength,
            current,
            input,
            change,
            focus,
            blur,
            attrs
        ))
    } else {
        let pt = Arc::new(pt);
        macro_rules! reactive {
            ($($field:ident),+) => {
                $(let $field = { let props = props.clone(); move || props.$field.get() };)+
            };
        }
        reactive!(id, name, placeholder, autocomplete, aria_label, aria_describedby, disabled, readonly, required, maxlength);
        let class = {
            let (props, pt) = (props.clone(), pt.clone());
            move || class_of(&props, &pt)
        };
        let style = {
            let (props, pt) = (props.clone(), pt.clone());
            move || style_of(&props, &pt)
        };
        let data = {
            let props = props.clone();
            move || data_of(&props)
        };
        let ty = {
            let props = props.clone();
            move || type_of(&props)
        };
        let current = move || props.value.map(|v| v.get());
        Either::Right(input_markup!(
            ty,
            class,
            style,
            data,
            id,
            name,
            placeholder,
            autocomplete,
            aria_label,
            aria_describedby,
            disabled,
            readonly,
            required,
            maxlength,
            current,
            input,
            change,
            focus,
            blur,
            attrs
        ))
    }
}

/// Разметка InputText: одна для статической и реактивной ветвей (меняются только выражения атрибутов).
macro_rules! input_markup {
    (
        $ty:expr, $class:expr, $style:expr, $data:expr, $id:expr, $name:expr, $placeholder:expr, $autocomplete:expr, $aria_label:expr,
        $aria_describedby:expr, $disabled:expr, $readonly:expr, $required:expr, $maxlength:expr, $current:expr, $input:expr, $change:expr,
        $focus:expr, $blur:expr, $attrs:expr
    ) => {
        view! {
            <input
                type={$ty}
                class={$class}
                style={$style}
                id={$id}
                name={$name}
                placeholder={$placeholder}
                autocomplete={$autocomplete}
                aria-label={$aria_label}
                aria-describedby={$aria_describedby}
                disabled={$disabled}
                readonly={$readonly}
                required={$required}
                maxlength={$maxlength}
                value={$current.clone()}
                prop:value={$current}
                data-p={$data}
                on:input={$input}
                on:change={$change}
                on:focus={$focus}
                on:blur={$blur}
                {..$attrs}
            />
        }
    };
}
use input_markup;

fn state_of(props: &Props) -> InputTextState {
    InputTextState {
        filled: props.value.is_some_and(|v| v.with(|s| !s.is_empty())),
        size: props.size.get(),
        invalid: props.invalid.get(),
        variant: props.variant.get().or(props.config_variant),
        fluid: props.fluid.get(),
    }
}

fn class_of(props: &Props, pt: &Pt) -> Cow<'static, str> {
    let mut cn = ClassNames::new();
    write_classes(&mut cn, &state_of(props));
    props.style_class.with(|c| cn.add_user(c.as_deref()));
    cn.add_user(pt.extra_class());
    cn.finish_cow()
}

fn style_of(props: &Props, pt: &Pt) -> Option<String> {
    props.style.with(|st| (st.is_some() || pt.has_style()).then(|| pt.merge_style(st.as_deref().unwrap_or(""))))
}

fn data_of(props: &Props) -> Cow<'static, str> {
    data_p(&state_of(props))
}

fn type_of(props: &Props) -> Cow<'static, str> {
    props.input_type.with(|t| t.as_deref().map_or(Cow::Borrowed("text"), |s| Cow::Owned(s.to_owned())))
}

/// Свойства экземпляра.
struct Props {
    value: Option<RwSignal<String>>,
    input_type: OptProp<String>,
    id: OptProp<String>,
    name: OptProp<String>,
    placeholder: OptProp<String>,
    autocomplete: OptProp<String>,
    aria_label: OptProp<String>,
    aria_describedby: OptProp<String>,
    disabled: Prop<bool>,
    readonly: Prop<bool>,
    required: Prop<bool>,
    maxlength: OptProp<i32>,
    size: OptProp<Size>,
    variant: OptProp<Variant>,
    invalid: Prop<bool>,
    fluid: Prop<bool>,
    style: OptProp<String>,
    style_class: OptProp<String>,
    config_variant: Option<Variant>,
}
