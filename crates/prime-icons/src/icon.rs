//! Общая отрисовка иконок.

use leptos::prelude::*;
use prime_core::ClassNames;
use std::borrow::Cow;

/// `viewBox` всех иконок PrimeNG.
pub const VIEW_BOX: &str = "0 0 14 14";

fn attrs(
    class: Option<String>,
    spin: bool,
    aria_label: &Option<String>,
) -> (Cow<'static, str>, Option<&'static str>, Option<&'static str>) {
    let mut cn = ClassNames::new();
    cn.add("p-icon").add_if(spin, "p-icon-spin").add_user(class.as_deref());
    let decorative = aria_label.is_none();
    (cn.finish_cow(), decorative.then_some("true"), (!decorative).then_some("img"))
}

/// Рисует иконку; `inner` — готовая разметка контуров (`<path …></path>`), вставляется как есть без аллокаций.
/// Используется сгенерированными компонентами.
pub fn icon_view(inner: &'static str, class: Option<String>, spin: bool, size: u32, aria_label: Option<String>) -> impl IntoView {
    let (class, aria_hidden, role) = attrs(class, spin, &aria_label);
    view! {
        <svg
            width=size
            height=size
            viewBox=VIEW_BOX
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
            class=class
            aria-hidden=aria_hidden
            role=role
            aria-label=aria_label
            inner_html=inner
        ></svg>
    }
}

/// Прозрачная иконка-заполнитель (`BlankIcon` в PrimeNG).
#[cfg(feature = "blank")]
#[component]
pub fn BlankIcon(
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional)] spin: bool,
    #[prop(default = 14)] size: u32,
    #[prop(optional, into)] aria_label: Option<String>,
) -> impl IntoView {
    let (class, aria_hidden, role) = attrs(class, spin, &aria_label);
    view! {
        <svg
            width=size
            height=size
            viewBox=VIEW_BOX
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
            class=class
            aria-hidden=aria_hidden
            role=role
            aria-label=aria_label
        >
            <rect width="1" height="1" fill="currentColor" fill-opacity="0" />
        </svg>
    }
}

/// Объявляет компонент иконки по набору контуров.
#[allow(unused_macros)]
macro_rules! icon {
    ($name:ident, [$($d:expr),+ $(,)?]) => {
        /// Иконка PrimeNG (см. документацию крейта).
        #[::leptos::component]
        pub fn $name(
            #[prop(optional, into)] class: Option<String>,
            #[prop(optional)] spin: bool,
            #[prop(default = 14)] size: u32,
            #[prop(optional, into)] aria_label: Option<String>,
        ) -> impl ::leptos::prelude::IntoView {
            const INNER: &str = concat!($("<path d=\"", $d, "\" fill=\"currentColor\"></path>"),+);
            $crate::icon::icon_view(INNER, class, spin, size, aria_label)
        }
    };
}
