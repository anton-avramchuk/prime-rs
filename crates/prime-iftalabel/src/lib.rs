//! IftaLabel — порт `p-iftalabel` из PrimeNG 21.1.10.
//!
//! Рисует `<span class="p-iftalabel">` со вложенными полем и `<label>`; положение метки над значением и отступы
//! поля задаёт CSS темы. Блок `/* For PrimeNG */` (`.ng-invalid.ng-dirty`) к этой разметке не применим: цвет
//! недопустимого значения даёт базовое правило `:has(.p-invalid) label`.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_iftalabel::IftaLabel;
//!
//! let html = view! { <IftaLabel><input class="p-inputtext" id="u" /><label for="u">"Имя"</label></IftaLabel> }.to_html();
//! assert!(html.contains(r#"class="p-iftalabel""#));
//! ```

use leptos::either::Either;
use leptos::prelude::*;
use prime_core::wrapper::{root_class, root_style};
use prime_core::{OptProp, Pt, all_static};
use std::sync::Arc;

/// Pass-through IftaLabel.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct IftaLabelPt {
    /// Корневой элемент.
    pub root: Pt,
}

/// Метка над значением внутри поля.
///
/// Свойства принимают значение или сигнал.
#[component]
pub fn IftaLabel(
    /// Встроенный стиль.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Дополнительный класс.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Pass-through.
    #[prop(optional)]
    pt: Option<IftaLabelPt>,
    /// Содержимое.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let root = pt.unwrap_or_default().root;
    let content = children.map(|c| c());
    let attrs = root.attrs();

    if all_static!(style, style_class) {
        let (class, style) = (root_class("p-iftalabel", &style_class, &root), root_style(&style, &root));
        Either::Left(iftalabel_markup!(class, style, attrs, content))
    } else {
        let root = Arc::new(root);
        let class = {
            let root = root.clone();
            move || root_class("p-iftalabel", &style_class, &root)
        };
        let style = move || root_style(&style, &root);
        Either::Right(iftalabel_markup!(class, style, attrs, content))
    }
}

/// Разметка: одна для статической и реактивной ветвей (меняются только выражения атрибутов корня).
macro_rules! iftalabel_markup {
    ($class:expr, $style:expr, $attrs:expr, $content:expr) => {
        view! { <span class={$class} style={$style} {..$attrs}>{$content}</span> }
    };
}
use iftalabel_markup;
