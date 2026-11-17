## ADDED Requirements

### Requirement: Card properties
The `Card` component of the `prime-card` crate SHALL accept `header` and `subheader` (title and subtitle text), the slots `header_template`, `title_template`, `subtitle_template`, `footer_template` (Leptos content), `style`, `style_class`, `children` (content) and `pt`. All properties are optional; those that change at runtime SHALL accept a value or a signal.

#### Scenario: Empty card
- **WHEN** `Card` is rendered without properties
- **THEN** there is a root `p-card p-component`, a `p-card-body` block and an empty `p-card-content`, and there are no `p-card-header`, `p-card-title`, `p-card-subtitle`, `p-card-footer` blocks

### Requirement: Card structure
The root SHALL be an element with the classes `p-card p-component`. Inside, in order: a `div.p-card-header` block if `header_template` is set; then always `div.p-card-body`, containing in order `div.p-card-title` (if `header` or `title_template` is set), `div.p-card-subtitle` (if `subheader` or `subtitle_template` is set), `div.p-card-content` (always, with `children`) and `div.p-card-footer` (if `footer_template` is set). `style_class` and `style` SHALL be added without losing the own classes.

#### Scenario: Simple card
- **WHEN** `header = "Simple Card"` and content are set
- **THEN** inside `p-card-body` there is a `p-card-title` with the text `Simple Card` and a `p-card-content` with the content

#### Scenario: Advanced card
- **WHEN** `header_template`, `title_template`, `subtitle_template`, `footer_template` and content are set
- **THEN** the block order is: `p-card-header`, then `p-card-body` with `p-card-title`, `p-card-subtitle`, `p-card-content`, `p-card-footer`

### Requirement: Card slot priority
If both `header` and `title_template` are set, only `title_template` SHALL be rendered; the same for `subheader` and `subtitle_template`.

#### Scenario: Slot instead of text
- **WHEN** `header = "Text"` and a `title_template` with content `Slot` are set
- **THEN** `p-card-title` contains only `Slot`

### Requirement: Pass-through Card
`Card` SHALL accept `pt` for the root, header, body, title, subtitle, content and footer; classes and attributes are added to the own ones, not replacing them.

#### Scenario: pt on the body
- **WHEN** `pt` with the class `extra` is passed for the body
- **THEN** `p-card-body` has both `p-card-body` and `extra`

### Requirement: Card rendering cost
`Card` SHALL use the static render branch if all properties are static, and the reactive one otherwise; both branches SHALL produce the same markup. The number of allocations per instance in SSR SHALL fit the `card` budget in `benches/alloc-budgets.toml`, the wasm contribution — the `card` budget in `benches/budgets.toml`.

#### Scenario: Branch parity
- **WHEN** the same set of values is passed statically and as signals
- **THEN** the HTML matches

#### Scenario: Budgets
- **WHEN** `cargo xtask alloc-budget` and `cargo xtask wasm-size` are run
- **THEN** Card values do not exceed the budgets
