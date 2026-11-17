# Design

## Context

Requirements are in `specs/autocomplete`. PrimeNG 21.1.10 sources: `autocomplete/autocomplete.ts`, `style/autocompletestyle.ts`; demos `apps/showcase/doc/autocomplete/*-doc.ts`. Reference implementations: `prime-select`/`prime-treeselect`/`prime-cascadeselect` (panel inside the root, window listeners, `align_overlay`), `prime-inputtext` (field classes), `prime-multiselect` (chips).

## Goals / Non-Goals

**Goals:** the same DOM, classes, `aria-*`, keys, query and selection.

**Non-Goals:** objects as options, virtual scrolling, Angular forms, `appendTo`, `header`/`footer`/icon templates.

## Decisions

### D1. Model
`AutoCompleteOption { label, value: Option<String>, disabled, items }`. Flat list for navigation: `flatten(options, group)` yields `Row::Group(label)` / `Row::Option { index_in_flat, label, value, disabled }` rows; pure functions `first/last/next/prev` over eligible rows (not disabled and not headers), `selected_index`, `matches(label, text)` (`force_selection`), `input_text(value, options)` (value label), `split_separator`: table-driven tests.

### D2. State
`Core`: `suggestions: Prop<Vec<AutoCompleteOption>>`, `value: RwSignal<String>`, `values: RwSignal<Vec<String>>`, `open`, `focused`, `loading`, `focus_index: RwSignal<Option<usize>>`, `focused_chip: RwSignal<Option<usize>>`, `dirty`, `search_timer` (wasm), references to the root, field and panel; open effects (z-index, `align_overlay`, outside-click and `resize` listeners) are under `cfg(wasm32)`. An effect on `suggestions` shows the panel while a query is in progress.

### D3. Input
The field text is its own `RwSignal<String>` `text` (single mode synchronizes with the value: value to label, selection to label). `on:input` updates `text`, and with `typeahead` and length >= `min_length` sets the `search` timer. `text` becomes the value when `!force_selection`.

### D4. Chips and keyboard
The `ul` container gets focus on Left on an empty field; `focused_chip` is the index; removal is `removeOption`.

### D5. Tests
SSR: single/multiple mode, classes, clear, button, spinner, `pt`; browser `tests/web.rs` as one sequential test: input and query (`delay`), selection by click and key, `multiple`, chips, arrow button, clear, `force_selection`, click outside, groups; `sleep` after a key.

## Risks / Trade-offs

- [Two "values" (text and option value)] For string options they coincide; they differ only when an option has a `value`.
- [Query timer] Reset on unmount (`StoredValue`), an `is_disposed` check.

## Open Questions

- Whether to reopen the panel if `suggestions` arrive without a query (external update): as in PrimeNG, only while `loading`.

## Migration Plan

No migration. Rollback is `git revert`.
