# upstream-tracking Specification

## Purpose
Binding the port to a version of PrimeNG, finding changes in its sources, and generating spec skeletons of the components.

## Requirements

### Requirement: Pinned PrimeNG version
The repository SHALL store in `upstream.toml` the version (`21.1.10`) and the commit (`fe82b291d6`) of PrimeNG with which the port is synchronized, as well as the path to the local copy, which can be overridden.

#### Scenario: Reading the version
- **WHEN** the tooling requests the upstream version
- **THEN** it gets the values from `upstream.toml`

### Requirement: Finding changes in upstream
The system SHALL provide a command that shows which PrimeNG sources referenced by the ported components have changed between the pinned commit and the target commit.

#### Scenario: A source changed
- **WHEN** the file `button.ts` changed between the commits, and the Button component is ported
- **THEN** the command lists Button among the affected components

#### Scenario: No changes
- **WHEN** the sources of the ported components have not changed
- **THEN** the command reports that there are no changes and exits with code 0

### Requirement: Spec skeleton from the sources
The system SHALL provide a command that generates a spec skeleton of a component (props, events, DOM template, classes) from the TypeScript sources of PrimeNG, with a unified structure of sections and a reference to the source file and the commit.

#### Scenario: Generating a skeleton for Button
- **WHEN** the command is run for `button`
- **THEN** a skeleton with the sections props, events, DOM and classes, a11y, keyboard and performance is created, together with a reference to `button.ts` and the pinned commit

#### Scenario: Unknown component
- **WHEN** a non-existent component is requested
- **THEN** the command exits with a clear error and creates no files
