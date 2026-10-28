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

/// Удаляет артефакты, не использовавшиеся дольше `days` дней, и печатает освобождённый объём.
pub fn run(root: &Path, days: u32) -> Result<(), String> {
    let target = root.join("target");
    let before = dir_size(&target);
    let status = Command::new("cargo")
        .current_dir(root)
        .args(["sweep", "--time", &days.to_string()])
        .status()
        .map_err(|e| format!("cargo sweep: {e}"))?;
    if !status.success() {
        return Err("cargo sweep завершился с ошибкой; установите: cargo install cargo-sweep".into());
    }
    let after = dir_size(&target);
    println!("освобождено: {}", human(before.saturating_sub(after)));
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
    fn dir_size_sums_files() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("a"), [0u8; 10]).unwrap();
        std::fs::create_dir(dir.path().join("d")).unwrap();
        std::fs::write(dir.path().join("d/b"), [0u8; 5]).unwrap();
        assert_eq!(dir_size(dir.path()), 15);
    }
}
