# contextmenu Specification

## Purpose
ContextMenu shows a menu with nested submenus at the cursor on a right click (or long press) on a target or on the whole document.

## Requirements

### Requirement: Markup
`ContextMenu` of the `prime-contextmenu` crate SHALL accept `prime_menu::MenuItem` items and, while the menu is shown, output `div.p-contextmenu.p-component` (`p-contextmenu-mobile` while the window is not wider than `breakpoint`; `id`, `style`, `style_class`, `pt`) with the root list `ul.p-contextmenu-root-list[role=menu]` (`tabindex`, `aria-label`, `aria-labelledby`, `aria-activedescendant`). Items, groups, separators, `aria-*` and `data-p-*` SHALL be output as in `TieredMenu`, with the classes `p-contextmenu-item`, `-item-content`, `-item-link`, `-item-icon`, `-item-label`, `-submenu-icon`, `-submenu`, `-separator`. While the menu is not shown, there MUST be no panel in the DOM.

#### Scenario: Hidden by default
- **WHEN** the menu is rendered and has not been shown
- **THEN** `div.p-contextmenu` is absent from the DOM

#### Scenario: Composition
- **WHEN** the menu is shown, with two items, one of them a group
- **THEN** `li.p-contextmenu-item` are output, the group has an arrow and `aria-haspopup="menu"`

### Requirement: Opening
The menu SHALL be shown on the `trigger_event` event (`contextmenu` by default) on `target` or, with `global`, on the document; on touch devices after a touch longer than `press_delay` (500 ms), an earlier release cancels the show. Showing SHALL cancel the default action and event propagation, reset the path and focus, call `on_show` and move focus to the root list. The menu SHALL be positioned at (`pageX + 1`, `pageY + 1`); if it does not fit right/below, it SHALL shift by its width/height left/up; then it SHALL be clamped to the window's left/top edge. A repeated event while the menu is shown SHALL only recompute the position. The trigger listeners SHALL be removed on unmount.

#### Scenario: Right click
- **WHEN** the user presses the right button on `target`
- **THEN** the menu is shown at the cursor, the browser's context menu does not open

#### Scenario: At the window edge
- **WHEN** a right click is made near the bottom-right corner of the window
- **THEN** the menu is entirely within the window

### Requirement: Closing and ref
`ContextMenuRef` SHALL provide `show(event)`, `hide`, `toggle(event)`, `visible`. The menu SHALL close on a click outside the menu (except the right button and a click with Ctrl), on window resize, on selecting a leaf, on `Escape` and `Tab`; closing SHALL reset the path and focus and call `on_hide`. Window/document listeners SHALL be attached for the duration of the show (the click on the next tick) and removed on hide and unmount. The panel SHALL get the `Menu` group layer (`auto_z_index`, `base_z_index`).

#### Scenario: Click outside
- **WHEN** the menu is shown and the user left-clicks outside the menu
- **THEN** the menu is hidden, `on_hide` is called

#### Scenario: Leaf
- **WHEN** the user clicks a leaf
- **THEN** `command` is called, the menu is hidden

### Requirement: Submenus and keyboard
Expanding submenus by mouse (click, hover with `dirty`), the right/left position by available room and the keyboard (↓ ↑ → ← `Home` `End` `Enter` `Space` `Escape` `Tab`, search by characters reset after 500 ms) SHALL work as in `TieredMenu`.

#### Scenario: To the right
- **WHEN** focus is on a group and → is pressed
- **THEN** the submenu is expanded, focus is on its first valid item

### Requirement: Templates
`item` SHALL replace the content of `div.p-contextmenu-item-content`, `submenu_icon` the group arrow.

#### Scenario: Custom item
- **WHEN** `item` is set
- **THEN** inside `div.p-contextmenu-item-content` of each item there is its own view
