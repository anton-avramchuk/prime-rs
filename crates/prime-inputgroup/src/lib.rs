//! InputGroup и InputGroupAddon — порт `p-inputgroup` и `p-inputgroup-addon` из PrimeNG 21.1.10.
//!
//! Структура и классы совпадают с PrimeNG: `div.p-inputgroup` и `div.p-inputgroupaddon` с проецируемым содержимым.
//! Вместо хостов `<p-inputgroup>` и `<p-inputgroup-addon>` отрисовываются `<div class="…">`. Блок `/*For PrimeNG*/`
//! из `inputgroupstyle.ts` (правила для Angular-обёрток `p-button`, `p-inputwrapper`, `p-inputmask`) не нужен.
//!
//! Свойства — [`OptProp`]; если все они статические, компонент рисует готовые значения без замыканий и `RenderEffect`.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_inputgroup::{InputGroup, InputGroupAddon};
//!
//! let html = view! { <InputGroup><InputGroupAddon>"$"</InputGroupAddon><input class="p-inputtext" /></InputGroup> }.to_html();
//! assert!(html.contains("p-inputgroup") && html.contains("p-inputgroupaddon"));
//! ```

use leptos::either::Either;
use leptos::prelude::*;
use prime_core::{ClassNames, OptProp, Pt, all_static};
use std::borrow::Cow;
use std::sync::Arc;

/// Pass-through InputGroup.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct InputGroupPt {
    /// Корневой элемент.
    pub root: Pt,
}

/// Pass-through InputGroupAddon.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct InputGroupAddonPt {
    /// Корневой элемент.
    pub root: Pt,
}

/// Группа: поле ввода с текстом, иконками и кнопками рядом.
///
/// Свойства принимают значение или сигнал.
#[component]
pub fn InputGroup(
    /// Встроенный стиль.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Дополнительный класс.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Pass-through.
    #[prop(optional)]
    pt: Option<InputGroupPt>,
    /// Содержимое группы.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    group_view("p-inputgroup", style, style_class, pt.unwrap_or_default().root, children)
}

/// Аддон группы: текст, иконка или кнопка рядом с полем.
///
/// Свойства принимают значение или сигнал.
#[component]
pub fn InputGroupAddon(
    /// Встроенный стиль.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Дополнительный класс.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Pass-through.
    #[prop(optional)]
    pt: Option<InputGroupAddonPt>,
    /// Содержимое аддона.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    group_view("p-inputgroupaddon", style, style_class, pt.unwrap_or_default().root, children)
}

/// Общая разметка обоих компонентов; различается только собственный класс.
fn group_view(
    base: &'static str,
    style: OptProp<String>,
    style_class: OptProp<String>,
    root: Pt,
    children: Option<Children>,
) -> impl IntoView {
    let is_static = all_static!(style, style_class);
    let props = Props { style, style_class };
    let content = children.map(|c| c());
    let attrs = root.attrs();

    if is_static {
        let (class, style) = (class_of(base, &props, &root), style_of(&props, &root));
        Either::Left(group_markup!(class, style, attrs, content))
    } else {
        let (props, root) = (Arc::new(props), Arc::new(root));
        let class = {
            let (props, root) = (props.clone(), root.clone());
            move || class_of(base, &props, &root)
        };
        let style = move || style_of(&props, &root);
        Either::Right(group_markup!(class, style, attrs, content))
    }
}

/// Разметка: одна для статической и реактивной ветвей (меняются только выражения атрибутов корня).
macro_rules! group_markup {
    ($class:expr, $style:expr, $attrs:expr, $content:expr) => {
        view! { <div class={$class} style={$style} {..$attrs}>{$content}</div> }
    };
}
use group_markup;

fn class_of(base: &'static str, props: &Props, root: &Pt) -> Cow<'static, str> {
    let mut cn = ClassNames::new();
    cn.add(base);
    props.style_class.with(|c| cn.add_user(c.as_deref()));
    cn.add_user(root.extra_class());
    cn.finish_cow()
}

fn style_of(props: &Props, root: &Pt) -> Option<String> {
    props.style.with(|st| (st.is_some() || root.has_style()).then(|| root.merge_style(st.as_deref().unwrap_or(""))))
}

/// Свойства экземпляра.
struct Props {
    style: OptProp<String>,
    style_class: OptProp<String>,
}
