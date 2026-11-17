## Purpose
OverlayBadge overlays a badge on the top-right corner of arbitrary content (an icon, an avatar, a button), without changing the layout of the content.

## ADDED Requirements

### Requirement: Markup of OverlayBadge
The `OverlayBadge` component of the `prime-overlaybadge` crate SHALL render `<div class="p-overlaybadge">`, inside which the child content comes first, then the `Badge`. `style_class` and `style` SHALL be passed to the nested `Badge`, not to the root, as in PrimeNG. `pt.root` SHALL add classes, style and attributes to the root, `pt.pc_badge` — to the nested `Badge`.

#### Scenario: Order
- **WHEN** an icon is nested inside `OverlayBadge` with `value = "2"`
- **THEN** inside `div.p-overlaybadge` the icon comes first, then `span.p-badge`

#### Scenario: Badge properties
- **WHEN** `severity = Danger`, `badge_size = Large`, `style_class = "mine"` are set
- **THEN** the nested badge has the classes `p-badge-danger`, `p-badge-lg` and `mine`, and the root `p-overlaybadge` does not

#### Scenario: Without a value
- **WHEN** `value` is not set
- **THEN** the nested badge is rendered as a dot (`p-badge-dot`)

#### Scenario: Hiding
- **WHEN** `badge_disabled = true`
- **THEN** the nested badge is hidden, and the content remains

#### Scenario: pt
- **WHEN** `pt.root` contains the attribute `data-x="1"`, and `pt.pc_badge` the class `custom`
- **THEN** the root has `data-x="1"`, and the nested badge has the class `custom`

### Requirement: Position of the badge
The position of the badge SHALL be set by the CSS of the theme `overlaybadge`: the root is positioned relatively, the badge is pinned to the top-right corner and shifted outward by half of its size.

#### Scenario: Corner
- **WHEN** a page with the theme CSS connected shows an icon with a badge
- **THEN** the centre of the badge coincides with the top-right corner of the root

### Requirement: Rendering cost of OverlayBadge
The number of allocations per instance during SSR SHALL fit the budget `overlaybadge-1000` in `benches/alloc-budgets.toml`, and the wasm contribution — the budget `overlaybadge` in `benches/budgets.toml`.

#### Scenario: Budgets
- **WHEN** `cargo xtask alloc-budget` and `cargo xtask wasm-size` are run
- **THEN** the values of OverlayBadge do not exceed the budgets
