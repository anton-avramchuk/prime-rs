# steps Specification

## Purpose
Steps shows numbered wizard steps in a row, marks the current one and, if needed, allows moving between steps.

## ADDED Requirements

### Requirement: Markup
`Steps` of the `prime-steps` crate SHALL render `nav.p-steps.p-component` (`p-readonly` with `readonly`; `style`, `style_class`, `pt`; `data-pc-name="steps"`) > `ul.p-steps-list` > `li.p-steps-item` for each model item (`p-steps-item-active` on the active one, `p-disabled` on a disabled one, `aria-current="step"` on the active one, `id`, `style`) > `a.p-steps-item-link[role=link]` (`href` from `url`, `target`, `tabindex`, `aria-expanded`, `aria-disabled`) with `span.p-steps-item-number` (the item number, counted from one) and `span.p-steps-item-label` (the label). An item SHALL be considered disabled if it has `disabled` or `readonly` is on and the item is not active. The link SHALL have `tabindex="-1"` on a disabled item, the item's `tabindex` (or `0`) on the active one and the item's `tabindex` (or `-1`) on the rest; `aria-expanded` SHALL be true on the active item, `aria-disabled` — equal to the disabled flag (on an item with `disabled` or on an inactive one with `readonly`).

#### Scenario: Composition
- **WHEN** `Steps` of three items with `active_index = 1` is rendered
- **THEN** three `li` are rendered, the second has `p-steps-item-active` and `aria-current="step"`, numbers 1, 2, 3

#### Scenario: Readonly
- **WHEN** `readonly = true` (the default)
- **THEN** the root has `p-readonly`, inactive items have `p-disabled` and `aria-disabled="true"`

### Requirement: Click
In `readonly` mode or on a disabled item a click SHALL cancel link navigation and call nothing. In interactive mode a click on an available item SHALL call `on_active_index_change(index)`, update the passed `active_index_signal` signal (if set), cancel link navigation if the item has no `url`, and call the item's `command` with the event, the item and the index.

#### Scenario: Interactive click
- **WHEN** `readonly = false` and the user clicks the third item
- **THEN** `on_active_index_change(2)` and `command` with index 2 are called

#### Scenario: Disabled item
- **WHEN** the user clicks an item with `disabled`
- **THEN** nothing is called

### Requirement: Keyboard
On item links → and ← SHALL move focus to the next and previous items, `Home` and `End` — to the first and last; focus SHALL move and `tabindex="0"` SHALL pass to the focused link (the previous one gets `-1`), at the bounds focus stays. `Tab` SHALL return `tabindex="0"` to the active item's link. `Enter` and `Space` SHALL act as a click. Handled keys SHALL cancel the default action.

#### Scenario: Right arrow
- **WHEN** focus is on the first link and → is pressed
- **THEN** focus and `tabindex="0"` are on the second link, the first has `tabindex="-1"`

#### Scenario: End of the list
- **WHEN** focus is on the last item and → is pressed
- **THEN** focus stays on it
