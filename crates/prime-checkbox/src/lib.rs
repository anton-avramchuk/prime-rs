//! Checkbox — port of `p-checkbox` from PrimeNG 21.1.10.
//!
//! Markup and classes as in PrimeNG: root `div.p-checkbox` (instead of the host `<p-checkbox>`), inside it `input.p-checkbox-input`
//! and `div.p-checkbox-box` with a `check` or `minus` icon.
//!
//! Two value modes: `binary` with `checked: RwSignal<bool>`, and a set — `value` plus `model: RwSignal<Vec<String>>`.
//! Without signals the field is uncontrolled: the initial state comes from `default_checked`, after that the DOM keeps it, and the root classes are not tracked.
//! `indeterminate` is a two-way signal: after a change by the user the component resets it to `false`.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_checkbox::Checkbox;
//!
//! let checked = RwSignal::new(true);
//! let html = view! { <Checkbox binary=true checked=checked name="agree" /> }.to_html();
//! assert!(html.contains("p-checkbox-checked") && html.contains("p-checkbox-input") && html.contains(r#"name="agree""#));
//! ```

mod state;

use leptos::either::{Either, EitherOf4};
use leptos::prelude::*;
use prime_icons::{CheckIcon, MinusIcon};
use prime_rs_core::{ClassNames, OptProp, Prop, Pt, Size, Variant, all_static, use_prime_config};
use std::borrow::Cow;
use std::sync::Arc;

pub use state::{CheckboxState, data_p, write_classes};

/// Change event: the new state and the original browser event.
#[derive(Clone, Debug)]
pub struct CheckboxChange {
    /// New state (in set mode — whether `value` is in the set).
    pub checked: bool,
    /// Original `change` event.
    pub original_event: web_sys::Event,
}

/// Pass-through for the parts of Checkbox. For `icon` only the class is taken into account: the icon is a separate component.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CheckboxPt {
    /// Root element.
    pub root: Pt,
    /// Native `<input>`.
    pub input: Pt,
    /// Box (`div.p-checkbox-box`).
    pub box_: Pt,
    /// Icon (class only).
    pub icon: Pt,
}

/// Checkbox.
///
/// Each property accepts a value or a signal; `checked`, `model` and `indeterminate` are two-way signals.
#[component]
pub fn Checkbox(
    /// Single boolean value mode (`checked`); otherwise — set mode (`value` and `model`).
    #[prop(optional)]
    binary: bool,
    /// Two-way state signal in `binary` mode.
    #[prop(optional)]
    checked: Option<RwSignal<bool>>,
    /// Value of this checkbox in set mode (and the `value` attribute of `<input>`).
    #[prop(optional, into)]
    value: OptProp<String>,
    /// Two-way set of checked values in set mode.
    #[prop(optional)]
    model: Option<RwSignal<Vec<String>>>,
    /// Initial state of an uncontrolled checkbox.
    #[prop(optional)]
    default_checked: bool,
    /// Indeterminate state; reset to `false` after a change by the user.
    #[prop(optional)]
    indeterminate: Option<RwSignal<bool>>,
    /// `id` attribute of `<input>`.
    #[prop(optional, into)]
    input_id: OptProp<String>,
    /// `name` attribute.
    #[prop(optional, into)]
    name: OptProp<String>,
    /// `tabindex` attribute.
    #[prop(optional, into)]
    tabindex: OptProp<i32>,
    /// `aria-label` attribute.
    #[prop(optional, into)]
    aria_label: OptProp<String>,
    /// `aria-labelledby` attribute.
    #[prop(optional, into)]
    aria_labelledby: OptProp<String>,
    /// Unavailable.
    #[prop(optional, into)]
    disabled: Prop<bool>,
    /// Read-only.
    #[prop(optional, into)]
    readonly: Prop<bool>,
    /// Required.
    #[prop(optional, into)]
    required: Prop<bool>,
    /// Invalid value.
    #[prop(optional, into)]
    invalid: Prop<bool>,
    /// Size.
    #[prop(optional, into)]
    size: OptProp<Size>,
    /// Variant; if not set, `PrimeConfig.input_variant` is used.
    #[prop(optional, into)]
    variant: OptProp<Variant>,
    /// Inline style of the root.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Extra class of the root.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Extra class of `<input>`.
    #[prop(optional, into)]
    input_class: OptProp<String>,
    /// Inline style of `<input>`.
    #[prop(optional, into)]
    input_style: OptProp<String>,
    /// Custom icon instead of `check`/`minus`.
    #[prop(optional, into)]
    icon_template: Option<ViewFn>,
    /// Called after a change of state by the user.
    #[prop(optional, into)]
    on_change: Option<Callback<CheckboxChange>>,
    /// Called when focus is received.
    #[prop(optional, into)]
    on_focus: Option<Callback<web_sys::FocusEvent>>,
    /// Called when focus is lost.
    #[prop(optional, into)]
    on_blur: Option<Callback<web_sys::FocusEvent>>,
    /// Pass-through for the parts.
    #[prop(optional)]
    pt: Option<CheckboxPt>,
) -> impl IntoView {
    let pt = pt.unwrap_or_default();
    let is_static = all_static!(
        value,
        input_id,
        name,
        tabindex,
        aria_label,
        aria_labelledby,
        disabled,
        readonly,
        required,
        invalid,
        size,
        variant,
        style,
        style_class,
        input_class,
        input_style
    ) && checked.is_none()
        && model.is_none()
        && indeterminate.is_none();
    let props = Arc::new(Props {
        binary,
        checked,
        value,
        model,
        default_checked,
        indeterminate,
        input_id,
        name,
        tabindex,
        aria_label,
        aria_labelledby,
        disabled,
        readonly,
        required,
        invalid,
        size,
        variant,
        style,
        style_class,
        input_class,
        input_style,
        icon_template,
        config_variant: use_prime_config().input_variant,
    });
    let (root_attrs, input_attrs, box_attrs) = (pt.root.attrs(), pt.input.attrs(), pt.box_.attrs());

    let change = {
        let props = props.clone();
        move |e: web_sys::Event| {
            let input = event_target::<web_sys::HtmlInputElement>(&e);
            if props.readonly.get() || props.disabled.get() {
                input.set_checked(checked_of(&props));
                return;
            }
            let now = input.checked();
            apply(&props, now);
            if let Some(cb) = on_change {
                cb.run(CheckboxChange { checked: now, original_event: e });
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
        let (p, t) = (&*props, &pt);
        let root_class = root_class(p, t);
        let root_style = root_style(p, t);
        let (highlight, data) = (bool_attr(checked_of(p)), data_of(p));
        let disabled_attr = bool_attr(p.disabled.get());
        let (id, value, name, tabindex) = (p.input_id.get(), p.value.get(), p.name.get(), p.tabindex.get());
        let (required, readonly, disabled) = (p.required.get(), p.readonly.get(), p.disabled.get());
        let (aria_label, aria_labelledby) = (p.aria_label.get(), p.aria_labelledby.get());
        let (input_class, input_style) = (input_class_of(p, t), p.input_style.get());
        let (is_checked, box_class, icon) = (checked_of(p), t.box_.merge_class("p-checkbox-box"), icon_of(p, t));
        Either::Left(checkbox_markup!(
            root_class,
            root_style,
            highlight,
            disabled_attr,
            data,
            id,
            value,
            name,
            tabindex,
            required,
            readonly,
            disabled,
            aria_label,
            aria_labelledby,
            input_class,
            input_style,
            is_checked,
            box_class,
            icon,
            change,
            focus,
            blur,
            root_attrs,
            input_attrs,
            box_attrs
        ))
    } else {
        let pt = Arc::new(pt);
        let (p, t) = (&props, &pt);
        let root_class = rx(p, t, root_class);
        let root_style = rx(p, t, root_style);
        let highlight = rx(p, t, |p, _| bool_attr(checked_of(p)));
        let disabled_attr = rx(p, t, |p, _| bool_attr(p.disabled.get()));
        let data = rx(p, t, |p, _| data_of(p));
        let id = rx(p, t, |p, _| p.input_id.get());
        let value = rx(p, t, |p, _| p.value.get());
        let name = rx(p, t, |p, _| p.name.get());
        let tabindex = rx(p, t, |p, _| p.tabindex.get());
        let required = rx(p, t, |p, _| p.required.get());
        let readonly = rx(p, t, |p, _| p.readonly.get());
        let disabled = rx(p, t, |p, _| p.disabled.get());
        let aria_label = rx(p, t, |p, _| p.aria_label.get());
        let aria_labelledby = rx(p, t, |p, _| p.aria_labelledby.get());
        let input_class = rx(p, t, input_class_of);
        let input_style = rx(p, t, |p, _| p.input_style.get());
        let is_checked = rx(p, t, |p, _| checked_of(p));
        let box_class = rx(p, t, |_, t| t.box_.merge_class("p-checkbox-box"));
        let icon = rx(p, t, icon_of);
        Either::Right(checkbox_markup!(
            root_class,
            root_style,
            highlight,
            disabled_attr,
            data,
            id,
            value,
            name,
            tabindex,
            required,
            readonly,
            disabled,
            aria_label,
            aria_labelledby,
            input_class,
            input_style,
            is_checked,
            box_class,
            icon,
            change,
            focus,
            blur,
            root_attrs,
            input_attrs,
            box_attrs
        ))
    }
}

/// Checkbox markup: one for the static and reactive branches (only the attribute expressions differ).
macro_rules! checkbox_markup {
    (
        $root_class:expr, $root_style:expr, $highlight:expr, $disabled_attr:expr, $data:expr, $id:expr, $value:expr, $name:expr,
        $tabindex:expr, $required:expr, $readonly:expr, $disabled:expr, $aria_label:expr, $aria_labelledby:expr, $input_class:expr,
        $input_style:expr, $is_checked:expr, $box_class:expr, $icon:expr, $change:expr, $focus:expr, $blur:expr, $root_attrs:expr,
        $input_attrs:expr, $box_attrs:expr
    ) => {{
        let data_root = $data.clone();
        view! {
            <div
                class={$root_class}
                style={$root_style}
                data-p-highlight={$highlight.clone()}
                data-p-checked={$highlight}
                data-p-disabled={$disabled_attr}
                data-p={data_root}
                {..$root_attrs}
            >
                <input
                    id={$id}
                    type="checkbox"
                    value={$value}
                    name={$name}
                    checked={$is_checked.clone()}
                    prop:checked={$is_checked}
                    tabindex={$tabindex}
                    required={$required}
                    readonly={$readonly}
                    disabled={$disabled}
                    aria-labelledby={$aria_labelledby}
                    aria-label={$aria_label}
                    style={$input_style}
                    class={$input_class}
                    on:focus={$focus}
                    on:blur={$blur}
                    on:change={$change}
                    {..$input_attrs}
                />
                <div class={$box_class} data-p={$data} {..$box_attrs}>{$icon}</div>
            </div>
        }
    }};
}
use checkbox_markup;

/// Closure over the instance properties: `Arc` clones and a function pointer without capture.
fn rx<T>(
    props: &Arc<Props>,
    pt: &Arc<CheckboxPt>,
    f: fn(&Props, &CheckboxPt) -> T,
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
    if props.indeterminate.is_some_and(|s| s.get()) {
        false
    } else if props.binary {
        props.checked.map_or(props.default_checked, |s| s.get())
    } else if let Some(model) = props.model {
        props.value.with(|v| v.as_deref().is_some_and(|v| model.with(|set| set.iter().any(|x| x == v))))
    } else {
        props.default_checked
    }
}

/// Writes the new state to the signals and resets `indeterminate`.
fn apply(props: &Props, now: bool) {
    if let Some(ind) = props.indeterminate {
        ind.set(false);
    }
    if props.binary {
        if let Some(sig) = props.checked {
            sig.set(now);
        }
    } else if let (Some(model), Some(value)) = (props.model, props.value.get()) {
        model.update(|set| {
            set.retain(|x| *x != value);
            if now {
                set.push(value);
            }
        });
    }
}

fn state_of(props: &Props) -> CheckboxState {
    CheckboxState {
        checked: checked_of(props),
        disabled: props.disabled.get(),
        invalid: props.invalid.get(),
        variant: props.variant.get().or(props.config_variant),
        size: props.size.get(),
    }
}

fn data_of(props: &Props) -> Cow<'static, str> {
    data_p(&state_of(props))
}

fn root_class(props: &Props, pt: &CheckboxPt) -> Cow<'static, str> {
    let mut cn = ClassNames::new();
    write_classes(&mut cn, &state_of(props));
    props.style_class.with(|c| cn.add_user(c.as_deref()));
    cn.add_user(pt.root.extra_class());
    cn.finish_cow()
}

fn root_style(props: &Props, pt: &CheckboxPt) -> Option<String> {
    props.style.with(|st| (st.is_some() || pt.root.has_style()).then(|| pt.root.merge_style(st.as_deref().unwrap_or(""))))
}

fn input_class_of(props: &Props, pt: &CheckboxPt) -> Cow<'static, str> {
    let mut cn = ClassNames::new();
    cn.add("p-checkbox-input");
    props.input_class.with(|c| cn.add_user(c.as_deref()));
    cn.add_user(pt.input.extra_class());
    cn.finish_cow()
}

fn icon_of(props: &Props, pt: &CheckboxPt) -> impl IntoView + use<> {
    if let Some(template) = &props.icon_template {
        return EitherOf4::A(template.run());
    }
    let class = pt.icon.merge_class("p-checkbox-icon");
    if props.indeterminate.is_some_and(|s| s.get()) {
        EitherOf4::B(view! { <MinusIcon class=class /> })
    } else if checked_of(props) {
        EitherOf4::C(view! { <CheckIcon class=class /> })
    } else {
        EitherOf4::D(())
    }
}

/// Instance properties.
struct Props {
    binary: bool,
    checked: Option<RwSignal<bool>>,
    value: OptProp<String>,
    model: Option<RwSignal<Vec<String>>>,
    default_checked: bool,
    indeterminate: Option<RwSignal<bool>>,
    input_id: OptProp<String>,
    name: OptProp<String>,
    tabindex: OptProp<i32>,
    aria_label: OptProp<String>,
    aria_labelledby: OptProp<String>,
    disabled: Prop<bool>,
    readonly: Prop<bool>,
    required: Prop<bool>,
    invalid: Prop<bool>,
    size: OptProp<Size>,
    variant: OptProp<Variant>,
    style: OptProp<String>,
    style_class: OptProp<String>,
    input_class: OptProp<String>,
    input_style: OptProp<String>,
    icon_template: Option<ViewFn>,
    config_variant: Option<Variant>,
}
