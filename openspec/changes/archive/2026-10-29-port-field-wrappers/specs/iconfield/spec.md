## Purpose
IconField places an icon inside an input field on the left or on the right; InputIcon is the container of this icon.

## ADDED Requirements

### Requirement: Markup of IconField and InputIcon
The `IconField` component of the `prime-iconfield` crate SHALL render `<span class="p-iconfield p-iconfield-left">` (with `icon_position = Right` — `p-iconfield-right`; default `Left`) with the child content. The `InputIcon` component SHALL render `<span class="p-inputicon">` with the child content (the icon). Both SHALL accept `style`, `style_class` (added after the own classes) and `pt` with the part `root`, adding classes, style and attributes without replacing the own ones. The position of the icon inside the field SHALL be provided by the CSS of the theme `iconfield` according to the order of the child elements.

#### Scenario: Icon on the left
- **WHEN** `InputIcon` and a field are nested inside `IconField`
- **THEN** `span.p-iconfield.p-iconfield-left` is rendered, containing `span.p-inputicon` and the field in the original order

#### Scenario: Icon on the right
- **WHEN** `icon_position = Right`
- **THEN** the root has `p-iconfield-right` and does not have `p-iconfield-left`

#### Scenario: Custom icon
- **WHEN** an `<svg>` is nested inside `InputIcon`
- **THEN** the `<svg>` is rendered inside `span.p-inputicon` unchanged

#### Scenario: Classes and pt
- **WHEN** `InputIcon` has `style_class = "pi"` and `pt.root` with the class `custom`
- **THEN** the class is `p-inputicon pi custom`

### Requirement: Position of the icon in the theme
The theme CSS SHALL place the `InputIcon` that comes first in `IconField` at the left edge of the field, and the one that comes last — at the right edge; a field bordering the icon on the left or on the right SHALL get an inner padding for the icon.

#### Scenario: Position
- **WHEN** a page shows an `IconField` with the icon before the field
- **THEN** the icon lies inside the left bounds of the field, and the text of the field is shifted to the right of it

### Requirement: Rendering cost of IconField
The number of allocations per instance during SSR (`IconField` with an icon and a field) SHALL fit the budget `iconfield-1000` in `benches/alloc-budgets.toml`, and the wasm contribution — the budget `iconfield` in `benches/budgets.toml`.

#### Scenario: Budgets
- **WHEN** `cargo xtask alloc-budget` and `cargo xtask wasm-size` are run
- **THEN** the values of IconField do not exceed the budgets
