# chip Specification

## Purpose
The Chip component (port of PrimeNG `p-chip`): a label, an icon or an image, removal by mouse and keyboard, events, a localizable label, pass-through.

## Requirements

### Requirement: Chip properties
The `Chip` component of the `prime-chip` crate SHALL accept `label`, `icon` (Leptos content), `image` (URL), `alt`, `disabled`, `removable`, `remove_icon` (Leptos content), `style`, `style_class`, `children`, `pt` and the events `on_remove`, `on_image_error`. All properties are optional; those that can change at runtime SHALL accept a value or a signal.

#### Scenario: Label
- **WHEN** `Chip` is rendered with `label = "Amy Elsner"`
- **THEN** it contains `<div class="p-chip-label">Amy Elsner</div>`

#### Scenario: Without a label
- **WHEN** `label` is not set
- **THEN** there is no `p-chip-label` element

### Requirement: Classes and attributes of Chip
The root SHALL be an element with the classes `p-chip p-component` and `p-disabled` when `disabled`; the `data-p` attribute SHALL equal `removable` when `removable` and be empty otherwise. `style_class` and `style` SHALL be added without losing the own classes.

#### Scenario: Disabled and removable
- **WHEN** `disabled` and `removable` are set
- **THEN** the root has `p-disabled`, and `data-p` equals `removable`

### Requirement: Content of Chip
The content SHALL be rendered in this order: `children`, then the image `<img class="p-chip-image">` with `src` and `alt` if `image` is set, otherwise the icon in `<span class="p-chip-icon">` if it is set, then the label, then the remove element if `removable`.

#### Scenario: Image instead of icon
- **WHEN** `image` and `icon` are set
- **THEN** `img.p-chip-image` is rendered, and `p-chip-icon` is absent

#### Scenario: Icon
- **WHEN** only `icon` is set
- **THEN** `span.p-chip-icon` is rendered

### Requirement: Removing a Chip
With `removable` the element `span.p-chip-remove-icon` SHALL be rendered with `role="button"`, `tabindex="0"` (`-1` when `disabled`) and an `aria-label` from `PrimeConfig.locale` (`aria.removeLabel`, default `Remove`); inside it — `remove_icon` or `TimesCircleIcon`. A click on it, as well as the Enter and Backspace keys, SHALL hide the Chip (`display: none`) and call `on_remove` exactly once.

#### Scenario: Click
- **WHEN** the user clicks the remove element
- **THEN** the Chip is hidden (`display: none`) and `on_remove` is called once

#### Scenario: Keyboard
- **WHEN** the focus is on the remove element and Enter or Backspace is pressed
- **THEN** the Chip is hidden and `on_remove` is called once

#### Scenario: Label from the locale
- **WHEN** `aria.removeLabel = "Delete"` is set in `PrimeConfig.locale`
- **THEN** the remove element has `aria-label="Delete"`

#### Scenario: Not removable
- **WHEN** `removable` is not set
- **THEN** there is no remove element

### Requirement: Image loading error
On an image loading error `img.p-chip-image` SHALL call `on_image_error` with the original event.

#### Scenario: Error
- **WHEN** the image fails to load
- **THEN** `on_image_error` is called once

### Requirement: Pass-through of Chip
`Chip` SHALL accept `pt` for the root, the image, the icon, the label and the remove element; the classes and attributes are added to the own ones, without replacing them.

#### Scenario: pt on the label
- **WHEN** `pt` with the class `extra` is passed for the label
- **THEN** `p-chip-label` has both `p-chip-label` and `extra`

### Requirement: Rendering cost of Chip
`Chip` SHALL use the static render branch if all properties are static and it is not removable, and the reactive branch otherwise; both branches SHALL produce the same markup. The number of allocations per instance during SSR SHALL fit the budget `chip` in `benches/alloc-budgets.toml`, and the wasm contribution — the budget `chip` in `benches/budgets.toml`.

#### Scenario: Branch parity
- **WHEN** the same set of values is passed statically and through signals
- **THEN** the HTML matches

#### Scenario: Budgets
- **WHEN** `cargo xtask alloc-budget` and `cargo xtask wasm-size` are run
- **THEN** the values of Chip do not exceed the budgets
