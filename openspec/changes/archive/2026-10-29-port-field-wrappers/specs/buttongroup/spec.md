## Purpose
ButtonGroup joins buttons into one connected group with shared rounded corners on the edges.

## ADDED Requirements

### Requirement: ButtonGroup markup
The `ButtonGroup` component of the `prime-buttongroup` crate SHALL render `<span class="p-buttongroup p-component" role="group">` with the child content inside. `style_class` SHALL be added to the root classes after the own ones, `style` — sets the inline style, `pt.root` — adds classes, style and attributes without replacing the own ones. The joined look of the buttons inside the group (a shared border, rounded corners only on the outer ones) SHALL be provided by the CSS of the theme `buttongroup`.

#### Scenario: Root
- **WHEN** three buttons are nested inside the group
- **THEN** one `span.p-buttongroup.p-component[role=group]` is rendered, containing three `button.p-button` in the original order

#### Scenario: Classes and style
- **WHEN** `style_class = "mine"` and `style = "width: 20rem"` are set
- **THEN** the root class is `p-buttongroup p-component mine`, the attribute `style="width: 20rem;"`

#### Scenario: Joined buttons
- **WHEN** a page with the theme CSS connected shows three buttons in a group
- **THEN** the middle button has no rounding, the first has only its left corners rounded, the last only its right corners

#### Scenario: pt
- **WHEN** `pt.root` contains the class `custom` and the attribute `data-x="1"`
- **THEN** the root has `p-buttongroup p-component custom` and `data-x="1"`

### Requirement: Rendering cost of ButtonGroup
The number of allocations per instance during SSR SHALL fit the budget `buttongroup-1000` in `benches/alloc-budgets.toml`, and the wasm contribution — the budget `buttongroup` in `benches/budgets.toml`.

#### Scenario: Budgets
- **WHEN** `cargo xtask alloc-budget` and `cargo xtask wasm-size` are run
- **THEN** the values of ButtonGroup do not exceed the budgets
