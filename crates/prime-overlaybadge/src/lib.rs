//! OverlayBadge — порт `p-overlaybadge` из PrimeNG 21.1.10.
//!
//! Корень `div.p-overlaybadge` (вместо хоста `<p-overlaybadge>`) с дочерним содержимым и вложенным [`Badge`].
//! `style_class` и `style` относятся к бейджу, как в PrimeNG; положение бейджа в углу задаёт CSS темы.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_overlaybadge::{OverlayBadge, BadgeSeverity};
//!
//! let html = view! { <OverlayBadge value="2" severity=BadgeSeverity::Danger><span>"x"</span></OverlayBadge> }.to_html();
//! assert!(html.contains("p-overlaybadge") && html.contains("p-badge-danger"));
//! ```

use leptos::prelude::*;
use prime_badge::{Badge, BadgePt};
use prime_core::{OptProp, Prop, Pt};

pub use prime_badge::{BadgeSeverity, BadgeSize};

/// Pass-through частей OverlayBadge.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct OverlayBadgePt {
    /// Корневой элемент.
    pub root: Pt,
    /// Вложенный бейдж.
    pub pc_badge: BadgePt,
}

/// Бейдж, наложенный на правый верхний угол содержимого.
///
/// Каждое свойство принимает значение или сигнал; `style` читается один раз при создании.
#[component]
pub fn OverlayBadge(
    /// Значение бейджа; без значения бейдж — точка.
    #[prop(optional, into)]
    value: OptProp<String>,
    /// Серьёзность бейджа.
    #[prop(optional, into)]
    severity: OptProp<BadgeSeverity>,
    /// Размер бейджа.
    #[prop(optional, into)]
    badge_size: OptProp<BadgeSize>,
    /// Скрыть бейдж, оставив содержимое.
    #[prop(optional, into)]
    badge_disabled: Prop<bool>,
    /// Дополнительный класс бейджа.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Встроенный стиль бейджа.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Pass-through.
    #[prop(optional)]
    pt: Option<OverlayBadgePt>,
    /// Содержимое, на которое накладывается бейдж.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let OverlayBadgePt { root, mut pc_badge } = pt.unwrap_or_default();
    if let Some(st) = style.get_untracked() {
        let merged = pc_badge.root.merge_style(&st);
        pc_badge.root = pc_badge.root.style(merged);
    }
    let attrs = root.attrs();
    let class = root.merge_class("p-overlaybadge");
    let style = root.has_style().then(|| root.merge_style(""));
    let kids = children.map(|c| c());
    view! {
        <div class=class style={style} {..attrs}>
            {kids}
            <Badge value=value severity=severity size=badge_size badge_disabled=badge_disabled style_class=style_class pt=pc_badge />
        </div>
    }
}
