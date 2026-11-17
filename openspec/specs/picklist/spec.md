# picklist Specification

## Purpose
PickList — moving items between two lists with reordering.

## Requirements

### Requirement: Transfer
`move_selected` SHALL move the selected source items to the end of the target in selection order, skipping items whose values are already in the target; `move_all` — all source items visible under the current search, in the original order. Moved items SHALL be removed from the source; symmetrically — from the target to the source.

#### Scenario: Selected
- **WHEN** source `A B C D`, target `X`, `C A` are selected (in that order)
- **THEN** source `B D`, target `X C A`

#### Scenario: Duplicate
- **WHEN** the target already has an item with value `A`, source `A` is selected
- **THEN** the item is not moved and stays in the source

#### Scenario: All under search
- **WHEN** source `Apple Banana Avocado`, search `ap`
- **THEN** `move_all` moves only `Apple`

### Requirement: Markup
`PickList` SHALL render `div.p-picklist.p-component` with five blocks in the order: source controls (`p-picklist-controls p-picklist-source-controls`, with `show_source_controls`), the source list container (`p-picklist-list-container p-picklist-source-list-container`), transfer controls (`p-picklist-controls p-picklist-transfer-controls`), the target list container, target controls (with `show_target_controls`). The source and target buttons are "up", "to top", "down", "to bottom"; the transfer buttons are "to target", "all to target", "to source", "all to source" with `aria-label` from the translation.

#### Scenario: Empty selection
- **WHEN** the selection is empty, both lists are non-empty
- **THEN** the reorder buttons and "to target"/"to source" are disabled, "all to target" and "all to source" are enabled

### Requirement: Actions
The transfer buttons SHALL change `source` and `target` and call the events with the moved items; after the transfer the source selection SHALL be cleared (with `keep_selection` — the target selection is extended with the moved ones). The reorder buttons SHALL change the order of the selected ones in their own list and call `on_source_reorder`/`on_target_reorder`. A double click on an item SHALL move the selected ones. With `disabled` all buttons are disabled.

#### Scenario: To target
- **WHEN** source `B` is selected, click "to target"
- **THEN** `B` is at the end of the target, the source selection is empty, `on_move_to_target` was called

#### Scenario: Double click
- **WHEN** a double click on a source item
- **THEN** the selected items are moved to the target
