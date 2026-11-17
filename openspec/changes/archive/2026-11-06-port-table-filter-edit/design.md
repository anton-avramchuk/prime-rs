# Design

## Context

Requirements are in `specs/table`. PrimeNG 21.1.10 sources: `table/table.ts` (`filter`, `filterGlobal`, `_filter`, `executeLocalFilter`, `ColumnFilter`, `EditableColumn`, `CellEditor`, `EditableRow`, `RowEditor`, `initRowEdit/saveRowEdit/cancelRowEdit`, `createLazyLoadMetadata`), `api/filtermatchmode.ts`, `table/style/tablestyle.ts`.

## Decisions

### D1. Pure filtering in `model.rs`
`FilterMeta { field, value, match_mode }`, `MatchMode` (ten modes), `matches(text, value, mode)`, `set_filter`, `filter_rows(rows, filters, global, fields, text_of)`. The field value is taken by the column function (`Column::filterable(text_of)`). Case is ignored; numeric modes parse both values as `f64`.

### D2. Filters are column data
`Column::filterable(text_of)` sets the value function; `filter_match_mode` sets the mode. The filter row is built under the header, the field is `input.p-inputtext`. Signal `filters: RwSignal<Vec<FilterMeta>>`.

### D3. Editing without mutation
`Column::editable(editor)`, where the editor is a `Callback<RwSignal<T>, AnyView>` and the signal is the row draft; the editor writes to the draft, completion hands the application a new row. The ready-made `Column::edit_text(get, set)` is a text field. Cell editing is a single key+field pair in the state; row editing uses `editing_keys` and drafts by key.

### D4. Lazy
`lazy` disables the internal sorting, filtering and slicing; `LazyLoadEvent` is assembled from the signals `first`, `rows`, `sort`, `filters`, `global_filter`; an `Effect` under `cfg(wasm32)` calls `on_lazy_load` on change, including on mount.

### D5. Tests
Model: tables. SSR: the filter row, filtering, the initial state of editors, lazy. Browser: input, clear, cell editing (Enter/Escape/blur), row editing, lazy. e2e and benchmarks as for the other components.

## Risks / Trade-offs

- [Size] No new dependencies: the fields are plain `input` elements with `p-inputtext` classes.
- [Difference] No filter menu, no `in`/`between`/date modes - later.

## Migration Plan

No migration: the new properties are optional. Rollback is `git revert`.
