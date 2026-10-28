//! Статический CSS темы Aura, полученный из токенов и стилей PrimeNG командой `cargo xtask gen-theme`.
//!
//! Базовый CSS (переменные `--p-*`, палитры, глобальные стили) подключается всегда, CSS компонентов — только
//! для включённых feature-флагов, поэтому приложение не загружает стили неиспользуемых компонентов.
//!
//! ```
//! let css = prime_theme::stylesheet(&[]);
//! assert!(css.contains("--p-primary-color"));
//! ```

mod generated;

/// Базовый CSS: переменные пресета, палитры и глобальные стили.
pub const BASE: &str = include_str!("../css/base.css");

/// Селектор тёмного режима, с которым сгенерирован CSS (`dark_mode_selector` в `upstream.toml`).
pub const DARK_MODE_SELECTOR: &str = ".p-dark";

/// CSS компонента по имени каталога PrimeNG (`button`); `None`, если его feature не включён.
pub fn component(name: &str) -> Option<&'static str> {
    generated::COMPONENTS.iter().find(|(n, _)| *n == name).map(|(_, css)| *css)
}

/// Базовый CSS и CSS перечисленных компонентов одной строкой; неизвестные и не включённые имена пропускаются.
pub fn stylesheet(components: &[&str]) -> String {
    let mut out = String::from(BASE);
    for name in components {
        if let Some(css) = component(name) {
            out.push_str(css);
        }
    }
    out
}

#[allow(unused_imports)]
pub use generated::*;
