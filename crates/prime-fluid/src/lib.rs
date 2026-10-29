//! Fluid — порт `p-fluid` из PrimeNG 21.1.10.
//!
//! Рисует `<div class="p-fluid">` и предоставляет потомкам контекст [`prime_core::fluid`]: `Button`, `InputText`,
//! `Textarea`, `InputNumber` и `Select` без явного `fluid` растягиваются на всю ширину. Явное `fluid` (в том числе `false`)
//! главнее контекста. Собственных стилей у `Fluid` нет (модуля `@primeuix/styles/fluid` не существует).
//!
//! ```
//! use leptos::prelude::*;
//! use prime_fluid::Fluid;
//!
//! let html = view! { <Fluid><input class="p-inputtext" /></Fluid> }.to_html();
//! assert!(html.contains(r#"class="p-fluid""#));
//! ```

use leptos::either::Either;
use leptos::prelude::*;
use prime_core::wrapper::{root_class, root_style};
use prime_core::{OptProp, Pt, all_static};
use std::sync::Arc;

/// Pass-through Fluid.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FluidPt {
    /// Корневой элемент.
    pub root: Pt,
}

/// Контейнер, растягивающий вложенные поля и кнопки на всю ширину.
///
/// Свойства принимают значение или сигнал.
#[component]
pub fn Fluid(
    /// Встроенный стиль.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Дополнительный класс.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Pass-through.
    #[prop(optional)]
    pt: Option<FluidPt>,
    /// Содержимое.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let root = pt.unwrap_or_default().root;
    // Контекст кладётся в дочернего владельца, чтобы не протечь на соседей, идущих после `Fluid` в том же владельце.
    // Владелец держится до очистки родителя: сам по себе он освободился бы сразу и унёс бы за собой реактивные значения потомков.
    let content = children.map(|c| match Owner::current() {
        Some(parent) => {
            let owner = parent.child();
            let content = owner.with(|| {
                prime_core::fluid::provide_fluid();
                c()
            });
            on_cleanup(move || drop(owner));
            content
        }
        None => c(),
    });
    let attrs = root.attrs();

    if all_static!(style, style_class) {
        let (class, style) = (root_class("p-fluid", &style_class, &root), root_style(&style, &root));
        Either::Left(fluid_markup!(class, style, attrs, content))
    } else {
        let root = Arc::new(root);
        let class = {
            let root = root.clone();
            move || root_class("p-fluid", &style_class, &root)
        };
        let style = move || root_style(&style, &root);
        Either::Right(fluid_markup!(class, style, attrs, content))
    }
}

/// Разметка: одна для статической и реактивной ветвей (меняются только выражения атрибутов корня).
macro_rules! fluid_markup {
    ($class:expr, $style:expr, $attrs:expr, $content:expr) => {
        view! { <div class={$class} style={$style} {..$attrs}>{$content}</div> }
    };
}
use fluid_markup;
