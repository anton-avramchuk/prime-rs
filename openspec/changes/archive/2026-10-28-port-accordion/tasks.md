# Tasks

## 1. Shared logic and theme

- [x] 1.1 `prime-rs-core::collapsible::bind_region_with` (binding to a derived state), `bind_region` through it; check that the Panel and Fieldset tests (`wasm-bindgen-test`) pass unchanged
- [x] 1.2 `xtask gen-theme`: the `/* For PrimeNG */` block from `<name>style.ts` with `dt()` resolution; check that the CSS of other components did not change (`git diff`), and with a unit test of block parsing

## 2. Crate and components

- [x] 2.1 Create the crate `crates/prime-accordion` (`publish = false`, features `ssr`/`hydrate`, dependencies `prime-rs-core`, `prime-icons` with `chevrondown`, `chevronup`, `web-sys`), add to the workspace; check `cargo check -p prime-accordion --all-features`
- [x] 2.2 `toggle_value` and neighbouring header selection (`state.rs`) with tables of unit tests
- [x] 2.3 `Accordion`, `AccordionPanel`, `AccordionHeader`, `AccordionContent`, `AccordionPt`, context, classes and ARIA; check with SSR tests (structure, classes, ARIA, `pt`, icon slot, initial hidden state)
- [x] 2.4 Click, keyboard, `select_on_focus`, `on_open`/`on_close`, animation; check with `wasm-bindgen-test` (single mode and `multiple`, disabled panel, arrows with skipping and wrapping, Home/End, animation class, descendants' `tabindex`)
- [x] 2.5 rustdoc with an example; `cargo test -p prime-accordion --all-features`

## 3. Showcase and e2e

- [x] 3.1 Add `accordion` to `theme.components` (`upstream.toml`), `cargo xtask gen-theme`, attach the CSS
- [x] 3.2 `/accordion` page (`basic`, `multiple`, `disabled`, `controlled`, `dynamic`, `template`), menu, route, `stylesheet(&[..])`
- [x] 3.3 `e2e/tests/pages.json` and `e2e/tests/accordion.spec.ts`; `npx playwright test accordion`

## 4. Benchmarks and budgets

- [x] 4.1 Scenarios `accordion-1`, `accordion-1000` (three panels per instance), `case-accordion`, budgets from measurement

## 5. Integration

- [x] 5.1 `docs/accordion.md`; clippy, tests, deny, machete, `openspec validate port-accordion`; `prime-accordion` in the browser job of CI; CI `dev` is green

## Workflow follow-up

- Archive after CI.
