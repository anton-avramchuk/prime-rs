# Tasks

## 1. Crate

- [x] 1.1 `crates/prime-splitter`: `state.rs` (`initial_sizes`, `flex_basis`, `drag_delta`, `apply`, `key_step`, classes, `data_p`) with table-driven tests
- [x] 1.2 `Splitter` component: properties, markup (panels, gutters, handles), `SplitterPt`, SSR tests, rustdoc
- [x] 1.3 Behaviour: mouse and touch dragging, `min_sizes`, keyboard, events, storage; browser tests `tests/web.rs`

## 2. Theme, showcase, e2e

- [x] 2.1 Add `splitter` to `theme.components` (`upstream.toml`), `cargo xtask gen-theme`, make sure the CSS of other components did not change
- [x] 2.2 Page `/splitter` (`horizontal`, `vertical`, `size`, `nested`), menu, route, `stylesheet(&[..])`, the crate in the workspace and showcase
- [x] 2.3 `e2e/tests/pages.json` and `splitter.spec.ts` (mouse dragging, arrows, minimum sizes, nesting); wait for `networkidle`, restart the showcase server; `PW_CHANNEL=chrome npx playwright test splitter`

## 3. Benchmarks and budgets

- [x] 3.1 Scenarios `-1` and `-1000`, `case-splitter` and budgets (first `cargo bench -p render-bench --bench alloc`)

## 4. Integration

- [x] 4.1 `docs/splitter.md` (differences from PrimeNG, what was not verified); `prime-splitter` in the browser job of `ci.yml` (three places)
- [x] 4.2 `cargo fmt --all -- --check`, clippy with `--all-features` (and for the wasm target), tests, deny, machete, `openspec validate port-splitter`; CI `dev` green

## Workflow follow-up

- Archived after CI.
- Next: Tabs.
