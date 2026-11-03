//! Чистая логика Steps: классы, отключённость, `tabindex`, `aria-disabled`, исход клика и клавиши (по `steps.ts` PrimeNG).

use prime_core::ClassNames;
use std::borrow::Cow;

/// Классы корня: `p-steps p-component`, `p-readonly`, пользовательский класс.
pub fn root_classes(readonly: bool, extra: Option<&str>) -> Cow<'static, str> {
    let mut cn = ClassNames::new();
    cn.add("p-steps p-component").add_if(readonly, "p-readonly").add_user(extra);
    cn.finish_cow()
}

/// Классы пункта: `p-steps-item`, `p-steps-item-active`, `p-disabled`.
pub fn item_classes(active: bool, disabled: bool) -> Cow<'static, str> {
    let mut cn = ClassNames::new();
    cn.add("p-steps-item").add_if(active, "p-steps-item-active").add_if(disabled, "p-disabled");
    cn.finish_cow()
}

/// Пункт отключён: у него `disabled`, либо включено чтение, а пункт не активен (`isItemDisabled`).
pub fn is_item_disabled(item_disabled: bool, index: usize, active: usize, readonly: bool) -> bool {
    item_disabled || (readonly && index != active)
}

/// `tabindex` ссылки без перемещения фокуса: у отключённого `-1`, у активного собственный (или `0`), у остальных собственный
/// (или `-1`) (`getItemTabIndex`).
pub fn link_tabindex(item_disabled: bool, item_tabindex: Option<i32>, index: usize, active: usize) -> i32 {
    if item_disabled {
        -1
    } else if index == active {
        item_tabindex.filter(|t| *t != 0).unwrap_or(0)
    } else {
        item_tabindex.unwrap_or(-1)
    }
}

/// `aria-disabled` ссылки: `disabled` пункта или чтение у неактивного пункта.
pub fn aria_disabled(item_disabled: bool, index: usize, active: usize, readonly: bool) -> bool {
    item_disabled || (readonly && index != active)
}

/// Что делает клик.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Click {
    /// Только отменить переход по ссылке.
    Ignore,
    /// Выбрать пункт и вызвать команду; `prevent_default` — отменить ли переход (у пункта нет `url`).
    Activate {
        /// Отменить ли переход по ссылке.
        prevent_default: bool,
    },
}

/// Исход клика (`onItemClick`): чтение или отключённый пункт — игнорируются.
pub fn click_outcome(item_disabled: bool, readonly: bool, has_url: bool) -> Click {
    if readonly || item_disabled { Click::Ignore } else { Click::Activate { prevent_default: !has_url } }
}

/// Пункт, на который переносится фокус по клавише; на границах остаётся тот же. `None` — клавиша не двигает фокус.
pub fn key_target(code: &str, index: usize, count: usize) -> Option<usize> {
    let last = count.checked_sub(1)?;
    match code {
        "ArrowRight" => Some((index + 1).min(last)),
        "ArrowLeft" => Some(index.saturating_sub(1)),
        "Home" => Some(0),
        "End" => Some(last),
        _ => None,
    }
}

/// `Enter` и `Space` действуют как клик.
pub fn is_activation_key(code: &str) -> bool {
    matches!(code, "Enter" | "Space")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn root_and_item_classes() {
        assert_eq!(root_classes(false, None), "p-steps p-component");
        assert_eq!(root_classes(true, Some("mine")), "p-steps p-component p-readonly mine");
        assert_eq!(item_classes(false, false), "p-steps-item");
        assert_eq!(item_classes(true, false), "p-steps-item p-steps-item-active");
        assert_eq!(item_classes(false, true), "p-steps-item p-disabled");
        assert_eq!(item_classes(true, true), "p-steps-item p-steps-item-active p-disabled");
    }

    #[test]
    fn disabled_table() {
        // (disabled, индекс, активный, чтение) → отключён
        for (dis, i, a, ro, want) in [
            (false, 1, 1, true, false),
            (false, 0, 1, true, true),
            (false, 0, 1, false, false),
            (true, 1, 1, false, true),
            (true, 1, 1, true, true),
        ] {
            assert_eq!(is_item_disabled(dis, i, a, ro), want, "{dis} {i} {a} {ro}");
            assert_eq!(aria_disabled(dis, i, a, ro), want, "{dis} {i} {a} {ro}");
        }
    }

    #[test]
    fn tabindex_table() {
        assert_eq!(link_tabindex(true, Some(5), 1, 1), -1, "отключённый");
        assert_eq!(link_tabindex(false, None, 1, 1), 0, "активный");
        assert_eq!(link_tabindex(false, Some(3), 1, 1), 3, "активный со своим");
        assert_eq!(link_tabindex(false, Some(0), 1, 1), 0);
        assert_eq!(link_tabindex(false, None, 0, 1), -1, "остальные");
        assert_eq!(link_tabindex(false, Some(2), 0, 1), 2);
    }

    #[test]
    fn click_table() {
        assert_eq!(click_outcome(false, true, false), Click::Ignore);
        assert_eq!(click_outcome(true, false, false), Click::Ignore);
        assert_eq!(click_outcome(false, false, false), Click::Activate { prevent_default: true });
        assert_eq!(click_outcome(false, false, true), Click::Activate { prevent_default: false });
    }

    #[test]
    fn key_targets_stay_inside() {
        assert_eq!(key_target("ArrowRight", 0, 4), Some(1));
        assert_eq!(key_target("ArrowRight", 3, 4), Some(3));
        assert_eq!(key_target("ArrowLeft", 2, 4), Some(1));
        assert_eq!(key_target("ArrowLeft", 0, 4), Some(0));
        assert_eq!(key_target("Home", 2, 4), Some(0));
        assert_eq!(key_target("End", 1, 4), Some(3));
        assert_eq!(key_target("Tab", 1, 4), None);
        assert_eq!(key_target("End", 0, 0), None);
        assert!(is_activation_key("Enter") && is_activation_key("Space") && !is_activation_key("KeyA"));
    }
}
