# Tasks

## 1. Crate

- [x] 1.1 `crates/prime-breadcrumb`: `BreadcrumbItem` (builder), `BreadcrumbClick`, `BreadcrumbPt`, `click_plan` with a table-driven test
- [x] 1.2 Component `Breadcrumb`: root, list, the "home" item, items, separators, `item` and `separator` templates, clicks; SSR tests for all spec scenarios, rustdoc

## 2. Theme, showcase, e2e

- [x] 2.1 Add `breadcrumb` to `theme.components` (`upstream.toml`), `cargo xtask gen-theme`, make sure the CSS of other components did not change
- [x] 2.2 The `/breadcrumb` page (`basic`, `template`), menu, route, `stylesheet(&[..])`, crate in the workspace and showcase
- [x] 2.3 `e2e/tests/pages.json` and `breadcrumb.spec.ts`; wait for `networkidle`, restart the showcase server; `PW_CHANNEL=chrome npx playwright test breadcrumb`

## 3. Benchmarks and budgets

- [x] 3.1 Scenarios `-1` and `-1000`, `case-breadcrumb` and budgets (first `cargo bench -p render-bench --bench alloc`)

## 4. Integration

- [x] 4.1 `docs/breadcrumb.md` (differences from PrimeNG: no `routerLink`, `BreadcrumbItem` instead of `MenuItem`, `Steps` is not ported, the `router` demo is skipped)
- [x] 4.2 `cargo fmt --all -- --check`, clippy with `--all-features`, tests, deny, machete, `openspec validate port-breadcrumb`; CI `dev` green

## Workflow follow-up

- Archive after CI.
- Next: InputOtp, ScrollTop, Password.
