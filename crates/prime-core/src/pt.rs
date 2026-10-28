//! Pass-through (`pt`): дополнительные атрибуты и классы для частей компонента.

use crate::class_names::ClassNames;
use leptos::attr::any_attribute::{AnyAttribute, IntoAnyAttribute};
use leptos::attr::custom::custom_attribute;
use std::borrow::Cow;

/// Дополнения для одного элемента компонента (корня или его части).
///
/// Собственные классы компонента не затираются: `class` из `Pt` добавляется после них.
///
/// ```
/// use prime_core::Pt;
///
/// let pt = Pt::new().class("custom").attr("data-testid", "root");
/// assert_eq!(pt.merge_class("p-button"), "p-button custom");
/// ```
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Pt {
    class: Option<String>,
    style: Option<String>,
    attrs: Vec<(String, String)>,
}

impl Pt {
    /// Пустой pass-through.
    pub fn new() -> Self {
        Self::default()
    }

    /// Добавляет класс(ы).
    pub fn class(mut self, class: impl Into<String>) -> Self {
        self.class = Some(class.into());
        self
    }

    /// Добавляет встроенный стиль (`margin: 4px`).
    pub fn style(mut self, style: impl Into<String>) -> Self {
        self.style = Some(style.into());
        self
    }

    /// Добавляет атрибут (например, `data-testid`, `style`, `aria-describedby`).
    pub fn attr(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.attrs.push((name.into(), value.into()));
        self
    }

    /// Дополнительный класс из `Pt` (без копирования).
    pub fn extra_class(&self) -> Option<&str> {
        self.class.as_deref()
    }

    /// Задан ли встроенный стиль.
    pub fn has_style(&self) -> bool {
        self.style.is_some()
    }

    /// Собственные классы компонента плюс класс из `Pt`.
    ///
    /// Без класса в `Pt` строка интернируется и не аллоцирует (см. [`ClassNames::finish_cow`]).
    pub fn merge_class(&self, base: &str) -> Cow<'static, str> {
        let mut cn = ClassNames::new();
        cn.add(base).add_user(self.class.as_deref());
        cn.finish_cow()
    }

    /// Собственный стиль компонента плюс стиль из `Pt`; пустые части пропускаются, `;` на концах нормализуются.
    ///
    /// ```
    /// use prime_core::Pt;
    ///
    /// let pt = Pt::new().style("color: red;");
    /// assert_eq!(pt.merge_style("margin: 4px"), "margin: 4px; color: red");
    /// assert_eq!(Pt::new().merge_style(""), "");
    /// ```
    pub fn merge_style(&self, base: &str) -> String {
        let part = |s: &str| s.trim().trim_end_matches(';').trim().to_owned();
        let (base, extra) = (part(base), self.style.as_deref().map(part).unwrap_or_default());
        match (base.is_empty(), extra.is_empty()) {
            (_, true) => base,
            (true, false) => extra,
            (false, false) => {
                let mut out = base;
                out.push_str("; ");
                out.push_str(&extra);
                out
            }
        }
    }

    /// Атрибуты для `view! { <div {..pt.attrs()} /> }`.
    pub fn attrs(&self) -> Vec<AnyAttribute> {
        self.attrs.iter().map(|(k, v)| custom_attribute(k.clone(), v.clone()).into_any_attr()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merges_class_after_base() {
        let pt = Pt::new().class("custom");
        assert_eq!(pt.merge_class("p-button p-component"), "p-button p-component custom");
        assert_eq!(Pt::new().merge_class("p-button"), "p-button");
    }

    #[test]
    fn merges_style_normalizing_semicolons() {
        let pt = Pt::new().style("color: red;");
        assert_eq!(pt.merge_style("margin: 4px;"), "margin: 4px; color: red");
        assert_eq!(pt.merge_style(""), "color: red");
        assert_eq!(Pt::new().merge_style("margin: 4px"), "margin: 4px");
        assert_eq!(Pt::new().merge_style(""), "");
    }

    #[test]
    fn collects_attrs() {
        let pt = Pt::new().attr("data-testid", "x").attr("aria-label", "y");
        assert_eq!(pt.attrs().len(), 2);
    }
}
