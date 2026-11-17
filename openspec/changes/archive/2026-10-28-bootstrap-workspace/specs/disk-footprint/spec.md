## ADDED Requirements

### Requirement: Economical build profiles
Build profiles SHALL be configured so that the dev build does not contain full debug information and uses a fast linker, and incremental compilation is enabled only in dev.

#### Scenario: Dev profile
- **WHEN** `cargo build` is run in the dev profile
- **THEN** debug information is limited to line tables, and the incremental cache is created

#### Scenario: Release and CI
- **WHEN** a release build or a CI build is run
- **THEN** the incremental cache is not created

### Requirement: Cleanup command
The system SHALL provide a cleanup command that removes stale `target/` artifacts (unused for longer than a set period) and reports the freed size.

#### Scenario: Cleanup
- **WHEN** the cleanup command is run
- **THEN** stale artifacts are removed, fresh ones are kept, the freed size is printed

### Requirement: Dependency limits
External dependencies SHALL be included with `default-features = false` where possible, and CI SHALL check for the absence of unused and forbidden dependencies.

#### Scenario: Forbidden dependency
- **WHEN** a PR adds a dependency that violates the `cargo-deny` policy
- **THEN** the CI check fails

### Requirement: CI cache size control
CI SHALL remove stale artifacts before saving the cache and SHALL NOT exceed the set cache size limit.

#### Scenario: Saving the cache
- **WHEN** CI finishes a job and saves the `target/` cache
- **THEN** stale artifacts are removed, and the cache size does not exceed the limit

### Requirement: Documented shared build directory
The documentation SHALL describe moving `CARGO_TARGET_DIR` to a shared directory as the recommended approach for a developer.

#### Scenario: Reading the documentation
- **WHEN** a developer opens the build section
- **THEN** they find instructions for `CARGO_TARGET_DIR` for Windows and Linux
