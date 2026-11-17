# Tasks

## 1. Crate and logic

- [x] 1.1 Create the crate `crates/prime-radiobutton` (`publish = false`, features `ssr`/`hydrate`, dependencies `prime-core`, `web-sys`), add it to the workspace; check `cargo check -p prime-radiobutton --all-features`
- [x] 1.2 Implement `RadioButtonState`, `write_classes`, `data_p` per `radiobuttonstyle.ts` and `RadioButton.dataP`; check with a table of unit tests

## 2. Component

- [x] 2.1 Implement `RadioButton` (markup, classes, attributes, `pt`, two branches); check with SSR tests and parity
- [x] 2.2 Implement the selection through `model`, `disabled` and events; check with `wasm-bindgen-test`
- [x] 2.3 Add rustdoc with an example; check `cargo test -p prime-radiobutton --all-features`

## 3. Showcase and e2e

- [x] 3.1 The `/radiobutton` page (`group`, `filled`, `sizes`, `invalid`, `disabled`), the menu, the CSS `radiobutton`
- [x] 3.2 `e2e/tests/pages.json` and `e2e/tests/radiobutton.spec.ts`; check `npx playwright test radiobutton`

## 4. Benchmarks and budgets

- [x] 4.1 Scenarios `radiobutton-1`, `radiobutton-1000`, `case-radiobutton`, budgets by measurement

## 5. Integration

- [x] 5.1 `docs/radiobutton.md`; clippy, tests, deny, machete, `openspec validate port-radiobutton`; `prime-radiobutton` in the browser CI job; CI on `dev` is green

## Workflow follow-up

- Archive after CI; next are ToggleSwitch, Message, ProgressBar.
