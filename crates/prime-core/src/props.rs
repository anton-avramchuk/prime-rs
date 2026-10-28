//! Общие значения пропсов полей ввода (как `BaseInput` в PrimeNG).

use crate::config::PrimeConfig;

/// Вариант оформления поля ввода.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Variant {
    /// Поле с рамкой (по умолчанию в теме).
    Outlined,
    /// Поле с заливкой.
    Filled,
}

impl Variant {
    /// Строковое значение, как в PrimeNG.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Outlined => "outlined",
            Self::Filled => "filled",
        }
    }

    /// Итоговый вариант: явный пропс, иначе значение из конфигурации (`None` — тема по умолчанию).
    pub fn resolve(explicit: Option<Self>, config: &PrimeConfig) -> Option<Self> {
        explicit.or(config.input_variant)
    }
}

/// Размер компонента.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Size {
    /// Маленький (`small`).
    Small,
    /// Большой (`large`).
    Large,
}

impl Size {
    /// Строковое значение, как в PrimeNG (`small`, `large`).
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Large => "large",
        }
    }

    /// Суффикс класса: `sm` или `lg` (например, `p-inputtext-sm`).
    pub fn class_suffix(self) -> &'static str {
        match self {
            Self::Small => "sm",
            Self::Large => "lg",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn values_match_primeng() {
        assert_eq!(Variant::Filled.as_str(), "filled");
        assert_eq!(Variant::Outlined.as_str(), "outlined");
        assert_eq!(Size::Small.as_str(), "small");
        assert_eq!(Size::Large.class_suffix(), "lg");
    }

    #[test]
    fn explicit_variant_wins_over_config() {
        let cfg = PrimeConfig { input_variant: Some(Variant::Filled), ..Default::default() };
        assert_eq!(Variant::resolve(None, &cfg), Some(Variant::Filled));
        assert_eq!(Variant::resolve(Some(Variant::Outlined), &cfg), Some(Variant::Outlined));
        assert_eq!(Variant::resolve(None, &PrimeConfig::default()), None);
    }
}
