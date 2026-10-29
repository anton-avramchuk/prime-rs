//! Чистая логика Rating: выбор значения, классы и `data-p` (по `ratingstyle.ts` и `Rating.dataP` PrimeNG).

use prime_core::{ClassNames, DataP};
use std::borrow::Cow;

/// Значение после выбора звезды: повторный выбор текущего значения сбрасывает его (как `onOptionSelect` PrimeNG).
pub fn next_value(current: Option<u32>, star: u32) -> Option<u32> {
    if current == Some(star) { None } else { Some(star) }
}

/// Снимок свойств, влияющих на классы и `data-p` корня.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RatingState {
    /// Только чтение.
    pub readonly: bool,
    /// Недоступен.
    pub disabled: bool,
}

/// Добавляет классы корня в порядке `ratingstyle.ts`.
pub fn write_classes(cn: &mut ClassNames, s: &RatingState) {
    cn.add("p-rating").add_if(s.readonly, "p-readonly").add_if(s.disabled, "p-disabled");
}

/// Значение `data-p` корня: `readonly`, `disabled`.
pub fn data_p(s: &RatingState) -> Cow<'static, str> {
    let mut d = DataP::new();
    d.add_if(s.readonly, "readonly").add_if(s.disabled, "disabled");
    d.finish_cow()
}

/// Добавляет классы опции (звезды): `p-rating-option`, активная, видимый фокус.
pub fn write_option_classes(cn: &mut ClassNames, active: bool, focus_visible: bool) {
    cn.add("p-rating-option").add_if(active, "p-rating-option-active").add_if(focus_visible, "p-focus-visible");
}

/// Добавляет классы иконки: включённая или выключенная, `p-invalid`.
pub fn write_icon_classes(cn: &mut ClassNames, on: bool, invalid: bool) {
    cn.add(if on { "p-rating-icon p-rating-on-icon" } else { "p-rating-icon p-rating-off-icon" }).add_if(invalid, "p-invalid");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn built(f: impl FnOnce(&mut ClassNames)) -> String {
        let mut cn = ClassNames::new();
        f(&mut cn);
        cn.finish()
    }

    #[test]
    fn next_value_table() {
        assert_eq!(next_value(None, 3), Some(3));
        assert_eq!(next_value(Some(1), 3), Some(3));
        assert_eq!(next_value(Some(3), 1), Some(1));
        assert_eq!(next_value(Some(3), 3), None);
    }

    #[test]
    fn root_classes_and_data_p_table() {
        let st = |readonly, disabled| RatingState { readonly, disabled };
        let cases = [
            (st(false, false), "p-rating", ""),
            (st(true, false), "p-rating p-readonly", "readonly"),
            (st(false, true), "p-rating p-disabled", "disabled"),
            (st(true, true), "p-rating p-readonly p-disabled", "readonly disabled"),
        ];
        for (s, class, data) in cases {
            assert_eq!(built(|cn| write_classes(cn, &s)), class, "{s:?}");
            assert_eq!(data_p(&s), data, "{s:?}");
        }
    }

    #[test]
    fn option_classes_table() {
        let cases = [
            (false, false, "p-rating-option"),
            (true, false, "p-rating-option p-rating-option-active"),
            (false, true, "p-rating-option p-focus-visible"),
            (true, true, "p-rating-option p-rating-option-active p-focus-visible"),
        ];
        for (active, focus, class) in cases {
            assert_eq!(built(|cn| write_option_classes(cn, active, focus)), class, "{active} {focus}");
        }
    }

    #[test]
    fn icon_classes_table() {
        let cases = [
            (true, false, "p-rating-icon p-rating-on-icon"),
            (false, false, "p-rating-icon p-rating-off-icon"),
            (true, true, "p-rating-icon p-rating-on-icon p-invalid"),
            (false, true, "p-rating-icon p-rating-off-icon p-invalid"),
        ];
        for (on, invalid, class) in cases {
            assert_eq!(built(|cn| write_icon_classes(cn, on, invalid)), class, "{on} {invalid}");
        }
    }
}
