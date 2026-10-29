//! SelectButton — порт `p-selectbutton` из PrimeNG 21.1.10.
//!
//! Разметка и классы как в PrimeNG: корень `div.p-selectbutton[role=group]` (вместо хоста `<p-selectbutton>`), внутри по
//! [`ToggleButton`](prime_togglebutton::ToggleButton) на вариант с `on_label = off_label = label`.
//!
//! Одиночный выбор — двусторонний сигнал `value: RwSignal<Option<String>>`, множественный (`multiple`) —
//! `values: RwSignal<Vec<String>>`. Без сигнала компонент ведёт внутреннее состояние (`default_value`, `default_values`).
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
use prime_core::{ClassNames, OptProp, Prop, Pt, Size};
use prime_togglebutton::{ToggleButton, ToggleButtonChange, ToggleButtonPt};
use std::sync::Arc;

pub use state::{
    SelectButtonOption, SelectButtonState, SelectButtonValue, data_p, select_multiple, select_single, toggle_allow_empty, write_classes,
};

/// Событие изменения выбора: новое значение и исходное событие браузера.
#[derive(Clone, Debug)]
pub struct SelectButtonChange {
    /// Новое значение.
    pub value: SelectButtonValue,
    /// Исходное событие (`click` или `keydown`).
    pub original_event: web_sys::Event,
}

/// Событие нажатия на вариант (после `on_change`).
#[derive(Clone, Debug)]
pub struct SelectButtonOptionClick {
    /// Нажатый вариант.
    pub option: SelectButtonOption,
    /// Его индекс в списке.
    pub index: usize,
    /// Исходное событие (`click` или `keydown`).
    pub original_event: web_sys::Event,
}

/// Pass-through частей SelectButton.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SelectButtonPt {
    /// Корневой элемент.
    pub root: Pt,
    /// Каждая вложенная кнопка `ToggleButton`.
    pub toggle_button: ToggleButtonPt,
}

/// Выбор одного или нескольких вариантов кнопками.
///
/// Каждое свойство принимает значение или сигнал; `value` и `values` — двусторонние сигналы.
#[component]
pub fn SelectButton(
    /// Варианты.
    #[prop(optional, into)]
    options: Prop<Vec<SelectButtonOption>>,
    /// Двусторонний сигнал значения одиночного выбора.
    #[prop(optional)]
    value: Option<RwSignal<Option<String>>>,
    /// Двусторонний сигнал значений множественного выбора.
    #[prop(optional)]
    values: Option<RwSignal<Vec<String>>>,
    /// Начальное значение, если `value` не передан.
    #[prop(optional, into)]
    default_value: OptProp<String>,
    /// Начальные значения, если `values` не передан.
    #[prop(optional)]
    default_values: Vec<String>,
    /// Множественный выбор.
    #[prop(optional, into)]
    multiple: Prop<bool>,
    /// Разрешено ли снимать выбор; ограничение действует только при явном `false`.
    #[prop(optional, into)]
    allow_empty: OptProp<bool>,
    /// Недоступен.
    #[prop(optional, into)]
    disabled: Prop<bool>,
    /// Недопустимое значение.
    #[prop(optional, into)]
    invalid: Prop<bool>,
    /// На всю ширину.
    #[prop(optional, into)]
    fluid: Prop<bool>,
    /// Размер кнопок.
    #[prop(optional, into)]
    size: OptProp<Size>,
    /// Атрибут `aria-labelledby`.
    #[prop(optional, into)]
    aria_labelledby: OptProp<String>,
    /// Встроенный стиль корня.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Дополнительный класс каждой кнопки (не корня, как в PrimeNG).
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Содержимое кнопки; получает вариант и его индекс.
    #[prop(optional, into)]
    item_template: Option<Callback<(SelectButtonOption, usize), AnyView>>,
    /// Вызывается после изменения выбора пользователем.
    #[prop(optional, into)]
    on_change: Option<Callback<SelectButtonChange>>,
    /// Вызывается после `on_change` с нажатым вариантом.
    #[prop(optional, into)]
    on_option_click: Option<Callback<SelectButtonOptionClick>>,
    /// Pass-through частей.
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
    // Ограничение `allow_empty` для вложенных кнопок общее для всех вариантов (`getAllowEmpty`).
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

/// Кнопка одного варианта: `ToggleButton`, чей сигнал всегда равен «вариант выбран».
fn option_button(
    props: &Arc<Props>,
    toggle_pt: &ToggleButtonPt,
    allow_empty: Signal<Option<bool>>,
    index: usize,
    option: SelectButtonOption,
) -> impl IntoView + use<> {
    let selected = RwSignal::new(props.selected_untracked(&option.value));
    // Синхронизация нужна только в браузере: при SSR разметка строится один раз, а `Effect` без executor паникует.
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
    // `content_template` у `ToggleButton` принимает только готовый `Callback`, поэтому ветви две.
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

/// Свойства экземпляра.
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
    /// Выбран ли вариант (с подпиской на значение активного режима); нужен только синхронизации в браузере.
    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    fn is_selected(&self, value: &str) -> bool {
        if self.multiple.get() {
            self.values.with(|list| list.iter().any(|v| v == value))
        } else {
            self.value.with(|current| current.as_deref() == Some(value))
        }
    }

    /// Как `onOptionSelect`: меняет значение, зовёт события и выравнивает сигнал кнопки с итоговым состоянием.
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
