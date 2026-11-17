# Tasks

## 1. Crate and logic

- [x] 1.1 Create the crate `crates/prime-chip` (`publish = false`, features `ssr`/`hydrate`, dependencies `prime-core`, `prime-icons/timescircle`, `web-sys`) and add it to the workspace; check `cargo check -p prime-chip --all-features`
- [x] 1.2 Implement `ChipState`, `write_classes`, `data_p` and the parsing of the remove key (`is_remove_key`: Enter, Backspace); check the spec scenarios (classes, `data-p`, keys) with unit tests

## 2. Component

- [x] 2.1 Implement `Chip`: the root, the order of `children`/image or icon/label/removal, `pt` (`ChipPt`), the static branch for `all_static && !removable`, the reactive one with an internal `visible`; check the structure, classes, order, label from the locale and `pt` with SSR tests
- [x] 2.2 Implement removal (click, Enter, Backspace, hiding `display: none`, `on_remove` once) and `on_image_error`; check the spec scenarios with `wasm-bindgen-test`, including `disabled` (`tabindex=-1`)
- [x] 2.3 Add a branch parity table and rustdoc with an example; check `cargo test -p prime-chip --all-features` and the doc tests

## 3. Showcase and e2e

- [x] 3.1 Add the `/chip` page (`basic`, `icon`, `image`, `template`), a menu item and the CSS `chip` in the wrapper; check `cargo leptos build` and the page in Chrome without console errors
- [x] 3.2 Add entries to `e2e/tests/pages.json` and `e2e/tests/chip.spec.ts` (removal by mouse and keyboard, hiding, label); check `npx playwright test chip` locally

## 4. Benchmarks and budgets

- [x] 4.1 Add the scenarios `chip-1`, `chip-1000` in `benches/render`, `case-chip` in `benches/wasm-size`, budgets by measurement; check `cargo bench -p render-bench --bench alloc`, `cargo xtask alloc-budget` and `cargo xtask wasm-size`

## 5. Integration

- [x] 5.1 Write `docs/chip.md` (conformance with PrimeNG, differences); run `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace --all-features` (except the applications), `cargo deny check`, `openspec validate port-chip`; make sure CI on `dev` is green

## Workflow follow-up

- Archive the change after CI passes; the next wave-1 component: Avatar.
