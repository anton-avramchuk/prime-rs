## ADDED Requirements

### Requirement: Markup and classes of Skeleton
The `Skeleton` component of the `prime-skeleton` crate SHALL render `<div aria-hidden="true">` with the classes `p-skeleton p-component`, `p-skeleton-circle` (when `shape = circle`), `p-skeleton-animation-none` (when `animation = none`) and `data-p` equal to the shape name (`rectangle` by default, `circle`). `style_class` SHALL be added without losing the own classes.

#### Scenario: Default values
- **WHEN** the properties are not set
- **THEN** the root has the class `p-skeleton p-component` and `data-p="rectangle"`, without `p-skeleton-circle` and `p-skeleton-animation-none`

#### Scenario: Circle without animation
- **WHEN** `shape = circle` and `animation = none` are set
- **THEN** the classes `p-skeleton-circle` and `p-skeleton-animation-none` are present, and `data-p` equals `circle`

### Requirement: Size of Skeleton
The inline style SHALL start with `position: relative;`, then `width` (default `100%`) and `height` (default `1rem`); when `size` is set, the width and height SHALL equal `size`; when `border_radius` is set, `border-radius` SHALL be added.

#### Scenario: Width and height
- **WHEN** `width = "10rem"` and `height = "4rem"` are set
- **THEN** the style equals `position: relative; width: 10rem; height: 4rem;`

#### Scenario: Size
- **WHEN** `size = "3rem"` is set together with `width`
- **THEN** the width and height equal `3rem`, and `width` is ignored

#### Scenario: Border radius
- **WHEN** `border_radius = "16px"` is set
- **THEN** the style ends with `border-radius: 16px;`

### Requirement: Pass-through Skeleton
`pt` SHALL accept `root`, adding classes, style and attributes without replacing the own ones.

#### Scenario: Class through pt
- **WHEN** the class `custom` is passed in `pt.root`
- **THEN** the root has both `custom` and `p-skeleton`

### Requirement: Rendering cost of Skeleton
A Skeleton with static properties SHALL use the static branch; both branches SHALL produce the same markup. The number of allocations per instance during SSR SHALL fit the budget `skeleton` in `benches/alloc-budgets.toml`, and the wasm contribution — the budget `skeleton` in `benches/budgets.toml`.

#### Scenario: Branch parity
- **WHEN** the same set of values is passed statically and through signals
- **THEN** the HTML matches

#### Scenario: Budgets
- **WHEN** `cargo xtask alloc-budget` and `cargo xtask wasm-size` are run
- **THEN** the values of Skeleton do not exceed the budgets
