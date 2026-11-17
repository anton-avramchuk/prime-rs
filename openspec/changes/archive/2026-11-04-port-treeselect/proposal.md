# Proposal

## Why

TreeSelect is a field for selecting a tree node (or nodes) in a dropdown panel. The panel contains a ready-made `Tree` (whose port was just added), and the shell is like `Select`/`MultiSelect`: a label with commas or chips, a clear icon, a dropdown button, a panel inside the root with `align_overlay` alignment.

## What Changes

- New crate `prime-treeselect` (port of `p-treeselect` from PrimeNG 21.1.10) on `prime-tree`, `prime-chip`, `prime-dom`.
- `TreeSelect`: root `div.p-treeselect.p-component.p-inputwrapper` (`p-treeselect-display-chip`, `p-disabled`, `p-invalid`, `p-focus`, `p-variant-filled`, `p-inputwrapper-filled|focus`, `p-treeselect-open`, `p-treeselect-clearable`, `p-treeselect-fluid`, `-sm`, `-lg`) > hidden field `div.p-hidden-accessible > input[role=combobox][readonly]` (`aria-haspopup="tree"`, `aria-expanded`, `aria-controls`) > `div.p-treeselect-label-container` > `div.p-treeselect-label` (comma: captions of selected nodes joined by `, ` or `placeholder`; chips: `div.p-treeselect-chip-item` > `Chip.p-treeselect-chip`) > clear icon `p-treeselect-clear-icon` > `div.p-treeselect-dropdown[role=button]` > panel `div.p-treeselect-overlay.p-component-overlay.p-component` > `div.p-treeselect-tree-container` (`max-height` = `scroll_height`, `400px` by default) > `Tree`.
- The value is `RwSignal<Vec<String>>` of node keys (in `Single` — 0 or 1 key). Modes `Single` (default), `Multiple`, `Checkbox`; `meta_key_selection`, `propagate_selection_up/down`; filter, empty message, loading and expansion are passed to `Tree`.
- Behavior: a click on the field opens/closes the panel; `Single` closes the panel after selection and returns focus; on open the ancestors of selected nodes are expanded; an outside click, `resize` and `Escape` close; clear resets the value; the panel position is recomputed after a node is expanded.
- Keyboard on the hidden field: Down opens and moves focus to the first tree node, `Space`/`Enter` open, `Escape` closes and returns focus, `Tab` closes.
- Properties: `options`, `value`, `selection_mode`, `display`, `placeholder`, `scroll_height`, `meta_key_selection`, `propagate_selection_up/down`, `show_clear`, `disabled`, `invalid`, `variant`, `size`, `fluid`, `filter`, `filter_by`, `filter_mode`, `filter_placeholder`, `empty_message`, `loading`, `input_id`, `tabindex`, `aria_label`, `aria_labelledby`, `id`, `style`, `style_class`, `panel_style_class`; events `on_node_select`, `on_node_unselect`, `on_node_expand`, `on_node_collapse`, `on_clear`, `on_show`, `on_hide`, `on_focus`, `on_blur`; views `value` (label), `node` (node label).
- Page `/treeselect` (`basic`, `multiple`, `checkbox`, `filter`, `clearicon`, `disabled`, `invalid`, `lazy`, `template`), e2e, benchmarks and budgets, `docs/treeselect.md`, `treeselect` in `theme.components` and `primeng_extra`.
- **Out of scope:** virtual scrolling, `appendTo` (the panel is inside the root), `Ripple`, `Bind`, `pTemplate`, `header`/`footer`/`empty`/`triggericon`/`itemcheckboxicon`/`itemloadingicon`/`filtericon`/`closeicon` templates, Angular forms (`ngModel`, reactive forms), hidden panel focus traps, `resetFilterOnHide` (the filter is reset on every close).

## Capabilities

### New Capabilities
- `treeselect`: selecting tree nodes in a dropdown panel.

### Modified Capabilities
<!-- None. -->

## Impact

- New crate `crates/prime-treeselect` (`publish = false`); dependencies: `prime-core`, `prime-dom`, `prime-tree`, `prime-chip`, `prime-icons` (chevrondown, times).
- Changed: workspace `Cargo.toml`, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml`. The behavior of existing crates does not change.
