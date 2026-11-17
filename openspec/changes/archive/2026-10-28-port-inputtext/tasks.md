# Tasks

## 1. Crate and logic

- [x] 1.1 Create the crate `crates/prime-inputtext` (`publish = false`, features `ssr`/`hydrate`, dependencies `prime-core`, `web-sys`) and add to the workspace; check `cargo check -p prime-inputtext --all-features`
- [x] 1.2 Implement `InputTextState`, `write_classes`, `data_p` after `inputtextstyle.ts` and `InputText.dataP`; check the spec scenarios with a table of unit tests (size, variant, `invalid`, `fluid`, `filled`, `data-p` order)

## 2. Component

- [x] 2.1 Implement `InputText` (attributes, classes, `data-p`, variant from `PrimeConfig`, `pt`, two rendering branches); check with SSR tests of structure, attributes, classes, the value in the HTML and branch parity
- [x] 2.2 Implement the two-way `value` binding and the events `on_input`, `on_change`, `on_focus`, `on_blur`; check with `wasm-bindgen-test` (input writes to the signal, the signal updates the field, events, `disabled`/`readonly`)
- [x] 2.3 Add rustdoc with an example; check `cargo test -p prime-inputtext --all-features` and doc tests

## 3. Showcase and e2e

- [x] 3.1 Add the `/inputtext` page (`basic`, `filled`, `sizes`, `invalid`, `disabled`, `fluid`), a menu item and the `inputtext` CSS; check `cargo leptos build` and the page in Chrome without console errors
- [x] 3.2 Add entries to `e2e/tests/pages.json` and `e2e/tests/inputtext.spec.ts` (input, classes, `invalid`, `disabled`); check `npx playwright test inputtext` locally

## 4. Benchmarks and budgets

- [x] 4.1 Add scenarios `inputtext-1`, `inputtext-1000` to `benches/render`, `case-inputtext` to `benches/wasm-size`, budgets from measurement; check `cargo bench -p render-bench --bench alloc`, `cargo xtask alloc-budget` and `cargo xtask wasm-size`

## 5. Integration

- [x] 5.1 Write `docs/inputtext.md` (correspondence to PrimeNG, differences, the value model as a reference for input fields); run `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace --all-features` (except apps), `cargo deny check`, `openspec validate port-inputtext`; make sure CI on `dev` is green

## Workflow follow-up

- Archive the change after CI passes; next wave 1 components: Checkbox, RadioButton, ToggleSwitch, Message, ProgressBar.
