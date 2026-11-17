# Proposal

## Why

Paginator is the twelfth wave 3 component: pagination (first/previous/next/last page buttons, page links, jumping to a page via a list or a field, rows-per-page selection, a "page N of M" report). It is assembled from the already finished Select and InputNumber and is needed as a base for future tables and lists with pagination.

## What Changes

- New crate `prime-paginator` (a port of PrimeNG 21.1.10 `p-paginator`).
- `Paginator`: root `div.p-paginator.p-component` > optional `div.p-paginator-content-start` > report `span.p-paginator-current` > buttons `p-paginator-first`, `p-paginator-prev` > `span.p-paginator-pages` with `button.p-paginator-page` links (`p-paginator-page-selected`, `aria-current="page"`) > jump-to-page dropdown `p-paginator-jtp-dropdown` (`Select`) > buttons `p-paginator-next`, `p-paginator-last` > jump field `p-paginator-jtp-input` (`InputNumber`) > rows dropdown `p-paginator-rpp-dropdown` (`Select`) > optional `div.p-paginator-content-end`.
- External control: two-way signals `first: RwSignal<usize>` (index of the first record) and `rows: RwSignal<usize>` (rows per page), `total_records`; the `on_page_change` event receives `PaginatorState { page, first, rows, page_count }` — like PrimeNG's `onPageChange`.
- Properties: `total_records`, `rows_per_page_options` (`Vec<RowsPerPage>`: a number or "show all"), `page_link_size` (5), `always_show` (yes), `show_current_page_report`, `current_page_report_template` (`{currentPage} of {totalPages}`, supported: `{currentPage}`, `{totalPages}`, `{first}`, `{last}`, `{rows}`, `{totalRecords}`), `show_first_last_icon` (yes), `show_page_links` (yes), `show_jump_to_page_dropdown`, `show_jump_to_page_input`, `dropdown_scroll_height` (`200px`), `style`, `style_class`, `pt`; icons `first_page_link_icon`, `previous_page_link_icon`, `next_page_link_icon`, `last_page_link_icon`, `dropdown_icon`; views `template_left`, `template_right` (receive the state).
- Behaviour: transitions via buttons and links (at the bounds and for an empty list the buttons are disabled), a link window around the current page (`calculatePageLinkBoundaries`), jumping via list and field, a rows count change recomputes the page, when `total_records` shrinks the current page steps back; with `always_show = false` and a single page the component is hidden.
- Showcase page `/paginator` with demos `basic`, `currentpagereport`, `template`, `images`; e2e; benchmarks and budgets; `docs/paginator.md`.
- Styles: `paginator` is added to `theme.components`.
- **Out of scope:** `dropdownAppendTo`/`appendTo` (the list panels are always inside the root, as in Select), `locale` for digits (digits are always Latin), `jumpToPageItem` and `dropdownItem` templates, `Ripple`, `Bind`, `pTemplate`, `@primeuix/motion`.

## Capabilities

### New Capabilities
- `paginator`: pagination with page links, buttons, jumping to a page, rows-per-page selection and a current page report.

### Modified Capabilities
<!-- None. -->

## Impact

- New crate `crates/prime-paginator` (`publish = false`); dependencies: `prime-core`, `prime-icons`, `prime-select`, `prime-inputnumber`, `web-sys`.
- Changed: workspace `Cargo.toml`, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml` (browser job, three places).
