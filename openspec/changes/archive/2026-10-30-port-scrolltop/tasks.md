# Tasks

## 1. Crate

- [x] 1.1 `crates/prime-scrolltop`: `state.rs` (`is_visible`, `root_classes`, the animation classes) with table tests
- [x] 1.2 The `ScrollTop` component: `ScrollTopTarget`, `ScrollTopBehavior`, `ScrollTopPt`, properties, markup over `Button`, SSR tests, rustdoc
- [x] 1.3 Behaviour: subscription to `scroll` (window/parent) and its removal, the initial check, phases and animation by classes, z-index, click with scrolling; browser tests `tests/web.rs`

## 2. Theme, showcase, e2e

- [x] 2.1 Add `scrolltop` to `theme.components` (`upstream.toml`), `cargo xtask gen-theme`, check that the CSS of the other components did not change
- [x] 2.2 The `/scrolltop` page (`basic`, `element`), the menu, the route, `stylesheet(&[..])`, the crate in the workspace and the showcase
- [x] 2.3 `e2e/tests/pages.json` and `scrolltop.spec.ts` (threshold, click, return, `Parent`, keys); wait for `networkidle`, restart the showcase server; `PW_CHANNEL=chrome npx playwright test scrolltop`

## 3. Benchmarks and budgets

- [x] 3.1 Scenarios `-1` and `-1000`, `case-scrolltop` and the budgets (first `cargo bench -p render-bench --bench alloc`)

## 4. Integration

- [x] 4.1 `docs/scrolltop.md` (differences from PrimeNG, what was not verified); `prime-scrolltop` in the browser job of `ci.yml` (three places)
- [x] 4.2 `cargo fmt --all -- --check`, clippy with `--all-features` (and for the wasm target), tests, deny, machete, `openspec validate port-scrolltop`; CI `dev` is green

## Workflow follow-up

- Archived after CI.
- Next: Password.
