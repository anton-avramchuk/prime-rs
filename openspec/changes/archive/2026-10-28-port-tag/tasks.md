# Tasks

## 1. Crate and logic

- [x] 1.1 Create the crate `crates/prime-tag` (`publish = false`, features `ssr`/`hydrate`, dependency `prime-core`) and add it to the workspace; check `cargo check -p prime-tag --all-features`
- [x] 1.2 Implement `TagSeverity`, `TagState`, `write_classes`, `data_p` after `tagstyle.ts` and `Tag.dataP`; verify with a table of unit tests for the spec scenarios (severities, `rounded`, `data-p`)

## 2. Component

- [x] 2.1 Implement `Tag` with two render branches (`all_static!`, `Either`, markup in a macro), the `children`, icon and label blocks, `pt` (`TagPt`); verify with SSR tests of all spec scenarios
- [x] 2.2 Add a parity table (values and signals, HTML matches) and rustdoc with an example; check `cargo test -p prime-tag --all-features` and doc tests

## 3. Showcase and e2e

- [x] 3.1 Add the page `/tag` (`basic`, `icon`, `pill`, `severity`, `template`) and a menu item, include the `tag` CSS in the shell; check `cargo leptos build` and the page in Chrome without console errors
- [x] 3.2 Add entries to `e2e/tests/pages.json` and `e2e/tests/tag.spec.ts` (demos, severity classes, content order); check `npx playwright test tag` locally

## 4. Benchmarks and budgets

- [x] 4.1 Add scenarios `tag-1`, `tag-1000` to `benches/render`, `case-tag` to `benches/wasm-size`, budgets in `benches/alloc-budgets.toml` and `benches/budgets.toml` by measurement; check `cargo bench -p render-bench --bench alloc`, `cargo xtask alloc-budget` and `cargo xtask wasm-size`

## 5. Integration

- [x] 5.1 Write `docs/tag.md` (correspondence to PrimeNG, differences); run `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace --all-features` (except applications), `cargo deny check`, `openspec validate port-tag`; make sure CI on `dev` is green

## Workflow follow-up

- Archive the change after CI passes; the next wave 1 component: Chip.
