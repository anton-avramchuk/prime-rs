# disk-footprint Specification

## Purpose
Measures against the growth of `target/` and of the CI cache: build profiles, cleaning, limiting dependencies.

## Requirements

### Requirement: Economical build profiles
The build profiles SHALL be configured so that the dev build does not contain full debug information and uses a fast linker, and incremental compilation is enabled only in dev.

#### Scenario: Dev profile
- **WHEN** `cargo build` is run in the dev profile
- **THEN** the debug information is limited to line tables, and the incremental cache is created

#### Scenario: Release and CI
- **WHEN** a release build or a build in CI is run
- **THEN** no incremental cache is created

### Requirement: Cleaning command
The system SHALL provide a cleaning command that removes obsolete `target/` artifacts (not used for longer than the given period) and reports the freed amount.

#### Scenario: Cleaning
- **WHEN** the cleaning command is run
- **THEN** obsolete artifacts are removed, fresh ones are kept, and the freed amount is printed

### Requirement: Limiting dependencies
External dependencies SHALL be included with `default-features = false` wherever possible, and CI SHALL check for the absence of unused and forbidden dependencies.

#### Scenario: Forbidden dependency
- **WHEN** a PR adds a dependency that violates the `cargo-deny` policy
- **THEN** the CI check fails

### Requirement: Control of the cache size in CI
CI SHALL remove obsolete artifacts before saving the cache, and SHALL not exceed the given cache size limit.

#### Scenario: Saving the cache
- **WHEN** CI finishes a job and saves the `target/` cache
- **THEN** obsolete artifacts are removed, and the cache size does not exceed the limit

### Requirement: Documented shared build directory
The documentation SHALL describe moving `CARGO_TARGET_DIR` to a shared directory as the recommended way for developers.

#### Scenario: Reading the documentation
- **WHEN** a developer opens the section on the build
- **THEN** they find the instruction for `CARGO_TARGET_DIR` for Windows and Linux
