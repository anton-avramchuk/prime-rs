//! Pure Stepper logic: identifiers, step states, separator, classes (after PrimeNG `stepper.ts` and `style/*`).

use prime_rs_core::ClassNames;
use std::borrow::Cow;

/// Step button identifier: `<id>_step_<value>`.
pub fn step_id(id: &str, value: u32) -> String {
    format!("{id}_step_{value}")
}

/// Panel identifier: `<id>_steppanel_<value>`.
pub fn panel_id(id: &str, value: u32) -> String {
    format!("{id}_steppanel_{value}")
}

/// Whether the step with value `value` is active at the current `current`.
pub fn is_active(current: Option<u32>, value: u32) -> bool {
    current == Some(value)
}

/// Whether the step is unavailable: inactive and either in linear mode or disabled (`isStepDisabled`).
pub fn step_disabled(active: bool, linear: bool, disabled: bool) -> bool {
    !active && (linear || disabled)
}

/// Whether a separator is needed after a step/item: the value is not last in the registry (one absent from the registry needs it, as in PrimeNG).
pub fn separator_after(values: &[u32], value: u32) -> bool {
    values.iter().position(|v| *v == value) != values.len().checked_sub(1)
}

/// Root classes: `p-stepper p-component` and `p-readonly` in linear mode.
pub fn root_classes(linear: bool) -> &'static str {
    if linear { "p-stepper p-component p-readonly" } else { "p-stepper p-component" }
}

/// Step classes.
pub fn step_classes(active: bool, disabled: bool) -> Cow<'static, str> {
    let mut cn = ClassNames::new();
    cn.add("p-step").add_if(active, "p-step-active").add_if(disabled, "p-disabled");
    cn.finish_cow()
}

/// Vertical layout item classes.
pub fn item_classes(active: bool) -> &'static str {
    if active { "p-stepitem p-stepitem-active" } else { "p-stepitem" }
}

/// Panel classes: `p-steppanel-active` only on the active vertical one.
pub fn panel_classes(vertical: bool, active: bool) -> &'static str {
    if vertical && active { "p-steppanel p-steppanel-active" } else { "p-steppanel" }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids() {
        assert_eq!(step_id("pn_id_1", 2), "pn_id_1_step_2");
        assert_eq!(panel_id("pn_id_1", 2), "pn_id_1_steppanel_2");
    }

    #[test]
    fn active_and_disabled_table() {
        assert!(is_active(Some(2), 2));
        assert!(!is_active(Some(1), 2));
        assert!(!is_active(None, 1));
        // (active, linear, disabled) -> unavailable
        let cases = [
            (true, false, false, false),
            (true, true, false, false),
            (true, false, true, false),
            (true, true, true, false),
            (false, false, false, false),
            (false, true, false, true),
            (false, false, true, true),
            (false, true, true, true),
        ];
        for (active, linear, disabled, expected) in cases {
            assert_eq!(step_disabled(active, linear, disabled), expected, "{active} {linear} {disabled}");
        }
    }

    #[test]
    fn separator_is_skipped_only_after_the_last_registered_value() {
        assert!(separator_after(&[1, 2, 3], 1));
        assert!(separator_after(&[1, 2, 3], 2));
        assert!(!separator_after(&[1, 2, 3], 3));
        assert!(!separator_after(&[1], 1), "the only step");
        assert!(separator_after(&[1, 2], 9), "unknown value: like indexOf = -1 in PrimeNG");
        assert!(!separator_after(&[], 1), "empty registry: indexOf = -1 and length - 1 = -1 are equal");
    }

    #[test]
    fn classes_table() {
        assert_eq!(root_classes(false), "p-stepper p-component");
        assert_eq!(root_classes(true), "p-stepper p-component p-readonly");
        assert_eq!(step_classes(false, false), "p-step");
        assert_eq!(step_classes(true, false), "p-step p-step-active");
        assert_eq!(step_classes(false, true), "p-step p-disabled");
        assert_eq!(step_classes(true, true), "p-step p-step-active p-disabled");
        assert_eq!(item_classes(false), "p-stepitem");
        assert_eq!(item_classes(true), "p-stepitem p-stepitem-active");
        assert_eq!(panel_classes(false, true), "p-steppanel", "horizontal panel without -active");
        assert_eq!(panel_classes(true, false), "p-steppanel");
        assert_eq!(panel_classes(true, true), "p-steppanel p-steppanel-active");
    }
}
