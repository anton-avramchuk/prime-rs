## ADDED Requirements

### Requirement: Class and data-p assembly without allocation
The shared assembly of `class` and `data-p` strings in `prime-core` SHALL NOT perform heap allocations for strings composed only of component constants (without user `style_class` and `pt`): such strings SHALL be returned as `&'static str` from a bounded interning table. Strings with user classes SHALL be assembled on a stack buffer and handed out as a single `String`.

#### Scenario: Re-render of the same state
- **WHEN** the same component in the same state is rendered a second time
- **THEN** the `class` and `data-p` strings do not allocate

#### Scenario: User class
- **WHEN** `style_class` is passed
- **THEN** the resulting string equals the previous one in content and does not enter the interning table

### Requirement: Allocation budgets follow measurements
The budgets in `benches/alloc-budgets.toml` SHALL NOT exceed the rounded-up measured value by more than 15 % (but by no less than 1 allocation of headroom).

#### Scenario: Budget check
- **WHEN** `cargo xtask alloc-budget` is run
- **THEN** all scenarios fit within the budgets, and the headroom over the rounded measurement does not exceed 15 %
