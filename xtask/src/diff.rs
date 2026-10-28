//! `upstream-diff`: какие портированные компоненты изменились в PrimeNG.

use std::collections::BTreeSet;
use std::path::Path;
use std::process::Command;

/// Крейты, которые не являются портами компонентов.
const INFRA: &[&str] = &["core", "dom", "theme", "icons"];

/// Имена портированных компонентов: `crates/prime-<name>` без инфраструктурных крейтов.
pub fn ported_components(root: &Path) -> Vec<String> {
    let Ok(entries) = std::fs::read_dir(root.join("crates")) else {
        return Vec::new();
    };
    let mut names: Vec<String> = entries
        .flatten()
        .filter_map(|e| e.file_name().into_string().ok())
        .filter_map(|n| n.strip_prefix("prime-").map(str::to_owned))
        .filter(|n| !INFRA.contains(&n.as_str()))
        .collect();
    names.sort();
    names
}

/// Компоненты из `ported`, у которых между `from` и `to` менялись файлы в `packages/primeng/src/<name>/`.
pub fn changed_components(repo: &Path, from: &str, to: &str, ported: &[String]) -> Result<Vec<String>, String> {
    let out = Command::new("git")
        .current_dir(repo)
        .args(["diff", "--name-only", &format!("{from}..{to}"), "--", "packages/primeng/src"])
        .output()
        .map_err(|e| format!("git: {e}"))?;
    if !out.status.success() {
        return Err(String::from_utf8_lossy(&out.stderr).trim().to_owned());
    }
    let text = String::from_utf8_lossy(&out.stdout);
    let touched: BTreeSet<&str> = text
        .lines()
        .filter_map(|l| l.strip_prefix("packages/primeng/src/"))
        .filter_map(|l| l.split('/').next())
        .collect();
    Ok(ported.iter().filter(|p| touched.contains(p.as_str())).cloned().collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn git(dir: &Path, args: &[&str]) {
        let status = Command::new("git")
            .current_dir(dir)
            .args(["-c", "user.name=t", "-c", "user.email=t@t", "-c", "commit.gpgsign=false"])
            .args(args)
            .status()
            .unwrap();
        assert!(status.success(), "git {args:?}");
    }

    fn write(dir: &Path, rel: &str, text: &str) {
        let p = dir.join(rel);
        std::fs::create_dir_all(p.parent().unwrap()).unwrap();
        std::fs::write(p, text).unwrap();
    }

    fn repo() -> tempfile::TempDir {
        let dir = tempfile::tempdir().unwrap();
        git(dir.path(), &["init", "-q"]);
        write(dir.path(), "packages/primeng/src/button/button.ts", "a");
        write(dir.path(), "packages/primeng/src/badge/badge.ts", "a");
        git(dir.path(), &["add", "."]);
        git(dir.path(), &["commit", "-q", "-m", "one"]);
        write(dir.path(), "packages/primeng/src/button/button.ts", "b");
        write(dir.path(), "packages/primeng/src/card/card.ts", "a");
        git(dir.path(), &["add", "."]);
        git(dir.path(), &["commit", "-q", "-m", "two"]);
        dir
    }

    #[test]
    fn reports_changed_ported_component() {
        let dir = repo();
        let ported = vec!["badge".to_owned(), "button".to_owned()];
        let changed = changed_components(dir.path(), "HEAD~1", "HEAD", &ported).unwrap();
        assert_eq!(changed, vec!["button".to_owned()]);
    }

    #[test]
    fn reports_nothing_when_unchanged() {
        let dir = repo();
        let ported = vec!["badge".to_owned()];
        assert!(changed_components(dir.path(), "HEAD~1", "HEAD", &ported).unwrap().is_empty());
    }

    #[test]
    fn bad_revision_is_error() {
        let dir = repo();
        assert!(changed_components(dir.path(), "nope", "HEAD", &[]).is_err());
    }

    #[test]
    fn ported_components_skip_infra() {
        let dir = tempfile::tempdir().unwrap();
        for n in ["prime-core", "prime-button", "prime-icons", "prime-badge"] {
            std::fs::create_dir_all(dir.path().join("crates").join(n)).unwrap();
        }
        assert_eq!(ported_components(dir.path()), vec!["badge".to_owned(), "button".to_owned()]);
    }
}
