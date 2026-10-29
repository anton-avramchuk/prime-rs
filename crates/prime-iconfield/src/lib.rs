//! IconField и InputIcon — порт `p-iconfield` и `p-inputicon` из PrimeNG 21.1.10.
//!
//! `IconField` рисует `<span class="p-iconfield p-iconfield-left|right">`, `InputIcon` — `<span class="p-inputicon">`.
//! Положение значка внутри поля задаёт CSS темы по порядку дочерних элементов (`:first-child`/`:last-child`), класс
//! `p-iconfield-left|right` информационный. Значок передаётся дочерним элементом (например, SVG из `prime-icons`).
//! У `InputIcon` в PrimeNG нет собственной темы: его правила входят в `iconfield`.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_iconfield::{IconField, InputIcon};
//!
//! let html = view! { <IconField><InputIcon><svg /></InputIcon><input class="p-inputtext" /></IconField> }.to_html();
//! assert!(html.contains("p-iconfield p-iconfield-left") && html.contains("p-inputicon"));
//! ```

use leptos::either::Either;
use leptos::prelude::*;
use prime_core::wrapper::{root_class, root_style};
use prime_core::{ClassNames, OptProp, Prop, Pt, all_static};
use std::borrow::Cow;
use std::sync::Arc;

/// Положение значка относительно поля.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum IconPosition {
    /// Значок слева (по умолчанию).
    #[default]
    Left,
    /// Значок справа.
    Right,
}

/// Дописывает собственные классы корня `IconField`.
pub fn write_classes(cn: &mut ClassNames, position: IconPosition) {
    cn.add(match position {
        IconPosition::Left => "p-iconfield p-iconfield-left",
        IconPosition::Right => "p-iconfield p-iconfield-right",
    });
}

/// Pass-through IconField.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct IconFieldPt {
    /// Корневой элемент.
    pub root: Pt,
}

/// Поле со значком внутри.
///
/// Свойства принимают значение или сигнал.
#[component]
pub fn IconField(
    /// Положение значка.
    #[prop(optional, into)]
    icon_position: Prop<IconPosition>,
    /// Встроенный стиль.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Дополнительный класс.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Pass-through.
    #[prop(optional)]
    pt: Option<IconFieldPt>,
    /// Содержимое.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let root = pt.unwrap_or_default().root;
    let content = children.map(|c| c());
    let attrs = root.attrs();

    if all_static!(style, style_class, icon_position) {
        let (class, style) = (class_of(icon_position.get_untracked(), &style_class, &root), root_style(&style, &root));
        Either::Left(iconfield_markup!(class, style, attrs, content))
    } else {
        let root = Arc::new(root);
        let class = {
            let root = root.clone();
            move || class_of(icon_position.get(), &style_class, &root)
        };
        let style = move || root_style(&style, &root);
        Either::Right(iconfield_markup!(class, style, attrs, content))
    }
}

/// Разметка: одна для статической и реактивной ветвей (меняются только выражения атрибутов корня).
macro_rules! iconfield_markup {
    ($class:expr, $style:expr, $attrs:expr, $content:expr) => {
        view! { <span class={$class} style={$style} {..$attrs}>{$content}</span> }
    };
}
use iconfield_markup;

fn class_of(icon_position: IconPosition, style_class: &OptProp<String>, root: &Pt) -> Cow<'static, str> {
    let mut cn = ClassNames::new();
    write_classes(&mut cn, icon_position);
    style_class.with(|c| cn.add_user(c.as_deref()));
    cn.add_user(root.extra_class());
    cn.finish_cow()
}

/// Pass-through InputIcon.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct InputIconPt {
    /// Корневой элемент.
    pub root: Pt,
}

/// Значок внутри `IconField`.
///
/// Свойства принимают значение или сигнал.
#[component]
pub fn InputIcon(
    /// Встроенный стиль.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Дополнительный класс.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Pass-through.
    #[prop(optional)]
    pt: Option<InputIconPt>,
    /// Содержимое.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let root = pt.unwrap_or_default().root;
    let content = children.map(|c| c());
    let attrs = root.attrs();

    if all_static!(style, style_class) {
        let (class, style) = (root_class("p-inputicon", &style_class, &root), root_style(&style, &root));
        Either::Left(inputicon_markup!(class, style, attrs, content))
    } else {
        let root = Arc::new(root);
        let class = {
            let root = root.clone();
            move || root_class("p-inputicon", &style_class, &root)
        };
        let style = move || root_style(&style, &root);
        Either::Right(inputicon_markup!(class, style, attrs, content))
    }
}

/// Разметка: одна для статической и реактивной ветвей (меняются только выражения атрибутов корня).
macro_rules! inputicon_markup {
    ($class:expr, $style:expr, $attrs:expr, $content:expr) => {
        view! { <span class={$class} style={$style} {..$attrs}>{$content}</span> }
    };
}
use inputicon_markup;
