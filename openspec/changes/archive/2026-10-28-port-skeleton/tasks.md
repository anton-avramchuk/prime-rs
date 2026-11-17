# Tasks

## 1. Crate and component

- [x] 1.1 Add `skeleton` and `textarea` to `[theme] components` (`upstream.toml`) and run `cargo xtask gen-theme`; create the crate `crates/prime-skeleton` (`publish = false`, features `ssr`/`hydrate`, dependency `prime-core`), add it to the workspace; check `cargo check -p prime-skeleton --all-features`
- [x] 1.2 Implement `Skeleton`, `SkeletonShape`, `SkeletonAnimation`, the classes, the style, `data-p`, `pt`, two branches, rustdoc with an example and a unit test of the class table; check with SSR tests and parity

## 2. Showcase and e2e

- [x] 2.1 The `/skeleton` page (`shapes`, `card`, `list`), the menu, the CSS `skeleton`; check `cargo leptos build` and the page in Chrome
- [x] 2.2 `e2e/tests/pages.json` and `e2e/tests/skeleton.spec.ts`; check `npx playwright test skeleton`

## 3. Benchmarks and budgets

- [x] 3.1 Scenarios `skeleton-1`, `skeleton-1000`, `case-skeleton`, budgets by measurement

## 4. Integration

- [x] 4.1 `docs/skeleton.md`; clippy, tests, deny, machete, `openspec validate port-skeleton`; CI `dev` is green

## Workflow follow-up

- Archive after CI; next is Textarea.
