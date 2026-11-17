# badge Specification

## Purpose
The Badge component (port of p-badge from PrimeNG): value, severity, size, dot and circle, hiding, pass-through.

## Requirements

### Requirement: Badge properties
The `Badge` component of the `prime-badge` crate SHALL accept a value (`value`: string or number), severity (`severity`: `secondary`, `info`, `success`, `warn`, `danger`, `contrast`), size (`size`: `small`, `large`, `xlarge`), a hide flag (`badge_disabled`) and an extra class (`style_class`). All properties are optional. Properties that change at runtime SHALL be reactive.

#### Scenario: Value is rendered as text
- **WHEN** `Badge` is rendered with the value `5`
- **THEN** the root element contains the text `5`

#### Scenario: Default properties
- **WHEN** `Badge` is rendered without properties
- **THEN** it is rendered without text, without severity and size classes

### Requirement: Badge classes and attributes
The root element SHALL have the classes `p-badge p-component` and modifiers, as in PrimeNG: `p-badge-circle` for a single-character value, `p-badge-dot` for an empty value, `p-badge-sm`, `p-badge-lg`, `p-badge-xl` for sizes and `p-badge-<severity>` for severity. The `data-p` attribute SHALL contain, space-separated, the same flags as in PrimeNG (`circle`, `empty`, `disabled`, severity, size).

#### Scenario: A single character is a circle
- **WHEN** the value is `5`
- **THEN** the root has the class `p-badge-circle` and not `p-badge-dot`

#### Scenario: An empty value is a dot
- **WHEN** the value is not set
- **THEN** the root has the class `p-badge-dot` and `data-p` contains `empty`

#### Scenario: Severity and size
- **WHEN** `severity = danger` and `size = xlarge` are set
- **THEN** the root has the classes `p-badge-danger` and `p-badge-xl`

#### Scenario: Extra class
- **WHEN** `style_class = "custom"` is set
- **THEN** the root has both `custom` and the component's own classes

### Requirement: Badge hiding
With `badge_disabled = true` the component SHALL stay in the DOM but not be displayed (`display: none`); with `false` the `display` style SHALL be absent.

#### Scenario: Hidden badge
- **WHEN** `badge_disabled = true`
- **THEN** the root has the style `display: none` and `data-p` contains `disabled`

### Requirement: Pass-through Badge
`Badge` SHALL accept `pt` for the root element (extra classes and attributes).

#### Scenario: Class via pt
- **WHEN** a `pt` with the class `custom` is passed
- **THEN** the root contains `custom` and the component's own classes

### Requirement: Badge performance
`Badge` rendering SHALL fit within the budgets set by `benches/budgets.toml` and have scenarios for 1 and 1000 instances in `render-bench`.

#### Scenario: Size budget
- **WHEN** `cargo xtask wasm-size` is run
- **THEN** the `case-badge` contribution does not exceed the budget

### Requirement: Badge rendering cost
Rendering a `Badge` with static properties SHALL NOT create signals or `Memo` (neither for properties nor for state), and the number of allocations per instance on SSR SHALL fit within the budget set in `benches` and checked by the benchmark command. Classes and attributes SHALL be computed in reactive attribute closures and recomputed only when input properties change.

#### Scenario: Static Badge
- **WHEN** 1000 `Badge` instances with static properties are rendered
- **THEN** the number of allocations per instance does not exceed the `badge` budget

#### Scenario: Reactive Badge
- **WHEN** the `value` property is passed as a signal and changes
- **THEN** the badge text and classes are updated, and `class` is not overwritten when a property that does not affect classes changes

#### Scenario: Behavior unchanged
- **WHEN** the `badge` spec tests (classes, `data-p`, hiding, `pt`) are run
- **THEN** all of them pass without changes to the expected values

### Requirement: Badge static render path
If all `Badge` properties are static, it SHALL be rendered with ready-made attribute and text values, without reactive closures and `RenderEffect` in the browser; if at least one property is passed as a signal, the reactive path SHALL be used. Both paths SHALL produce the same markup for the same values, and the markup SHALL be described once.

#### Scenario: Same markup
- **WHEN** the same set of values is passed statically and via signals
- **THEN** the HTML of both variants matches

#### Scenario: Static Badge in the browser
- **WHEN** 1000 static `Badge` instances are mounted in the browser
- **THEN** the growth of wasm linear memory is noticeably lower than on the reactive path (values recorded in `docs/benchmarks.md`), and no `RenderEffect`s are created for attributes

#### Scenario: Reactive Badge
- **WHEN** at least one `Badge` property is passed as a signal and changes
- **THEN** the text, classes and `data-p` are updated
