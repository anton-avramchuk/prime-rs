//! AvatarGroup — port of `p-avatar-group` from PrimeNG 21.1.10.
//!
//! A container for several Avatars: root `div.p-avatar-group.p-component` (instead of the host `<p-avatar-group>`) with child content
//! without wrappers. The placement and overlap of the avatars are set by the theme CSS.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_avatargroup::AvatarGroup;
//!
//! let html = view! { <AvatarGroup style_class="mine"><span>"a"</span><span>"b"</span></AvatarGroup> }.to_html();
//! assert!(html.contains("p-avatar-group p-component mine") && html.contains("a</span>"));
//! ```

use leptos::either::Either;
use leptos::prelude::*;
use prime_core::{ClassNames, OptProp, Pt, all_static};
use std::borrow::Cow;
use std::sync::Arc;

/// Group of avatars.
///
/// Each property accepts a value or a signal.
#[component]
pub fn AvatarGroup(
    /// Inline style of the root.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Extra class of the root.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Pass-through of the root.
    #[prop(optional)]
    pt: Option<Pt>,
    /// Avatars of the group.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let pt = pt.unwrap_or_default();
    let attrs = pt.attrs();
    let kids = children.map(|c| c());

    if all_static!(style, style_class) {
        let class = class_of(&style_class, &pt);
        let style = style_of(&style, &pt);
        Either::Left(group_markup!(class, style, attrs, kids))
    } else {
        let pt = Arc::new(pt);
        let class = {
            let (style_class, pt) = (style_class, pt.clone());
            move || class_of(&style_class, &pt)
        };
        let style = {
            let pt = pt.clone();
            move || style_of(&style, &pt)
        };
        Either::Right(group_markup!(class, style, attrs, kids))
    }
}

/// AvatarGroup markup: one for the static and reactive branches (only the attribute expressions differ).
macro_rules! group_markup {
    ($class:expr, $style:expr, $attrs:expr, $kids:expr) => {
        view! { <div class={$class} style={$style} {..$attrs}>{$kids}</div> }
    };
}
use group_markup;

fn class_of(style_class: &OptProp<String>, pt: &Pt) -> Cow<'static, str> {
    let mut cn = ClassNames::new();
    cn.add("p-avatar-group p-component");
    style_class.with(|c| cn.add_user(c.as_deref()));
    cn.add_user(pt.extra_class());
    cn.finish_cow()
}

fn style_of(style: &OptProp<String>, pt: &Pt) -> Option<String> {
    style.with(|st| (st.is_some() || pt.has_style()).then(|| pt.merge_style(st.as_deref().unwrap_or(""))))
}
