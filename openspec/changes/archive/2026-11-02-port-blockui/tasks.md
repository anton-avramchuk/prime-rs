# Tasks

## 1. Foundation

- [x] 1.1 Add `blockui` to `theme.components` (`upstream.toml`), `cargo xtask gen-theme`; check that the CSS of the other components did not change and that the classes `p-blockui*` are present; make sure that `p-overlay-mask` and its animations are in `base.css`

## 2. Crate

- [x] 2.1 `crates/prime-blockui`: `state.rs` (`mask_classes`, `aria_busy`, `mask_style`) with table tests
- [x] 2.2 `BlockUI`, `BlockTarget`: properties, rendering of the mask through `Portal` into the target or `body`, the state machine, the SSR test (no mask), rustdoc
- [x] 2.3 The layer, the scroll blocking, the `position` of the target, the exit by `animationend`/timer, re-enabling, release on unmount; browser `tests/web.rs`

## 3. Showcase and e2e

- [x] 3.1 The `/blockui` page (`basic`, `document`), the menu, the route, `stylesheet(&[..])`, the crate in the workspace and the showcase
- [x] 3.2 `e2e/tests/pages.json` and `blockui.spec.ts` (the mask over a panel, a click under the mask, the document for three seconds); wait for `networkidle`, restart the showcase server; `PW_CHANNEL=chrome npx playwright test blockui`

## 4. Benchmarks and budgets

- [x] 4.1 Scenarios `-1` and `-1000`, `case-blockui` and the budgets (first `cargo bench -p render-bench --bench alloc`; the wasm measurement — in the background, with a separate budgets file)

## 5. Integration

- [x] 5.1 `docs/blockui.md` (differences from PrimeNG, what was not verified); `prime-blockui` in the browser job of `ci.yml` (three places)
- [x] 5.2 `cargo fmt --all -- --check`, clippy with the CI exceptions and `--all-features` (and for the wasm target), tests, deny, machete, `openspec validate port-blockui`; CI `dev` is green

## Workflow follow-up

- Archive after CI.
- Next: SplitButton.
