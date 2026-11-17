## ADDED Requirements

### Requirement: Static branch without client primitives
Components with a static branch (Button, Chip) SHALL NOT create `NodeRef`, `Arc<Props>` and `Arc<Pt>` during SSR when the properties are static: they are allowed only in the reactive branch and on the client.

#### Scenario: Static button during SSR
- **WHEN** a `Button` with static properties is rendered on the native target
- **THEN** the allocation count per instance fits within the `button-1000` budget and does not include a `NodeRef`

#### Scenario: Behavior does not change
- **WHEN** the button is `disabled` or in the loading state
- **THEN** `on_click` is still not called, and the Button and Chip browser tests pass
