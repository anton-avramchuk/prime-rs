# Tasks

## 1. Crate and logic

- [x] 1.1 Create the crate `crates/prime-fieldset` (`publish = false`, features `ssr`/`hydrate`, dependencies `prime-rs-core`, `prime-icons` with `minus`, `plus`, `web-sys`), add it to the workspace; check `cargo check -p prime-fieldset --all-features`
- [x] 1.2 Implement `write_root_classes`, `data_p` (`state.rs`) with a table of unit tests

## 2. Component

- [x] 2.1 Implement `Fieldset` and `FieldsetPt` (the `macro_rules!` markup, two branches, the legend, the button, the icons and slots, `bind_region`); check with SSR tests (structure, classes, ARIA, slots, `pt`) and branch parity
- [x] 2.2 Toggling, `on_before_toggle`, `on_after_toggle`; check with `wasm-bindgen-test` (click, signal, `aria-expanded`, animation, `tabindex`)
- [x] 2.3 rustdoc with an example; `cargo test -p prime-fieldset --all-features`

## 3. Theme, showcase, e2e

- [x] 3.1 Add `fieldset` to `theme.components` (`upstream.toml`), `cargo xtask gen-theme`, connect the CSS
- [x] 3.2 The `/fieldset` page (`basic`, `toggleable`, `template`), the menu, the route, `stylesheet(&[..])`
- [x] 3.3 `e2e/tests/pages.json` and `e2e/tests/fieldset.spec.ts`; `npx playwright test fieldset`

## 4. Benchmarks and budgets

- [x] 4.1 Scenarios `fieldset-1`, `fieldset-1000`, `case-fieldset`, budgets by measurement

## 5. Integration

- [x] 5.1 `docs/fieldset.md`; clippy, tests, deny, machete, `openspec validate port-fieldset`; `prime-fieldset` in the browser CI job; CI on `dev` is green

## Workflow follow-up

- Archive after CI.
