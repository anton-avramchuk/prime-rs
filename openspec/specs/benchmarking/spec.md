# benchmarking Specification

## Purpose
Benchmarks of SSR rendering and allocations, wasm size budgets, regression control and browser memory measurements.

## Requirements

### Requirement: SSR rendering benchmarks
The system SHALL contain benchmarks that measure the render time of each ported component into an HTML string, on typical sets of props and on a large number of instances.

#### Scenario: Running a component benchmark
- **WHEN** the benchmark is run
- **THEN** the render time of one instance and of 1000 instances is printed

### Requirement: Allocation measurement
The system SHALL measure the number and the total size of allocations when a component is rendered, and print them in the benchmark report.

#### Scenario: Allocation report
- **WHEN** the benchmark finishes
- **THEN** the report contains the number of allocations and the peak memory per render

### Requirement: wasm size budget
For each component a budget of its contribution to the wasm binary size SHALL be set, and CI SHALL report when the budget is exceeded.

#### Scenario: Budget exceeded
- **WHEN** a PR increases a component's contribution above the budget
- **THEN** the CI check fails, stating the component, the size and the budget

### Requirement: Regression control
The system SHALL compare the benchmark results with the baseline of the main branch. A regression of more than 10 % in time or in allocations SHALL be reported with a comment in the PR; it SHALL block the merge only after blocking has been explicitly enabled.

#### Scenario: 15 % regression
- **WHEN** a PR slows the rendering of a component by 15 %
- **THEN** a comment with the result appears in the PR, and the merge is not blocked

### Requirement: Browser memory benchmarks
On a schedule, browser measurements of the time to create N components and of heap memory usage SHALL run, and the results SHALL be saved as artifacts.

#### Scenario: Weekly measurement
- **WHEN** the weekly job runs
- **THEN** a report on time and memory is produced for the scenarios "1000 components" and "creation and removal in a loop"

### Requirement: Building class and data-p without allocation
The common building of the `class` and `data-p` strings in `prime-rs-core` SHALL perform no heap allocations for strings composed only of component constants (without user `style_class` and `pt`): such strings SHALL be returned as `&'static str` from a bounded interning table. Strings with user classes SHALL be assembled in a stack buffer and returned as a single `String`.

#### Scenario: Repeated render of the same state
- **WHEN** the same component in the same state is rendered a second time
- **THEN** the `class` and `data-p` strings do not allocate

#### Scenario: User class
- **WHEN** `style_class` is passed
- **THEN** the resulting string equals the previous one in content and does not enter the interning table

### Requirement: Allocation budgets follow the measurements
The budgets in `benches/alloc-budgets.toml` SHALL not exceed the measured value rounded up by more than 15 % (but by at least 1 allocation of margin).

#### Scenario: Budget check
- **WHEN** `cargo xtask alloc-budget` is run
- **THEN** all scenarios fit the budgets, and the margin above the rounded measurement does not exceed 15 %

### Requirement: Static branch without client primitives
Components with a static branch (Button, Chip) SHALL NOT create `NodeRef`, `Arc<Props>` and `Arc<Pt>` during SSR when the properties are static: these are allowed only in the reactive branch and on the client.

#### Scenario: Static button during SSR
- **WHEN** `Button` with static properties is rendered on a native target
- **THEN** the number of allocations per instance fits the budget `button-1000` and does not include `NodeRef`

#### Scenario: Behaviour unchanged
- **WHEN** the button is `disabled` or in the loading state
- **THEN** `on_click` is still not called, and the browser tests of Button and Chip pass
