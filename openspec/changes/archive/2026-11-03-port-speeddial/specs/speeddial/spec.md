# speeddial Specification

## Purpose
SpeedDial shows a floating button that reveals a set of quick actions on press, in a line or along an arc around it.

## ADDED Requirements

### Requirement: Markup
`SpeedDial` of the `prime-speeddial` crate SHALL render `div.p-speeddial.p-component.p-speeddial-<type>` (`type` — `linear`, `circle`, `semi-circle`, `quarter-circle`; for types other than `circle` also `p-speeddial-direction-<direction>`; `p-speeddial-open` when shown; `p-disabled` with `disabled`; `style`, `style_class`, `pt`) with a toggle button `button.p-button.p-button-icon-only.p-speeddial-button.p-button-rounded` (`p-speeddial-rotate` if rotation is enabled and there is no `hide_icon`; `aria-expanded`, `aria-haspopup="true"`, `aria-controls="<id>_list"`, `aria-label`, `aria-labelledby`, `disabled`) with a "plus" icon or `show_icon`/`hide_icon`, and a list `ul.p-speeddial-list[role=menu]` (`id="<id>_list"`, `tabindex="-1"`, `aria-activedescendant` while the list has focus and an item is focused). Each model item SHALL be rendered as `li.p-speeddial-item[role=menuitem]` with `id="<id>_<index>"`, `p-hidden` (with `visible = false`), `p-focus`, `data-p-active` and a `p-speeddial-action` button (secondary, round, small) with an icon, `aria-label` from `label` and `disabled`. The `id` SHALL be unique by default. With `mask` and shown, `div.p-speeddial-mask.p-overlay-mask` SHALL be rendered.

#### Scenario: Composition
- **WHEN** a `SpeedDial` with three items is rendered
- **THEN** a button and a list of three `li.p-speeddial-item` are rendered, the root has `p-speeddial-linear` and `p-speeddial-direction-up`

#### Scenario: Circle
- **WHEN** `type = Circle` is set
- **THEN** there is no `p-speeddial-direction-*` class

### Requirement: Layout
For `linear`, only the `flex-direction` of the root and list SHALL be set by direction (`up` — `column-reverse`, `down` — `column`, `left` — `row-reverse`, `right` — `row`). For `circle`, each item SHALL get `left` and `top` of the form `calc(<x>px + var(--item-diff-x, 0px))` with angle `2π·i/n`; for `semi-circle` — step `π/(n-1)` and a pair of sides by direction (`up`: `left`/`bottom`, `down`: `left`/`top`, `left`: `right`/`top`, `right`: `left`/`top`); for `quarter-circle` — step `π/(2(n-1))` and a pair of sides by diagonal direction (`up-left`: `right`/`bottom`, `up-right`: `left`/`bottom`, `down-left`: `right`/`top`, `down-right`: `left`/`top`). The radius SHALL be `radius`, or `20 × n` when it is `0`. Each item SHALL get a `transition-delay`: `i · transition_delay` ms when showing, `(n − i − 1) · transition_delay` when hiding. After showing in the browser, for types other than `linear` the list's `--item-diff-x`/`--item-diff-y` SHALL be set — half the difference between the width/height of the button and the first item.

#### Scenario: Delays
- **WHEN** the list is shown, `transition_delay = 30`, three items
- **THEN** the delays are 0, 30 and 60 ms

#### Scenario: Semicircle up
- **WHEN** `type = SemiCircle`, `direction = Up`, three items, radius 80
- **THEN** for the first item `left` is computed from `80·cos 0`, `bottom` — from `80·sin 0`

### Requirement: Showing, commands and closing
Clicking the button SHALL toggle display and call `on_click`, `on_visible_change(bool)` and `on_show`/`on_hide`; the `visible` property SHALL be two-way. Clicking an item SHALL call `command` (if the item is not disabled), hide the list and not hide it again via the outside click. With `hide_on_click_outside` (yes by default) a click outside the component while shown SHALL hide the list; the listener SHALL be attached for the time it is shown (on the next tick) and removed on hide and unmount. `Escape` on the button and in the list SHALL hide the list and return focus to the button. A disabled component SHALL not react to a button click.

#### Scenario: Item command
- **WHEN** the list is shown and an item with `command` is clicked
- **THEN** `command` is called, the list is hidden

#### Scenario: Outside click
- **WHEN** the list is shown and the user clicks outside the component
- **THEN** the list is hidden

### Requirement: Keyboard
On the button, `ArrowDown` and `ArrowLeft` SHALL open the list and move focus to the next item, `ArrowUp` and `ArrowRight` — to the previous (the last one when focus is empty), with `preventDefault`. In the list the arrows SHALL move through items according to the direction: for `up` ↑ is next, ↓ is previous; for `down` the opposite; for `left`, `up-right`, `down-left` ← is next, → is previous; for `right`, `up-left`, `down-right` the opposite; for the other values ↑ and → are next, ↓ and ← are previous. `Home` SHALL go to the first item, `End` — to the last; `Enter` and `Space` SHALL activate the focused item and return focus to the button. Disabled and hidden items SHALL be skipped; at the boundaries focus SHALL stay.

#### Scenario: Opening with an arrow
- **WHEN** `ArrowDown` is pressed on the button
- **THEN** the list is shown, the first available item is focused (`p-focus`, `aria-activedescendant`)

#### Scenario: Direction down
- **WHEN** `direction = Down` and `ArrowDown` is pressed in the list
- **THEN** focus moves to the next item

### Requirement: Tooltips and templates
With `tooltip_options` each item SHALL get a tooltip with the `label` text and the template's parameters. The `button_content` view SHALL replace the button and receive the toggle callback; `item_content` SHALL replace the button inside `li` and receive the item, index and click callback; `icon_content` SHALL replace the button icon.

#### Scenario: Custom item
- **WHEN** `item_content` is set
- **THEN** inside each `li.p-speeddial-item` the custom view is rendered instead of the action button
