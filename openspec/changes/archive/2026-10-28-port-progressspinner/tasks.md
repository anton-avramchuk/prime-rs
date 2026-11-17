# Tasks

## 1. Crate and component

- [x] 1.1 Create the crate `crates/prime-progressspinner` (`publish = false`, features `ssr`/`hydrate`, dependency `prime-core`), add it to the workspace; check `cargo check -p prime-progressspinner --all-features`
- [x] 1.2 Implement `ProgressSpinner` and `ProgressSpinnerPt` (markup, SVG, ARIA, `stroke_width`, `fill`, `animation_duration`, two branches); SSR tests: structure, default values, custom values, `aria-label`, `style`, `pt`, branch parity, reactive update
- [x] 1.3 rustdoc with an example; `cargo test -p prime-progressspinner --all-features`

## 2. Theme, showcase, e2e

- [x] 2.1 Add `progressspinner` to `theme.components` (`upstream.toml`), `cargo xtask gen-theme`, connect the CSS
- [x] 2.2 The `/progressspinner` page (`basic`, `custom`), the menu, the route, `stylesheet(&[..])`
- [x] 2.3 `e2e/tests/pages.json` and `e2e/tests/progressspinner.spec.ts` (markup, animation, absence of hydration errors); `npx playwright test progressspinner`

## 3. Benchmarks and budgets

- [x] 3.1 Scenarios `progressspinner-1`, `progressspinner-1000`, `case-progressspinner`, budgets by measurement

## 4. Integration

- [x] 4.1 `docs/progressspinner.md`; clippy, tests, deny, machete, `openspec validate port-progressspinner`; CI on `dev` is green

## Workflow follow-up

- Archive after CI.
