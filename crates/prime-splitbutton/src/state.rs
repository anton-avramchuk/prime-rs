//! Чистая логика SplitButton: классы корня и клавиши открытия меню (по `splitbuttonstyle.ts` и `onDropdownButtonKeydown`).

use prime_core::{ClassNames, Size};
use std::borrow::Cow;

/// Классы корня: `p-splitbutton p-component` и модификаторы `raised`, `rounded`, `outlined`, `text`, размер (`small → sm`, иначе `lg`).
pub fn root_classes(raised: bool, rounded: bool, outlined: bool, text: bool, size: Option<Size>) -> Cow<'static, str> {
    let mut cn = ClassNames::new();
    cn.add("p-splitbutton p-component")
        .add_if(raised, "p-splitbutton-raised")
        .add_if(rounded, "p-splitbutton-rounded")
        .add_if(outlined, "p-splitbutton-outlined")
        .add_if(text, "p-splitbutton-text");
    if let Some(size) = size {
        cn.add(if size == Size::Small { "p-splitbutton-sm" } else { "p-splitbutton-lg" });
    }
    cn.finish_cow()
}

/// Клавиши кнопки раскрытия, открывающие меню: ↓ и ↑.
pub fn is_toggle_key(code: &str) -> bool {
    matches!(code, "ArrowDown" | "ArrowUp")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classes_table() {
        assert_eq!(root_classes(false, false, false, false, None), "p-splitbutton p-component");
        assert_eq!(root_classes(true, false, false, false, None), "p-splitbutton p-component p-splitbutton-raised");
        assert_eq!(
            root_classes(true, true, true, true, Some(Size::Small)),
            "p-splitbutton p-component p-splitbutton-raised p-splitbutton-rounded p-splitbutton-outlined p-splitbutton-text p-splitbutton-sm"
        );
        assert_eq!(root_classes(false, false, false, false, Some(Size::Large)), "p-splitbutton p-component p-splitbutton-lg");
    }

    #[test]
    fn toggle_keys() {
        assert!(is_toggle_key("ArrowDown") && is_toggle_key("ArrowUp"));
        assert!(!is_toggle_key("Enter") && !is_toggle_key("Space") && !is_toggle_key("ArrowLeft") && !is_toggle_key(""));
    }
}
