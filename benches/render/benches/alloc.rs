//! Allocations during rendering (dhat): `cargo bench -p render-bench --bench alloc`.
//!
//! Each scenario runs in a separate process so that peak memory does not get mixed.
//! The result is written to `target/bench/alloc.json`.

use render_bench::{SCENARIOS, out_dir};
use std::collections::BTreeMap;

#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn measure(name: &str) {
    let scenario = SCENARIOS.iter().find(|s| s.name == name).expect("unknown scenario");
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

/// Full dhat heap profile of one scenario (allocation sites with backtraces), for finding what allocates:
/// `cargo bench -p render-bench --bench alloc -- --profile tree-1000` writes `target/bench/dhat-<name>.json`,
/// viewable in https://nnethercote.github.io/dh_view/dh_view.html.
fn profile(name: &str) {
    let scenario = SCENARIOS.iter().find(|s| s.name == name).expect("unknown scenario");
    let path = out_dir().join(format!("dhat-{name}.json"));
    let profiler = dhat::Profiler::builder().file_name(&path).trim_backtraces(Some(12)).build();
    std::hint::black_box((scenario.run)());
    drop(profiler);
    println!("profile: {}", path.display());
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if let Some(i) = args.iter().position(|a| a == "--profile") {
        return profile(&args[i + 1]);
    }
    if let Some(i) = args.iter().position(|a| a == "--scenario") {
        return measure(&args[i + 1]);
    }
    let exe = std::env::current_exe().unwrap();
    let mut metrics = BTreeMap::new();
    for s in SCENARIOS {
        let out = std::process::Command::new(&exe).args(["--scenario", s.name]).output().expect("scenario launch");
        assert!(out.status.success(), "{}: {}", s.name, String::from_utf8_lossy(&out.stderr));
        let v: serde_json::Value = serde_json::from_slice(&out.stdout).expect("scenario JSON");
        println!(
            "{:<20} allocations {:>8}  total {:>10} B  peak {:>10} B  html {:>8} B",
            s.name, v["blocks"], v["bytes"], v["peak_bytes"], v["html_bytes"]
        );
        for key in ["blocks", "bytes", "peak_bytes"] {
            metrics.insert(format!("alloc.{}.{key}", s.name), v[key].clone());
        }
    }
    let path = out_dir().join("alloc.json");
    std::fs::write(&path, serde_json::to_string_pretty(&serde_json::json!({ "metrics": metrics })).unwrap()).unwrap();
    println!("report: {}", path.display());
}
