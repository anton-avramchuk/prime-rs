## ADDED Requirements

### Requirement: Filters
A column with `filterable` SHALL get a second `tr` row in `thead` with a `p-datatable-filter` cell containing an input and a clear button. Input SHALL set the column filter with a match mode (default `startsWith`; `Column::filter_match_mode`), keep the rows whose field value satisfies all filters, return to the first page and call `on_filter`. An empty value SHALL clear the filter. String comparison SHALL be case-insensitive, numeric modes compare by number. The global filter `global_filter` SHALL keep the rows in which at least one of `global_filter_fields` contains the value. The paginator SHALL work over the filtered data; when no rows match, the empty table message SHALL be shown.

#### Scenario: Column filter
- **WHEN** "bam" is entered in the "Name" column input
- **THEN** only the rows whose name starts with "bam" remain, the page is the first

#### Scenario: Global filter
- **WHEN** `global_filter` = "acc", fields are name and category
- **THEN** the rows where "acc" occurs in any of the fields remain

### Requirement: Cell editing
In `edit_mode = cell` mode a cell of an `editable` column SHALL have the class `p-editable-column`; a click on it SHALL open the editor (class `p-cell-editing`) and call `on_edit_init`. `Enter` or blur SHALL complete editing with the `on_edit_complete` event carrying the new row and close the editor; `Escape` SHALL cancel editing with the `on_edit_cancel` event. The `value` data is not changed.

#### Scenario: Completion
- **WHEN** click on the cell, input "X", `Enter`
- **THEN** the complete event contains the row with the new value, the editor is closed

### Requirement: Row editing
In `edit_mode = row` mode a `row_editor` column SHALL show an "edit" button; pressing it SHALL open the editors of the `editable` columns in the row and show "save" and "cancel" buttons. Saving SHALL call `on_row_edit_save` with the new row, cancelling SHALL call `on_row_edit_cancel` without changes. The keys of the rows being edited SHALL be stored in the `editing_keys` signal.

#### Scenario: Saving
- **WHEN** "edit" is pressed, the value is changed, "save" is pressed
- **THEN** `on_row_edit_save` is called with the new row, the row left editing

### Requirement: Lazy loading
With `lazy` the table SHALL NOT sort, filter or slice pages itself, SHALL show `value` as is, count pages by `total_records` and call `on_lazy_load` with `first`, `rows`, the sort, the filters and the global filter on mount and after each change of them.

#### Scenario: Page change
- **WHEN** `lazy`, `total_records = 100`, go to the second page
- **THEN** `on_lazy_load` is called with `first = 10`
