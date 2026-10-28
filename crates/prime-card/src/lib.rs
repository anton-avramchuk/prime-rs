//! Card — порт `p-card` из PrimeNG 21.1.10.
//!
//! Структура и классы совпадают с PrimeNG: шапка (`p-card-header`, если задан `header_template`), тело `p-card-body` с заголовком,
//! подзаголовком, содержимым `p-card-content` и подвалом (`p-card-footer`, если задан `footer_template`). Вместо хост-элемента `<p-card>`
//! отрисовывается `<div class="p-card …">`. Слоты — `ViewFn` (аналог шаблонов `#header`, `#title`, `#subtitle`, `#footer`).
//!
//! Свойства — [`Prop`]/[`OptProp`]; если все они статические, компонент рисует готовые значения без замыканий и `RenderEffect`.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_card::Card;
//!
//! let html = view! { <Card header="Simple Card"><p>"Содержимое"</p></Card> }.to_html();
//! assert!(html.contains("p-card-title") && html.contains("Simple Card") && html.contains("p-card-content"));
//! ```

use leptos::either::Either;
use leptos::prelude::*;
use prime_core::{ClassNames, OptProp, Pt, all_static};
use std::sync::Arc;

/// Pass-through частей Card.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CardPt {
    /// Корневой элемент.
    pub root: Pt,
    /// Шапка (`div.p-card-header`).
    pub header: Pt,
    /// Тело (`div.p-card-body`).
    pub body: Pt,
    /// Заголовок (`div.p-card-title`).
    pub title: Pt,
    /// Подзаголовок (`div.p-card-subtitle`).
    pub subtitle: Pt,
    /// Содержимое (`div.p-card-content`).
    pub content: Pt,
    /// Подвал (`div.p-card-footer`).
    pub footer: Pt,
}

/// Карточка: шапка, заголовок, подзаголовок, содержимое, подвал.
///
/// Текстовые свойства принимают значение или сигнал. Слот имеет приоритет над соответствующим текстом.
#[component]
pub fn Card(
    /// Текст заголовка (`p-card-title`), если нет `title_template`.
    #[prop(optional, into)]
    header: OptProp<String>,
    /// Текст подзаголовка (`p-card-subtitle`), если нет `subtitle_template`.
    #[prop(optional, into)]
    subheader: OptProp<String>,
    /// Слот шапки (`p-card-header`), например изображение.
    #[prop(optional, into)]
    header_template: Option<ViewFn>,
    /// Слот заголовка; заменяет текст `header`.
    #[prop(optional, into)]
    title_template: Option<ViewFn>,
    /// Слот подзаголовка; заменяет текст `subheader`.
    #[prop(optional, into)]
    subtitle_template: Option<ViewFn>,
    /// Слот подвала (`p-card-footer`).
    #[prop(optional, into)]
    footer_template: Option<ViewFn>,
    /// Встроенный стиль.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Дополнительный класс.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Pass-through.
    #[prop(optional)]
    pt: Option<CardPt>,
    /// Содержимое (`p-card-content`).
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let pt = Arc::new(pt.unwrap_or_default());
    let is_static = all_static!(header, subheader, style, style_class);
    let props =
        Arc::new(Props { header, subheader, style, style_class, header_template, title_template, subtitle_template, footer_template });
    let content = children.map(|c| c());
    let (attrs, body_class, body_attrs, content_class, content_attrs) = (
        pt.root.attrs(),
        pt.body.merge_class("p-card-body"),
        pt.body.attrs(),
        pt.content.merge_class("p-card-content"),
        pt.content.attrs(),
    );

    if is_static {
        let (class, style) = (class_of(&props, &pt), style_of(&props, &pt));
        let (head, title, subtitle, footer) =
            (header_of(&props, &pt), title_of(&props, &pt), subtitle_of(&props, &pt), footer_of(&props, &pt));
        Either::Left(card_markup!(
            class,
            style,
            attrs,
            head,
            body_class,
            body_attrs,
            title,
            subtitle,
            content_class,
            content_attrs,
            content,
            footer
        ))
    } else {
        let class = {
            let (props, pt) = (props.clone(), pt.clone());
            move || class_of(&props, &pt)
        };
        let style = {
            let (props, pt) = (props.clone(), pt.clone());
            move || style_of(&props, &pt)
        };
        let head = {
            let (props, pt) = (props.clone(), pt.clone());
            move || header_of(&props, &pt)
        };
        let title = {
            let (props, pt) = (props.clone(), pt.clone());
            move || title_of(&props, &pt)
        };
        let subtitle = {
            let (props, pt) = (props.clone(), pt.clone());
            move || subtitle_of(&props, &pt)
        };
        let footer = move || footer_of(&props, &pt);
        Either::Right(card_markup!(
            class,
            style,
            attrs,
            head,
            body_class,
            body_attrs,
            title,
            subtitle,
            content_class,
            content_attrs,
            content,
            footer
        ))
    }
}

/// Разметка Card: одна для статической и реактивной ветвей (меняются только выражения атрибутов и блоков).
macro_rules! card_markup {
    ($class:expr, $style:expr, $attrs:expr, $head:expr, $body_class:expr, $body_attrs:expr, $title:expr, $subtitle:expr, $content_class:expr, $content_attrs:expr, $content:expr, $footer:expr) => {
        view! {
            <div class={$class} style={$style} {..$attrs}>
                {$head}
                <div class={$body_class} {..$body_attrs}>
                    {$title}
                    {$subtitle}
                    <div class={$content_class} {..$content_attrs}>
                        {$content}
                    </div>
                    {$footer}
                </div>
            </div>
        }
    };
}
use card_markup;

fn class_of(props: &Props, pt: &CardPt) -> String {
    let mut cn = ClassNames::new();
    cn.add("p-card p-component");
    props.style_class.with(|c| cn.add_opt(c.as_deref()));
    cn.add_opt(pt.root.extra_class());
    cn.finish()
}

fn style_of(props: &Props, pt: &CardPt) -> Option<String> {
    props.style.with(|st| (st.is_some() || pt.root.has_style()).then(|| pt.root.merge_style(st.as_deref().unwrap_or(""))))
}

/// Шапка: только при заданном слоте.
fn header_of(props: &Props, pt: &CardPt) -> Option<AnyView> {
    props
        .header_template
        .clone()
        .map(|slot| view! { <div class=pt.header.merge_class("p-card-header") {..pt.header.attrs()}>{slot.run()}</div> }.into_any())
}

/// Заголовок: слот заменяет текст.
fn title_of(props: &Props, pt: &CardPt) -> Option<AnyView> {
    block(&pt.title, "p-card-title", props.title_template.clone(), props.header.get())
}

/// Подзаголовок: слот заменяет текст.
fn subtitle_of(props: &Props, pt: &CardPt) -> Option<AnyView> {
    block(&pt.subtitle, "p-card-subtitle", props.subtitle_template.clone(), props.subheader.get())
}

/// Подвал: только при заданном слоте.
fn footer_of(props: &Props, pt: &CardPt) -> Option<AnyView> {
    props
        .footer_template
        .clone()
        .map(|slot| view! { <div class=pt.footer.merge_class("p-card-footer") {..pt.footer.attrs()}>{slot.run()}</div> }.into_any())
}

/// Блок с текстом или слотом (слот приоритетнее); `None`, если нет ни того, ни другого.
fn block(pt: &Pt, class: &'static str, slot: Option<ViewFn>, text: Option<String>) -> Option<AnyView> {
    let inner = match (slot, text.filter(|t| !t.is_empty())) {
        (Some(slot), _) => slot.run(),
        (None, Some(text)) => text.into_any(),
        (None, None) => return None,
    };
    Some(view! { <div class=pt.merge_class(class) {..pt.attrs()}>{inner}</div> }.into_any())
}

/// Свойства экземпляра.
struct Props {
    header: OptProp<String>,
    subheader: OptProp<String>,
    style: OptProp<String>,
    style_class: OptProp<String>,
    header_template: Option<ViewFn>,
    title_template: Option<ViewFn>,
    subtitle_template: Option<ViewFn>,
    footer_template: Option<ViewFn>,
}
