# inputtext Specification

## Purpose
TBD - created by archiving change port-inputtext. Update Purpose after archive.

## Requirements

### Requirement: InputText value and events
The `InputText` component of the `prime-inputtext` crate SHALL render a native `<input>` and accept an optional two-way `value` (`RwSignal<String>`): the signal value is shown in the field, user input is written back to the signal. It SHALL call `on_input`, `on_change`, `on_focus`, `on_blur` with the original event.

#### Scenario: Signal in the field
- **WHEN** `value` contains `abc`
- **THEN** the `<input>` has the value `abc`; when the signal changes the field updates

#### Scenario: Input into the signal
- **WHEN** the user types text into the field
- **THEN** the `value` signal receives the typed text and `on_input` is called

### Requirement: InputText attributes
The component SHALL pass to the `<input>` the attributes `type` (`input_type`, `text` by default), `id`, `name`, `placeholder`, `disabled`, `readonly`, `required`, `maxlength`, `autocomplete`, `aria-label`, `aria-describedby`; attributes that are not set SHALL be absent.

#### Scenario: Attributes
- **WHEN** `placeholder = "Name"`, `name = "n"`, `disabled = true` are set
- **THEN** the `<input>` has `placeholder="Name"`, `name="n"` and `disabled`, and no `readonly` or `required`

#### Scenario: Default type
- **WHEN** `input_type` is not set
- **THEN** the `<input>` has the attribute `type="text"`

### Requirement: InputText classes and attributes
The root SHALL have the classes `p-inputtext p-component` and the modifiers: `p-filled` (`value` is not empty), `p-inputtext-sm` and `p-inputtext-lg` (by `size`), `p-invalid` (`invalid`), `p-variant-filled` (`variant = filled`, and when `variant` is not set — `PrimeConfig.input_variant = filled`), `p-inputtext-fluid` (`fluid`). The `data-p` attribute SHALL contain, space-separated, `invalid`, `fluid`, `filled` (with the `filled` variant) and the size (`small` or `large`) in that order. `style_class` and `style` SHALL be added without losing its own classes.

#### Scenario: Size, variant, invalid, fluid
- **WHEN** `size = small`, `variant = filled`, `invalid`, `fluid` are set
- **THEN** the classes are `p-inputtext-sm`, `p-variant-filled`, `p-invalid`, `p-inputtext-fluid`, and `data-p` equals `invalid fluid filled small`

#### Scenario: Filled field
- **WHEN** `value` is not empty
- **THEN** the `<input>` has `p-filled`; when the value is cleared the class disappears

#### Scenario: Variant from the configuration
- **WHEN** `variant` is not set and `PrimeConfig.input_variant = filled`
- **THEN** the `<input>` has `p-variant-filled`

### Requirement: Pass-through InputText
`InputText` SHALL accept `pt` for the root element; classes and attributes are added to its own, not replacing them.

#### Scenario: Class via pt
- **WHEN** `pt` with the class `custom` is passed
- **THEN** the `<input>` has both `custom` and its own classes

### Requirement: InputText rendering cost
`InputText` without a bound `value` and with static properties SHALL use the static rendering branch; with `value` or signal properties — the reactive one; both branches SHALL produce the same markup. The number of allocations per instance in SSR SHALL fit within the `inputtext` budget in `benches/alloc-budgets.toml`, the wasm contribution — within the `inputtext` budget in `benches/budgets.toml`.

#### Scenario: Branch parity
- **WHEN** the same set of values is passed statically and as signals
- **THEN** the HTML matches

#### Scenario: Budgets
- **WHEN** `cargo xtask alloc-budget` and `cargo xtask wasm-size` are run
- **THEN** the InputText values do not exceed the budgets
