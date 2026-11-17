# icon-set Specification

## Purpose
PrimeNG SVG icon set in the prime-icons crate: feature-flag inclusion, accessibility, reproducible generation.

## Requirements

### Requirement: PrimeNG icons in one crate
The system SHALL provide the `prime-icons` crate containing 53 SVG icons from `packages/primeng/src/icons` (without `baseicon` and `blank`) with the same names (for example `CheckIcon`, `ChevronDownIcon`), path data and `viewBox` as in PrimeNG.

#### Scenario: Match with PrimeNG
- **WHEN** the `check` icon is rendered
- **THEN** its SVG contains the same `d` and `viewBox` as the PrimeNG source

### Requirement: Including icons by feature flags
Each icon SHALL be available only when its own feature flag is enabled, so that unused icons do not end up in the build.

#### Scenario: Icon not enabled
- **WHEN** the icon's feature is not enabled
- **THEN** its type is unavailable, and the wasm size does not include its data

### Requirement: Common icon behaviour
All icons SHALL support a size, a spin mode (`spin`), a custom class and a11y attributes: `aria-hidden="true"` by default, `role="img"` and `aria-label` when a label is set.

#### Scenario: Decorative icon
- **WHEN** the icon is rendered without a label
- **THEN** the `svg` has `aria-hidden="true"` and no `role`

#### Scenario: Icon with a label
- **WHEN** `aria-label` is set
- **THEN** the `svg` has `role="img"` and `aria-label`, and `aria-hidden` is absent

### Requirement: Reproducible generation
The icon code SHALL be produced by a generation command from the PrimeNG sources, and a rerun with no changes in the sources SHALL NOT change the files.

#### Scenario: Repeated generation
- **WHEN** the generation command is run a second time with no changes
- **THEN** `git diff` is empty
