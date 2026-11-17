//! Pure Steps logic: classes, disabled state, `tabindex`, `aria-disabled`, click and key outcome (after PrimeNG `steps.ts`).

use prime_rs_core::ClassNames;
use std::borrow::Cow;

/// Root classes: `p-steps p-component`, `p-readonly`, the user class.
pub fn root_classes(readonly: bool, extra: Option<&str>) -> Cow<'static, str> {
    let mut cn = ClassNames::new();
    cn.add("p-steps p-component").add_if(readonly, "p-readonly").add_user(extra);
    cn.finish_cow()
}

/// Item classes: `p-steps-item`, `p-steps-item-active`, `p-disabled`.
pub fn item_classes(active: bool, disabled: bool) -> Cow<'static, str> {
    let mut cn = ClassNames::new();
    cn.add("p-steps-item").add_if(active, "p-steps-item-active").add_if(disabled, "p-disabled");
    cn.finish_cow()
}

/// The item is disabled: it has `disabled`, or readonly is on and the item is not active (`isItemDisabled`).
pub fn is_item_disabled(item_disabled: bool, index: usize, active: usize, readonly: bool) -> bool {
    item_disabled || (readonly && index != active)
}

/// Link `tabindex` without moving focus: `-1` for a disabled one, its own (or `0`) for the active one, its own
/// (or `-1`) for the rest (`getItemTabIndex`).
pub fn link_tabindex(item_disabled: bool, item_tabindex: Option<i32>, index: usize, active: usize) -> i32 {
    if item_disabled {
        -1
    } else if index == active {
        item_tabindex.filter(|t| *t != 0).unwrap_or(0)
    } else {
        item_tabindex.unwrap_or(-1)
    }
}

/// Link `aria-disabled`: the item's `disabled` or readonly on an inactive item.
pub fn aria_disabled(item_disabled: bool, index: usize, active: usize, readonly: bool) -> bool {
    item_disabled || (readonly && index != active)
}

/// What a click does.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Click {
    /// Only cancel the link navigation.
    Ignore,
    /// Select the item and call the command; `prevent_default` — whether to cancel navigation (the item has no `url`).
    Activate {
        /// Whether to cancel the link navigation.
        prevent_default: bool,
    },
}

/// Click outcome (`onItemClick`): readonly or a disabled item — ignored.
pub fn click_outcome(item_disabled: bool, readonly: bool, has_url: bool) -> Click {
    if readonly || item_disabled { Click::Ignore } else { Click::Activate { prevent_default: !has_url } }
}

/// The item that focus moves to on a key; at the bounds it stays the same. `None` — the key does not move focus.
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

/// `Enter` and `Space` act as a click.
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
        // (disabled, index, active, readonly) → disabled
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
        assert_eq!(link_tabindex(true, Some(5), 1, 1), -1, "disabled");
        assert_eq!(link_tabindex(false, None, 1, 1), 0, "active");
        assert_eq!(link_tabindex(false, Some(3), 1, 1), 3, "active with its own");
        assert_eq!(link_tabindex(false, Some(0), 1, 1), 0);
        assert_eq!(link_tabindex(false, None, 0, 1), -1, "the rest");
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
