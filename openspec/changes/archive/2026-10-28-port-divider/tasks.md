# Tasks

## 1. Crate and logic

- [x] 1.1 Create the crate `crates/prime-divider` (`publish = false`, features `ssr`/`hydrate`, dependency `prime-core`) and add it to the workspace; check `cargo check -p prime-divider --all-features`
- [x] 1.2 Implement `DividerLayout`, `DividerType`, `DividerAlign`, `DividerState`, `write_classes`, `align_style`, `data_p` after `dividerstyle.ts` and `Divider.dataP`; verify with unit test tables all `layout` x `align` combinations (classes, style) and `data-p`

## 2. Component

- [x] 2.1 Implement `Divider` with two rendering branches, the `p-divider-content` block, `role`, `aria-orientation`, the inline style and `pt` (`DividerPt`); verify all spec scenarios with SSR tests
- [x] 2.2 Add a branch parity table and rustdoc with an example; check `cargo test -p prime-divider --all-features` and doc tests

## 3. Showcase and e2e

- [x] 3.1 Add the `/divider` page (`basic`, `content`, `type`, `vertical`; `login` is not ported: it needs InputText), a menu item and the `divider` CSS to the shell; check `cargo leptos build` and the page in Chrome with no console errors
- [x] 3.2 Add entries to `e2e/tests/pages.json` and `e2e/tests/divider.spec.ts` (classes, `role`, style, content); check `npx playwright test divider` locally

## 4. Benchmarks and budgets

- [x] 4.1 Add the scenarios `divider-1`, `divider-1000` to `benches/render`, `case-divider` to `benches/wasm-size`, budgets from measurement; check `cargo bench -p render-bench --bench alloc`, `cargo xtask alloc-budget` and `cargo xtask wasm-size`

## 5. Integration

- [x] 5.1 Write `docs/divider.md` (correspondence with PrimeNG, differences); run `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace --all-features` (except apps), `cargo deny check`, `openspec validate port-divider`; make sure CI on `dev` is green

## Workflow follow-up

- Archive the change after CI passes; the next wave 1 component: Card.
