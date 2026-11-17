# Proposal

## Why

Tree is the next wave 5 component: hierarchical data with expansion, selection (single, multiple, checkboxes) and filtering. It is needed both on its own and as the base for TreeSelect. Expansion and keyboard are similar to `PanelMenu`, selection to `Listbox`/`MultiSelect`, and the checkbox and filter field are built on existing crates.

## What Changes

- New crate `prime-tree` (a port of PrimeNG 21.1.10 `p-tree`).
- Model: `TreeNode` (`key` — a required unique key, `label`, `icon`, `expanded_icon`, `collapsed_icon`, `children`, `leaf`, `expanded`, `selectable`, `loading`, `type_`, `style_class`, `style`); state (expanded keys, selected keys) is stored in signals, not in the nodes.
- `Tree`: root `div.p-tree.p-component` (`p-tree-selectable`, `p-tree-loading`, `p-tree-flex-scrollable`; `id`, `style`, `style_class`) > loading mask `div.p-tree-mask.p-overlay-mask` with a spinner (`loading_mode = Mask`) > filter field (`filter`) in an `IconField` > `div.p-tree-root` (`max-height` = `scroll_height`) > `ul.p-tree-root-children[role=tree]` > `li.p-tree-node[role=treeitem]` (`aria-label`, `aria-checked`, `aria-selected`, `aria-expanded`, `aria-setsize`, `aria-posinset`, `aria-level`, `tabindex`, `data-id`) > `div.p-tree-node-content` (indent `level * indentation rem`) > `button.p-tree-node-toggle-button` with a chevron, checkbox `p-tree-node-checkbox`, icon `p-tree-node-icon`, label `p-tree-node-label` + `ul.p-tree-node-children[role=group]`; `div.p-tree-empty-message`.
- Selection: `Single` (a repeated click deselects), `Multiple`, `Checkbox` (with upward and downward propagation, partial state `partialSelected`), `meta_key_selection`, `highlight_on_select`, nodes that cannot be selected (`selectable = false`). Selection is an `RwSignal<Vec<String>>` of keys (in `Single` — 0 or 1 key).
- Expansion: chevron button and keyboard arrows; events `on_node_expand`/`on_node_collapse` (for lazy loading: a node with `leaf = false` and no children, `loading` shows a spinner icon with `loading_mode = Icon`).
- Filter: `filter`, `filter_by` (`label`/`key` fields, comma-separated), `filter_mode` (`Lenient`/`Strict`), `filter_placeholder`; matching parents are expanded; `on_filter`.
- Keyboard (roving tabindex): ↓ ↑ over visible nodes, → expands or goes to the first descendant, ← collapses or goes to the parent, `Enter`/`Space` — selection, `Home`/`End`, `Tab`.
- Properties: `value`, `selection_mode`, `selection`, `expanded_keys`, `meta_key_selection`, `propagate_selection_up`/`down`, `highlight_on_select`, `loading`, `loading_mode`, `empty_message`, `aria_label`, `aria_labelledby`, `scroll_height`, `indentation` (1.5), `id`, `style`, `style_class`, `filter*`; events `on_node_select`, `on_node_unselect`, `on_node_expand`, `on_node_collapse`, `on_node_double_click`, `on_filter`; views `node` (label content; by `type_`), `togglericon`.
- The `/tree` page (`basic`, `single`, `multiple`, `checkbox`, `controlled`, `event`, `filter`, `lazy`, `template`), e2e, benchmarks and budgets, `docs/tree.md`, `tree` in `theme.components` (the style source has no `For PrimeNG` block).
- **Out of scope:** node drag and drop (`draggableNodes`, `droppableNodes`, `validateDrop`), virtual scroll and the `lazy` feed, `contextMenu` (the context menu is attached by the user's own `ContextMenu` via `on:contextmenu`), `trackBy`, `filterOptions`/filter template, `Ripple`, `Bind`, `pTemplate`, `header`/`footer`/`loader`/`checkboxicon`/`loadingicon`/`filtericon` templates.

## Capabilities

### New Capabilities
- `tree`: a tree with expansion, selection, checkboxes, a filter and keyboard support.

### Modified Capabilities
<!-- None. -->

## Impact

- New crate `crates/prime-tree` (`publish = false`); dependencies: `prime-rs-core`, `prime-icons` (chevronright, chevrondown, spinner, search, check, minus), `prime-iconfield`, `prime-inputtext`, `prime-checkbox` (classes) if needed.
- Changed: workspace `Cargo.toml`, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml`. The behaviour of existing crates does not change.
