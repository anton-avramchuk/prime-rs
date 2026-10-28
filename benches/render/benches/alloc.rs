//! Аллокации при рендере (dhat): `cargo bench -p render-bench --bench alloc`.
//!
//! Каждый сценарий выполняется в отдельном процессе, чтобы пиковая память не смешивалась.
//! Результат пишется в `target/bench/alloc.json`.

use render_bench::{SCENARIOS, out_dir};
use std::collections::BTreeMap;

#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn measure(name: &str) {
    let scenario = SCENARIOS.iter().find(|s| s.name == name).expect("неизвестный сценарий");
    let _profiler = dhat::Profiler::builder().testing().build();
    let before = dhat::HeapStats::get();
    let html = (scenario.run)();
    let after = dhat::HeapStats::get();
    std::hint::black_box(&html);
    println!(
        "{}",
        serde_json::json!({
            "blocks": after.total_blocks - before.total_blocks,
            "bytes": after.total_bytes - before.total_bytes,
            "peak_bytes": after.max_bytes,
            "html_bytes": html.len(),
        })
    );
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if let Some(i) = args.iter().position(|a| a == "--scenario") {
        return measure(&args[i + 1]);
    }
    let exe = std::env::current_exe().unwrap();
    let mut metrics = BTreeMap::new();
    for s in SCENARIOS {
        let out = std::process::Command::new(&exe).args(["--scenario", s.name]).output().expect("запуск сценария");
        assert!(out.status.success(), "{}: {}", s.name, String::from_utf8_lossy(&out.stderr));
        let v: serde_json::Value = serde_json::from_slice(&out.stdout).expect("JSON сценария");
        println!(
            "{:<20} аллокаций {:>8}  всего {:>10} Б  пик {:>10} Б  html {:>8} Б",
            s.name, v["blocks"], v["bytes"], v["peak_bytes"], v["html_bytes"]
        );
        for key in ["blocks", "bytes", "peak_bytes"] {
            metrics.insert(format!("alloc.{}.{key}", s.name), v[key].clone());
        }
    }
    let path = out_dir().join("alloc.json");
    std::fs::write(&path, serde_json::to_string_pretty(&serde_json::json!({ "metrics": metrics })).unwrap()).unwrap();
    println!("отчёт: {}", path.display());
}
