# megamenu Specification

## Purpose
MegaMenu shows a bar of items, each of which expands a wide overlay with submenu columns.

## Requirements

### Requirement: Model
`prime-megamenu` SHALL accept `Vec<MegaMenuItem>`: a root item has the common menu item fields and `columns: Vec<Vec<MegaMenuSubmenu>>` (a column is a list of submenus); `MegaMenuSubmenu` has `label`, `visible`, classes and `items: Vec<prime_menu::MenuItem>`. An item with non-empty columns SHALL be considered a group.

#### Scenario: Group
- **WHEN** a root item has at least one column with a submenu
- **THEN** an arrow and an overlay are rendered for it

### Requirement: Markup
`MegaMenu` SHALL render `div.p-megamenu.p-component` (`p-megamenu-horizontal` or `p-megamenu-vertical`; `p-megamenu-mobile` while the window is no wider than `breakpoint`; `p-megamenu-mobile-active` when the button is open; `id`, `style`, `style_class`), the slots `div.p-megamenu-start` and `div.p-megamenu-end`, the button `a.p-megamenu-button[role=button]` (only with a non-empty model; `aria-haspopup`, `aria-expanded`, `aria-controls`, `aria-label`) and the root list `ul.p-megamenu-root-list[role=menubar]` (`aria-orientation`, `tabindex`, `aria-activedescendant`). A root item SHALL be `li.p-megamenu-item[role=menuitem]` (`data-p-active|focused|disabled`, `aria-level`, `aria-setsize`, `aria-posinset`, `aria-haspopup`, `aria-expanded`, `p-megamenu-item-active`, `p-focus`, `p-disabled`) with `div.p-megamenu-item-content` > `a.p-megamenu-item-link` (icon, label, arrow: down in horizontal orientation and on mobile, right in vertical) and, for a group, the overlay `div.p-megamenu-overlay` > `div.p-megamenu-grid` > columns `div.p-megamenu-col-N`. A submenu SHALL be `ul.p-megamenu-submenu[role=menu]` with a header `li.p-megamenu-submenu-label[role=presentation]` and items; a separator is `li.p-megamenu-separator[role=separator]`. The column class SHALL depend on the number of item columns: 2 → `col-6`, 3 → `col-4`, 4 → `col-3`, 6 → `col-2`, otherwise and on mobile — `col-12`.

#### Scenario: Columns
- **WHEN** an item has three columns
- **THEN** each column gets the class `p-megamenu-col-4`

#### Scenario: Orientation
- **WHEN** `orientation = Vertical`
- **THEN** the root has `p-megamenu-vertical`, the list has `aria-orientation="vertical"`, the group arrow points right

### Requirement: Mouse expansion
A click on a root group SHALL expand its overlay, a repeated click SHALL close it; a click on a leaf SHALL call `command` (if the item is not disabled) and close the overlay. Hover SHALL expand a group after the first click/press on the menu; on a narrow window the path does not change. A click outside the menu SHALL close the overlay (and the mobile panel); `resize` SHALL close it.

#### Scenario: Click
- **WHEN** the user clicks a root item with columns
- **THEN** the item has `p-megamenu-item-active` and `aria-expanded="true"`, the overlay is shown

#### Scenario: Leaf
- **WHEN** the user clicks a leaf inside the overlay
- **THEN** `command` is called, the overlay is closed

### Requirement: Keyboard
Focus SHALL belong to the root list (`aria-activedescendant`, `p-focus`). In horizontal orientation Left/Right SHALL move between root items, Down SHALL expand a group and enter the first item of the overlay; inside the overlay Down/Up SHALL walk the items, Left/Right — between columns, Up on the first item — return to the parent; in vertical orientation the root directions swap (Down/Up across the root, Right expands and enters, Left returns). `Home`/`End`, `Enter`/`Space` (click on the item), `Escape` (close and return focus to the parent), `Tab` and search by label prefix SHALL work as in PrimeNG. Unavailable items SHALL be skipped.

#### Scenario: Entering the overlay
- **WHEN** focus is on an expandable root item and Down is pressed
- **THEN** the overlay is expanded, focus on the first focusable item of the first column

#### Scenario: Escape
- **WHEN** focus is inside the overlay and `Escape` is pressed
- **THEN** the overlay is closed, focus on the root item

### Requirement: Mobile mode
While the window is no wider than `breakpoint`, the root list SHALL be hidden, and the button SHALL show/hide it (`p-megamenu-mobile-active`, `aria-expanded`); the open list gets the `Menu` group layer (`auto_z_index`, `base_z_index`), closing returns focus to the button.

#### Scenario: Button
- **WHEN** on a narrow window the user clicks the button
- **THEN** `p-megamenu-mobile-active` is added and `aria-expanded="true"`, a repeated click closes it

### Requirement: Templates
`start` and `end` SHALL be rendered in their slots; `item` SHALL replace the content of `div.p-megamenu-item-content`; `submenu_icon` — the group arrow; `menu_icon` — the button icon.

#### Scenario: Slots
- **WHEN** `start` and `end` are set
- **THEN** they are rendered before the button and after the list
