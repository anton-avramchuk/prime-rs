//! Чистая логика Select: классы и `data-p` (по `selectstyle.ts` и `Select.*DataP` PrimeNG).

use prime_core::{ClassNames, DataP, Size};
use std::borrow::Cow;

/// Снимок свойств, влияющих на классы и `data-p` корня.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SelectState {
    /// Недоступен.
    pub disabled: bool,
    /// Вариант `filled`.
    pub variant_filled: bool,
    /// Метка в фокусе.
    pub focused: bool,
    /// Недопустимое значение.
    pub invalid: bool,
    /// Выбрано значение.
    pub filled: bool,
    /// Панель открыта.
    pub open: bool,
    /// На всю ширину.
    pub fluid: bool,
    /// Размер.
    pub size: Option<Size>,
}

/// Добавляет классы корня в порядке `selectstyle.ts`.
pub fn write_classes(cn: &mut ClassNames, s: &SelectState) {
    cn.add("p-select p-component p-inputwrapper")
        .add_if(s.disabled, "p-disabled")
        .add_if(s.variant_filled, "p-variant-filled")
        .add_if(s.focused, "p-focus")
        .add_if(s.invalid, "p-invalid")
        .add_if(s.filled, "p-inputwrapper-filled")
        .add_if(s.focused || s.open, "p-inputwrapper-focus")
        .add_if(s.open, "p-select-open")
        .add_if(s.fluid, "p-select-fluid")
        .add_if(s.size == Some(Size::Small), "p-select-sm p-inputfield-sm")
        .add_if(s.size == Some(Size::Large), "p-select-lg p-inputfield-lg");
}

/// Значение `data-p` корня: `invalid`, `disabled`, `focus`, `fluid`, `filled`, размер.
pub fn data_p(s: &SelectState) -> Cow<'static, str> {
    let mut d = DataP::new();
    d.add_if(s.invalid, "invalid")
        .add_if(s.disabled, "disabled")
        .add_if(s.focused, "focus")
        .add_if(s.fluid, "fluid")
        .add_if(s.variant_filled, "filled")
        .add_opt(s.size.map(Size::as_str));
    d.finish_cow()
}

/// Что показывает метка.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LabelKind {
    /// Метка выбранного варианта (или шаблон выбранного).
    Selected,
    /// Подсказка `placeholder`.
    Placeholder,
    /// Ничего: нет ни выбора, ни подсказки.
    Empty,
}

/// Добавляет классы метки.
pub fn write_label_classes(cn: &mut ClassNames, kind: LabelKind) {
    cn.add("p-select-label")
        .add_if(kind == LabelKind::Placeholder, "p-placeholder")
        .add_if(kind == LabelKind::Empty, "p-select-label-empty");
}

/// Значение `data-p` метки: `placeholder`, `clearable`, `disabled`, размер, `empty`.
pub fn label_data_p(kind: LabelKind, clearable: bool, disabled: bool, size: Option<Size>) -> Cow<'static, str> {
    let mut d = DataP::new();
    d.add_if(kind == LabelKind::Placeholder, "placeholder")
        .add_if(clearable, "clearable")
        .add_if(disabled, "disabled")
        .add_opt(size.map(Size::as_str))
        .add_if(kind == LabelKind::Empty, "empty");
    d.finish_cow()
}

/// Добавляет классы варианта.
pub fn write_option_classes(cn: &mut ClassNames, selected: bool, checkmark: bool, disabled: bool, focused: bool) {
    cn.add("p-select-option")
        .add_if(selected && !checkmark, "p-select-option-selected")
        .add_if(disabled, "p-disabled")
        .add_if(focused, "p-focus");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn classes(s: &SelectState) -> String {
        let mut cn = ClassNames::new();
        write_classes(&mut cn, s);
        cn.finish()
    }

    #[test]
    fn root_classes_and_data_p_table() {
        let b = SelectState::default();
        let cases = [
            (b, "p-select p-component p-inputwrapper", ""),
            (SelectState { disabled: true, ..b }, "p-select p-component p-inputwrapper p-disabled", "disabled"),
            (SelectState { variant_filled: true, ..b }, "p-select p-component p-inputwrapper p-variant-filled", "filled"),
            (SelectState { focused: true, ..b }, "p-select p-component p-inputwrapper p-focus p-inputwrapper-focus", "focus"),
            (SelectState { invalid: true, ..b }, "p-select p-component p-inputwrapper p-invalid", "invalid"),
            (SelectState { filled: true, ..b }, "p-select p-component p-inputwrapper p-inputwrapper-filled", ""),
            (SelectState { open: true, ..b }, "p-select p-component p-inputwrapper p-inputwrapper-focus p-select-open", ""),
            (SelectState { fluid: true, ..b }, "p-select p-component p-inputwrapper p-select-fluid", "fluid"),
            (SelectState { size: Some(Size::Small), ..b }, "p-select p-component p-inputwrapper p-select-sm p-inputfield-sm", "small"),
            (SelectState { size: Some(Size::Large), ..b }, "p-select p-component p-inputwrapper p-select-lg p-inputfield-lg", "large"),
            (
                SelectState {
                    disabled: true,
                    variant_filled: true,
                    focused: true,
                    invalid: true,
                    filled: true,
                    open: true,
                    fluid: true,
                    size: Some(Size::Large),
                },
                "p-select p-component p-inputwrapper p-disabled p-variant-filled p-focus p-invalid p-inputwrapper-filled p-inputwrapper-focus p-select-open p-select-fluid p-select-lg p-inputfield-lg",
                "invalid disabled focus fluid filled large",
            ),
        ];
        for (s, class, data) in cases {
            assert_eq!(classes(&s), class, "{s:?}");
            assert_eq!(data_p(&s), data, "{s:?}");
        }
    }

    #[test]
    fn label_classes_and_data_p() {
        let built = |kind| {
            let mut cn = ClassNames::new();
            write_label_classes(&mut cn, kind);
            cn.finish()
        };
        assert_eq!(built(LabelKind::Selected), "p-select-label");
        assert_eq!(built(LabelKind::Placeholder), "p-select-label p-placeholder");
        assert_eq!(built(LabelKind::Empty), "p-select-label p-select-label-empty");
        assert_eq!(label_data_p(LabelKind::Selected, false, false, None), "");
        assert_eq!(label_data_p(LabelKind::Placeholder, true, true, Some(Size::Small)), "placeholder clearable disabled small");
        assert_eq!(label_data_p(LabelKind::Empty, false, false, None), "empty");
    }

    #[test]
    fn option_classes_table() {
        let built = |selected, checkmark, disabled, focused| {
            let mut cn = ClassNames::new();
            write_option_classes(&mut cn, selected, checkmark, disabled, focused);
            cn.finish()
        };
        assert_eq!(built(false, false, false, false), "p-select-option");
        assert_eq!(built(true, false, false, false), "p-select-option p-select-option-selected");
        assert_eq!(built(true, true, false, false), "p-select-option", "с галочкой выделения классом нет");
        assert_eq!(built(false, false, true, true), "p-select-option p-disabled p-focus");
        assert_eq!(built(true, false, true, true), "p-select-option p-select-option-selected p-disabled p-focus");
    }
}
