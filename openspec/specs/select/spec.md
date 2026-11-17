# select Specification

## Purpose
Select lets the user choose a single value from a list of options in a dropdown panel with search, groups and full keyboard control.

## Requirements

### Requirement: Value and label
The `Select` component of the `prime-select` crate SHALL accept a two-way `value` (`RwSignal<Option<String>>`); without it, it SHALL keep internal state starting from `default_value`. The label (`span.p-select-label`) SHALL show the `label` of the option with the selected value (with groups, among the options of all groups); when nothing is selected, the `placeholder` with the `p-placeholder` class, and without a `placeholder`, an empty label with the `p-select-label-empty` class. `selected_item_template` SHALL replace the label content for the selected option. When `value` changes externally, the label SHALL update; a value that is not among the options SHALL be treated as unselected.

#### Scenario: Selected value
- **WHEN** the value equals the value of the option "New York"
- **THEN** the label shows "New York" without the `p-placeholder` class

#### Scenario: Placeholder
- **WHEN** there is no value and `placeholder = "Select a City"` is set
- **THEN** the label shows "Select a City" with the `p-placeholder` class

#### Scenario: Unknown value
- **WHEN** the value matches no option
- **THEN** the label shows the placeholder

### Requirement: Panel
The `div.p-select-overlay` panel SHALL open on a click on the field, the down and up arrows, `Enter`, `Space`, `Home`/`End` and typing a printable character, and programmatically via the two-way `open`; it SHALL close on a repeated click, `Escape`, `Tab`, option selection, a click outside the component and a window resize. An open panel SHALL be positioned below the field (above it if there is not enough room below) with the overlay group z-index, and `aria-expanded` of the label and the button SHALL be `true`. On open, the selected option SHALL be scrolled into view, and with `filter` focus SHALL move to the search field (except with `autofocus_filter = false`). Closing by `Escape` and by selection SHALL return focus to the label. With `disabled`, `readonly` and `loading` the panel SHALL NOT open. The `on_show` and `on_hide` events SHALL be called on open and close.

#### Scenario: Open by click
- **WHEN** the user clicks the closed field
- **THEN** the panel is open, `aria-expanded="true"`, `on_show` was called

#### Scenario: Close from outside
- **WHEN** the panel is open and the user presses outside the component
- **THEN** the panel is closed, `on_hide` was called

#### Scenario: Disabled
- **WHEN** `disabled` is set
- **THEN** a click on the field does not open the panel

### Requirement: Option selection
A click on an option SHALL select its value, close the panel and call `on_change` once with the new value and the original event; a click on an already selected option SHALL only close the panel. Disabled options (`disabled`) SHALL NOT be selectable and SHALL be skipped by the keyboard. The selected option SHALL have `aria-selected="true"` and the `p-select-option-selected` class (with `checkmark`, a `p-select-option-check-icon` checkmark, and the others get an empty `p-select-option-blank-icon` icon). The focused option SHALL have the `p-focus` class and `data-p-focused`; `focus_on_hover` (on by default) SHALL move focus to the option under the pointer. `item_template` SHALL replace the option content, receiving the option.

#### Scenario: Selection
- **WHEN** the panel is open and the user clicks "London"
- **THEN** the value equals the value of "London", the panel is closed, `on_change` was called once

#### Scenario: Disabled option
- **WHEN** the user clicks an option with `disabled`
- **THEN** the value and the panel state do not change

#### Scenario: Checkmark
- **WHEN** `checkmark` is set
- **THEN** the selected option has `p-select-option-check-icon`, the others have `p-select-option-blank-icon`

### Requirement: Keyboard
The label SHALL accept focus (`tabindex`, default `0`; `-1` with `disabled`). Arrow down SHALL open the panel, and in an open one move focus to the next enabled option; arrow up to the previous one (with `Alt` it selects the focused option and closes the panel); `Home` and `End` to the first and last enabled option; `PageUp` and `PageDown` scroll to the start and end of the list; `Enter` and `Space` on a closed panel open it, and on an open one select the focused option and close; `Escape` closes; `Tab` on an open panel selects the focused option (if any) and closes the panel; `Delete` with `show_clear` clears the value. Typing printable characters SHALL open the panel and move focus to the first option whose label starts with the typed text (accumulated characters reset after 500 ms). With `select_on_focus`, moving focus SHALL select the option without closing the panel. The label SHALL have `aria-activedescendant` pointing to the focused option.

#### Scenario: Navigation
- **WHEN** the panel is open, focus is on the first option and arrow down is pressed
- **THEN** focus is on the second, and disabled options are skipped

#### Scenario: Selection by Enter
- **WHEN** focus is on an option and `Enter` is pressed
- **THEN** the option is selected, the panel is closed, focus is on the label

#### Scenario: Type-ahead search
- **WHEN** the characters "Lo" are typed
- **THEN** focus is on the first option whose label starts with "lo", case-insensitive

#### Scenario: Close by Escape
- **WHEN** the panel is open and `Escape` is pressed
- **THEN** the panel is closed, the value did not change

### Requirement: Search
With `filter`, the panel SHALL contain a search field (`input.p-select-filter` in `p-iconfield` with a search icon and the `filter_placeholder` hint), and the list SHALL show options whose label contains the entered string, case-insensitive; with groups, only groups with matching options SHALL remain. Input SHALL reset option focus, call `on_filter` with the string and recompute the panel position. If there are no matching options, `li.p-select-empty-message` SHALL be rendered with `empty_filter_message` (default "No results found"). Keys in the search field SHALL work as in the list (arrows, `Enter`, `Escape`, `Tab`), while `Home`, `End` and the left and right arrows move the cursor. With `reset_filter_on_hide`, the string SHALL be reset on close.

#### Scenario: Filtering
- **WHEN** "or" is entered
- **THEN** the list has only options containing "or", case-insensitive

#### Scenario: No matches
- **WHEN** "zzz" is entered
- **THEN** the list contains only the message "No results found"

### Requirement: Groups and empty list
When `groups` are set, the list SHALL show for each group an `li.p-select-option-group` with a `span.p-select-option-group-label` (or `group_template`), followed by its options; group headers SHALL NOT be selectable and SHALL NOT receive keyboard focus. Without options, `li.p-select-empty-message` SHALL be rendered with `empty_message` (default "No results found"). Each option SHALL have `role="option"`, `aria-label`, `aria-setsize` (number of selectable options) and `aria-posinset` (number among the selectable ones).

#### Scenario: Group
- **WHEN** two groups of three options are set
- **THEN** the list contains two headers and six options in group order, `aria-setsize="6"`

#### Scenario: Empty list
- **WHEN** there are no options
- **THEN** the panel contains the message "No results found"

### Requirement: Clear and loading state
With `show_clear`, a selected value and not `disabled`, the `p-select-clear-icon` clear icon SHALL be rendered; a click on it SHALL reset the value to `None` without opening the panel and call `on_clear`. With `loading`, the `p-select-dropdown` button SHALL show the `p-select-loading-icon` loading icon instead of the arrow, and input and opening the panel SHALL be blocked.

#### Scenario: Clear
- **WHEN** an option is selected, `show_clear` is set and the icon is clicked
- **THEN** the value is `None`, the label shows the placeholder, the panel did not open, `on_clear` was called

#### Scenario: Loading
- **WHEN** `loading` is set
- **THEN** the loading icon replaces the arrow, a click on the field does not open the panel

### Requirement: Structure, classes and attributes
The root SHALL be `<div class="p-select p-component p-inputwrapper">` with the modifiers `p-disabled`, `p-variant-filled`, `p-focus`, `p-invalid`, `p-inputwrapper-filled`, `p-inputwrapper-focus` (focused or with an open panel), `p-select-open`, `p-select-fluid`, `p-select-sm p-inputfield-sm`, `p-select-lg p-inputfield-lg`, `style_class` and the attributes `id` and `data-p` (`invalid`, `disabled`, `focus`, `fluid`, `filled`, size). The label SHALL have `role="combobox"`, `aria-haspopup="listbox"`, `aria-expanded`, `aria-controls` (to the list when the panel is open), `aria-disabled`, `aria-label` (the given one or the option label), `aria-labelledby`, `aria-required`, `required`, `disabled`, `id` (`input_id`). The button SHALL have `role="button"`, `aria-label="dropdown trigger"`, `aria-haspopup="listbox"`, `aria-expanded`. The list SHALL have `role="listbox"`, `aria-label` from the `aria.listLabel` translation, and option identifiers `{id}_{index}`, where the index is counted over the list together with group headers.

#### Scenario: Root of an open field
- **WHEN** the panel is open and `size = Large` is set
- **THEN** the root has `p-select-open p-inputwrapper-focus p-select-lg p-inputfield-lg`

#### Scenario: Disabled field
- **WHEN** `disabled` is set
- **THEN** the root has `p-disabled`, the label has `tabindex="-1"`, `aria-disabled="true"`, `disabled`

### Requirement: Properties and pass-through
The properties `options`, `groups`, `placeholder`, `show_clear`, `checkmark`, `filter`, `loading`, `disabled`, `readonly`, `invalid`, `variant`, `size`, `fluid`, `scroll_height`, messages, labels, `style`, `style_class`, `panel_style`, `panel_style_class` SHALL accept a value or a signal and update the markup when the signal changes. `pt` SHALL accept the parts `root`, `label`, `clear_icon`, `dropdown`, `dropdown_icon`, `loading_icon`, `overlay`, `header`, `filter`, `list_container`, `list`, `option_group`, `option_group_label`, `option`, `option_label`, `option_check_icon`, `option_blank_icon`, `empty_message`, adding classes, style and attributes without replacing the component's own.

#### Scenario: Reactive options
- **WHEN** `options` is passed as a signal and grows while the panel is open
- **THEN** the list updates without closing the panel

#### Scenario: Class via pt
- **WHEN** the class `custom` is passed in `pt.option`
- **THEN** every option has `custom` and `p-select-option`

### Requirement: Select render cost
The number of allocations per instance in SSR (panel closed) SHALL fit the `select-1000` budget in `benches/alloc-budgets.toml`, and the contribution to wasm SHALL fit the `select` budget in `benches/budgets.toml`.

#### Scenario: Budgets
- **WHEN** `cargo xtask alloc-budget` and `cargo xtask wasm-size` are run
- **THEN** Select values do not exceed the budgets
