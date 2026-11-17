# table Specification

## Purpose
Table is a data table with sorting, selection, row expansion and a paginator.

## ADDED Requirements

### Requirement: Sorting
A click on the header of a sortable column in `single` mode SHALL sort by it: the first click uses the default order (1), a repeated click the reverse; a click on another column resets the previous one. In `multiple` mode a click without Ctrl/⌘ SHALL leave a single column, with Ctrl/⌘ it SHALL add a column or change its order. Sorting SHALL be stable and multi-level, `value` is not changed. The header SHALL have `p-datatable-sortable-column` (`-column-sorted` on the sorted one), `tabindex=0`, `aria-sort` (`ascending`/`descending`/`none`), the `p-datatable-sort-icon` icon, the `p-sortable-column-badge` counter with several columns; `Enter`/`Space` SHALL sort the same way as a click. Sorting SHALL return to the first page.

#### Scenario: Cycle
- **WHEN** the "Name" header is clicked twice
- **THEN** the order is ascending, then descending by name; `aria-sort` changes

#### Scenario: Several columns
- **WHEN** click on "Category", Ctrl-click on "Quantity"
- **THEN** rows are sorted by category, then by quantity; the headers have counters 1 and 2

### Requirement: Selection
With `selection_mode` set, a click on a row SHALL change the selection: `single` replaces it or clears it on a repeated click, `multiple` toggles the row, with `meta_key_selection` the selection is replaced without Ctrl/⌘, extended with it and a selected row is cleared with it. Rows SHALL have `p-datatable-selectable-row` (`-row-selected`) and `aria-selected`. The checkbox column SHALL toggle the row, the header checkbox SHALL select and clear all rows (or only the page's with `selection_page_only`) and be shown checked when all are selected; the radio button column selects one row. A click on a checkbox, radio button or button inside a row SHALL NOT select the row again.

#### Scenario: Header
- **WHEN** click on the header checkbox
- **THEN** all rows are selected, the checkbox is checked; a repeated click clears the selection

#### Scenario: Ctrl
- **WHEN** `meta_key_selection`, row 3 is selected, Ctrl-click on row 1
- **THEN** rows 3 and 1 are selected

### Requirement: Expansion and pages
The expansion button SHALL add a `tr.p-datatable-row-expansion` after the row with the `row_expansion` content (in `row_expand_single` mode only for one row) and call events. The paginator SHALL show rows `[first, first + rows)` of the sorted data, `first` out of range returns to the last page. When there are no rows the `p-datatable-empty-message` message SHALL be shown (if not loading), with `loading` a mask with an icon.

#### Scenario: Expansion
- **WHEN** click on the expansion button of the second row
- **THEN** right after it the expansion row spanning the full table width

#### Scenario: Page
- **WHEN** five rows, `rows = 2`, go to the second page
- **THEN** the third and fourth rows are shown
