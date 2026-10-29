//! Общие помощники контейнеров-обёрток (`ButtonGroup`, `IconField`, `FloatLabel` и других): классы и стиль корня.

use crate::{ClassNames, OptProp, Pt};
use std::borrow::Cow;

/// Классы корня: собственные, затем `style_class`, затем класс из `pt.root`.
///
/// ```
/// use leptos::prelude::*;
/// use prime_core::{OptProp, Pt, wrapper::root_class};
///
/// Owner::new().with(|| {
///     let extra: OptProp<String> = "mine".into();
///     assert_eq!(root_class("p-buttongroup p-component", &extra, &Pt::new().class("custom")), "p-buttongroup p-component mine custom");
/// });
/// ```
pub fn root_class(base: &str, style_class: &OptProp<String>, root: &Pt) -> Cow<'static, str> {
    let mut cn = ClassNames::new();
    cn.add(base);
    style_class.with(|c| cn.add_user(c.as_deref()));
    cn.add_user(root.extra_class());
    cn.finish_cow()
}

/// Встроенный стиль корня: `style`, затем стиль из `pt.root`; `None`, если ни того, ни другого нет.
pub fn root_style(style: &OptProp<String>, root: &Pt) -> Option<String> {
    style.with(|st| (st.is_some() || root.has_style()).then(|| root.merge_style(st.as_deref().unwrap_or(""))))
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::*;

    #[test]
    fn class_and_style_merge_in_order() {
        Owner::new().with(|| {
            let none: OptProp<String> = OptProp::default();
            assert_eq!(root_class("p-x", &none, &Pt::new()), "p-x");
            let extra: OptProp<String> = "a b".into();
            assert_eq!(root_class("p-x", &extra, &Pt::new().class("c")), "p-x a b c");
            assert_eq!(root_style(&none, &Pt::new()), None);
            let style: OptProp<String> = "width: 1px".into();
            assert_eq!(root_style(&style, &Pt::new().style("color: red")).as_deref(), Some("width: 1px; color: red"));
            assert_eq!(root_style(&none, &Pt::new().style("color: red")).as_deref(), Some("color: red"));
        });
    }
}
