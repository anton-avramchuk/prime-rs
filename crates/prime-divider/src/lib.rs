//! Divider — порт `p-divider` из PrimeNG 21.1.10.
//!
//! Разметка, классы, `role="separator"`, `aria-orientation`, inline-стиль выравнивания и `data-p` совпадают с PrimeNG;
//! вместо хост-элемента `<p-divider>` отрисовывается `<div class="p-divider …">` с блоком `p-divider-content` внутри.
//!
//! Свойства — [`Prop`]/[`OptProp`]; если все они статические, компонент рисует готовые значения без замыканий и `RenderEffect`.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_divider::{Divider, DividerAlign, DividerType};
//!
//! let html = view! { <Divider align=DividerAlign::Right divider_type=DividerType::Dashed>"OR"</Divider> }.to_html();
//! assert!(html.contains("p-divider-right") && html.contains("p-divider-dashed") && html.contains("justify-content: flex-end"));
//! ```

mod state;

use leptos::either::Either;
use leptos::prelude::*;
use prime_core::{ClassNames, OptProp, Prop, Pt, all_static};
use std::sync::Arc;

pub use state::{DividerAlign, DividerLayout, DividerState, DividerType, align_style, data_p, write_classes};

/// Pass-through частей Divider.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DividerPt {
    /// Корневой элемент.
    pub root: Pt,
    /// Блок содержимого (`div.p-divider-content`).
    pub content: Pt,
}

/// Разделитель: направление, тип линии, выравнивание содержимого.
///
/// Каждое свойство принимает значение или сигнал. `children` выводятся внутри `div.p-divider-content`.
#[component]
pub fn Divider(
    /// Направление (по умолчанию `horizontal`).
    #[prop(optional, into)]
    layout: Prop<DividerLayout>,
    /// Тип линии (по умолчанию `solid`); атрибута `type` в DOM нет.
    #[prop(optional, into)]
    divider_type: Prop<DividerType>,
    /// Выравнивание содержимого.
    #[prop(optional, into)]
    align: OptProp<DividerAlign>,
    /// Встроенный стиль.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Дополнительный класс.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Pass-through.
    #[prop(optional)]
    pt: Option<DividerPt>,
    /// Содержимое внутри блока `p-divider-content`.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let pt = Arc::new(pt.unwrap_or_default());
    let is_static = all_static!(layout, divider_type, align, style, style_class);
    let props = Arc::new(Props { layout, divider_type, align, style, style_class });
    let content = children.map(|c| c());
    let (attrs, content_class, content_attrs) = (pt.root.attrs(), pt.content.merge_class("p-divider-content"), pt.content.attrs());

    if is_static {
        let (class, style, data, orientation) =
            (class_of(&props, &pt), style_of(&props, &pt), data_of(&props), props.layout.get().as_str());
        Either::Left(divider_markup!(class, style, data, orientation, attrs, content_class, content_attrs, content))
    } else {
        let class = {
            let (props, pt) = (props.clone(), pt.clone());
            move || class_of(&props, &pt)
        };
        let style = {
            let (props, pt) = (props.clone(), pt.clone());
            move || style_of(&props, &pt)
        };
        let data = {
            let props = props.clone();
            move || data_of(&props)
        };
        let orientation = move || props.layout.get().as_str();
        Either::Right(divider_markup!(class, style, data, orientation, attrs, content_class, content_attrs, content))
    }
}

/// Разметка Divider: одна для статической и реактивной ветвей (меняются только выражения атрибутов).
macro_rules! divider_markup {
    ($class:expr, $style:expr, $data:expr, $orientation:expr, $attrs:expr, $content_class:expr, $content_attrs:expr, $content:expr) => {
        view! {
            <div class={$class} style={$style} role="separator" aria-orientation={$orientation} data-p={$data} {..$attrs}>
                <div class={$content_class} {..$content_attrs}>
                    {$content}
                </div>
            </div>
        }
    };
}
use divider_markup;

fn state_of(props: &Props) -> DividerState {
    DividerState { layout: props.layout.get(), divider_type: props.divider_type.get(), align: props.align.get() }
}

fn class_of(props: &Props, pt: &DividerPt) -> String {
    let mut cn = ClassNames::new();
    write_classes(&mut cn, &state_of(props));
    props.style_class.with(|c| cn.add_opt(c.as_deref()));
    cn.add_opt(pt.root.extra_class());
    cn.finish()
}

/// Стиль корня: выравнивание, затем `style` и стиль из `pt`.
fn style_of(props: &Props, pt: &DividerPt) -> Option<String> {
    let align = align_style(&state_of(props)).map(|(k, v)| format!("{k}: {v}")).unwrap_or_default();
    props.style.with(|st| {
        let merged = pt.root.merge_style(&Pt::new().style(st.as_deref().unwrap_or("")).merge_style(&align));
        (!merged.is_empty()).then_some(merged)
    })
}

fn data_of(props: &Props) -> String {
    data_p(&state_of(props))
}

/// Свойства экземпляра.
struct Props {
    layout: Prop<DividerLayout>,
    divider_type: Prop<DividerType>,
    align: OptProp<DividerAlign>,
    style: OptProp<String>,
    style_class: OptProp<String>,
}
