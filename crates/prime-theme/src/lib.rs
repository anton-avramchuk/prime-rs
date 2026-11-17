//! Static CSS of the Aura theme, produced from PrimeNG tokens and styles by `cargo xtask gen-theme`.
//!
//! The base CSS (`--p-*` variables, palettes, global styles) is always included; component CSS only
//! for enabled feature flags, so an application does not load styles of unused components.
//!
//! ```
//! let css = prime_theme::stylesheet(&[]);
//! assert!(css.contains("--p-primary-color"));
//! ```

mod generated;

/// Base CSS: preset variables, palettes and global styles.
pub const BASE: &str = include_str!("../css/base.css");

/// Dark mode selector the CSS was generated with (`dark_mode_selector` in `upstream.toml`).
pub const DARK_MODE_SELECTOR: &str = ".p-dark";

/// Component CSS by PrimeNG directory name (`button`); `None` if its feature is not enabled.
pub fn component(name: &str) -> Option<&'static str> {
    generated::COMPONENTS.iter().find(|(n, _)| *n == name).map(|(_, css)| *css)
}

/// The base CSS and the CSS of the listed components as one string; unknown and not enabled names are skipped.
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
