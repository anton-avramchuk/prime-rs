//! Чистая логика Stepper: идентификаторы, состояния шага, разделитель, классы (по `stepper.ts` и `style/*` PrimeNG).

use prime_core::ClassNames;
use std::borrow::Cow;

/// Идентификатор кнопки шага: `<id>_step_<значение>`.
pub fn step_id(id: &str, value: u32) -> String {
    format!("{id}_step_{value}")
}

/// Идентификатор панели: `<id>_steppanel_<значение>`.
pub fn panel_id(id: &str, value: u32) -> String {
    format!("{id}_steppanel_{value}")
}

/// Активен ли шаг со значением `value` при текущем `current`.
pub fn is_active(current: Option<u32>, value: u32) -> bool {
    current == Some(value)
}

/// Недоступен ли шаг: неактивный и либо в линейном режиме, либо отключённый (`isStepDisabled`).
pub fn step_disabled(active: bool, linear: bool, disabled: bool) -> bool {
    !active && (linear || disabled)
}

/// Нужен ли разделитель после шага/элемента: значение не последнее в реестре (отсутствующее в реестре — нужен, как в PrimeNG).
pub fn separator_after(values: &[u32], value: u32) -> bool {
    values.iter().position(|v| *v == value) != values.len().checked_sub(1)
}

/// Классы корня: `p-stepper p-component` и `p-readonly` в линейном режиме.
pub fn root_classes(linear: bool) -> &'static str {
    if linear { "p-stepper p-component p-readonly" } else { "p-stepper p-component" }
}

/// Классы шага.
pub fn step_classes(active: bool, disabled: bool) -> Cow<'static, str> {
    let mut cn = ClassNames::new();
    cn.add("p-step").add_if(active, "p-step-active").add_if(disabled, "p-disabled");
    cn.finish_cow()
}

/// Классы элемента вертикальной раскладки.
pub fn item_classes(active: bool) -> &'static str {
    if active { "p-stepitem p-stepitem-active" } else { "p-stepitem" }
}

/// Классы панели: `p-steppanel-active` только у активной вертикальной.
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
        // (активен, linear, disabled) -> недоступен
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
        assert!(!separator_after(&[1], 1), "единственный шаг");
        assert!(separator_after(&[1, 2], 9), "неизвестное значение — как indexOf = -1 в PrimeNG");
        assert!(!separator_after(&[], 1), "пустой реестр: indexOf = -1 и длина − 1 = -1 равны");
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
        assert_eq!(panel_classes(false, true), "p-steppanel", "горизонтальная панель без -active");
        assert_eq!(panel_classes(true, false), "p-steppanel");
        assert_eq!(panel_classes(true, true), "p-steppanel p-steppanel-active");
    }
}
