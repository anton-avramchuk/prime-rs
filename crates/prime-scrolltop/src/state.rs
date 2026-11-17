//! Pure ScrollTop logic: visibility threshold, root classes and show phase (after PrimeNG `scrolltop.ts` and `scrolltopstyle.ts`).

/// What the button scrolls.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ScrollTopTarget {
    /// The window (the button is `fixed`).
    #[default]
    Window,
    /// The parent element (the button is `sticky`).
    Parent,
}

/// Scroll behavior on click.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ScrollTopBehavior {
    /// Smooth scrolling.
    #[default]
    Smooth,
    /// Instant jump.
    Auto,
}

/// Button show phase.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    /// Not shown.
    Hidden,
    /// Appearing (the enter animation is running).
    Entering,
    /// Shown.
    Shown,
    /// Disappearing (the exit animation is running).
    Leaving,
}

/// The button must be visible: scroll is strictly greater than the threshold.
pub fn is_visible(scroll: f64, threshold: f64) -> bool {
    scroll > threshold
}

/// Root classes: `p-scrolltop` and `p-scrolltop-sticky` for the parent.
pub fn root_classes(target: ScrollTopTarget) -> &'static str {
    match target {
        ScrollTopTarget::Window => "p-scrolltop",
        ScrollTopTarget::Parent => "p-scrolltop p-scrolltop-sticky",
    }
}

/// Enter animation classes: first the initial state, then (on the next frame) only the active transition.
pub fn enter_classes(started: bool) -> &'static str {
    if started { "p-scrolltop-enter-active p-scrolltop-enter-to" } else { "p-scrolltop-enter-from p-scrolltop-enter-active" }
}

/// Exit animation classes.
pub fn leave_classes(started: bool) -> &'static str {
    if started { "p-scrolltop-leave-active p-scrolltop-leave-to" } else { "p-scrolltop-leave-from p-scrolltop-leave-active" }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn visibility_is_strictly_above_threshold() {
        let cases = [
            (0.0, 400.0, false),
            (400.0, 400.0, false),
            (400.5, 400.0, true),
            (401.0, 400.0, true),
            (150.0, 100.0, true),
            (100.0, 100.0, false),
        ];
        for (scroll, threshold, want) in cases {
            assert_eq!(is_visible(scroll, threshold), want, "{scroll} > {threshold}");
        }
    }

    #[test]
    fn root_classes_by_target() {
        assert_eq!(root_classes(ScrollTopTarget::Window), "p-scrolltop");
        assert_eq!(root_classes(ScrollTopTarget::Parent), "p-scrolltop p-scrolltop-sticky");
    }

    #[test]
    fn animation_classes() {
        assert_eq!(enter_classes(false), "p-scrolltop-enter-from p-scrolltop-enter-active");
        assert_eq!(enter_classes(true), "p-scrolltop-enter-active p-scrolltop-enter-to");
        assert_eq!(leave_classes(false), "p-scrolltop-leave-from p-scrolltop-leave-active");
        assert_eq!(leave_classes(true), "p-scrolltop-leave-active p-scrolltop-leave-to");
    }

    #[test]
    fn defaults() {
        assert_eq!(ScrollTopTarget::default(), ScrollTopTarget::Window);
        assert_eq!(ScrollTopBehavior::default(), ScrollTopBehavior::Smooth);
    }
}
