//! Badge — порт `p-badge` из PrimeNG 21.1.10.
//!
//! Разметка и классы совпадают с PrimeNG (`p-badge p-component` + модификаторы, `data-p`), но вместо
//! хост-элемента `<p-badge>` отрисовывается `<span>`: CSS темы привязан к классам, а не к имени тега.
//!
//! Свойства — [`Prop`]/[`OptProp`]: значение не создаёт реактивных примитивов, сигнал подписывает атрибуты.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_badge::{Badge, BadgeSeverity};
//!
//! let html = view! { <Badge value="5" severity=BadgeSeverity::Danger /> }.to_html();
//! assert!(html.contains("p-badge-circle") && html.contains("p-badge-danger"));
//! ```

mod state;

use leptos::prelude::*;
use prime_core::{ClassNames, OptProp, Prop, Pt};
use std::sync::Arc;

pub use state::{BadgeSeverity, BadgeSize, BadgeState, classes, data_p, write_classes};

/// Pass-through частей Badge.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct BadgePt {
    /// Корневой элемент.
    pub root: Pt,
}

/// Бейдж: значение, серьёзность, размер; без значения — точка, из одного символа — круг.
///
/// Каждое свойство принимает значение или сигнал; значение не создаёт реактивных примитивов.
#[component]
pub fn Badge(
    /// Значение (строка). Пусто — точка (`p-badge-dot`), один символ — круг (`p-badge-circle`).
    #[prop(optional, into)]
    value: OptProp<String>,
    /// Серьёзность.
    #[prop(optional, into)]
    severity: OptProp<BadgeSeverity>,
    /// Размер.
    #[prop(optional, into)]
    size: OptProp<BadgeSize>,
    /// Скрыть бейдж (`display: none`), оставив его в DOM.
    #[prop(optional, into)]
    badge_disabled: Prop<bool>,
    /// Дополнительный класс.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Pass-through.
    #[prop(optional)]
    pt: Option<BadgePt>,
) -> impl IntoView {
    let pt = Arc::new(pt.unwrap_or_default().root);
    // Все свойства в одном `Arc`: замыкания атрибутов клонируют счётчик ссылок, а не строки значений.
    let props = Arc::new(Props { value, severity, size, disabled: badge_disabled, style_class });

    let class = {
        let (props, pt) = (props.clone(), pt.clone());
        move || {
            let mut cn = ClassNames::new();
            props.with_state(|s| write_classes(&mut cn, s));
            props.style_class.with(|c| cn.add_opt(c.as_deref()));
            cn.add_opt(pt.extra_class());
            cn.finish()
        }
    };
    let data_attr = {
        let props = props.clone();
        move || props.with_state(data_p)
    };
    let style = {
        let (props, pt) = (props.clone(), pt.clone());
        move || {
            let hidden = props.disabled.get();
            (hidden || pt.has_style()).then(|| pt.merge_style(if hidden { "display: none" } else { "" }))
        }
    };
    let text = {
        let props = props.clone();
        move || props.value.get()
    };
    let attrs = pt.attrs();

    view! {
        <span class=class style=style data-p={data_attr} {..attrs}>
            {text}
        </span>
    }
}

/// Свойства экземпляра.
struct Props {
    value: OptProp<String>,
    severity: OptProp<BadgeSeverity>,
    size: OptProp<BadgeSize>,
    disabled: Prop<bool>,
    style_class: OptProp<String>,
}

impl Props {
    /// Вызывает `f` с заимствованным снимком состояния (без копирования значения).
    fn with_state<R>(&self, f: impl FnOnce(&BadgeState) -> R) -> R {
        self.value.with(|v| {
            f(&BadgeState { value: v.as_deref(), severity: self.severity.get(), size: self.size.get(), disabled: self.disabled.get() })
        })
    }
}
