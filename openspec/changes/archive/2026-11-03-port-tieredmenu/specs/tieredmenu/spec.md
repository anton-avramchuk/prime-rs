# tieredmenu Specification

## Purpose
TieredMenu shows a menu whose groups expand into nested panels next to the item; it is controlled by mouse and keyboard and can be a popup.

## ADDED Requirements

### Requirement: Markup
The `TieredMenu` of the `prime-tieredmenu` crate SHALL accept `prime_menu::MenuItem` items and render `div.p-tieredmenu.p-component` (`p-tieredmenu-overlay` in `popup` mode, `p-tieredmenu-mobile` while the window is not wider than `breakpoint`; `id`, `style`, `style_class`, `pt`) with a root list `ul.p-tieredmenu-root-list[role=menu][aria-orientation=vertical]` (`tabindex` is `tabindex` or `-1` when `disabled`, `aria-label`, `aria-labelledby`, `aria-activedescendant`). Each visible item SHALL be rendered as `li.p-tieredmenu-item[role=menuitem]` with `id` (`<menu id>_<key>`, the key is `0`, `0_1`, ...), `p-tieredmenu-item-active` (expanded), `p-focus`, `p-disabled`, `data-p-highlight`, `data-p-focused`, `data-p-disabled`, `aria-label`, `aria-disabled`, `aria-setsize`, `aria-posinset` (separators are not counted) and, on groups, `aria-haspopup="menu"` and `aria-expanded`; inside it, `div.p-tieredmenu-item-content` > `a.p-tieredmenu-item-link` (`href`, `target`, `title`) with the `p-tieredmenu-item-icon` icon, the `p-tieredmenu-item-label` label and, on groups, the `p-tieredmenu-submenu-icon` arrow, then, while the group is expanded, a nested `ul.p-tieredmenu-submenu[role=menu]` with sub-items. A separator SHALL be `li.p-tieredmenu-separator[role=separator]`; invisible items SHALL NOT be rendered.

#### Scenario: Composition
- **WHEN** a menu of two items is rendered, one of them a group with two sub-items
- **THEN** `li.p-tieredmenu-item` elements are rendered, the group has an arrow and `aria-haspopup="menu"`, and the nested `ul.p-tieredmenu-submenu` appears after it is expanded

#### Scenario: Numbering
- **WHEN** a separator sits between items
- **THEN** `aria-setsize` does not count the separator, and `aria-posinset` of the second item is 2

### Requirement: Expanding by mouse
A click on a group SHALL expand it (the group is added to the path of active items, extra branches are removed); a repeated click on an expanded group or its ancestor SHALL collapse the branch down to that item, leaving focus on it. A click on a leaf SHALL invoke `command` (if the item is not disabled), close all submenus and return focus to the root list; in `popup` mode it SHALL also hide the menu. Hovering an item SHALL expand a group only if something was already expanded by click before and `auto_display` (on by default) is enabled; in a narrow window hover does not change the path. A submenu SHALL be shown only for the expanded item and aligned to the right of it, and to the left when there is not enough room on the right.

#### Scenario: Expanding and collapsing
- **WHEN** the user clicks a group twice
- **THEN** the first click shows the submenu, the second hides it

#### Scenario: Leaf
- **WHEN** the user clicks a leaf of a nested group
- **THEN** `command` is invoked and all submenus are closed

#### Scenario: Hover
- **WHEN** a group is expanded by click and then the cursor is moved onto a sibling group
- **THEN** the sibling group expands

### Requirement: Keyboard
In the root list the keys SHALL work as follows: ↓ is the next valid item of the current level (the first or selected one if there is no focus); ↑ is the previous (the last one when focus is empty); → expands the focused group and moves to its first valid item; ← collapses the submenu and returns to the parent; `Home`/`End` are the first/last valid item; `Enter` and `Space` click the item link; `Escape` closes everything and returns focus; `Tab` activates the focused leaf and closes everything; `Alt+↑` in `popup` mode closes the menu. A valid item SHALL be visible, not disabled and not a separator. A printable character SHALL search for an item of the current level by label prefix, case-insensitively, continuing from the current item and wrapping around; the accumulated search string SHALL be reset after 500 ms. Handled keys SHALL cancel the default action, and `aria-activedescendant` SHALL point to the focused item while the list has focus.

#### Scenario: Down and right
- **WHEN** focus is on a group and → is pressed, then ↓
- **THEN** the group is expanded, focus is on the second valid item of the submenu

#### Scenario: Search
- **WHEN** the user types "e" with the items "File", "Edit", "Users"
- **THEN** focus is on "Edit"

### Requirement: Popup mode
With `popup` the menu SHALL be rendered only while it is shown via `TieredMenuRef` (`show`, `show_at`, `toggle`, `hide`, `visible`): it aligns to the target, receives the `Menu` group layer (`auto_z_index`, `base_z_index`) and focus of the root list, then calls `on_show`. A click outside the menu and target, a window resize (except on touch devices) and scrolling an ancestor of the target SHALL close the menu and reset the path and focus; after hiding, `on_hide` is called. Listeners SHALL be attached for the duration of the display (the click one on the next tick) and removed on hide and unmount. Outside `popup` mode the outside-click listener SHALL work while the path of active items is not empty, and close the submenus.

#### Scenario: Toggle
- **WHEN** `toggle` is called with a button event
- **THEN** the menu is shown at the button, a repeated call hides it

#### Scenario: Outside click
- **WHEN** the menu is shown and the user clicks outside the menu and target
- **THEN** the menu is hidden

### Requirement: Templates
The `item` view SHALL replace the content of `div.p-tieredmenu-item-content` and receive the item and a flag indicating whether it has a submenu; `submenu_icon` SHALL replace the group arrow. A disabled menu (`disabled`) SHALL NOT accept focus (`tabindex="-1"`).

#### Scenario: Custom item
- **WHEN** `item` is set
- **THEN** inside `div.p-tieredmenu-item-content` of each item the custom view is rendered instead of the link
