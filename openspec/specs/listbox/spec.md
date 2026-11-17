# listbox Specification

## Purpose
Listbox shows a list of options (including grouped ones) and lets the user select one or several values with mouse and keys, with search and checkboxes.

## Requirements

### Requirement: Markup
`Listbox` of the `prime-listbox` crate SHALL render `div.p-listbox.p-component` with `data-p`, inside it `div.p-listbox-list-container` (height `scroll_height`, default `14rem`) > `ul.p-listbox-list[role=listbox]` with `tabindex`, `aria-label`, `aria-multiselectable` (with `multiple`), `aria-disabled`, `aria-activedescendant` (while focused) and options `li.p-listbox-option[role=option]` with `id` of the form `<id>_<index>`, `aria-label`, `aria-selected`, `aria-disabled`, `aria-setsize`, `data-p-selected`, `data-p-focused`, `data-p-disabled`. `style_class`, `style` and `pt` SHALL apply to the root; `list_style`/`list_style_class` to the list. Root classes: `p-listbox-striped`, `p-disabled`, `p-invalid`, `p-listbox-fluid`. Option classes: `p-listbox-option-selected` (selected and `highlight_on_select`), `p-focus`, `p-disabled`.

#### Scenario: Simple list
- **WHEN** a `Listbox` with three options is rendered
- **THEN** three `li.p-listbox-option[role=option]` with labels are rendered, none selected

#### Scenario: Selected value
- **WHEN** `value` equals the value of one of the options
- **THEN** that option has `aria-selected=true` and the class `p-listbox-option-selected`

### Requirement: Groups and empty list
If `groups` is set, `Listbox` SHALL render `li.p-listbox-option-group[role=option]` with a header followed by the group's options; headers are not counted in `aria-setsize`/`aria-posinset` and are not selectable. When there are no options, `div.p-listbox-empty-message` SHALL be rendered with `empty_message`, and for an empty search result with `empty_filter_message`.

#### Scenario: Group
- **WHEN** two groups of two options are set
- **THEN** two headers and four options are rendered, `aria-setsize=4`

#### Scenario: Empty search result
- **WHEN** the search string matches no option
- **THEN** `div.p-listbox-empty-message` with `empty_filter_message` is rendered

### Requirement: Single selection
Without `multiple`, a click on an enabled option SHALL write its value to `value` and call `on_change`; a repeated click on the selected one SHALL clear the value (`None`). With `meta_key_selection`, a repeated click without `Ctrl`/`⌘` SHALL keep the selection, and with them SHALL clear it. A disabled option and `readonly`/`disabled` mode SHALL ignore clicks.

#### Scenario: Select and clear
- **WHEN** the user clicks an option, then the same one again
- **THEN** `value` first equals the option's value, then `None`; `on_change` is called twice

#### Scenario: Disabled option
- **WHEN** the user clicks a disabled option
- **THEN** `value` does not change, `on_change` is not called

### Requirement: Multiple selection
With `multiple`, a click SHALL add an option to `values` or remove it. With `meta_key_selection`, a click without `Ctrl`/`⌘` SHALL replace the selection with one option, and with them add/remove. `Shift`+↓/↑ SHALL select a range from the anchor option to the focused one, `Ctrl+Shift+Home/End` up to the first/last, `Ctrl+A` all enabled visible options.

#### Scenario: Add and remove
- **WHEN** the user clicks two options, then the first one
- **THEN** `values` contains only the second

#### Scenario: Range
- **WHEN** focus is on the first option and `Shift`+↓ is pressed twice
- **THEN** `values` contains the first three enabled options

#### Scenario: All
- **WHEN** `Ctrl+A` is pressed
- **THEN** `values` contains all enabled visible options

### Requirement: Checkboxes and "select all"
With `checkbox` and `multiple`, each option SHALL contain a `Checkbox` with `p-listbox-option-check-icon`, and with `show_toggle_all` the header SHALL contain a "select all" checkbox: it selects all enabled visible options (respecting search) or clears the selection if all are already selected, and calls `on_select_all_change` and `on_change`. With `checkmark`, the selected option SHALL show a tick icon (`p-listbox-option-check-icon`), an unselected one an empty icon (`p-listbox-option-blank-icon`), and `p-listbox-option-selected` SHALL NOT be added.

#### Scenario: Select all
- **WHEN** the user clicks the header checkbox
- **THEN** all enabled options are selected, the checkbox is checked

#### Scenario: Select all with search
- **WHEN** the search string leaves two options and the "select all" checkbox is clicked
- **THEN** only these two are selected

#### Scenario: Checkmark
- **WHEN** `checkmark` is set and an option is selected
- **THEN** the option has `.p-listbox-option-check-icon` and no class `p-listbox-option-selected`

### Requirement: Search
With `filter`, the header SHALL contain `span.p-iconfield` with the field `input.p-listbox-filter[role=searchbox]` (`aria-owns`, `aria-activedescendant`, `placeholder` from `filter_placeholder`, `aria-label` from `aria_filter_label`) and the icon `p-inputicon`. Input SHALL leave options whose label contains the string (case-insensitive, leading and trailing whitespace dropped); groups without matching options are omitted; the focused option is cleared; `on_filter` is called. Arrows, `Home`/`End` (in the field they move the cursor) and `Enter` in the field SHALL work as in PrimeNG.

#### Scenario: Filtering
- **WHEN** the user types "lon"
- **THEN** options containing "lon" in any case remain, `on_filter` is called with the string

### Requirement: Keyboard
Focus on the list SHALL make the selected option focused (with `auto_option_focus`, the first enabled one), `Tab` is not intercepted. ↓/↑ SHALL move to the next/previous enabled option, `Home`/`End` to the first/last, `PageUp`/`PageDown` scroll, `Enter`/`Space` select the focused option, a printable character moves to the next option whose label starts with the typed string (characters accumulate for 500 ms). With `focus_on_hover` and a focused list, mouse hover SHALL move the focused option. With `select_on_focus`, moving focus with arrows (without `Shift`) SHALL select the option. `on_focus`/`on_blur` SHALL be called when focus enters/leaves.

#### Scenario: Navigation
- **WHEN** the list is focused and ↓ is pressed twice
- **THEN** the focused option is the third (if enabled), `aria-activedescendant` points to it

#### Scenario: Selection by key
- **WHEN** an option is focused and Enter is pressed
- **THEN** the option is selected

#### Scenario: Type-ahead search
- **WHEN** a letter is typed that starts the label of one of the options
- **THEN** focus moves to it

### Requirement: Disabled state and validity
With `disabled`, the root SHALL get `p-disabled`, the list `aria-disabled`, `tabindex=-1`; selection and keys SHALL NOT work. With `readonly`, the selection SHALL NOT change, the list remains focusable. With `invalid`, the root SHALL get `p-invalid`. `fluid` without a value SHALL be taken from the `Fluid` context.

#### Scenario: Disabled
- **WHEN** `disabled=true` and the user clicks an option
- **THEN** the value does not change

### Requirement: Mouse events
A click on an enabled option SHALL call `on_click`; a double click calls `on_dbl_click` (not with `readonly`/`disabled`/a disabled option). A click SHALL move the focused option to the clicked one.

#### Scenario: Double click
- **WHEN** the user double-clicks an option
- **THEN** `on_dbl_click` is called with the option's value

### Requirement: Custom views
`option` SHALL replace the content of `li.p-listbox-option` (except the checkbox/checkmark) and receive the option and the selected flag; `group` the group header content.

#### Scenario: Option template
- **WHEN** `option` is set
- **THEN** the passed view is inside the option

### Requirement: Rendering without a browser
Outside the browser (SSR) `Listbox` SHALL render completely (including selected values and checkboxes); `window` and `document` SHALL NOT be used.

#### Scenario: SSR
- **WHEN** a page with `Listbox` is rendered on the server
- **THEN** the markup has `ul.p-listbox-list` with options, selected ones are marked
