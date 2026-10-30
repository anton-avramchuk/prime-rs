//! Чистая логика InputOtp: токены, решения по клавишам и вставке (по `inputotp.ts` PrimeNG).

/// Что делать с нажатой клавишей.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyAction {
    /// Оставить действие браузеру.
    Allow,
    /// Отменить действие браузера.
    Prevent,
    /// Отменить и перевести фокус на предыдущую ячейку.
    Prev,
    /// Отменить и перевести фокус на следующую ячейку.
    Next,
}

/// Куда переводить фокус после `input`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Move {
    /// На предыдущую ячейку.
    Prev,
    /// На следующую ячейку.
    Next,
    /// Оставить фокус.
    None,
}

/// Символы значения по порядку, не более `length`.
pub fn tokens_of(value: &str, length: usize) -> Vec<String> {
    value.chars().take(length).map(String::from).collect()
}

/// Склеивает токены (пустые пропускаются).
pub fn join(tokens: &[String]) -> String {
    tokens.concat()
}

/// Решение по `keydown`. `ctrl_like` — нажат `Alt`, `Ctrl` или `Meta`.
pub fn key_decision(
    key: &str,
    ctrl_like: bool,
    integer_only: bool,
    has_selection: bool,
    filled: usize,
    length: usize,
    cell_empty: bool,
) -> KeyAction {
    if ctrl_like {
        return KeyAction::Allow;
    }
    match key {
        "ArrowLeft" => KeyAction::Prev,
        "ArrowRight" => KeyAction::Next,
        "ArrowUp" | "ArrowDown" => KeyAction::Prevent,
        "Backspace" if cell_empty => KeyAction::Prev,
        // Управляющие клавиши (Tab, Enter, Delete, Home…) не ограничиваются: «символ» — ровно один знак.
        _ if key.chars().count() != 1 => KeyAction::Allow,
        _ if integer_only && !key.chars().all(|c| c.is_ascii_digit()) => KeyAction::Prevent,
        _ if filled >= length && !has_selection => KeyAction::Prevent,
        _ => KeyAction::Allow,
    }
}

/// Значение из вставленного текста: до `length` символов; `None`, если пусто или при `integer_only` есть не только цифры.
pub fn paste_value(text: &str, length: usize, integer_only: bool) -> Option<String> {
    if text.is_empty() || (integer_only && !text.chars().all(|c| c.is_ascii_digit())) {
        return None;
    }
    Some(text.chars().take(length).collect())
}

/// Переход фокуса по `InputEvent.inputType`.
pub fn input_move(input_type: &str) -> Move {
    match input_type {
        "deleteContentBackward" => Move::Prev,
        "insertText" | "deleteContentForward" => Move::Next,
        _ => Move::None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokens_and_join() {
        assert_eq!(tokens_of("12", 4), ["1", "2"]);
        assert_eq!(tokens_of("123456", 4), ["1", "2", "3", "4"]);
        assert_eq!(tokens_of("", 4), Vec::<String>::new());
        assert_eq!(tokens_of("я1", 4), ["я", "1"]);
        assert_eq!(join(&["1".into(), String::new(), "3".into()]), "13");
        assert_eq!(join(&[]), "");
    }

    #[test]
    fn key_decision_table() {
        use KeyAction::*;
        // (key, ctrl, int_only, selection, filled, length, cell_empty) -> action
        let cases = [
            (("ArrowLeft", false, false, false, 0, 4, true), Prev),
            (("ArrowRight", false, false, false, 0, 4, true), Next),
            (("ArrowUp", false, false, false, 0, 4, true), Prevent),
            (("ArrowDown", false, false, false, 0, 4, true), Prevent),
            (("Backspace", false, false, false, 1, 4, true), Prev),
            (("Backspace", false, false, false, 1, 4, false), Allow),
            (("a", true, true, false, 4, 4, false), Allow),
            (("1", false, true, false, 1, 4, true), Allow),
            (("a", false, true, false, 1, 4, true), Prevent),
            (("a", false, false, false, 1, 4, true), Allow),
            (("1", false, false, false, 4, 4, false), Prevent),
            (("1", false, false, true, 4, 4, false), Allow),
            (("Delete", false, false, false, 4, 4, false), Allow),
            (("Tab", false, true, false, 4, 4, false), Allow),
            (("Enter", false, true, false, 0, 4, true), Allow),
        ];
        for ((key, ctrl, int, sel, filled, len, empty), want) in cases {
            assert_eq!(
                key_decision(key, ctrl, int, sel, filled, len, empty),
                want,
                "{key} ctrl={ctrl} int={int} sel={sel} filled={filled}"
            );
        }
    }

    #[test]
    fn paste_table() {
        assert_eq!(paste_value("123456", 4, false).as_deref(), Some("1234"));
        assert_eq!(paste_value("12", 4, true).as_deref(), Some("12"));
        assert_eq!(paste_value("12ab", 4, true), None);
        assert_eq!(paste_value("12ab", 4, false).as_deref(), Some("12ab"));
        assert_eq!(paste_value("", 4, false), None);
    }

    #[test]
    fn input_move_table() {
        assert_eq!(input_move("deleteContentBackward"), Move::Prev);
        assert_eq!(input_move("insertText"), Move::Next);
        assert_eq!(input_move("deleteContentForward"), Move::Next);
        assert_eq!(input_move("insertFromPaste"), Move::None);
        assert_eq!(input_move(""), Move::None);
    }
}
