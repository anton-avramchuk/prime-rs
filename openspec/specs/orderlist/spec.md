# orderlist Specification

## Purpose
OrderList — a list with a changeable order of the items.

## Requirements

### Requirement: Reordering
The model SHALL reorder the selected items, keeping their relative order and the order of the others: `move_up` shifts each selected item one position up, if the item above is not selected; `move_down` — down; `move_top` and `move_bottom` move all selected items to the start and to the end. An item already at the edge (or running into a selected block at the edge) SHALL stay in place.

#### Scenario: Up
- **WHEN** the list is `A B C D`, `C` is selected
- **THEN** the result is `A C B D`

#### Scenario: Block at the edge
- **WHEN** the list is `A B C`, `A B` are selected, `move_up`
- **THEN** the order does not change

#### Scenario: Several
- **WHEN** the list is `A B C D E`, `B D` are selected, `move_up`
- **THEN** the result is `B A D C E`

#### Scenario: To the end
- **WHEN** the list is `A B C D`, `A C` are selected, `move_bottom`
- **THEN** the result is `B D A C`

### Requirement: Markup
`OrderList` SHALL render `div.p-orderlist.p-component` (`p-orderlist-controls-left` or `-right` according to `controls_position`) with `div.p-orderlist-controls` (four buttons in the order "up", "to the top", "down", "to the bottom" with `aria-label` "Move Up", "Move Top", "Move Down", "Move Bottom") and the `Listbox` list with multiple selection. The buttons SHALL be unavailable if no items are selected or with `disabled`.

#### Scenario: No selection
- **WHEN** nothing is selected
- **THEN** all four buttons are `disabled`

### Requirement: Actions
A click on a button SHALL change the order of the `value` and call `on_reorder` with the selected items; the selection SHALL be kept; the search `filter` SHALL filter only the display, the reordering goes over the full list.

#### Scenario: Click
- **WHEN** the second item is selected and "up" is clicked
- **THEN** it is first in `value`, the selection is the same, and `on_reorder` is called
