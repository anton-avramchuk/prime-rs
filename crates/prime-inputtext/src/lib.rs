//! InputText — port of the PrimeNG 21.1.10 `pInputText` directive.
//!
//! In PrimeNG it is a directive on a native `<input>`; here the component renders an `<input>` with the same classes (`p-inputtext p-component`,
//! `p-filled`, `p-inputtext-sm`/`-lg`, `p-invalid`, `p-variant-filled`, `p-inputtext-fluid`) and `data-p`.
//!
//! The value is an optional two-way `value: RwSignal<String>`: the field shows the signal, user input is written back.
//! Without `value` the field is uncontrolled: the value lives in the DOM, and `p-filled` is not tracked.
//!
//! If all properties are static and `value` is not set, the component renders ready values without closures and `RenderEffect`.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_rs_core::Size;
//! use prime_inputtext::InputText;
//!
//! let value = RwSignal::new("abc".to_owned());
//! let html = view! { <InputText value=value placeholder="Name" size=Size::Small invalid=true /> }.to_html();
//! assert!(html.contains("p-inputtext-sm") && html.contains("p-invalid") && html.contains("p-filled") && html.contains(r#"placeholder="Name""#));
//! ```

mod state;

use leptos::either::Either;
use leptos::prelude::*;
use prime_rs_core::{ClassNames, OptProp, Prop, Pt, Size, Variant, all_static, use_prime_config};
use std::borrow::Cow;
use std::sync::Arc;

pub use state::{InputTextState, data_p, write_classes};

/// Text input field.
///
/// Each property accepts a value or a signal. `value` is a two-way binding (`RwSignal<String>`).
#[component]
pub fn InputText(
    /// Two-way bound value; without it the field is uncontrolled.
    #[prop(optional)]
    value: Option<RwSignal<String>>,
    /// `type` attribute (`text` by default).
    #[prop(optional, into)]
    input_type: OptProp<String>,
    /// `id` attribute.
    #[prop(optional, into)]
    id: OptProp<String>,
    /// `name` attribute.
    #[prop(optional, into)]
    name: OptProp<String>,
    /// Hint in an empty field.
    #[prop(optional, into)]
    placeholder: OptProp<String>,
    /// `autocomplete` attribute.
    #[prop(optional, into)]
    autocomplete: OptProp<String>,
    /// `aria-label` attribute.
    #[prop(optional, into)]
    aria_label: OptProp<String>,
    /// `aria-describedby` attribute.
    #[prop(optional, into)]
    aria_describedby: OptProp<String>,
    /// Disabled.
    #[prop(optional, into)]
    disabled: Prop<bool>,
    /// Read-only.
    #[prop(optional, into)]
    readonly: Prop<bool>,
    /// Required.
    #[prop(optional, into)]
    required: Prop<bool>,
    /// Maximum length (`maxlength`).
    #[prop(optional, into)]
    maxlength: OptProp<i32>,
    /// Size.
    #[prop(optional, into)]
    size: OptProp<Size>,
    /// Variant; if not set, `PrimeConfig.input_variant` is used.
    #[prop(optional, into)]
    variant: OptProp<Variant>,
    /// Invalid value.
    #[prop(optional, into)]
    invalid: Prop<bool>,
    /// Full container width.
    #[prop(optional, into)]
    fluid: OptProp<bool>,
    /// Inline style.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Extra class.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Called on input, after the value is written to `value`.
    #[prop(optional, into)]
    on_input: Option<Callback<web_sys::Event>>,
    /// Called when the value is committed (blur or Enter).
    #[prop(optional, into)]
    on_change: Option<Callback<web_sys::Event>>,
    /// Called on focus.
    #[prop(optional, into)]
    on_focus: Option<Callback<web_sys::FocusEvent>>,
    /// Called on blur.
    #[prop(optional, into)]
    on_blur: Option<Callback<web_sys::FocusEvent>>,
    /// Pass-through of the root element.
    #[prop(optional)]
    pt: Option<Pt>,
) -> impl IntoView {
    let pt = pt.unwrap_or_default();
    let fluid = prime_rs_core::fluid::fluid_prop(fluid);
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

/// InputText markup: one for the static and reactive branches (only the attribute expressions change).
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

/// Instance properties.
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
