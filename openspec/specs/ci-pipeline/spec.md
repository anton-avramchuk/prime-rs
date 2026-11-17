# ci-pipeline Specification

## Purpose
Checks on every PR and heavy weekly checks (e2e, screenshots, browser benchmarks) that do not block PRs.

## Requirements

### Requirement: Checks on every PR
CI on GitHub Actions SHALL run on every PR: a formatting check, `clippy` with warnings denied, unit tests, `wasm-bindgen-test` in a headless browser, `cargo-deny`, benchmarks per the `benchmarking` spec, and builds for SSR and wasm. Browser e2e tests SHALL NOT run on PRs by default.

#### Scenario: Regular PR
- **WHEN** a PR is opened without special labels
- **THEN** the listed checks run, and the e2e job does not start

#### Scenario: Formatting error
- **WHEN** a PR contains unformatted code
- **THEN** the formatting job fails and blocks the merge

### Requirement: Checking only the affected crates
CI SHALL determine the changed crates and the crates depending on them, and run the component checks only for them. When the shared core, the dependencies or the configuration change, CI SHALL check the whole workspace.

#### Scenario: One component changed
- **WHEN** a PR changes only `crates/prime-button`
- **THEN** the tests run for `prime-button` and the crates depending on it, and the others are skipped

#### Scenario: Core changed
- **WHEN** a PR changes `crates/prime-rs-core`
- **THEN** the whole workspace is checked

### Requirement: Weekly and manual checks
The e2e tests, the screenshot comparison with PrimeNG and the browser memory benchmarks SHALL run on a schedule once a week and manually through `workflow_dispatch`, and also on PRs labelled `run-e2e`.

#### Scenario: Schedule
- **WHEN** the weekly run time arrives
- **THEN** the e2e, the screenshot comparison and the browser benchmarks run

#### Scenario: Label on a PR
- **WHEN** the label `run-e2e` is added to a PR
- **THEN** the e2e job starts

### Requirement: Reference from PrimeNG without committing binaries
The screenshot comparison SHALL build the reference inside CI itself: clone PrimeNG at the pinned commit, build its showcase and take screenshots. Reference images SHALL NOT be committed to the repository. The comparison artifacts SHALL be kept for no more than 7 days.

#### Scenario: Running the comparison
- **WHEN** the weekly job runs
- **THEN** the reference is created anew from the pinned commit, and the result is published as an artifact kept for no more than 7 days

### Requirement: Comparison results do not block PRs
Differences in the screenshots and in the weekly checks SHALL be reported and SHALL NOT block the merge of a PR.

#### Scenario: Screenshot difference
- **WHEN** a screenshot differs from the reference
- **THEN** the job publishes a report, but the merge is not blocked
