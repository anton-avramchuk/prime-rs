# fluid Specification

## Purpose
Fluid stretches nested fields and buttons to the full width of the container without requiring `fluid` to be set on each of them.

## Requirements

### Requirement: Markup of Fluid
The `Fluid` component of the `prime-fluid` crate SHALL render `<div class="p-fluid">` with the child content. `style_class` SHALL be added to the root classes after the own ones, `style` — sets the inline style, `pt.root` — adds classes, style and attributes without replacing the own ones.

#### Scenario: Root
- **WHEN** two fields are nested inside `Fluid`
- **THEN** one `div.p-fluid` containing both fields is rendered

#### Scenario: Classes and style
- **WHEN** `style_class = "card"` and `style = "gap: 1rem"` are set
- **THEN** the root class is `p-fluid card`, the attribute `style="gap: 1rem;"`

### Requirement: Inheritance of fluid
`Fluid` SHALL provide descendants with a context by which `Button`, `InputText`, `Textarea`, `InputNumber` and `Select` determine their width: if the component's `fluid` property is not set, it SHALL behave as with `fluid = true` when it has a `Fluid` ancestor (at any depth), and as with `fluid = false` without such an ancestor. An explicitly set `fluid` (including `false`) SHALL take precedence over the context. Outside `Fluid` the behaviour of the components SHALL remain as before.

#### Scenario: Inheritance
- **WHEN** an `InputText` without `fluid` is inside `Fluid`
- **THEN** the field has the class `p-inputtext-fluid`, and its `data-p` contains `fluid`

#### Scenario: Priority of an explicit value
- **WHEN** an `InputText` with `fluid = false` is inside `Fluid`
- **THEN** it does not have the class `p-inputtext-fluid`

#### Scenario: Nesting
- **WHEN** a `Select` is inside a `div` inside `Fluid` inside another container
- **THEN** the `Select` has the class `p-select-fluid`

#### Scenario: Outside Fluid
- **WHEN** a `Button`, a `Textarea` and an `InputNumber` are placed without `Fluid` and without `fluid`
- **THEN** none of them has a stretch class

#### Scenario: All consumers
- **WHEN** `Button`, `InputText`, `Textarea`, `InputNumber`, `Select` are nested inside `Fluid`
- **THEN** each has its own stretch class (`p-button-fluid`, `p-inputtext-fluid`, `p-textarea-fluid`, `p-inputnumber-fluid`, `p-select-fluid`)

#### Scenario: Reactive fluid
- **WHEN** the `fluid` of a descendant inside `Fluid` is passed as a signal and takes the value `false`
- **THEN** the stretch class is removed

### Requirement: Rendering cost of Fluid
The number of allocations per instance during SSR SHALL fit the budget `fluid-1000` in `benches/alloc-budgets.toml`, and the wasm contribution — the budget `fluid` in `benches/budgets.toml`.

#### Scenario: Budgets
- **WHEN** `cargo xtask alloc-budget` and `cargo xtask wasm-size` are run
- **THEN** the values of Fluid do not exceed the budgets
