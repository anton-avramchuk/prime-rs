# multiselect Specification

## Purpose
MultiSelect lets the user select several values from a list of options in a dropdown panel with search, groups, checkboxes, chip mode and keyboard control.

## ADDED Requirements

### Requirement: Values and label
`MultiSelect` of the `prime-multiselect` crate SHALL accept a two-way `values` (`RwSignal<Vec<String>>`); without it SHALL keep internal state starting with `default_values`. The `div.p-multiselect-label` label in `comma` mode SHALL show the labels of the selected options joined with ", " in selection order, and when more values than `max_selected_labels` are selected — `selected_items_label` (by default "{0} items selected" from the translation, `{0}` is the selected count). With no selection the label SHALL show `placeholder` with the `p-placeholder` class, and without `placeholder` — an empty label with the `p-multiselect-label-empty` class. With `display_selected_label = false` the label SHALL show `placeholder`. `selected_items` SHALL replace the label content. Values that are not among the options SHALL NOT appear in the label.

#### Scenario: List of labels
- **WHEN** "New York" and "London" are selected, `max_selected_labels` is 3
- **THEN** the label shows "New York, London"

#### Scenario: Summary
- **WHEN** four values are selected, `max_selected_labels` is 3
- **THEN** the label shows "4 items selected"

#### Scenario: Hint
- **WHEN** nothing is selected and `placeholder = "Select Cities"` is set
- **THEN** the label shows "Select Cities" with the `p-placeholder` class

### Requirement: Chip mode
With `display = Chip` the root SHALL get the `p-multiselect-display-chip` class, and the label — one `div.p-multiselect-chip-item` with a `Chip` (`p-multiselect-chip`) per selected option, at most `max_selected_labels`; when the limit is exceeded `selected_items_label` SHALL be shown. A chip, if the field is available (not `disabled`, not `readonly`), SHALL have a remove icon (`chip_icon` or the standard one); a click on it SHALL remove the value, call `on_remove` and `on_change`, without opening the panel.

#### Scenario: Chips
- **WHEN** two values are selected in `chip` mode
- **THEN** two `.p-multiselect-chip-item` with labels are rendered

#### Scenario: Chip removal
- **WHEN** the user clicks the chip remove icon
- **THEN** the value is removed from `values`, the panel did not open, `on_remove` and `on_change` were called

### Requirement: Panel
The `div.p-multiselect-overlay` panel SHALL open on a click on the field, the down and up arrows, `Enter`, `Space`, `Home`/`End` and typing a printable character, and programmatically through the two-way `open`; it SHALL close on a repeated click, `Escape`, `Tab`, a click outside the component, `resize` and `Alt+↑`. Selecting an option SHALL keep the panel open. An open panel SHALL be positioned below the field (above it if there is not enough room below) with the overlay group z-index, and the field's `aria-expanded` SHALL be `true`. With `filter` and `autofocus_filter` focus SHALL move to the search field. Closing with `Escape` SHALL return focus to the field. With `disabled`, `readonly` and `loading` the panel SHALL NOT open. `on_show` and `on_hide` SHALL be called on open and close; `reset_filter_on_hide` SHALL reset the search string on close.

#### Scenario: Open on click
- **WHEN** the user clicks the closed field
- **THEN** the panel is open, `aria-expanded="true"`, `on_show` was called

#### Scenario: Selection does not close
- **WHEN** the panel is open and the user clicks an option
- **THEN** the value is added, the panel stayed open

#### Scenario: Disabled
- **WHEN** `disabled` is set
- **THEN** a click on the field does not open the panel

### Requirement: Option selection
A click on an `li.p-multiselect-option[role=option]` option SHALL add its value to `values` or remove it and call `on_change` once with the list of values and the original event. Disabled options SHALL NOT be selected and SHALL be skipped by the keyboard. A selected option SHALL have `aria-selected="true"`, a checkbox with the `p-checkbox-checked` class and (with `highlight_on_select`) the `p-multiselect-option-selected` class; the focused option — the `p-focus` class and `data-p-focused`; `focus_on_hover` SHALL move focus to the option under the pointer. `aria-setsize`/`aria-posinset` SHALL NOT count group headers. `item` SHALL replace the option content (after the checkbox), receiving the option and the selected flag.

#### Scenario: Add and remove
- **WHEN** the user clicks two options, then the first one
- **THEN** `values` contains only the second, `on_change` was called three times

#### Scenario: Disabled option
- **WHEN** the user clicks an option with `disabled`
- **THEN** `values` does not change

### Requirement: Selection limit
With `selection_limit` and the number of selected values equal to the limit, all unselected options SHALL be considered disabled (the `p-disabled` class, not selectable, skipped by the keyboard), the selected ones SHALL stay available for deselection, and the "select all" checkbox SHALL NOT be rendered.

#### Scenario: Limit reached
- **WHEN** `selection_limit = 2` and two values are selected
- **THEN** the other options are disabled, deselecting a selected one is possible

### Requirement: Select all
The `div.p-multiselect-header` header (with `show_header`) SHALL, with `show_toggle_all` and no `selection_limit`, contain a "select all" checkbox. It SHALL select all available visible options (accounting for search, without losing those already selected outside the search result and the selected disabled ones), and if all available visible ones are already selected — deselect them (selected disabled ones remain). The checkbox SHALL be checked when all available visible options are selected. A click SHALL call `on_select_all_change` and `on_change`.

#### Scenario: Select all with search
- **WHEN** the search leaves two options and the checkbox is clicked
- **THEN** these two and the previously selected ones are selected

#### Scenario: Deselect all
- **WHEN** all available visible options are selected and the checkbox is clicked
- **THEN** the selection is cleared, except the selected disabled ones

### Requirement: Search
With `filter` the header SHALL contain a `p-iconfield.p-multiselect-filter-container` with an `input.p-multiselect-filter[role=searchbox]` field (`aria-owns`, `aria-activedescendant`, `placeholder` from `filter_placeholder`, `aria-label` from `aria_filter_label`) and a `p-inputicon` icon. Input SHALL keep the options whose label contains the string (case-insensitive); groups without matching options are omitted; the focused option is reset; `on_filter` is called; with an empty result `li.p-multiselect-empty-message` with `empty_filter_message` SHALL be rendered, with an empty list — with `empty_message`.

#### Scenario: Filtering
- **WHEN** the user types "lon"
- **THEN** the options containing "lon" in any case remain, `on_filter` is called with the string

#### Scenario: Empty result
- **WHEN** the string matches no option
- **THEN** `li.p-multiselect-empty-message` is rendered

### Requirement: Keyboard
The hidden `combobox` field SHALL accept focus (`tabindex`, `0` by default; `-1` when `disabled`) and have `aria-activedescendant` pointing to the focused option. ↓/↑ SHALL open the panel and move focus to the next/previous available option (`Shift` — select a range from the anchor); `Alt+↑` SHALL select the focused option and close the panel; `Home`/`End` — to the first/last available one (`Ctrl+Shift` — range); `PageUp`/`PageDown` — scroll to the start/end; `Enter`/`Space` on a closed panel open it, on an open one — toggle the focused option (`Shift` — range); `Escape` — close; `Tab` — select the focused option if it is not selected, and close the panel; `Ctrl+A` — select all available visible options; a printable character — open the panel and go to the option whose label starts with the typed string (accumulation 500 ms). With `select_on_focus`, moving focus with the keys SHALL select the option.

#### Scenario: Navigation and selection
- **WHEN** the field is focused, ↓ is pressed (the panel opens), then ↓ and Enter
- **THEN** the second available option is selected, the panel is open

#### Scenario: Escape
- **WHEN** the panel is open and Escape is pressed
- **THEN** the panel is closed, focus is on the field

### Requirement: Clear, loading, blocking
With `show_clear` and selected values the `p-multiselect-clear-icon` icon SHALL be rendered; a click on it SHALL reset `values`, call `on_change` and `on_clear` and not open the panel. With `loading` the dropdown button SHALL show the `p-multiselect-loading-icon` loading icon. The root SHALL get `p-disabled`, `p-invalid`, `p-variant-filled`, `p-focus`, `p-inputwrapper-filled`, `p-inputwrapper-focus`, `p-multiselect-open`, `p-multiselect-fluid`, `p-multiselect-sm`/`-lg` by state; `fluid` without a value SHALL come from the `Fluid` context. With `readonly` and `disabled`, selection, clearing and chip removal SHALL NOT work.

#### Scenario: Clear
- **WHEN** the user clicks the clear icon
- **THEN** `values` is empty, the panel did not open, `on_change` and `on_clear` were called

### Requirement: Rendering without a browser
Outside the browser (SSR) `MultiSelect` SHALL render fully with a closed panel (label, chips, icons); `window` and `document` SHALL NOT be used.

#### Scenario: SSR
- **WHEN** a page with `MultiSelect` and selected values is rendered on the server
- **THEN** the markup contains the root, a label with the selected values and no panel
