//! Чистая логика Fieldset: классы и `data-p` (по `fieldsetstyle.ts` и `Fieldset.dataP` PrimeNG).

use prime_core::{ClassNames, DataP};
use std::borrow::Cow;

/// Добавляет классы корня в порядке `fieldsetstyle.ts`.
pub fn write_root_classes(cn: &mut ClassNames, toggleable: bool, collapsed: bool) {
    cn.add("p-fieldset p-component").add_if(toggleable, "p-fieldset-toggleable").add_if(toggleable && collapsed, "p-fieldset-collapsed");
}

/// Значение `data-p` корня и легенды: `toggleable`.
pub fn data_p(toggleable: bool) -> Cow<'static, str> {
    let mut d = DataP::new();
    d.add_if(toggleable, "toggleable");
    d.finish_cow()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn root(toggleable: bool, collapsed: bool) -> String {
        let mut cn = ClassNames::new();
        write_root_classes(&mut cn, toggleable, collapsed);
        cn.finish()
    }

    #[test]
    fn root_classes_and_data_p_table() {
        let cases = [
            (false, false, "p-fieldset p-component", ""),
            (false, true, "p-fieldset p-component", ""),
            (true, false, "p-fieldset p-component p-fieldset-toggleable", "toggleable"),
            (true, true, "p-fieldset p-component p-fieldset-toggleable p-fieldset-collapsed", "toggleable"),
        ];
        for (t, c, class, data) in cases {
            assert_eq!(root(t, c), class, "toggleable={t} collapsed={c}");
            assert_eq!(data_p(t), data, "toggleable={t}");
        }
    }
}
