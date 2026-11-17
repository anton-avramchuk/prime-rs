# Tasks

## 1. Crate and logic

- [x] 1.1 Create the crate `crates/prime-checkbox` (`publish = false`, features `ssr`/`hydrate`, dependencies `prime-core`, `prime-icons`, `web-sys`), add it to the workspace; check `cargo check -p prime-checkbox --all-features`
- [x] 1.2 Implement `CheckboxState`, `write_classes`, `data_p` per `checkboxstyle.ts` and `Checkbox.dataP`; check with a table of unit tests

## 2. Component

- [x] 2.1 Implement `Checkbox` (markup, classes, attributes, `pt`, icons, two branches); check with SSR tests and parity
- [x] 2.2 Implement the `binary` and set modes, `indeterminate`, `readonly`, events; check with `wasm-bindgen-test`
- [x] 2.3 Add rustdoc with an example; check `cargo test -p prime-checkbox --all-features`

## 3. Showcase and e2e

- [x] 3.1 The `/checkbox` page (`basic`, `multiple`, `filled`, `sizes`, `indeterminate`, `invalid`, `disabled`), the menu, the CSS `checkbox`
- [x] 3.2 `e2e/tests/pages.json` and `e2e/tests/checkbox.spec.ts`; check `npx playwright test checkbox`

## 4. Benchmarks and budgets

- [x] 4.1 Scenarios `checkbox-1`, `checkbox-1000`, `case-checkbox`, budgets by measurement

## 5. Integration

- [x] 5.1 `docs/checkbox.md`; clippy, tests, deny, machete, `openspec validate port-checkbox`; `prime-checkbox` in the browser CI job; CI on `dev` is green

## Workflow follow-up

- Archive after CI; next are RadioButton, ToggleSwitch, Message, ProgressBar.
