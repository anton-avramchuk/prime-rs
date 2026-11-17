# theme-css Specification

## Purpose
Static CSS of the Aura theme, produced from PrimeNG tokens and styles and attached without runtime generation.

## Requirements

### Requirement: Static CSS from Aura tokens
The system SHALL generate the static theme CSS via a developer command from the Aura preset tokens and the component styles used by PrimeNG, without generating CSS at application runtime. The generated CSS SHALL be stored in the repository so that a regular build does not require Node or network access.

#### Scenario: Generation
- **WHEN** the theme generation command is run
- **THEN** CSS is created with variables and rules for the `p-*` classes of the ported components

#### Scenario: Regular build
- **WHEN** the application is built without Node and without network access
- **THEN** the build uses the committed CSS and succeeds

#### Scenario: Reproducibility
- **WHEN** generation is run twice on the same inputs
- **THEN** the result is byte-identical

### Requirement: Compatibility with PrimeNG classes
The theme CSS SHALL use the same class and CSS variable (`--p-*`) names as PrimeNG so that the markup of the ported components works with it unchanged.

#### Scenario: Token variable
- **WHEN** PrimeNG has a `primary.color` token
- **THEN** the generated CSS has a variable with the same name and value

### Requirement: Light and dark scheme
The theme SHALL support light and dark color schemes switched by a selector set in the configuration (like `darkModeSelector` in PrimeNG).

#### Scenario: Switching
- **WHEN** the dark mode selector is added to the root element
- **THEN** the dark scheme token values are applied

### Requirement: Attaching only the needed CSS
An application SHALL be able to attach the base theme CSS separately from the CSS of specific components so as not to load styles of unused components.

#### Scenario: Application uses only Button
- **WHEN** the application attaches the base CSS and the Button CSS
- **THEN** there are no styles of other components in the loaded files
