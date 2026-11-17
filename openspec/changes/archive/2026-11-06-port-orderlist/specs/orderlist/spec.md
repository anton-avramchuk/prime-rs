# orderlist Specification

## Purpose
OrderList — a list with a changeable order of items.

## ADDED Requirements

### Requirement: Reordering
The model SHALL move the selected items while keeping their mutual order and the order of the rest: `move_up` shifts each selected item one position up, if the item above is not selected; `move_down` — down; `move_top` and `move_bottom` move all selected items to the top and to the bottom. An item that already stands at the edge (or runs into the selected block at the edge) SHALL stay in place.

#### Scenario: Up
- **WHEN** the list is `A B C D`, `C` is selected
- **THEN** the result is `A C B D`

#### Scenario: Block at the edge
- **WHEN** the list is `A B C`, `A B` are selected, `move_up`
- **THEN** the order does not change

#### Scenario: Several items
- **WHEN** the list is `A B C D E`, `B D` are selected, `move_up`
- **THEN** the result is `B A D C E`

#### Scenario: To the bottom
- **WHEN** the list is `A B C D`, `A C` are selected, `move_bottom`
- **THEN** the result is `B D A C`

### Requirement: Markup
`OrderList` SHALL render `div.p-orderlist.p-component` (`p-orderlist-controls-left` or `-right` according to `controls_position`) with `div.p-orderlist-controls` (four buttons in the order "up", "to the top", "down", "to the bottom" with the `aria-label`s "Move Up", "Move Top", "Move Down", "Move Bottom") and a `Listbox` with multiple selection. The buttons SHALL be unavailable if no items are selected or `disabled`.

#### Scenario: No selection
- **WHEN** nothing is selected
- **THEN** all four buttons are `disabled`

### Requirement: Actions
A click on a button SHALL change the order of the `value` and call `on_reorder` with the selected items; the selection SHALL be kept; the `filter` search SHALL filter only the display, the reordering works on the full list.

#### Scenario: Click
- **WHEN** the second item is selected, the "up" click
- **THEN** it is first in `value`, the selection is the same, `on_reorder` is called
