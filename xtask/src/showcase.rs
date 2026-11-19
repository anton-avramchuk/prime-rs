//! Static showcase build (GitHub Pages): the theme stylesheet as a file and the size-optimized trunk build.
//!
//! The client-only build links the CSS from `index.html` instead of carrying it inside the wasm binary, so the
//! browser downloads styles in parallel with the wasm and can paint the loading shell before the app starts.

use std::path::{Path, PathBuf};
use std::process::Command;

/// Theme stylesheet for the showcase: `base.css` followed by the CSS of each component listed in
/// `showcase/theme-components.txt`, in that order. Byte-identical to `prime_theme::stylesheet(list)`.
pub fn theme_css(root: &Path) -> Result<String, String> {
    let read = |p: PathBuf| std::fs::read_to_string(&p).map_err(|e| format!("{}: {e}", p.display()));
    let theme = root.join("crates/prime-theme/css");
    let list = read(root.join("showcase/theme-components.txt"))?;
    let mut out = read(theme.join("base.css"))?;
    for name in list.lines().map(str::trim).filter(|l| !l.is_empty()) {
        out.push_str(&read(theme.join("components").join(format!("{name}.css")))?);
    }
    Ok(out)
}

/// Where `showcase-css` writes the stylesheet; `showcase/index.html` links it from there.
pub fn css_path(root: &Path) -> PathBuf {
    root.join("target/showcase/theme.css")
}

/// Writes the theme stylesheet to [`css_path`], skipping the write when the content is unchanged (keeps trunk's
/// watcher quiet).
pub fn write_css(root: &Path) -> Result<(), String> {
    let css = theme_css(root)?;
    let path = css_path(root);
    if std::fs::read_to_string(&path).is_ok_and(|old| old == css) {
        return Ok(());
    }
    std::fs::create_dir_all(path.parent().expect("has a parent")).map_err(|e| e.to_string())?;
    std::fs::write(&path, css).map_err(|e| format!("{}: {e}", path.display()))
}

/// Size settings of the static build, applied to the wasm target only (build scripts and proc macros are not
/// affected): Leptos erases component view types into `AnyView` (far fewer generic instantiations), and the code is
/// optimized for size. Together they cut the gzipped wasm from about 6 MB to about 2.4 MB.
pub const SIZE_ENV: &[(&str, &str)] =
    &[("CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUSTFLAGS", "--cfg erase_components"), ("CARGO_PROFILE_RELEASE_OPT_LEVEL", "z")];

/// `trunk build --release` of the static showcase into `dist`, served under `base` (e.g. `/prime-rs`, empty for the
/// domain root). GitHub Pages serves `404.html` for unknown paths, so the SPA shell is copied there too.
pub fn pages(root: &Path, base: &str, dist: &Path) -> Result<(), String> {
    let base = base.trim_end_matches('/');
    let status = Command::new("trunk")
        .current_dir(root.join("showcase"))
        .envs(SIZE_ENV.iter().copied())
        .env("SHOWCASE_BASE", base)
        .args(["build", "--release", "--public-url"])
        .arg(format!("{base}/"))
        .arg("--dist")
        .arg(dist)
        .status()
        .map_err(|e| format!("trunk: {e} (install it with `cargo install trunk`)"))?;
    if !status.success() {
        return Err(format!("trunk build failed: {status}"));
    }
    std::fs::copy(dist.join("index.html"), dist.join("404.html")).map_err(|e| format!("404.html: {e}"))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn theme_css_matches_prime_theme_stylesheet() {
        let root = crate::root();
        let list = std::fs::read_to_string(root.join("showcase/theme-components.txt")).unwrap();
        let names: Vec<&str> = list.lines().filter(|l| !l.is_empty()).collect();
        assert!(names.len() > 50, "the showcase uses most components");
        let expected = prime_theme::stylesheet(&names);
        let actual = theme_css(&root).unwrap();
        assert!(actual == expected, "xtask theme.css differs from prime_theme::stylesheet ({} vs {} bytes)", actual.len(), expected.len());
    }
}
