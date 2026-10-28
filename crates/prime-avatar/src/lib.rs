//! Avatar — порт `p-avatar` из PrimeNG 21.1.10.
//!
//! Разметка, классы и `data-p` совпадают с PrimeNG; вместо хост-элемента `<p-avatar>` отрисовывается `<div class="p-avatar …">`.
//! Содержимое: `children`, затем метка, иначе иконка, иначе изображение.
//!
//! Свойства — [`Prop`]/[`OptProp`]; если все они статические, компонент рисует готовые значения без замыканий и `RenderEffect`.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_avatar::{Avatar, AvatarShape, AvatarSize};
//!
//! let html = view! { <Avatar label="P" shape=AvatarShape::Circle size=AvatarSize::XLarge /> }.to_html();
//! assert!(html.contains("p-avatar-circle") && html.contains("p-avatar-xl") && html.contains(r#"data-p="circle xlarge""#));
//! ```

mod state;

use leptos::either::Either;
use leptos::prelude::*;
use prime_core::{ClassNames, OptProp, Prop, Pt, all_static};
use std::borrow::Cow;
use std::sync::Arc;

pub use state::{AvatarShape, AvatarSize, AvatarState, data_p, write_classes};

/// Pass-through частей Avatar.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AvatarPt {
    /// Корневой элемент.
    pub root: Pt,
    /// Метка (`span.p-avatar-label`).
    pub label: Pt,
    /// Иконка (`span.p-avatar-icon`).
    pub icon: Pt,
    /// Изображение (`img`).
    pub image: Pt,
}

/// Аватар: метка, иконка или изображение.
///
/// Каждое свойство принимает значение или сигнал. Приоритет блоков: метка, иконка, изображение.
#[component]
pub fn Avatar(
    /// Текст метки.
    #[prop(optional, into)]
    label: OptProp<String>,
    /// Иконка; выводится в `span.p-avatar-icon`, если нет метки.
    #[prop(optional, into)]
    icon: Option<ViewFn>,
    /// URL изображения; выводится, если нет метки и иконки.
    #[prop(optional, into)]
    image: OptProp<String>,
    /// Размер (по умолчанию `normal`).
    #[prop(optional, into)]
    size: Prop<AvatarSize>,
    /// Форма (по умолчанию `square`).
    #[prop(optional, into)]
    shape: Prop<AvatarShape>,
    /// Встроенный стиль.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Дополнительный класс.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Атрибут `aria-label` корня и изображения.
    #[prop(optional, into)]
    aria_label: OptProp<String>,
    /// Атрибут `aria-labelledby` корня.
    #[prop(optional, into)]
    aria_labelledby: OptProp<String>,
    /// Вызывается при ошибке загрузки изображения.
    #[prop(optional, into)]
    on_image_error: Option<Callback<web_sys::Event>>,
    /// Pass-through.
    #[prop(optional)]
    pt: Option<AvatarPt>,
    /// Содержимое перед меткой, иконкой и изображением.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let pt = pt.unwrap_or_default();
    let is_static = all_static!(label, image, size, shape, style, style_class, aria_label, aria_labelledby);
    let props = Props { label, image, size, shape, style, style_class, aria_label, aria_labelledby, icon };
    let content = children.map(|c| c());
    let attrs = pt.root.attrs();

    if is_static {
        let (class, style, data) = (class_of(&props, &pt), style_of(&props, &pt), data_of(&props));
        let (aria, labelledby) = (props.aria_label.get(), props.aria_labelledby.get());
        let block = block_of(&props, &pt, on_image_error);
        Either::Left(avatar_markup!(class, style, data, aria, labelledby, attrs, content, block))
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
        let data = {
            let props = props.clone();
            move || data_of(&props)
        };
        let aria = {
            let props = props.clone();
            move || props.aria_label.get()
        };
        let labelledby = {
            let props = props.clone();
            move || props.aria_labelledby.get()
        };
        let block = move || block_of(&props, &pt, on_image_error);
        Either::Right(avatar_markup!(class, style, data, aria, labelledby, attrs, content, block))
    }
}

/// Разметка Avatar: одна для статической и реактивной ветвей (меняются только выражения атрибутов и блока).
macro_rules! avatar_markup {
    ($class:expr, $style:expr, $data:expr, $aria:expr, $labelledby:expr, $attrs:expr, $content:expr, $block:expr) => {
        view! {
            <div class={$class} style={$style} aria-label={$aria} aria-labelledby={$labelledby} data-p={$data} {..$attrs}>
                {$content}
                {$block}
            </div>
        }
    };
}
use avatar_markup;

fn state_of(props: &Props) -> AvatarState {
    AvatarState { has_image: props.image.with(Option::is_some), shape: props.shape.get(), size: props.size.get() }
}

fn class_of(props: &Props, pt: &AvatarPt) -> Cow<'static, str> {
    let mut cn = ClassNames::new();
    write_classes(&mut cn, &state_of(props));
    props.style_class.with(|c| cn.add_user(c.as_deref()));
    cn.add_user(pt.root.extra_class());
    cn.finish_cow()
}

fn style_of(props: &Props, pt: &AvatarPt) -> Option<String> {
    props.style.with(|st| (st.is_some() || pt.root.has_style()).then(|| pt.root.merge_style(st.as_deref().unwrap_or(""))))
}

fn data_of(props: &Props) -> Cow<'static, str> {
    data_p(&state_of(props))
}

/// Один из блоков: метка, иначе иконка, иначе изображение.
fn block_of(props: &Props, pt: &AvatarPt, on_image_error: Option<Callback<web_sys::Event>>) -> Option<AnyView> {
    let data = data_of(props);
    if let Some(label) = props.label.get().filter(|s| !s.is_empty()) {
        return Some(
            view! { <span class=pt.label.merge_class("p-avatar-label") data-p={data} {..pt.label.attrs()}>{label}</span> }.into_any(),
        );
    }
    if let Some(icon) = props.icon.clone() {
        return Some(
            view! { <span class=pt.icon.merge_class("p-avatar-icon") data-p={data} {..pt.icon.attrs()}>{icon.run()}</span> }.into_any(),
        );
    }
    let src = props.image.get()?;
    let aria = props.aria_label.get();
    let class = pt.image.extra_class().map(str::to_owned);
    Some(
        view! {
            <img
                src=src
                aria-label=aria
                data-p={data}
                class=class
                on:error=move |e| {
                    if let Some(cb) = on_image_error {
                        cb.run(e.into());
                    }
                }
                {..pt.image.attrs()}
            />
        }
        .into_any(),
    )
}

/// Свойства экземпляра.
struct Props {
    label: OptProp<String>,
    image: OptProp<String>,
    size: Prop<AvatarSize>,
    shape: Prop<AvatarShape>,
    style: OptProp<String>,
    style_class: OptProp<String>,
    aria_label: OptProp<String>,
    aria_labelledby: OptProp<String>,
    icon: Option<ViewFn>,
}
