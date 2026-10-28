//! Служебные команды репозитория (`cargo xtask <команда>`).

mod affected;
mod bench;
mod clean;
mod diff;
mod gen_icons;
mod gen_spec;
mod gen_theme;
mod upstream;

use std::path::PathBuf;
use std::process::ExitCode;

const HELP: &str = "\
cargo xtask <команда>

Команды:
  upstream-diff [--to <rev>]   портированные компоненты, изменившиеся в PrimeNG с закреплённого коммита
  gen-spec <имя> [--out <файл>] заготовка спеки компонента из исходников PrimeNG
  gen-icons                    сгенерировать crates/prime-icons из иконок PrimeNG
  gen-theme                    сгенерировать CSS темы Aura в crates/prime-theme (нужен Node)
  wasm-size [--budgets <файл>] вклад компонентов в размер wasm и проверка бюджетов (benches/budgets.toml)
  alloc-budget [--budgets <файл>] проверить target/bench/alloc.json по benches/alloc-budgets.toml
  bench-compare <база> <сейчас> [--threshold 10] [--fail-over]
                               сравнить каталоги отчётов бенчмарков (*.json), вывод — Markdown
  affected [--base <ref> | файлы...]  затронутые крейты для выборочных проверок CI (формат $GITHUB_OUTPUT)
  clean [--days <N>]           удалить артефакты target/ старше N дней (по умолчанию 7)
";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().expect("xtask лежит в корне репозитория").to_path_buf()
}

fn flag(args: &[String], name: &str) -> Option<String> {
    args.iter().position(|a| a == name).and_then(|i| args.get(i + 1)).cloned()
}

fn run(args: &[String]) -> Result<(), String> {
    let root = root();
    match args.first().map(String::as_str) {
        Some("upstream-diff") => {
            let up = upstream::Upstream::load(&root)?;
            let to = flag(args, "--to").unwrap_or_else(|| "HEAD".into());
            let ported = diff::ported_components(&root);
            let changed = diff::changed_components(&up.path, &up.commit, &to, &ported)?;
            if changed.is_empty() {
                println!("изменений в портированных компонентах нет ({}..{to})", &up.commit[..10.min(up.commit.len())]);
            } else {
                println!("изменились компоненты:");
                changed.iter().for_each(|c| println!("  {c}"));
            }
            Ok(())
        }
        Some("gen-spec") => {
            let name = args.get(1).filter(|a| !a.starts_with("--")).ok_or("укажите имя компонента")?;
            let up = upstream::Upstream::load(&root)?;
            let text = gen_spec::generate(&up.src_dir(), name, &up.commit, &up.version)?;
            match flag(args, "--out") {
                Some(out) => std::fs::write(&out, text).map_err(|e| format!("{out}: {e}")),
                None => {
                    print!("{text}");
                    Ok(())
                }
            }
        }
        Some("gen-icons") => {
            let up = upstream::Upstream::load(&root)?;
            let n = gen_icons::run(&up.src_dir().join("icons"), &root.join("crates/prime-icons"))?;
            println!("иконок сгенерировано: {n} (+ blank вручную)");
            Ok(())
        }
        Some("gen-theme") => {
            let up = upstream::Upstream::load(&root)?;
            gen_theme::run(&root, &up)
        }
        Some("wasm-size") => {
            let budgets = flag(args, "--budgets").map_or_else(|| root.join("benches/budgets.toml"), PathBuf::from);
            bench::wasm_size(&root, &budgets)
        }
        Some("alloc-budget") => {
            let budgets = flag(args, "--budgets").map_or_else(|| root.join("benches/alloc-budgets.toml"), PathBuf::from);
            bench::alloc_budget(&root, &budgets)
        }
        Some("bench-compare") => {
            let (Some(base), Some(cur)) = (args.get(1), args.get(2)) else {
                return Err("использование: bench-compare <каталог базы> <каталог текущих отчётов>".into());
            };
            let threshold = flag(args, "--threshold").map_or(Ok(10.0), |v| v.parse::<f64>().map_err(|e| format!("--threshold: {e}")))?;
            bench::bench_compare(&PathBuf::from(base), &PathBuf::from(cur), threshold, args.iter().any(|a| a == "--fail-over"))
        }
        Some("affected") => {
            let pkgs = affected::read_packages(&root)?;
            let changed = match flag(args, "--base") {
                Some(base) => affected::changed_files(&root, &base)?,
                None => args.iter().skip(1).filter(|a| !a.starts_with("--")).cloned().collect(),
            };
            print!("{}", affected::render_outputs(&affected::affected(&pkgs, &changed)));
            Ok(())
        }
        Some("clean") => {
            let days = flag(args, "--days").map_or(Ok(7), |d| d.parse::<u32>().map_err(|e| format!("--days: {e}")))?;
            clean::run(&root, days)
        }
        Some("--help" | "-h" | "help") | None => {
            print!("{HELP}");
            Ok(())
        }
        Some(other) => Err(format!("неизвестная команда `{other}`\n\n{HELP}")),
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match run(&args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("ошибка: {e}");
            ExitCode::FAILURE
        }
    }
}
