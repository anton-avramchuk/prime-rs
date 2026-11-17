# DataView

Port of `p-dataview` from PrimeNG 21.1.10 (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-dataview::DataView<T>`, `DataViewPt`, `DataViewLayout`, `PaginatorPosition`, `LazyLoadEvent`, `PageEvent` | `packages/primeng/src/dataview/dataview.ts`, `dataview/style/dataviewstyle.ts` |
| Filter, sort, page, total | `prime-dataview::data` - pure logic for `filter`, `sort`, `slice`, `updateTotalRecords`, table-driven tests |
| Paginator | `prime-paginator::Paginator` (`p-paginator` in PrimeNG) |
| CSS | `@primeuix/styles/dataview` (`dataview` in `theme.components`) |
| Demos | `apps/showcase/doc/dataview/*-doc.ts` (`basic`, `layout`, `pagination`, `sorting`, `loading`) |

Spec: `openspec/specs/dataview`. The component is generic over the item type; `list` and `grid` are functions that receive the items of the current page.

```rust
let list = Callback::new(|page: Vec<Product>| view! { /* rows */ }.into_any());
view! { <DataView value=products list=list paginator=true default_rows=10usize /> }
```

## What matches PrimeNG

- Markup: `p-dataview p-component p-dataview-list|grid`, `p-dataview-loading` (mask and icon), `p-dataview-header`, paginator `p-dataview-paginator-top|bottom`, `p-dataview-content`, `p-dataview-empty-message`, `p-dataview-footer`.
- `lazy` mode (the value is already a page, `total_records` for the paginator, `on_lazy_load` on creation, page change and sort change), `paginator_position`, `always_show_paginator`, `rows_per_page_options` and the other paginator settings.
- Sorting returns to the first page; `first` out of range returns to the last page.

## Differences

| Difference | Reason |
|---|---|
| Sorting is `compare: Callback<(T, T), Ordering>` and `sort_order` instead of `sortField`; the filter is a predicate `filter: Callback<T, bool>` instead of `filterBy`/`filterLocale` | Items are typed, there is no access to a field by name (`resolveFieldData`). |
| Data is not sorted in place: the order is computed when shown | Application signals are not mutated. |
| `list` and `grid` are functions of the item page, not `ng-template` | Leptos has no Angular templates. |
| No `trackBy`, `gridStyleClass`, `loadingIcon` (there is one loading icon), `getBlockableElement`, `onChangeLayout`/`onSort` events | Outside the port scope: the application knows about layout and order changes itself. |
| `on_page` may arrive again if the paginator normalizes an out-of-range `first` | The paginator reports the normalization. |

## Not verified

- Screen reading: `aria-*` were checked against PrimeNG, screen reader behavior was not.
- Operation in `lazy` mode with a real server (verified through events).

## Performance

See `benches/` (`dataview-1`, `dataview-1000`, `case-dataview`) and `benches/*budgets.toml`.
