# Proposal

## Why

AutoComplete is an input field with suggestions: the user types, the component requests options (`on_complete`) and shows them in a panel. It needs a multiple selection mode with chips, an arrow button, clear, groups and `force_selection`. The shell and panel are as in `Select`/`TreeSelect`, the field uses `InputText` classes, chips use `Chip`.

## What Changes

- New crate `prime-autocomplete` (port of PrimeNG 21.1.10 `p-autocomplete`).
- Model: `AutoCompleteOption` with `label`, `value` (defaults to `label`), `disabled`, `items` (nested options, a group when `group = true`). The value in single mode is `RwSignal<String>` (the typed text or the value of the selected option), in multiple mode `RwSignal<Vec<String>>`.
- `AutoComplete`: root `div.p-autocomplete.p-component.p-inputwrapper` (`p-invalid`, `p-focus`, `p-inputwrapper-filled|focus`, `p-autocomplete-open|clearable|fluid`) > single mode: `input.p-autocomplete-input.p-inputtext[role=combobox][aria-autocomplete=list]`; multiple: `ul.p-autocomplete-input-multiple[role=listbox]` > `li.p-autocomplete-chip-item` (`Chip.p-autocomplete-chip` with a remove button) and `li.p-autocomplete-input-chip` > `input.p-autocomplete-input`; clear icon `p-autocomplete-clear-icon`, spinner `p-autocomplete-loader`, button `button.p-autocomplete-dropdown`; panel `div.p-autocomplete-overlay` > `div.p-autocomplete-list-container` (`max-height` = `scroll_height`, `200px`) > `ul.p-autocomplete-list[role=listbox]` > `li.p-autocomplete-option[role=option]` (`-selected`, `p-focus`, `p-disabled`; `aria-label|selected|disabled|setsize|posinset`, `data-p-selected|focused`), `li.p-autocomplete-option-group`, `li.p-autocomplete-empty-message`.
- Behaviour: input of at least `min_length` characters calls `on_complete` with the query after `delay` (300 ms) and shows the spinner until `suggestions` are updated; new options open the panel (with an empty list, a message if `show_empty_message`); selecting an option writes the value (in `multiple` it adds a chip, `unique`) and closes the panel; the arrow button requests options (`dropdown_mode`: an empty query or the current text); `complete_on_focus`; `force_selection` clears a non-matching input; clear; a click outside closes; `typeahead = false` in `multiple` adds the typed text on `Enter`, the separator, `Tab` (`add_on_tab`), blur (`add_on_blur`) and paste (`separator`).
- Keyboard: Down/Up over options (`Alt+Up` selects and closes), `Enter` selects, `Escape` closes, `Tab` selects the focused one and closes, `Home`/`End` move the caret, `PageDown`/`PageUp` scroll, `Backspace` on an empty `multiple` field removes the last chip; Left on an empty `multiple` field moves to the chips (Left/Right over them, `Backspace` removes).
- Properties: `suggestions`, `value`/`values`, `multiple`, `dropdown`, `dropdown_mode`, `show_clear`, `force_selection`, `complete_on_focus`, `auto_option_focus`, `focus_on_hover`, `min_length`, `delay`, `placeholder`, `scroll_height`, `typeahead`, `unique`, `group`, `show_empty_message`, `empty_message`, `add_on_tab`, `add_on_blur`, `separator`, `disabled`, `readonly`, `invalid`, `variant`, `size`, `fluid`, `input_type`, `input_id`, `tabindex`, `aria_label`, `aria_labelledby`, `id`, `style`, `style_class`, `input_style_class`, `panel_style_class`; events `on_complete`, `on_select`, `on_unselect`, `on_add`, `on_clear`, `on_dropdown_click`, `on_show`, `on_hide`, `on_focus`, `on_blur`; views `item`, `group_view`, `selected_item`.
- The `/autocomplete` page (`basic`, `dropdown`, `multiple`, `forceselection`, `group`, `clearicon`, `disabled`, `invalid`, `template`), e2e, benchmarks and budgets, `docs/autocomplete.md`, `autocomplete` in `theme.components` and `primeng_extra`.
- **Out of scope:** objects as options (`optionLabel`/`optionValue`/`dataKey`), virtual scrolling and `lazy`, Angular forms, `appendTo`, `Ripple`, `Bind`, `pTemplate`, `header`/`footer`/`loader`/icon templates, screen reader messages (`aria-live`), `onKeyUp`/`onInputKeydown`.

## Capabilities

### New Capabilities
- `autocomplete`: an input field with suggestions, single and multiple mode.

### Modified Capabilities
<!-- None. -->

## Impact

- New crate `crates/prime-autocomplete` (`publish = false`); dependencies: `prime-core`, `prime-dom`, `prime-inputtext` (classes), `prime-chip`, `prime-icons` (chevrondown, times, timescircle, spinner).
- Changed: workspace `Cargo.toml`, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml`.
