//! Чистая логика SelectButton: варианты, выбор и классы корня (по `selectbutton.ts` и `selectbuttonstyle.ts` PrimeNG).

use prime_core::{ClassNames, DataP};
use std::borrow::Cow;

/// Вариант выбора.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SelectButtonOption {
    /// Метка кнопки.
    pub label: String,
    /// Значение варианта.
    pub value: String,
    /// Вариант недоступен.
    pub disabled: bool,
}

impl SelectButtonOption {
    /// Доступный вариант с меткой и значением.
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self { label: label.into(), value: value.into(), disabled: false }
    }

    /// Недоступный вариант.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// Значение SelectButton: одиночное или множественное.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SelectButtonValue {
    /// Одиночный выбор (`None` — ничего не выбрано).
    Single(Option<String>),
    /// Множественный выбор в порядке добавления.
    Multiple(Vec<String>),
}

/// Результат выбора в одиночном режиме: `None` — значение не меняется, `Some(v)` — новое значение.
///
/// Как `onOptionSelect` PrimeNG: повторный клик по выбранному варианту сбрасывает значение, если `allow_empty` не равен `false`.
pub fn select_single(current: Option<&str>, target: &str, allow_empty: Option<bool>) -> Option<Option<String>> {
    if current == Some(target) { if allow_empty == Some(false) { None } else { Some(None) } } else { Some(Some(target.to_owned())) }
}

/// Результат выбора в множественном режиме: `None` — список не меняется.
///
/// Выбранное значение удаляется, невыбранное добавляется в конец; при `allow_empty = false` единственное значение не удаляется.
pub fn select_multiple(current: &[String], target: &str, allow_empty: Option<bool>) -> Option<Vec<String>> {
    if current.iter().any(|v| v == target) {
        if allow_empty == Some(false) && current.len() == 1 {
            return None;
        }
        Some(current.iter().filter(|v| *v != target).cloned().collect())
    } else {
        let mut next = current.to_vec();
        next.push(target.to_owned());
        Some(next)
    }
}

/// `allow_empty` для вложенной кнопки (`getAllowEmpty`): во множественном режиме последнее значение снять нельзя при `false`.
pub fn toggle_allow_empty(multiple: bool, allow_empty: Option<bool>, selected_count: usize) -> Option<bool> {
    if multiple { Some(allow_empty.unwrap_or(true) || selected_count != 1) } else { allow_empty }
}

/// Снимок свойств, влияющих на классы и `data-p` корня.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SelectButtonState {
    /// Недопустимое значение.
    pub invalid: bool,
    /// На всю ширину.
    pub fluid: bool,
}

/// Добавляет классы корня в порядке `selectbuttonstyle.ts`.
pub fn write_classes(cn: &mut ClassNames, s: &SelectButtonState) {
    cn.add("p-selectbutton p-component").add_if(s.invalid, "p-invalid").add_if(s.fluid, "p-selectbutton-fluid");
}

/// Значение `data-p`: `invalid`.
pub fn data_p(s: &SelectButtonState) -> Cow<'static, str> {
    let mut d = DataP::new();
    d.add_if(s.invalid, "invalid");
    d.finish_cow()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strings(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| (*s).to_owned()).collect()
    }

    #[test]
    fn single_selection_table() {
        let cases = [
            (None, "a", None, Some(Some("a"))),
            (Some("b"), "a", None, Some(Some("a"))),
            (Some("b"), "a", Some(false), Some(Some("a"))),
            (Some("a"), "a", None, Some(None)),
            (Some("a"), "a", Some(true), Some(None)),
            (Some("a"), "a", Some(false), None),
        ];
        for (current, target, allow_empty, expected) in cases {
            let expected = expected.map(|v| v.map(str::to_owned));
            assert_eq!(select_single(current, target, allow_empty), expected, "{current:?} {target} {allow_empty:?}");
        }
    }

    #[test]
    fn multiple_selection_table() {
        let cases = [
            (vec![], "a", None, Some(vec!["a"])),
            (vec!["a"], "b", None, Some(vec!["a", "b"])),
            (vec!["a", "b"], "a", None, Some(vec!["b"])),
            (vec!["a"], "a", None, Some(vec![])),
            (vec!["a"], "a", Some(false), None),
            (vec!["a", "b"], "a", Some(false), Some(vec!["b"])),
        ];
        for (current, target, allow_empty, expected) in cases {
            let current = strings(&current);
            assert_eq!(
                select_multiple(&current, target, allow_empty),
                expected.map(|e| strings(&e)),
                "{current:?} {target} {allow_empty:?}"
            );
        }
    }

    #[test]
    fn toggle_allow_empty_follows_get_allow_empty() {
        assert_eq!(toggle_allow_empty(false, None, 1), None);
        assert_eq!(toggle_allow_empty(false, Some(false), 1), Some(false));
        assert_eq!(toggle_allow_empty(true, None, 1), Some(true));
        assert_eq!(toggle_allow_empty(true, Some(false), 1), Some(false));
        assert_eq!(toggle_allow_empty(true, Some(false), 0), Some(true));
        assert_eq!(toggle_allow_empty(true, Some(false), 2), Some(true));
    }

    #[test]
    fn classes_and_data_p_table() {
        let st = |invalid, fluid| SelectButtonState { invalid, fluid };
        let cases = [
            (st(false, false), "p-selectbutton p-component", ""),
            (st(true, false), "p-selectbutton p-component p-invalid", "invalid"),
            (st(false, true), "p-selectbutton p-component p-selectbutton-fluid", ""),
            (st(true, true), "p-selectbutton p-component p-invalid p-selectbutton-fluid", "invalid"),
        ];
        for (s, class, data) in cases {
            let mut cn = ClassNames::new();
            write_classes(&mut cn, &s);
            assert_eq!(cn.finish(), class, "{s:?}");
            assert_eq!(data_p(&s), data, "{s:?}");
        }
    }

    #[test]
    fn option_builder() {
        let o = SelectButtonOption::new("One", "1").disabled(true);
        assert_eq!((o.label.as_str(), o.value.as_str(), o.disabled), ("One", "1", true));
        assert!(!SelectButtonOption::new("a", "b").disabled);
    }
}
