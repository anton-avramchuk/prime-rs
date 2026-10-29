//! ToggleButton — порт `p-togglebutton` из PrimeNG 21.1.10.
//!
//! Разметка и классы как в PrimeNG: корень `div.p-togglebutton[role=button]` (вместо хоста `<p-togglebutton>`), внутри
//! `span.p-togglebutton-content` с иконкой и `span.p-togglebutton-label`. Нативного `<input>` нет.
//!
//! Состояние — двусторонний сигнал `checked: RwSignal<bool>`; без него компонент ведёт внутреннее состояние, начинающееся
//! с `default_checked`. Клик и клавиши Enter и Space переключают состояние и вызывают `on_change`. Иконка передаётся как
//! `ViewFn` (например, `<CheckIcon />` из `prime-icons`), а не именем класса шрифта.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_togglebutton::ToggleButton;
//!
//! let checked = RwSignal::new(true);
//! let html = view! { <ToggleButton checked=checked on_label="Вкл" off_label="Выкл" /> }.to_html();
//! assert!(html.contains("p-togglebutton-checked") && html.contains(r#"aria-pressed="true""#) && html.contains("Вкл"));
//! ```

mod state;

use leptos::prelude::*;
use prime_core::{ClassNames, OptProp, Prop, Pt, Size};
use std::borrow::Cow;
use std::sync::Arc;

pub use state::{IconPos, ToggleButtonState, data_p, write_classes};

/// Событие изменения: новое состояние и исходное событие браузера.
#[derive(Clone, Debug)]
pub struct ToggleButtonChange {
    /// Новое состояние.
    pub checked: bool,
    /// Исходное событие (`click` или `keydown`).
    pub original_event: web_sys::Event,
}

/// Pass-through частей ToggleButton.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ToggleButtonPt {
    /// Корневой элемент.
    pub root: Pt,
    /// Содержимое (`span.p-togglebutton-content`).
    pub content: Pt,
    /// Оболочка иконки (`span.p-togglebutton-icon`).
    pub icon: Pt,
    /// Метка (`span.p-togglebutton-label`).
    pub label: Pt,
}

/// Кнопка с булевым состоянием.
///
/// Каждое свойство принимает значение или сигнал; `checked` — двусторонний сигнал.
#[component]
pub fn ToggleButton(
    /// Двусторонний сигнал состояния.
    #[prop(optional)]
    checked: Option<RwSignal<bool>>,
    /// Начальное состояние, если `checked` не передан.
    #[prop(optional)]
    default_checked: bool,
    /// Метка включённой кнопки (по умолчанию «Yes»).
    #[prop(optional, into)]
    on_label: OptProp<String>,
    /// Метка выключенной кнопки (по умолчанию «No»).
    #[prop(optional, into)]
    off_label: OptProp<String>,
    /// Иконка включённой кнопки; выводится в `span.p-togglebutton-icon`.
    #[prop(optional, into)]
    on_icon: Option<ViewFn>,
    /// Иконка выключенной кнопки; выводится в `span.p-togglebutton-icon`.
    #[prop(optional, into)]
    off_icon: Option<ViewFn>,
    /// Положение иконки.
    #[prop(optional, into)]
    icon_pos: Prop<IconPos>,
    /// Размер.
    #[prop(optional, into)]
    size: OptProp<Size>,
    /// Недоступна.
    #[prop(optional, into)]
    disabled: Prop<bool>,
    /// Недопустимое значение.
    #[prop(optional, into)]
    invalid: Prop<bool>,
    /// Разрешено ли снимать выбор; ограничение действует только при явном `false`.
    #[prop(optional, into)]
    allow_empty: OptProp<bool>,
    /// На всю ширину.
    #[prop(optional, into)]
    fluid: Prop<bool>,
    /// Атрибут `tabindex` (по умолчанию `0`, при `disabled` `-1`).
    #[prop(optional, into)]
    tabindex: OptProp<i32>,
    /// Атрибут `aria-label`.
    #[prop(optional, into)]
    aria_label: OptProp<String>,
    /// Атрибут `aria-labelledby`.
    #[prop(optional, into)]
    aria_labelledby: OptProp<String>,
    /// Встроенный стиль корня.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Дополнительный класс корня.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Заменяет иконку по умолчанию; получает текущее состояние.
    #[prop(optional, into)]
    icon_template: Option<Callback<bool, AnyView>>,
    /// Заменяет всё содержимое после `span.p-togglebutton-content`; получает текущее состояние.
    #[prop(optional, into)]
    content_template: Option<Callback<bool, AnyView>>,
    /// Вызывается после переключения пользователем.
    #[prop(optional, into)]
    on_change: Option<Callback<ToggleButtonChange>>,
    /// Вызывается при получении фокуса.
    #[prop(optional, into)]
    on_focus: Option<Callback<web_sys::FocusEvent>>,
    /// Вызывается при потере фокуса.
    #[prop(optional, into)]
    on_blur: Option<Callback<web_sys::FocusEvent>>,
    /// Pass-through частей.
    #[prop(optional)]
    pt: Option<ToggleButtonPt>,
) -> impl IntoView {
    let pt = Arc::new(pt.unwrap_or_default());
    let props = Arc::new(Props {
        checked: checked.unwrap_or_else(|| RwSignal::new(default_checked)),
        on_label,
        off_label,
        on_icon,
        off_icon,
        icon_pos,
        size,
        disabled,
        invalid,
        allow_empty,
        fluid,
        tabindex,
        aria_label,
        aria_labelledby,
        style,
        style_class,
        icon_template,
        content_template,
    });

    // Как `toggle` в PrimeNG: и клик, и Enter/Space; `allow_empty = false` не даёт выключить включённую кнопку.
    let toggle = {
        let props = props.clone();
        move |e: web_sys::Event| {
            let was = props.checked.get_untracked();
            if props.disabled.get_untracked() || (was && props.allow_empty.get_untracked() == Some(false)) {
                return;
            }
            props.checked.set(!was);
            if let Some(cb) = on_change {
                cb.run(ToggleButtonChange { checked: !was, original_event: e });
            }
        }
    };
    let click = {
        let toggle = toggle.clone();
        move |e: web_sys::MouseEvent| toggle(e.into())
    };
    let keydown = move |e: web_sys::KeyboardEvent| {
        if matches!(e.code().as_str(), "Enter" | "Space") {
            e.prevent_default();
            toggle(e.into());
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

    let (p, t) = (&props, &pt);
    let root_class = rx(p, t, root_class);
    let root_style = rx(p, t, root_style);
    let pressed = rx(p, t, |p, _| bool_attr(p.checked.get()));
    let checked_attr = pressed.clone();
    let disabled_attr = rx(p, t, |p, _| bool_attr(p.disabled.get()));
    let data = rx(p, t, |p, _| data_p(&state_of(p)));
    let tabindex = rx(p, t, |p, _| p.tabindex.get().unwrap_or(if p.disabled.get() { -1 } else { 0 }));
    let aria_label = rx(p, t, |p, _| p.aria_label.get());
    let aria_labelledby = rx(p, t, |p, _| p.aria_labelledby.get());
    let content_class = rx(p, t, |_, t| t.content.merge_class("p-togglebutton-content"));
    let content_data = data.clone();
    let content = rx(p, t, content_of);
    let (root_attrs, content_attrs) = (pt.root.attrs(), pt.content.attrs());

    view! {
        <div
            class={root_class}
            style={root_style}
            role="button"
            aria-pressed={pressed}
            tabindex={tabindex}
            aria-label={aria_label}
            aria-labelledby={aria_labelledby}
            data-pc-name="togglebutton"
            data-p-checked={checked_attr}
            data-p-disabled={disabled_attr}
            data-p={data}
            on:click={click}
            on:keydown={keydown}
            on:focus={focus}
            on:blur={blur}
            {..root_attrs}
        >
            <span class={content_class} data-p={content_data} {..content_attrs}>{content}</span>
        </div>
    }
}

/// Замыкание над свойствами экземпляра: клоны `Arc` и указатель на функцию без захвата.
fn rx<T>(
    props: &Arc<Props>,
    pt: &Arc<ToggleButtonPt>,
    f: fn(&Props, &ToggleButtonPt) -> T,
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

fn state_of(props: &Props) -> ToggleButtonState {
    ToggleButtonState {
        checked: props.checked.get(),
        disabled: props.disabled.get(),
        invalid: props.invalid.get(),
        size: props.size.get(),
        fluid: props.fluid.get(),
    }
}

fn root_class(props: &Props, pt: &ToggleButtonPt) -> Cow<'static, str> {
    let mut cn = ClassNames::new();
    write_classes(&mut cn, &state_of(props));
    props.style_class.with(|c| cn.add_user(c.as_deref()));
    cn.add_user(pt.root.extra_class());
    cn.finish_cow()
}

/// Стиль корня; пустой результат — `None`, чтобы не выводить `style=""`.
fn root_style(props: &Props, pt: &ToggleButtonPt) -> Option<String> {
    let style = props.style.with(|st| pt.root.merge_style(st.as_deref().unwrap_or_default()));
    (!style.is_empty()).then_some(style)
}

/// Метка с подстановкой по умолчанию; пустая метка — один пробел, как в шаблоне PrimeNG.
fn label_of(props: &Props) -> String {
    let (label, default) = if props.checked.get() { (props.on_label.get(), "Yes") } else { (props.off_label.get(), "No") };
    match label {
        None => default.to_owned(),
        Some(l) if l.is_empty() => " ".to_owned(),
        Some(l) => l,
    }
}

fn content_of(props: &Props, pt: &ToggleButtonPt) -> AnyView {
    let checked = props.checked.get();
    if let Some(template) = props.content_template {
        return template.run(checked);
    }
    let icon = match props.icon_template {
        Some(template) => Some(template.run(checked)),
        None if props.on_icon.is_some() || props.off_icon.is_some() => {
            let icon = if checked { &props.on_icon } else { &props.off_icon };
            Some(
                view! {
                    <span class=pt.icon.merge_class(props.icon_pos.get().class()) {..pt.icon.attrs()}>
                        {icon.as_ref().map(ViewFn::run)}
                    </span>
                }
                .into_any(),
            )
        }
        None => None,
    };
    view! {
        {icon}
        <span class=pt.label.merge_class("p-togglebutton-label") {..pt.label.attrs()}>{label_of(props)}</span>
    }
    .into_any()
}

/// Свойства экземпляра.
struct Props {
    checked: RwSignal<bool>,
    on_label: OptProp<String>,
    off_label: OptProp<String>,
    on_icon: Option<ViewFn>,
    off_icon: Option<ViewFn>,
    icon_pos: Prop<IconPos>,
    size: OptProp<Size>,
    disabled: Prop<bool>,
    invalid: Prop<bool>,
    allow_empty: OptProp<bool>,
    fluid: Prop<bool>,
    tabindex: OptProp<i32>,
    aria_label: OptProp<String>,
    aria_labelledby: OptProp<String>,
    style: OptProp<String>,
    style_class: OptProp<String>,
    icon_template: Option<Callback<bool, AnyView>>,
    content_template: Option<Callback<bool, AnyView>>,
}
