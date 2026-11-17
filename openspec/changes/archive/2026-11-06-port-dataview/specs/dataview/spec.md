# dataview Specification

## Purpose
DataView is data as a list or a grid with a paginator, sorting and a filter.

## ADDED Requirements

### Requirement: Selection and page
Items SHALL pass the filter (a predicate), then a stable sort (`sort_order` 1 ascending, -1 descending, 0 no sorting), then the page `[first, first + rows)` is selected if the paginator is enabled. In `lazy` mode the value SHALL be shown as is, and the paginator's record count SHALL be taken from `total_records`. A `first` beyond the range SHALL be returned to the first record of the last page.

#### Scenario: Sorting
- **WHEN** `[5, 3, 8, 1]`, order -1
- **THEN** `[8, 5, 3, 1]`

#### Scenario: Page
- **WHEN** 25 items, `rows = 10`, `first = 20`
- **THEN** the last 5 items are shown

#### Scenario: Out of range
- **WHEN** `first = 90` with 25 items and `rows = 10`
- **THEN** `first` becomes 20

### Requirement: Markup
`DataView` SHALL render `div.p-dataview.p-component.p-dataview-list` or `-grid` with blocks in the order: loading (`p-dataview-loading` > `p-dataview-loading-overlay p-overlay-mask` > icon), header, paginator on top (`p-dataview-paginator-top`), `p-dataview-content` (the `list` or `grid` template, the `p-dataview-empty-message` message when the data is empty and not loading), paginator at the bottom, footer.

#### Scenario: Empty
- **WHEN** there are no items
- **THEN** the message "No results found" is shown (or the given `empty_message`/`empty`)

#### Scenario: Loading
- **WHEN** `loading = true`
- **THEN** there is a mask with an icon, no empty data message

### Requirement: Events and lazy mode
A page change SHALL update `first` and `rows` and call `on_page`; in `lazy` mode also `on_lazy_load` (`first`, `rows`, `sort_order`). `on_lazy_load` SHALL be called on creation (if `lazy_load_on_init`) and on a sort order change; an order change returns to the first page.

#### Scenario: Lazy
- **WHEN** `lazy` mode, go to the second page
- **THEN** `on_lazy_load` is called with `first = rows`
