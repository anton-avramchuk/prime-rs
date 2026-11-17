## Purpose
InputOtp lets the user enter a one-time code one character at a time in separate cells with auto-advance, pasting of the whole code and masking.

## ADDED Requirements

### Requirement: InputOtp markup
The `InputOtp` component of the `prime-inputotp` crate SHALL render `<div class="p-inputotp p-component">` with `length` (default 4) `<input type="text">` fields, each with the classes `p-inputtext p-component p-inputotp-input`. `style_class` SHALL be appended to the root classes, `style` SHALL set the root style, `pt.root` and `pt.input` SHALL add classes, style and attributes to the root and each cell.

#### Scenario: Number of cells
- **WHEN** `length = 6`
- **THEN** six `input.p-inputotp-input` fields are rendered

#### Scenario: Default
- **WHEN** `length` is not set
- **THEN** four fields are rendered

#### Scenario: pt
- **WHEN** `pt.input` contains the class `custom`
- **THEN** each field has the class `custom`

### Requirement: InputOtp value
The value SHALL be a string whose characters fill the cells in order; cells beyond the value length are empty, extra characters beyond `length` are not shown. User input SHALL be written to `value` as a joined string of tokens.

#### Scenario: Initial value
- **WHEN** `value = "12"` with `length = 4`
- **THEN** the first two cells contain `1` and `2`, the rest are empty

#### Scenario: Write
- **WHEN** the user types `1`, `2`, `3`, `4` into the cells in order
- **THEN** `value` equals `1234`, and `on_change` is called with this value

### Requirement: InputOtp cell attributes
Cells SHALL get `maxlength="1"` (the first one gets `maxlength` equal to `length` to accept paste), `inputmode="numeric"` with `integer_only` and `text` otherwise, `type="password"` with `mask`, as well as `name`, `tabindex`, `required`, `readonly` and `disabled` from the properties. Cell classes SHALL follow the `InputText` rules: `p-invalid`, `p-variant-filled`, `p-inputtext-sm|lg`.

#### Scenario: Mask
- **WHEN** `mask = true`
- **THEN** each cell has `type="password"`

#### Scenario: Digits only
- **WHEN** `integer_only = true`
- **THEN** each cell has `inputmode="numeric"`

#### Scenario: Size and invalid value
- **WHEN** `size = Small`, `invalid = true`
- **THEN** cells have the classes `p-inputtext-sm` and `p-invalid`

#### Scenario: Disabled
- **WHEN** `disabled = true`
- **THEN** each cell has the `disabled` attribute

### Requirement: InputOtp cell navigation
After a character is typed into a cell, focus SHALL move to the next cell, and on backward deletion to the previous one. `ArrowRight` and `ArrowLeft` SHALL move focus to the neighboring cell, `ArrowUp` and `ArrowDown` SHALL be cancelled, `Backspace` in an empty cell SHALL move to the previous one. When a cell receives focus its content SHALL be selected.

#### Scenario: Auto-advance
- **WHEN** the user types a character into the first cell
- **THEN** focus moves to the second

#### Scenario: Backspace in an empty cell
- **WHEN** the user presses `Backspace` in an empty second cell
- **THEN** focus moves to the first

#### Scenario: Arrows
- **WHEN** the user presses `ArrowRight`, then `ArrowLeft`
- **THEN** focus shifts to the next, then to the previous cell

### Requirement: InputOtp typing restrictions
With `integer_only`, typing non-digit characters SHALL be cancelled. With `length` characters filled, further typing without a selection SHALL be cancelled (except `Delete`). Keys with `Alt`, `Ctrl` or `Meta` SHALL NOT be handled.

#### Scenario: Digits only
- **WHEN** `integer_only = true`, the user presses `a`
- **THEN** the character is not entered

#### Scenario: Overflow
- **WHEN** all cells are filled and the user presses a digit without a selection
- **THEN** the character is not entered

#### Scenario: Key combination
- **WHEN** the user presses `Ctrl+A`
- **THEN** the key press is not cancelled by the component

### Requirement: InputOtp paste
Pasting from the clipboard into any cell SHALL distribute up to `length` characters across the cells from the start and write the value; with `integer_only` a paste with non-digit content SHALL be ignored. With `readonly` and `disabled` paste SHALL be ignored.

#### Scenario: Pasting a code
- **WHEN** the user pastes `123456` with `length = 4`
- **THEN** the cells contain `1`, `2`, `3`, `4`, the value equals `1234`

#### Scenario: Non-digit paste
- **WHEN** `integer_only = true`, the user pastes `12ab`
- **THEN** the value does not change

#### Scenario: Read-only
- **WHEN** `readonly = true`, the user pastes a code
- **THEN** the value does not change

### Requirement: InputOtp cell template
The `input` template SHALL replace the standard field of each cell, receiving the cell token, its index and event handlers (`input`, `keydown`, `focus`, `blur`, `paste`) that the template attaches to its field.

#### Scenario: Custom field
- **WHEN** the `input` template is set, rendering `input.custom-otp-input`
- **THEN** `length` such fields are rendered instead of the standard ones

### Requirement: InputOtp rendering cost
The number of allocations per instance on SSR SHALL fit within the `inputotp-1000` budget in `benches/alloc-budgets.toml`, the wasm contribution within the `inputotp` budget in `benches/budgets.toml`.

#### Scenario: Budgets
- **WHEN** `cargo xtask alloc-budget` and `cargo xtask wasm-size` are run
- **THEN** InputOtp values do not exceed the budgets
