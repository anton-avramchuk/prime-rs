# Tasks

## 1. Crate

- [x] 1.1 Create the crate `crates/prime-card` (`publish = false`, features `ssr`/`hydrate`, dependency `prime-core`) and add it to the workspace; check `cargo check -p prime-card --all-features`

## 2. Component

- [x] 2.1 Implement `Card` with two render branches, header/body/title/subtitle/content/footer blocks, slot priority over text and `pt` (`CardPt`); verify all spec scenarios with SSR tests
- [x] 2.2 Add the branch parity table and rustdoc with an example; check `cargo test -p prime-card --all-features` and doc tests

## 3. Showcase and e2e

- [x] 3.1 Add the `/card` page (`basic`, `advanced` with a Button in the footer), a menu item and the `card` CSS (already in the shell); check `cargo leptos build` and the page in Chrome without console errors
- [x] 3.2 Add entries to `e2e/tests/pages.json` and `e2e/tests/card.spec.ts` (structure, block order, a footer with buttons); check `npx playwright test card` locally

## 4. Benchmarks and budgets

- [x] 4.1 Add scenarios `card-1`, `card-1000` to `benches/render`, `case-card` to `benches/wasm-size`, budgets by measurement; check `cargo bench -p render-bench --bench alloc`, `cargo xtask alloc-budget` and `cargo xtask wasm-size`

## 5. Integration

- [x] 5.1 Write `docs/card.md` (PrimeNG correspondence, differences); run `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace --all-features` (except apps), `cargo deny check`, `openspec validate port-card`; make sure CI on `dev` is green

## Workflow follow-up

- Archive the change after CI passes; next wave 1 components: InputText, Checkbox, RadioButton, ToggleSwitch, Message, ProgressBar.
