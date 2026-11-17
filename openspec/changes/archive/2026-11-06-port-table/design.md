# Design

## Context

Requirements are in `specs/table`. PrimeNG 21.1.10 sources: `table/table.ts` (6613 lines; stage 1 takes `sort`, `sortSingle`, `sortMultiple`, `handleRowClick`, `toggleRowWithCheckbox`, `toggleRowsWithCheckbox`, `toggleRow`, `SortableColumn`, `SortIcon`), `table/style/tablestyle.ts` (classes and the `For PrimeNG` block), `@primeuix/styles/datatable`.

## Decisions

### D1. Columns are data, not templates
In PrimeNG the header and body are assembled by `ng-template` templates with directives. Here we have `Column<T>` (field, header, cell function, sorting with `compare`, column kind); the table builds the header, rows and footer itself. Generic `Table<T: Clone + Send + Sync + 'static>`.

### D2. Pure model (`model.rs`)
Sorting (`toggle_sort`, `sort_rows`), selection (`row_click`, `toggle_all`, `all_selected`) and expansion (`toggle_expanded`) are functions over `String` keys. The row key is set by the application (`row_key`), like `dataKey`.

### D3. State
Signals `selection`, `expanded_keys`, `sort`, `first`, `rows` (external or internal). Headers, rows and the footer are built in reactive closures; sort classes and icons read the signal inside the cell, so a click on a header does not rebuild the table, only the row order changes. The header checkbox and radio buttons are synced with `selection` by effects under `cfg(wasm32)`; row checkboxes are `Checkbox` with `model` = `selection` and `value` = the key.

### D4. Theme
`datatable` in `theme.components` and `primeng_extra` (the `For PrimeNG` block gives header cell padding, stripes and sorting). `xtask gen-theme` gained `primeng_dir`: the `table` sources directory instead of `datatable`.

### D5. Tests
Model: tables. SSR: classes, headers and icons, row order, paginator, selection, checkboxes/radio, expansion, empty/loading/footer. Browser: sorting, selection, checkboxes, radio, Ctrl, expansion, pages. e2e: theme styles and clicks.

## Risks / Trade-offs

- [Scope] Stage 1 is the foundation; extensions (filters, editing, etc.) are added as new columns/properties without changing the model.
- [Size] The table pulls in Paginator (Select, InputNumber), Checkbox, RadioButton, Badge, Button: about 1.9 MB of wasm.

## Migration Plan

No migration. Rollback is `git revert`.
