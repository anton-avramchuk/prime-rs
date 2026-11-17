# Tasks

## 1. Foundation

- [x] 1.1 Add `imagecompare` to `theme.components` (`upstream.toml`), `cargo xtask gen-theme`; check that the CSS of the other components did not change and that the classes `p-imagecompare`, `p-imagecompare-slider` are present

## 2. Crate

- [x] 2.1 `crates/prime-imagecompare`: `state.rs` (`clip_path`) with table tests
- [x] 2.2 The `ImageCompare` component, `ImageComparePt`: the root, `left`/`right`, the slider, attributes, the input handler (`clip-path`, `rtl`), SSR tests, rustdoc
- [x] 2.3 Browser `tests/web.rs`: input of a value, `rtl`, repeated input, input without a sibling

## 3. Showcase and e2e

- [x] 3.1 The `/imagecompare` page (`basic`, `responsive`), the menu, the route, `stylesheet(&[..])`, the crate in the workspace and the showcase
- [x] 3.2 `e2e/tests/pages.json` and `imagecompare.spec.ts` (markup, dragging the handle, `clip-path`); wait for `networkidle`, restart the showcase server; `PW_CHANNEL=chrome npx playwright test imagecompare`

## 4. Benchmarks and budgets

- [x] 4.1 Scenarios `-1` and `-1000`, `case-imagecompare` and the budgets (first `cargo bench -p render-bench --bench alloc`; the wasm measurement — in the background, with a separate budgets file)

## 5. Integration

- [x] 5.1 `docs/imagecompare.md` (differences from PrimeNG, what was not verified); `prime-imagecompare` in the browser job of `ci.yml` (three places)
- [x] 5.2 `cargo fmt --all -- --check`, clippy with the CI exceptions and `--all-features` (and for the wasm target), tests, deny, machete, `openspec validate port-imagecompare`; CI `dev` is green

## Workflow follow-up

- Archive after CI.
- Next: BlockUI.
