//! Repository utility commands (`cargo xtask <command>`).

mod affected;
mod bench;
mod clean;
mod diff;
mod gen_docs;
mod gen_icons;
mod gen_readme;
mod gen_spec;
mod gen_theme;
mod showcase;
mod upstream;

use std::path::PathBuf;
use std::process::ExitCode;

const HELP: &str = "\
cargo xtask <command>

Commands:
  upstream-diff [--to <rev>]   ported components changed in PrimeNG since the pinned commit
  gen-spec <name> [--out <file>] component spec skeleton from PrimeNG sources
  gen-docs [--check]           docs site data showcase/src/generated.rs from the sources
  gen-readme [--check]         crates/*/README.md from Cargo.toml and the crate docs
  gen-icons                    generate crates/prime-icons from PrimeNG icons
  gen-theme                    generate the Aura theme CSS into crates/prime-theme (needs Node)
  wasm-size [--budgets <file>] contribution of components to wasm size and budget check (benches/budgets.toml)
  alloc-budget [--budgets <file>] check target/bench/alloc.json against benches/alloc-budgets.toml
  bench-compare <base> <now> [--threshold 10] [--fail-over]
                               compare benchmark report directories (*.json), output is Markdown
  affected [--base <ref> | files...]  affected crates for selective CI checks ($GITHUB_OUTPUT format)
  pages [--base <path>] [--dist <dir>]  static showcase: pre-rendered pages + split wasm (target/pages by default)
  clean [--days <N>] [--max-size <GB>]  remove target/ artifacts older than N days (7 by default) and/or the oldest ones while target/ is larger than GB
";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().expect("xtask lives in the repository root").to_path_buf()
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
                println!("no changes in ported components ({}..{to})", &up.commit[..10.min(up.commit.len())]);
            } else {
                println!("changed components:");
                changed.iter().for_each(|c| println!("  {c}"));
            }
            Ok(())
        }
        Some("gen-spec") => {
            let name = args.get(1).filter(|a| !a.starts_with("--")).ok_or("specify the component name")?;
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
            println!("icons generated: {n} (+ blank by hand)");
            Ok(())
        }
        Some("gen-docs") => gen_docs::run(&root, args.iter().any(|a| a == "--check")),
        Some("gen-readme") => gen_readme::run(&root, args.iter().any(|a| a == "--check")),
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
                return Err("usage: bench-compare <base directory> <current reports directory>".into());
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
        Some("pages") => {
            let base = flag(args, "--base").unwrap_or_default();
            let dist = flag(args, "--dist").map_or_else(|| root.join("target/pages"), PathBuf::from);
            showcase::pages(&root, &base, &dist)
        }
        Some("clean") => {
            let days = flag(args, "--days").map(|d| d.parse::<u32>().map_err(|e| format!("--days: {e}"))).transpose()?;
            let max_gb = flag(args, "--max-size").map(|d| d.parse::<u32>().map_err(|e| format!("--max-size: {e}"))).transpose()?;
            clean::run(&root, days, max_gb)
        }
        Some("--help" | "-h" | "help") | None => {
            print!("{HELP}");
            Ok(())
        }
        Some(other) => Err(format!("unknown command `{other}`\n\n{HELP}")),
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match run(&args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}
