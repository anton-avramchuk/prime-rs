# dock Specification

## Purpose
Dock shows command icons in a panel at the edge of the container and is controlled by the mouse and the keyboard.

## ADDED Requirements

### Requirement: Markup
The `Dock` of the `prime-dock` crate SHALL render `div.p-dock.p-component.p-dock-<position>` (`style_class`, `style`, `pt`; `p-dock-mobile` while the window is not wider than `breakpoint`) > `div.p-dock-list-container` > `ul.p-dock-list[role=menu]` with `id`, `aria-orientation` (`horizontal` for `Bottom`/`Top`, `vertical` for `Left`/`Right`), `tabindex="0"`, `aria-label`, `aria-labelledby` and `aria-activedescendant` (while an item has focus). Each model item SHALL be rendered as `li.p-dock-item[role=menuitem]` with `id`, `aria-label` from `label`, `aria-disabled`, `data-p-disabled`, `data-p-focused`, the classes `p-focus` (the item with focus), `p-disabled` and the item class, and inside it — `div.p-dock-item-content` > `a.p-dock-item-link[aria-hidden=true]` with `href` (from `url`), `target`, `title` and the link class.

#### Scenario: Composition
- **WHEN** `Dock` with three items is rendered
- **THEN** three `li.p-dock-item` with the links `a.p-dock-item-link` are rendered, the root has the class `p-dock-bottom`

#### Scenario: Position
- **WHEN** `position = Left` is set
- **THEN** the root has the class `p-dock-left`, the list has `aria-orientation="vertical"`

### Requirement: Icon, badge and view
If an item has an `icon` and there is no `item` view, a `span.p-dock-item-icon` with the icon SHALL be rendered; the `item` view SHALL replace the icon and receive the item. With `badge` a `Badge` with the value SHALL be rendered. An item with `disabled` SHALL not call `command` on click.

#### Scenario: Icon
- **WHEN** the item has an `icon` set
- **THEN** inside the link there is `.p-dock-item-icon` with the icon

#### Scenario: Template
- **WHEN** `item` is set
- **THEN** inside the link, instead of the icon, the passed view is rendered

### Requirement: Click
A click on an item SHALL call its `command` with the original event and the item, if the item is not disabled.

#### Scenario: Command
- **WHEN** the user clicks an item with `command`
- **THEN** `command` is called with this item

#### Scenario: Disabled item
- **WHEN** the user clicks an item with `disabled`
- **THEN** `command` is not called

### Requirement: Keyboard and focus
Receiving the focus by the list SHALL call `on_focus` and put the focus on the first available item; losing the focus — `on_blur` and a reset. For `Bottom`/`Top` the → key SHALL move the focus to the next available item, ← — to the previous one; for `Left`/`Right` ↓ and ↑ do the same (the perpendicular arrows are cancelled without any action). `Home` SHALL move the focus to the first, `End` — to the last available item, `Enter` and `Space` — click the link of the focused item. Disabled items SHALL be skipped; at the edges the focus SHALL stay on the extreme item.

#### Scenario: Navigation
- **WHEN** the list has focus, the position is `Bottom`, and → is pressed
- **THEN** the focus is on the second available item, and `aria-activedescendant` points to it

#### Scenario: Disabled item
- **WHEN** the second item is disabled and → is pressed from the first
- **THEN** the focus is on the third

#### Scenario: Enter
- **WHEN** the focus is on an item with `command` and Enter is pressed
- **THEN** `command` is called

### Requirement: Mobile mode
If the window is not wider than `breakpoint` (by default `960px`), the root SHALL get the class `p-dock-mobile`; the class SHALL change when the window width changes. Outside the browser (SSR) there is no class.

#### Scenario: Narrow window
- **WHEN** the window width is less than `breakpoint`
- **THEN** the root has `p-dock-mobile`

### Requirement: Rendering without a browser
Outside the browser (SSR) `Dock` SHALL render fully; `window` and `document` SHALL not be used.

#### Scenario: SSR
- **WHEN** a page with `Dock` is rendered on the server
- **THEN** the markup contains the list of items
