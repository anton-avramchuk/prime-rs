# core-runtime Specification

## Purpose
Shared component core: configuration via context, class building, shared props, pass-through, z-index and DOM helpers for overlays.

## Requirements

### Requirement: Global configuration via context
`prime-rs-core` SHALL provide a configuration (locale, ripple, base z-index, input field variant, theme name) that components obtain from the Leptos context. If no configuration is provided, components SHALL work with default values, as in PrimeNG.

#### Scenario: Configuration not set
- **WHEN** a component is rendered outside a configuration provider
- **THEN** it uses default values and does not panic

#### Scenario: Configuration overridden
- **WHEN** the application sets `ripple = true` via the provider
- **THEN** components inside the provider get this value

### Requirement: CSS class list building
`prime-rs-core` SHALL provide a way to assemble a list of `p-*` classes from base classes, conditional modifiers and user classes without extra allocations if the set of classes has not changed.

#### Scenario: Conditional classes
- **WHEN** a component is in the `disabled` state
- **THEN** the resulting class string contains the base class and `p-disabled`, without empty or duplicate items

### Requirement: Base input props
`prime-rs-core` SHALL define the shared props `disabled`, `invalid`, `variant`, `size` with the same values as in PrimeNG, for reuse by input components.

#### Scenario: Variant values
- **WHEN** `variant = filled`
- **THEN** the component adds the same modifier class as PrimeNG

### Requirement: Root element pass-through
Each component SHALL accept `pt` for the root element and main parts, allowing attributes and classes to be added without breaking the component's own classes.

#### Scenario: Adding a class via pt
- **WHEN** a `pt` with the class `custom` is passed
- **THEN** the root element contains both `custom` and the component's own classes

### Requirement: z-index manager
`prime-rs-core` SHALL issue increasing z-index values for overlays by group (`modal`, `overlay`, `menu`, `tooltip`) and release them on close.

#### Scenario: Two overlays in a row
- **WHEN** a second overlay of the same group opens
- **THEN** its z-index is greater than the first's

### Requirement: DOM helpers for overlays
`prime-dom` SHALL provide positioning relative to an anchor taking window bounds into account, focus management (focus trap) and a portal into `body`. These functions SHALL be SSR-safe: on the server they perform no DOM operations.

#### Scenario: Not enough space below
- **WHEN** an overlay does not fit below the anchor
- **THEN** it is positioned the same way as in PrimeNG and stays within the window

#### Scenario: Focus trap
- **WHEN** a focus trap is active and the user presses Tab on the last focusable element
- **THEN** focus moves to the first focusable element

#### Scenario: SSR
- **WHEN** the helper is called under `ssr`
- **THEN** it finishes without panicking and without touching `window`

### Requirement: "Value or signal" property
`prime-rs-core` SHALL provide a component property type that accepts either a ready-made value or a signal (`Signal`, `ReadSignal`, `RwSignal`, `Memo`). Passing a ready-made value SHALL NOT create reactive primitives (Leptos arena entries); the default value SHALL be static. Reading the property SHALL be reactive if a signal is passed, and SHALL subscribe the reading closure to it.

#### Scenario: Static value
- **WHEN** a component is passed the value `"5"` or an enum constant
- **THEN** no signal or `Memo` is created when the property is created, and reading returns this value

#### Scenario: Signal
- **WHEN** a component is passed an `RwSignal` whose value then changes
- **THEN** reading the property in a reactive closure returns the new value and the closure restarts

#### Scenario: Default value
- **WHEN** an optional property is not passed
- **THEN** it equals the type's default value and creates no reactive primitives

### Requirement: Static detection
The property type SHALL allow finding out that the value is static, so a component can skip subscriptions.

#### Scenario: All properties static
- **WHEN** all of a component's properties are static
- **THEN** the static check returns `true` for each of them

### Requirement: Static check of a property set
`prime-rs-core` SHALL provide a way to check that all of a component's properties are static (`Prop::Static`), so the component can choose a render path without reactive closures. The check SHALL be cheap (no allocations and no subscriptions) and performed once when an instance is created.

#### Scenario: All properties static
- **WHEN** a set of `Prop::Static` values and default values is checked
- **THEN** the result is `true`

#### Scenario: A signal is present
- **WHEN** at least one of the checked properties is passed as a signal
- **THEN** the result is `false`
