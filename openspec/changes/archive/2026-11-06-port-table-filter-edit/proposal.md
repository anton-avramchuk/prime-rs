# Proposal

## Why

Table stage 1 provides sorting, selection, expansion and a paginator. Working tables still lack filtering, data editing and server-side (lazy) loading. By the user's decision stage 2 is done in three packages; this is **package 1**: filters, cell and row editing, `lazy`.

## What Changes

- Column filters in the row under the header (`filter_display = row`): an input, a match mode (`startsWith`, `contains`, `notContains`, `endsWith`, `equals`, `notEquals`, `lt`, `lte`, `gt`, `gte`), a clear button; two-way signal `filters`.
- Global filter: signal `global_filter` + `global_filter_fields`; `contains` mode.
- Pure model `model.rs`: `FilterMeta`, `MatchMode`, `matches`, `filter_rows`; table-driven tests.
- Cell editing (`edit_mode = cell`): `Column::editable(...)` with an editor; a click on the cell opens the editor, `Enter`/blur completes, `Escape` cancels; events `on_edit_init`, `on_edit_complete`, `on_edit_cancel`.
- Row editing (`edit_mode = row`): a `Column::row_editor()` column with "edit / save / cancel" buttons; events `on_row_edit_init|save|cancel`.
- Data is not mutated: the edit result arrives in an event with the new row, the application updates `value`.
- `lazy`: `lazy`, `total_records`, `on_lazy_load(LazyLoadEvent { first, rows, sort, filters, global_filter })`; with `lazy` the table does not sort, filter or slice pages itself.
- The `/table` page - new demos, e2e, benchmarks, `docs/table.md`.
- **Not included (packages 2-3 and later):** filter menu (`display = menu`) with several conditions, `in`/`between`/date modes, dropdown filter; column resizing, reordering, freezing; grouping, virtual scrolling, row reordering.

## Capabilities

### Modified Capabilities
- `table`: filters, cell and row editing, lazy loading.

## Impact

- `crates/prime-table` (`model.rs`, `lib.rs`, tests), showcase, `e2e`, `benches/*` (recompute budgets), `docs/table.md`.
