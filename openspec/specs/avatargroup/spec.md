# avatargroup Specification

## Purpose
TBD - created by archiving change port-avatargroup. Update Purpose after archiving.

## Requirements

### Requirement: Markup of AvatarGroup
The `AvatarGroup` component of the `prime-avatargroup` crate SHALL render the root `<div class="p-avatar-group p-component">` with the child content without wrappers; `style_class` and `style` SHALL be added without losing the own classes.

#### Scenario: Children
- **WHEN** three elements are passed inside AvatarGroup
- **THEN** they are rendered in the root in the same order

#### Scenario: Class and style
- **WHEN** `style_class = "mine"` and `style = "gap: 1px"` are set
- **THEN** the root has the classes `p-avatar-group p-component mine` and the style `gap: 1px`

### Requirement: Pass-through of AvatarGroup
`pt` SHALL accept `root`, adding classes and attributes without replacing the own ones.

#### Scenario: Class through pt
- **WHEN** the class `custom` is passed in `pt.root`
- **THEN** the root has both `custom` and `p-avatar-group`

### Requirement: Rendering cost of AvatarGroup
An AvatarGroup with static properties SHALL use the static branch; both branches SHALL produce the same markup. The number of allocations per instance during SSR SHALL fit the budget `avatargroup` in `benches/alloc-budgets.toml`, and the wasm contribution — the budget `avatargroup` in `benches/budgets.toml`.

#### Scenario: Branch parity
- **WHEN** the same set of values is passed statically and through signals
- **THEN** the HTML matches

#### Scenario: Budgets
- **WHEN** `cargo xtask alloc-budget` and `cargo xtask wasm-size` are run
- **THEN** the values of AvatarGroup do not exceed the budgets
