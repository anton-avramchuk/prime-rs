# progressspinner Specification

## Purpose
TBD - created by archiving change port-progressspinner. Update Purpose after archiving.

## Requirements

### Requirement: Structure and classes of ProgressSpinner
The `ProgressSpinner` component of the `prime-progressspinner` crate SHALL render `<div class="p-progressspinner">` with `role="progressbar"` and `aria-busy="true"`, inside which there is `svg.p-progressspinner-spin` with `viewBox="25 25 50 50"` and a nested `circle.p-progressspinner-circle` (`cx="50"`, `cy="50"`, `r="20"`, `stroke-miterlimit="10"`). `style_class` and `style` SHALL be added to the root without losing the own classes.

#### Scenario: Default markup
- **WHEN** `ProgressSpinner` is rendered without properties
- **THEN** the HTML contains `p-progressspinner`, `role="progressbar"`, `aria-busy="true"`, `p-progressspinner-spin`, `p-progressspinner-circle`, `fill="none"`, `stroke-width="2"` and `animation-duration: 2s`

#### Scenario: Additional class
- **WHEN** `style_class = "mine"` is set
- **THEN** the root has `p-progressspinner` and `mine`

### Requirement: Properties of the circle and the animation
`stroke_width`, `fill` and `animation_duration` SHALL accept a value or a signal and be rendered as `stroke-width` and `fill` of the `circle` and `animation-duration` in the `style` of the `svg`. When the signal changes, the markup SHALL update.

#### Scenario: Custom values
- **WHEN** `stroke_width = "8"`, `fill = "transparent"`, `animation_duration = ".5s"` are set
- **THEN** the circle has `stroke-width="8"` and `fill="transparent"`, and the `svg` has `animation-duration: .5s`

#### Scenario: Reactive value
- **WHEN** `stroke_width` is passed as a signal and changes
- **THEN** the new rendering contains the new value

### Requirement: Accessibility of ProgressSpinner
The `aria_label` property SHALL be rendered as the `aria-label` of the root and not rendered if it is not set.

#### Scenario: Label
- **WHEN** `aria_label = "loading"` is set
- **THEN** the root has `aria-label="loading"`

#### Scenario: Without a label
- **WHEN** `aria_label` is not set
- **THEN** there is no `aria-label` attribute in the HTML

### Requirement: Pass-through of ProgressSpinner
`pt` SHALL accept the parts `root`, `spin`, `circle`, adding classes, style and attributes without replacing the own ones.

#### Scenario: Class through pt
- **WHEN** the class `custom` is passed in `pt.circle`
- **THEN** the circle has both `custom` and `p-progressspinner-circle`

### Requirement: Rendering cost of ProgressSpinner
A ProgressSpinner with static properties SHALL use the static branch; both branches SHALL produce the same markup. The number of allocations per instance during SSR SHALL fit the budget `progressspinner-1000` in `benches/alloc-budgets.toml`, and the wasm contribution — the budget `progressspinner` in `benches/budgets.toml`.

#### Scenario: Branch parity
- **WHEN** the same set of values is passed statically and through signals
- **THEN** the HTML matches

#### Scenario: Budgets
- **WHEN** `cargo xtask alloc-budget` and `cargo xtask wasm-size` are run
- **THEN** the values of ProgressSpinner do not exceed the budgets
