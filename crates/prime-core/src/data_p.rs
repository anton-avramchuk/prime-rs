//! Атрибут `data-p`: признаки состояния через пробел (аналог `cn({...})` в PrimeNG).

use crate::class_names::ClassNames;
use std::borrow::Cow;

/// Собирает значение `data-p`: без пустых и повторяющихся признаков.
///
/// ```
/// use prime_core::DataP;
///
/// let mut d = DataP::new();
/// d.add_opt(Some("small")).add_if(true, "icon-only").add_if(false, "loading");
/// assert_eq!(d.finish(), "small icon-only");
/// ```
#[derive(Debug, Default, Clone)]
pub struct DataP(ClassNames);

impl DataP {
    /// Пустой набор признаков.
    pub fn new() -> Self {
        Self(ClassNames::with_capacity(48))
    }

    /// Добавляет признак(и) через пробел.
    pub fn add(&mut self, flags: &str) -> &mut Self {
        self.0.add(flags);
        self
    }

    /// Добавляет признак, если условие истинно.
    pub fn add_if(&mut self, condition: bool, flag: &str) -> &mut Self {
        self.0.add_if(condition, flag);
        self
    }

    /// Добавляет признак, если он задан.
    pub fn add_opt(&mut self, flag: Option<&str>) -> &mut Self {
        self.0.add_opt(flag);
        self
    }

    /// Готовое значение атрибута.
    pub fn finish(self) -> String {
        self.0.finish()
    }

    /// Готовое значение без аллокации для повторяющихся комбинаций признаков (интернирование).
    pub fn finish_cow(self) -> Cow<'static, str> {
        self.0.finish_cow()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_order_skips_empty_and_duplicates() {
        let mut d = DataP::new();
        d.add("circle danger").add("").add_opt(None).add_opt(Some("danger")).add_if(false, "x").add_if(true, "xlarge");
        assert_eq!(d.finish(), "circle danger xlarge");
    }

    #[test]
    fn empty_is_empty_string() {
        assert_eq!(DataP::new().finish(), "");
    }
}
