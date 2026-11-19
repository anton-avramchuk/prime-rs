//! Shared icon rendering.

use leptos::prelude::*;
use prime_rs_core::ClassNames;
use std::borrow::Cow;

/// `viewBox` of all PrimeNG icons.
pub const VIEW_BOX: &str = "0 0 14 14";

/// Root classes. A static extra class (`Cow::Borrowed`, e.g. the part class a component passes) keeps the result
/// interned, so the icon renders without allocating; an owned one is treated as user input.
fn attrs(
    class: Option<Cow<'static, str>>,
    spin: bool,
    aria_label: &Option<String>,
) -> (Cow<'static, str>, Option<&'static str>, Option<&'static str>) {
    let mut cn = ClassNames::new();
    cn.add("p-icon").add_if(spin, "p-icon-spin");
    match &class {
        Some(Cow::Borrowed(c)) => {
            cn.add(c);
        }
        Some(Cow::Owned(c)) => {
            cn.add_user(Some(c));
        }
        None => {}
    }
    let decorative = aria_label.is_none();
    (cn.finish_cow(), decorative.then_some("true"), (!decorative).then_some("img"))
}

/// Renders an icon; `inner` is ready-made path markup (`<path …></path>`), inserted as is without allocations.
/// Used by the generated components.
pub fn icon_view(
    inner: &'static str,
    class: Option<Cow<'static, str>>,
    spin: bool,
    size: u32,
    aria_label: Option<String>,
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
            inner_html=inner
        ></svg>
    }
}

/// Transparent placeholder icon (`BlankIcon` in PrimeNG).
#[cfg(feature = "blank")]
#[component]
pub fn BlankIcon(
    #[prop(optional, into)] class: Option<Cow<'static, str>>,
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

/// Declares an icon component from a set of paths.
#[allow(unused_macros)]
macro_rules! icon {
    ($name:ident, [$($d:expr),+ $(,)?]) => {
        /// PrimeNG icon (see the crate documentation).
        #[::leptos::component]
        pub fn $name(
            #[prop(optional, into)] class: Option<::std::borrow::Cow<'static, str>>,
            #[prop(optional)] spin: bool,
            #[prop(default = 14)] size: u32,
            #[prop(optional, into)] aria_label: Option<String>,
        ) -> impl ::leptos::prelude::IntoView {
            const INNER: &str = concat!($("<path d=\"", $d, "\" fill=\"currentColor\"></path>"),+);
            $crate::icon::icon_view(INNER, class, spin, size, aria_label)
        }
    };
}
