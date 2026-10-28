//! Toolbar — порт `p-toolbar` из PrimeNG 21.1.10.
//!
//! Структура и классы совпадают с PrimeNG: корень `div.p-toolbar` с `role="toolbar"`, внутри проецируемое содержимое
//! и области `p-toolbar-start`, `p-toolbar-center`, `p-toolbar-end`, которые выводятся только при заданном слоте.
//! Вместо хоста `<p-toolbar>` отрисовывается `<div class="p-toolbar …">`. Слоты — `ViewFn` (аналоги шаблонов `#start`, `#center`, `#end`).
//!
//! Свойства — [`Prop`]/[`OptProp`]; если все они статические, компонент рисует готовые значения без замыканий и `RenderEffect`.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_toolbar::Toolbar;
//!
//! let html = view! { <Toolbar start_template=ViewFn::from(|| view! { "Начало" }) end_template=ViewFn::from(|| view! { "Конец" }) /> }.to_html();
//! assert!(html.contains(r#"role="toolbar""#) && html.contains("p-toolbar-start") && html.contains("p-toolbar-end") && !html.contains("p-toolbar-center"));
//! ```

use leptos::either::Either;
use leptos::prelude::*;
use prime_core::{ClassNames, OptProp, Pt, all_static};
use std::borrow::Cow;
use std::sync::Arc;

/// Pass-through частей Toolbar.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ToolbarPt {
    /// Корневой элемент.
    pub root: Pt,
    /// Область начала (`div.p-toolbar-start`).
    pub start: Pt,
    /// Центральная область (`div.p-toolbar-center`).
    pub center: Pt,
    /// Область конца (`div.p-toolbar-end`).
    pub end: Pt,
}

/// Панель инструментов: начало, центр и конец.
///
/// Свойства принимают значение или сигнал. `children` выводятся перед областями.
#[component]
pub fn Toolbar(
    /// Атрибут `aria-labelledby`.
    #[prop(optional, into)]
    aria_labelledby: OptProp<String>,
    /// Встроенный стиль.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Дополнительный класс.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Слот области начала (`p-toolbar-start`).
    #[prop(optional, into)]
    start_template: Option<ViewFn>,
    /// Слот центральной области (`p-toolbar-center`).
    #[prop(optional, into)]
    center_template: Option<ViewFn>,
    /// Слот области конца (`p-toolbar-end`).
    #[prop(optional, into)]
    end_template: Option<ViewFn>,
    /// Pass-through.
    #[prop(optional)]
    pt: Option<ToolbarPt>,
    /// Проецируемое содержимое: стоит перед областями.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let pt = pt.unwrap_or_default();
    let is_static = all_static!(aria_labelledby, style, style_class);
    let props = Props { aria_labelledby, style, style_class };
    let content = children.map(|c| c());
    let attrs = pt.root.attrs();
    let start = area(&pt.start, "p-toolbar-start", start_template);
    let center = area(&pt.center, "p-toolbar-center", center_template);
    let end = area(&pt.end, "p-toolbar-end", end_template);

    if is_static {
        let (class, style, aria) = (class_of(&props, &pt), style_of(&props, &pt), props.aria_labelledby.get());
        Either::Left(toolbar_markup!(class, style, aria, attrs, content, start, center, end))
    } else {
        let (props, pt) = (Arc::new(props), Arc::new(pt));
        let class = {
            let (props, pt) = (props.clone(), pt.clone());
            move || class_of(&props, &pt)
        };
        let style = {
            let (props, pt) = (props.clone(), pt);
            move || style_of(&props, &pt)
        };
        let aria = move || props.aria_labelledby.get();
        Either::Right(toolbar_markup!(class, style, aria, attrs, content, start, center, end))
    }
}

/// Разметка Toolbar: одна для статической и реактивной ветвей (меняются только выражения атрибутов корня).
macro_rules! toolbar_markup {
    ($class:expr, $style:expr, $aria:expr, $attrs:expr, $content:expr, $start:expr, $center:expr, $end:expr) => {
        view! {
            <div class={$class} style={$style} role="toolbar" aria-labelledby={$aria} {..$attrs}>
                {$content}
                {$start}
                {$center}
                {$end}
            </div>
        }
    };
}
use toolbar_markup;

fn class_of(props: &Props, pt: &ToolbarPt) -> Cow<'static, str> {
    let mut cn = ClassNames::new();
    cn.add("p-toolbar p-component");
    props.style_class.with(|c| cn.add_user(c.as_deref()));
    cn.add_user(pt.root.extra_class());
    cn.finish_cow()
}

fn style_of(props: &Props, pt: &ToolbarPt) -> Option<String> {
    props.style.with(|st| (st.is_some() || pt.root.has_style()).then(|| pt.root.merge_style(st.as_deref().unwrap_or(""))))
}

/// Область: только при заданном слоте.
fn area(pt: &Pt, class: &'static str, slot: Option<ViewFn>) -> Option<AnyView> {
    slot.map(|slot| {
        let style = pt.has_style().then(|| pt.merge_style(""));
        view! { <div class=pt.merge_class(class) style={style} {..pt.attrs()}>{slot.run()}</div> }.into_any()
    })
}

/// Свойства экземпляра.
struct Props {
    aria_labelledby: OptProp<String>,
    style: OptProp<String>,
    style_class: OptProp<String>,
}
