# cascadeselect Specification

## Purpose
CascadeSelect lets the user pick a value from nested lists that unfold as a cascade.

## Requirements

### Requirement: Markup
`CascadeSelect` of crate `prime-cascadeselect` SHALL accept `Vec<CascadeOption>` and render `div.p-cascadeselect.p-component.p-inputwrapper` with state classes, a hidden input `input[role=combobox][readonly]` (`aria-haspopup="tree"`, `aria-expanded`, `aria-controls`, `aria-activedescendant`), a label `span.p-cascadeselect-label` (label of the selected leaf or `placeholder` with `p-placeholder`; with no placeholder an empty one gets `p-cascadeselect-label-empty`) and a button `div.p-cascadeselect-dropdown` with an arrow (a spinner when `loading`). The panel `div.p-cascadeselect-overlay` > `div.p-cascadeselect-list-container` > `ul.p-cascadeselect-list[role=tree]` SHALL be rendered only when the field is open. An option SHALL be `li.p-cascadeselect-option[role=treeitem]` with `aria-level`, `aria-setsize`, `aria-posinset`, `aria-label`, `aria-selected` (on leaves), `id` and content `div.p-cascadeselect-option-content` > `span.p-cascadeselect-option-text`; a group has `p-cascadeselect-option-group` and `span.p-cascadeselect-group-icon`.

#### Scenario: Closed
- **WHEN** the leaf "Sydney" is selected
- **THEN** the label contains "Sydney", there is no panel

#### Scenario: Open
- **WHEN** the field is open
- **THEN** the root list with groups and leaves is rendered

### Requirement: Expanding and selecting
A click on the field SHALL open or close the panel. A click on a group SHALL expand its nested list `ul.p-cascadeselect-option-list[role=group]` (`p-cascadeselect-option-active` on the group), a repeated click SHALL collapse it. A click on a leaf SHALL write its value, call `on_change` and close the panel. Hover SHALL expand groups (before the first click — root ones only; after a click or with a selected value — any); in a narrow window the path SHALL NOT change. On open the path to the selected value SHALL be expanded. An outside click and `resize` SHALL close the panel; closing resets the path.

#### Scenario: Cascade
- **WHEN** the user clicks a root group, then a group in the expanded list
- **THEN** two nested lists are rendered, both groups have `p-cascadeselect-option-active`

#### Scenario: Leaf
- **WHEN** the user clicks a leaf
- **THEN** the value is written, the panel is closed, the label is updated

### Requirement: Clearing
With `show_clear`, a non-empty value and an available field the icon `p-cascadeselect-clear-icon` SHALL be rendered; a click on it SHALL clear the value, call `on_clear` and not open the panel.

#### Scenario: Clearing
- **WHEN** the user clicks the clear icon
- **THEN** the value is empty, the label shows the placeholder

### Requirement: Keyboard
On the hidden input ↓ SHALL open the panel, and in an open one move to the next eligible option of the level; focus on a group expands its list. ↑ — to the previous one (opens the panel if closed), `Alt+↑` — closes. → on a group SHALL expand it and move to the first option of the nested list, ← — close the list and return focus to the parent group. `Home`/`End`, `Enter`/`Space` (click on an option; on a leaf — closing), `Escape` (close and return focus), `Tab` (close the panel), `Backspace` (clear with `show_clear`) and search by label prefix SHALL work as in PrimeNG. Unavailable options SHALL be skipped.

#### Scenario: Entering a group
- **WHEN** the panel is open, focus is on a group and → is pressed
- **THEN** the group list is expanded, focus is on its first option

### Requirement: Mobile mode
While the window is not wider than `breakpoint`, the root SHALL have `p-cascadeselect-mobile`, the panel `p-cascadeselect-mobile-active`, and hover SHALL NOT change the path.

#### Scenario: Narrow window
- **WHEN** the window is narrower than `breakpoint`
- **THEN** the root has `p-cascadeselect-mobile`
