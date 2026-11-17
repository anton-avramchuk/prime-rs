## ADDED Requirements

### Requirement: Divider properties
The `Divider` component of the `prime-divider` crate SHALL accept `layout` (`horizontal`, `vertical`; `horizontal` by default), `divider_type` (`solid`, `dashed`, `dotted`; `solid` by default), `align` (`left`, `center`, `right`, `top`, `bottom`), `style`, `style_class`, `children` and `pt`. All properties are optional; those that change at runtime SHALL accept a value or a signal.

#### Scenario: Default values
- **WHEN** `Divider` is rendered without properties
- **THEN** the root has `role="separator"`, `aria-orientation="horizontal"`, the classes `p-divider p-component p-divider-horizontal p-divider-solid p-divider-left`, and `data-p` equals `horizontal solid`

### Requirement: Divider root classes
The root SHALL have the classes `p-divider p-component`, `p-divider-<layout>`, `p-divider-<type>` and an alignment class: for horizontal `p-divider-left` (with `align` unset or `left`), `p-divider-center` (`center`), `p-divider-right` (`right`); for vertical `p-divider-top` (`top`), `p-divider-center` (unset or `center`), `p-divider-bottom` (`bottom`). For an invalid combination (for example `left` on a vertical one) there SHALL NOT be an alignment class.

#### Scenario: Horizontal centered
- **WHEN** `layout = horizontal`, `align = center`, `divider_type = dotted`
- **THEN** the classes `p-divider-horizontal p-divider-dotted p-divider-center`

#### Scenario: Vertical by default
- **WHEN** `layout = vertical`
- **THEN** the classes `p-divider-vertical p-divider-center`

#### Scenario: Invalid alignment
- **WHEN** `layout = vertical`, `align = left`
- **THEN** there is no `p-divider-left` class and no alignment class at all

### Requirement: Divider attributes and style
The root SHALL have `role="separator"` and `aria-orientation` equal to `layout`; `data-p` SHALL contain, space-separated, `align` (if set), `layout` and the type. The inline style SHALL contain `justify-content` (horizontal: `center` for center and for unset, `flex-start` for `left`, `flex-end` for `right`) or `align-items` (vertical: `center` for center and for unset, `flex-start` for `top`, `flex-end` for `bottom`); `style` and the style from `pt` are appended to it.

#### Scenario: Alignment style
- **WHEN** `layout = horizontal`, `align = right`
- **THEN** the root style contains `justify-content: flex-end`

#### Scenario: Custom style
- **WHEN** `style = "margin: 4px"` is set
- **THEN** the style contains both `margin: 4px` and the alignment style

### Requirement: Divider content
The content (`children`) SHALL be rendered inside `<div class="p-divider-content">`, which is always present, even without content.

#### Scenario: Content
- **WHEN** `Text` content is passed
- **THEN** inside the root there is `<div class="p-divider-content">Text</div>`

#### Scenario: No content
- **WHEN** there is no content
- **THEN** `p-divider-content` is present and empty

### Requirement: Pass-through Divider
`Divider` SHALL accept `pt` for the root and the content block; classes and attributes are added to its own, not replacing them.

#### Scenario: pt on the content
- **WHEN** a `pt` with the class `extra` is passed for the content
- **THEN** `p-divider-content` has both `p-divider-content` and `extra`

### Requirement: Divider rendering cost
`Divider` SHALL use the static rendering branch if all properties are static, and the reactive one otherwise; both branches SHALL produce identical markup. The number of allocations per instance in SSR SHALL fit the `divider` budget in `benches/alloc-budgets.toml`, and the wasm contribution the `divider` budget in `benches/budgets.toml`.

#### Scenario: Branch parity
- **WHEN** the same set of values is passed statically and via signals
- **THEN** the HTML matches

#### Scenario: Budgets
- **WHEN** `cargo xtask alloc-budget` and `cargo xtask wasm-size` are run
- **THEN** the Divider values do not exceed the budgets
