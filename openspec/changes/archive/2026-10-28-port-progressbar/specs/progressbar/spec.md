## ADDED Requirements

### Requirement: ProgressBar markup and ARIA
The `ProgressBar` component of the `prime-progressbar` crate SHALL render a root `<div role="progressbar">` with `aria-valuemin="0"`, `aria-valuemax="100"`, the classes `p-progressbar p-component p-progressbar-determinate` (default) or `p-progressbar-indeterminate` and `data-p` (`determinate` or `indeterminate`). When `value` is set the root SHALL have `aria-valuenow` with the value and `aria-level` with the value and unit; without a value these attributes SHALL be absent.

#### Scenario: Determinate mode
- **WHEN** `value = 50`
- **THEN** the root has the classes `p-progressbar p-component p-progressbar-determinate`, `aria-valuenow="50"`, `aria-level="50%"`, `data-p="determinate"`

#### Scenario: Without a value
- **WHEN** `value` is not set
- **THEN** the root has no `aria-valuenow` and `aria-level`

### Requirement: ProgressBar determinate mode
In `determinate` mode ProgressBar SHALL render `div.p-progressbar-value` with the style `width: <value>%; display: flex` (and `background: <color>` with `color`) and `value_style_class`, and inside it a `div.p-progressbar-label` with a `div` of the value and unit (`unit`, `%` by default), which is hidden (`display: none`) when there is no value or it is zero. With `show_value = false` the value is not rendered; with `content_template` the template receiving the value is rendered instead of the value.

#### Scenario: Width and label
- **WHEN** `value = 40`
- **THEN** the style of `p-progressbar-value` contains `width: 40%`, and the label contains `40%`

#### Scenario: Zero value
- **WHEN** `value = 0`
- **THEN** the label is present, but its `div` has `display: none`

#### Scenario: Template
- **WHEN** `content_template` is set
- **THEN** the result of the template that received the value is rendered instead of the value

### Requirement: ProgressBar indeterminate mode
In `indeterminate` mode ProgressBar SHALL render only `div.p-progressbar-value` (with `background: <color>` with `color` and `value_style_class`), without width and label.

#### Scenario: Indeterminate
- **WHEN** `mode = indeterminate`
- **THEN** there is `p-progressbar-indeterminate`, `p-progressbar-value` without `width` and no `p-progressbar-label`

### Requirement: Pass-through ProgressBar
`pt` SHALL accept `root`, `value`, `label`, adding classes and attributes without replacing the own ones.

#### Scenario: Class via pt
- **WHEN** the class `custom` is passed in `pt.value`
- **THEN** `p-progressbar-value` has both `custom` and its own class

### Requirement: ProgressBar rendering cost
A ProgressBar with static properties SHALL use the static branch; both branches SHALL produce the same markup. The number of allocations per instance in SSR SHALL fit the `progressbar` budget in `benches/alloc-budgets.toml`, the contribution to wasm - the `progressbar` budget in `benches/budgets.toml`.

#### Scenario: Branch parity
- **WHEN** one set of values is passed statically and as signals
- **THEN** the HTML matches

#### Scenario: Budgets
- **WHEN** `cargo xtask alloc-budget` and `cargo xtask wasm-size` are applied
- **THEN** the ProgressBar values do not exceed the budgets
