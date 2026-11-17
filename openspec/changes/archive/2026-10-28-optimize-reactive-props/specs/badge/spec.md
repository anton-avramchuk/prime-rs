## ADDED Requirements

### Requirement: Badge render cost
Rendering a `Badge` with static properties SHALL NOT create signals or `Memo`s (neither for properties nor for state), and the allocation count per instance during SSR SHALL fit within the budget set in `benches` and checked by the benchmark command. Classes and attributes SHALL be computed in reactive attribute closures and recomputed only when the input properties change.

#### Scenario: Static Badge
- **WHEN** 1000 instances of `Badge` with static properties are rendered
- **THEN** the allocation count per instance does not exceed the `badge` budget

#### Scenario: Reactive Badge
- **WHEN** the `value` property is passed as a signal and changes
- **THEN** the badge's text and classes update, and `class` is not rewritten when a property that does not affect the classes changes

#### Scenario: Behavior unchanged
- **WHEN** the tests of the `badge` spec (classes, `data-p`, hiding, `pt`) are run
- **THEN** all of them pass without changing expected values
