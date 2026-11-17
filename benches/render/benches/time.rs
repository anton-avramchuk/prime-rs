//! Median render time in JSON for comparison with the baseline: `cargo bench -p render-bench --bench time`.
//!
//! Detailed measurements are in `render` (divan); this benchmark is for machine comparison (`cargo xtask bench-compare`).

use render_bench::{SCENARIOS, out_dir};
use std::collections::BTreeMap;
use std::time::Instant;

const WARMUP: usize = 5;
const RUNS: usize = 31;

fn main() {
    let mut metrics = BTreeMap::new();
    for s in SCENARIOS {
        for _ in 0..WARMUP {
            std::hint::black_box((s.run)());
        }
        let mut samples: Vec<u128> = (0..RUNS)
            .map(|_| {
                let t = Instant::now();
                std::hint::black_box((s.run)());
                t.elapsed().as_nanos()
            })
            .collect();
        samples.sort_unstable();
        let median = samples[RUNS / 2];
        println!("{:<20} median {:>10} ns", s.name, median);
        metrics.insert(format!("time.{}.median_ns", s.name), median);
    }
    let path = out_dir().join("time.json");
    std::fs::write(&path, serde_json::to_string_pretty(&serde_json::json!({ "metrics": metrics })).unwrap()).unwrap();
    println!("report: {}", path.display());
}
