# panelmenu Specification

## Purpose
PanelMenu shows a vertical list of header panels with expandable nested branches (a hybrid of Accordion and Tree).

## ADDED Requirements

### Requirement: Markup
The `PanelMenu` of the `prime-panelmenu` crate SHALL accept `prime_menu::MenuItem` and render `div.p-panelmenu.p-component` (`id`, `style`, `style_class`). Each visible root item (`visible != false`) SHALL be rendered as a `div.p-panelmenu-panel` with a `div.p-panelmenu-header` header (`p-panelmenu-header-active` on an expanded group, `p-disabled` on a disabled one; `aria-expanded`, `aria-controls`, `aria-disabled`, `data-p-highlight`) > `div.p-panelmenu-header-content` > `a.p-panelmenu-header-link` (the `p-panelmenu-submenu-icon` chevron on a group — right when collapsed, down when expanded; `p-panelmenu-header-icon`; `p-panelmenu-header-label`) and with a `div.p-panelmenu-content-container` area (`p-panelmenu-expanded` when expanded) > `div.p-panelmenu-content-wrapper` > `div.p-panelmenu-content` > `ul.p-panelmenu-root-list[role=tree]` (`tabindex`, `aria-activedescendant`, `aria-hidden`). Nested items SHALL be rendered as `li.p-panelmenu-item[role=treeitem]` (`aria-label`, `aria-level`, `aria-setsize`, `aria-posinset`, `aria-expanded` on groups, `data-p-focused`, `data-p-disabled`, `p-focus`, `p-disabled`) > `div.p-panelmenu-item-content` > `a.p-panelmenu-item-link` (`p-panelmenu-item-icon`, `p-panelmenu-item-label`, group chevron) and a nested `ul.p-panelmenu-submenu`; separators — `li.p-menuitem-separator[role=separator]`.

#### Scenario: Composition
- **WHEN** a menu with a two-level group is rendered
- **THEN** panels, headers, content areas and trees with `role=tree`/`treeitem` and `aria-level` by depth are rendered

#### Scenario: Hidden items
- **WHEN** an item has `visible = Some(false)`
- **THEN** it is absent from the markup

### Requirement: Expansion
A click on a group header or on a group item SHALL toggle its expansion. The initial state SHALL come from the items' `expanded`. With `multiple = false` (default) expanding a root panel SHALL collapse the previously expanded root panel; with `multiple = true` panels are independent. Nested branches SHALL be independent: expanding one does not collapse siblings. Expanding and collapsing SHALL be animated via `prime_core::collapsible`.

#### Scenario: Single mode
- **WHEN** panel A is expanded, the user clicks the header of B
- **THEN** B is expanded, A is collapsed

#### Scenario: Multiple mode
- **WHEN** `multiple = true` and A is expanded, a click on B
- **THEN** both are expanded

#### Scenario: Initial state
- **WHEN** an item has `expanded = Some(true)`
- **THEN** it is rendered expanded without animation

### Requirement: Commands and disabled items
A click on a leaf SHALL invoke the item's `command` if it is not disabled; a click on a disabled item or header SHALL do nothing. Disabled items SHALL be skipped during navigation.

#### Scenario: Command
- **WHEN** the user clicks a leaf with a `command`
- **THEN** `command` is invoked once with the item

#### Scenario: Disabled
- **WHEN** the user clicks a disabled header
- **THEN** the expansion does not change

### Requirement: Controlled expansion
`PanelMenuRef` SHALL allow expanding all groups, collapsing all and toggling all (`expand_all`, `collapse_all`, `toggle_all`); the state SHALL be stored by item keys (`key` or position) so that the `controlled` demo is reproducible.

#### Scenario: Expand all
- **WHEN** `expand_all` is called
- **THEN** all groups are expanded and `aria-expanded="true"`

### Requirement: Keyboard
A panel header SHALL be focusable (`tabindex="0"`): ↓ SHALL move into the tree of an expanded panel, otherwise to the next header; ↑ — to the previous one (into its tree if it is expanded); `Home`/`End` — to the first/last header; `Enter`/`Space` — toggle the panel. Disabled headers are skipped, movement wraps around. In the panel tree (highlight — `aria-activedescendant` and `p-focus`) ↓/↑ SHALL move to the next/previous visible allowed item, and beyond the edges — to the adjacent header / the header of its own panel; → SHALL expand a branch, and on an expanded one — move to the first descendant; ← SHALL collapse an expanded branch, otherwise move to the parent; `Home`/`End` — to the first/last item; `Enter`/`Space` — perform a click on the item; printable characters — search by label prefix. `Tab` SHALL move focus out of the component.

#### Scenario: Arrows
- **WHEN** focus is on an expanded group item and → is pressed, then ↓
- **THEN** the first descendant is active, then the second

#### Scenario: Collapsing
- **WHEN** focus is on an expanded group and ← is pressed
- **THEN** the group is collapsed

#### Scenario: Enter
- **WHEN** focus is on a leaf with a `command` and `Enter` is pressed
- **THEN** `command` is invoked

### Requirement: Templates
`item` SHALL replace the content of `div.p-panelmenu-item-content`; `submenu_icon` — the group chevron. The root panel header SHALL stay standard.

#### Scenario: Custom item
- **WHEN** `item` is set
- **THEN** its content is rendered in `p-panelmenu-item-content` instead of the link
