## ADDED Requirements

### Requirement: SSR render benchmarks
The system SHALL contain benchmarks that measure the time to render each ported component to an HTML string on typical prop sets and on a large number of instances.

#### Scenario: Running a component benchmark
- **WHEN** a benchmark is run
- **THEN** the render time of one instance and of 1000 instances is printed

### Requirement: Allocation measurement
The system SHALL measure the count and total size of allocations when rendering a component and print them in the benchmark report.

#### Scenario: Allocation report
- **WHEN** a benchmark finishes
- **THEN** the report contains the allocation count and the peak memory per render

### Requirement: wasm size budget
A budget for the contribution to the wasm binary size SHALL be set for each component, and CI SHALL report budget overruns.

#### Scenario: Budget exceeded
- **WHEN** a PR raises a component's contribution above the budget
- **THEN** the CI check fails, naming the component, the size and the budget

### Requirement: Regression control
The system SHALL compare benchmark results with the baseline of the main branch. A regression of more than 10 % in time or allocations SHALL be reported as a PR comment; it SHALL block the merge only after blocking is explicitly enabled.

#### Scenario: 15 % regression
- **WHEN** a PR slows a component's render by 15 %
- **THEN** a comment with the result appears on the PR, the merge is not blocked

### Requirement: Browser memory benchmarks
On a schedule, browser measurements of the time to create N components and of heap memory usage SHALL be run, and the results saved as artifacts.

#### Scenario: Weekly measurement
- **WHEN** the weekly job runs
- **THEN** a report of time and memory is produced for the "1000 components" and "create and remove in a loop" scenarios
