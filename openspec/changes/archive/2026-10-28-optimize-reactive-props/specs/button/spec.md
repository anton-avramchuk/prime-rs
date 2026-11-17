## ADDED Requirements

### Requirement: Button render cost
Rendering a `Button` with static properties SHALL NOT create signals or `Memo`s (neither for properties nor for state), except the autofocus and ripple effects, which exist only when building for `wasm32`. The allocation count per instance during SSR SHALL fit within the budget set in `benches` and checked by the benchmark command. Classes and attributes SHALL be computed in reactive attribute closures.

#### Scenario: Static Button
- **WHEN** 1000 instances of `Button` with a label and severity are rendered
- **THEN** the allocation count per instance does not exceed the `button` budget

#### Scenario: Full button
- **WHEN** 1000 buttons with an icon, label and badge are rendered
- **THEN** the allocation count per instance does not exceed the `button-full` budget

#### Scenario: Reactive button
- **WHEN** the `loading` property is passed as a signal and changes
- **THEN** the class, attributes, disabled state and indicator update, and `class` is not rewritten when a property that does not affect the classes changes

#### Scenario: Behavior unchanged
- **WHEN** the tests of the `button` spec (SSR and browser) are run
- **THEN** all of them pass without changing expected values
