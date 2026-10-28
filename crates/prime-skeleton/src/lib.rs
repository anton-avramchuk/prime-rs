//! Skeleton — порт `p-skeleton` из PrimeNG 21.1.10.
//!
//! Заглушка загрузки: `div.p-skeleton.p-component[aria-hidden=true]` (вместо хоста `<p-skeleton>`), форма, размеры и скругление задаются
//! свойствами и попадают во встроенный стиль, как в PrimeNG.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_skeleton::{Skeleton, SkeletonShape};
//!
//! let html = view! { <Skeleton shape=SkeletonShape::Circle size="4rem" /> }.to_html();
//! assert!(html.contains("p-skeleton-circle") && html.contains("width: 4rem; height: 4rem;"));
//! ```

mod state;

use leptos::either::Either;
use leptos::prelude::*;
use prime_core::{ClassNames, OptProp, Prop, Pt, all_static};
use std::borrow::Cow;
use std::sync::Arc;

pub use state::{SkeletonAnimation, SkeletonShape, container_style, data_p, write_classes};

/// Заглушка загрузки.
///
/// Каждое свойство принимает значение или сигнал.
#[component]
pub fn Skeleton(
    /// Форма (по умолчанию прямоугольник).
    #[prop(optional, into)]
    shape: Prop<SkeletonShape>,
    /// Анимация (по умолчанию волна).
    #[prop(optional, into)]
    animation: Prop<SkeletonAnimation>,
    /// Скругление (CSS `border-radius`).
    #[prop(optional, into)]
    border_radius: OptProp<String>,
    /// Размер: ширина и высота разом; приоритетнее `width` и `height`.
    #[prop(optional, into)]
    size: OptProp<String>,
    /// Ширина (по умолчанию `100%`).
    #[prop(optional, into)]
    width: OptProp<String>,
    /// Высота (по умолчанию `1rem`).
    #[prop(optional, into)]
    height: OptProp<String>,
    /// Дополнительный класс корня.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Pass-through корня.
    #[prop(optional)]
    pt: Option<Pt>,
) -> impl IntoView {
    let pt = pt.unwrap_or_default();
    let attrs = pt.attrs();
    let is_static = all_static!(shape, animation, border_radius, size, width, height, style_class);
    let props = Props { shape, animation, border_radius, size, width, height, style_class };

    if is_static {
        let (class, style, data) = (class_of(&props, &pt), style_of(&props, &pt), data_p(props.shape.get()));
        Either::Left(skeleton_markup!(class, style, data, attrs))
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
        let data = move || data_p(props.shape.get());
        Either::Right(skeleton_markup!(class, style, data, attrs))
    }
}

/// Разметка Skeleton: одна для статической и реактивной ветвей (меняются только выражения атрибутов).
macro_rules! skeleton_markup {
    ($class:expr, $style:expr, $data:expr, $attrs:expr) => {
        view! { <div aria-hidden="true" class={$class} style={$style} data-p={$data} {..$attrs}></div> }
    };
}
use skeleton_markup;

fn class_of(props: &Props, pt: &Pt) -> Cow<'static, str> {
    let mut cn = ClassNames::new();
    write_classes(&mut cn, props.shape.get(), props.animation.get());
    props.style_class.with(|c| cn.add_user(c.as_deref()));
    cn.add_user(pt.extra_class());
    cn.finish_cow()
}

fn style_of(props: &Props, pt: &Pt) -> String {
    props.size.with(|size| {
        props.width.with(|width| {
            props.height.with(|height| {
                props.border_radius.with(|radius| {
                    let own = container_style(size.as_deref(), width.as_deref(), height.as_deref(), radius.as_deref());
                    if pt.has_style() { pt.merge_style(&own) } else { own }
                })
            })
        })
    })
}

/// Свойства экземпляра.
struct Props {
    shape: Prop<SkeletonShape>,
    animation: Prop<SkeletonAnimation>,
    border_radius: OptProp<String>,
    size: OptProp<String>,
    width: OptProp<String>,
    height: OptProp<String>,
    style_class: OptProp<String>,
}
