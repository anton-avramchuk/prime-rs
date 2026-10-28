//! Pass-through (`pt`): дополнительные атрибуты и классы для частей компонента.

use crate::class_names::ClassNames;
use leptos::attr::any_attribute::{AnyAttribute, IntoAnyAttribute};
use leptos::attr::custom::custom_attribute;

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

    /// Добавляет атрибут (например, `data-testid`, `style`, `aria-describedby`).
    pub fn attr(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.attrs.push((name.into(), value.into()));
        self
    }

    /// Собственные классы компонента плюс класс из `Pt`.
    pub fn merge_class(&self, base: &str) -> String {
        let mut cn = ClassNames::new();
        cn.add(base).add_opt(self.class.as_deref());
        cn.finish()
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
    fn collects_attrs() {
        let pt = Pt::new().attr("data-testid", "x").attr("aria-label", "y");
        assert_eq!(pt.attrs().len(), 2);
    }
}
