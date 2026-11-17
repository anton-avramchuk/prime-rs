//! Pure Inplace logic: view classes and state transitions (based on PrimeNG `inplace.ts` and `inplacestyle.ts`).

/// User action.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    /// Click on the view.
    Click,
    /// `Enter` on the view.
    Enter,
    /// The close button or the close function from the content.
    Close,
}

/// New `active` state after the action.
///
/// When `disabled`, nothing changes; `prevent_click` disables the click and closing, but not `Enter` (as in PrimeNG).
pub fn next_state(active: bool, action: Action, disabled: bool, prevent_click: bool) -> bool {
    if disabled {
        return active;
    }
    match action {
        Action::Click if !prevent_click => true,
        Action::Close if !prevent_click => false,
        Action::Enter => true,
        Action::Click | Action::Close => active,
    }
}

/// View classes.
pub fn display_classes(disabled: bool) -> &'static str {
    if disabled { "p-inplace-display p-disabled" } else { "p-inplace-display" }
}

#[cfg(test)]
mod tests {
    use super::Action::{Click, Close, Enter};
    use super::*;

    #[test]
    fn next_state_table() {
        // (active, action, disabled, prevent_click, expected)
        let cases = [
            (false, Click, false, false, true),
            (false, Click, false, true, false),
            (false, Click, true, false, false),
            (false, Enter, false, false, true),
            (false, Enter, false, true, true),
            (false, Enter, true, false, false),
            (true, Close, false, false, false),
            (true, Close, false, true, true),
            (true, Close, true, false, true),
            (true, Click, false, false, true),
            (true, Enter, false, false, true),
        ];
        for (active, action, disabled, prevent, expected) in cases {
            assert_eq!(
                next_state(active, action, disabled, prevent),
                expected,
                "{active} {action:?} disabled={disabled} prevent={prevent}"
            );
        }
    }

    #[test]
    fn display_classes_table() {
        assert_eq!(display_classes(false), "p-inplace-display");
        assert_eq!(display_classes(true), "p-inplace-display p-disabled");
    }
}
