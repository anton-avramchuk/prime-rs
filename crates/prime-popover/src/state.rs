//! Pure Popover logic: root classes, arrow, flip, closing on click and target change (after PrimeNG `popover.ts`).

/// Root classes.
pub fn root_classes(flipped: bool) -> &'static str {
    if flipped { "p-popover p-component p-popover-flipped" } else { "p-popover p-component" }
}

/// Value of `--p-popover-arrow-left`: the arrow shift if the panel's left edge is to the left of the target's left edge.
pub fn arrow_left(container_left: f64, target_left: f64, border_radius: f64) -> f64 {
    if container_left < target_left { target_left - container_left - border_radius * 2.0 } else { 0.0 }
}

/// The panel is above the target (i.e. flipped).
pub fn is_flipped(container_top: f64, target_top: f64) -> bool {
    container_top < target_top
}

/// A click outside: outside the panel and outside the target.
pub fn outside_click(inside_panel: bool, inside_target: bool) -> bool {
    !inside_panel && !inside_target
}

/// What a `toggle` call should do.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Retarget {
    /// The panel is hidden: show it.
    Show,
    /// The panel is visible at the same target: hide it.
    Hide,
    /// The panel is visible at a different target: hide it and show at the new one.
    Switch,
}

/// Decision for `toggle`: `visible` — the panel is visible, `same_target` — the target did not change.
pub fn retarget(visible: bool, same_target: bool) -> Retarget {
    match (visible, same_target) {
        (false, _) => Retarget::Show,
        (true, true) => Retarget::Hide,
        (true, false) => Retarget::Switch,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn root_classes_table() {
        assert_eq!(root_classes(false), "p-popover p-component");
        assert_eq!(root_classes(true), "p-popover p-component p-popover-flipped");
    }

    #[test]
    fn arrow_left_table() {
        // (container_left, target_left, radius, expected)
        let cases = [(100.0, 120.0, 6.0, 8.0), (120.0, 120.0, 6.0, 0.0), (130.0, 120.0, 6.0, 0.0), (0.0, 50.0, 0.0, 50.0)];
        for (c, t, r, want) in cases {
            assert_eq!(arrow_left(c, t, r), want, "{c} {t} {r}");
        }
    }

    #[test]
    fn flipped_table() {
        assert!(is_flipped(100.0, 200.0));
        assert!(!is_flipped(200.0, 100.0));
        assert!(!is_flipped(100.0, 100.0));
    }

    #[test]
    fn outside_click_table() {
        assert!(outside_click(false, false));
        assert!(!outside_click(true, false));
        assert!(!outside_click(false, true));
        assert!(!outside_click(true, true));
    }

    #[test]
    fn retarget_table() {
        assert_eq!(retarget(false, true), Retarget::Show);
        assert_eq!(retarget(false, false), Retarget::Show);
        assert_eq!(retarget(true, true), Retarget::Hide);
        assert_eq!(retarget(true, false), Retarget::Switch);
    }
}
