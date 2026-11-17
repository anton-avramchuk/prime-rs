//! Shared input prop values (like `BaseInput` in PrimeNG).

use crate::config::PrimeConfig;

/// Input field styling variant.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Variant {
    /// Field with a border (the theme default).
    Outlined,
    /// Field with a fill.
    Filled,
}

impl Variant {
    /// String value, as in PrimeNG.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Outlined => "outlined",
            Self::Filled => "filled",
        }
    }

    /// Resulting variant: the explicit prop, otherwise the value from the configuration (`None` means the theme default).
    pub fn resolve(explicit: Option<Self>, config: &PrimeConfig) -> Option<Self> {
        explicit.or(config.input_variant)
    }
}

/// Component size.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Size {
    /// Small (`small`).
    Small,
    /// Large (`large`).
    Large,
}

impl Size {
    /// String value, as in PrimeNG (`small`, `large`).
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Large => "large",
        }
    }

    /// Class suffix: `sm` or `lg` (for example, `p-inputtext-sm`).
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
