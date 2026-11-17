# toggleswitch Specification

## Purpose
TBD - created by archiving change port-toggleswitch. Update Purpose after archiving.

## Requirements

### Requirement: State and events of ToggleSwitch
The `ToggleSwitch` component of the `prime-toggleswitch` crate SHALL accept a two-way `checked` (`RwSignal<bool>`), toggle it on a click on any part of the root (including the native `<input>`), call `on_change` with the new value and the original event exactly once per click, return the focus to `<input>`, and call `on_focus` and `on_blur`. With `disabled` and `readonly` the state SHALL not change.

#### Scenario: Click
- **WHEN** the user clicks the ToggleSwitch
- **THEN** the signal changes its value, and `on_change` is called once with the new value

#### Scenario: Readonly and disabled
- **WHEN** `readonly` or `disabled` is enabled and the user clicks
- **THEN** the signal, the native state of `<input>` and `on_change` are not affected

### Requirement: Markup and classes of ToggleSwitch
The root SHALL be a `<div>` with the classes `p-toggleswitch p-component` and the modifiers `p-toggleswitch-checked`, `p-disabled`, `p-invalid`, with `style="position: relative"` (`style` and `pt.root` are attached to it); inside it SHALL come `<input type="checkbox" role="switch" class="p-toggleswitch-input">` with `aria-checked`, then `<div class="p-toggleswitch-slider">` with `<div class="p-toggleswitch-handle">`. The root SHALL have `data-p-checked`, `data-p-disabled`, and the root, the slider and the handle — `data-p` (separated by spaces: `checked`, `disabled`, `invalid`).

#### Scenario: Enabled
- **WHEN** `checked = true`
- **THEN** the class `p-toggleswitch-checked`, `aria-checked="true"`, `data-p-checked="true"`, `data-p="checked"` are present, and `<input>` has the `checked` attribute

#### Scenario: Handle slot
- **WHEN** `handle_template` is set
- **THEN** its content is inside `p-toggleswitch-handle`, and the template receives the current `checked`

### Requirement: Attributes and pass-through of ToggleSwitch
`<input>` SHALL get `id` (`input_id`), `name`, `tabindex`, `required`, `disabled`, `aria-label`, `aria-labelledby`; unset attributes SHALL be absent. `pt` SHALL accept `root`, `input`, `slider`, `handle`, adding classes and attributes without replacing the own ones.

#### Scenario: Attributes
- **WHEN** `input_id`, `name` are set
- **THEN** `<input>` has `id`, `name`, and there is no `required` or `tabindex`

### Requirement: Rendering cost of ToggleSwitch
A ToggleSwitch with static properties and without `checked` SHALL use the static branch; both branches SHALL produce the same markup. The number of allocations per instance during SSR SHALL fit the budget `toggleswitch` in `benches/alloc-budgets.toml`, and the wasm contribution — the budget `toggleswitch` in `benches/budgets.toml`.

#### Scenario: Branch parity
- **WHEN** the same set of values is passed statically and through signals
- **THEN** the HTML matches

#### Scenario: Budgets
- **WHEN** `cargo xtask alloc-budget` and `cargo xtask wasm-size` are run
- **THEN** the values of ToggleSwitch do not exceed the budgets
