//! Чистая логика Password: оценка надёжности, шкала и фазы панели (по `password.ts` и `passwordstyle.ts` PrimeNG).

/// Уровень надёжности пароля.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PasswordStrength {
    /// Слабый.
    Weak,
    /// Средний.
    Medium,
    /// Сильный.
    Strong,
}

impl PasswordStrength {
    /// Имя уровня: часть класса `p-password-meter-<имя>` и значение `data-p`.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Weak => "weak",
            Self::Medium => "medium",
            Self::Strong => "strong",
        }
    }

    /// Класс метки шкалы.
    pub fn meter_class(self) -> &'static str {
        match self {
            Self::Weak => "p-password-meter-label p-password-meter-weak",
            Self::Medium => "p-password-meter-label p-password-meter-medium",
            Self::Strong => "p-password-meter-label p-password-meter-strong",
        }
    }

    /// Ширина метки шкалы.
    pub fn width(self) -> &'static str {
        match self {
            Self::Weak => "33.33%",
            Self::Medium => "66.66%",
            Self::Strong => "100%",
        }
    }
}

/// Оценка по умолчанию, равная регулярным выражениям PrimeNG `mediumRegex` и `strongRegex`:
/// пусто — `None`; от 8 символов и строчные, прописные, цифры — сильный; от 6 символов и два вида — средний; иначе слабый.
pub fn strength(value: &str) -> Option<PasswordStrength> {
    if value.is_empty() {
        return None;
    }
    let len = value.chars().count();
    let lower = value.chars().any(|c| c.is_ascii_lowercase());
    let upper = value.chars().any(|c| c.is_ascii_uppercase());
    let digit = value.chars().any(|c| c.is_ascii_digit());
    let kinds = usize::from(lower) + usize::from(upper) + usize::from(digit);
    Some(if len >= 8 && kinds == 3 {
        PasswordStrength::Strong
    } else if len >= 6 && kinds >= 2 {
        PasswordStrength::Medium
    } else {
        PasswordStrength::Weak
    })
}

/// Фаза показа панели.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    /// Не показана.
    Hidden,
    /// Появляется (анимация входа).
    Entering,
    /// Показана.
    Shown,
    /// Исчезает (анимация выхода).
    Leaving,
}

impl Phase {
    /// Класс анимации (блок `/* For PrimeNG */` темы).
    pub fn animation_class(self) -> &'static str {
        match self {
            Self::Entering => "p-password-enter",
            Self::Leaving => "p-password-leave",
            Self::Hidden | Self::Shown => "",
        }
    }

    /// Панель есть в DOM.
    pub fn is_visible(self) -> bool {
        matches!(self, Self::Entering | Self::Shown)
    }
}

/// Снимок свойств, влияющих на классы корня.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RootState {
    /// Значение не пусто.
    pub filled: bool,
    /// Поле в фокусе.
    pub focused: bool,
    /// Вариант `filled`.
    pub variant_filled: bool,
    /// На всю ширину.
    pub fluid: bool,
}

/// Классы корня в порядке `passwordstyle.ts`.
pub fn root_classes(s: RootState) -> String {
    let mut out = String::from("p-password p-component p-inputwrapper");
    for (on, class) in [
        (s.filled, "p-inputwrapper-filled"),
        (s.variant_filled, "p-variant-filled"),
        (s.focused, "p-inputwrapper-focus"),
        (s.fluid, "p-password-fluid"),
    ] {
        if on {
            out.push(' ');
            out.push_str(class);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::PasswordStrength::{Medium, Strong, Weak};
    use super::*;

    #[test]
    fn strength_table() {
        let cases = [
            ("", None),
            ("a", Some(Weak)),
            ("12345678", Some(Weak)),
            ("abcdefgh", Some(Weak)),
            ("ABCDEFGH", Some(Weak)),
            ("aB1", Some(Weak)),
            ("aB123", Some(Weak)),
            ("abcDE", Some(Weak)),
            ("abcDEF", Some(Medium)),
            ("abc123", Some(Medium)),
            ("ABC123", Some(Medium)),
            ("abcDEF1", Some(Medium)),
            ("abcDEFG1", Some(Strong)),
            ("abcDEF12", Some(Strong)),
            ("abcdef12345", Some(Medium)),
            ("пароль12", Some(Weak)),
            ("abcDEF1!", Some(Strong)),
        ];
        for (value, expected) in cases {
            assert_eq!(strength(value), expected, "{value:?}");
        }
    }

    #[test]
    fn strength_counts_characters_not_bytes() {
        // «яяя» — 3 символа (6 байт): длина считается в символах, как у регулярного выражения.
        assert_eq!(strength("яяяaB1"), Some(Medium));
    }

    #[test]
    fn meter_table() {
        let cases = [
            (Weak, "weak", "33.33%", "p-password-meter-label p-password-meter-weak"),
            (Medium, "medium", "66.66%", "p-password-meter-label p-password-meter-medium"),
            (Strong, "strong", "100%", "p-password-meter-label p-password-meter-strong"),
        ];
        for (level, name, width, class) in cases {
            assert_eq!((level.as_str(), level.width(), level.meter_class()), (name, width, class));
        }
    }

    #[test]
    fn phase_table() {
        assert_eq!(Phase::Hidden.animation_class(), "");
        assert_eq!(Phase::Entering.animation_class(), "p-password-enter");
        assert_eq!(Phase::Shown.animation_class(), "");
        assert_eq!(Phase::Leaving.animation_class(), "p-password-leave");
        assert!(!Phase::Hidden.is_visible() && Phase::Entering.is_visible() && Phase::Shown.is_visible() && !Phase::Leaving.is_visible());
    }

    #[test]
    fn root_classes_table() {
        let st = |filled, focused, variant_filled, fluid| RootState { filled, focused, variant_filled, fluid };
        let cases = [
            (RootState::default(), "p-password p-component p-inputwrapper"),
            (st(true, false, false, false), "p-password p-component p-inputwrapper p-inputwrapper-filled"),
            (st(false, true, false, false), "p-password p-component p-inputwrapper p-inputwrapper-focus"),
            (st(false, false, true, false), "p-password p-component p-inputwrapper p-variant-filled"),
            (st(false, false, false, true), "p-password p-component p-inputwrapper p-password-fluid"),
            (
                st(true, true, true, true),
                "p-password p-component p-inputwrapper p-inputwrapper-filled p-variant-filled p-inputwrapper-focus p-password-fluid",
            ),
        ];
        for (s, class) in cases {
            assert_eq!(root_classes(s), class, "{s:?}");
        }
    }
}
