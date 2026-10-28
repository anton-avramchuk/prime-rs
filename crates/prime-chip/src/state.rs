//! Чистая логика Chip: классы, `data-p` и клавиша удаления (по `chipstyle.ts` и `Chip` PrimeNG).

use prime_core::{ClassNames, DataP};
use std::borrow::Cow;

/// Снимок свойств, влияющих на классы и `data-p`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ChipState {
    /// Недоступен.
    pub disabled: bool,
    /// Удаляемый.
    pub removable: bool,
}

/// Добавляет классы корня в порядке `chipstyle.ts`.
pub fn write_classes(cn: &mut ClassNames, s: &ChipState) {
    cn.add("p-chip p-component").add_if(s.disabled, "p-disabled");
}

/// Значение `data-p`: `removable`.
pub fn data_p(s: &ChipState) -> Cow<'static, str> {
    let mut d = DataP::new();
    d.add_if(s.removable, "removable");
    d.finish_cow()
}

/// Клавиши, закрывающие Chip на элементе удаления: Enter и Backspace.
pub fn is_remove_key(key: &str) -> bool {
    matches!(key, "Enter" | "Backspace")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn classes(s: &ChipState) -> String {
        let mut cn = ClassNames::new();
        write_classes(&mut cn, s);
        cn.finish()
    }

    #[test]
    fn classes_and_data_p_table() {
        let cases = [
            (ChipState::default(), "p-chip p-component", ""),
            (ChipState { disabled: true, removable: false }, "p-chip p-component p-disabled", ""),
            (ChipState { disabled: false, removable: true }, "p-chip p-component", "removable"),
            (ChipState { disabled: true, removable: true }, "p-chip p-component p-disabled", "removable"),
        ];
        for (s, class, data) in cases {
            assert_eq!(classes(&s), class, "{s:?}");
            assert_eq!(data_p(&s), data, "{s:?}");
        }
    }

    #[test]
    fn only_enter_and_backspace_remove() {
        assert!(is_remove_key("Enter") && is_remove_key("Backspace"));
        for k in ["Delete", " ", "Escape", "a", "enter", ""] {
            assert!(!is_remove_key(k), "{k}");
        }
    }
}
