# Table (stage 2, package 1)

Port of `p-table` (DataTable) from PrimeNG 21.1.10 (commit `fe82b291d6`, see `upstream.toml`). Stage 1 is the core table;
**stage 2, package 1** added filters (per-column and global), cell and row editing, and lazy loading. Column resizing,
reordering and freezing, grouping, virtual scrolling and row reordering are packages 2-3
(list below).

| Our component | PrimeNG sources |
|---|---|
| `prime-table::Table<T>`, `Column<T>`, `ColumnKind`, `TablePt`, `PaginatorPosition`, `PageEvent`, `EditMode`, `EditEvent<T>`, `LazyLoadEvent` | `packages/primeng/src/table/table.ts` (`Table`, `SortableColumn`, `SortIcon`, `TableCheckbox`, `TableHeaderCheckbox`, `TableRadioButton`, `RowToggler`), `table/style/tablestyle.ts` |
| Sorting, selection, expansion, filters | `prime-table::model` - pure logic for `sort`, `multisortField`, `handleRowClick`, `toggleRowsWithCheckbox`, `toggleRow`, `filter`/`executeLocalFilter` (`MatchMode`, `FilterMeta`, `matches`, `filter_rows`), table-driven tests |
| Paginator, page slice | `prime-paginator::Paginator`, `prime_dataview::data::{page, clamp_first}` |
| CSS | `@primeuix/styles/datatable` (`datatable` in `theme.components`) |
| Demos | `apps/showcase/doc/table/*-doc.ts` (`basic`, `singlecolumnsort`, `multiplecolumnssort`, `paginatorbasic`, `singleselection`, `multipleselection`, `checkboxselection`, `radiobuttonselection`, `rowexpansion`, `striped`/`gridlines`/`size`, `verticalscroll`, `loadingmask`; package 1: `filterrow`, `filterglobal`, `celledit`, `rowedit`, `lazyload`) |

Spec: `openspec/specs/table`. Rows are identified by keys (`row_key`, like `dataKey`); selection and expansion store keys.

```rust
let columns = vec![
    Column::text("name", "Name", |p: &Product| p.name.clone()).sortable(|a, b| a.name.cmp(&b.name)),
    Column::text("qty", "Quantity", |p: &Product| p.quantity.to_string()),
];
view! { <Table value=products columns=columns row_key=Callback::new(|p: Product| p.code) paginator=true default_rows=10usize /> }
```

## What matches PrimeNG

- Markup and classes: `p-datatable p-component` (`-hoverable`, `-striped`, `-gridlines`, `-sm`, `-lg`, `-scrollable`, `-flex-scrollable`), `p-datatable-mask`, `p-datatable-header`, `p-datatable-table-container` (`overflow: auto`, `max-height`), `p-datatable-table`, `p-datatable-thead`/`tbody`/`tfoot`, `p-datatable-footer`, `p-datatable-empty-message`, `p-datatable-row-expansion`, `p-datatable-sortable-column` (`-column-sorted`), `p-datatable-sort-icon`, `p-sortable-column-badge`, `p-datatable-selectable-row` (`-row-selected`), `p-datatable-row-toggle-button`.
- Sorting: single (a repeated click flips the order) and multi-level (Ctrl/⌘ adds a column, order counters), `Enter`/`Space` keys on the header, return to the first page on sort.
- Selection: single and multiple by click (with and without `meta_key_selection`), checkbox and radio button columns, a "select all" checkbox (over all rows or only the page).
- Row expansion (including only one row) and expansion content; paginator, `striped_rows`, `show_gridlines`, `size`, vertical scrolling, loading indicator, empty message, `caption`, `summary`, column footer.

### Filters, editing, lazy loading (package 1)

```rust
let columns = vec![
    Column::text("name", "Name", |p: &Product| p.name.clone())
        .filterable(|p| p.name.clone())                       // input in the filter row, default mode startsWith
        .edit_text(|p| p.name.clone(), |p, v| Product { name: v, ..p.clone() }), // cell editor
    Column::text("qty", "Qty", |p: &Product| p.quantity.to_string()).filterable(|p| p.quantity.to_string()).filter_match_mode(MatchMode::Gte),
    Column::row_editor(),
];
view! { <Table value=rows columns=columns row_key=key edit_mode=Some(EditMode::Cell) on_edit_complete=save global_filter=search /> }
```

- Filters: an input for `filterable` columns in the second `thead` row (`p-datatable-filter`), modes `startsWith`, `contains`, `notContains`, `endsWith`, `equals`, `notEquals`, `lt`, `lte`, `gt`, `gte` (numeric ones compare as numbers), a clear button, return to the first page; `filters` is a two-way signal. The global filter is `global_filter` ("contains" mode) over `global_filter_fields` or over columns with filter text (`filterable`/`searchable`).
- Editing: `edit_mode = Cell` - a click on an `editable` cell opens the editor (`p-editable-column`, `p-cell-editing`), `Enter`/blur completes, `Escape` cancels; `edit_mode = Row` - a `row_editor` column with "edit / save / cancel" buttons (`p-datatable-row-editor-*`), `editing_keys` is a two-way signal.
- `lazy`: sorting, filters and pages are handled by the application via `on_lazy_load(LazyLoadEvent)`; the number of records is `total_records`.

## Differences

| Difference | Reason |
|---|---|
| Columns are described by data (`Column<T>`: `field`, `header`, `body`, `sortable` + `compare`, `footer`), not by `ng-template` templates with `pSortableColumn`, `pSelectableRow` directives | Leptos has no Angular templates; the table builds the header, rows and footer itself. |
| Sorting uses the column's compare function, not a field name (`resolveFieldData`) | Rows are typed. |
| A copy of the data is sorted, `value` is not changed | Application signals are not mutated. |
| Selection stores row keys (`Vec<String>`), not objects; `row_key` is required | Type `T` is arbitrary, the key is set by the application. |
| Editing does not change `value`: the result arrives in `EditEvent { original, row, field }`, the application updates the data itself; the editor receives a draft of the row (`RwSignal<T>`) | Application data is not mutated. |
| A column filter is only a `thead` row with an input (`display="row"`), the match mode is set by `Column::filter_match_mode` without a menu; the filter applies immediately, without `filterDelay` | The filter menu is for later packages. |
| The filter value is taken by the column function (`filterable`/`searchable`), not `resolveFieldData` | Rows are typed. |
| No Shift range selection, `selectionAutoFocus`, context menu, `rowSelectable`, `compareSelectionBy` | Stage 2+. |
| No `removableSort` | It is absent from the 21.1.10 source too. |
| No `stateStorage`, `customSort`, `resetScrollTop`, `getBlockableElement` | Stage 2+ / outside the port scope. |

## Not included

Filter menu (`display="menu"`) with several conditions and AND/OR operators, modes `in`, `between`, `is`, `before`/`after`, date ones, dropdown filter and `filterDelay`; default template editors (number, date, list), `onEditInit` with cancellation, Tab key between cells; column resizing and reordering, row dragging, freezing columns and rows, virtual scrolling, row grouping (including by `rowspan`), `lazyLoadOnInit`, CSV export, column toggle menu, state saving, `p-table` inside `ContextMenu`, keyboard navigation over rows.

## Not verified

- Screen reading: `aria-*` were checked against PrimeNG, screen reader behavior was not.
- Tables with thousands of rows (without virtual scrolling they are built whole).

## Performance

See `benches/` (`table-1`, `table-1000`, `case-table`) and `benches/*budgets.toml`.
