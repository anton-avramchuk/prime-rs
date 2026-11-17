# Tasks

## 1. Model

- [x] 1.1 `model.rs`: `MatchMode`, `FilterMeta`, `matches`, `filter_rows`, table-driven tests
- [x] 1.2 Check against the source's `executeLocalFilter`, `FilterMatchMode`, `createLazyLoadMetadata`

## 2. Component

- [x] 2.1 Column and global filters: `Column::filterable`, the filter row, signals, events
- [x] 2.2 Cell editing: `Column::editable`, `edit_text`, events
- [x] 2.3 Row editing: `Column::row_editor`, `editing_keys`, events
- [x] 2.4 `lazy`, `total_records`, `on_lazy_load`
- [x] 2.5 SSR tests and browser `tests/web.rs`

## 3. Showcase and e2e

- [x] 3.1 Demos on `/table`: filters, global filter, cell editing, row editing, lazy
- [x] 3.2 `e2e/tests/table.spec.ts`

## 4. Benchmarks and budgets

- [x] 4.1 Recompute the `table-1`, `table-1000`, `case-table`, wasm budgets

## 5. Integration

- [x] 5.1 `docs/table.md`, `openspec/specs/table`
- [x] 5.2 fmt, clippy, tests, deny, machete, `openspec validate port-table-filter-edit`
