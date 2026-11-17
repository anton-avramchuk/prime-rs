## ADDED Requirements

### Requirement: Global configuration via context
`prime-core` SHALL provide a configuration (locale, ripple, base z-index values, input variant, theme name) that components obtain from the Leptos context. If no configuration is provided, components SHALL work with the default values, as in PrimeNG.

#### Scenario: Configuration not set
- **WHEN** a component is rendered outside a configuration provider
- **THEN** it uses the default values and does not panic

#### Scenario: Configuration overridden
- **WHEN** the application sets `ripple = true` via the provider
- **THEN** components inside the provider receive this value

### Requirement: Building the CSS class list
`prime-core` SHALL provide a way to assemble a list of `p-*` classes from base classes, conditional modifiers and user classes without extra allocations if the class set has not changed.

#### Scenario: Conditional classes
- **WHEN** a component is in the `disabled` state
- **THEN** the resulting class string contains the base class and `p-disabled`, with no empty or repeated items

### Requirement: Base input props
`prime-core` SHALL define the shared props `disabled`, `invalid`, `variant`, `size` with the same values as in PrimeNG, for reuse by input components.

#### Scenario: variant values
- **WHEN** `variant = filled` is set
- **THEN** the component adds the same modifier class as PrimeNG

### Requirement: Root element pass-through
Every component SHALL accept `pt` for the root element and the main parts, allowing attributes and classes to be added without breaking the component's own classes.

#### Scenario: Adding a class via pt
- **WHEN** a `pt` with the class `custom` is passed
- **THEN** the root element contains both `custom` and the component's own classes

### Requirement: z-index manager
`prime-core` SHALL hand out increasing z-index values for overlays by group (`modal`, `overlay`, `menu`, `tooltip`) and release them on close.

#### Scenario: Two overlays in a row
- **WHEN** a second overlay of the same group is opened
- **THEN** its z-index is greater than the first one's

### Requirement: DOM helpers for overlays
`prime-dom` SHALL provide positioning relative to an anchor taking window bounds into account, focus management (focus trap) and a portal into `body`. These functions SHALL be SSR-safe: on the server they perform no DOM operations.

#### Scenario: Not enough room below
- **WHEN** an overlay does not fit under the anchor
- **THEN** it is positioned the same way as in PrimeNG and stays within the window

#### Scenario: Focus trap
- **WHEN** the focus trap is active and the user presses Tab on the last focusable element
- **THEN** focus moves to the first focusable element

#### Scenario: SSR
- **WHEN** the helper is called under `ssr`
- **THEN** it finishes without panicking and without touching `window`
