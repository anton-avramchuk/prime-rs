//! `affected`: какие крейты затронуты изменениями (для выборочных проверок в CI).
//!
//! Правила:
//! - изменение общего ядра (`prime-core`, `prime-dom`), зависимостей (`Cargo.toml`, `Cargo.lock`), тулчейна, `xtask`
//!   или конфигурации CI/сборки означает полный прогон всего workspace;
//! - иначе — изменённые крейты и все крейты, которые от них зависят;
//! - изменения вне крейтов (документация, `openspec/`) ничего не затрагивают.

use serde::Deserialize;
use std::collections::BTreeSet;
use std::path::Path;
use std::process::Command;

/// Крейт workspace.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pkg {
    pub name: String,
    /// Каталог относительно корня, с `/` (`crates/prime-button`).
    pub dir: String,
    /// Имена workspace-крейтов, от которых он зависит.
    pub deps: Vec<String>,
}

/// Результат.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Affected {
    /// Проверять весь workspace.
    All,
    /// Проверять только эти крейты (может быть пусто).
    Packages(Vec<String>),
}

/// Крейты общего ядра: их изменение затрагивает всех.
const CORE: &[&str] = &["prime-core", "prime-dom"];
/// Файлы и каталоги, изменение которых требует полного прогона.
const GLOBAL: &[&str] = &[
    "Cargo.toml",
    "Cargo.lock",
    "rust-toolchain.toml",
    "rustfmt.toml",
    "deny.toml",
    ".cargo/",
    ".github/",
    "xtask/",
    "benches/budgets.toml",
];

fn is_global(file: &str) -> bool {
    GLOBAL.iter().any(|g| file == *g || (g.ends_with('/') && file.starts_with(g)))
}

/// Определяет затронутые крейты по списку изменённых файлов (пути относительно корня, с `/`).
pub fn affected(pkgs: &[Pkg], changed: &[String]) -> Affected {
    if changed.iter().any(|f| is_global(f)) {
        return Affected::All;
    }
    let mut set: BTreeSet<&str> =
        changed.iter().filter_map(|f| pkgs.iter().find(|p| f.starts_with(&format!("{}/", p.dir)))).map(|p| p.name.as_str()).collect();
    if set.iter().any(|n| CORE.contains(n)) {
        return Affected::All;
    }
    loop {
        let before = set.len();
        for p in pkgs {
            if p.deps.iter().any(|d| set.contains(d.as_str())) {
                set.insert(&p.name);
            }
        }
        if set.len() == before {
            break;
        }
    }
    Affected::Packages(set.into_iter().map(str::to_owned).collect())
}

#[derive(Deserialize)]
struct Metadata {
    packages: Vec<MetaPkg>,
    workspace_root: String,
}

#[derive(Deserialize)]
struct MetaPkg {
    name: String,
    manifest_path: String,
    dependencies: Vec<MetaDep>,
}

#[derive(Deserialize)]
struct MetaDep {
    name: String,
}

/// Читает крейты workspace через `cargo metadata`.
pub fn read_packages(root: &Path) -> Result<Vec<Pkg>, String> {
    let out = Command::new("cargo")
        .current_dir(root)
        .args(["metadata", "--format-version", "1", "--no-deps"])
        .output()
        .map_err(|e| format!("cargo metadata: {e}"))?;
    if !out.status.success() {
        return Err(String::from_utf8_lossy(&out.stderr).into_owned());
    }
    let meta: Metadata = serde_json::from_slice(&out.stdout).map_err(|e| e.to_string())?;
    let names: BTreeSet<&str> = meta.packages.iter().map(|p| p.name.as_str()).collect();
    let ws = meta.workspace_root.replace('\\', "/");
    Ok(meta
        .packages
        .iter()
        .map(|p| {
            let manifest = p.manifest_path.replace('\\', "/");
            let dir = manifest.strip_prefix(&format!("{ws}/")).and_then(|m| m.strip_suffix("/Cargo.toml")).unwrap_or_default().to_owned();
            Pkg {
                name: p.name.clone(),
                dir,
                deps: p.dependencies.iter().filter(|d| names.contains(d.name.as_str())).map(|d| d.name.clone()).collect(),
            }
        })
        .collect())
}

/// Изменённые файлы относительно `base` (`git diff --name-only base...HEAD`).
pub fn changed_files(root: &Path, base: &str) -> Result<Vec<String>, String> {
    let out = Command::new("git")
        .current_dir(root)
        .args(["diff", "--name-only", &format!("{base}...HEAD")])
        .output()
        .map_err(|e| format!("git: {e}"))?;
    if !out.status.success() {
        return Err(String::from_utf8_lossy(&out.stderr).trim().to_owned());
    }
    Ok(String::from_utf8_lossy(&out.stdout).lines().map(str::to_owned).collect())
}

/// Крейты-приложения: у них свои проверки (job `apps`), в `-p`-список библиотек они не попадают.
const APPS: &[&str] = &["showcase", "wasm-size", "render-bench", "xtask"];

/// Строки для `$GITHUB_OUTPUT`: `all` (полный прогон), `any` (есть затронутые библиотеки), `args`
/// (`--workspace --exclude …` или `-p a -p b` только для библиотек), `apps` (затронуты приложения).
pub fn render_outputs(result: &Affected) -> String {
    let exclude = APPS.iter().map(|a| format!("--exclude {a}")).collect::<Vec<_>>().join(" ");
    match result {
        Affected::All => format!(
            "all=true
any=true
args=--workspace {exclude}
apps=true
"
        ),
        Affected::Packages(p) => {
            let libs: Vec<&String> = p.iter().filter(|n| !APPS.contains(&n.as_str())).collect();
            let apps = p.iter().any(|n| APPS.contains(&n.as_str()));
            let args = libs.iter().map(|n| format!("-p {n}")).collect::<Vec<_>>().join(" ");
            format!(
                "all=false
any={}
args={args}
apps={apps}
",
                !libs.is_empty()
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pkg(name: &str, dir: &str, deps: &[&str]) -> Pkg {
        Pkg { name: name.into(), dir: dir.into(), deps: deps.iter().map(|d| (*d).into()).collect() }
    }

    fn workspace() -> Vec<Pkg> {
        vec![
            pkg("prime-core", "crates/prime-core", &[]),
            pkg("prime-dom", "crates/prime-dom", &[]),
            pkg("prime-icons", "crates/prime-icons", &["prime-core"]),
            pkg("prime-button", "crates/prime-button", &["prime-core", "prime-dom", "prime-icons"]),
            pkg("prime-badge", "crates/prime-badge", &["prime-core"]),
            pkg("prime-chip", "crates/prime-chip", &["prime-core", "prime-icons"]),
            pkg("showcase", "showcase", &["prime-button"]),
        ]
    }

    fn files(f: &[&str]) -> Vec<String> {
        f.iter().map(|s| (*s).into()).collect()
    }

    #[test]
    fn one_component_checks_it_and_its_dependents_only() {
        let r = affected(&workspace(), &files(&["crates/prime-button/src/lib.rs"]));
        assert_eq!(r, Affected::Packages(vec!["prime-button".into(), "showcase".into()]));
    }

    #[test]
    fn leaf_without_dependents_is_alone() {
        let r = affected(&workspace(), &files(&["crates/prime-badge/src/lib.rs"]));
        assert_eq!(r, Affected::Packages(vec!["prime-badge".into()]));
    }

    #[test]
    fn dependents_are_transitive() {
        let r = affected(&workspace(), &files(&["crates/prime-icons/src/generated.rs"]));
        assert_eq!(r, Affected::Packages(vec!["prime-button".into(), "prime-chip".into(), "prime-icons".into(), "showcase".into()]));
    }

    #[test]
    fn core_change_checks_everything() {
        assert_eq!(affected(&workspace(), &files(&["crates/prime-core/src/lib.rs"])), Affected::All);
        assert_eq!(affected(&workspace(), &files(&["crates/prime-dom/src/focus.rs"])), Affected::All);
    }

    #[test]
    fn dependency_and_config_changes_check_everything() {
        for f in ["Cargo.lock", "Cargo.toml", "xtask/src/main.rs", ".github/workflows/ci.yml", ".cargo/config.toml", "rust-toolchain.toml"]
        {
            assert_eq!(affected(&workspace(), &files(&[f])), Affected::All, "{f}");
        }
    }

    #[test]
    fn docs_only_changes_affect_nothing() {
        assert_eq!(affected(&workspace(), &files(&["README.md", "docs/build.md", "openspec/specs/x/spec.md"])), Affected::Packages(vec![]));
    }

    #[test]
    fn outputs_format() {
        let exclude = "--exclude showcase --exclude wasm-size --exclude render-bench --exclude xtask";
        assert_eq!(
            render_outputs(&Affected::All),
            format!(
                "all=true
any=true
args=--workspace {exclude}
apps=true
"
            )
        );
        assert_eq!(
            render_outputs(&Affected::Packages(vec![])),
            "all=false
any=false
args=
apps=false
"
        );
        assert_eq!(
            render_outputs(&Affected::Packages(vec!["a".into(), "b".into()])),
            "all=false
any=true
args=-p a -p b
apps=false
"
        );
    }

    #[test]
    fn apps_are_split_from_libraries() {
        let r = Affected::Packages(vec!["prime-button".into(), "showcase".into()]);
        assert_eq!(
            render_outputs(&r),
            "all=false
any=true
args=-p prime-button
apps=true
"
        );
        let only_app = Affected::Packages(vec!["showcase".into()]);
        assert_eq!(
            render_outputs(&only_app),
            "all=false
any=false
args=
apps=true
"
        );
    }

    #[test]
    fn reads_this_workspace() {
        let pkgs = read_packages(Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap()).unwrap();
        let icons = pkgs.iter().find(|p| p.name == "prime-icons").unwrap();
        assert_eq!(icons.dir, "crates/prime-icons");
        assert!(icons.deps.contains(&"prime-core".to_owned()));
    }
}
