//! ToggleSwitch — порт `p-toggleswitch` из PrimeNG 21.1.10.
//!
//! Разметка и классы как в PrimeNG: корень `div.p-toggleswitch` (вместо хоста `<p-toggleswitch>`) со `style="position: relative"`,
//! внутри `input.p-toggleswitch-input[role=switch]` и `div.p-toggleswitch-slider` с `div.p-toggleswitch-handle`.
//!
//! Состояние — двусторонний сигнал `checked: RwSignal<bool>`; клик по любой части корня переключает его и вызывает `on_change`.
//! Без `checked` переключатель неконтролируемый: начальное состояние из `default_checked`, дальше его хранит DOM, а классы корня
//! не отслеживаются.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_toggleswitch::ToggleSwitch;
//!
//! let checked = RwSignal::new(true);
//! let html = view! { <ToggleSwitch checked=checked name="notify" /> }.to_html();
//! assert!(html.contains("p-toggleswitch-checked") && html.contains("p-toggleswitch-slider") && html.contains(r#"name="notify""#));
//! ```

mod state;

use leptos::either::Either;
use leptos::html;
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use prime_core::{ClassNames, OptProp, Prop, Pt, all_static};
use std::sync::Arc;

pub use state::{ToggleSwitchState, data_p, write_classes};

/// Событие изменения: новое состояние и исходное событие браузера.
#[derive(Clone, Debug)]
pub struct ToggleSwitchChange {
    /// Новое состояние.
    pub checked: bool,
    /// Исходное событие `click`.
    pub original_event: web_sys::Event,
}

/// Pass-through частей ToggleSwitch.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ToggleSwitchPt {
    /// Корневой элемент.
    pub root: Pt,
    /// Нативный `<input>`.
    pub input: Pt,
    /// Дорожка (`div.p-toggleswitch-slider`).
    pub slider: Pt,
    /// Ручка (`div.p-toggleswitch-handle`).
    pub handle: Pt,
}

/// Переключатель булева значения.
///
/// Каждое свойство принимает значение или сигнал; `checked` — двусторонний сигнал.
#[component]
pub fn ToggleSwitch(
    /// Двусторонний сигнал состояния.
    #[prop(optional)]
    checked: Option<RwSignal<bool>>,
    /// Начальное состояние неконтролируемого переключателя.
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
    /// Недоступен.
    #[prop(optional, into)]
    disabled: Prop<bool>,
    /// Только для чтения.
    #[prop(optional, into)]
    readonly: Prop<bool>,
    /// Обязателен.
    #[prop(optional, into)]
    required: Prop<bool>,
    /// Недопустимое значение.
    #[prop(optional, into)]
    invalid: Prop<bool>,
    /// Встроенный стиль корня (после `position: relative`).
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Дополнительный класс корня.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Содержимое ручки; получает текущее состояние.
    #[prop(optional, into)]
    handle_template: Option<Callback<bool, AnyView>>,
    /// Вызывается после переключения пользователем.
    #[prop(optional, into)]
    on_change: Option<Callback<ToggleSwitchChange>>,
    /// Вызывается при получении фокуса.
    #[prop(optional, into)]
    on_focus: Option<Callback<web_sys::FocusEvent>>,
    /// Вызывается при потере фокуса.
    #[prop(optional, into)]
    on_blur: Option<Callback<web_sys::FocusEvent>>,
    /// Pass-through частей.
    #[prop(optional)]
    pt: Option<ToggleSwitchPt>,
) -> impl IntoView {
    let pt = Arc::new(pt.unwrap_or_default());
    let is_static =
        all_static!(input_id, name, tabindex, aria_label, aria_labelledby, disabled, readonly, required, invalid, style, style_class)
            && checked.is_none();
    let props = Arc::new(Props {
        checked,
        default_checked,
        input_id,
        name,
        tabindex,
        aria_label,
        aria_labelledby,
        disabled,
        readonly,
        required,
        invalid,
        style,
        style_class,
        handle_template,
    });
    let (root_attrs, input_attrs, slider_attrs, handle_attrs) = (pt.root.attrs(), pt.input.attrs(), pt.slider.attrs(), pt.handle.attrs());

    let input_ref = NodeRef::<html::Input>::new();
    let click = {
        let props = props.clone();
        move |e: web_sys::MouseEvent| {
            let Some(input) = input_ref.get() else { return };
            // Клик по самому `<input>` переключил его нативно ещё до обработчика корня; клик по слайдеру нет.
            let via_input = e.target().is_some_and(|t| t.is_instance_of::<web_sys::HtmlInputElement>());
            let before = match props.checked {
                Some(sig) => sig.get_untracked(),
                None if via_input => !input.checked(),
                None => input.checked(),
            };
            if is_blocked(&props) {
                input.set_checked(before);
                return;
            }
            let now = !before;
            if let Some(sig) = props.checked {
                sig.set(now);
            }
            input.set_checked(now);
            if let Some(cb) = on_change {
                cb.run(ToggleSwitchChange { checked: now, original_event: e.into() });
            }
            let _ = input.focus();
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
        let (id, name, tabindex) = (p.input_id.get(), p.name.get(), p.tabindex.get());
        let (required, disabled) = (p.required.get(), p.disabled.get());
        let (aria_label, aria_labelledby) = (p.aria_label.get(), p.aria_labelledby.get());
        let (is_checked, input_class) = (checked_of(p), t.input.merge_class("p-toggleswitch-input"));
        let (slider_class, handle_class) = (t.slider.merge_class("p-toggleswitch-slider"), t.handle.merge_class("p-toggleswitch-handle"));
        let handle = handle_of(p, checked_of(p));
        Either::Left(switch_markup!(
            root_class,
            root_style,
            checked_attr,
            disabled_attr,
            data,
            id,
            name,
            tabindex,
            required,
            disabled,
            aria_label,
            aria_labelledby,
            input_class,
            is_checked,
            slider_class,
            handle_class,
            handle,
            input_ref,
            click,
            focus,
            blur,
            root_attrs,
            input_attrs,
            slider_attrs,
            handle_attrs
        ))
    } else {
        let (p, t) = (&props, &pt);
        let root_class = rx(p, t, root_class);
        let root_style = rx(p, t, root_style);
        let checked_attr = rx(p, t, |p, _| bool_attr(checked_of(p)));
        let disabled_attr = rx(p, t, |p, _| bool_attr(p.disabled.get()));
        let data = rx(p, t, |p, _| data_of(p));
        let id = rx(p, t, |p, _| p.input_id.get());
        let name = rx(p, t, |p, _| p.name.get());
        let tabindex = rx(p, t, |p, _| p.tabindex.get());
        let required = rx(p, t, |p, _| p.required.get());
        let disabled = rx(p, t, |p, _| p.disabled.get());
        let aria_label = rx(p, t, |p, _| p.aria_label.get());
        let aria_labelledby = rx(p, t, |p, _| p.aria_labelledby.get());
        let input_class = rx(p, t, |_, t| t.input.merge_class("p-toggleswitch-input"));
        let is_checked = rx(p, t, |p, _| checked_of(p));
        let slider_class = rx(p, t, |_, t| t.slider.merge_class("p-toggleswitch-slider"));
        let handle_class = rx(p, t, |_, t| t.handle.merge_class("p-toggleswitch-handle"));
        let handle = rx(p, t, |p, _| handle_of(p, checked_of(p)));
        Either::Right(switch_markup!(
            root_class,
            root_style,
            checked_attr,
            disabled_attr,
            data,
            id,
            name,
            tabindex,
            required,
            disabled,
            aria_label,
            aria_labelledby,
            input_class,
            is_checked,
            slider_class,
            handle_class,
            handle,
            input_ref,
            click,
            focus,
            blur,
            root_attrs,
            input_attrs,
            slider_attrs,
            handle_attrs
        ))
    }
}

/// Разметка ToggleSwitch: одна для статической и реактивной ветвей (меняются только выражения атрибутов).
macro_rules! switch_markup {
    (
        $root_class:expr, $root_style:expr, $checked_attr:expr, $disabled_attr:expr, $data:expr, $id:expr, $name:expr, $tabindex:expr,
        $required:expr, $disabled:expr, $aria_label:expr, $aria_labelledby:expr, $input_class:expr, $is_checked:expr, $slider_class:expr,
        $handle_class:expr, $handle:expr, $input_ref:expr, $click:expr, $focus:expr, $blur:expr, $root_attrs:expr, $input_attrs:expr,
        $slider_attrs:expr, $handle_attrs:expr
    ) => {{
        let checked_root = $checked_attr.clone();
        let checked_aria = $checked_attr.clone();
        let data_slider = $data.clone();
        let data_handle = $data.clone();
        view! {
            <div
                class={$root_class}
                style={$root_style}
                data-p-checked={checked_root}
                data-p-disabled={$disabled_attr}
                data-p={$data}
                on:click={$click}
                {..$root_attrs}
            >
                <input
                    node_ref={$input_ref}
                    id={$id}
                    type="checkbox"
                    role="switch"
                    class={$input_class}
                    checked={$is_checked.clone()}
                    prop:checked={$is_checked}
                    required={$required}
                    disabled={$disabled}
                    aria-checked={checked_aria}
                    aria-labelledby={$aria_labelledby}
                    aria-label={$aria_label}
                    name={$name}
                    tabindex={$tabindex}
                    on:focus={$focus}
                    on:blur={$blur}
                    {..$input_attrs}
                />
                <div class={$slider_class} data-p={data_slider} {..$slider_attrs}>
                    <div class={$handle_class} data-p={data_handle} {..$handle_attrs}>{$handle}</div>
                </div>
            </div>
        }
    }};
}
use switch_markup;

/// Замыкание над свойствами экземпляра: клоны `Arc` и указатель на функцию без захвата.
fn rx<T>(
    props: &Arc<Props>,
    pt: &Arc<ToggleSwitchPt>,
    f: fn(&Props, &ToggleSwitchPt) -> T,
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
    props.checked.map_or(props.default_checked, |s| s.get())
}

fn is_blocked(props: &Props) -> bool {
    props.disabled.get() || props.readonly.get()
}

fn state_of(props: &Props) -> ToggleSwitchState {
    ToggleSwitchState { checked: checked_of(props), disabled: props.disabled.get(), invalid: props.invalid.get() }
}

fn data_of(props: &Props) -> String {
    data_p(&state_of(props))
}

fn root_class(props: &Props, pt: &ToggleSwitchPt) -> String {
    let mut cn = ClassNames::new();
    write_classes(&mut cn, &state_of(props));
    props.style_class.with(|c| cn.add_opt(c.as_deref()));
    cn.add_opt(pt.root.extra_class());
    cn.finish()
}

fn root_style(props: &Props, pt: &ToggleSwitchPt) -> String {
    props.style.with(|st| {
        pt.root.merge_style(&match st.as_deref() {
            Some(s) if !s.is_empty() => format!("position: relative; {s}"),
            _ => "position: relative".to_owned(),
        })
    })
}

fn handle_of(props: &Props, checked: bool) -> AnyView {
    match props.handle_template {
        Some(template) => template.run(checked),
        None => ().into_any(),
    }
}

/// Свойства экземпляра.
struct Props {
    checked: Option<RwSignal<bool>>,
    default_checked: bool,
    input_id: OptProp<String>,
    name: OptProp<String>,
    tabindex: OptProp<i32>,
    aria_label: OptProp<String>,
    aria_labelledby: OptProp<String>,
    disabled: Prop<bool>,
    readonly: Prop<bool>,
    required: Prop<bool>,
    invalid: Prop<bool>,
    style: OptProp<String>,
    style_class: OptProp<String>,
    handle_template: Option<Callback<bool, AnyView>>,
}
