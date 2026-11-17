# splitbutton Specification

## Purpose
SplitButton combines a button with a default action and an arrow button that opens a popup menu of additional commands.

## Requirements

### Requirement: Markup
`SplitButton` of crate `prime-splitbutton` SHALL render `div.p-splitbutton.p-component` with `data-p-severity` (when `severity` is set), modifiers `p-splitbutton-raised`, `p-splitbutton-rounded`, `p-splitbutton-outlined`, `p-splitbutton-text`, `p-splitbutton-sm`/`p-splitbutton-lg` (by `size`), `style`, `style_class` and `pt`. Inside, in order: the main button `button.p-button.p-splitbutton-button` (label `label`, icon `icon` with `icon_pos`), the dropdown button `button.p-button.p-splitbutton-dropdown.p-button-icon-only` with an arrow icon or `dropdown_icon`, the popup menu. The main and dropdown buttons SHALL inherit `severity`, `text`, `outlined`, `size` and `plain` from `SplitButton`; `raised` and `rounded` SHALL be set by root classes.

#### Scenario: Composition
- **WHEN** a `SplitButton` with `label = "Save"` and `model` is rendered
- **THEN** two buttons are rendered: "Save" with class `p-splitbutton-button` and an arrow with class `p-splitbutton-dropdown`

#### Scenario: Modifiers
- **WHEN** `raised`, `rounded`, `size = small`, `severity = success` are set
- **THEN** the root has classes `p-splitbutton-raised`, `p-splitbutton-rounded`, `p-splitbutton-sm` and `data-p-severity="success"`, and both buttons have class `p-button-success`

### Requirement: Main button
A click on the main button SHALL call `on_click` with the event and close the menu if it is open. `content` SHALL replace the content of the main button (instead of the label and icon). `disabled` and `button_disabled` SHALL make the main button unavailable (`disabled`); `tabindex`, `autofocus` and `button_aria_label` SHALL apply to it.

#### Scenario: Click
- **WHEN** the user clicks the main button while the menu is open
- **THEN** `on_click` is called, the menu is closed

#### Scenario: Unavailability
- **WHEN** `button_disabled` is set
- **THEN** the main button has `disabled`, and the dropdown button stays available

### Requirement: Dropdown button
The dropdown button SHALL have `aria-haspopup="true"`, `aria-expanded` (`true` while the menu is open), `aria-controls` (the menu identifier) and `aria-label` from `expand_aria_label`. A click on it SHALL call `on_dropdown_click` and toggle the menu; pressing ↓ or ↑ on it SHALL toggle the menu and cancel the default action. `disabled` and `menu_button_disabled` SHALL make it unavailable.

#### Scenario: Opening
- **WHEN** the user clicks the dropdown button
- **THEN** the menu is open, `aria-expanded="true"`, `on_dropdown_click` and `on_menu_show` are called

#### Scenario: Key
- **WHEN** focus is on the dropdown button and ↓ is pressed
- **THEN** the menu is open, the default action is cancelled

#### Scenario: Closing
- **WHEN** the menu is open and the user clicks outside the component
- **THEN** the menu is closed, `aria-expanded="false"`, `on_menu_hide` is called

### Requirement: Menu
The menu SHALL be a popup `Menu` with `model`, `menu_style`, `menu_style_class` and be attached to the `SplitButton` root (the panel is aligned under the whole component, not under the arrow). Selecting a menu item SHALL close the menu and run the item `command`, as in `Menu`.

#### Scenario: Position
- **WHEN** the menu is open
- **THEN** the menu panel is located under the `SplitButton` root and is not narrower than it

#### Scenario: Command
- **WHEN** the user selects a menu item with a `command`
- **THEN** the `command` is run, the menu is closed

### Requirement: Rendering without a browser
Outside the browser (SSR) `SplitButton` SHALL render with both buttons and a closed menu (the panel is not rendered); `window` and `document` SHALL NOT be used.

#### Scenario: SSR
- **WHEN** a page with `SplitButton` is rendered on the server
- **THEN** the markup has two buttons, `aria-expanded="false"` and no menu panel
