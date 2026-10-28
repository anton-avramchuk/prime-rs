//! Время SSR-рендера (divan): `cargo bench -p render-bench --bench render`.

use render_bench::SCENARIOS;

fn main() {
    divan::main();
}

#[divan::bench(args = SCENARIOS.iter().map(|s| s.name))]
fn render(bencher: divan::Bencher, name: &str) {
    let scenario = SCENARIOS.iter().find(|s| s.name == name).unwrap();
    bencher.bench_local(|| divan::black_box((scenario.run)()));
}
