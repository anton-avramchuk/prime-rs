//! Pure ConfirmPopup logic: classes, arrow, flip, outside clicks, keys and focus (per `confirmpopup.ts` of PrimeNG).

use prime_confirmation::ConfirmFocus;
use prime_core::ClassNames;
use std::borrow::Cow;

/// Root classes: `p-confirmpopup p-component`; above the target, `p-confirm-popup-flipped` (as in PrimeNG) and
/// `p-confirmpopup-flipped` (this is the rule name in the theme, so without it the flip would not be styled).
pub fn root_classes(flipped: bool, extra: Option<&str>) -> Cow<'static, str> {
    let mut cn = ClassNames::new();
    cn.add("p-confirmpopup p-component").add_if(flipped, "p-confirm-popup-flipped p-confirmpopup-flipped").add_user(extra);
    cn.finish_cow()
}

/// Value of `--p-confirmpopup-arrow-left`: arrow shift if the left edge of the panel is left of the left edge of the target, otherwise `0`.
pub fn arrow_left(container_left: f64, target_left: f64) -> f64 {
    if container_left < target_left { target_left - container_left } else { 0.0 }
}

/// The panel is positioned above the target (that is, flipped).
pub fn is_flipped(container_top: f64, target_top: f64) -> bool {
    container_top < target_top
}

/// Outside click: outside the panel and outside the target.
pub fn outside_click(inside_panel: bool, inside_target: bool) -> bool {
    !inside_panel && !inside_target
}

/// The confirmation is addressed to this component: the keys are equal (both unset also counts).
pub fn accepts_key(component: Option<&str>, confirmation: Option<&str>) -> bool {
    component == confirmation
}

/// Where to move focus: the confirmation value, and without it the component property.
pub fn focus_target(per_confirmation: Option<ConfirmFocus>, component: ConfirmFocus) -> ConfirmFocus {
    per_confirmation.unwrap_or(component)
}

/// Button label: the `*_button_props.label` property, then `*_label`, then the default translation.
pub fn button_label(props_label: Option<&str>, label: Option<&str>, default: &str) -> String {
    props_label.filter(|l| !l.is_empty()).or(label.filter(|l| !l.is_empty())).unwrap_or(default).to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn root_classes_table() {
        assert_eq!(root_classes(false, None), "p-confirmpopup p-component");
        assert_eq!(root_classes(true, None), "p-confirmpopup p-component p-confirm-popup-flipped p-confirmpopup-flipped");
        assert_eq!(root_classes(false, Some("mine")), "p-confirmpopup p-component mine");
    }

    #[test]
    fn arrow_and_flip_table() {
        assert_eq!(arrow_left(100.0, 120.0), 20.0);
        assert_eq!(arrow_left(120.0, 120.0), 0.0);
        assert_eq!(arrow_left(130.0, 120.0), 0.0);
        assert!(is_flipped(100.0, 200.0));
        assert!(!is_flipped(200.0, 100.0));
        assert!(!is_flipped(100.0, 100.0));
    }

    #[test]
    fn outside_click_table() {
        assert!(outside_click(false, false));
        assert!(!outside_click(true, false) && !outside_click(false, true) && !outside_click(true, true));
    }

    #[test]
    fn key_table() {
        assert!(accepts_key(None, None));
        assert!(accepts_key(Some("a"), Some("a")));
        assert!(!accepts_key(Some("a"), None));
        assert!(!accepts_key(None, Some("a")));
        assert!(!accepts_key(Some("a"), Some("b")));
    }

    #[test]
    fn focus_and_label_table() {
        assert_eq!(focus_target(None, ConfirmFocus::Accept), ConfirmFocus::Accept);
        assert_eq!(focus_target(Some(ConfirmFocus::Reject), ConfirmFocus::Accept), ConfirmFocus::Reject);
        assert_eq!(button_label(None, None, "Yes"), "Yes");
        assert_eq!(button_label(None, Some("Save"), "Yes"), "Save");
        assert_eq!(button_label(Some("Props"), Some("Save"), "Yes"), "Props");
        assert_eq!(button_label(Some(""), Some(""), "Yes"), "Yes");
    }
}
