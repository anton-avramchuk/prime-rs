# Tasks

## 1. Crate and logic

- [x] 1.1 Create the crate `crates/prime-toggleswitch` (`publish = false`, features `ssr`/`hydrate`, dependencies `prime-core`, `web-sys`), add it to the workspace; check `cargo check -p prime-toggleswitch --all-features`
- [x] 1.2 Implement `ToggleSwitchState`, `write_classes`, `data_p` per `toggleswitchstyle.ts` and `ToggleSwitch.dataP`; check with a table of unit tests

## 2. Component

- [x] 2.1 Implement `ToggleSwitch` (markup, classes, the root style, attributes, `pt`, the slot, two branches); check with SSR tests and parity
- [x] 2.2 Implement the click on the root, `readonly`/`disabled`, the return of focus and the events; check with `wasm-bindgen-test`
- [x] 2.3 Add rustdoc with an example; check `cargo test -p prime-toggleswitch --all-features`

## 3. Showcase and e2e

- [x] 3.1 The `/toggleswitch` page (`basic`, `preselection`, `template`, `invalid`, `disabled`), the menu, the CSS `toggleswitch`
- [x] 3.2 `e2e/tests/pages.json` and `e2e/tests/toggleswitch.spec.ts`; check `npx playwright test toggleswitch`

## 4. Benchmarks and budgets

- [x] 4.1 Scenarios `toggleswitch-1`, `toggleswitch-1000`, `case-toggleswitch`, budgets by measurement

## 5. Integration

- [x] 5.1 `docs/toggleswitch.md`; clippy, tests, deny, machete, `openspec validate port-toggleswitch`; `prime-toggleswitch` in the browser CI job; CI `dev` is green

## Workflow follow-up

- Archive after CI; next are Message, ProgressBar.
