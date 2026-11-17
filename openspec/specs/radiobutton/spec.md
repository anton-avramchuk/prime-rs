# radiobutton Specification

## Purpose
TBD - created by archiving change port-radiobutton. Update Purpose after archiving.

## Requirements

### Requirement: Selection and events of RadioButton
The `RadioButton` component of the `prime-radiobutton` crate SHALL accept `value` and a two-way `model` (`RwSignal<Option<String>>`), consider itself checked when `model == Some(value)`, on the user's selection write `Some(value)` to `model` and call `on_click` with the value and the original event; call `on_focus` and `on_blur`. With `disabled` the selection SHALL be ignored.

#### Scenario: Selection in a group
- **WHEN** two buttons share one `model` and the user selects the second one
- **THEN** `model` equals the value of the second, the first stops being checked, and `on_click` is called

#### Scenario: Disabled
- **WHEN** `disabled` is enabled and the user clicks
- **THEN** `model` and `on_click` are not affected

### Requirement: Markup and classes of RadioButton
The root SHALL be a `<div>` with the classes `p-radiobutton p-component` and the modifiers `p-radiobutton-checked`, `p-disabled`, `p-invalid`, `p-variant-filled` (the `filled` variant or `PrimeConfig.input_variant = filled`), `p-radiobutton-sm p-inputfield-sm`, `p-radiobutton-lg p-inputfield-lg`; inside it SHALL come `<input type="radio" class="p-radiobutton-input">` with `aria-checked`, then `<div class="p-radiobutton-box">` with `<div class="p-radiobutton-icon">`. The root SHALL have `data-p-checked`, `data-p-disabled` and `data-p` (separated by spaces: `invalid`, `checked`, `disabled`, `filled`, the size).

#### Scenario: Checked
- **WHEN** `model == Some(value)`
- **THEN** the class `p-radiobutton-checked`, `aria-checked="true"`, `data-p-checked="true"` are present, and `<input>` has the `checked` attribute

#### Scenario: Modifiers
- **WHEN** `disabled`, `invalid`, `size = small`, `variant = filled` are set
- **THEN** the corresponding classes are present, and `data-p` equals `invalid disabled filled small`

### Requirement: Attributes and pass-through of RadioButton
`<input>` SHALL get `id` (`input_id`), `name`, `value`, `tabindex`, `required`, `disabled`, `aria-label`, `aria-labelledby`; unset attributes SHALL be absent. `pt` SHALL accept `root`, `input`, `box`, `icon`, adding classes and attributes without replacing the own ones.

#### Scenario: Attributes
- **WHEN** `input_id`, `name`, `value` are set
- **THEN** `<input>` has `id`, `name`, `value`, and there is no `required` or `tabindex`

### Requirement: Rendering cost of RadioButton
A RadioButton with static properties and without `model` SHALL use the static branch; both branches SHALL produce the same markup. The number of allocations per instance during SSR SHALL fit the budget `radiobutton` in `benches/alloc-budgets.toml`, and the wasm contribution — the budget `radiobutton` in `benches/budgets.toml`.

#### Scenario: Branch parity
- **WHEN** the same set of values is passed statically and through signals
- **THEN** the HTML matches

#### Scenario: Budgets
- **WHEN** `cargo xtask alloc-budget` and `cargo xtask wasm-size` are run
- **THEN** the values of RadioButton do not exceed the budgets
