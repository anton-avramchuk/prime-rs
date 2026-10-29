//! FloatLabel — порт `p-floatlabel` из PrimeNG 21.1.10.
//!
//! Рисует `<span class="p-floatlabel p-floatlabel-over|on|in">` со вложенными полем и `<label>`; положение и подъём
//! метки задаёт CSS темы по состоянию поля. Блок `/* For PrimeNG */` (`.ng-invalid.ng-dirty`) здесь не нужен:
//! цвет недопустимого значения даёт базовое правило `:has(.p-invalid) label`.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_floatlabel::{FloatLabel, FloatLabelVariant};
//!
//! let html = view! {
//!     <FloatLabel variant=FloatLabelVariant::On><input class="p-inputtext" id="u" /><label for="u">"Имя"</label></FloatLabel>
//! }.to_html();
//! assert!(html.contains("p-floatlabel p-floatlabel-on"));
//! ```

use leptos::either::Either;
use leptos::prelude::*;
use prime_core::wrapper::root_style;
use prime_core::{ClassNames, OptProp, Prop, Pt, all_static};
use std::borrow::Cow;
use std::sync::Arc;

/// Вариант расположения метки.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FloatLabelVariant {
    /// Метка над полем при подъёме (по умолчанию).
    #[default]
    Over,
    /// Метка садится на верхнюю границу поля.
    On,
    /// Метка внутри поля.
    In,
}

/// Дописывает собственные классы корня: `p-floatlabel` и ровно один класс варианта.
pub fn write_classes(cn: &mut ClassNames, variant: FloatLabelVariant) {
    cn.add(match variant {
        FloatLabelVariant::Over => "p-floatlabel p-floatlabel-over",
        FloatLabelVariant::On => "p-floatlabel p-floatlabel-on",
        FloatLabelVariant::In => "p-floatlabel p-floatlabel-in",
    });
}

/// Pass-through FloatLabel.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FloatLabelPt {
    /// Корневой элемент.
    pub root: Pt,
}

/// Метка, поднимающаяся при фокусе или вводе.
///
/// Свойства принимают значение или сигнал.
#[component]
pub fn FloatLabel(
    /// Положение метки.
    #[prop(optional, into)]
    variant: Prop<FloatLabelVariant>,
    /// Встроенный стиль.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Дополнительный класс.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Pass-through.
    #[prop(optional)]
    pt: Option<FloatLabelPt>,
    /// Содержимое.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let root = pt.unwrap_or_default().root;
    let content = children.map(|c| c());
    let attrs = root.attrs();

    if all_static!(style, style_class, variant) {
        let (class, style) = (class_of(variant.get_untracked(), &style_class, &root), root_style(&style, &root));
        Either::Left(floatlabel_markup!(class, style, attrs, content))
    } else {
        let root = Arc::new(root);
        let class = {
            let root = root.clone();
            move || class_of(variant.get(), &style_class, &root)
        };
        let style = move || root_style(&style, &root);
        Either::Right(floatlabel_markup!(class, style, attrs, content))
    }
}

/// Разметка: одна для статической и реактивной ветвей (меняются только выражения атрибутов корня).
macro_rules! floatlabel_markup {
    ($class:expr, $style:expr, $attrs:expr, $content:expr) => {
        view! { <span class={$class} style={$style} {..$attrs}>{$content}</span> }
    };
}
use floatlabel_markup;

fn class_of(variant: FloatLabelVariant, style_class: &OptProp<String>, root: &Pt) -> Cow<'static, str> {
    let mut cn = ClassNames::new();
    write_classes(&mut cn, variant);
    style_class.with(|c| cn.add_user(c.as_deref()));
    cn.add_user(root.extra_class());
    cn.finish_cow()
}
