# inputgroup Specification

## Purpose
TBD - created by archiving change port-inputgroup. Update Purpose after archive.

## Requirements

### Requirement: InputGroup structure and classes
The `InputGroup` component of the `prime-inputgroup` crate SHALL render `<div class="p-inputgroup">` with projected content (`children`) without extra wrappers and without `role`. `style_class` and `style` SHALL be added without losing the own class.

#### Scenario: Content
- **WHEN** `InputGroupAddon`, `InputText` and `Button` are passed into `InputGroup`
- **THEN** the HTML contains `div.p-inputgroup`, inside which they stand in the same order

#### Scenario: Custom class
- **WHEN** `style_class = "custom"` is set
- **THEN** the root has both `p-inputgroup` and `custom`

### Requirement: InputGroupAddon structure and classes
The `InputGroupAddon` component SHALL render `<div class="p-inputgroupaddon">` with projected content. `style_class` and `style` SHALL be added without losing the own class.

#### Scenario: Text and icon
- **WHEN** text or an icon is passed into `InputGroupAddon`
- **THEN** they are inside `div.p-inputgroupaddon`

#### Scenario: Inline style
- **WHEN** `style = "width: 3rem"` is set
- **THEN** the root has a `style` attribute with this value

### Requirement: Properties and pass-through
The `style_class` and `style` properties of both components SHALL accept a value or a signal and update the markup when the signal changes. `pt` SHALL accept the `root` part, adding classes, style and attributes without replacing the component's own.

#### Scenario: Reactive class
- **WHEN** `style_class` is passed as a signal and changes
- **THEN** the root class is updated, the own class is kept

#### Scenario: Class via pt
- **WHEN** the class `custom` is passed in `pt.root`
- **THEN** the root has `custom` and the component's own class

### Requirement: InputGroup rendering cost
Components with static properties SHALL use the static branch; both branches SHALL produce identical markup. The number of allocations per instance in SSR SHALL fit the `inputgroup-1000` budget in `benches/alloc-budgets.toml`, and the wasm contribution the `inputgroup` budget in `benches/budgets.toml`.

#### Scenario: Branch parity
- **WHEN** one set of values is passed statically and via signals
- **THEN** the HTML matches

#### Scenario: Budgets
- **WHEN** `cargo xtask alloc-budget` and `cargo xtask wasm-size` are run
- **THEN** the InputGroup values do not exceed the budgets
