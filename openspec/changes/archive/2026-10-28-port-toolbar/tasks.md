# Tasks

## 1. Crate and component

- [x] 1.1 Create the crate `crates/prime-toolbar` (`publish = false`, features `ssr`/`hydrate`, dependency `prime-core`), add it to the workspace; check `cargo check -p prime-toolbar --all-features`
- [x] 1.2 Implement `Toolbar` and `ToolbarPt` (slots, `role`, `aria-labelledby`, two branches, `children` before the areas); check with SSR tests (structure, order, empty areas, attributes, `style`, `pt`) and branch parity
- [x] 1.3 rustdoc with an example; `cargo test -p prime-toolbar --all-features`

## 2. Theme, showcase, e2e

- [x] 2.1 Add `toolbar` to `theme.components` (`upstream.toml`), `cargo xtask gen-theme`, connect the CSS
- [x] 2.2 The `/toolbar` page (`basic`, `custom`), the menu, the route, `stylesheet(&[..])`
- [x] 2.3 `e2e/tests/pages.json` and `e2e/tests/toolbar.spec.ts`; `npx playwright test toolbar`

## 3. Benchmarks and budgets

- [x] 3.1 Scenarios `toolbar-1`, `toolbar-1000`, `case-toolbar`, budgets by measurement

## 4. Integration

- [x] 4.1 `docs/toolbar.md`; clippy, tests, deny, machete, `openspec validate port-toolbar`; CI `dev` is green

## Workflow follow-up

- Archive after CI.
