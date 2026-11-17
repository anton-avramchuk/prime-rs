//! Global configuration (analogous to the `PrimeNG` config service), delivered via a Leptos context.

use crate::props::Variant;
use crate::translation_defaults::DEFAULTS;
use crate::zindex::ZGroup;
use leptos::prelude::*;
use std::collections::BTreeMap;

/// Base z-index of overlay groups (defaults as in PrimeNG).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ZIndexBases {
    /// Modal windows.
    pub modal: i32,
    /// Overlays.
    pub overlay: i32,
    /// Menus.
    pub menu: i32,
    /// Tooltips.
    pub tooltip: i32,
}

impl Default for ZIndexBases {
    fn default() -> Self {
        Self { modal: 1100, overlay: 1000, menu: 1000, tooltip: 1100 }
    }
}

impl ZIndexBases {
    /// Base value for the group.
    pub fn base(&self, group: ZGroup) -> i32 {
        match group {
            ZGroup::Modal => self.modal,
            ZGroup::Overlay => self.overlay,
            ZGroup::Menu => self.menu,
            ZGroup::Tooltip => self.tooltip,
        }
    }
}

/// Localization: overrides on top of the default English strings.
///
/// Keys match the keys of `PrimeNG.translation` (`accept`, `aria.close`, …).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Locale {
    overrides: BTreeMap<String, String>,
}

impl Locale {
    /// Overrides a string.
    pub fn with(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.overrides.insert(key.into(), value.into());
        self
    }

    /// Returns the translation string or `None` for an unknown key.
    pub fn get(&self, key: &str) -> Option<&str> {
        if let Some(v) = self.overrides.get(key) {
            return Some(v);
        }
        DEFAULTS.binary_search_by_key(&key, |(k, _)| k).ok().map(|i| DEFAULTS[i].1)
    }

    /// Translation string; for an unknown key an empty string.
    pub fn text(&self, key: &str) -> &str {
        self.get(key).unwrap_or("")
    }
}

/// Library configuration.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PrimeConfig {
    /// Ripple effect on buttons and fields (off by default, as in PrimeNG).
    pub ripple: bool,
    /// Default input field variant (`None` means as the theme sets, usually outlined).
    pub input_variant: Option<Variant>,
    /// Base z-index of overlays.
    pub z_index: ZIndexBases,
    /// Dark mode selector (as `darkModeSelector` in PrimeNG).
    pub dark_mode_selector: String,
    /// Localization.
    pub locale: Locale,
}

impl Default for PrimeConfig {
    fn default() -> Self {
        Self {
            ripple: false,
            input_variant: None,
            z_index: ZIndexBases::default(),
            dark_mode_selector: ".p-dark".into(),
            locale: Locale::default(),
        }
    }
}

/// Makes the configuration available to all components lower in the tree.
///
/// Called inside a provider component; outside a provider [`use_prime_config`] returns the defaults.
///
/// ```
/// use leptos::prelude::*;
/// use prime_core::{PrimeConfig, provide_prime_config, use_prime_config};
///
/// Owner::new().with(|| {
///     provide_prime_config(PrimeConfig { ripple: true, ..Default::default() });
///     assert!(use_prime_config().ripple);
/// });
/// ```
pub fn provide_prime_config(config: PrimeConfig) {
    provide_context(config);
}

/// Current configuration; outside a provider, the defaults.
pub fn use_prime_config() -> PrimeConfig {
    use_context::<PrimeConfig>().unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_match_primeng() {
        let c = PrimeConfig::default();
        assert!(!c.ripple);
        assert_eq!(c.z_index.base(ZGroup::Modal), 1100);
        assert_eq!(c.z_index.base(ZGroup::Overlay), 1000);
        assert_eq!(c.z_index.base(ZGroup::Menu), 1000);
        assert_eq!(c.z_index.base(ZGroup::Tooltip), 1100);
    }

    #[test]
    fn locale_defaults_and_overrides() {
        let l = Locale::default();
        assert_eq!(l.get("accept"), Some("Yes"));
        assert_eq!(l.get("aria.close"), Some("Close"));
        assert_eq!(l.get("nope"), None);
        let l = l.with("accept", "Agree");
        assert_eq!(l.text("accept"), "Agree");
        assert_eq!(l.text("reject"), "No");
    }

    #[test]
    fn defaults_table_is_sorted() {
        assert!(DEFAULTS.windows(2).all(|w| w[0].0 < w[1].0));
    }

    #[test]
    fn no_provider_gives_defaults() {
        let owner = Owner::new();
        owner.with(|| assert_eq!(use_prime_config(), PrimeConfig::default()));
    }

    #[test]
    fn provider_overrides() {
        let owner = Owner::new();
        owner.with(|| {
            provide_prime_config(PrimeConfig { ripple: true, ..Default::default() });
            assert!(use_prime_config().ripple);
        });
    }
}
