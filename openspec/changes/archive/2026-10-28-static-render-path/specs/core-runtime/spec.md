## ADDED Requirements

### Requirement: Checking that a set of properties is static
`prime-core` SHALL provide a way to check that all properties of a component are static (`Prop::Static`), so that the component can select a render path without reactive closures. The check SHALL be cheap (without allocations and without subscriptions) and SHALL run once when the instance is created.

#### Scenario: All properties are static
- **WHEN** a set of `Prop::Static` values and default values is checked
- **THEN** the result is `true`

#### Scenario: There is a signal
- **WHEN** at least one of the checked properties is passed as a signal
- **THEN** the result is `false`
