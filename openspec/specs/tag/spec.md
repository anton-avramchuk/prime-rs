# tag Specification

## Purpose
The Tag component (port of PrimeNG `p-tag`): value, severity, rounding, icon, content, pass-through, and static and reactive render branches.

## Requirements

### Requirement: Tag properties
The `Tag` component of the `prime-tag` crate SHALL accept `value` (label text), `severity` (`secondary`, `info`, `success`, `warn`, `danger`, `contrast`), `rounded`, `icon` (Leptos content), `style`, `style_class`, `children` and `pt`. All properties are optional; those that change at runtime SHALL accept a value or a signal.

#### Scenario: Value
- **WHEN** `Tag` is rendered with the value `New`
- **THEN** it contains `<span class="p-tag-label">New</span>`

#### Scenario: Empty Tag
- **WHEN** `Tag` is rendered without properties
- **THEN** there is a root element with the classes `p-tag p-component` and an empty label `p-tag-label`, as in PrimeNG

### Requirement: Tag classes and attributes
The root element SHALL have the classes `p-tag p-component` and modifiers: `p-tag-<severity>` for severity (`info`, `success`, `warn`, `danger`, `secondary`, `contrast`) and `p-tag-rounded` with `rounded`. The `data-p` attribute SHALL contain `rounded` and the severity separated by a space, as in PrimeNG. `style_class` and `style` SHALL be appended without losing the component's own classes.

#### Scenario: Severity and rounding
- **WHEN** `severity = success` and `rounded = true` are set
- **THEN** the root has the classes `p-tag-success` and `p-tag-rounded`, and `data-p` equals `rounded success`

#### Scenario: Custom class and style
- **WHEN** `style_class = "custom"` and `style = "margin: 4px"` are set
- **THEN** the root has `custom`, the component's own classes remain, and the style equals `margin: 4px`

### Requirement: Tag content and icon
`children`, if set, SHALL be rendered first, like projected content in PrimeNG; then the icon in `<span class="p-tag-icon">`, then the `p-tag-label` label. The icon SHALL be rendered only if set.

#### Scenario: Content order
- **WHEN** `children`, `icon` and `value` are set
- **THEN** the order of child nodes is: `children`, `span.p-tag-icon`, `span.p-tag-label`

#### Scenario: Without an icon
- **WHEN** `icon` is not set
- **THEN** there is no `p-tag-icon` element

### Requirement: Pass-through Tag
`Tag` SHALL accept `pt` for the root, icon and label; classes and attributes from `pt` are added to the component's own, not replacing them.

#### Scenario: pt on the label
- **WHEN** a `pt` with the class `extra` is passed for the label
- **THEN** `p-tag-label` has both `p-tag-label` and `extra`

### Requirement: Tag rendering cost
`Tag` SHALL use the static render branch if all properties are static, and the reactive one otherwise; both branches SHALL produce the same markup. The number of allocations per instance on SSR SHALL fit within the `tag` budget in `benches/alloc-budgets.toml`, the wasm contribution within the `tag` budget in `benches/budgets.toml`.

#### Scenario: Branch parity
- **WHEN** the same set of values is passed statically and as signals
- **THEN** the HTML matches

#### Scenario: Budgets
- **WHEN** `cargo xtask alloc-budget` and `cargo xtask wasm-size` are run
- **THEN** Tag values do not exceed the budgets
