# checkbox Specification

## Purpose
TBD - created by archiving change port-checkbox. Update Purpose after archiving.

## Requirements

### Requirement: State and events of Checkbox
The `Checkbox` component of the `prime-checkbox` crate SHALL support the `binary` mode (value `RwSignal<bool>`) and the set mode (`value` and `RwSignal<Vec<String>>`), toggle the state when the native `<input type="checkbox">` changes, and call `on_change` (with the new `checked` and the original event), `on_focus`, `on_blur`; with `readonly` and `disabled` the state SHALL not change.

#### Scenario: Binary
- **WHEN** the user clicks a Checkbox in `binary` mode
- **THEN** the signal changes its value, and `on_change` is called with the new `checked`

#### Scenario: Set of values
- **WHEN** in set mode a Checkbox with `value = "a"` is clicked
- **THEN** `a` is added to the signal, and a repeated click removes it

#### Scenario: Readonly
- **WHEN** `readonly` is enabled and the user clicks
- **THEN** the signal and `on_change` are not affected

### Requirement: Markup and classes of Checkbox
The root SHALL be a `<div>` with the classes `p-checkbox p-component` and the modifiers `p-checkbox-checked p-highlight` (checked), `p-disabled`, `p-invalid`, `p-variant-filled`, `p-checkbox-sm p-inputfield-sm`, `p-checkbox-lg p-inputfield-lg`; inside it SHALL come `<input type="checkbox" class="p-checkbox-input">`, then `<div class="p-checkbox-box">` with the `check` icon (checked) or `minus` (`indeterminate`) and the class `p-checkbox-icon`. The root SHALL have `data-p-checked`, `data-p-highlight`, `data-p-disabled` and `data-p` (separated by spaces: `invalid`, `checked`, `disabled`, `filled` for the `filled` variant, the size).

#### Scenario: Checked
- **WHEN** `checked = true`
- **THEN** the classes `p-checkbox-checked p-highlight` are present, `<input>` has the `checked` attribute, and the box has the `check` icon

#### Scenario: Indeterminate
- **WHEN** `indeterminate` is enabled
- **THEN** the state is not counted as checked, and the box has the `minus` icon

#### Scenario: Variant from the configuration
- **WHEN** `variant` is not set and `PrimeConfig.input_variant = filled`
- **THEN** the class `p-variant-filled` is present

### Requirement: Attributes and pass-through of Checkbox
`<input>` SHALL get `id` (`input_id`), `name`, `value`, `tabindex`, `required`, `readonly`, `disabled`, `aria-label`, `aria-labelledby`, `input_class`, `input_style`; unset attributes SHALL be absent. `pt` SHALL accept `root`, `input`, `box`, `icon`, adding classes and attributes without replacing the own ones.

#### Scenario: Attributes
- **WHEN** `input_id`, `name`, `disabled` are set
- **THEN** `<input>` has `id`, `name`, `disabled`, and there is no `readonly` or `required`

### Requirement: Rendering cost of Checkbox
A Checkbox with static properties and without value signals SHALL use the static branch; both branches SHALL produce the same markup. The number of allocations per instance during SSR SHALL fit the budget `checkbox` in `benches/alloc-budgets.toml`, and the wasm contribution — the budget `checkbox` in `benches/budgets.toml`.

#### Scenario: Branch parity
- **WHEN** the same set of values is passed statically and through signals
- **THEN** the HTML matches

#### Scenario: Budgets
- **WHEN** `cargo xtask alloc-budget` and `cargo xtask wasm-size` are run
- **THEN** the values of Checkbox do not exceed the budgets
