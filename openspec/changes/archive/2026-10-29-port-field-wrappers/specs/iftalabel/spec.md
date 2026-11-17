## Purpose
IftaLabel shows a label above the value inside the field, keeping it in place (the "inside field, top aligned" style).

## ADDED Requirements

### Requirement: Markup of IftaLabel
The `IftaLabel` component of the `prime-iftalabel` crate SHALL render `<span class="p-iftalabel">` with the child content (the field and its `<label>`). `style_class` SHALL be added after the own classes, `style` — sets the inline style, `pt.root` — adds classes, style and attributes without replacing the own ones. The position of the label above the value and the paddings of the field SHALL be provided by the CSS of the theme `iftalabel`.

#### Scenario: Root
- **WHEN** a field and a label are nested inside `IftaLabel`
- **THEN** one `span.p-iftalabel` is rendered with both elements in the original order

#### Scenario: Classes and style
- **WHEN** `style_class = "mine"` and `style = "width: 14rem"` are set
- **THEN** the root class is `p-iftalabel mine`, the attribute `style="width: 14rem;"`

#### Scenario: Label position
- **WHEN** a page with the theme CSS connected shows an `IftaLabel` with an `InputText`
- **THEN** the label lies inside the bounds of the field above the value

#### Scenario: Invalid value
- **WHEN** the nested field has the class `p-invalid`
- **THEN** the label gets the invalid-value color from the theme

### Requirement: Rendering cost of IftaLabel
The number of allocations per instance during SSR SHALL fit the budget `iftalabel-1000` in `benches/alloc-budgets.toml`, and the wasm contribution — the budget `iftalabel` in `benches/budgets.toml`.

#### Scenario: Budgets
- **WHEN** `cargo xtask alloc-budget` and `cargo xtask wasm-size` are run
- **THEN** the values of IftaLabel do not exceed the budgets
