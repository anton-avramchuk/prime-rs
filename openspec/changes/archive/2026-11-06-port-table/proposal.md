# Proposal

## Why

Table (DataTable) is the main PrimeNG data component: a table with sorting, selection, row expansion and a paginator. The second component of wave 7. By the user's decision **stage 1** (the core table) is done; filters, editing, column resizing and reordering, freezing, virtual scrolling and grouping are stages 2+.

## What Changes

- New crate `prime-table` (port of `p-table` from PrimeNG 21.1.10, stage 1): generic `Table<T>`.
- Columns are described by data: `Column<T>` (`field`, `header`, `footer`, `body`, `sortable` + `compare`, `style`, `style_class`, `kind`), kinds: data, selection checkbox, selection radio button, expansion button.
- Pure model `model.rs`: `toggle_sort` (single and multi-level sorting, Ctrl/⌘), `sort_rows` (stable, multi-level), `row_click` (selection by click, `meta_key_selection`), `toggle_all`, `all_selected`, `toggle_expanded`; table-driven tests.
- Markup: `div.p-datatable.p-component` (`-hoverable`, `-striped`, `-gridlines`, `-sm`, `-lg`, `-scrollable`, `-flex-scrollable`) > `p-datatable-mask`, `p-datatable-header`, paginator `p-datatable-paginator-top|bottom`, `p-datatable-table-container` > `table.p-datatable-table` (`thead`, `tbody`, `tfoot`), `p-datatable-footer`.
- Behavior: sorting by click and `Enter`/`Space` on the header (icons, order counters, `aria-sort`), a paginator that returns to the first page on sort, row selection by click (single, multiple, with Ctrl/⌘), checkboxes and radio buttons, the "select all" checkbox, row expansion (including only one row), an empty message, a loading indicator.
- The `/table` page, e2e, benchmarks and budgets, `docs/table.md`; `datatable` in `theme.components` and `primeng_extra`; `xtask gen-theme` supports a sources directory different from the styles module name (`datatable` -> `table`).
- **Not included (stages 2+):** filters, cell and row editing, column resizing and reordering, row dragging, freezing, virtual scrolling, grouping, `lazy`, export, column menu, state, Shift range selection, keyboard navigation over rows.

## Capabilities

### New Capabilities
- `table`: data table with sorting, selection, row expansion and a paginator.

### Modified Capabilities
<!-- None. -->

## Impact

- New crate `crates/prime-table` (`publish = false`); dependencies: `prime-core`, `prime-dataview` (page slice), `prime-paginator`, `prime-checkbox`, `prime-radiobutton`, `prime-badge`, `prime-button`, `prime-icons`.
- Changed: workspace `Cargo.toml`, `upstream.toml`, `xtask/src/gen_theme.rs`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml`.
