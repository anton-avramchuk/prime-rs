# tree Specification

## Purpose
Tree shows hierarchical data with expansion, node selection (including checkboxes) and a filter.

## Requirements

### Requirement: Markup
`Tree` of the `prime-tree` crate SHALL accept `Vec<TreeNode>` and render `div.p-tree.p-component` (`p-tree-selectable` when a selection mode is set, `p-tree-loading` when `loading`, `p-tree-flex-scrollable` when `scroll_height = "flex"`), `div.p-tree-root` and `ul.p-tree-root-children[role=tree]` (`aria-label`, `aria-labelledby`). A node SHALL be `li.p-tree-node[role=treeitem]` (`p-tree-node-leaf` on a leaf; `aria-label`, `aria-level`, `aria-setsize`, `aria-posinset`, `aria-expanded`, `aria-selected` in `Single`/`Multiple` modes, `aria-checked` in `Checkbox`, `tabindex`, `data-id` = `key`) with `div.p-tree-node-content` (`p-tree-node-selectable`, `p-tree-node-selected`; left indent `level × indentation` rem), a `button.p-tree-node-toggle-button`, a `p-tree-node-checkbox` checkbox (in `Checkbox` mode), a `p-tree-node-icon` icon, a `p-tree-node-label` label and, on an expanded node with children, `ul.p-tree-node-children[role=group]`. With an empty list and no loading, `div.p-tree-empty-message` SHALL be rendered.

#### Scenario: Composition
- **WHEN** a tree with an expanded node of two children is rendered
- **THEN** `ul.p-tree-root-children`, the node, the chevron button and `ul.p-tree-node-children` with two `li` are rendered

#### Scenario: Empty
- **WHEN** `value` is empty
- **THEN** the message from `empty_message` is rendered

### Requirement: Expansion
A click on a node's button SHALL toggle its expansion and call `on_node_expand`/`on_node_collapse`; the chevron SHALL point right on a collapsed node and down on an expanded one. Initial expansion SHALL come from `TreeNode::expanded` or from the passed `expanded_keys` signal. A node with `loading = true` and `loading_mode = Icon` SHALL show a spinner instead of the chevron.

#### Scenario: Toggle
- **WHEN** the user clicks the button of a collapsed node
- **THEN** the node is expanded, `on_node_expand` is called

### Requirement: Selection
In `Single` mode a click on a node SHALL select it, a repeated one — deselect it; in `Multiple` — add to and remove from the selection (with `meta_key_selection` without Ctrl/Meta the selection is replaced, with it — extended or reduced). In `Checkbox` mode a click SHALL toggle the node and, with `propagate_selection_down`, all descendants, and with `propagate_selection_up` — recompute ancestors (an ancestor is selected when all its children are selected; a partially selected ancestor gets `data-p-partialchecked` and the partial checkbox state). Nodes with `selectable = false` SHALL NOT be selectable. A click on the toggle button SHALL NOT change the selection. The `on_node_select`/`on_node_unselect` events SHALL be called on select/deselect.

#### Scenario: Single selection
- **WHEN** the user clicks a node in `Single` mode, then the same node again
- **THEN** first the key is in the selection and `aria-selected="true"`, then the selection is empty

#### Scenario: Checkboxes
- **WHEN** in `Checkbox` mode all children of a node are checked
- **THEN** the node is checked too; if not all are checked — the node is partially selected

### Requirement: Filter
With `filter`, a search field with an icon SHALL be rendered. Input SHALL keep the nodes whose `filter_by` (comma-separated fields: `label`, `key`) contains the text, case-insensitively; in `Lenient` mode a matching node keeps all its children, in `Strict` — only matching ones; ancestors of matching nodes stay and are expanded. An empty field SHALL restore the full tree; `on_filter` is called.

#### Scenario: Match of a nested node
- **WHEN** text is entered that matches only the label of a nested node
- **THEN** it and its ancestors stay, the ancestors are expanded

### Requirement: Keyboard
One node SHALL have `tabindex="0"` (the selected one, else the first), the rest `-1`. ↓/↑ SHALL move to the next/previous visible node, → — expand a collapsed node and move to its first descendant, ← — collapse an expanded one or go to the parent, `Enter`/`Space` — select the node, `Home`/`End` — to the first/last visible one, `Tab` — leave the tree.

#### Scenario: Arrows
- **WHEN** focus is on a collapsed node with children, → is pressed
- **THEN** the node is expanded, focus is on the first child

### Requirement: Templates
`node` SHALL replace the node's label content (receives the node), `togglericon` — the chevron.

#### Scenario: Custom node
- **WHEN** `node` is set
- **THEN** its content is rendered inside `p-tree-node-label`
