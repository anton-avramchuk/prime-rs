## ADDED Requirements

### Requirement: Component page
The component page SHALL contain a title and a description, an "Import" block with the `use` line of the crate, a list of examples (each — an anchor heading, a live demo and a collapsible "Code" block with the source of the demo), the sections "Props", "Events", "Pass Through", "Theming" (CSS classes), "Accessibility" and "Notes", as well as a table of contents of the page with links to the anchors and navigation to the previous and next component. The identifiers `h3#<name>` and the `data-demo` attribute of the demos SHALL be kept.

#### Scenario: Example code
- **WHEN** on the page `/autocomplete` the "Code" button of the example "Basic" is pressed
- **THEN** the source of the demo `Basic` expands, matching the code in `showcase/src/pages/autocomplete.rs`

#### Scenario: Table of contents
- **WHEN** the "Dropdown" link in the table of contents of the page is clicked
- **THEN** the page scrolls to `#dropdown`

### Requirement: Menu
The left menu SHALL group the components by sections, highlight the current page and filter by a substring of the name, case-insensitively.

#### Scenario: Search
- **WHEN** "tree" is entered in the menu field
- **THEN** only Tree, TreeSelect, TreeTable are visible

### Requirement: Generation
`cargo xtask gen-docs` SHALL create `showcase/src/generated/*` from the sources of the demos, the properties of `#[component]` and `docs/*.md` deterministically; `--check` SHALL fail if the files are outdated. The properties table SHALL contain the name, the type, the default value (if any) and the `///` description.

#### Scenario: Freshness
- **WHEN** a demo is changed and the generation is not run
- **THEN** `gen-docs --check` fails and names the outdated file
