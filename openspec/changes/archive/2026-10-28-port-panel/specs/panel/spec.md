## ADDED Requirements

### Requirement: Structure and classes of Panel
The `Panel` component of the `prime-panel` crate SHALL render the root `<div class="p-panel p-component">`, the header `p-panel-header` (if `show_header`, default yes), in it the title `p-panel-title` (if `header` is set) and the block `p-panel-header-actions`, then the container `p-panel-content-container` with the wrapper `p-panel-content-wrapper`, the content `p-panel-content` and the footer `p-panel-footer` (only with `footer_template`). With `toggleable` the root SHALL get `p-panel-toggleable` and `p-panel-expanded` or `p-panel-collapsed`. The actions block SHALL get `p-panel-icons-start`, `p-panel-icons-end` (default) or `p-panel-icons-center` according to `icon_pos`. `style_class` and `style` SHALL be added without losing the own classes.

#### Scenario: Simple panel
- **WHEN** `<Panel header="Header">` with content is rendered
- **THEN** the HTML contains `p-panel-header`, `p-panel-title` with the text `Header`, `p-panel-content`, and `p-panel-footer` and the toggle are absent

#### Scenario: Collapsible panel
- **WHEN** `toggleable` and `collapsed = true` are set
- **THEN** the root has `p-panel-toggleable p-panel-collapsed`, and with `collapsed = false` — `p-panel-expanded`

#### Scenario: Without a header
- **WHEN** `show_header = false`
- **THEN** `p-panel-header` is absent, and the content is rendered

### Requirement: Toggling Panel
With `toggleable` Panel SHALL render the toggle button `p-panel-toggle-button` (text, round, secondary) with a "minus" icon when expanded and a "plus" icon when collapsed. A click on the button (`toggler = icon`, the default) or on the header (`toggler = header`) SHALL invert `collapsed`, writing the new value to an `RwSignal<bool>` (a two-way binding; without a signal the state is internal, initially collapsed when `collapsed = true`). Enter and Space on the button SHALL toggle the panel. Without `toggleable` there is no button, and clicks change nothing.

#### Scenario: Click on the icon
- **WHEN** the user presses the button of an expanded panel
- **THEN** the `collapsed` signal becomes `true`, and the icon changes to "plus"

#### Scenario: Toggler in the header
- **WHEN** `toggler = header` and the user clicks the header
- **THEN** the panel toggles, and a click on the button itself does not cause a double toggle

#### Scenario: Keyboard
- **WHEN** the focus is on the button and Enter or Space is pressed
- **THEN** the panel toggles

### Requirement: Toggle events of Panel
Panel SHALL call `on_before_toggle` with the original event and the current `collapsed` before the state changes, and `on_after_toggle` with the new `collapsed` after the expand or collapse animation ends.

#### Scenario: Before and after
- **WHEN** the panel toggles
- **THEN** `on_before_toggle` is called with the previous value, and `on_after_toggle` — after the animation completes, with the new one

### Requirement: Accessibility of Panel
The toggle button SHALL have `type="button"`, `aria-label` (the `header` text), `aria-controls` (the id of the content container) and `aria-expanded` (`true` when the panel is expanded). The content container SHALL have an `id`, `role="region"`, `aria-labelledby`, `aria-hidden` (`true` when collapsed) and `tabindex="-1"` only when the panel is collapsed. When collapsing, focusable elements inside the content SHALL get `tabindex="-1"`, and when expanding they SHALL lose this attribute. The identifiers SHALL be derived from the `id` of the panel and be unique between instances.

#### Scenario: aria-expanded
- **WHEN** the panel is expanded and then collapsed
- **THEN** the `aria-expanded` of the button changes from `true` to `false`, and the `aria-hidden` of the container from `false` to `true`

#### Scenario: Link between the button and the region
- **WHEN** the panel is rendered
- **THEN** the `aria-controls` of the button equals the `id` of the content container, and the `aria-labelledby` of the container points to the `id` of the header

#### Scenario: Focus of hidden content
- **WHEN** the panel is collapsed and there is a button or a link inside
- **THEN** they have `tabindex="-1"`

### Requirement: Expand animation of Panel
Collapsed content SHALL be hidden. When toggled in the browser, the container SHALL get the class `p-collapsible-enter-active` (expanding) or `p-collapsible-leave-active` (collapsing) for the duration of the animation, and after it ends (`animationend`) SHALL return to the final state. During SSR and the first render there is no animation.

#### Scenario: Expanding
- **WHEN** a collapsed panel is expanded in the browser
- **THEN** the container gets `p-collapsible-enter-active`, then the class is removed and the content is visible

#### Scenario: Initial state
- **WHEN** the panel is rendered with `collapsed = true`
- **THEN** the content is hidden from the start, and there is no animation

### Requirement: Slots and pass-through of Panel
`header_template` SHALL be rendered in the header after the title, `icons_template` — in the actions block before the toggle, `header_icons_template` SHALL replace the toggle icon and receive the current `collapsed`, `footer_template` — in the footer. `pt` SHALL accept the parts `root`, `header`, `title`, `header_actions`, `toggle_button`, `content_container`, `content_wrapper`, `content`, `footer`, adding classes, style and attributes without replacing the own ones.

#### Scenario: Custom toggle icon
- **WHEN** `header_icons_template` is set
- **THEN** it is rendered inside the button instead of "minus"/"plus", and it receives the current `collapsed`

#### Scenario: Class through pt
- **WHEN** the class `custom` is passed in `pt.header`
- **THEN** the header has both `custom` and `p-panel-header`

### Requirement: Rendering cost of Panel
A Panel without `toggleable` and with static properties SHALL use the static branch; both branches SHALL produce the same markup. The number of allocations per instance during SSR SHALL fit the budget `panel` in `benches/alloc-budgets.toml`, and the wasm contribution — the budget `panel` in `benches/budgets.toml`.

#### Scenario: Branch parity
- **WHEN** the same set of values is passed statically and through signals
- **THEN** the HTML matches

#### Scenario: Budgets
- **WHEN** `cargo xtask alloc-budget` and `cargo xtask wasm-size` are run
- **THEN** the values of Panel do not exceed the budgets
