# paginator Specification

## Purpose
Paginator shows pagination over a set of records: buttons and page links, jumping to a page, rows-per-page selection and a current page report.

## ADDED Requirements

### Requirement: Markup
`Paginator` of the `prime-paginator` crate SHALL render `div.p-paginator.p-component` with `style`/`style_class`/`pt` on the root. In order: `div.p-paginator-content-start` (with `template_left`), `span.p-paginator-current` (with `show_current_page_report`), the `p-paginator-first` button (with `show_first_last_icon`), the `p-paginator-prev` button, `span.p-paginator-pages` with links (with `show_page_links`), the jump dropdown (with `show_jump_to_page_dropdown`), the `p-paginator-next` and `p-paginator-last` buttons (the latter with `show_first_last_icon`), the jump field (with `show_jump_to_page_input`), the rows dropdown (with a non-empty `rows_per_page_options`), `div.p-paginator-content-end` (with `template_right`). Buttons SHALL have `type="button"` and an `aria-label` from the translation (`firstPageLabel`, `prevPageLabel`, `nextPageLabel`, `lastPageLabel`) and an icon (`first_page_link_icon` and the others replace the standard ones).

#### Scenario: Default composition
- **WHEN** a `Paginator` with `total_records = 120` and `rows = 10` is rendered
- **THEN** four buttons and five page links are rendered, and there is no report, lists or jump field

### Requirement: Page links
Links SHALL show a window of `min(page_link_size, page count)` adjacent pages with the current one in the middle if possible; at the start and end of the list the window bounds shift so that exactly `min(page_link_size, page count)` links are shown. The current link SHALL have the class `p-paginator-page-selected` and `aria-current="page"`; each link has an `aria-label` from `aria.pageLabel` with `{page}` substituted. A click on a link SHALL go to its page.

#### Scenario: Window at the start
- **WHEN** 12 pages, the current one is the first, `page_link_size = 5`
- **THEN** links 1–5, the first is selected

#### Scenario: Window in the middle
- **WHEN** 12 pages, the current one is the sixth
- **THEN** links 4–8

#### Scenario: Window at the end
- **WHEN** 12 pages, the current one is the twelfth
- **THEN** links 8–12

### Requirement: Button navigation
The "first" button SHALL go to page 1, "previous" — to `page − 1`, "next" — to `page + 1`, "last" — to the last one. On the first page "first" and "previous", on the last page "next" and "last", and with an empty list all of them SHALL get the class `p-disabled`; "previous", "next" and "last" SHALL also get the `disabled` attribute. Each transition SHALL write the new `first` (`rows × page`) and call `on_page_change` with `PaginatorState { page, first, rows, page_count }` once. A transition to a non-existent page SHALL NOT be performed.

#### Scenario: Next page
- **WHEN** the first page and a click on "next"
- **THEN** `first` equals `rows`, `on_page_change` is called with `page = 1`

#### Scenario: Bound
- **WHEN** the last page
- **THEN** "next" and "last" are disabled

### Requirement: Report
With `show_current_page_report`, `span.p-paginator-current` SHALL show `current_page_report_template` with substitution of `{currentPage}` (counted from one, 0 for an empty list), `{totalPages}`, `{first}` (counted from one, 0 for an empty list), `{last}` (`min(first + rows, total_records)`), `{rows}`, `{totalRecords}`; each placeholder is replaced once (the first occurrence).

#### Scenario: Template
- **WHEN** the template is "Showing {first} to {last} of {totalRecords}", `first = 10`, `rows = 10`, `total_records = 120`
- **THEN** the report is "Showing 11 to 20 of 120"

### Requirement: Jump to a page
With `show_jump_to_page_dropdown` a `Select` with the class `p-paginator-jtp-dropdown` SHALL be rendered, with options "1"…"N", the current page selected and the report as the label instead of the usual one; selecting SHALL go to the page. With `show_jump_to_page_input` an `InputNumber` with the class `p-paginator-jtp-input` SHALL be rendered, with the value "current page" (counted from one); entering a number SHALL go to page `number − 1` if it exists. Both elements SHALL be disabled for an empty list.

#### Scenario: Page list
- **WHEN** the fifth page is selected in the list
- **THEN** `first` equals `rows × 4`

### Requirement: Rows per page
With a non-empty `rows_per_page_options` a `Select` with the class `p-paginator-rpp-dropdown` SHALL be rendered, with options from the settings (a number — label and value; `ShowAll(label)` — an option with the value `total_records`), the current `rows` selected and the `aria-label` `rowsPerPageLabel`. Selecting SHALL write the new `rows` and go to the page on which the previous `first` stays (if the page does not exist — to the last existing one), calling `on_page_change`.

#### Scenario: Rows count change
- **WHEN** `rows = 10`, `first = 20` and 20 is selected
- **THEN** `rows` equals 20, `first` equals 20, page 2, `on_page_change` is called

### Requirement: External control
`first` and `rows` SHALL be two-way signals; an external change of `first`, `rows` or `total_records` SHALL re-render the link window, the report and the button state. If after `total_records` shrinks `first` is not less than `total_records` and the page is greater than the first, the component SHALL go to the previous page (no further than the last existing one) until `first` is within range.

#### Scenario: Record count shrinks
- **WHEN** `first = 110`, `rows = 10`, then `total_records` is reduced from 120 to 50
- **THEN** `first` becomes 40

### Requirement: Hiding
With `always_show = false` and a single page the root SHALL get `display: none`.

#### Scenario: Single page
- **WHEN** `always_show = false`, `total_records = 5`, `rows = 10`
- **THEN** the root is hidden

### Requirement: Empty list and edge values
With `total_records = 0` there SHALL be zero pages, no links, all buttons disabled; with `rows = 0` there SHALL be zero pages (no division by zero).

#### Scenario: Empty
- **WHEN** `total_records = 0`
- **THEN** no links, all buttons disabled, the report is "0 of 0"

### Requirement: Rendering without a browser
Outside the browser (SSR) `Paginator` SHALL render fully; `window` and `document` SHALL NOT be used.

#### Scenario: SSR
- **WHEN** a page with `Paginator` is rendered on the server
- **THEN** the markup has buttons and page links, the current one is selected
