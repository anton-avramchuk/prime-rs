//! `clean`: удаление устаревших артефактов `target/` через `cargo-sweep`.

use std::path::Path;
use std::process::Command;

/// Размер каталога в байтах.
pub fn dir_size(path: &Path) -> u64 {
    let Ok(entries) = std::fs::read_dir(path) else {
        return 0;
    };
    entries
        .flatten()
        .map(|e| match e.metadata() {
            Ok(m) if m.is_dir() => dir_size(&e.path()),
            Ok(m) => m.len(),
            Err(_) => 0,
        })
        .sum()
}

/// Человекочитаемый размер.
pub fn human(bytes: u64) -> String {
    const UNITS: [&str; 4] = ["Б", "КБ", "МБ", "ГБ"];
    let mut value = bytes as f64;
    let mut unit = 0;
    while value >= 1024.0 && unit < UNITS.len() - 1 {
        value /= 1024.0;
        unit += 1;
    }
    format!("{value:.1} {}", UNITS[unit])
}

/// Аргументы `cargo sweep` по условиям очистки: давность в днях и/или предельный размер `target/` в ГБ
/// (`--maxsize` у cargo-sweep задаётся в МБ). Без условий — давность `default_days`.
pub fn sweep_args(days: Option<u32>, max_gb: Option<u32>, default_days: u32) -> Vec<Vec<String>> {
    let mut runs = Vec::new();
    if days.is_some() || max_gb.is_none() {
        runs.push(vec!["sweep".to_owned(), "--time".to_owned(), days.unwrap_or(default_days).to_string()]);
    }
    if let Some(gb) = max_gb {
        runs.push(vec!["sweep".to_owned(), "--maxsize".to_owned(), (u64::from(gb) * 1024).to_string()]);
    }
    runs
}

/// Удаляет артефакты, не использовавшиеся дольше `days` дней, и/или самые старые, пока `target/` больше `max_gb` ГБ;
/// печатает размер до и после.
pub fn run(root: &Path, days: Option<u32>, max_gb: Option<u32>) -> Result<(), String> {
    let target = root.join("target");
    let before = dir_size(&target);
    for args in sweep_args(days, max_gb, 7) {
        let status = Command::new("cargo").current_dir(root).args(&args).status().map_err(|e| format!("cargo sweep: {e}"))?;
        if !status.success() {
            return Err("cargo sweep завершился с ошибкой; установите: cargo install cargo-sweep".into());
        }
    }
    let after = dir_size(&target);
    println!("было: {}, стало: {}, освобождено: {}", human(before), human(after), human(before.saturating_sub(after)));
    if let Some(gb) = max_gb
        && after > u64::from(gb) * 1024 * 1024 * 1024
    {
        println!("предупреждение: target/ всё ещё больше {gb} ГБ (используемые артефакты cargo-sweep не трогает)");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn human_sizes() {
        assert_eq!(human(0), "0.0 Б");
        assert_eq!(human(1536), "1.5 КБ");
        assert_eq!(human(5 * 1024 * 1024), "5.0 МБ");
    }

    #[test]
    fn sweep_arguments() {
        let v = |a: &[&str]| a.iter().map(|x| (*x).to_owned()).collect::<Vec<_>>();
        assert_eq!(sweep_args(None, None, 7), vec![v(&["sweep", "--time", "7"])]);
        assert_eq!(sweep_args(Some(3), None, 7), vec![v(&["sweep", "--time", "3"])]);
        assert_eq!(sweep_args(None, Some(5), 7), vec![v(&["sweep", "--maxsize", "5120"])]);
        assert_eq!(sweep_args(Some(2), Some(5), 7), vec![v(&["sweep", "--time", "2"]), v(&["sweep", "--maxsize", "5120"])]);
    }

    #[test]
    fn dir_size_sums_files() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("a"), [0u8; 10]).unwrap();
        std::fs::create_dir(dir.path().join("d")).unwrap();
        std::fs::write(dir.path().join("d/b"), [0u8; 5]).unwrap();
        assert_eq!(dir_size(dir.path()), 15);
    }
}
