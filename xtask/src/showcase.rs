//! Static showcase build (GitHub Pages): pre-rendered pages that hydrate with a split wasm.
//!
//! `cargo leptos build --release --split` (with the showcase `split` feature) builds the server and a wasm split per
//! lazy route (each component page is a `#[lazy_route]`, see `lazy_page!` in `showcase/src/app.rs`), so the browser
//! loads the shared code plus the chunk of the open page.
//! The server binary then renders every route to HTML (`showcase --export`), so the page is readable before the wasm
//! arrives; hydration only attaches the behavior.

use std::path::Path;
use std::process::Command;

/// Size settings of the static build: the code is optimized for size. Applies to the server too, which only renders
/// the pages once.
pub const SIZE_ENV: &[(&str, &str)] = &[("CARGO_PROFILE_RELEASE_OPT_LEVEL", "z")];

/// Builds the static showcase into `dist`, served under `base` (e.g. `/prime-rs`, empty for the domain root):
/// `index.html`, `<route>.html` per page (GitHub Pages serves `/button` from `button.html`), `404.html` and the
/// `pkg` directory with the wasm and its chunks.
pub fn pages(root: &Path, base: &str, dist: &Path) -> Result<(), String> {
    let base = base.trim_end_matches('/');
    run(
        Command::new("cargo").current_dir(root.join("showcase")).envs(SIZE_ENV.iter().copied()).env("SHOWCASE_BASE", base).args([
            "leptos",
            "build",
            "--release",
            "--split",
            "--lib-features",
            "hydrate,split",
        ]),
        "cargo leptos build (install it with `cargo install cargo-leptos`)",
    )?;

    if dist.exists() {
        std::fs::remove_dir_all(dist).map_err(|e| format!("{}: {e}", dist.display()))?;
    }
    let site = root.join("target/site");
    let server = root.join("target/release").join(format!("showcase{}", std::env::consts::EXE_SUFFIX));
    run(
        Command::new(server)
            .current_dir(root)
            .env("LEPTOS_OUTPUT_NAME", "showcase")
            .env("LEPTOS_SITE_ROOT", &site)
            .env("LEPTOS_SITE_PKG_DIR", "pkg")
            .env("LEPTOS_ENV", "PROD")
            .arg("--export")
            .arg(dist),
        "showcase --export",
    )?;
    copy_dir(&site.join("pkg"), &dist.join("pkg"))
}

fn run(command: &mut Command, what: &str) -> Result<(), String> {
    let status = command.status().map_err(|e| format!("{what}: {e}"))?;
    if status.success() { Ok(()) } else { Err(format!("{what} failed: {status}")) }
}

fn copy_dir(from: &Path, to: &Path) -> Result<(), String> {
    std::fs::create_dir_all(to).map_err(|e| format!("{}: {e}", to.display()))?;
    for entry in std::fs::read_dir(from).map_err(|e| format!("{}: {e}", from.display()))? {
        let entry = entry.map_err(|e| e.to_string())?;
        let target = to.join(entry.file_name());
        if entry.file_type().map_err(|e| e.to_string())?.is_dir() {
            copy_dir(&entry.path(), &target)?;
        } else {
            std::fs::copy(entry.path(), &target).map_err(|e| format!("{}: {e}", target.display()))?;
        }
    }
    Ok(())
}
