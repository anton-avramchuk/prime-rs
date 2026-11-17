# docs-site Specification

## Purpose
TBD - created by archiving change showcase-docs-site. Update Purpose after archiving.

## Requirements

### Requirement: Component page
A component page SHALL contain a title and a description, an "Import" block with the `use` line of the crate, a list of examples (each — an anchor heading, a live demo and a collapsible "Code" block with the demo source), the sections "Props", "Events", "Pass Through", "Theming" (CSS classes), "Accessibility" and "Notes", and a table of contents of the page with links to the anchors and navigation to the previous and next component. The identifiers `h3#<name>` and the `data-demo` attribute of the demos SHALL be preserved.

#### Scenario: Example code
- **WHEN** the "Code" button of the "Basic" example is pressed on the page `/autocomplete`
- **THEN** the source of the `Basic` demo expands, matching the code in `showcase/src/pages/autocomplete.rs`

#### Scenario: Table of contents
- **WHEN** the "Dropdown" link in the table of contents of the page is pressed
- **THEN** the page scrolls to `#dropdown`

### Requirement: Menu
The left menu SHALL group the components by section, highlight the current page, and be filtered by a substring of the name, case-insensitively.

#### Scenario: Search
- **WHEN** "tree" is typed in the menu field
- **THEN** only Tree, TreeSelect, TreeTable are visible

### Requirement: Generation
`cargo xtask gen-docs` SHALL create `showcase/src/generated/*` from the sources of the demos, the `#[component]` properties and `docs/*.md`, deterministically; `--check` SHALL fail if the files are out of date. The properties table SHALL contain the name, the type, the default value (if any) and the `///` description.

#### Scenario: Up to date
- **WHEN** a demo is changed and the generation is not run
- **THEN** `gen-docs --check` fails and names the outdated file
