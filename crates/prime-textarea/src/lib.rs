//! Textarea — порт директивы `pTextarea` из PrimeNG 21.1.10.
//!
//! В PrimeNG это директива на нативном `<textarea>`; здесь компонент отрисовывает `<textarea>` с теми же классами
//! (`p-textarea p-component`, `p-filled`, `p-textarea-resizable`, `p-variant-filled`, `p-textarea-fluid`, размеры, `p-invalid`).
//!
//! Значение — необязательное двустороннее `value: RwSignal<String>`: при SSR оно выводится дочерним текстом `<textarea>`, на клиенте
//! обновляется через `prop:value`. Без `value` поле неконтролируемое, а `p-filled` не отслеживается.
//!
//! `auto_resize` подгоняет высоту по `scrollHeight` (с учётом встроенного `max-height`) в браузере; `on_resize` вызывается после подгонки.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_textarea::Textarea;
//!
//! let value = RwSignal::new("line".to_owned());
//! let html = view! { <Textarea value=value rows=3u32 auto_resize=true /> }.to_html();
//! assert!(html.contains("p-textarea-resizable") && html.contains("p-filled") && html.contains(r#"rows="3""#) && html.contains(">line</textarea>"));
//! ```

mod state;

use leptos::either::Either;
use leptos::prelude::*;
use prime_core::{ClassNames, OptProp, Prop, Pt, Size, Variant, all_static, use_prime_config};
use std::borrow::Cow;
use std::sync::Arc;

pub use state::{Fit, TextareaState, fit_height, write_classes};

/// Подгонка высоты по содержимому (только в браузере).
#[cfg(target_arch = "wasm32")]
fn resize(el: &web_sys::HtmlTextAreaElement, on_resize: Option<Callback<Option<web_sys::Event>>>, event: Option<web_sys::Event>) {
    let style = web_sys::HtmlElement::style(AsRef::<web_sys::HtmlElement>::as_ref(el));
    let _ = style.set_property("height", "auto");
    let max = style.get_property_value("max-height").ok().and_then(|v| v.trim().trim_end_matches("px").parse::<f64>().ok());
    let fit = fit_height(f64::from(el.scroll_height()), max);
    let _ = style.set_property("height", &format!("{}px", fit.height));
    if fit.scroll {
        let _ = style.set_property("overflow-y", "scroll");
    } else {
        let _ = style.set_property("overflow", "hidden");
    }
    if let Some(cb) = on_resize {
        cb.run(event);
    }
}

/// Многострочное поле ввода.
///
/// Каждое свойство принимает значение или сигнал. `value` — двусторонняя привязка (`RwSignal<String>`).
#[component]
pub fn Textarea(
    /// Двусторонне связанное значение; без него поле неконтролируемое.
    #[prop(optional)]
    value: Option<RwSignal<String>>,
    /// Атрибут `id`.
    #[prop(optional, into)]
    id: OptProp<String>,
    /// Атрибут `name`.
    #[prop(optional, into)]
    name: OptProp<String>,
    /// Подсказка в пустом поле.
    #[prop(optional, into)]
    placeholder: OptProp<String>,
    /// Число строк (`rows`).
    #[prop(optional, into)]
    rows: OptProp<u32>,
    /// Число столбцов (`cols`).
    #[prop(optional, into)]
    cols: OptProp<u32>,
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
    /// Автоматическая подгонка высоты по содержимому.
    #[prop(optional)]
    auto_resize: bool,
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
    /// Вызывается при фиксации значения (потеря фокуса).
    #[prop(optional, into)]
    on_change: Option<Callback<web_sys::Event>>,
    /// Вызывается при получении фокуса.
    #[prop(optional, into)]
    on_focus: Option<Callback<web_sys::FocusEvent>>,
    /// Вызывается при потере фокуса.
    #[prop(optional, into)]
    on_blur: Option<Callback<web_sys::FocusEvent>>,
    /// Вызывается после подгонки высоты (`auto_resize`); событие — если подгонка вызвана вводом.
    #[prop(optional, into)]
    on_resize: Option<Callback<Option<web_sys::Event>>>,
    /// Pass-through корня.
    #[prop(optional)]
    pt: Option<Pt>,
) -> impl IntoView {
    let pt = pt.unwrap_or_default();
    let is_static = all_static!(
        id,
        name,
        placeholder,
        rows,
        cols,
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
    let props = Props {
        value,
        auto_resize,
        size,
        variant,
        invalid,
        fluid,
        style,
        style_class,
        id,
        name,
        placeholder,
        rows,
        cols,
        aria_label,
        aria_describedby,
        disabled,
        readonly,
        required,
        maxlength,
        config_variant: use_prime_config().input_variant,
    };
    let attrs = pt.attrs();

    // `NodeRef` и `Effect` нужны только клиенту с `auto_resize`; при SSR их нет вовсе.
    #[cfg(target_arch = "wasm32")]
    let node = {
        let node = NodeRef::<leptos::html::Textarea>::new();
        if auto_resize {
            Effect::new(move |_| {
                if let Some(v) = value {
                    v.track();
                }
                if let Some(el) = node.get() {
                    resize(&el, on_resize, None);
                }
            });
        }
        leptos::tachys::html::node_ref::node_ref::<leptos::html::Textarea, _>(node)
    };
    #[cfg(not(target_arch = "wasm32"))]
    #[allow(clippy::let_unit_value)]
    let node = {
        let _ = on_resize;
    };

    let input = move |e: web_sys::Event| {
        if let Some(sig) = value {
            sig.set(event_target_value(&e));
        } else {
            #[cfg(target_arch = "wasm32")]
            if auto_resize {
                resize(&event_target::<web_sys::HtmlTextAreaElement>(&e), on_resize, Some(e.clone()));
            }
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
        let (class, style) = (class_of(&props, &pt), style_of(&props, &pt));
        let (id, name, placeholder, rows, cols) =
            (props.id.get(), props.name.get(), props.placeholder.get(), props.rows.get(), props.cols.get());
        let (aria_label, aria_describedby) = (props.aria_label.get(), props.aria_describedby.get());
        let (disabled, readonly, required, maxlength) =
            (props.disabled.get(), props.readonly.get(), props.required.get(), props.maxlength.get());
        let (current, initial) = (None::<String>, None::<String>);
        Either::Left(textarea_markup!(
            class,
            style,
            id,
            name,
            placeholder,
            rows,
            cols,
            aria_label,
            aria_describedby,
            disabled,
            readonly,
            required,
            maxlength,
            current,
            initial,
            node,
            input,
            change,
            focus,
            blur,
            attrs
        ))
    } else {
        let (props, pt) = (Arc::new(props), Arc::new(pt));
        macro_rules! reactive {
            ($($field:ident),+) => {
                $(let $field = { let props = props.clone(); move || props.$field.get() };)+
            };
        }
        reactive!(id, name, placeholder, rows, cols, aria_label, aria_describedby, disabled, readonly, required, maxlength);
        let class = {
            let (props, pt) = (props.clone(), pt.clone());
            move || class_of(&props, &pt)
        };
        let style = {
            let (props, pt) = (props.clone(), pt.clone());
            move || style_of(&props, &pt)
        };
        let initial = value.map(|v| v.with_untracked(|s| initial_text(s)));
        let current = move || props.value.map(|v| v.get());
        Either::Right(textarea_markup!(
            class,
            style,
            id,
            name,
            placeholder,
            rows,
            cols,
            aria_label,
            aria_describedby,
            disabled,
            readonly,
            required,
            maxlength,
            current,
            initial,
            node,
            input,
            change,
            focus,
            blur,
            attrs
        ))
    }
}

/// Разметка Textarea: одна для статической и реактивной ветвей (меняются только выражения атрибутов).
macro_rules! textarea_markup {
    (
        $class:expr, $style:expr, $id:expr, $name:expr, $placeholder:expr, $rows:expr, $cols:expr, $aria_label:expr, $aria_describedby:expr,
        $disabled:expr, $readonly:expr, $required:expr, $maxlength:expr, $current:expr, $initial:expr, $node:expr, $input:expr, $change:expr,
        $focus:expr, $blur:expr, $attrs:expr
    ) => {
        view! {
            <textarea
                id={$id}
                name={$name}
                placeholder={$placeholder}
                rows={$rows}
                cols={$cols}
                aria-label={$aria_label}
                aria-describedby={$aria_describedby}
                disabled={$disabled}
                readonly={$readonly}
                required={$required}
                maxlength={$maxlength}
                class={$class}
                style={$style}
                prop:value={$current}
                on:input={$input}
                on:change={$change}
                on:focus={$focus}
                on:blur={$blur}
                {..$attrs}
                {..$node}
            >
                {$initial}
            </textarea>
        }
    };
}
use textarea_markup;

/// Начальный текст `<textarea>` для SSR и гидратации.
///
/// Leptos выводит детей `<textarea>` без экранирования (как `<script>`), поэтому `&`, `<` и `>` экранируются здесь, иначе значение вроде
/// `</textarea><script>` ломало бы разметку. Парсер HTML отбрасывает первый перевод строки внутри `<textarea>`, поэтому ведущий
/// перевод строки значения защищается лишним. В чисто клиентской сборке (CSR) текст остаётся как есть: значение поля задаёт `prop:value`.
fn initial_text(value: &str) -> String {
    if cfg!(all(target_arch = "wasm32", not(feature = "hydrate"))) {
        return value.to_owned();
    }
    let mut out = String::with_capacity(value.len() + 8);
    if value.starts_with('\n') || value.starts_with("\r\n") {
        out.push('\n');
    }
    for c in value.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            c => out.push(c),
        }
    }
    out
}

fn state_of(props: &Props) -> TextareaState {
    TextareaState {
        filled: props.value.is_some_and(|v| v.with(|s| !s.is_empty())),
        auto_resize: props.auto_resize,
        variant: props.variant.get().or(props.config_variant),
        fluid: props.fluid.get(),
        size: props.size.get(),
        invalid: props.invalid.get(),
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

/// Свойства экземпляра.
struct Props {
    value: Option<RwSignal<String>>,
    auto_resize: bool,
    size: OptProp<Size>,
    variant: OptProp<Variant>,
    invalid: Prop<bool>,
    fluid: Prop<bool>,
    style: OptProp<String>,
    style_class: OptProp<String>,
    id: OptProp<String>,
    name: OptProp<String>,
    placeholder: OptProp<String>,
    rows: OptProp<u32>,
    cols: OptProp<u32>,
    aria_label: OptProp<String>,
    aria_describedby: OptProp<String>,
    disabled: Prop<bool>,
    readonly: Prop<bool>,
    required: Prop<bool>,
    maxlength: OptProp<i32>,
    config_variant: Option<Variant>,
}
