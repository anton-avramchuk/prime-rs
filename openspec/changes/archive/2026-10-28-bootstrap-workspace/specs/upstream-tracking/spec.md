## ADDED Requirements

### Requirement: Pinned PrimeNG version
The repository SHALL store in `upstream.toml` the version (`21.1.10`) and commit (`fe82b291d6`) of PrimeNG the port is synchronized with, as well as the path to a local copy with an override option.

#### Scenario: Reading the version
- **WHEN** the tooling requests the upstream version
- **THEN** it gets the values from `upstream.toml`

### Requirement: Finding changes in upstream
The system SHALL provide a command showing which PrimeNG sources referenced by the ported components changed between the pinned and the target commit.

#### Scenario: Source changed
- **WHEN** the file `button.ts` changed between commits and the Button component is ported
- **THEN** the command lists Button among the affected components

#### Scenario: No changes
- **WHEN** the sources of the ported components did not change
- **THEN** the command reports no changes and exits with code 0

### Requirement: Spec stub from sources
The system SHALL provide a command that produces a component spec stub (props, events, DOM template, classes) from the PrimeNG TypeScript sources with a uniform section structure and a link to the source file and commit.

#### Scenario: Generating a stub for Button
- **WHEN** the command is run for `button`
- **THEN** a stub is created with sections for props, events, DOM and classes, a11y, keyboard and performance, and a link to `button.ts` and the pinned commit

#### Scenario: Unknown component
- **WHEN** a nonexistent component is requested
- **THEN** the command fails with a clear error and creates no files
