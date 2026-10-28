//! Badge — порт `p-badge` из PrimeNG 21.1.10.
//!
//! Разметка и классы совпадают с PrimeNG (`p-badge p-component` + модификаторы, `data-p`), но вместо
//! хост-элемента `<p-badge>` отрисовывается `<span>`: CSS темы привязан к классам, а не к имени тега.
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
use prime_core::{Pt, class_memo};

pub use state::{BadgeSeverity, BadgeSize, BadgeState, classes, data_p};

/// Pass-through частей Badge.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct BadgePt {
    /// Корневой элемент.
    pub root: Pt,
}

/// Бейдж: значение, серьёзность, размер; без значения — точка, из одного символа — круг.
///
/// `value`, `severity`, `size`, `badge_disabled` и `style_class` реактивны (принимают значение или сигнал).
#[component]
pub fn Badge(
    /// Значение (строка). Пусто — точка (`p-badge-dot`), один символ — круг (`p-badge-circle`).
    #[prop(optional, into)]
    value: MaybeProp<String>,
    /// Серьёзность.
    #[prop(optional, into)]
    severity: MaybeProp<BadgeSeverity>,
    /// Размер.
    #[prop(optional, into)]
    size: MaybeProp<BadgeSize>,
    /// Скрыть бейдж (`display: none`), оставив его в DOM.
    #[prop(optional, into)]
    badge_disabled: Signal<bool>,
    /// Дополнительный класс.
    #[prop(optional, into)]
    style_class: MaybeProp<String>,
    /// Pass-through.
    #[prop(optional)]
    pt: Option<BadgePt>,
) -> impl IntoView {
    let pt = pt.unwrap_or_default().root;
    let state =
        Memo::new(move |_| BadgeState { value: value.get(), severity: severity.get(), size: size.get(), disabled: badge_disabled.get() });
    let class = class_memo({
        let pt = pt.clone();
        move |cn| {
            cn.add(&classes(&state.read())).add_opt(style_class.get().as_deref());
            let merged = pt.merge_class("");
            cn.add(&merged);
        }
    });
    let style = {
        let pt = pt.clone();
        move || {
            let s = pt.merge_style(if state.read().disabled { "display: none" } else { "" });
            (!s.is_empty()).then_some(s)
        }
    };
    view! {
        <span class=class style=style data-p=move || data_p(&state.read()) {..pt.attrs()}>
            {move || state.read().value.clone()}
        </span>
    }
}
