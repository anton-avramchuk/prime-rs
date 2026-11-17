# Tasks

## 1. New crates

- [x] 1.1 `crates/prime-overlaybadge` (`OverlayBadge`, `OverlayBadgePt`): crate, component, SSR tests, rustdoc
- [x] 1.2 `crates/prime-timeline` (`Timeline`, `TimelineAlign`, `TimelineLayout`, `TimelinePt`): `state.rs` with classes and `data_p` (table-driven test), component with templates, SSR tests, rustdoc
- [x] 1.3 `crates/prime-metergroup` (`MeterGroup`, `MeterItem`, `MeterGroupPt`, enums): `state.rs` with `percent`/`total_percent`/`cumulative` (table-driven tests), component with labels and templates, SSR tests, rustdoc

## 2. Theme, showcase, e2e

- [x] 2.1 Add `overlaybadge`, `timeline`, `metergroup` to `theme.components` (`upstream.toml`), `cargo xtask gen-theme`, make sure the CSS of other components has not changed
- [x] 2.2 Pages `/overlaybadge`, `/timeline`, `/metergroup`, menu, routes, `stylesheet(&[..])`; new crates in the workspace
- [x] 2.3 `e2e/tests/pages.json` and spec files (badge corner, timeline direction and positions, segment width and labels); wait for `networkidle`, restart the showcase server; `PW_CHANNEL=chrome npx playwright test`

## 3. Benchmarks and budgets

- [x] 3.1 Scenarios `-1` and `-1000`, `case-*` and budgets for the three crates (first `cargo bench -p render-bench --bench alloc`)

## 4. Integration

- [x] 4.1 `docs/display-batch.md` (differences from PrimeNG: the `align` enum, callback templates, `MeterItem` icons, demos without the missing icons)
- [x] 4.2 `cargo fmt --all -- --check`, clippy with the CI exceptions and with `--all-features`, tests, deny, machete, `openspec validate port-display-batch`; CI `dev` green

## Workflow follow-up

- Archive after CI.
- Next batch: the `MenuItem` model + `Breadcrumb` + `Steps`, then InputOtp, ScrollTop, Password.
