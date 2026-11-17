## ADDED Requirements

### Requirement: Avatar properties
The `Avatar` component of the `prime-avatar` crate SHALL accept `label`, `icon` (Leptos content), `image` (URL), `size` (`normal`, `large`, `xlarge`; `normal` by default), `shape` (`square`, `circle`; `square` by default), `style`, `style_class`, `aria_label`, `aria_labelledby`, `children`, `pt` and the `on_image_error` event. All properties are optional; those that change at runtime SHALL accept a value or a signal.

#### Scenario: Label
- **WHEN** `Avatar` is rendered with `label = "P"`
- **THEN** it contains `<span class="p-avatar-label">P</span>`

#### Scenario: Default values
- **WHEN** `Avatar` is rendered without properties
- **THEN** the root has the classes `p-avatar p-component` and `data-p` equals `square normal`

### Requirement: Avatar classes and attributes
The root SHALL be an element with the classes `p-avatar p-component` and the modifiers: `p-avatar-image` when `image` is set, `p-avatar-circle` for `shape = circle`, `p-avatar-lg` for `size = large`, `p-avatar-xl` for `size = xlarge`. The `data-p` attribute SHALL contain, space-separated, the shape and size; `aria-label` and `aria-labelledby` SHALL be carried over to the root. `style_class` and `style` SHALL be added without losing the own classes.

#### Scenario: Shape and size
- **WHEN** `shape = circle` and `size = xlarge` are set
- **THEN** the root has the classes `p-avatar-circle` and `p-avatar-xl`, and `data-p` equals `circle xlarge`

#### Scenario: Image
- **WHEN** `image` is set
- **THEN** the root has `p-avatar-image`

#### Scenario: Accessibility
- **WHEN** `aria_label = "Photo"` and `aria_labelledby = "id1"` are set
- **THEN** the root has the attributes `aria-label="Photo"` and `aria-labelledby="id1"`

### Requirement: Avatar content
Content SHALL be rendered in the order: `children`, then the `span.p-avatar-label` label if set; otherwise the `span.p-avatar-icon` icon if set; otherwise the `<img>` image with `src`, `aria-label` and `data-p` if `image` is set. Each of these elements SHALL carry the same `data-p` as the root.

#### Scenario: Priority
- **WHEN** `label`, `icon` and `image` are set
- **THEN** only the label is rendered

#### Scenario: Icon instead of image
- **WHEN** `icon` and `image` are set but there is no `label`
- **THEN** `span.p-avatar-icon` is rendered, there is no image

### Requirement: Avatar image load error
When the `<img>` fails to load, `on_image_error` SHALL be called with the original event.

#### Scenario: Error
- **WHEN** the image failed to load
- **THEN** `on_image_error` is called once

### Requirement: Pass-through Avatar
`Avatar` SHALL accept `pt` for the root, label, icon and image; classes and attributes are added to its own, not replacing them.

#### Scenario: pt on the label
- **WHEN** a `pt` with the class `extra` is passed for the label
- **THEN** `p-avatar-label` has both `p-avatar-label` and `extra`

### Requirement: Avatar rendering cost
`Avatar` SHALL use the static render branch if all properties are static, and the reactive one otherwise; both branches SHALL produce the same markup. The number of allocations per instance in SSR SHALL fit the `avatar` budget in `benches/alloc-budgets.toml`, and the wasm contribution the `avatar` budget in `benches/budgets.toml`.

#### Scenario: Branch parity
- **WHEN** the same set of values is passed statically and as signals
- **THEN** the HTML matches

#### Scenario: Budgets
- **WHEN** `cargo xtask alloc-budget` and `cargo xtask wasm-size` are run
- **THEN** the Avatar values do not exceed the budgets
