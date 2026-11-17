# Design

## Context

Requirements are in `specs/dataview`. PrimeNG 21.1.10 sources: `dataview/dataview.ts` (`paginate`, `sort`, `filter`, `isEmpty`), `style/dataviewstyle.ts`, `@primeuix/styles/dataview`. The paginator is the ready-made `prime-paginator`.

## Decisions

### D1. Pure logic (`data.rs`)
`prepare(items, filter, compare, order)` - filter and stable sort; `page(...)` - page slice (everything in `lazy`); `total(...)`; `clamp_first(...)`. Table-driven tests.

### D2. Generic component
`DataView<T: Clone + Send + Sync + 'static>`; `list` and `grid` are `Callback<Vec<T>, AnyView>` (page items). Sorting is `compare: Callback<(T, T), Ordering>` and `sort_order`; the filter is `Callback<T, bool>` (reads its own signals, hence reactive).

### D3. State
`first` and `rows` are signals (external or internal). Effects under `cfg(wasm32)`: returning `first` to the valid range, the initial `on_lazy_load`, reset to the first page and `on_lazy_load` on order change.

### D4. Tests
Logic: tables. SSR: lists, grid, pages and paginator positions, sorting, filter, empty, loading, `lazy`. Browser: page switching, `first` returning, sorting and filter, `lazy`, empty state. e2e: styles and clicks.

## Risks / Trade-offs

- [first normalization] The paginator may report a page again when `first` is returned.

## Migration Plan

No migration. Rollback is `git revert`.
