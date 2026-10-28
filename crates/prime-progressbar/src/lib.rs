//! ProgressBar — порт `p-progressbar` из PrimeNG 21.1.10.
//!
//! Разметка и классы как в PrimeNG: корень `div.p-progressbar[role=progressbar]` (вместо хоста `<p-progressbar>`); в режиме
//! `determinate` внутри `div.p-progressbar-value` (ширина по значению) с `div.p-progressbar-label`, в режиме `indeterminate` —
//! только `div.p-progressbar-value`.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_progressbar::ProgressBar;
//!
//! let html = view! { <ProgressBar value=50.0 /> }.to_html();
//! assert!(html.contains("p-progressbar-determinate") && html.contains("width: 50%") && html.contains(r#"aria-valuenow="50""#));
//! ```

mod state;

use leptos::either::Either;
use leptos::prelude::*;
use prime_core::{ClassNames, OptProp, Prop, Pt, all_static};
use std::sync::Arc;

pub use state::{ProgressBarMode, ProgressBarState, data_p, write_classes};

/// Pass-through частей ProgressBar.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ProgressBarPt {
    /// Корневой элемент.
    pub root: Pt,
    /// Полоса (`div.p-progressbar-value`).
    pub value: Pt,
    /// Подпись (`div.p-progressbar-label`).
    pub label: Pt,
}

/// Индикатор выполнения.
///
/// Каждое свойство принимает значение или сигнал.
#[component]
pub fn ProgressBar(
    /// Значение от 0 до 100.
    #[prop(optional, into)]
    value: OptProp<f64>,
    /// Режим (по умолчанию `determinate`).
    #[prop(optional, into)]
    mode: Prop<ProgressBarMode>,
    /// Показывать значение в подписи.
    #[prop(default = true)]
    show_value: bool,
    /// Единица после значения (по умолчанию `%`).
    #[prop(optional, into)]
    unit: OptProp<String>,
    /// Цвет полосы (CSS `background`).
    #[prop(optional, into)]
    color: OptProp<String>,
    /// Встроенный стиль корня.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Дополнительный класс корня.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Дополнительный класс полосы.
    #[prop(optional, into)]
    value_style_class: OptProp<String>,
    /// Своё содержимое подписи; получает значение.
    #[prop(optional, into)]
    content_template: Option<Callback<Option<f64>, AnyView>>,
    /// Pass-through частей.
    #[prop(optional)]
    pt: Option<ProgressBarPt>,
) -> impl IntoView {
    let pt = Arc::new(pt.unwrap_or_default());
    let is_static = all_static!(value, mode, unit, color, style, style_class, value_style_class);
    let props = Arc::new(Props { value, mode, show_value, unit, color, style, style_class, value_style_class, content_template });
    let root_attrs = pt.root.attrs();

    if is_static {
        let (p, t) = (&*props, &*pt);
        let (root_class, root_style, data) = (root_class(p, t), root_style(p, t), data_of(p));
        let (valuenow, level) = (valuenow_of(p), level_of(p));
        let body = body_of(p, t, &data);
        Either::Left(progress_markup!(root_class, root_style, valuenow, level, data, body, root_attrs))
    } else {
        let (p, t) = (&props, &pt);
        let root_class = rx(p, t, root_class);
        let root_style = rx(p, t, root_style);
        let data = rx(p, t, |p, _| data_of(p));
        let valuenow = rx(p, t, |p, _| valuenow_of(p));
        let level = rx(p, t, |p, _| level_of(p));
        let body = rx(p, t, |p, t| body_of(p, t, &data_of(p)));
        Either::Right(progress_markup!(root_class, root_style, valuenow, level, data, body, root_attrs))
    }
}

/// Разметка корня ProgressBar: одна для статической и реактивной ветвей (меняются только выражения атрибутов).
macro_rules! progress_markup {
    ($root_class:expr, $root_style:expr, $valuenow:expr, $level:expr, $data:expr, $body:expr, $root_attrs:expr) => {
        view! {
            <div
                role="progressbar"
                aria-valuemin="0"
                aria-valuenow={$valuenow}
                aria-valuemax="100"
                class={$root_class}
                style={$root_style}
                data-p={$data}
                {..leptos::attr::custom::custom_attribute("aria-level", $level)}
                {..$root_attrs}
            >
                {$body}
            </div>
        }
    };
}
use progress_markup;

/// Замыкание над свойствами экземпляра: клоны `Arc` и указатель на функцию без захвата.
fn rx<T>(
    props: &Arc<Props>,
    pt: &Arc<ProgressBarPt>,
    f: fn(&Props, &ProgressBarPt) -> T,
) -> impl Fn() -> T + Clone + Send + Sync + 'static + use<T>
where
    T: 'static,
{
    let (props, pt) = (props.clone(), pt.clone());
    move || f(&props, &pt)
}

fn data_of(props: &Props) -> String {
    data_p(&ProgressBarState { mode: props.mode.get() })
}

fn valuenow_of(props: &Props) -> Option<String> {
    props.value.get().map(|v| v.to_string())
}

fn unit_of(props: &Props) -> String {
    props.unit.with(|u| u.as_deref().unwrap_or("%").to_owned())
}

fn level_of(props: &Props) -> Option<String> {
    props.value.get().map(|v| format!("{v}{}", unit_of(props)))
}

fn root_class(props: &Props, pt: &ProgressBarPt) -> String {
    let mut cn = ClassNames::new();
    write_classes(&mut cn, &ProgressBarState { mode: props.mode.get() });
    props.style_class.with(|c| cn.add_opt(c.as_deref()));
    cn.add_opt(pt.root.extra_class());
    cn.finish()
}

fn root_style(props: &Props, pt: &ProgressBarPt) -> Option<String> {
    props.style.with(|st| (st.is_some() || pt.root.has_style()).then(|| pt.root.merge_style(st.as_deref().unwrap_or(""))))
}

/// Стиль полосы: ширина и `display: flex` в определённом режиме, цвет в обоих.
fn value_style(props: &Props, determinate: bool) -> Option<String> {
    let mut style = String::new();
    if determinate {
        if let Some(v) = props.value.get() {
            style.push_str(&format!("width: {v}%; "));
        }
        style.push_str("display: flex; ");
    }
    if let Some(color) = props.color.get() {
        style.push_str(&format!("background: {color}; "));
    }
    let style = style.trim_end().to_owned();
    (!style.is_empty()).then_some(style)
}

fn body_of(props: &Props, pt: &ProgressBarPt, data: &str) -> AnyView {
    let determinate = props.mode.get() == ProgressBarMode::Determinate;
    let mut cn = ClassNames::new();
    cn.add("p-progressbar-value");
    props.value_style_class.with(|c| cn.add_opt(c.as_deref()));
    cn.add_opt(pt.value.extra_class());
    let (class, style, value_attrs) = (cn.finish(), value_style(props, determinate), pt.value.attrs());
    if !determinate {
        return view! { <div class={class} style={style} data-p={data.to_owned()} {..value_attrs}></div> }.into_any();
    }
    let value = props.value.get();
    let inner = match props.content_template {
        Some(template) => Some(template.run(value)),
        None if props.show_value => {
            let shown = if value.is_some_and(|v| v != 0.0) { "display: flex" } else { "display: none" };
            let text = format!("{}{}", value.map(|v| v.to_string()).unwrap_or_default(), unit_of(props));
            Some(view! { <div style={shown}>{text}</div> }.into_any())
        }
        None => None,
    };
    let label_class = pt.label.merge_class("p-progressbar-label");
    view! {
        <div class={class} style={style} data-p={data.to_owned()} {..value_attrs}>
            <div class={label_class} data-p={data.to_owned()} {..pt.label.attrs()}>{inner}</div>
        </div>
    }
    .into_any()
}

/// Свойства экземпляра.
struct Props {
    value: OptProp<f64>,
    mode: Prop<ProgressBarMode>,
    show_value: bool,
    unit: OptProp<String>,
    color: OptProp<String>,
    style: OptProp<String>,
    style_class: OptProp<String>,
    value_style_class: OptProp<String>,
    content_template: Option<Callback<Option<f64>, AnyView>>,
}
