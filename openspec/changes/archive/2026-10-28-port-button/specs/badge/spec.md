## ADDED Requirements

### Requirement: Badge properties
The `Badge` component of the `prime-badge` crate SHALL accept a value (`value`: a string or a number), a severity (`severity`: `secondary`, `info`, `success`, `warn`, `danger`, `contrast`), a size (`size`: `small`, `large`, `xlarge`), a hiding flag (`badge_disabled`) and an additional class (`style_class`). All properties are optional. Properties that change at runtime SHALL be reactive.

#### Scenario: The value is rendered as text
- **WHEN** `Badge` is rendered with the value `5`
- **THEN** the root element contains the text `5`

#### Scenario: Default properties
- **WHEN** `Badge` is rendered without properties
- **THEN** it is rendered without text, without severity and size classes

### Requirement: Classes and attributes of Badge
The root element SHALL have the classes `p-badge p-component` and the modifiers, as in PrimeNG: `p-badge-circle` for a value of one character, `p-badge-dot` for an empty value, `p-badge-sm`, `p-badge-lg`, `p-badge-xl` for the sizes and `p-badge-<severity>` for the severity. The `data-p` attribute SHALL contain, separated by spaces, the same flags as in PrimeNG (`circle`, `empty`, `disabled`, severity, size).

#### Scenario: A single character — a circle
- **WHEN** the value equals `5`
- **THEN** the root has the class `p-badge-circle`, and there is no `p-badge-dot`

#### Scenario: An empty value — a dot
- **WHEN** the value is not set
- **THEN** the root has the class `p-badge-dot`, and `data-p` contains `empty`

#### Scenario: Severity and size
- **WHEN** `severity = danger` and `size = xlarge` are set
- **THEN** the root has the classes `p-badge-danger` and `p-badge-xl`

#### Scenario: An additional class
- **WHEN** `style_class = "custom"` is set
- **THEN** the root has both `custom` and the component's own classes

### Requirement: Hiding Badge
With `badge_disabled = true` the component SHALL stay in the DOM but not be displayed (`display: none`); with `false` the `display` style SHALL be absent.

#### Scenario: A hidden badge
- **WHEN** `badge_disabled = true`
- **THEN** the root has the style `display: none`, and `data-p` contains `disabled`

### Requirement: Pass-through Badge
`Badge` SHALL accept `pt` for the root element (additional classes and attributes).

#### Scenario: Class through pt
- **WHEN** `pt` with the class `custom` is passed
- **THEN** the root contains `custom` and the own classes

### Requirement: Performance of Badge
The rendering of `Badge` SHALL fit within the budgets set by `benches/budgets.toml`, and it SHALL have scenarios for 1 and 1000 instances in `render-bench`.

#### Scenario: Size budget
- **WHEN** `cargo xtask wasm-size` is run
- **THEN** the contribution of `case-badge` does not exceed the budget
