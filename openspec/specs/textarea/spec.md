# textarea Specification

## Purpose
TBD - created by archiving change port-textarea. Update Purpose after archiving.

## Requirements

### Requirement: Value and events of Textarea
The `Textarea` component of the `prime-textarea` crate SHALL render a native `<textarea>` and accept an optional two-way `value` (`RwSignal<String>`): the signal's value is rendered in the field (during SSR as child text), and the user's input is written back. It SHALL call `on_input`, `on_change`, `on_focus`, `on_blur` with the original event.

#### Scenario: Signal in the field
- **WHEN** `value` contains `abc`
- **THEN** the HTML contains `<textarea …>abc</textarea>`, and a change of the signal on the client updates the field

#### Scenario: Input into the signal
- **WHEN** the user types text
- **THEN** the signal receives the text and `on_input` is called

### Requirement: Attributes and classes of Textarea
`<textarea>` SHALL get `id`, `name`, `placeholder`, `rows`, `cols`, `disabled`, `readonly`, `required`, `maxlength`, `aria-label`, `aria-describedby`; unset attributes SHALL be absent. The classes SHALL be `p-textarea p-component` and the modifiers in this order: `p-filled` (the value is not empty), `p-textarea-resizable` (`auto_resize`), `p-variant-filled` (`variant = filled` or `PrimeConfig.input_variant = filled`), `p-textarea-fluid`, `p-inputfield-sm p-textarea-sm` (small), `p-textarea-lg p-inputfield-lg` (large), `p-invalid`. `style_class` and `style` SHALL be added without losing the own classes.

#### Scenario: Modifiers
- **WHEN** `size = small`, `variant = filled`, `invalid`, `fluid`, `auto_resize` are set
- **THEN** the classes `p-textarea-resizable`, `p-variant-filled`, `p-textarea-fluid`, `p-inputfield-sm p-textarea-sm`, `p-invalid` are present

#### Scenario: Attributes
- **WHEN** `rows = 5`, `cols = 30`, `placeholder = "Hi"` are set
- **THEN** `<textarea>` has `rows="5"`, `cols="30"`, `placeholder="Hi"`, and there is no `disabled` or `readonly`

### Requirement: Height auto-resize of Textarea
With `auto_resize` the Textarea SHALL, in the browser, on mounting and on each change of the value, set the height from `scrollHeight`; when `max-height` is reached it SHALL enable `overflow-y: scroll` and limit the height, otherwise `overflow: hidden`; after each adjustment it SHALL call `on_resize`.

#### Scenario: Text growth
- **WHEN** multi-line text is typed into a field with `auto_resize`
- **THEN** the height of the field increases and `on_resize` is called

#### Scenario: Height limit
- **WHEN** the content is above `max-height`
- **THEN** the height equals `max-height` and vertical scrolling is enabled

### Requirement: Pass-through of Textarea
`pt` SHALL accept the root `<textarea>`, adding classes, style and attributes without replacing the own ones.

#### Scenario: Class through pt
- **WHEN** `pt` with the class `custom` is passed
- **THEN** `<textarea>` has both `custom` and the own classes

### Requirement: Rendering cost of Textarea
A Textarea without `value` and with static properties SHALL use the static branch; both branches SHALL produce the same markup. The number of allocations per instance during SSR SHALL fit the budget `textarea` in `benches/alloc-budgets.toml`, and the wasm contribution — the budget `textarea` in `benches/budgets.toml`.

#### Scenario: Branch parity
- **WHEN** the same set of values is passed statically and through signals
- **THEN** the HTML matches

#### Scenario: Budgets
- **WHEN** `cargo xtask alloc-budget` and `cargo xtask wasm-size` are run
- **THEN** the values of Textarea do not exceed the budgets
