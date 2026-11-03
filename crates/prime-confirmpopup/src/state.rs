//! Чистая логика ConfirmPopup: классы, стрелка, переворот, клики снаружи, ключи и фокус (по `confirmpopup.ts` PrimeNG).

use prime_confirmation::ConfirmFocus;
use prime_core::ClassNames;
use std::borrow::Cow;

/// Классы корня: `p-confirmpopup p-component`; над целью — `p-confirm-popup-flipped` (как в PrimeNG) и
/// `p-confirmpopup-flipped` (так называется правило в теме, поэтому без него переворот не оформился бы).
pub fn root_classes(flipped: bool, extra: Option<&str>) -> Cow<'static, str> {
    let mut cn = ClassNames::new();
    cn.add("p-confirmpopup p-component").add_if(flipped, "p-confirm-popup-flipped p-confirmpopup-flipped").add_user(extra);
    cn.finish_cow()
}

/// Значение `--p-confirmpopup-arrow-left`: сдвиг стрелки, если левый край панели левее левого края цели, иначе `0`.
pub fn arrow_left(container_left: f64, target_left: f64) -> f64 {
    if container_left < target_left { target_left - container_left } else { 0.0 }
}

/// Панель расположена выше цели (то есть перевернута).
pub fn is_flipped(container_top: f64, target_top: f64) -> bool {
    container_top < target_top
}

/// Клик снаружи: вне панели и вне цели.
pub fn outside_click(inside_panel: bool, inside_target: bool) -> bool {
    !inside_panel && !inside_target
}

/// Подтверждение адресовано этому компоненту: ключи равны (оба не заданы — тоже).
pub fn accepts_key(component: Option<&str>, confirmation: Option<&str>) -> bool {
    component == confirmation
}

/// Куда переносить фокус: значение подтверждения, а без него — свойство компонента.
pub fn focus_target(per_confirmation: Option<ConfirmFocus>, component: ConfirmFocus) -> ConfirmFocus {
    per_confirmation.unwrap_or(component)
}

/// Подпись кнопки: свойство `*_button_props.label`, затем `*_label`, затем перевод по умолчанию.
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
