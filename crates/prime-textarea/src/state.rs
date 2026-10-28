//! Чистая логика Textarea: классы (по `textareastyle.ts` PrimeNG) и расчёт высоты при автоподгонке.

use prime_core::{ClassNames, Size, Variant};

/// Снимок свойств, влияющих на классы.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TextareaState {
    /// Значение не пусто.
    pub filled: bool,
    /// Включена автоподгонка высоты.
    pub auto_resize: bool,
    /// Итоговый вариант (свойство или `PrimeConfig.input_variant`).
    pub variant: Option<Variant>,
    /// На всю ширину контейнера.
    pub fluid: bool,
    /// Размер.
    pub size: Option<Size>,
    /// Недопустимое значение.
    pub invalid: bool,
}

/// Добавляет классы корня в порядке `textareastyle.ts`.
pub fn write_classes(cn: &mut ClassNames, s: &TextareaState) {
    cn.add("p-textarea p-component")
        .add_if(s.filled, "p-filled")
        .add_if(s.auto_resize, "p-textarea-resizable")
        .add_if(s.variant == Some(Variant::Filled), "p-variant-filled")
        .add_if(s.fluid, "p-textarea-fluid")
        .add_if(s.size == Some(Size::Small), "p-inputfield-sm p-textarea-sm")
        .add_if(s.size == Some(Size::Large), "p-textarea-lg p-inputfield-lg")
        .add_if(s.invalid, "p-invalid");
}

/// Результат автоподгонки высоты.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Fit {
    /// Высота в пикселях.
    pub height: f64,
    /// Вертикальная прокрутка (иначе `overflow: hidden`).
    pub scroll: bool,
}

/// Высота при автоподгонке: `scroll_height`, но не больше `max_height` (если он задан); при достижении предела включается прокрутка.
pub fn fit_height(scroll_height: f64, max_height: Option<f64>) -> Fit {
    match max_height {
        Some(max) if scroll_height >= max => Fit { height: max, scroll: true },
        _ => Fit { height: scroll_height, scroll: false },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn classes(s: &TextareaState) -> String {
        let mut cn = ClassNames::new();
        write_classes(&mut cn, s);
        cn.finish()
    }

    #[test]
    fn classes_table() {
        let cases = [
            (TextareaState::default(), "p-textarea p-component"),
            (TextareaState { filled: true, ..Default::default() }, "p-textarea p-component p-filled"),
            (TextareaState { auto_resize: true, ..Default::default() }, "p-textarea p-component p-textarea-resizable"),
            (TextareaState { variant: Some(Variant::Filled), ..Default::default() }, "p-textarea p-component p-variant-filled"),
            (TextareaState { variant: Some(Variant::Outlined), ..Default::default() }, "p-textarea p-component"),
            (TextareaState { fluid: true, ..Default::default() }, "p-textarea p-component p-textarea-fluid"),
            (TextareaState { size: Some(Size::Small), ..Default::default() }, "p-textarea p-component p-inputfield-sm p-textarea-sm"),
            (TextareaState { size: Some(Size::Large), ..Default::default() }, "p-textarea p-component p-textarea-lg p-inputfield-lg"),
            (TextareaState { invalid: true, ..Default::default() }, "p-textarea p-component p-invalid"),
            (
                TextareaState {
                    filled: true,
                    auto_resize: true,
                    variant: Some(Variant::Filled),
                    fluid: true,
                    size: Some(Size::Large),
                    invalid: true,
                },
                "p-textarea p-component p-filled p-textarea-resizable p-variant-filled p-textarea-fluid p-textarea-lg p-inputfield-lg p-invalid",
            ),
        ];
        for (s, class) in cases {
            assert_eq!(classes(&s), class, "{s:?}");
        }
    }

    #[test]
    fn fit_height_follows_content_until_max() {
        assert_eq!(fit_height(80.0, None), Fit { height: 80.0, scroll: false });
        assert_eq!(fit_height(80.0, Some(200.0)), Fit { height: 80.0, scroll: false });
        assert_eq!(fit_height(200.0, Some(200.0)), Fit { height: 200.0, scroll: true });
        assert_eq!(fit_height(500.0, Some(200.0)), Fit { height: 200.0, scroll: true });
    }
}
