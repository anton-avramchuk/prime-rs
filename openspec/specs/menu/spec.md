# menu Specification

## Purpose
Menu shows a list of commands and links with groups and separators, static or popping up at a target element, and is controlled by mouse and keys.

## Requirements

### Requirement: List markup
`Menu` of the `prime-menu` crate SHALL render `div.p-menu.p-component` with `data-p`, containing `ul.p-menu-list[role=menu]` with `tabindex`, `aria-label`, `aria-labelledby`, `aria-activedescendant` (while focused) and items `li.p-menu-item[role=menuitem]` with `id`, `aria-label`, `aria-disabled` and `data-p-disabled`, in which `div.p-menu-item-content` > `a.p-menu-item-link` (`tabindex=-1`, `href` from `url`, `target`, `title`) with an icon `span.p-menu-item-icon` (if `icon` is set) and `span.p-menu-item-label`. `style_class`, `style` and `pt` SHALL apply to the root; the item's `style_class`/`style` — to `li`.

#### Scenario: Simple model
- **WHEN** a `Menu` with the items "New" and "Search" is rendered
- **THEN** two `li.p-menu-item` are rendered with labels in `.p-menu-item-label`

#### Scenario: Item with a link
- **WHEN** an item has `url` set
- **THEN** `a.p-menu-item-link` has this `href`

### Requirement: Groups and separators
An item with `items` SHALL render `li.p-menu-submenu-label[role=none]` with the label and then its subitems; an item with `separator` SHALL be rendered as `li.p-menu-separator[role=separator]`. Invisible (`visible=false`) items and groups SHALL NOT be rendered.

#### Scenario: Group
- **WHEN** the model contains a group "Documents" with two items
- **THEN** the header "Documents" and two `li.p-menu-item` are rendered

#### Scenario: Invisible item
- **WHEN** an item has `visible=false`
- **THEN** it is absent from the markup

### Requirement: Item command
A click on an item SHALL call its `command` with the event and the item; for an item without `url`, following the link SHALL be cancelled. A click on a disabled item SHALL cancel navigation and not call `command`. An item with `disabled` SHALL have `p-disabled`.

#### Scenario: Command
- **WHEN** the user clicks an item with `command`
- **THEN** `command` is called once with this item

#### Scenario: Disabled item
- **WHEN** the user clicks an item with `disabled`
- **THEN** `command` is not called

### Requirement: Keyboard navigation
The list SHALL accept focus; in static mode, when it receives focus the first available item is focused (`p-focus`, `aria-activedescendant`). Down/Up SHALL move to the next/previous available item (disabled ones are skipped), Home/End — to the first/last, Enter and Space — activate the item (a click on its link). When focus is lost, `p-focus` and `aria-activedescendant` SHALL be removed. `on_focus`/`on_blur` SHALL be called.

#### Scenario: Down arrow
- **WHEN** the list is focused on the first item and Down is pressed
- **THEN** `p-focus` is on the second available item

#### Scenario: Enter
- **WHEN** an item with `command` is focused and Enter is pressed
- **THEN** `command` is called

### Requirement: Popup mode
With `popup` the menu SHALL NOT be rendered until it is shown; `MenuRef::toggle(event)`/`show(event)` SHALL show it at the event's `currentTarget` (`position: absolute`, class `p-menu-overlay`, aligned to the target), focus the list and call `on_show`; `hide()`, a click on an item, a click outside the menu and the target, a window resize (except on touch devices) and scrolling of a nested ancestor of the target SHALL hide it, calling `on_hide`. Escape and Tab SHALL hide the menu and return focus to the target; Alt+Up — the same.

#### Scenario: Show
- **WHEN** `toggle` is called from a button click
- **THEN** `.p-menu.p-menu-overlay` appears next to the button, the list is focused

#### Scenario: Outside click
- **WHEN** the menu is shown and the user clicks outside it and outside the target
- **THEN** the menu is hidden

#### Scenario: Escape
- **WHEN** the menu is shown and Escape is pressed
- **THEN** the menu is hidden, focus is on the target

### Requirement: Layer
With `auto_z_index` the shown popup menu SHALL get the `z-index` of the `Menu` group offset by `base_z_index`; after hiding the index SHALL be released.

#### Scenario: Layer
- **WHEN** the menu is shown, then hidden
- **THEN** `z-index` was set and released

### Requirement: Custom views
`item` SHALL replace the content of the item's `div.p-menu-item-content` and receive the item; `submenu_header` — the content of the group header; `start` and `end` — be rendered in `div.p-menu-start` before the list and `div.p-menu-end` after it.

#### Scenario: Item template
- **WHEN** `item` is set
- **THEN** inside `.p-menu-item-content` there is only the passed view

### Requirement: Rendering without a browser
Outside the browser (SSR) a static `Menu` SHALL be rendered fully, a popup one SHALL NOT be rendered; `window` and `document` SHALL NOT be used.

#### Scenario: SSR
- **WHEN** a page with a static `Menu` is rendered on the server
- **THEN** the markup contains `ul.p-menu-list` with items
