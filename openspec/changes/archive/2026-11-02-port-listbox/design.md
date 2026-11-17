# Design

## Context

Requirements are in `specs/listbox`. PrimeNG 21.1.10 sources: `listbox/listbox.ts`, `listbox/style/listboxstyle.ts`; demos `apps/showcase/doc/listbox/*-doc.ts`. Reference implementations in the project: `prime-select` (`SelectOption`/`SelectGroup`, flat `Entry` list, focus by index, type-ahead search, filter with `p-iconfield`), `prime-checkbox` (checkboxes), `prime-menu` (`flatten`, pure key logic, the `*_untracked` rule in list closures).

Observations about the source:
- One template: header ("select all" checkbox and/or search field) → `div.p-listbox-list-container` → `ul[role=listbox]` with groups and options or `li.p-listbox-empty-message`.
- Option identifier `<id>_<index>` over the flat list including group headers; `focusedOptionIndex` is an index in this list (−1 means none).
- Selection: `onOptionSelectSingle`/`Multiple` with `metaKeySelection`; touch (`optionTouched`) disables meta mode. Range is `onOptionSelectRange` between `startRangeIndex` and the focus; `Shift` without arrows updates the anchor (`onShiftKey`).
- Focus on the list: the focused one stays, otherwise the first enabled (`autoOptionFocus`) or the selected one; blur clears the focus, the anchor and the accumulated search string.
- An Angular user gets the value via `ngModel`; for us, signals.
- `p-listbox-header > .p-iconfield { flex-grow: 1 }` and `.p-listbox-list-container { height: 100% }` are in the `/* For PrimeNG */` block.

## Goals / Non-Goals

**Goals:** the same DOM, classes, roles and keys (including ranges and `Ctrl+A`), single and multiple selection, groups, search, checkboxes and checkmark.

**Non-Goals:** virtual scroll, lazy loading, `dragdrop`, Angular forms, `optionLabel` and related, hidden focus boundaries, `aria-live` messages, `header`/`footer`/`filter`/`empty`/`loader` templates, external `selectAll`.

## Decisions

### D1. Crate and model
`prime-listbox` with `Listbox`, `ListboxOption { label, value, disabled }`, `ListboxGroup { label, items }`, `ListboxPt`. The types have the same shape as `SelectOption`/`SelectGroup` but are separate: depending on `prime-select` for two structs would pull all of Select into the binary. Values are strings. `options`/`groups` are `Prop<Vec<_>>`.

### D2. Value
`value: RwSignal<Option<String>>` (single mode) and `values: RwSignal<Vec<String>>` (`multiple`); both are optional, without them internal state is used (`default_value`/`default_values`), as in Select. A value that is not among the options stays in the model but is not highlighted. One component, two modes: the shared choice goes through an internal `Selection` (pure functions below).

### D3. Pure logic (`logic.rs`)
- `visible_entries(options, groups, filter) -> Vec<Entry>` modeled on Select (substring, case-insensitive, groups without matches are omitted).
- Indices: `first_valid`, `last_valid`, `next_valid`, `prev_valid`, `find_by_prefix`, `nearest_selected(…)`.
- Selection: `select_single(current, value, meta, meta_key) -> Option<Option<String>>` (None means no change), `select_multiple(current, value, meta, meta_key) -> Vec<String>`, `select_range(entries, start, end) -> Vec<String>`, `select_all(entries) -> Vec<String>`, `all_selected(entries, selected) -> bool`.
- `root_classes`, `option_classes`, `data_p`.
Table-driven tests for each; the component calls only these functions, no DOM checks of the logic are needed.

### D4. State and markup
`focused_index: RwSignal<i32>`, `start_range: RwSignal<i32>`, `focused: RwSignal<bool>`, `filter: RwSignal<String>`, `search: StoredValue<(String, Option<TimeoutHandle>)>`. List entries are a `Memo<Vec<Entry>>` over options, groups and the search string; `For` keyed by "index+value". In closures that build the list, read the focus and selection signals only via `*_untracked` (lesson from Menu/SelectButton); `p-focus`, `aria-*`, `data-p-*` and the selected class are reactive attributes of each option.

### D5. Checkboxes and search field
The option checkbox and "select all" use `Checkbox` markup (classes and `data-p` from `prime_checkbox::{CheckboxState, write_classes, data_p}`) with state from closures: the `Checkbox` component would need a signal per option and an `Effect`, which does not work in SSR. A click on the checkbox field is cancelled (`preventDefault`) and handled by the list; `tabindex=-1`. The search field is hand-written markup `span.p-iconfield` > `input.p-inputtext.p-component.p-listbox-filter` + `span.p-inputicon` with a `search` icon (as in Select), without `prime-iconfield`/`prime-inputtext`: fewer dependencies, the `p-listbox-filter` class is set by the field itself.

### D6. Keys
`keydown` of the list and the search field is a `match event.code` table as in PrimeNG; `Shift` ranges, `Ctrl+A`, type-ahead search (500 ms, accumulated string in `StoredValue`, the timer is cleared in `on_cleanup` and checked with `is_disposed`). `PageUp`/`PageDown` scroll the container by the height of the visible area via `scroll_top` (in PrimeNG it is `scrollInView(0)`, the difference is in docs). Scrolling to the focused option is `scrollIntoView({block: nearest})` after insertion/focus change, only on wasm (`Effect::new` under `cfg(wasm32)`).

### D7. Mouse and touch
`mousedown` sets the focused option, `mouseenter` does so when `focus_on_hover && focused`, `click` selects and calls `on_click`, `dblclick` calls `on_dbl_click`. `touchend` on an option disables meta mode until the next selection (a `StoredValue` flag).

### D8. Views
`option: Option<Callback<(ListboxOption, bool), AnyView>>` (option, whether selected), `group: Option<Callback<String, AnyView>>`.

### D9. CSS
`listbox` is added to `theme.components` and to `primeng_extra` (the `/* For PrimeNG */` block); `iconfield` is already enabled by Select. Check that `p-listbox-*` is in the preset.

### D10. Tests
SSR: markup, groups, empty message, selected (single/multiple), checkboxes, checkmark, search field, `data-p`, `style_class`, `pt`, `aria-*`. Browser `tests/web.rs` (one sequential run since listeners and timers are shared): click and clear, meta mode, multiple, `Shift` range, `Ctrl+A`, "select all" with search, filter, keys and `aria-activedescendant`, type-ahead search, `readonly`/`disabled`, `on_*`, unsubscribing. e2e: real mouse and keys. The crate is added in three places of `ci.yml`.

### D11. Benchmarks
Scenarios `listbox-1`/`-1000` (five options) and `case-listbox`; first `cargo bench -p render-bench --bench alloc`, for wasm a temporary large budget (replace by unique string/line number).

## Risks / Trade-offs

- [Focus] Rebuilding the list drops focus → only `*_untracked` in list closures.
- [Two value modes] A double set of properties (`value`/`values`) complicates the API → one internal `Selection`, documented in rustdoc and docs.
- [Search timers] Closures may outlive the component → `is_disposed()` and clearing in `on_cleanup`.
- [wasm size] Keys, ranges and checkboxes are heavier than Menu → the budget is chosen by measurement.

## Open Questions

- Whether `scroll_height` needs to be `Prop<String>` or a static string suffices: we take `Prop<String>` (like `style`).

## Migration Plan

No migration. Rollback is `git revert`.
