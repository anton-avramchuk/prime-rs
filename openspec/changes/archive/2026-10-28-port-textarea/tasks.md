# Tasks

## 1. Crate and logic

- [x] 1.1 Create the crate `crates/prime-textarea` (`publish = false`, features `ssr`/`hydrate`, dependencies `prime-core`, `web-sys`), add it to the workspace; check `cargo check -p prime-textarea --all-features`
- [x] 1.2 Implement `TextareaState`, `write_classes` per `textareastyle.ts` and the pure function `fit_height` for the auto-resize; check with a table of unit tests

## 2. Component

- [x] 2.1 Implement `Textarea` (attributes, classes, the value as child text, the variant from `PrimeConfig`, `pt`, two branches); check with SSR tests and parity
- [x] 2.2 Implement the two-way binding, the events and `auto_resize` with `on_resize`; check with `wasm-bindgen-test`
- [x] 2.3 Add rustdoc with an example; check `cargo test -p prime-textarea --all-features`

## 3. Showcase and e2e

- [x] 3.1 The `/textarea` page (`basic`, `autoresize`, `filled`, `sizes`, `invalid`, `disabled`, `fluid`), the menu, the CSS `textarea`
- [x] 3.2 `e2e/tests/pages.json` and `e2e/tests/textarea.spec.ts`; check `npx playwright test textarea`

## 4. Benchmarks and budgets

- [x] 4.1 Scenarios `textarea-1`, `textarea-1000`, `case-textarea`, budgets by measurement

## 5. Integration

- [x] 5.1 `docs/textarea.md`; clippy, tests, deny, machete, `openspec validate port-textarea`; `prime-textarea` in the browser CI job; CI `dev` is green

## Workflow follow-up

- Archive after CI.
