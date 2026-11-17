# Design

## Context

Requirements are in `specs/multiselect`. PrimeNG 21.1.10 sources: `multiselect/multiselect.ts`, `multiselect/style/multiselectstyle.ts`; demos `apps/showcase/doc/multiselect/*-doc.ts`. Models in the project:
- `prime-select` — root, panel inside the root, `align_overlay(Relative)`, `Overlay` group z-index, `window` listeners while open, open effect under `cfg(wasm32)`, clear/loading icon, search field focus, type-ahead search, `realign` after the list changes;
- `prime-listbox` — option model, `visible_entries`, indices, `select_multiple`, `select_range`, `select_all`, checkbox as `Checkbox` markup with state from closures, keys with ranges.

Observations about the source:
- Keyboard focus is on the hidden `input[role=combobox]` in `p-hidden-accessible`; it holds `aria-expanded`, `aria-controls`, `aria-activedescendant`.
- Label: `label()` = labels of the selected ones joined with ", " or `selectedItemsLabel` when `maxSelectedLabels` is exceeded (3 by default), otherwise `placeholder`. In `chip` mode — `chipSelectedItems()` (the first `maxSelectedLabels`) in `p-chip` with `removable`.
- `isOptionDisabled`: with `selectionLimit` and the limit reached, unselected options are disabled.
- `onToggleAll`: result = selected before the search (if searching and not all are selected) + selected disabled + visible (all selected/valid, or only selected-invalid if everything is selected) — without duplicates.
- Keys: as in Listbox, plus opening the panel with ↓ ↑ Home End and by typing, `Alt+↑` — select and close, `Tab` — select the focused one (if not selected) and close, `Escape` — close.
- Unlike Select, selection does not close the panel; focus stays on the hidden field, the list does not take focus.
- `filter` defaults to `true`.

## Goals / Non-Goals

**Goals:** the same DOM, classes and roles; selection behaviour, limit, "select all", keys and closing; chip mode; `values`/`open` as two-way signals.

**Non-Goals:** virtual scrolling, `appendTo`/`overlayOptions`, `@primeuix/motion`, Angular forms, `optionLabel` and related, `tooltip*`, `header`/`footer`/`filter`/`empty`/`loader` templates, external `selectAll`.

## Decisions

### D1. Crate and model
`prime-multiselect` with `MultiSelect`, `MultiSelectOption`, `MultiSelectGroup`, `MultiSelectDisplay { Comma, Chip }`, `MultiSelectPt`. The types match `ListboxOption`/`ListboxGroup` in shape, but are our own — a dependency on `prime-listbox` just for the structs would pull the whole Listbox into the binary.

### D2. Pure logic (`logic.rs`)
A copy of the needed Listbox functions (flat list, filter, indices, type-ahead search, ranges, `select_all`) plus specific ones:
- `option_disabled(option, selected, limit)` — accounts for the selection limit;
- `entries_with_limit(...)` or a `limit` parameter in `is_valid` — unselected options are disabled at the limit;
- `toggle_all(all_options, visible, selected, filter_active) -> Vec<String>` — the PrimeNG formula without duplicates;
- `all_selected(visible, selected)`;
- `label_text(selected_labels, max, summary_template, placeholder, display_selected_label)` and `chip_labels(selected_labels, max)`;
- `summary(template, n)` — substituting `{0}`;
- `root_classes`, `label_classes`, `option_classes`, `data_p`.
Table tests for each. Duplication with Listbox is intentionally left until a third consumer.

### D3. State and markup
`values`, `open`, `filter_text`, `focused_index: Option<usize>`, `start_range`, `focused` (field), `clicked` (panel opened by mouse). Root classes and `data-p` are reactive closures; the label reads `values` and the options; the list is as in Select (`ul` is stable, items are rebuilt from options, groups and the search string); option state (selected, focus, disabled by the limit) is reactive attributes so that clicks and navigation do not recreate elements and lose scroll position.

### D4. Panel and window
Modelled on Select: open effect under `cfg(wasm32)` (`opened`/`closed`), `align_overlay(Relative)`, z-index `ZGroup::Overlay`, window `click` and `resize` listeners while open, `realign` after search input and after the chip count changes, `release` in `on_cleanup`. A click inside the root does not close the panel. Hidden focus boundaries are as in Select.

### D5. Chips
`Chip` from `prime-chip` with `removable`; with `chip_icon` — its own remove icon. Click on remove: `stop_propagation`, remove the value, `on_remove`, `on_change`. If `Chip` lacks the needed extension points, the chip markup is built by hand with the same classes (`p-chip`, `p-chip-label`, `p-chip-remove-icon`), and the decision is recorded in docs.

### D6. Selection and limit
`select_option` (click/Enter): `option.disabled` or the limit → nothing; otherwise toggle the value in `values`, `on_change`. The limit is `Prop<Option<usize>>`; unselected options are disabled when `values.len() >= limit`. All selections go through `update_model(values, event)`.

### D7. Keys
Two handlers (`key_down(e, in_filter)`) — the hidden field and the search field; a `match code` table. Opening the panel from any navigation key as in PrimeNG. Ranges and `Ctrl+A` come from the logic. `PageUp`/`PageDown` — page-wise container scrolling, as in Listbox.

### D8. Views
`item: Option<Callback<(MultiSelectOption, bool), AnyView>>`, `group: Option<Callback<MultiSelectGroup, AnyView>>`, `selected_items: Option<Callback<Vec<MultiSelectOption>, AnyView>>`; icons — `Option<ViewFn>`.

### D9. CSS
`multiselect` in `theme.components` (the `/* For PrimeNG */` block contains only Angular `ng-invalid` selectors and is not needed, `primeng_extra` is not used); `iconfield` and `chip` are already included. Check `p-multiselect-*` in the preset.

### D10. Tests
SSR: root and label markup (`comma`/`chip` modes, summary, hint), root classes by state, clear/loading icons, `data-p`, `pt`, `style_class`, no panel. Browser `tests/web.rs` (window listeners are shared → one sequential run for tests with an open panel): open/close, selection does not close, limit, "select all" with search, chips and removal, clear, keys (including ranges, `Ctrl+A`, `Alt+↑`, `Tab`, `Escape`), type-ahead search, filter, external `open`/`values`, z-index and position, blocking, unsubscribing. e2e: real mouse and keys, chips, panel position. The crate is added in three places in `ci.yml`.

### D11. Benchmarks
Scenarios `multiselect-1`/`-1000` (five options, closed panel, two selected) and `case-multiselect`; first `cargo bench -p render-bench --bench alloc`; for wasm — a separate budgets file `--budgets` with only this case, then the budget in `benches/budgets.toml`.

## Risks / Trade-offs

- [wasm size] Select + Listbox + Chip in one component — the budget is chosen by measurement.
- [Focus] Rebuilding the list on selection would drop scroll position → only reactive attributes on options.
- [Shared window listeners] Several MultiSelects in one test → one sequential run.
- [Duplication] The list logic partly repeats Listbox → extract into a shared module at the third consumer.
- [Chip] If `Chip` does not allow replacing the remove icon — markup by hand.

## Open Questions

- Whether a show animation is needed: check whether the theme has `@keyframes` for `p-anchored-overlay`; if not — no animation (as in Select).

## Migration Plan

No migration. Rollback — `git revert`.
