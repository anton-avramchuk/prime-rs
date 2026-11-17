//! SelectButton — port of `p-selectbutton` from PrimeNG 21.1.10.
//!
//! Markup and classes as in PrimeNG: root `div.p-selectbutton[role=group]` (instead of the `<p-selectbutton>` host), inside a
//! [`ToggleButton`](prime_togglebutton::ToggleButton) per option with `on_label = off_label = label`.
//!
//! Single selection is a two-way signal `value: RwSignal<Option<String>>`, multiple selection (`multiple`) is
//! `values: RwSignal<Vec<String>>`. Without a signal the component keeps internal state (`default_value`, `default_values`).
//!
//! ```
//! use leptos::prelude::*;
//! use prime_selectbutton::{SelectButton, SelectButtonOption};
//!
//! let value = RwSignal::new(Some("one-way".to_owned()));
//! let options = vec![SelectButtonOption::new("One-Way", "one-way"), SelectButtonOption::new("Return", "return")];
//! let html = view! { <SelectButton options=options value=value /> }.to_html();
//! assert!(html.contains("p-selectbutton") && html.contains("One-Way") && html.contains(r#"aria-pressed="true""#));
//! ```

mod state;

use leptos::either::Either;
use leptos::prelude::*;
use prime_rs_core::{ClassNames, OptProp, Prop, Pt, Size};
use prime_togglebutton::{ToggleButton, ToggleButtonChange, ToggleButtonPt};
use std::sync::Arc;

pub use state::{
    SelectButtonOption, SelectButtonState, SelectButtonValue, data_p, select_multiple, select_single, toggle_allow_empty, write_classes,
};

/// Selection change event: the new value and the original browser event.
#[derive(Clone, Debug)]
pub struct SelectButtonChange {
    /// New value.
    pub value: SelectButtonValue,
    /// Original event (`click` or `keydown`).
    pub original_event: web_sys::Event,
}

/// Option press event (after `on_change`).
#[derive(Clone, Debug)]
pub struct SelectButtonOptionClick {
    /// The pressed option.
    pub option: SelectButtonOption,
    /// Its index in the list.
    pub index: usize,
    /// Original event (`click` or `keydown`).
    pub original_event: web_sys::Event,
}

/// Pass-through of SelectButton parts.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SelectButtonPt {
    /// Root element.
    pub root: Pt,
    /// Each nested `ToggleButton` button.
    pub toggle_button: ToggleButtonPt,
}

/// Selection of one or several options with buttons.
///
/// Each property accepts a value or a signal; `value` and `values` are two-way signals.
#[component]
pub fn SelectButton(
    /// Options.
    #[prop(optional, into)]
    options: Prop<Vec<SelectButtonOption>>,
    /// Two-way signal of the single-selection value.
    #[prop(optional)]
    value: Option<RwSignal<Option<String>>>,
    /// Two-way signal of the multiple-selection values.
    #[prop(optional)]
    values: Option<RwSignal<Vec<String>>>,
    /// Initial value if `value` is not passed.
    #[prop(optional, into)]
    default_value: OptProp<String>,
    /// Initial values if `values` is not passed.
    #[prop(optional)]
    default_values: Vec<String>,
    /// Multiple selection.
    #[prop(optional, into)]
    multiple: Prop<bool>,
    /// Whether deselecting is allowed; the restriction applies only with an explicit `false`.
    #[prop(optional, into)]
    allow_empty: OptProp<bool>,
    /// Disabled.
    #[prop(optional, into)]
    disabled: Prop<bool>,
    /// Invalid value.
    #[prop(optional, into)]
    invalid: Prop<bool>,
    /// Full width.
    #[prop(optional, into)]
    fluid: Prop<bool>,
    /// Button size.
    #[prop(optional, into)]
    size: OptProp<Size>,
    /// `aria-labelledby` attribute.
    #[prop(optional, into)]
    aria_labelledby: OptProp<String>,
    /// Inline style of the root.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Extra class of each button (not of the root, as in PrimeNG).
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Button content; receives the option and its index.
    #[prop(optional, into)]
    item_template: Option<Callback<(SelectButtonOption, usize), AnyView>>,
    /// Called after the selection is changed by the user.
    #[prop(optional, into)]
    on_change: Option<Callback<SelectButtonChange>>,
    /// Called after `on_change` with the pressed option.
    #[prop(optional, into)]
    on_option_click: Option<Callback<SelectButtonOptionClick>>,
    /// Pass-through of parts.
    #[prop(optional)]
    pt: Option<SelectButtonPt>,
) -> impl IntoView {
    let pt = Arc::new(pt.unwrap_or_default());
    let props = Arc::new(Props {
        options,
        value: value.unwrap_or_else(|| RwSignal::new(default_value.get_untracked())),
        values: values.unwrap_or_else(|| RwSignal::new(default_values)),
        multiple,
        allow_empty,
        disabled,
        invalid,
        fluid,
        size,
        aria_labelledby,
        style,
        style_class,
        item_template,
        on_change,
        on_option_click,
    });
    // The `allow_empty` restriction for nested buttons is shared by all options (`getAllowEmpty`).
    let toggle_empty = {
        let props = props.clone();
        Signal::derive(move || toggle_allow_empty(props.multiple.get(), props.allow_empty.get(), props.values.with(Vec::len)))
    };

    let root_class = {
        let props = props.clone();
        let pt = pt.clone();
        move || {
            let mut cn = ClassNames::new();
            write_classes(&mut cn, &state_of(&props));
            cn.add_user(pt.root.extra_class());
            cn.finish_cow()
        }
    };
    let root_style = {
        let props = props.clone();
        let pt = pt.clone();
        move || {
            let style = props.style.with(|st| pt.root.merge_style(st.as_deref().unwrap_or_default()));
            (!style.is_empty()).then_some(style)
        }
    };
    let data = {
        let props = props.clone();
        move || data_p(&state_of(&props))
    };
    let aria_labelledby = {
        let props = props.clone();
        move || props.aria_labelledby.get()
    };
    let buttons = {
        let (props, pt) = (props.clone(), pt.clone());
        move || {
            let options = props.options.get();
            options
                .into_iter()
                .enumerate()
                .map(|(index, option)| option_button(&props, &pt.toggle_button, toggle_empty, index, option))
                .collect_view()
        }
    };
    let root_attrs = pt.root.attrs();

    view! {
        <div class={root_class} style={root_style} role="group" aria-labelledby={aria_labelledby} data-p={data} {..root_attrs}>
            {buttons}
        </div>
    }
}

/// Single-option button: a `ToggleButton` whose signal always equals "option is selected".
fn option_button(
    props: &Arc<Props>,
    toggle_pt: &ToggleButtonPt,
    allow_empty: Signal<Option<bool>>,
    index: usize,
    option: SelectButtonOption,
) -> impl IntoView + use<> {
    let selected = RwSignal::new(props.selected_untracked(&option.value));
    // Synchronization is needed only in the browser: with SSR the markup is built once, and an `Effect` without an executor panics.
    #[cfg(target_arch = "wasm32")]
    {
        let (props, value) = (props.clone(), option.value.clone());
        Effect::new(move |_| selected.set(props.is_selected(&value)));
    }
    let on_change = {
        let (props, option) = (props.clone(), option.clone());
        Callback::new(move |e: ToggleButtonChange| props.choose(&option, index, selected, e.original_event))
    };
    let disabled: Prop<bool> = if props.disabled.is_static() {
        (props.disabled.get() || option.disabled).into()
    } else {
        let (disabled, own) = (props.disabled.clone(), option.disabled);
        Signal::derive(move || disabled.get() || own).into()
    };
    let (size, fluid, style_class) = (props.size.clone(), props.fluid.clone(), props.style_class.clone());
    let (label, toggle_pt) = (option.label.clone(), toggle_pt.clone());
    // `content_template` of `ToggleButton` accepts only a ready `Callback`, hence two branches.
    match props.item_template {
        Some(template) => {
            let content = Callback::new(move |_: bool| template.run((option.clone(), index)));
            Either::Left(view! {
                <ToggleButton
                    checked=selected
                    on_label=label.clone()
                    off_label=label
                    disabled=disabled
                    allow_empty=allow_empty
                    size=size
                    fluid=fluid
                    style_class=style_class
                    content_template=content
                    on_change=on_change
                    pt=toggle_pt
                />
            })
        }
        None => Either::Right(view! {
            <ToggleButton
                checked=selected
                on_label=label.clone()
                off_label=label
                disabled=disabled
                allow_empty=allow_empty
                size=size
                fluid=fluid
                style_class=style_class
                on_change=on_change
                pt=toggle_pt
            />
        }),
    }
}

fn state_of(props: &Props) -> SelectButtonState {
    SelectButtonState { invalid: props.invalid.get(), fluid: props.fluid.get() }
}

/// Instance properties.
struct Props {
    options: Prop<Vec<SelectButtonOption>>,
    value: RwSignal<Option<String>>,
    values: RwSignal<Vec<String>>,
    multiple: Prop<bool>,
    allow_empty: OptProp<bool>,
    disabled: Prop<bool>,
    invalid: Prop<bool>,
    fluid: Prop<bool>,
    size: OptProp<Size>,
    aria_labelledby: OptProp<String>,
    style: OptProp<String>,
    style_class: OptProp<String>,
    item_template: Option<Callback<(SelectButtonOption, usize), AnyView>>,
    on_change: Option<Callback<SelectButtonChange>>,
    on_option_click: Option<Callback<SelectButtonOptionClick>>,
}

impl Props {
    /// Whether the option is selected (subscribing to the value of the active mode); needed only for synchronization in the browser.
    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    fn is_selected(&self, value: &str) -> bool {
        if self.multiple.get() {
            self.values.with(|list| list.iter().any(|v| v == value))
        } else {
            self.value.with(|current| current.as_deref() == Some(value))
        }
    }

    /// Like `onOptionSelect`: changes the value, fires the events and aligns the button signal with the final state.
    fn choose(&self, option: &SelectButtonOption, index: usize, selected: RwSignal<bool>, original_event: web_sys::Event) {
        if !(self.disabled.get_untracked() || option.disabled) {
            let allow_empty = self.allow_empty.get_untracked();
            let changed = if self.multiple.get_untracked() {
                self.values
                    .with_untracked(|list| select_multiple(list, &option.value, allow_empty))
                    .inspect(|next| self.values.set(next.clone()))
                    .map(SelectButtonValue::Multiple)
            } else {
                self.value
                    .with_untracked(|current| select_single(current.as_deref(), &option.value, allow_empty))
                    .inspect(|next| self.value.set(next.clone()))
                    .map(SelectButtonValue::Single)
            };
            if let Some(value) = changed {
                if let Some(cb) = self.on_change {
                    cb.run(SelectButtonChange { value, original_event: original_event.clone() });
                }
                if let Some(cb) = self.on_option_click {
                    cb.run(SelectButtonOptionClick { option: option.clone(), index, original_event });
                }
            }
        }
        selected.set(self.selected_untracked(&option.value));
    }

    fn selected_untracked(&self, value: &str) -> bool {
        if self.multiple.get_untracked() {
            self.values.with_untracked(|list| list.iter().any(|v| v == value))
        } else {
            self.value.with_untracked(|current| current.as_deref() == Some(value))
        }
    }
}
