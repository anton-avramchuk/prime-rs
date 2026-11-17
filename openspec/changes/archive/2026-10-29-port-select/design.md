# Design

## Context

Requirements are in `specs/select`. PrimeNG 21.1.10 sources: `packages/primeng/src/select/select.ts` (2015 lines: `Select` and the nested `SelectItem`), `select/style/selectstyle.ts`, demos `apps/showcase/doc/select`. Reference implementations: `crates/prime-inputnumber` and `crates/prime-knob` (`window` listeners and timers from handlers, `StoredValue<_, LocalStorage>`, `Effect` only under `cfg(target_arch = "wasm32")`, closures that do not recreate elements), `crates/prime-selectbutton` (string values, `SelectButtonOption`), `crates/prime-dom` (`position::absolute_position`, `dom::align_overlay`, `FocusTrap` with focusable boundaries) and `prime_core::zindex` (`ZGroup::Overlay`). The scope of the first change was chosen by the user: core, filter and groups.

Observations about the source that affect the decisions:
- The panel is a separate `p-overlay` (positioning, appending to `body` or `self`, closing on outside click, animation); we have no such component, and Select is its first consumer.
- Option indices (`focusedOptionIndex`, id `{id}_{index}`) are computed over the flat list in which group headers occupy positions; only non-disabled options are valid for focus (`isValidOption`). Type-ahead search (`searchOptions`) accumulates characters for 500 ms and looks for the first valid option with that prefix.
- The `filter` search in PrimeNG uses `FilterService` (match modes, fields, locale); the default is "contains". With groups, the groups that have matching options are kept.
- `Tab` on an open panel with focusable elements moves focus to a hidden boundary element of the panel (focus trap); otherwise it selects the focused option and closes the panel.
- The `isVisibleClearIcon` and `emptyMessage` selectors depend on translation (`emptyMessage`, `emptyFilterMessage`, `aria.listLabel`); these keys already exist in `prime-core::translation_defaults`.
- The `/* For PrimeNG */` block in `selectstyle.ts` contains the needed placeholder color rule `.p-select-label.p-placeholder { color: dt('select.placeholder.color') }` (plus Angular Forms rules), so `select` is added to `primeng_extra`.
- The search field in PrimeNG is `p-iconfield` + `p-inputicon`; we have not ported those components, but their CSS (`iconfield`, `inputicon`) is needed for the search field markup.

## Decisions

### D1. Data and value
`SelectOption { label: String, value: String, disabled: bool }`, `SelectGroup { label: String, items: Vec<SelectOption> }`; `options: Prop<Vec<SelectOption>>` and `groups: Prop<Vec<SelectGroup>>` (if there are no groups, a flat list). Values are strings (as in `SelectButton`). `value: Option<RwSignal<Option<String>>>`, otherwise an internal signal with `default_value`. Openness state: `open: Option<RwSignal<bool>>`, otherwise internal.

### D2. Flat list (`options.rs`, pure functions)
`Entry::Group(label)` and `Entry::Option { option, group }`; `visible_entries(options, groups, filter) -> Vec<Entry>` (case-insensitive "contains" filtering; groups without matching options are omitted). Pure index functions: `first_valid`, `last_valid`, `next_valid`, `prev_valid` (like `findNextOptionIndex` and `findPrevOptionIndex`: when there is no next one, the index stays the same), `selected_index`, `first_focused` (the selected or the first valid), `search_match(entries, typed)`, `set_size`, `pos_in_set`. Table-driven tests for groups, disabled options and boundaries.

### D3. State and reactivity
`value`, `open`, `filter: RwSignal<String>`, `focused_index: RwSignal<Option<usize>>` (index in the flat list), `focused: RwSignal<bool>` (label is focused), `clicked: RwSignal<bool>` (opened with the mouse: the first arrow goes to the first or last option rather than the selected one), the type-ahead buffer (`StoredValue`) with a 500 ms timer. The label, clear icon and button read only `value`, `open`, `loading` and properties; the panel is in the DOM only when `open`, so the panel closure can be recreated, while the label (the element that receives focus) is not recreated (lesson from `port-selectbutton`). Inside the list, items are built from the flat list; option focus is a class and attributes via closures, without recreating `li`.

### D4. Panel (`overlay.rs`)
The panel is a descendant of the root (`position: relative` from the theme CSS), absolutely positioned by `prime_dom::align_overlay` (below the field, above it when there is not enough room), z-index is `prime_core::zindex::generate(ZGroup::Overlay, base)` from the configuration, released on close. `window` subscriptions (click/`mousedown` outside the component, `resize`) are created on open from a handler or an `Effect` under `cfg(wasm32)` and removed on close and `on_cleanup`. There is no animation (the panel appears and disappears instantly), `appendTo` is not supported (always inside the root). `scroll_height` is the `max-height` of the list container; on open the selected option is scrolled with `scrollIntoView({ block: nearest })`.

### D5. Keyboard and focus
The `keydown` of the label (and the search field) is handled by `KeyboardEvent.code`: `ArrowDown`, `ArrowUp` (+`Alt`), `Home`, `End`, `PageUp`, `PageDown`, `Space`, `Enter`/`NumpadEnter`, `Escape`, `Tab`, `Delete`, `Backspace` (in the search field), printable characters (type-ahead search). The rules are as in `onKeyDown` and `onFilterKeyDown`. A blocked state (`disabled`, `readonly`, `loading`) ignores keys. `Tab`: with an open panel that has focusable elements, focus goes to the hidden boundary (the trap from `prime-dom`), otherwise the focused option is selected and the panel closes. Panel focus returns to the label on close (`Escape`, selection, `Tab`).

### D6. Search
The search field is `input.p-inputtext p-component p-select-filter` inside `span.p-iconfield` with a `SearchIcon` in `span.p-inputicon`; `on:input` writes `filter`, resets `focused_index`, calls `on_filter` and recomputes the panel position after re-render. `reset_filter_on_hide` resets the string on close. The empty result message is `empty_filter_message` or the `emptyFilterMessage` translation, the empty list message is `empty_message` or `emptyMessage` (as in PrimeNG: `emptyMessageLabel || emptyFilterMessageLabel`).

### D7. Markup, classes, attributes
Classes and `data-p` follow `selectstyle.ts` (`state.rs`, table-driven tests): root `p-select p-component p-inputwrapper` + modifiers; label `p-select-label` (+`p-placeholder`, `p-select-label-empty`); panel `p-select-overlay p-component-overlay p-component`; option `p-select-option` (+`p-select-option-selected` without `checkmark`, `p-disabled`, `p-focus`). The `id` is `pn_id_<n>` from `prime_core::ids::next_unique`, options are `{id}_{index}`, the list is `{id}_list`. The label is a `span[role=combobox]`, not an `<input>`. Extra element attributes are collected into `Vec<AnyAttribute>` when exceeding 26 (lesson from `port-inputnumber`).

### D8. Icons and templates
By default `ChevronDownIcon`, `TimesIcon`, `SpinnerIcon` (spinning), `CheckIcon`, `BlankIcon`, `SearchIcon`; `dropdown_icon`, `clear_icon`, `loading_icon` replace them (`ViewFn`). Templates: `item_template: Callback<SelectOption, AnyView>`, `selected_item_template: Callback<SelectOption, AnyView>`, `group_template: Callback<SelectGroup, AnyView>`.

### D9. CSS
`select`, `iconfield` in `theme.components` and `select` in `primeng_extra` (`upstream.toml`), `cargo xtask gen-theme`; `@primeuix/styles` has no separate `inputicon` module: the `.p-inputicon` classes are part of `iconfield`; we need to check that the CSS of other components has not changed.

### D10. Demos
`basic`, `checkmark`, `clearicon`, `disabled`, `filled`, `filter`, `fluid`, `group`, `invalid`, `loadingstate`, `sizes`, `template` (twelve). In `template`, instead of country flags (CDN images) there are symbolic labels; in `group` there are city groups. `basic` has a value `echo` for e2e.

### D11. Tests
Pure: `options.rs` (indices, filter, groups, type-ahead) and `state.rs`. SSR: closed markup, label and placeholder, clear icon, loading, classes and `data-p`, open panel (via `open`): options, groups, `aria-*`, checkmarks, search field, empty message, `pt`, branch parity. Browser (`tests/web.rs`): open and close (click, Escape, outside click, Tab), selection by mouse and key, skipping disabled, type-ahead, filter, groups, clear, `disabled`/`readonly`/`loading`, focus returns to the label and the label is not recreated, external `value`, `open`, reactive options. e2e: real keyboard and mouse, panel position under the field.

## Risks / Trade-offs

- [First overlay: position, z-index, outside close, clipping by parents with `overflow`] → the panel is inside the root and positioning comes from `prime-dom`; we document the absence of `appendTo` and animation.
- [Large amount of keyboard logic] → pure index functions with tables, behaviour from `onKeyDown` is pinned by browser tests.
- [The filter is simpler than PrimeNG (only "contains" on the label)] → recorded in the spec and `docs/select.md`.
- [String value and typed options instead of `optionLabel`/`optionValue`] → a deliberate difference, as with SelectButton.

## Migration Plan

No migration: a new crate and theme data. Rollback is `git revert`.

## Open Questions

None.
