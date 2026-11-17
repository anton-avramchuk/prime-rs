//! Card — port of `p-card` from PrimeNG 21.1.10.
//!
//! Structure and classes match PrimeNG: a header (`p-card-header`, if `header_template` is set), a `p-card-body` body with a title,
//! a subtitle, `p-card-content` content and a footer (`p-card-footer`, if `footer_template` is set). Instead of the `<p-card>` host element
//! a `<div class="p-card …">` is rendered. Slots are `ViewFn` (the counterpart of the `#header`, `#title`, `#subtitle`, `#footer` templates).
//!
//! Properties are [`Prop`]/[`OptProp`]; if all of them are static, the component renders ready values without closures and `RenderEffect`.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_card::Card;
//!
//! let html = view! { <Card header="Simple Card"><p>"Content"</p></Card> }.to_html();
//! assert!(html.contains("p-card-title") && html.contains("Simple Card") && html.contains("p-card-content"));
//! ```

use leptos::either::Either;
use leptos::prelude::*;
use prime_rs_core::{ClassNames, OptProp, Pt, all_static};
use std::borrow::Cow;
use std::sync::Arc;

/// Pass-through of the Card parts.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CardPt {
    /// Root element.
    pub root: Pt,
    /// Header (`div.p-card-header`).
    pub header: Pt,
    /// Body (`div.p-card-body`).
    pub body: Pt,
    /// Title (`div.p-card-title`).
    pub title: Pt,
    /// Subtitle (`div.p-card-subtitle`).
    pub subtitle: Pt,
    /// Content (`div.p-card-content`).
    pub content: Pt,
    /// Footer (`div.p-card-footer`).
    pub footer: Pt,
}

/// Card: header, title, subtitle, content, footer.
///
/// Text properties accept a value or a signal. A slot takes priority over the corresponding text.
#[component]
pub fn Card(
    /// Title text (`p-card-title`), if there is no `title_template`.
    #[prop(optional, into)]
    header: OptProp<String>,
    /// Subtitle text (`p-card-subtitle`), if there is no `subtitle_template`.
    #[prop(optional, into)]
    subheader: OptProp<String>,
    /// Header slot (`p-card-header`), e.g. an image.
    #[prop(optional, into)]
    header_template: Option<ViewFn>,
    /// Title slot; replaces the `header` text.
    #[prop(optional, into)]
    title_template: Option<ViewFn>,
    /// Subtitle slot; replaces the `subheader` text.
    #[prop(optional, into)]
    subtitle_template: Option<ViewFn>,
    /// Footer slot (`p-card-footer`).
    #[prop(optional, into)]
    footer_template: Option<ViewFn>,
    /// Inline style.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Extra class.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Pass-through.
    #[prop(optional)]
    pt: Option<CardPt>,
    /// Content (`p-card-content`).
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let pt = pt.unwrap_or_default();
    let is_static = all_static!(header, subheader, style, style_class);
    let props = Props { header, subheader, style, style_class, header_template, title_template, subtitle_template, footer_template };
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
        let (props, pt) = (Arc::new(props), Arc::new(pt));
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

/// Card markup: one for the static and reactive branches (only the attribute and block expressions differ).
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

fn class_of(props: &Props, pt: &CardPt) -> Cow<'static, str> {
    let mut cn = ClassNames::new();
    cn.add("p-card p-component");
    props.style_class.with(|c| cn.add_user(c.as_deref()));
    cn.add_user(pt.root.extra_class());
    cn.finish_cow()
}

fn style_of(props: &Props, pt: &CardPt) -> Option<String> {
    props.style.with(|st| (st.is_some() || pt.root.has_style()).then(|| pt.root.merge_style(st.as_deref().unwrap_or(""))))
}

/// Header: only when the slot is set.
fn header_of(props: &Props, pt: &CardPt) -> Option<AnyView> {
    props
        .header_template
        .clone()
        .map(|slot| view! { <div class=pt.header.merge_class("p-card-header") {..pt.header.attrs()}>{slot.run()}</div> }.into_any())
}

/// Title: a slot replaces the text.
fn title_of(props: &Props, pt: &CardPt) -> Option<AnyView> {
    block(&pt.title, "p-card-title", props.title_template.clone(), props.header.get())
}

/// Subtitle: a slot replaces the text.
fn subtitle_of(props: &Props, pt: &CardPt) -> Option<AnyView> {
    block(&pt.subtitle, "p-card-subtitle", props.subtitle_template.clone(), props.subheader.get())
}

/// Footer: only when the slot is set.
fn footer_of(props: &Props, pt: &CardPt) -> Option<AnyView> {
    props
        .footer_template
        .clone()
        .map(|slot| view! { <div class=pt.footer.merge_class("p-card-footer") {..pt.footer.attrs()}>{slot.run()}</div> }.into_any())
}

/// A block with text or a slot (the slot takes priority); `None` if there is neither.
fn block(pt: &Pt, class: &'static str, slot: Option<ViewFn>, text: Option<String>) -> Option<AnyView> {
    let inner = match (slot, text.filter(|t| !t.is_empty())) {
        (Some(slot), _) => slot.run(),
        (None, Some(text)) => text.into_any(),
        (None, None) => return None,
    };
    Some(view! { <div class=pt.merge_class(class) {..pt.attrs()}>{inner}</div> }.into_any())
}

/// Instance properties.
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
