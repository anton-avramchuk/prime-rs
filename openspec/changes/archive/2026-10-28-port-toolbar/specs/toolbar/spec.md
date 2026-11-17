## ADDED Requirements

### Requirement: Structure and classes of Toolbar
The `Toolbar` component of the `prime-toolbar` crate SHALL render `<div class="p-toolbar p-component" role="toolbar">`, inside which the projected content (`children`) comes first, then the areas `div.p-toolbar-start`, `div.p-toolbar-center` and `div.p-toolbar-end` in this order. Each area SHALL be rendered only if the corresponding slot (`start_template`, `center_template`, `end_template`) is set. `style_class` and `style` SHALL be added without losing the own classes.

#### Scenario: Three areas
- **WHEN** all three slots are set
- **THEN** the HTML contains `p-toolbar-start`, `p-toolbar-center`, `p-toolbar-end` in exactly this order, and the root has `role="toolbar"`

#### Scenario: Empty areas are not rendered
- **WHEN** only `end_template` is set
- **THEN** `p-toolbar-end` is present, and `p-toolbar-start` and `p-toolbar-center` are absent

#### Scenario: Projected content
- **WHEN** `children` and the slot `start_template` are passed
- **THEN** the content of `children` comes before `p-toolbar-start`

### Requirement: Accessibility of Toolbar
Toolbar SHALL render `aria-labelledby` with the passed value, and not render the attribute if the value is not set.

#### Scenario: Label
- **WHEN** `aria_labelledby = "title"` is set
- **THEN** the root has `aria-labelledby="title"`

### Requirement: Properties and pass-through of Toolbar
The properties `aria_labelledby`, `style` and `style_class` SHALL accept a value or a signal and update the markup when the signal changes. `pt` SHALL accept the parts `root`, `start`, `center`, `end`, adding classes, style and attributes without replacing the own ones.

#### Scenario: Reactive class
- **WHEN** `style_class` is passed as a signal and changes
- **THEN** the root class is updated, and the own classes are kept

#### Scenario: Class through pt
- **WHEN** the class `custom` is passed in `pt.start`
- **THEN** the start area has both `custom` and `p-toolbar-start`

### Requirement: Rendering cost of Toolbar
A Toolbar with static properties SHALL use the static branch; both branches SHALL produce the same markup. The number of allocations per instance during SSR SHALL fit the budget `toolbar-1000` in `benches/alloc-budgets.toml`, and the wasm contribution — the budget `toolbar` in `benches/budgets.toml`.

#### Scenario: Branch parity
- **WHEN** the same set of values is passed statically and through signals
- **THEN** the HTML matches

#### Scenario: Budgets
- **WHEN** `cargo xtask alloc-budget` and `cargo xtask wasm-size` are run
- **THEN** the values of Toolbar do not exceed the budgets
