## ADDED Requirements

### Requirement: PrimeNG icons in one crate
The system SHALL provide a `prime-icons` crate containing the 53 SVG icons from `packages/primeng/src/icons` (without `baseicon` and `blank`) with the same names (for example, `CheckIcon`, `ChevronDownIcon`) and the same outline data and `viewBox` as in PrimeNG.

#### Scenario: Match with PrimeNG
- **WHEN** the `check` icon is rendered
- **THEN** its SVG contains the same `d` and `viewBox` as the PrimeNG source

### Requirement: Including icons via feature flags
Each icon SHALL be available only when its own feature flag is enabled, so that unused icons do not end up in the build.

#### Scenario: Icon not enabled
- **WHEN** the icon's feature is not enabled
- **THEN** its type is unavailable, and the wasm size does not include its data

### Requirement: Shared icon behavior
All icons SHALL support size, spin mode (`spin`), a custom class and a11y attributes: by default `aria-hidden="true"`, and with a label set `role="img"` and `aria-label`.

#### Scenario: Decorative icon
- **WHEN** an icon is rendered without a label
- **THEN** the `svg` has `aria-hidden="true"` and no `role`

#### Scenario: Icon with a label
- **WHEN** `aria-label` is set
- **THEN** the `svg` has `role="img"` and `aria-label`, and `aria-hidden` is absent

### Requirement: Reproducible generation
The icon code SHALL be produced by a generation command from the PrimeNG sources, and a rerun without changes in the sources SHALL NOT change the files.

#### Scenario: Repeated generation
- **WHEN** the generation command is run a second time without changes
- **THEN** `git diff` is empty
