# treeselect Specification

## Purpose
TreeSelect lets the user select a node or several nodes of a tree in a dropdown panel.

## Requirements

### Requirement: Markup
`TreeSelect` of the `prime-treeselect` crate SHALL accept `Vec<TreeNode>` and render `div.p-treeselect.p-component.p-inputwrapper` (state classes: `p-treeselect-display-chip`, `p-disabled`, `p-invalid`, `p-focus`, `p-variant-filled`, `p-inputwrapper-filled`, `p-inputwrapper-focus`, `p-treeselect-open`, `p-treeselect-clearable`, `p-treeselect-fluid`, `p-treeselect-sm`, `p-treeselect-lg`), a hidden field `input[role=combobox][readonly]` (`aria-haspopup="tree"`, `aria-expanded`, `aria-controls`, `tabindex`, `disabled`), a label container with `div.p-treeselect-label` and a button `div.p-treeselect-dropdown[role=button]` with an arrow. The label SHALL show the captions of selected nodes separated by commas or the `placeholder` (`p-placeholder`), and with `display = Chip` — chips `p-treeselect-chip-item` > `p-treeselect-chip`. The panel SHALL be rendered only in the open state.

#### Scenario: Closed
- **WHEN** the node "Work" is selected
- **THEN** the label contains "Work", there is no panel, `aria-expanded="false"`

#### Scenario: Empty
- **WHEN** nothing is selected
- **THEN** the label shows the `placeholder`

### Requirement: Opening and selection
A click on the field SHALL open or close the panel `div.p-treeselect-overlay` with `div.p-treeselect-tree-container` (`max-height` = `scroll_height`, `400px` by default) and the tree; the panel SHALL be inside the root and aligned to the field. On open the ancestors of selected nodes SHALL be expanded. In `Single` mode selecting a node SHALL close the panel and return focus to the field; in `Multiple` and `Checkbox` the panel stays open. The value SHALL be a two-way signal of keys. A click outside the component, `resize` and `Escape` SHALL close the panel.

#### Scenario: Single selection
- **WHEN** the panel is open and the user clicks a node
- **THEN** the node key is in the value, the panel is closed, `on_hide` is called

#### Scenario: Ancestors of the selection
- **WHEN** the value contains a nested node and the panel opens
- **THEN** its ancestors are expanded

### Requirement: Clear
With `show_clear` and a non-empty value the icon `p-treeselect-clear-icon` SHALL be rendered; a click on it SHALL clear the value, call `on_clear` and not open the panel.

#### Scenario: Clear
- **WHEN** the user clicks the clear icon
- **THEN** the value is empty, the panel is not open

### Requirement: Keyboard
On the hidden field Down SHALL open the panel and move focus to the first tree node, `Space`/`Enter` — open, `Escape` — close the panel and leave focus on the field, `Tab` — close the panel. A disabled field SHALL NOT react to clicks and keys.

#### Scenario: Down arrow
- **WHEN** focus is on the field and Down is pressed
- **THEN** the panel is open, focus is on the first tree node

### Requirement: Tree parameters
The selection mode, `meta_key_selection`, selection propagation, the filter (`filter`, `filter_by`, `filter_mode`, `filter_placeholder`), `empty_message`, `loading` SHALL be passed to `Tree`; the filter SHALL reset when the panel closes.

#### Scenario: Filter
- **WHEN** `filter` is set and the panel is open
- **THEN** the panel contains a tree filter field
