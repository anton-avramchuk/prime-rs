## ADDED Requirements

### Requirement: Accordion structure and classes
`Accordion` of the `prime-accordion` crate SHALL render `<div class="p-accordion p-component">` containing `AccordionPanel`. `AccordionPanel` SHALL render `<div class="p-accordionpanel">` with `p-accordionpanel-active` on the active panel and `p-disabled` on a disabled one, as well as `data-p-active` and `data-p-disabled`. `AccordionHeader` SHALL render `p-accordionheader` with content and the `p-accordionheader-toggle-icon` icon (chevron up on the active panel, down on an inactive one, `aria-hidden="true"`) and `data-p="active"` on the active one. `AccordionContent` SHALL render `p-accordioncontent` with nested `p-motion`, `p-accordioncontent-wrapper` and `p-accordioncontent-content`. The `style_class` of each part SHALL be added without losing its own classes.

#### Scenario: Basic structure
- **WHEN** three panels are rendered, the first is active
- **THEN** the first has `p-accordionpanel-active` and the others do not; the first header contains a chevron up, the others — down

#### Scenario: Disabled panel
- **WHEN** a panel has `disabled`
- **THEN** it has `p-disabled` and `data-p-disabled="true"`

### Requirement: Value and the multiple mode
`Accordion` SHALL accept a two-way `value` (`RwSignal<Vec<String>>`) or `default_value` for its own state. A panel is active if its `value` is in the list. A click on the header of a non-disabled panel SHALL, in single mode, open it and close the others, and a repeated click on the active one — close it; with `multiple` it SHALL add or remove the value without affecting the others.

#### Scenario: Single mode
- **WHEN** panel `0` is open and the header of panel `1` is clicked
- **THEN** `value` becomes `["1"]`, panel `0` closes

#### Scenario: Multiple mode
- **WHEN** `multiple`, panel `0` is open and the header of panel `1` is clicked
- **THEN** `value` contains `0` and `1`

#### Scenario: Closing the active one
- **WHEN** the header of the only active panel is clicked
- **THEN** `value` becomes empty

#### Scenario: Disabled panel
- **WHEN** the header of a disabled panel is clicked
- **THEN** `value` does not change

### Requirement: Accordion events
`Accordion` SHALL call `on_open` with the original event and the panel value when a panel became active after a click, and `on_close` when it became inactive; when one panel opens, the closing of the others in single mode is not accompanied by an `on_close` event.

#### Scenario: Opening
- **WHEN** the user opens an inactive panel
- **THEN** `on_open` is called with this panel's value

#### Scenario: Closing
- **WHEN** the user closes an active panel
- **THEN** `on_close` is called with this panel's value

### Requirement: Accordion accessibility and keyboard
`AccordionHeader` SHALL have `role="button"`, `aria-expanded`, `aria-controls` (the content identifier), `aria-disabled`, `tabindex="0"` (`-1` on a disabled one) and the identifier `{id}_accordionheader_{value}`. `AccordionContent` SHALL have `role="region"`, the identifier `{id}_accordioncontent_{value}` and `aria-labelledby` with the header identifier. Enter, Space and NumpadEnter on the header SHALL toggle the panel; ArrowDown and ArrowUp SHALL move focus to the header of the next and previous non-disabled panel with wrapping around; Home and End — to the first and last. With `select_on_focus`, a non-disabled header receiving focus SHALL toggle the panel.

#### Scenario: Header and content link
- **WHEN** a panel with the value `a` is rendered
- **THEN** the header's `aria-controls` equals the content `id`, and the content's `aria-labelledby` is the header `id`

#### Scenario: Arrows skip disabled ones
- **WHEN** focus is on the first panel's header, the second is disabled, ArrowDown is pressed
- **THEN** focus moves to the third panel's header

#### Scenario: Circular navigation
- **WHEN** focus is on the last panel's header and ArrowDown is pressed
- **THEN** focus moves to the first panel's header

#### Scenario: Keyboard toggling
- **WHEN** focus is on the header and Enter is pressed
- **THEN** the panel toggles once

### Requirement: Accordion animation
The content of an inactive panel SHALL be hidden from the first render. When the state changes in the browser, the content SHALL animate with the classes `p-collapsible-enter-active` and `p-collapsible-leave-active`, and after completion be hidden or shown; focusable descendants of hidden content SHALL get `tabindex="-1"`.

#### Scenario: Initial state
- **WHEN** a panel is inactive at the first render
- **THEN** its content is hidden immediately, without animation

#### Scenario: Expanding
- **WHEN** an inactive panel opens in the browser
- **THEN** the container gets `p-collapsible-enter-active`, then the class is removed and the content is visible

### Requirement: Accordion slots and pass-through
The header's `toggle_icon_template` SHALL receive `active` and replace the standard icon. The accordion's `pt` SHALL accept the parts `root`, `panel`, `header`, `toggle_icon`, `content_container`, `content_wrapper`, `content`, adding classes, style and attributes without replacing its own.

#### Scenario: Custom icon
- **WHEN** `toggle_icon_template` is set
- **THEN** the slot with the current `active` is rendered instead of the chevron

#### Scenario: Class via pt
- **WHEN** the class `custom` is passed in `pt.header`
- **THEN** every header has `custom` and `p-accordionheader`

### Requirement: Accordion rendering cost
The number of SSR allocations for an accordion of three panels SHALL fit within the `accordion-1000` budget in `benches/alloc-budgets.toml`, the wasm contribution — within the `accordion` budget in `benches/budgets.toml`.

#### Scenario: Budgets
- **WHEN** `cargo xtask alloc-budget` and `cargo xtask wasm-size` are run
- **THEN** the Accordion values do not exceed the budgets
