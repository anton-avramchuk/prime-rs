# Design

## Context

Requirements are in `specs/paginator`. PrimeNG 21.1.10 sources: `paginator/paginator.ts`, `paginator/style/paginatorstyle.ts`; demos `apps/showcase/doc/paginator/*-doc.ts`. Reference implementations in the project: `prime-select` (a list with `value: RwSignal<Option<String>>`, `selected_item_template`, `style_class`), `prime-inputnumber` (`value: RwSignal<Option<f64>>`), `prime-rating`/`prime-slider` (two-way numeric signals and a "value + event" pair).

Observations about the source:
- The component state is `first` (input) and `rows` (input, changed internally via the rows list's `ngModel`); everything else (`page`, `pageCount`, link window, report) is derived from them and `totalRecords`.
- `changePage(p)`: if `0 <= p < pageCount` — `first = rows × p`, recompute, `onPageChange` with `{ page, first, rows, pageCount }`. A click on the current link also fires the event.
- Window: `visible = min(pageLinkSize, pageCount)`; `start = max(0, ceil(page − visible/2))`; `end = min(pageCount − 1, start + visible − 1)`; `delta = pageLinkSize − (end − start + 1)`; `start = max(0, start − delta)`. With `pageLinkSize > pageCount` `delta` is positive and `start` stays 0.
- The "first" button has no `disabled` attribute (only the `p-disabled` class and a guard in the handler); the others use `[disabled]`.
- `updateFirst()` when `totalRecords` changes: if `page > 0` and `first >= totalRecords` — asynchronously to the previous page.
- The jump list shows the report instead of a label (the `selectedItem` template); option values are page indexes, labels "1"…"N".
- Rows list: labels are localized numbers, the "Show all" value is `totalRecords`.
- Report: `replace` with a string — only the first occurrence of each placeholder is changed.
- The root is hidden via `display: none` if `!alwaysShow` and there is at most one link.

## Goals / Non-Goals

**Goals:** the same DOM, classes and `aria-*`; the same page and link-window arithmetic; `first`/`rows` as two-way signals; the `on_page_change` event.

**Non-Goals:** `appendTo`, digits by `locale`, list item templates, `Ripple`, `Bind`, `pTemplate`.

## Decisions

### D1. Crate and state
`prime-paginator` with `Paginator`, `PaginatorState`, `RowsPerPage`, `PaginatorPt`. `first: Option<RwSignal<usize>>` and `rows: Option<RwSignal<usize>>` are two-way; without them internal state (`default_first`, `default_rows`, 0 and 10 by default). `total_records: Prop<usize>`.

### D2. Pure logic (`state.rs`)
- `page_count(total, rows)` (0 when `rows == 0`), `page(first, rows)`, `is_first/is_last/is_empty`.
- `link_bounds(page, page_count, link_size) -> (start, end)` — the PrimeNG formula; `links(...) -> Vec<usize>` (numbers counted from one).
- `report(template, page, page_count, first, rows, total)` — replaces the first occurrence of each placeholder.
- `rows_items(options, total) -> Vec<(label, value)>` and `page_items(count)`.
- `clamp_first_after_total(first, rows, total) -> Option<usize>` — the page to step back to (`updateFirst`).
- `next_first_for_rows(first, new_rows, total)` — the page on which the previous `first` stays after the rows count changes: `page = first / new_rows`, clamped to the last existing one.
- Classes of the root/buttons/links.
Table-driven tests for each.

### D3. Markup
One component without splitting into subcomponents; the parts are view functions. The reactive button attributes (`disabled`, `p-disabled`), the link list and the report read only `first`, `rows`, `total_records`, so a click on a link does not recreate the navigation buttons (focus is not lost): the links are rebuilt entirely, the buttons — only their attributes.

### D4. Lists
Both `Select`s are created with internal `Option<String>` signals. Rows list: `value` = `rows` as a string; `on_change` writes `rows` and calls `change_page` with the recomputed page (D2). Page list: `value` = the current page number, the label is the report via `selected_item_template`; `on_change` calls `change_page`. The reverse sync (an external change of `first`/`rows` → the list value) is an `Effect` under `cfg(target_arch = "wasm32")`, as in Select/SelectButton (without an executor it panics in native tests); in SSR the initial value is taken from the signals.

### D5. Jump field
`InputNumber` with the value `current_page` as `f64` and `on_change`; an empty value is ignored, a number is cast to an integer and bounded by the `0 <= p < page_count` check inside `change_page`.

### D6. External changes
An `Effect` under `cfg(wasm32)` watches `total_records`, `first` and `rows` and performs `updateFirst`: while `page > 0` and `first >= total`, it calls `change_page(page − 1)` (synchronously in the effect; one event per step, as in PrimeNG).

### D7. Hiding
The root `style` contains `display: none` if `!always_show` and there is at most one link (link count = `min(link_size, page_count)`).

### D8. CSS
`paginator` is added to `theme.components`; `select`, `inputnumber`, `inputtext` are already attached. Check `p-paginator-*` in the preset. The `/* For PrimeNG */` block is not used (if there is one — check).

### D9. Tests
SSR: markup by properties, link window, report, classes and `disabled`, icons, templates, hiding, empty, `pt`. Browser `tests/web.rs`: clicks on buttons and links, events, jump via list and field, rows count change, external signals and stepping back. e2e: a real mouse and keys (Tab/Enter on the buttons). The crate is added in three places of `ci.yml`.

### D10. Benchmarks
Scenarios `paginator-1`/`-1000` (`total_records = 120`, `rows = 10`, `rows_per_page_options = [10, 20, 30]`, default values) and `case-paginator`; first `cargo bench -p render-bench --bench alloc`; for wasm — a separate budgets file `--budgets`.

## Risks / Trade-offs

- [Size and allocations] Two `Select`s and an `InputNumber` inside — the heaviest "composite" component; budgets are chosen by measurement, the lists are created only when enabled by properties.
- [Signal sync] Two-way binding of the internal list signals with `first`/`rows` via `Effect` → one-way effects and a "value already matches" check, to avoid looping.
- [Digits] Digits are always Latin (no `Intl.NumberFormat`); recorded in the docs.

## Open Questions

- Whether `jump_to_page_item_template` and `dropdown_item_template` are needed: no, the default is enough; we will come back if tables need them.

## Migration Plan

No migration. Rollback — `git revert`.
