# Tasks

## 1. Crate and components

- [x] 1.1 Create the crate `crates/prime-inputgroup` (`publish = false`, features `ssr`/`hydrate`, dependency `prime-core`), add it to the workspace; check `cargo check -p prime-inputgroup --all-features`
- [x] 1.2 Implement `InputGroup`, `InputGroupPt`, `InputGroupAddon`, `InputGroupAddonPt` (two branches, `children`, `style_class`, `style`); verify with SSR tests (structure, classes, `style`, `pt`, nesting) and branch parity
- [x] 1.3 rustdoc with an example; `cargo test -p prime-inputgroup --all-features`

## 2. Theme, showcase, e2e

- [x] 2.1 Add `inputgroup` to `theme.components` (`upstream.toml`), `cargo xtask gen-theme`, wire up the CSS
- [x] 2.2 Page `/inputgroup` (`basic`, `button`, `checkbox`, `multiple`), menu, route, `stylesheet(&[..])`
- [x] 2.3 `e2e/tests/pages.json` and `e2e/tests/inputgroup.spec.ts`; `npx playwright test inputgroup`

## 3. Benchmarks and budgets

- [x] 3.1 Scenarios `inputgroup-1`, `inputgroup-1000`, `case-inputgroup`, budgets from measurement

## 4. Integration

- [x] 4.1 `docs/inputgroup.md`; clippy, tests, deny, machete, `openspec validate port-inputgroup`; CI `dev` green

## Workflow follow-up

- Archive after CI.
