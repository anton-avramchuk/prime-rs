//! Чтение `upstream.toml`: закреплённая версия PrimeNG.

use serde::Deserialize;
use std::path::{Path, PathBuf};

/// Содержимое `upstream.toml`.
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Upstream {
    pub version: String,
    pub commit: String,
    pub path: PathBuf,
    pub primeuix: Primeuix,
    pub theme: ThemeConfig,
}

/// Настройки генерации темы.
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct ThemeConfig {
    pub dark_mode_selector: String,
    pub components: Vec<String>,
}

/// Версии npm-пакетов `@primeuix/*`.
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Primeuix {
    pub themes: String,
    pub styles: String,
    pub styled: String,
    pub utils: String,
}

impl Upstream {
    /// Читает `upstream.toml` из корня репозитория; `PRIMENG_PATH` перекрывает `path`.
    pub fn load(root: &Path) -> Result<Self, String> {
        Self::load_with_env(root, std::env::var("PRIMENG_PATH").ok())
    }

    pub fn load_with_env(root: &Path, env_path: Option<String>) -> Result<Self, String> {
        let file = root.join("upstream.toml");
        let text = std::fs::read_to_string(&file).map_err(|e| format!("{}: {e}", file.display()))?;
        let mut upstream: Self = toml::from_str(&text).map_err(|e| format!("{}: {e}", file.display()))?;
        if let Some(p) = env_path.filter(|p| !p.is_empty()) {
            upstream.path = PathBuf::from(p);
        }
        Ok(upstream)
    }

    /// Каталог исходников компонентов PrimeNG.
    pub fn src_dir(&self) -> PathBuf {
        self.path.join("packages/primeng/src")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TOML: &str = r#"
version = "21.1.10"
commit = "fe82b291d6"
path = "C:/Projects/primeng"
[primeuix]
themes = "2.0.3"
styles = "2.0.3"
styled = "0.7.4"
utils = "0.7.2"
[theme]
dark_mode_selector = ".p-dark"
components = ["button"]
"#;

    fn root() -> tempfile::TempDir {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("upstream.toml"), TOML).unwrap();
        dir
    }

    #[test]
    fn reads_values() {
        let dir = root();
        let u = Upstream::load_with_env(dir.path(), None).unwrap();
        assert_eq!(u.version, "21.1.10");
        assert_eq!(u.commit, "fe82b291d6");
        assert_eq!(u.path, PathBuf::from("C:/Projects/primeng"));
        assert_eq!(u.primeuix.themes, "2.0.3");
        assert_eq!(u.theme.dark_mode_selector, ".p-dark");
        assert_eq!(u.theme.components, vec!["button"]);
    }

    #[test]
    fn env_overrides_path() {
        let dir = root();
        let u = Upstream::load_with_env(dir.path(), Some("/tmp/other".into())).unwrap();
        assert_eq!(u.path, PathBuf::from("/tmp/other"));
    }

    #[test]
    fn missing_file_is_error() {
        let dir = tempfile::tempdir().unwrap();
        assert!(Upstream::load_with_env(dir.path(), None).is_err());
    }
}
