//! Чистая логика ConfirmDialog: приоритет значений, положение, подписи кнопок и классы (по `confirmdialog.ts` PrimeNG).

use prime_dialog::DialogPosition;

/// `option(name)` PrimeNG: значение подтверждения, а если оно не задано, — свойство компонента.
pub fn pick<T>(from_confirmation: Option<T>, from_component: T) -> T {
    from_confirmation.unwrap_or(from_component)
}

/// Положение окна из строки подтверждения; неизвестное значение — по центру.
pub fn position_of(name: &str) -> DialogPosition {
    match name {
        "top" => DialogPosition::Top,
        "bottom" => DialogPosition::Bottom,
        "left" => DialogPosition::Left,
        "right" => DialogPosition::Right,
        "topleft" => DialogPosition::TopLeft,
        "topright" => DialogPosition::TopRight,
        "bottomleft" => DialogPosition::BottomLeft,
        "bottomright" => DialogPosition::BottomRight,
        _ => DialogPosition::Center,
    }
}

/// Подпись кнопки: `*_label`, затем `*_button_props.label`, затем перевод по умолчанию (пустые строки не считаются).
pub fn button_label(label: Option<&str>, props_label: Option<&str>, default: &str) -> String {
    label.filter(|l| !l.is_empty()).or(props_label.filter(|l| !l.is_empty())).unwrap_or(default).to_owned()
}

/// Класс кнопки: базовый плюс пользовательский.
pub fn button_class(base: &str, extra: Option<&str>) -> String {
    match extra.filter(|e| !e.is_empty()) {
        Some(extra) => format!("{base} {extra}"),
        None => base.to_owned(),
    }
}

/// Класс корня окна: `p-confirmdialog` плюс пользовательский.
pub fn root_class(extra: Option<&str>) -> String {
    button_class("p-confirmdialog", extra)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pick_prefers_the_confirmation() {
        assert_eq!(pick(Some(1), 2), 1);
        assert_eq!(pick(None, 2), 2);
        assert!(!pick(Some(false), true));
    }

    #[test]
    fn positions_parse() {
        let table = [
            ("center", DialogPosition::Center),
            ("top", DialogPosition::Top),
            ("bottom", DialogPosition::Bottom),
            ("left", DialogPosition::Left),
            ("right", DialogPosition::Right),
            ("topleft", DialogPosition::TopLeft),
            ("topright", DialogPosition::TopRight),
            ("bottomleft", DialogPosition::BottomLeft),
            ("bottomright", DialogPosition::BottomRight),
            ("nonsense", DialogPosition::Center),
            ("", DialogPosition::Center),
        ];
        for (name, want) in table {
            assert_eq!(position_of(name), want, "{name}");
        }
    }

    #[test]
    fn label_and_class_tables() {
        assert_eq!(button_label(None, None, "Yes"), "Yes");
        assert_eq!(button_label(Some("Save"), Some("Props"), "Yes"), "Save");
        assert_eq!(button_label(None, Some("Props"), "Yes"), "Props");
        assert_eq!(button_label(Some(""), Some(""), "Yes"), "Yes");
        assert_eq!(button_class("p-confirmdialog-accept-button", None), "p-confirmdialog-accept-button");
        assert_eq!(button_class("a", Some("b")), "a b");
        assert_eq!(button_class("a", Some("")), "a");
        assert_eq!(root_class(Some("mine")), "p-confirmdialog mine");
        assert_eq!(root_class(None), "p-confirmdialog");
    }
}
