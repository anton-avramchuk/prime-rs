# confirmation Specification

## ADDED Requirements

### Requirement: Response type
`prime-confirmation` SHALL provide `ConfirmEventType` with the variants `Accept` (accept by a button), `Reject` (reject by a button) and `Cancel` (closing the window without a choice: close icon, `Escape`, a click on the mask), and `as_str` SHALL return `"accept"`, `"reject"` and `"cancel"`.

#### Scenario: Values
- **WHEN** the type strings of `Accept`, `Reject` and `Cancel` are requested
- **THEN** `"accept"`, `"reject"` and `"cancel"` are obtained
