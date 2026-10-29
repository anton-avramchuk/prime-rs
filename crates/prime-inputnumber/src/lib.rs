//! InputNumber — порт `p-inputnumber` из PrimeNG 21.1.10.
//!
//! Разметка и классы как в PrimeNG: корень `span.p-inputnumber`, внутри `input[role=spinbutton]` (классы `InputText`), значок
//! очистки и кнопки «больше/меньше» в трёх раскладках. Число форматируется по малой таблице локалей и валют (вместо
//! `Intl.NumberFormat`), поэтому текст одинаков при SSR и в браузере.
//!
//! Значение — двусторонний сигнал `value: RwSignal<Option<f64>>`; без него компонент ведёт внутреннее состояние
//! (`default_value`). Ввод идёт через `beforeinput`: принимаются только цифры, минус и десятичный разделитель, текст
//! переформатируется на лету с сохранением курсора, а `Enter`, `Tab` и потеря фокуса фиксируют значение с ограничением
//! `min..max`.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_inputnumber::{InputNumber, InputNumberMode};
//!
//! let value = RwSignal::new(Some(1234.5));
//! let html = view! { <InputNumber value=value mode=InputNumberMode::Currency currency="USD" /> }.to_html();
//! assert!(html.contains("$1,234.50") && html.contains(r#"role="spinbutton""#));
//! ```

mod edit;
mod format;
mod locale;
mod state;

use leptos::attr::any_attribute::{AnyAttribute, IntoAnyAttribute};
use leptos::attr::custom::custom_attribute;
use leptos::either::Either;
use leptos::html;
use leptos::leptos_dom::helpers::TimeoutHandle;
use leptos::prelude::*;
use prime_core::{ClassNames, OptProp, Prop, Pt, Size, Variant, use_prime_config};
use prime_icons::{AngleDownIcon, AngleUpIcon, TimesIcon};
use prime_inputtext::{InputTextState, write_classes as input_classes};
use std::sync::Arc;
use std::time::Duration;

pub use edit::{
    Edit, EditCtx, clamp_caret, commit, delete_back, delete_forward, delete_range, insert_char, insert_text, paste, spin, validate,
};
pub use format::{CurrencyDisplay, FormatOptions, InputNumberMode, NumberFormat, Parsed, Parts, default_format};
pub use locale::{Currency, DEFAULT_LOCALE, Grouping, LocaleData, resolve_locale};
pub use state::{ButtonLayout, InputNumberState, data_p, write_classes};

/// Задержка перед повтором шага при удержании кнопки, мс.
const REPEAT_DELAY_MS: u64 = 500;
/// Интервал повторения шага, мс.
const REPEAT_INTERVAL_MS: u64 = 40;

/// Событие ввода: исходное событие, значение и текст поля.
#[derive(Clone, Debug)]
pub struct InputNumberInput {
    /// Исходное событие.
    pub original_event: web_sys::Event,
    /// Новое значение (`None` — пусто).
    pub value: Option<f64>,
    /// Текст поля после правки.
    pub formatted_value: String,
}

/// Pass-through частей InputNumber.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct InputNumberPt {
    /// Корневой элемент.
    pub root: Pt,
    /// Поле ввода.
    pub input: Pt,
    /// Значок очистки.
    pub clear_icon: Pt,
    /// Группа кнопок (раскладка `Stacked`).
    pub button_group: Pt,
    /// Кнопка увеличения.
    pub increment_button: Pt,
    /// Значок кнопки увеличения.
    pub increment_button_icon: Pt,
    /// Кнопка уменьшения.
    pub decrement_button: Pt,
    /// Значок кнопки уменьшения.
    pub decrement_button_icon: Pt,
}

/// Поле числового ввода.
///
/// Каждое свойство принимает значение или сигнал; `value` — двусторонний сигнал.
#[component]
pub fn InputNumber(
    /// Двусторонний сигнал значения.
    #[prop(optional)]
    value: Option<RwSignal<Option<f64>>>,
    /// Начальное значение, если `value` не передан.
    #[prop(optional)]
    default_value: Option<f64>,
    /// Режим форматирования.
    #[prop(optional, into)]
    mode: Prop<InputNumberMode>,
    /// Код валюты (режим `Currency`).
    #[prop(optional, into)]
    currency: OptProp<String>,
    /// Отображение валюты.
    #[prop(optional, into)]
    currency_display: Prop<CurrencyDisplay>,
    /// Локаль (по умолчанию `en-US`).
    #[prop(optional, into)]
    locale: OptProp<String>,
    /// Группировать разряды (по умолчанию `true`).
    #[prop(optional, into)]
    use_grouping: OptProp<bool>,
    /// Минимум знаков после запятой.
    #[prop(optional, into)]
    min_fraction_digits: OptProp<u32>,
    /// Максимум знаков после запятой.
    #[prop(optional, into)]
    max_fraction_digits: OptProp<u32>,
    /// Форматировать число (по умолчанию `true`).
    #[prop(optional, into)]
    format: OptProp<bool>,
    /// Текст перед числом.
    #[prop(optional, into)]
    prefix: OptProp<String>,
    /// Текст после числа.
    #[prop(optional, into)]
    suffix: OptProp<String>,
    /// Нижняя граница.
    #[prop(optional, into)]
    min: OptProp<f64>,
    /// Верхняя граница.
    #[prop(optional, into)]
    max: OptProp<f64>,
    /// Шаг стрелок и кнопок (по умолчанию 1).
    #[prop(optional, into)]
    step: OptProp<f64>,
    /// Разрешено ли пустое значение (по умолчанию `true`).
    #[prop(optional, into)]
    allow_empty: OptProp<bool>,
    /// Максимальная длина текста.
    #[prop(optional, into)]
    max_length: OptProp<usize>,
    /// Показывать кнопки.
    #[prop(optional, into)]
    show_buttons: Prop<bool>,
    /// Раскладка кнопок.
    #[prop(optional, into)]
    button_layout: Prop<ButtonLayout>,
    /// Показывать значок очистки.
    #[prop(optional, into)]
    show_clear: Prop<bool>,
    /// Только чтение.
    #[prop(optional, into)]
    readonly: Prop<bool>,
    /// Недоступно.
    #[prop(optional, into)]
    disabled: Prop<bool>,
    /// Недопустимое значение.
    #[prop(optional, into)]
    invalid: Prop<bool>,
    /// Обязательно.
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
    /// Подсказка в пустом поле.
    #[prop(optional, into)]
    placeholder: OptProp<String>,
    /// Атрибут `id` поля.
    #[prop(optional, into)]
    input_id: OptProp<String>,
    /// Атрибут `name`.
    #[prop(optional, into)]
    name: OptProp<String>,
    /// Атрибут `title`.
    #[prop(optional, into)]
    title: OptProp<String>,
    /// Атрибут `autocomplete`.
    #[prop(optional, into)]
    autocomplete: OptProp<String>,
    /// Атрибут `tabindex`.
    #[prop(optional, into)]
    tabindex: OptProp<i32>,
    /// Атрибут `aria-label`.
    #[prop(optional, into)]
    aria_label: OptProp<String>,
    /// Атрибут `aria-labelledby`.
    #[prop(optional, into)]
    aria_labelledby: OptProp<String>,
    /// Атрибут `aria-describedby`.
    #[prop(optional, into)]
    aria_describedby: OptProp<String>,
    /// Атрибут `aria-required`.
    #[prop(optional, into)]
    aria_required: Prop<bool>,
    /// Встроенный стиль поля.
    #[prop(optional, into)]
    input_style: OptProp<String>,
    /// Дополнительный класс поля.
    #[prop(optional, into)]
    input_style_class: OptProp<String>,
    /// Встроенный стиль корня.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Дополнительный класс корня.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Дополнительный класс кнопки увеличения.
    #[prop(optional, into)]
    increment_button_class: OptProp<String>,
    /// Дополнительный класс кнопки уменьшения.
    #[prop(optional, into)]
    decrement_button_class: OptProp<String>,
    /// Значок кнопки увеличения вместо стрелки вверх.
    #[prop(optional, into)]
    increment_button_icon: Option<ViewFn>,
    /// Значок кнопки уменьшения вместо стрелки вниз.
    #[prop(optional, into)]
    decrement_button_icon: Option<ViewFn>,
    /// Значок очистки вместо крестика.
    #[prop(optional, into)]
    clear_icon: Option<ViewFn>,
    /// Вызывается при изменении значения вводом или шагом.
    #[prop(optional, into)]
    on_input: Option<Callback<InputNumberInput>>,
    /// Вызывается при получении фокуса.
    #[prop(optional, into)]
    on_focus: Option<Callback<web_sys::FocusEvent>>,
    /// Вызывается при потере фокуса.
    #[prop(optional, into)]
    on_blur: Option<Callback<web_sys::FocusEvent>>,
    /// Вызывается при нажатии клавиши (без `Shift` и `Alt`).
    #[prop(optional, into)]
    on_key_down: Option<Callback<web_sys::KeyboardEvent>>,
    /// Вызывается при очистке значком.
    #[prop(optional, into)]
    on_clear: Option<Callback<()>>,
    /// Pass-through частей.
    #[prop(optional)]
    pt: Option<InputNumberPt>,
) -> impl IntoView {
    let pt = Arc::new(pt.unwrap_or_default());
    let value = value.unwrap_or_else(|| RwSignal::new(default_value));
    let props = Arc::new(Props {
        value,
        display: RwSignal::new(String::new()),
        focused: RwSignal::new(false),
        mode,
        currency,
        currency_display,
        locale,
        use_grouping,
        min_fraction_digits,
        max_fraction_digits,
        format,
        prefix,
        suffix,
        min,
        max,
        step,
        allow_empty,
        max_length,
        show_buttons,
        button_layout,
        show_clear,
        readonly,
        disabled,
        invalid,
        required,
        variant,
        config_variant: use_prime_config().input_variant,
        size,
        fluid,
        placeholder,
        input_id,
        name,
        title,
        autocomplete,
        tabindex,
        aria_label,
        aria_labelledby,
        aria_describedby,
        aria_required,
        input_style,
        input_style_class,
        style,
        style_class,
        increment_button_class,
        decrement_button_class,
        on_input,
        on_focus,
        on_blur,
        on_key_down,
        on_clear,
        input_ref: NodeRef::new(),
        timer: StoredValue::new_local(None),
        last_options: StoredValue::new(None),
    });
    props.display.set(untrack(|| props.text_for(value.get_untracked())));
    // Таймер повтора шага живёт только во время удержания кнопки; при размонтировании снимается.
    {
        let props = props.clone();
        on_cleanup(move || props.clear_timer());
    }
    // Внешнее изменение значения или параметров формата переформатирует текст; набираемый текст («1.») не трогается.
    #[cfg(target_arch = "wasm32")]
    {
        let props = props.clone();
        Effect::new(move |_| {
            let options = props.format_options();
            let (v, empty_ok) = (props.value.get(), props.allow_empty.get().unwrap_or(true));
            let previous = props.last_options.get_value();
            props.last_options.set_value(Some((options.clone(), empty_ok)));
            let current = NumberFormat::new(&options).parse(&props.display.get_untracked()).value();
            if previous != Some((options, empty_ok)) || current != v {
                props.set_text(props.text_for(v));
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
    let data_input = data.clone();
    let input_class = {
        let (props, pt) = (props.clone(), pt.clone());
        move || {
            let mut cn = ClassNames::new();
            input_classes(
                &mut cn,
                &InputTextState {
                    filled: !props.display.get().is_empty(),
                    size: props.size.get(),
                    invalid: props.invalid.get(),
                    variant: props.variant.get().or(props.config_variant),
                    fluid: props.fluid.get(),
                },
            );
            cn.add("p-inputnumber-input");
            props.input_style_class.with(|c| cn.add_user(c.as_deref()));
            cn.add_user(pt.input.extra_class());
            cn.finish_cow()
        }
    };
    let input_style = {
        let (props, pt) = (props.clone(), pt.clone());
        move || {
            let style = props.input_style.with(|st| pt.input.merge_style(st.as_deref().unwrap_or_default()));
            (!style.is_empty()).then_some(style)
        }
    };
    let display_attr = {
        let props = props.clone();
        move || props.display.get()
    };
    let display_prop = display_attr.clone();
    // У элемента не больше 26 атрибутов: необязательные собираются в один набор.
    macro_rules! attr {
        ($name:literal, |$p:ident| $body:expr) => {{
            let $p = props.clone();
            reactive_attr($name, move || $body)
        }};
    }
    let extra_attrs: Vec<AnyAttribute> = vec![
        attr!("id", |p| p.input_id.get()),
        attr!("placeholder", |p| p.placeholder.get()),
        attr!("title", |p| p.title.get()),
        attr!("name", |p| p.name.get()),
        attr!("autocomplete", |p| p.autocomplete.get()),
        attr!("tabindex", |p| p.tabindex.get().map(|v| v.to_string())),
        attr!("maxlength", |p| p.max_length.get().map(|v| v.to_string())),
        attr!("aria-label", |p| p.aria_label.get()),
        attr!("aria-labelledby", |p| p.aria_labelledby.get()),
        attr!("aria-describedby", |p| p.aria_describedby.get()),
        attr!("aria-required", |p| p.aria_required.get().then(|| "true".to_owned())),
        attr!("required", |p| p.required.get().then(String::new)),
        attr!("readonly", |p| p.readonly.get().then(String::new)),
        attr!("disabled", |p| p.disabled.get().then(String::new)),
        attr!("aria-valuemin", |p| p.min.get().map(|v| v.to_string())),
        attr!("aria-valuemax", |p| p.max.get().map(|v| v.to_string())),
        attr!("aria-valuenow", |p| p.value.get().map(|v| v.to_string())),
        attr!("min", |p| p.min.get().map(|v| v.to_string())),
        attr!("max", |p| p.max.get().map(|v| v.to_string())),
        attr!("step", |p| Some(p.step.get().unwrap_or(1.0).to_string())),
    ];
    let before_input = {
        let props = props.clone();
        move |e: web_sys::InputEvent| props.before_input(e)
    };
    let key_down = {
        let props = props.clone();
        move |e: web_sys::KeyboardEvent| props.key_down(e)
    };
    let click = {
        let props = props.clone();
        move |_: web_sys::MouseEvent| props.click()
    };
    let focus = {
        let props = props.clone();
        move |e: web_sys::FocusEvent| props.focus(e)
    };
    let blur = {
        let props = props.clone();
        move |e: web_sys::FocusEvent| props.blur(e)
    };
    let input_attrs = pt.input.attrs();
    let input_ref = props.input_ref;

    // Значок очистки зависит от значения и раскладки; кнопки — только от `show_buttons` и раскладки.
    let clear = {
        let (props, pt) = (props.clone(), pt.clone());
        move || {
            let show = props.show_clear.get() && props.button_layout.get() != ButtonLayout::Vertical && props.value.get().is_some();
            show.then(|| clear_view(&props, &pt, clear_icon.clone()))
        }
    };
    let buttons = {
        let (props, pt) = (props.clone(), pt.clone());
        move || {
            if !props.show_buttons.get() {
                return None;
            }
            let up = button_view(&props, &pt, true, increment_button_icon.clone());
            let down = button_view(&props, &pt, false, decrement_button_icon.clone());
            Some(if props.button_layout.get() == ButtonLayout::Stacked {
                let group_class = pt.button_group.merge_class("p-inputnumber-button-group");
                let data = {
                    let props = props.clone();
                    move || data_p(&props.state())
                };
                let attrs = pt.button_group.attrs();
                Either::Left(view! { <span class={group_class} data-p={data} {..attrs}>{up}{down}</span> })
            } else {
                Either::Right(view! { {up}{down} })
            })
        }
    };
    let root_attrs = pt.root.attrs();

    view! {
        <span class={root_class} style={root_style} data-p={data} {..root_attrs}>
            <input
                node_ref={input_ref}
                role="spinbutton"
                class={input_class}
                style={input_style}
                value={display_attr}
                prop:value={display_prop}
                inputmode="decimal"
                data-p={data_input}
                {..extra_attrs}
                on:beforeinput={before_input}
                on:keydown={key_down}
                on:click={click}
                on:focus={focus}
                on:blur={blur}
                {..input_attrs}
            />
            {clear}
            {buttons}
        </span>
    }
}

/// Реактивный атрибут с произвольным именем; `None` — атрибута нет.
fn reactive_attr(name: &'static str, f: impl Fn() -> Option<String> + Send + Sync + 'static) -> AnyAttribute {
    custom_attribute(name, f).into_any_attr()
}

/// Значок очистки.
fn clear_view(props: &Arc<Props>, pt: &Arc<InputNumberPt>, icon: Option<ViewFn>) -> impl IntoView + use<> {
    let class = pt.clear_icon.merge_class("p-inputnumber-clear-icon");
    let attrs = pt.clear_icon.attrs();
    let click = {
        let props = props.clone();
        move |_: web_sys::MouseEvent| props.clear()
    };
    match icon {
        Some(icon) => Either::Left(view! { <span class={class} on:click={click} {..attrs}>{icon.run()}</span> }),
        None => Either::Right(view! { <TimesIcon class={class.into_owned()} on:click={click} {..attrs} /> }),
    }
}

/// Кнопка увеличения (`up`) или уменьшения.
fn button_view(props: &Arc<Props>, pt: &Arc<InputNumberPt>, up: bool, icon: Option<ViewFn>) -> impl IntoView + use<> {
    let (button_pt, icon_pt, user_class) = if up {
        (&pt.increment_button, &pt.increment_button_icon, props.increment_button_class.clone())
    } else {
        (&pt.decrement_button, &pt.decrement_button_icon, props.decrement_button_class.clone())
    };
    let base =
        if up { "p-inputnumber-button p-inputnumber-increment-button" } else { "p-inputnumber-button p-inputnumber-decrement-button" };
    let class = {
        let button_pt = button_pt.clone();
        move || {
            let mut cn = ClassNames::new();
            cn.add(base);
            user_class.with(|c| cn.add_user(c.as_deref()));
            cn.add_user(button_pt.extra_class());
            cn.finish_cow()
        }
    };
    let dir = if up { 1.0 } else { -1.0 };
    let disabled = {
        let props = props.clone();
        move || props.disabled.get().then_some("")
    };
    let data = {
        let props = props.clone();
        move || data_p(&props.state())
    };
    let down = {
        let props = props.clone();
        move |e: web_sys::MouseEvent| {
            e.prevent_default();
            props.repeat(dir, REPEAT_DELAY_MS, e.into());
        }
    };
    let stop = {
        let props = props.clone();
        move |_: web_sys::MouseEvent| props.clear_timer()
    };
    let stop_leave = stop.clone();
    let key_down = {
        let props = props.clone();
        move |e: web_sys::KeyboardEvent| {
            if matches!(e.code().as_str(), "Enter" | "Space") {
                props.repeat(dir, REPEAT_DELAY_MS, e.into());
            }
        }
    };
    let key_up = {
        let props = props.clone();
        move |_: web_sys::KeyboardEvent| props.clear_timer()
    };
    let (button_attrs, icon_attrs) = (button_pt.attrs(), icon_pt.attrs());
    let content = match icon {
        Some(icon) => Either::Left(icon.run()),
        None if up => Either::Right(Either::Left(view! { <AngleUpIcon {..icon_attrs} /> })),
        None => Either::Right(Either::Right(view! { <AngleDownIcon {..icon_attrs} /> })),
    };
    view! {
        <button
            type="button"
            class={class}
            disabled={disabled}
            tabindex="-1"
            aria-hidden="true"
            data-p={data}
            on:mousedown={down}
            on:mouseup={stop}
            on:mouseleave={stop_leave}
            on:keydown={key_down}
            on:keyup={key_up}
            {..button_attrs}
        >
            {content}
        </button>
    }
}

/// Свойства и состояние экземпляра.
struct Props {
    value: RwSignal<Option<f64>>,
    /// Текст в поле: единственный источник отображаемого значения.
    display: RwSignal<String>,
    focused: RwSignal<bool>,
    mode: Prop<InputNumberMode>,
    currency: OptProp<String>,
    currency_display: Prop<CurrencyDisplay>,
    locale: OptProp<String>,
    use_grouping: OptProp<bool>,
    min_fraction_digits: OptProp<u32>,
    max_fraction_digits: OptProp<u32>,
    format: OptProp<bool>,
    prefix: OptProp<String>,
    suffix: OptProp<String>,
    min: OptProp<f64>,
    max: OptProp<f64>,
    step: OptProp<f64>,
    allow_empty: OptProp<bool>,
    max_length: OptProp<usize>,
    show_buttons: Prop<bool>,
    button_layout: Prop<ButtonLayout>,
    show_clear: Prop<bool>,
    readonly: Prop<bool>,
    disabled: Prop<bool>,
    invalid: Prop<bool>,
    required: Prop<bool>,
    variant: OptProp<Variant>,
    config_variant: Option<Variant>,
    size: OptProp<Size>,
    fluid: Prop<bool>,
    placeholder: OptProp<String>,
    input_id: OptProp<String>,
    name: OptProp<String>,
    title: OptProp<String>,
    autocomplete: OptProp<String>,
    tabindex: OptProp<i32>,
    aria_label: OptProp<String>,
    aria_labelledby: OptProp<String>,
    aria_describedby: OptProp<String>,
    aria_required: Prop<bool>,
    input_style: OptProp<String>,
    input_style_class: OptProp<String>,
    style: OptProp<String>,
    style_class: OptProp<String>,
    increment_button_class: OptProp<String>,
    decrement_button_class: OptProp<String>,
    on_input: Option<Callback<InputNumberInput>>,
    on_focus: Option<Callback<web_sys::FocusEvent>>,
    on_blur: Option<Callback<web_sys::FocusEvent>>,
    on_key_down: Option<Callback<web_sys::KeyboardEvent>>,
    on_clear: Option<Callback<()>>,
    input_ref: NodeRef<html::Input>,
    /// Таймер повтора шага при удержании кнопки.
    timer: StoredValue<Option<TimeoutHandle>, LocalStorage>,
    /// Параметры формата и `allow_empty` при последней синхронизации текста с внешним значением (только в браузере).
    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    last_options: StoredValue<Option<(FormatOptions, bool)>>,
}

impl Props {
    /// Параметры формата (с подпиской на сигналы).
    fn format_options(&self) -> FormatOptions {
        FormatOptions {
            locale: self.locale.get().unwrap_or_default(),
            mode: self.mode.get(),
            currency: self.currency.get().unwrap_or_default(),
            currency_display: self.currency_display.get(),
            use_grouping: self.use_grouping.get().unwrap_or(true),
            min_fraction_digits: self.min_fraction_digits.get(),
            max_fraction_digits: self.max_fraction_digits.get(),
            prefix: self.prefix.get().unwrap_or_default(),
            suffix: self.suffix.get().unwrap_or_default(),
            format: self.format.get().unwrap_or(true),
        }
    }

    /// Выполняет `f` с форматтером и контекстом правки (без подписок).
    fn with_ctx<R>(&self, f: impl FnOnce(&EditCtx) -> R) -> R {
        untrack(|| {
            let nf = NumberFormat::new(&self.format_options());
            let ctx = EditCtx {
                nf: &nf,
                min: self.min.get(),
                max: self.max.get(),
                allow_empty: self.allow_empty.get().unwrap_or(true),
                mode: self.mode.get(),
            };
            f(&ctx)
        })
    }

    /// Текст для значения: пусто или `0` (при `allow_empty = false`) для `None`.
    fn text_for(&self, value: Option<f64>) -> String {
        let nf = NumberFormat::new(&self.format_options());
        match value {
            Some(v) => nf.format(v),
            None if self.allow_empty.get().unwrap_or(true) => String::new(),
            None => nf.format(0.0),
        }
    }

    fn state(&self) -> InputNumberState {
        InputNumberState {
            filled: !self.display.get().is_empty(),
            always_filled: self.allow_empty.get() == Some(false),
            focused: self.focused.get(),
            show_buttons: self.show_buttons.get(),
            layout: self.button_layout.get(),
            fluid: self.fluid.get(),
            invalid: self.invalid.get(),
            disabled: self.disabled.get(),
            variant_filled: self.variant.get().or(self.config_variant) == Some(Variant::Filled),
            size: self.size.get(),
        }
    }

    /// Только чтение или недоступно: ввод, шаги и очистка заблокированы.
    fn blocked(&self) -> bool {
        self.readonly.get_untracked() || self.disabled.get_untracked()
    }

    /// Записывает текст в поле и в `display` (позиция курсора не меняется).
    fn set_text(&self, text: String) {
        if let Some(input) = self.input_ref.get_untracked() {
            input.set_value(&text);
        }
        self.display.set(text);
    }

    /// Применяет правку: текст, курсор, значение и `on_input`.
    fn apply_edit(&self, edit: Edit, event: web_sys::Event) {
        if let Some(max) = self.max_length.get_untracked()
            && edit.text.chars().count() > max
        {
            return;
        }
        if let Some(input) = self.input_ref.get_untracked() {
            input.set_value(&edit.text);
            let caret = edit.caret as u32;
            let _ = input.set_selection_range(caret, caret);
        }
        self.display.set(edit.text.clone());
        let value = edit.value.value();
        if self.value.get_untracked() != value {
            self.value.set(value);
            if let Some(cb) = self.on_input {
                cb.run(InputNumberInput { original_event: event, value, formatted_value: edit.text });
            }
        }
    }

    fn selection(&self) -> Option<(String, (usize, usize))> {
        let input = self.input_ref.get_untracked()?;
        let (start, end) = (input.selection_start().ok().flatten()?, input.selection_end().ok().flatten()?);
        Some((input.value(), (start as usize, end as usize)))
    }

    /// `beforeinput`: набор, удаление и вставка обрабатываются здесь, браузерная правка отменяется.
    fn before_input(&self, e: web_sys::InputEvent) {
        if self.blocked() {
            return;
        }
        let kind = e.input_type();
        if kind.contains("omposition") {
            return;
        }
        if e.cancelable() {
            e.prevent_default();
        }
        let Some((text, sel)) = self.selection() else { return };
        let data = e.data().or_else(|| e.data_transfer().and_then(|t| t.get_data("text").ok()));
        let edit = self.with_ctx(|ctx| match kind.as_str() {
            "insertText" | "insertReplacementText" => data.as_deref().and_then(|d| insert_text(ctx, &text, sel, d)),
            "insertFromPaste" | "insertFromDrop" | "insertFromYank" => data.as_deref().and_then(|d| paste(ctx, &text, sel, d)),
            k if k.starts_with("delete") && k.ends_with("Backward") => delete_back(ctx, &text, sel),
            k if k.starts_with("delete") && k.ends_with("Forward") => delete_forward(ctx, &text, sel),
            k if k.starts_with("delete") && sel.0 != sel.1 => delete_range(ctx, &text, sel),
            _ => None,
        });
        if let Some(edit) = edit {
            self.apply_edit(edit, e.into());
        }
    }

    fn key_down(&self, e: web_sys::KeyboardEvent) {
        if self.blocked() || e.shift_key() || e.alt_key() {
            return;
        }
        match e.key().as_str() {
            "ArrowUp" => {
                self.spin(1.0, e.clone().into());
                e.prevent_default();
            }
            "ArrowDown" => {
                self.spin(-1.0, e.clone().into());
                e.prevent_default();
            }
            "Enter" | "Tab" => self.commit_text(),
            "Home" => {
                if let Some(min) = self.min.get_untracked() {
                    self.set_value_text(min);
                    e.prevent_default();
                }
            }
            "End" => {
                if let Some(max) = self.max.get_untracked() {
                    self.set_value_text(max);
                    e.prevent_default();
                }
            }
            _ => {}
        }
        if let Some(cb) = self.on_key_down {
            cb.run(e);
        }
    }

    /// Записывает значение и его текст (без `on_input`).
    fn set_value_text(&self, value: f64) {
        let text = self.with_ctx(|ctx| ctx.nf.format(value));
        self.set_text(text);
        self.value.set(Some(value));
    }

    /// Фиксация: число ограничивается границами, текст форматируется заново.
    fn commit_text(&self) {
        let Some(input) = self.input_ref.get_untracked() else { return };
        let (value, text) = self.with_ctx(|ctx| commit(ctx, &input.value()));
        self.set_text(text);
        if self.value.get_untracked() != value {
            self.value.set(value);
        }
    }

    fn click(&self) {
        if self.blocked() {
            return;
        }
        let Some((text, (start, end))) = self.selection() else { return };
        if start == end {
            let caret = self.with_ctx(|ctx| clamp_caret(ctx.nf, &text, start));
            if caret != start
                && let Some(input) = self.input_ref.get_untracked()
            {
                let _ = input.set_selection_range(caret as u32, caret as u32);
            }
        }
    }

    fn focus(&self, e: web_sys::FocusEvent) {
        self.focused.set(true);
        if let Some(cb) = self.on_focus {
            cb.run(e);
        }
    }

    fn blur(&self, e: web_sys::FocusEvent) {
        self.focused.set(false);
        self.commit_text();
        if let Some(cb) = self.on_blur {
            cb.run(e);
        }
    }

    /// Шаг по стрелке или кнопке (`spin`).
    fn spin(&self, dir: f64, event: web_sys::Event) {
        if self.blocked() {
            return;
        }
        let Some(input) = self.input_ref.get_untracked() else { return };
        let step = self.step.get_untracked().unwrap_or(1.0);
        let (value, text) = self.with_ctx(|ctx| spin(ctx, &input.value(), dir, step));
        if let Some(max) = self.max_length.get_untracked()
            && text.chars().count() > max
        {
            return;
        }
        self.set_text(text.clone());
        let changed = self.value.get_untracked() != Some(value);
        self.value.set(Some(value));
        if changed && let Some(cb) = self.on_input {
            cb.run(InputNumberInput { original_event: event, value: Some(value), formatted_value: text });
        }
    }

    /// Удержание кнопки: первый шаг сразу, затем повтор через `interval` мс, дальше каждые 40 мс.
    fn repeat(self: &Arc<Self>, dir: f64, interval: u64, event: web_sys::Event) {
        if self.blocked() {
            return;
        }
        self.clear_timer();
        let (this, next) = (self.clone(), event.clone());
        let handle = set_timeout_with_handle(move || this.repeat(dir, REPEAT_INTERVAL_MS, next), Duration::from_millis(interval)).ok();
        self.timer.set_value(handle);
        self.spin(dir, event);
    }

    fn clear_timer(&self) {
        if let Some(handle) = self.timer.try_update_value(Option::take).flatten() {
            handle.clear();
        }
    }

    fn clear(&self) {
        let text = untrack(|| self.text_for(None));
        self.set_text(text);
        self.value.set(None);
        if let Some(cb) = self.on_clear {
            cb.run(());
        }
    }
}
