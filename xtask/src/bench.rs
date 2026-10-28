//! `wasm-size` (бюджеты размера wasm) и `bench-compare` (сравнение отчётов с базовой линией).

use serde::Deserialize;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Один случай измерения из `benches/budgets.toml`.
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Case {
    pub name: String,
    pub feature: String,
    pub max_delta_bytes: u64,
}

#[derive(Debug, Deserialize)]
struct Budgets {
    case: Vec<Case>,
}

/// Читает бюджеты.
pub fn read_budgets(path: &Path) -> Result<Vec<Case>, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    toml::from_str::<Budgets>(&text).map(|b| b.case).map_err(|e| format!("{}: {e}", path.display()))
}

/// Сообщения о превышении бюджета: «компонент, размер, бюджет».
pub fn check_budgets(baseline: u64, measured: &[(String, u64)], cases: &[Case]) -> Vec<String> {
    cases
        .iter()
        .filter_map(|c| {
            let size = measured.iter().find(|(n, _)| *n == c.name)?.1;
            let delta = size.saturating_sub(baseline);
            (delta > c.max_delta_bytes).then(|| format!("{}: вклад {delta} Б превышает бюджет {} Б", c.name, c.max_delta_bytes))
        })
        .collect()
}

fn target_dir(root: &Path) -> PathBuf {
    std::env::var_os("CARGO_TARGET_DIR").map(PathBuf::from).unwrap_or_else(|| root.join("target"))
}

fn build_size(root: &Path, feature: &str) -> Result<u64, String> {
    let status = Command::new("cargo")
        .current_dir(root)
        .args([
            "build",
            "-p",
            "wasm-size",
            "--release",
            "--target",
            "wasm32-unknown-unknown",
            "--no-default-features",
            "--features",
            feature,
        ])
        .status()
        .map_err(|e| format!("cargo build: {e}"))?;
    if !status.success() {
        return Err(format!("сборка wasm-size с {feature} не удалась"));
    }
    let file = target_dir(root).join("wasm32-unknown-unknown/release/wasm_size.wasm");
    std::fs::metadata(&file).map(|m| m.len()).map_err(|e| format!("{}: {e}", file.display()))
}

/// Измеряет вклад каждого случая, пишет `target/bench/wasm.json` и возвращает ошибку при превышении бюджета.
pub fn wasm_size(root: &Path, budgets: &Path) -> Result<(), String> {
    let cases = read_budgets(budgets)?;
    let baseline = build_size(root, "case-baseline")?;
    println!("baseline: {baseline} Б");
    let mut measured = Vec::new();
    let mut metrics = BTreeMap::new();
    metrics.insert("wasm.baseline.bytes".to_owned(), baseline as f64);
    for c in &cases {
        let size = build_size(root, &c.feature)?;
        let delta = size.saturating_sub(baseline);
        println!("{:<12} {size} Б, вклад {delta} Б (бюджет {} Б)", c.name, c.max_delta_bytes);
        metrics.insert(format!("wasm.{}.delta_bytes", c.name), delta as f64);
        measured.push((c.name.clone(), size));
    }
    let dir = target_dir(root).join("bench");
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let json = serde_json::to_string_pretty(&serde_json::json!({ "metrics": metrics })).map_err(|e| e.to_string())?;
    std::fs::write(dir.join("wasm.json"), json).map_err(|e| e.to_string())?;
    let violations = check_budgets(baseline, &measured, &cases);
    if violations.is_empty() {
        Ok(())
    } else {
        Err(format!("превышен бюджет размера wasm:\n  {}", violations.join("\n  ")))
    }
}

/// Строка сравнения метрики.
#[derive(Debug, PartialEq)]
pub struct Row {
    pub name: String,
    pub base: f64,
    pub cur: f64,
    pub pct: f64,
}

/// Сравнивает метрики (меньше — лучше); возвращает общие метрики по имени.
pub fn compare(base: &BTreeMap<String, f64>, cur: &BTreeMap<String, f64>) -> Vec<Row> {
    cur.iter()
        .filter_map(|(name, &c)| {
            let &b = base.get(name)?;
            let pct = if b == 0.0 { if c == 0.0 { 0.0 } else { f64::INFINITY } } else { (c - b) / b * 100.0 };
            Some(Row { name: name.clone(), base: b, cur: c, pct })
        })
        .collect()
}

/// Markdown-таблица для комментария в PR; регрессией считается рост больше `threshold` процентов.
pub fn render_markdown(rows: &[Row], threshold: f64) -> String {
    let regressions = rows.iter().filter(|r| r.pct > threshold).count();
    let mut s = if regressions == 0 {
        format!("Регрессий больше {threshold}% нет.\n\n")
    } else {
        format!("**Регрессии больше {threshold}%: {regressions}**\n\n")
    };
    s.push_str("| Метрика | База | Сейчас | Изменение |\n|---|---:|---:|---:|\n");
    for r in rows {
        let mark = if r.pct > threshold { " ⚠️" } else { "" };
        s.push_str(&format!("| {} | {} | {} | {:+.1}%{mark} |\n", r.name, r.base, r.cur, r.pct));
    }
    s
}

/// Читает все `*.json` каталога в одну карту метрик.
pub fn read_metrics(dir: &Path) -> Result<BTreeMap<String, f64>, String> {
    let mut all = BTreeMap::new();
    let entries = std::fs::read_dir(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    for e in entries.flatten().filter(|e| e.path().extension().is_some_and(|x| x == "json")) {
        let text = std::fs::read_to_string(e.path()).map_err(|er| format!("{}: {er}", e.path().display()))?;
        let v: serde_json::Value = serde_json::from_str(&text).map_err(|er| format!("{}: {er}", e.path().display()))?;
        for (k, val) in v["metrics"].as_object().ok_or_else(|| format!("{}: нет metrics", e.path().display()))? {
            if let Some(n) = val.as_f64() {
                all.insert(k.clone(), n);
            }
        }
    }
    Ok(all)
}

/// Сравнивает каталоги отчётов и печатает таблицу; при `fail_over` ошибка, если есть регрессия больше порога.
pub fn bench_compare(base_dir: &Path, cur_dir: &Path, threshold: f64, fail_over: bool) -> Result<(), String> {
    let rows = compare(&read_metrics(base_dir)?, &read_metrics(cur_dir)?);
    print!("{}", render_markdown(&rows, threshold));
    let count = rows.iter().filter(|r| r.pct > threshold).count();
    if fail_over && count > 0 { Err(format!("регрессий больше {threshold}%: {count}")) } else { Ok(()) }
}

/// Бюджет аллокаций сценария `render-bench` на один экземпляр.
#[derive(Debug, Deserialize, PartialEq)]
pub struct AllocCase {
    /// Имя сценария (`badge-1000`).
    pub name: String,
    /// Сколько экземпляров рендерит сценарий.
    pub instances: u64,
    /// Максимум аллокаций на экземпляр.
    pub max_per_instance: f64,
}

#[derive(Debug, Deserialize)]
struct AllocBudgets {
    scenario: Vec<AllocCase>,
}

/// Читает бюджеты аллокаций.
pub fn read_alloc_budgets(path: &Path) -> Result<Vec<AllocCase>, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    toml::from_str::<AllocBudgets>(&text).map(|b| b.scenario).map_err(|e| format!("{}: {e}", path.display()))
}

/// Сообщения о превышении: «сценарий, аллокаций на экземпляр, бюджет». Ошибка, если метрики сценария нет в отчёте.
pub fn check_alloc_budgets(metrics: &BTreeMap<String, f64>, cases: &[AllocCase]) -> Result<Vec<String>, String> {
    let mut violations = Vec::new();
    for c in cases {
        let key = format!("alloc.{}.blocks", c.name);
        let blocks = metrics
            .get(&key)
            .ok_or_else(|| format!("в отчёте нет метрики `{key}`: выполните `cargo bench -p render-bench --bench alloc`"))?;
        let per_instance = blocks / c.instances as f64;
        if per_instance > c.max_per_instance {
            violations.push(format!("{}: {per_instance:.1} аллокаций на экземпляр превышает бюджет {}", c.name, c.max_per_instance));
        }
    }
    Ok(violations)
}

/// Проверяет `target/bench/alloc.json` по бюджетам; ошибка при превышении.
pub fn alloc_budget(root: &Path, budgets: &Path) -> Result<(), String> {
    let cases = read_alloc_budgets(budgets)?;
    let dir = target_dir(root).join("bench");
    let all = read_metrics(&dir)?;
    for c in &cases {
        if let Some(blocks) = all.get(&format!("alloc.{}.blocks", c.name)) {
            println!("{:<18} {:>6.1} аллокаций на экземпляр (бюджет {})", c.name, blocks / c.instances as f64, c.max_per_instance);
        }
    }
    let violations = check_alloc_budgets(&all, &cases)?;
    if violations.is_empty() {
        Ok(())
    } else {
        Err(format!("превышен бюджет аллокаций:\n  {}", violations.join("\n  ")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn map(items: &[(&str, f64)]) -> BTreeMap<String, f64> {
        items.iter().map(|(k, v)| ((*k).to_owned(), *v)).collect()
    }

    #[test]
    fn five_percent_is_not_a_regression_fifteen_is() {
        let rows = compare(&map(&[("a", 100.0), ("b", 100.0), ("c", 100.0)]), &map(&[("a", 105.0), ("b", 115.0), ("c", 90.0)]));
        let flagged: Vec<_> = rows.iter().filter(|r| r.pct > 10.0).map(|r| r.name.as_str()).collect();
        assert_eq!(flagged, vec!["b"]);
        let md = render_markdown(&rows, 10.0);
        assert!(md.contains("Регрессии больше 10%: 1") && md.contains("+15.0% ⚠️") && md.contains("+5.0% |"));
    }

    #[test]
    fn only_common_metrics_are_compared() {
        let rows = compare(&map(&[("a", 1.0)]), &map(&[("a", 1.0), ("new", 5.0)]));
        assert_eq!(rows.len(), 1);
    }

    #[test]
    fn no_regressions_message() {
        let rows = compare(&map(&[("a", 100.0)]), &map(&[("a", 100.0)]));
        assert!(render_markdown(&rows, 10.0).starts_with("Регрессий больше 10% нет"));
    }

    #[test]
    fn fail_over_only_when_requested() {
        let dir = tempfile::tempdir().unwrap();
        let (b, c) = (dir.path().join("b"), dir.path().join("c"));
        std::fs::create_dir_all(&b).unwrap();
        std::fs::create_dir_all(&c).unwrap();
        std::fs::write(b.join("x.json"), r#"{"metrics":{"m":100}}"#).unwrap();
        std::fs::write(c.join("x.json"), r#"{"metrics":{"m":115}}"#).unwrap();
        assert!(bench_compare(&b, &c, 10.0, false).is_ok(), "по умолчанию только отчёт");
        assert!(bench_compare(&b, &c, 10.0, true).is_err());
    }

    #[test]
    fn budget_violation_names_component_size_and_budget() {
        let cases = vec![Case { name: "one-icon".into(), feature: "f".into(), max_delta_bytes: 1000 }];
        assert!(check_budgets(10_000, &[("one-icon".into(), 10_900)], &cases).is_empty());
        let bad = check_budgets(10_000, &[("one-icon".into(), 11_500)], &cases);
        assert_eq!(bad, vec!["one-icon: вклад 1500 Б превышает бюджет 1000 Б".to_owned()]);
    }

    #[test]
    fn reads_budgets_file() {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("b.toml");
        std::fs::write(&p, "[[case]]\nname = \"x\"\nfeature = \"case-x\"\nmax_delta_bytes = 5\n").unwrap();
        assert_eq!(read_budgets(&p).unwrap(), vec![Case { name: "x".into(), feature: "case-x".into(), max_delta_bytes: 5 }]);
    }

    #[test]
    fn alloc_budget_violation_and_missing_metric() {
        let cases = vec![AllocCase { name: "badge-1000".into(), instances: 1000, max_per_instance: 10.0 }];
        assert!(check_alloc_budgets(&map(&[("alloc.badge-1000.blocks", 8017.0)]), &cases).unwrap().is_empty());
        let bad = check_alloc_budgets(&map(&[("alloc.badge-1000.blocks", 12000.0)]), &cases).unwrap();
        assert_eq!(bad, vec!["badge-1000: 12.0 аллокаций на экземпляр превышает бюджет 10".to_owned()]);
        let err = check_alloc_budgets(&map(&[]), &cases).unwrap_err();
        assert!(err.contains("alloc.badge-1000.blocks"), "{err}");
    }

    #[test]
    fn reads_alloc_budgets_file() {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("a.toml");
        std::fs::write(&p, "[[scenario]]\nname = \"x-1000\"\ninstances = 1000\nmax_per_instance = 7.5\n").unwrap();
        assert_eq!(read_alloc_budgets(&p).unwrap(), vec![AllocCase { name: "x-1000".into(), instances: 1000, max_per_instance: 7.5 }]);
    }

    #[test]
    fn alloc_budget_reads_report_directory() {
        let dir = tempfile::tempdir().unwrap();
        let bench = dir.path().join("target/bench");
        std::fs::create_dir_all(&bench).unwrap();
        std::fs::write(bench.join("alloc.json"), r#"{"metrics":{"alloc.b-10.blocks":150}}"#).unwrap();
        let budgets = dir.path().join("b.toml");
        std::fs::write(&budgets, "[[scenario]]\nname = \"b-10\"\ninstances = 10\nmax_per_instance = 20\n").unwrap();
        // 15 на экземпляр <= 20
        // SAFETY-free: target_dir берёт CARGO_TARGET_DIR из окружения, поэтому проверяем через check_alloc_budgets напрямую.
        let metrics = read_metrics(&bench).unwrap();
        assert!(check_alloc_budgets(&metrics, &read_alloc_budgets(&budgets).unwrap()).unwrap().is_empty());
    }
}
