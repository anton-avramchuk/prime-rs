# Tasks

## 1. Foundation

- [x] 1.1 `datatable` in `theme.components` and `primeng_extra`; `xtask gen-theme`: the `table` sources directory; `cargo xtask gen-theme`
- [x] 1.2 Check against the source's `sort`, `handleRowClick`, `toggleRowsWithCheckbox`, `toggleRow`, `SortableColumn`, `SortIcon`

## 2. Crate `prime-table`

- [x] 2.1 Model `model.rs` with table-driven tests
- [x] 2.2 Component `Table<T>`, `Column<T>`, `TablePt`: markup, header, rows, footer, paginator, SSR tests, rustdoc
- [x] 2.3 Sorting (single, multi-level, keys), selection (click, checkboxes, radio, header), expansion, events
- [x] 2.4 Browser `tests/web.rs`

## 3. Showcase and e2e

- [x] 3.1 The `/table` page, menu, route, crate in the workspace and showcase
- [x] 3.2 `e2e/tests/pages.json` and `table.spec.ts`

## 4. Benchmarks and budgets

- [x] 4.1 Scenarios `-1` and `-1000`, `case-table`, budgets

## 5. Integration

- [x] 5.1 `docs/table.md`; `prime-table` in the browser job of `ci.yml` (three places)
- [x] 5.2 fmt, clippy, tests, deny, machete, `openspec validate port-table`

## Workflow follow-up

- Wave 7: after each component - commit and archive locally, CI once at the end of the wave.
