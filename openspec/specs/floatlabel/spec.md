# floatlabel Specification

## Purpose
FloatLabel shows a label over an empty field and lifts it on focus or input, in three placement variants.

## Requirements

### Requirement: Markup of FloatLabel
The `FloatLabel` component of the `prime-floatlabel` crate SHALL render `<span class="p-floatlabel p-floatlabel-over">` with the child content (the field and its `<label>`); `variant = On` SHALL give `p-floatlabel-on`, `variant = In` — `p-floatlabel-in`; the default is `Over`. The root classes SHALL contain exactly one of the three variants. `style_class` SHALL be added after the own classes, `style` — sets the inline style, `pt.root` — adds classes, style and attributes without replacing the own ones. The position, lift and size of the label SHALL be provided by the CSS of the theme `floatlabel` according to the state of the nested field (`:focus`, being filled `p-filled`, `p-inputwrapper-filled`, `p-inputwrapper-focus`).

#### Scenario: Default variant
- **WHEN** `variant` is not set
- **THEN** the root class is `p-floatlabel p-floatlabel-over`

#### Scenario: Variants
- **WHEN** `variant = In`, then `On`
- **THEN** the root classes are `p-floatlabel p-floatlabel-in` and `p-floatlabel p-floatlabel-on`

#### Scenario: Reactive variant
- **WHEN** `variant` is passed as a signal and changes from `Over` to `On`
- **THEN** the root class is replaced without recreating the child elements

#### Scenario: Label lift
- **WHEN** the nested field receives focus or a value
- **THEN** the label rises above the field (in the `Over` variant), or shrinks inside the field (`In`), or sits on the top border (`On`)

#### Scenario: Invalid value
- **WHEN** the nested field has the class `p-invalid`
- **THEN** the label gets the invalid-value color from the theme

### Requirement: Rendering cost of FloatLabel
The number of allocations per instance during SSR SHALL fit the budget `floatlabel-1000` in `benches/alloc-budgets.toml`, and the wasm contribution — the budget `floatlabel` in `benches/budgets.toml`.

#### Scenario: Budgets
- **WHEN** `cargo xtask alloc-budget` and `cargo xtask wasm-size` are run
- **THEN** the values of FloatLabel do not exceed the budgets
