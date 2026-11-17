## ADDED Requirements

### Requirement: "Value or signal" property
`prime-core` SHALL provide a component property type that accepts either a ready value or a signal (`Signal`, `ReadSignal`, `RwSignal`, `Memo`). Passing a ready value SHALL NOT create reactive primitives (entries in the Leptos arena); the default value SHALL be static. Reading the property SHALL be reactive if a signal is passed, and SHALL subscribe the reading closure to it.

#### Scenario: Static value
- **WHEN** the value `"5"` or an enum constant is passed to a component
- **THEN** no signal or `Memo` is created when the property is created, and reading returns this value

#### Scenario: Signal
- **WHEN** an `RwSignal` whose value then changes is passed to a component
- **THEN** reading the property in a reactive closure returns the new value and the closure reruns

#### Scenario: Default value
- **WHEN** an optional property is not passed
- **THEN** it equals the type's default value and creates no reactive primitives

### Requirement: Determining staticness
The property type SHALL make it possible to find out that the value is static so that a component can skip subscriptions.

#### Scenario: All properties static
- **WHEN** all of a component's properties are static
- **THEN** the staticness check returns `true` for each of them
