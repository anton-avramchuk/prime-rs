//! Глобальная конфигурация (аналог `PrimeNG` config service), доставляемая через Leptos context.

use crate::props::Variant;
use crate::translation_defaults::DEFAULTS;
use crate::zindex::ZGroup;
use leptos::prelude::*;
use std::collections::BTreeMap;

/// Базовые z-index групп оверлеев (значения по умолчанию, как в PrimeNG).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ZIndexBases {
    /// Модальные окна.
    pub modal: i32,
    /// Оверлеи.
    pub overlay: i32,
    /// Меню.
    pub menu: i32,
    /// Подсказки.
    pub tooltip: i32,
}

impl Default for ZIndexBases {
    fn default() -> Self {
        Self { modal: 1100, overlay: 1000, menu: 1000, tooltip: 1100 }
    }
}

impl ZIndexBases {
    /// Базовое значение для группы.
    pub fn base(&self, group: ZGroup) -> i32 {
        match group {
            ZGroup::Modal => self.modal,
            ZGroup::Overlay => self.overlay,
            ZGroup::Menu => self.menu,
            ZGroup::Tooltip => self.tooltip,
        }
    }
}

/// Локализация: переопределения поверх английских строк по умолчанию.
///
/// Ключи совпадают с ключами `PrimeNG.translation` (`accept`, `aria.close`, …).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Locale {
    overrides: BTreeMap<String, String>,
}

impl Locale {
    /// Переопределяет строку.
    pub fn with(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.overrides.insert(key.into(), value.into());
        self
    }

    /// Возвращает строку перевода или `None` для неизвестного ключа.
    pub fn get(&self, key: &str) -> Option<&str> {
        if let Some(v) = self.overrides.get(key) {
            return Some(v);
        }
        DEFAULTS.binary_search_by_key(&key, |(k, _)| k).ok().map(|i| DEFAULTS[i].1)
    }

    /// Строка перевода; для неизвестного ключа — пустая строка.
    pub fn text(&self, key: &str) -> &str {
        self.get(key).unwrap_or("")
    }
}

/// Конфигурация библиотеки.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PrimeConfig {
    /// Эффект ripple у кнопок и полей (по умолчанию выключен, как в PrimeNG).
    pub ripple: bool,
    /// Вариант полей ввода по умолчанию (`None` — как задаёт тема, обычно outlined).
    pub input_variant: Option<Variant>,
    /// Базовые z-index оверлеев.
    pub z_index: ZIndexBases,
    /// Селектор тёмного режима (как `darkModeSelector` в PrimeNG).
    pub dark_mode_selector: String,
    /// Локализация.
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

/// Делает конфигурацию доступной всем компонентам ниже по дереву.
///
/// Вызывается внутри компонента-провайдера; вне провайдера [`use_prime_config`] вернёт значения по умолчанию.
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

/// Текущая конфигурация; вне провайдера — значения по умолчанию.
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
        let l = l.with("accept", "Да");
        assert_eq!(l.text("accept"), "Да");
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
