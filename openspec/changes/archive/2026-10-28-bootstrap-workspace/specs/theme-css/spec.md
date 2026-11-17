## ADDED Requirements

### Requirement: Static CSS from the Aura tokens
The system SHALL generate the theme's static CSS with a developer command from the Aura preset tokens and the component styles used by PrimeNG, without generating CSS at application runtime. The generated CSS SHALL be stored in the repository so that a regular build does not require Node or network access.

#### Scenario: Generation
- **WHEN** the theme generation command is run
- **THEN** CSS is created with variables and rules for the `p-*` classes of the ported components

#### Scenario: Regular build
- **WHEN** the application is built without Node and without network access
- **THEN** the build uses the committed CSS and succeeds

#### Scenario: Reproducibility
- **WHEN** generation is run twice on the same inputs
- **THEN** the result matches byte for byte

### Requirement: Compatibility with PrimeNG classes
The theme CSS SHALL use the same class and CSS variable (`--p-*`) names as PrimeNG so that the markup of the ported components works with it unchanged.

#### Scenario: Token variable
- **WHEN** PrimeNG has the token `primary.color`
- **THEN** the generated CSS has a variable with the same name and value

### Requirement: Light and dark scheme
The theme SHALL support light and dark color schemes, switched by a selector set in the configuration (like `darkModeSelector` in PrimeNG).

#### Scenario: Switching
- **WHEN** the dark mode selector is added to the root element
- **THEN** the dark scheme token values apply

### Requirement: Including only the needed CSS
An application SHALL be able to include the theme's base CSS separately from the CSS of individual components so as not to load styles of unused components.

#### Scenario: Application uses only Button
- **WHEN** the application includes the base CSS and the Button CSS
- **THEN** there are no styles of other components in the loaded files
