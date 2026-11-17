# fieldset Specification

## Purpose
TBD - created by archiving change port-fieldset. Update Purpose after archiving.

## Requirements

### Requirement: Structure and classes of Fieldset
The `Fieldset` component of the `prime-fieldset` crate SHALL render `<fieldset class="p-fieldset p-component">` with `<legend class="p-fieldset-legend">` and the container `p-fieldset-content-container` with the wrapper `p-fieldset-content-wrapper` and the content `p-fieldset-content`. Without `toggleable` the legend SHALL contain `span.p-fieldset-legend-label` with the text `legend` and the slot `header_template`. With `toggleable` the root SHALL get `p-fieldset-toggleable`, and in the collapsed state — `p-fieldset-collapsed`; `data-p="toggleable"` SHALL be on the root and the legend. `style_class` and `style` SHALL be added without losing the own classes.

#### Scenario: Simple Fieldset
- **WHEN** `<Fieldset legend="Header">` with content is rendered
- **THEN** the HTML contains `<fieldset`, `<legend`, `p-fieldset-legend-label` with the text `Header`, `p-fieldset-content`, and the toggle button is absent

#### Scenario: Collapsible Fieldset
- **WHEN** `toggleable` and `collapsed = true` are set
- **THEN** the root has `p-fieldset-toggleable p-fieldset-collapsed`, and with `collapsed = false` the class `p-fieldset-collapsed` is absent

### Requirement: Toggle in the Fieldset legend
With `toggleable` the legend SHALL contain `<button class="p-fieldset-toggle-button">` with the icon `p-fieldset-toggle-icon` ("plus" when collapsed, "minus" when expanded; the slots `expand_icon_template` and `collapse_icon_template` replace them) and the legend content (`legend`, `header_template`) after the icon. A click on the button (including Enter and Space) SHALL invert `collapsed`, writing the value to an `RwSignal<bool>` (without a signal — an internal state, initially `default_collapsed`). The button SHALL have `aria-controls`, `aria-expanded` and `aria-label` (the text `legend`).

#### Scenario: Click
- **WHEN** the user presses the button of an expanded Fieldset
- **THEN** the `collapsed` signal becomes `true`, `aria-expanded` — `false`, and the icon changes to "plus"

#### Scenario: Custom icons
- **WHEN** `expand_icon_template` and `collapse_icon_template` are set
- **THEN** the slot matching the state is rendered instead of the standard icons

### Requirement: Events and accessibility of Fieldset
Fieldset SHALL call `on_before_toggle` with the original event and the previous `collapsed` before the state changes, and `on_after_toggle` with the new `collapsed` after the animation ends. The content region SHALL have an `id`, `role="region"`, `aria-labelledby` (the id of the button), `aria-hidden` (with `toggleable`) and `tabindex="-1"` only in the collapsed state; focusable descendants of a collapsed region SHALL get `tabindex="-1"`. Collapsed content SHALL be hidden from the first render, and a change of state in the browser SHALL be animated with the classes `p-collapsible-enter-active`/`p-collapsible-leave-active`.

#### Scenario: Link between the button and the region
- **WHEN** a Fieldset with `toggleable` is rendered
- **THEN** the `aria-controls` of the button equals the `id` of the region, and the `aria-labelledby` of the region equals the `id` of the button

#### Scenario: Before and after
- **WHEN** the Fieldset toggles
- **THEN** `on_before_toggle` is called with the previous value, and `on_after_toggle` — after the animation, with the new one

#### Scenario: Initial state
- **WHEN** the Fieldset is rendered with `collapsed = true`
- **THEN** the region is hidden right away, `aria-hidden="true"`, `tabindex="-1"`

### Requirement: Pass-through of Fieldset
`pt` SHALL accept the parts `root`, `legend`, `legend_label`, `toggle_button`, `toggle_icon`, `content_container`, `content_wrapper`, `content`, adding classes, style and attributes without replacing the own ones.

#### Scenario: Class through pt
- **WHEN** the class `custom` is passed in `pt.legend`
- **THEN** the legend has both `custom` and `p-fieldset-legend`

### Requirement: Rendering cost of Fieldset
A Fieldset without `toggleable` and with static properties SHALL use the static branch; both branches SHALL produce the same markup. The number of allocations per instance during SSR SHALL fit the budget `fieldset-1000` in `benches/alloc-budgets.toml`, and the wasm contribution — the budget `fieldset` in `benches/budgets.toml`.

#### Scenario: Branch parity
- **WHEN** the same set of values is passed statically and through signals
- **THEN** the HTML matches

#### Scenario: Budgets
- **WHEN** `cargo xtask alloc-budget` and `cargo xtask wasm-size` are run
- **THEN** the values of Fieldset do not exceed the budgets
