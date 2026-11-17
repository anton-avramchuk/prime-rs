# menubar Specification

## Purpose
Menubar shows a horizontal menu bar with dropdown submenus; on a narrow window it collapses into a button.

## ADDED Requirements

### Requirement: Markup
The `Menubar` of the `prime-menubar` crate SHALL accept `prime_menu::MenuItem` and render `div.p-menubar.p-component` (`p-menubar-mobile` while the window is not wider than `breakpoint`; `p-menubar-mobile-active` when the button is open; `id`, `style`, `style_class`) with the slot `div.p-menubar-start` (if `start` is set), the button `a.p-menubar-button[role=button][tabindex=0]` (only with a non-empty model; `aria-haspopup`, `aria-expanded`, `aria-controls`) with the "bars" icon or `menu_icon`, the root list `ul.p-menubar-root-list[role=menubar]` (`tabindex`, `aria-label`, `aria-labelledby`, `aria-activedescendant`) and the slot `div.p-menubar-end` (if `end` is set). Items, groups, separators, `aria-*` and `data-p-*` SHALL be rendered as in `TieredMenu` with the classes `p-menubar-item`, `-item-content`, `-item-link`, `-item-icon`, `-item-label`, `-submenu-icon`, `-submenu`, `-separator`; a submenu — only for an open group.

#### Scenario: Composition
- **WHEN** a menu with a group and the slots `start`/`end` is rendered
- **THEN** `p-menubar-start`, `p-menubar-button`, the root list with items and `p-menubar-end` are rendered

#### Scenario: Empty model
- **WHEN** the model is empty
- **THEN** there is no menu button

### Requirement: Opening with the mouse
A click on a group SHALL open it, a repeated click — collapse the branch; a click on a leaf SHALL call `command` (if the item is not disabled) and close all submenus. Hovering SHALL open a group with `auto_display` (true by default); on a narrow window the path does not change. With `auto_hide` the submenus SHALL close after `auto_hide_delay` (100 ms) after the cursor leaves the menu. A click outside the menu SHALL close the submenus (and the mobile panel); `resize` SHALL close them.

#### Scenario: Opening
- **WHEN** the user clicks a group of the root
- **THEN** the submenu under it is shown, and a repeated click hides it

#### Scenario: Auto-hide
- **WHEN** `auto_hide` is enabled, the submenu is open and the cursor has left the menu
- **THEN** the submenu is closed after `auto_hide_delay`

### Requirement: Keyboard
On the root (horizontal) level → and ← SHALL move to the next/previous allowed item, ↓ — open the group under the focus and move to its first allowed item; `Home`/`End` — to the first/last item of the level. Inside a submenu ↓ ↑ SHALL move between the items, → SHALL open a nested group and enter it, ← — close the submenu and return the focus to the parent. `Enter`/`Space` SHALL click the link of the item, `Escape` — close the submenu and return the focus to the parent, `Tab` — activate the leaf and close the submenu. A printable character SHALL search for an item of the level by the start of its label, case-insensitively and cyclically; the buffer is reset after 500 ms. An allowed item is visible, not disabled, and not a separator. `aria-activedescendant` SHALL point to the item under the focus. The menu button SHALL toggle on `Enter` and `Space`.

#### Scenario: Along the root
- **WHEN** the focus is on the first item of the root and → is pressed
- **THEN** the focus is on the second allowed item of the root

#### Scenario: Down
- **WHEN** the focus is on a group of the root and ↓ is pressed
- **THEN** the group is open, the focus is on the first allowed item of the submenu

### Requirement: Mobile mode
While the window is not wider than `breakpoint`, the root list SHALL be hidden, and the button — show/hide it (`p-menubar-mobile-active`, `aria-expanded`); the open list gets the layer of the `Menu` group (`auto_z_index`, `base_z_index`), closing returns the focus to the button. A click outside the list and the button closes the panel.

#### Scenario: Button
- **WHEN** on a narrow window the user clicks the button
- **THEN** `p-menubar-mobile-active` is added and `aria-expanded="true"`; a repeated click closes it

### Requirement: Templates
`start` and `end` SHALL be rendered in their slots; `item` SHALL replace the content of `div.p-menubar-item-content`; `submenu_icon` — the group arrow; `menu_icon` — the icon of the button.

#### Scenario: Slots
- **WHEN** `start` and `end` are set
- **THEN** they are rendered before the button and after the list
