//! Pure Tabs logic: identifiers, classes, `tabindex`, keys, adjacent tab lookup and button scrolling
//! (from PrimeNG `tabs.ts`, `tab.ts`, `tablist.ts`, `tabpanel.ts` and `style/*`).

/// Tab `id`.
pub fn tab_id(tabs_id: &str, value: &str) -> String {
    format!("{tabs_id}_tab_{value}")
}

/// Panel `id`.
pub fn panel_id(tabs_id: &str, value: &str) -> String {
    format!("{tabs_id}_tabpanel_{value}")
}

/// Whether the tab or panel with value `value` is active.
pub fn is_active(current: Option<&str>, value: &str) -> bool {
    current == Some(value)
}

/// Tab `tabindex`: the group's `tabindex` for the active enabled tab, -1 for the others.
pub fn tabindex_for(disabled: bool, active: bool, tabs_tabindex: i32) -> i32 {
    if !disabled && active { tabs_tabindex } else { -1 }
}

/// `Tabs` root classes.
pub fn root_classes(scrollable: bool) -> &'static str {
    if scrollable { "p-tabs p-component p-tabs-scrollable" } else { "p-tabs p-component" }
}

/// Tab classes.
pub fn tab_classes(active: bool, disabled: bool) -> &'static str {
    match (active, disabled) {
        (false, false) => "p-tab",
        (true, false) => "p-tab p-tab-active",
        (false, true) => "p-tab p-disabled",
        (true, true) => "p-tab p-tab-active p-disabled",
    }
}

/// Panel classes.
pub fn panel_classes(active: bool) -> &'static str {
    if active { "p-tabpanel p-tabpanel-active" } else { "p-tabpanel" }
}

/// Action for a key on a tab.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyAction {
    /// Focus the next enabled tab (wrapping around).
    Next,
    /// Focus the previous enabled tab (wrapping around).
    Prev,
    /// Focus the first enabled tab.
    First,
    /// Focus the last enabled tab.
    Last,
    /// Scroll to the first enabled tab without focus.
    ScrollFirst,
    /// Scroll to the last enabled tab without focus.
    ScrollLast,
    /// Activate the tab.
    Activate,
}

/// Action for a `KeyboardEvent.code`.
pub fn key_action(code: &str) -> Option<KeyAction> {
    Some(match code {
        "ArrowRight" => KeyAction::Next,
        "ArrowLeft" => KeyAction::Prev,
        "Home" => KeyAction::First,
        "End" => KeyAction::Last,
        "PageUp" => KeyAction::ScrollFirst,
        "PageDown" => KeyAction::ScrollLast,
        "Enter" | "NumpadEnter" | "Space" => KeyAction::Activate,
        _ => return None,
    })
}

/// Index of the tab the action moves to, among tabs with the `enabled` availability flags.
///
/// `Next`/`Prev` look for the nearest enabled one and, if none is found, take the first/last (wrapping around). `Activate` has no target.
pub fn wrap_target(current: usize, enabled: &[bool], action: KeyAction) -> Option<usize> {
    let first = || enabled.iter().position(|&e| e);
    let last = || enabled.iter().rposition(|&e| e);
    match action {
        KeyAction::Next => enabled.iter().enumerate().skip(current + 1).find(|&(_, &e)| e).map(|(i, _)| i).or_else(first),
        KeyAction::Prev => enabled.iter().enumerate().take(current).rev().find(|&(_, &e)| e).map(|(i, _)| i).or_else(last),
        KeyAction::First | KeyAction::ScrollFirst => first(),
        KeyAction::Last | KeyAction::ScrollLast => last(),
        KeyAction::Activate => None,
    }
}

/// Whether to render the panel content.
pub fn should_render(lazy: bool, has_rendered: bool, active: bool) -> bool {
    !lazy || has_rendered || active
}

/// Whether the "back" button is needed: the content is scrolled from the start.
pub fn prev_visible(scroll_left: f64) -> bool {
    scroll_left.abs() != 0.0
}

/// Whether the "forward" button is needed: there is a hidden part on the right.
pub fn next_visible(list_width: f64, content_width: f64, scroll_left: f64, scroll_width: f64, width: f64) -> bool {
    list_width >= content_width && (scroll_left.abs() - scroll_width + width).abs() > 1.0
}

/// New scroll position for the "back" button.
pub fn prev_scroll(scroll_left: f64, width: f64) -> f64 {
    (scroll_left.abs() - width).max(0.0)
}

/// New scroll position for the "forward" button; `buttons` is the total width of the visible buttons.
pub fn next_scroll(scroll_left: f64, width: f64, buttons: f64, scroll_width: f64) -> f64 {
    let step = width - buttons;
    let pos = scroll_left + step;
    let last = scroll_width - step;
    if pos >= last { last } else { pos }
}

/// Indicator bar style.
pub fn ink_style(width: f64, left: f64) -> String {
    format!("width: {width}px; left: {left}px")
}

#[cfg(test)]
mod tests {
    use super::KeyAction::{Activate, First, Last, Next, Prev, ScrollFirst, ScrollLast};
    use super::*;

    #[test]
    fn ids_active_and_tabindex_table() {
        assert_eq!(tab_id("pn_id_1", "a"), "pn_id_1_tab_a");
        assert_eq!(panel_id("pn_id_1", "a"), "pn_id_1_tabpanel_a");
        assert!(is_active(Some("a"), "a") && !is_active(Some("b"), "a") && !is_active(None, "a"));
        // (disabled, active, tabs_tabindex, expected)
        let cases = [(false, true, 0, 0), (false, true, 3, 3), (false, false, 0, -1), (true, true, 0, -1), (true, false, 0, -1)];
        for (disabled, active, tabindex, expected) in cases {
            assert_eq!(tabindex_for(disabled, active, tabindex), expected, "{disabled} {active} {tabindex}");
        }
    }

    #[test]
    fn classes_table() {
        assert_eq!(root_classes(false), "p-tabs p-component");
        assert_eq!(root_classes(true), "p-tabs p-component p-tabs-scrollable");
        assert_eq!(tab_classes(false, false), "p-tab");
        assert_eq!(tab_classes(true, false), "p-tab p-tab-active");
        assert_eq!(tab_classes(false, true), "p-tab p-disabled");
        assert_eq!(tab_classes(true, true), "p-tab p-tab-active p-disabled");
        assert_eq!(panel_classes(false), "p-tabpanel");
        assert_eq!(panel_classes(true), "p-tabpanel p-tabpanel-active");
    }

    #[test]
    fn key_action_table() {
        let cases = [
            ("ArrowRight", Some(Next)),
            ("ArrowLeft", Some(Prev)),
            ("Home", Some(First)),
            ("End", Some(Last)),
            ("PageUp", Some(ScrollFirst)),
            ("PageDown", Some(ScrollLast)),
            ("Enter", Some(Activate)),
            ("NumpadEnter", Some(Activate)),
            ("Space", Some(Activate)),
            ("ArrowDown", None),
            ("Tab", None),
            ("KeyA", None),
        ];
        for (code, expected) in cases {
            assert_eq!(key_action(code), expected, "{code}");
        }
    }

    #[test]
    fn wrap_target_table() {
        let all = [true, true, true];
        let gap = [true, false, true];
        let ends_off = [false, true, true, false];
        // (current, enabled, action, expected)
        let cases: Vec<(usize, &[bool], KeyAction, Option<usize>)> = vec![
            (0, &all, Next, Some(1)),
            (2, &all, Next, Some(0)),
            (0, &all, Prev, Some(2)),
            (2, &all, Prev, Some(1)),
            (0, &gap, Next, Some(2)),
            (2, &gap, Prev, Some(0)),
            (2, &gap, Next, Some(0)),
            (1, &ends_off, Next, Some(2)),
            (2, &ends_off, Next, Some(1)),
            (1, &ends_off, Prev, Some(2)),
            (1, &ends_off, First, Some(1)),
            (1, &ends_off, Last, Some(2)),
            (1, &ends_off, ScrollFirst, Some(1)),
            (1, &ends_off, ScrollLast, Some(2)),
            (0, &[false, false], Next, None),
            (0, &[false, false], First, None),
            (0, &[], Last, None),
            (0, &all, Activate, None),
            (0, &[true], Next, Some(0)),
            (0, &[true], Prev, Some(0)),
        ];
        for (current, enabled, action, expected) in cases {
            assert_eq!(wrap_target(current, enabled, action), expected, "{current} {enabled:?} {action:?}");
        }
    }

    #[test]
    fn should_render_table() {
        // (lazy, has_rendered, active, expected)
        let cases = [
            (false, false, false, true),
            (false, true, true, true),
            (true, false, false, false),
            (true, false, true, true),
            (true, true, false, true),
            (true, true, true, true),
        ];
        for (lazy, rendered, active, expected) in cases {
            assert_eq!(should_render(lazy, rendered, active), expected, "{lazy} {rendered} {active}");
        }
    }

    #[test]
    fn navigator_table() {
        assert!(!prev_visible(0.0) && prev_visible(10.0) && prev_visible(-10.0));
        // (list, content, scrollLeft, scrollWidth, width, expected)
        let cases = [
            (500.0, 500.0, 0.0, 1000.0, 500.0, true),
            (500.0, 500.0, 500.0, 1000.0, 500.0, false),
            (500.0, 500.0, 499.5, 1000.0, 500.0, false),
            (500.0, 500.0, 498.0, 1000.0, 500.0, true),
            (500.0, 500.0, 0.0, 500.0, 500.0, false),
            (400.0, 500.0, 0.0, 1000.0, 500.0, false),
            (500.0, 500.0, -100.0, 1000.0, 500.0, true),
        ];
        for (list, content, left, scroll_width, width, expected) in cases {
            assert_eq!(next_visible(list, content, left, scroll_width, width), expected, "{list} {content} {left}");
        }
    }

    #[test]
    fn scroll_table() {
        assert_eq!(prev_scroll(300.0, 200.0), 100.0);
        assert_eq!(prev_scroll(100.0, 200.0), 0.0);
        assert_eq!(prev_scroll(-300.0, 200.0), 100.0);
        assert_eq!(next_scroll(0.0, 400.0, 80.0, 2000.0), 320.0);
        assert_eq!(next_scroll(1500.0, 400.0, 80.0, 2000.0), 1680.0);
        assert_eq!(next_scroll(1500.0, 400.0, 0.0, 1800.0), 1400.0, "no further than the end: scrollWidth - step");
        assert_eq!(ink_style(120.0, 8.5), "width: 120px; left: 8.5px");
    }
}
