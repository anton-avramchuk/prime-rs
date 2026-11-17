# autocomplete Specification

## Purpose
AutoComplete is an input field that offers options for the typed text.

## Requirements

### Requirement: Markup
The `AutoComplete` of the `prime-autocomplete` crate SHALL render `div.p-autocomplete.p-component.p-inputwrapper` (state classes `p-invalid`, `p-focus`, `p-inputwrapper-filled|focus`, `p-autocomplete-open|clearable|fluid`). In single mode there SHALL be `input.p-autocomplete-input.p-inputtext[role=combobox][aria-autocomplete=list]` (`aria-expanded`, `aria-controls`, `aria-activedescendant`, `placeholder`, `type`, `disabled`, `readonly`) with the value being the label of the selected option or the typed text; in multiple mode, `ul.p-autocomplete-input-multiple[role=listbox]` with chips `li.p-autocomplete-chip-item` and `li.p-autocomplete-input-chip` > `input.p-autocomplete-input`. The `button.p-autocomplete-dropdown` button SHALL be rendered with `dropdown`, the `p-autocomplete-clear-icon` clear icon with `show_clear`, a non-empty value and an enabled field, the `p-autocomplete-loader` spinner while a query is in progress. The `div.p-autocomplete-overlay` panel SHALL be rendered only in the open state.

#### Scenario: Single mode
- **WHEN** the value is "Apple"
- **THEN** the field shows "Apple", there is no panel

#### Scenario: Multiple mode
- **WHEN** "A" and "B" are selected
- **THEN** two chips and the input field are rendered

### Requirement: Option query
Entering text of at least `min_length` characters SHALL call `on_complete` with the query after `delay` (by default 1 character and 300 ms); shorter text and clearing the field SHALL close the panel. Until `suggestions` are updated, a spinner SHALL be shown. Updated options SHALL open the panel (with an empty list, the `empty_message` message if `show_empty_message`). In single mode without `force_selection`, the typed text SHALL become the value. The arrow button SHALL request options (`dropdown_mode`: an empty query or the current text) or close an open panel; `complete_on_focus` SHALL request options on first focus.

#### Scenario: Query
- **WHEN** the user types "ap" and waits for `delay`
- **THEN** `on_complete` is called with the query "ap"

### Requirement: Selection
A click on an option or `Enter` on the focused one SHALL write the option value (in `multiple`, add a chip if `unique` and it is not there yet), call `on_select` and close the panel. Disabled options and group headers SHALL NOT be selectable and SHALL be skipped by keys. A chip SHALL be removed by its button, and `Backspace` in an empty field removes the last chip. `force_selection` SHALL clear input that matches no option. Clear SHALL reset the value and call `on_clear`.

#### Scenario: Selection
- **WHEN** the panel is open and the user clicks the option "Apple"
- **THEN** the value is "Apple", the panel is closed, `on_select` was called

### Requirement: Keyboard
Down/Up SHALL walk over the eligible options of the open panel, `Alt+Up` SHALL select the focused one and close; `Escape` SHALL close and return focus; `Tab` SHALL select the focused option and close. In `multiple`, Left on an empty field SHALL move to the chips, Left/Right walk over the chips, `Backspace` removes the focused one. With `typeahead = false` in `multiple`, the typed text SHALL be added on `Enter`, `separator`, `Tab` (`add_on_tab`) and blur (`add_on_blur`); a paste containing `separator` is split into chips.

#### Scenario: Arrows
- **WHEN** the panel is open and Down Down is pressed
- **THEN** the second eligible option is focused

### Requirement: Templates and states
`item` SHALL replace the option content, `group_view` the group header, `selected_item` the chip content. With `disabled`, the field SHALL NOT react to input and clicks; a click outside the component SHALL close the panel.

#### Scenario: Custom option
- **WHEN** `item` is set
- **THEN** the `li.p-autocomplete-option` contains its content
