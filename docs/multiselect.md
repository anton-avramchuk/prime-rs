# MultiSelect

Port of `p-multiselect` from PrimeNG 21.1.10 (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-multiselect::MultiSelect` | `packages/primeng/src/multiselect/multiselect.ts` (`MultiSelect` and `MultiSelectItem`), `multiselect/style/multiselectstyle.ts` |
| CSS | `@primeuix/styles/multiselect`, `iconfield`, `chip`, `checkbox` and the Aura preset — `cargo xtask gen-theme`; the `/* For PrimeNG */` block contains only Angular `ng-invalid` selectors and is not taken |
| Demos | `apps/showcase/doc/multiselect/*-doc.ts` (`basic`, `chips`, `clearicon`, `disabled`, `filled`, `filter`, `fluid`, `group`, `invalid`, `loadingstate`, `sizes`, `template`, `floatlabel`, `iftalabel`) |

Spec: `openspec/specs/multiselect`.

## What matches PrimeNG

- Structure: root `div.p-multiselect`, hidden field `input[role=combobox]` in `p-hidden-accessible`, `p-multiselect-label-container` > `p-multiselect-label`, clear icon `p-multiselect-clear-icon`, `p-multiselect-dropdown` with an arrow or loading icon, panel `div.p-multiselect-overlay` (id `{id}_list`) with hidden `p-hidden-focusable` boundaries, header `p-multiselect-header` ("select all" checkbox and `p-iconfield.p-multiselect-filter-container` with the `p-multiselect-filter` field), container `p-multiselect-list-container`, list `ul.p-multiselect-list[role=listbox][aria-multiselectable]`, options `li.p-multiselect-option` with a checkbox, group headers `p-multiselect-option-group` and `li.p-multiselect-empty-message`.
- Root classes and their order, `data-p` of the root and label (`placeholder`, `clearable`, `disabled`, size, `has-chip`, `empty`), `p-multiselect-label-empty` with the text `empty` (hidden by style), `p-multiselect-option-selected` (with `highlight_on_select`), `p-focus`, `data-p-focused`/`-highlight`/`-selected`/`-disabled`, `aria-selected`/`aria-checked`.
- Label: values joined with ", ", when `max_selected_labels` is exceeded (3 by default) — a summary from `selected_items_label` or the `selectionMessage` translation with `{0}` substituted; `chip` mode — `Chip` chips in `p-multiselect-chip-item`. As in PrimeNG, in `chip` mode the summary is already shown at `selected >= max_selected_labels` (PrimeNG compares the length of the truncated list with the limit).
- Selection does not close the panel; the `selection_limit` limit makes unselected options disabled (selected ones can be deselected) and removes the "select all" checkbox; "select all" collects values by the `onToggleAll` formula (selected disabled ones remain, with the search field enabled the selection outside the search result is kept) and reports `on_select_all_change` only if the result is empty or contains all options.
- Keys: ↓ ↑ Home End PageUp PageDown Enter Space Escape Tab Shift (ranges, `Ctrl+Shift+Home/End`), `Ctrl+A`, `Alt+↑` (select and close), type-ahead search (500 ms); search field keys — ↓ ↑ ← → Home End Enter.
- Closing on a repeated click, a click outside, `resize`, `Escape`, `Tab`; focus stays on the hidden field (a mouse press inside the panel does not move it, except in the search field).

## Differences

| Difference | Reason |
|---|---|
| The root is a `<div>`, not the `<p-multiselect>` host | The host in Angular is a framework detail. |
| `values: RwSignal<Vec<String>>` instead of `ngModel`; without it internal state (`default_values`); panel openness is a two-way `open` | Two-way binding without form infrastructure. |
| Options are `MultiSelectOption { label, value, disabled }` and `MultiSelectGroup { label, items }`, values are strings; no `optionLabel`, `optionValue`, `optionDisabled`, `optionGroup*`, `dataKey`, `filterBy`, `filterFields`, `filterMatchMode`, `filterLocale` | Same as Select and Listbox. |
| The panel is always inside the component, no `appendTo` and `overlayOptions`, no appear animation | Same as Select: there is no shared Overlay; an ancestor with `overflow` may clip the panel. |
| No virtual scrolling and lazy loading, `tooltip*`, `autofocus`, external `selectAll`, `header`, `footer`, `filter`, `empty`, `loader`, `headercheckboxicon` templates and template icons (`filtericon`, `itemcheckboxicon`) | Outside the change scope. |
| `End` moves focus to the last valid option | PrimeNG uses `findLastFocusedOptionIndex`, i.e. when there is a selection it goes to the first selected one rather than the end of the list — looks like a bug. `↑` without focus goes to the last selected (or the last valid) one. |
| `PageUp`/`PageDown` scroll the container by the visible area height | In PrimeNG — scroll to the first/last option. |
| `Tab` on an open panel with a search field is not intercepted — focus moves to the search field in natural order, closing on `Tab` from the field goes through the last hidden boundary | A simplification of the PrimeNG `hasFocusableElements` chain; same result. Without a search field `Tab` selects the focused option (if not selected) and closes the panel. |
| `autofocus_filter` is off by default | Same as `autofocusFilter = false` in PrimeNG. |
| Removing a chip: a click on the icon calls `on_remove` and `on_change`, the panel does not open | `Chip` hides itself; the icon list is rebuilt from `values`. |
| The `fluid` property is `OptProp<bool>`: without a value it is taken from the `Fluid` context | See `docs/field-wrappers.md`. |
| The identifier `pn_id_<n>` comes from `prime_rs_core::ids::next_unique`, or the `id` property | Same on the server and in the browser. |
| No `reactiveforms`, `templatedrivenforms`, `virtualscroll` demos; in `template`, country codes instead of CDN flags and no header and footer | Change scope and independence from the CDN. |

## What was not verified

- Touch and screen reading: `aria-*` attributes were checked against PrimeNG, screen reader and touch input behaviour was not.
- Panel position in a narrow window and when the parent overflows with `overflow: hidden`.

## Performance

See `benches/` (`multiselect-1`, `multiselect-1000`, `case-multiselect`) and `benches/*budgets.toml`. The list logic currently duplicates `prime-listbox`; a shared module will be extracted at the third consumer.
