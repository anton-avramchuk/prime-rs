# Tasks

## 1. Foundation

- [x] 1.1 `dataview` in `theme.components`, `cargo xtask gen-theme`
- [x] 1.2 Check against the source's `paginate`, `sort`, `filter`, `isEmpty`, template

## 2. Crate `prime-dataview`

- [x] 2.1 Module `data.rs` with table-driven tests
- [x] 2.2 Component `DataView<T>`, `DataViewPt`: markup, paginator, empty state, loading, SSR tests, rustdoc
- [x] 2.3 Events `on_page`, `on_lazy_load`, `first` returning, sorting, `lazy`
- [x] 2.4 Browser `tests/web.rs`

## 3. Showcase and e2e

- [x] 3.1 The `/dataview` page, menu, route, crate in the workspace and showcase
- [x] 3.2 `e2e/tests/pages.json` and `dataview.spec.ts`

## 4. Benchmarks and budgets

- [x] 4.1 Scenarios `-1` and `-1000`, `case-dataview`, budgets

## 5. Integration

- [x] 5.1 `docs/dataview.md`; `prime-dataview` in the browser job of `ci.yml` (three places)
- [x] 5.2 fmt, clippy, tests, deny, machete, `openspec validate port-dataview`

## Workflow follow-up

- Wave 7: after each component - commit and archive locally, CI once at the end of the wave.
