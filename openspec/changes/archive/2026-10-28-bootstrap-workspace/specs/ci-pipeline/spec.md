## ADDED Requirements

### Requirement: Checks on every PR
CI on GitHub Actions SHALL on every PR run: a formatting check, `clippy` with warnings denied, unit tests, `wasm-bindgen-test` in a headless browser, `cargo-deny`, the benchmarks per the `benchmarking` spec, and builds for SSR and wasm. Browser e2e tests SHALL NOT run on PRs by default.

#### Scenario: Regular PR
- **WHEN** a PR without special labels is opened
- **THEN** the listed checks run, and the e2e job does not start

#### Scenario: Formatting error
- **WHEN** a PR contains unformatted code
- **THEN** the formatting job fails and blocks the merge

### Requirement: Checking only affected crates
CI SHALL determine the changed crates and the crates that depend on them and run component checks only for them. When the shared core, dependencies or configuration change, CI SHALL check the whole workspace.

#### Scenario: One component changed
- **WHEN** a PR changes only `crates/prime-button`
- **THEN** tests run for `prime-button` and the crates depending on it, the rest are skipped

#### Scenario: Core changed
- **WHEN** a PR changes `crates/prime-core`
- **THEN** the whole workspace is checked

### Requirement: Weekly and manual checks
E2E tests, screenshot comparison with PrimeNG and browser memory benchmarks SHALL run on a schedule once a week and manually via `workflow_dispatch`, and also on PRs with the `run-e2e` label.

#### Scenario: Schedule
- **WHEN** the weekly run time comes
- **THEN** e2e, screenshot comparison and browser benchmarks run

#### Scenario: Label on a PR
- **WHEN** the `run-e2e` label is added to a PR
- **THEN** the e2e job starts

### Requirement: PrimeNG baseline without committing binaries
Screenshot comparison SHALL build the baseline in CI itself: clone PrimeNG at the pinned commit, build its showcase and take screenshots. Baseline images SHALL NOT be committed to the repository. Comparison artifacts SHALL be kept for at most 7 days.

#### Scenario: Running the comparison
- **WHEN** the weekly job runs
- **THEN** the baseline is created anew from the pinned commit, and the result is published as an artifact with a retention of at most 7 days

### Requirement: Comparison results do not block PRs
Discrepancies in screenshots and in the weekly checks SHALL be reported in a report and SHALL NOT block PR merges.

#### Scenario: Screenshot discrepancy
- **WHEN** a screenshot differs from the baseline
- **THEN** the job publishes a report, but the merge is not blocked
