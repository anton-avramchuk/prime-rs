//! Служебные команды репозитория (`cargo xtask <команда>`).

mod clean;
mod diff;
mod gen_icons;
mod gen_spec;
mod upstream;

use std::path::PathBuf;
use std::process::ExitCode;

const HELP: &str = "\
cargo xtask <команда>

Команды:
  upstream-diff [--to <rev>]   портированные компоненты, изменившиеся в PrimeNG с закреплённого коммита
  gen-spec <имя> [--out <файл>] заготовка спеки компонента из исходников PrimeNG
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
