//! ButtonGroup — порт `p-buttongroup` из PrimeNG 21.1.10.
//!
//! Вместо хоста `<p-buttongroup>` отрисовывается `<span class="p-buttongroup p-component" role="group">`
//! с проецируемыми кнопками. Слитный вид (общая граница, скругления только у крайних кнопок) задаёт CSS темы.
//! Блок `/* For PrimeNG */` из `buttongroupstyle.ts` нацелен на хосты `p-button` и здесь не нужен.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_buttongroup::ButtonGroup;
//!
//! let html = view! { <ButtonGroup><button class="p-button">"A"</button><button class="p-button">"B"</button></ButtonGroup> }.to_html();
//! assert!(html.contains("p-buttongroup p-component") && html.contains(r#"role="group""#));
//! ```

use leptos::either::Either;
use leptos::prelude::*;
use prime_core::wrapper::{root_class, root_style};
use prime_core::{OptProp, Pt, all_static};
use std::sync::Arc;

/// Pass-through ButtonGroup.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ButtonGroupPt {
    /// Корневой элемент.
    pub root: Pt,
}

/// Группа слитных кнопок.
///
/// Свойства принимают значение или сигнал.
#[component]
pub fn ButtonGroup(
    /// Встроенный стиль.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Дополнительный класс.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Pass-through.
    #[prop(optional)]
    pt: Option<ButtonGroupPt>,
    /// Содержимое.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let root = pt.unwrap_or_default().root;
    let content = children.map(|c| c());
    let attrs = root.attrs();

    if all_static!(style, style_class) {
        let (class, style) = (root_class("p-buttongroup p-component", &style_class, &root), root_style(&style, &root));
        Either::Left(buttongroup_markup!(class, style, attrs, content))
    } else {
        let root = Arc::new(root);
        let class = {
            let root = root.clone();
            move || root_class("p-buttongroup p-component", &style_class, &root)
        };
        let style = move || root_style(&style, &root);
        Either::Right(buttongroup_markup!(class, style, attrs, content))
    }
}

/// Разметка: одна для статической и реактивной ветвей (меняются только выражения атрибутов корня).
macro_rules! buttongroup_markup {
    ($class:expr, $style:expr, $attrs:expr, $content:expr) => {
        view! { <span class={$class} style={$style} role="group" {..$attrs}>{$content}</span> }
    };
}
use buttongroup_markup;
