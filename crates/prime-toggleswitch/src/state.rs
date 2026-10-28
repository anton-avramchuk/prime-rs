//! Чистая логика ToggleSwitch: классы и `data-p` (по `toggleswitchstyle.ts` и `ToggleSwitch.dataP` PrimeNG).

use prime_core::{ClassNames, DataP};

/// Снимок свойств, влияющих на классы и `data-p`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ToggleSwitchState {
    /// Включён.
    pub checked: bool,
    /// Недоступен.
    pub disabled: bool,
    /// Недопустимое значение.
    pub invalid: bool,
}

/// Добавляет классы корня в порядке `toggleswitchstyle.ts`.
pub fn write_classes(cn: &mut ClassNames, s: &ToggleSwitchState) {
    cn.add("p-toggleswitch p-component")
        .add_if(s.checked, "p-toggleswitch-checked")
        .add_if(s.disabled, "p-disabled")
        .add_if(s.invalid, "p-invalid");
}

/// Значение `data-p`: `checked`, `disabled`, `invalid`.
pub fn data_p(s: &ToggleSwitchState) -> String {
    let mut d = DataP::new();
    d.add_if(s.checked, "checked").add_if(s.disabled, "disabled").add_if(s.invalid, "invalid");
    d.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn classes(s: &ToggleSwitchState) -> String {
        let mut cn = ClassNames::new();
        write_classes(&mut cn, s);
        cn.finish()
    }

    #[test]
    fn classes_and_data_p_table() {
        let st = |checked, disabled, invalid| ToggleSwitchState { checked, disabled, invalid };
        let cases = [
            (ToggleSwitchState::default(), "p-toggleswitch p-component", ""),
            (st(true, false, false), "p-toggleswitch p-component p-toggleswitch-checked", "checked"),
            (st(false, true, false), "p-toggleswitch p-component p-disabled", "disabled"),
            (st(false, false, true), "p-toggleswitch p-component p-invalid", "invalid"),
            (st(true, true, true), "p-toggleswitch p-component p-toggleswitch-checked p-disabled p-invalid", "checked disabled invalid"),
        ];
        for (s, class, data) in cases {
            assert_eq!(classes(&s), class, "{s:?}");
            assert_eq!(data_p(&s), data, "{s:?}");
        }
    }
}
