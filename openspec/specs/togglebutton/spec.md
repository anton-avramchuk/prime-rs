# togglebutton Specification

## Purpose
TBD - created by archiving change port-togglebutton. Update Purpose after archive.

## Requirements

### Requirement: ToggleButton state and events
The `ToggleButton` component of the `prime-togglebutton` crate SHALL accept a two-way `checked` (`RwSignal<bool>`); without it SHALL keep internal state starting from `default_checked`. A click on the root and the Enter and Space keys SHALL toggle the state, call `on_change` with the new value and the original event exactly once, and cancel the default browser action for keys. With `disabled` the state and `on_change` SHALL NOT change. With `allow_empty = false` a checked button SHALL NOT be possible to switch off. `on_focus` and `on_blur` SHALL be called on focus gain and loss.

#### Scenario: Click
- **WHEN** the user clicks an unchecked button
- **THEN** the signal becomes `true` and `on_change` is called once with the value `true`

#### Scenario: Keys
- **WHEN** focus is on the button and Enter or Space is pressed
- **THEN** the state toggles, the page does not scroll on Space

#### Scenario: Disabled
- **WHEN** `disabled` is set and the user clicks
- **THEN** the signal and `on_change` are unaffected

#### Scenario: Cannot deselect
- **WHEN** `allow_empty = false`, the button is checked and the user clicks
- **THEN** the button stays checked, `on_change` is not called

### Requirement: ToggleButton structure and classes
The root SHALL be `<div class="p-togglebutton p-component">` with the modifiers `p-togglebutton-checked`, `p-invalid`, `p-disabled`, `p-togglebutton-sm p-inputfield-sm` (`Size::Small`), `p-togglebutton-lg p-inputfield-lg` (`Size::Large`), `p-togglebutton-fluid`. Inside SHALL be `span.p-togglebutton-content`; without `content_template` it SHALL contain an icon (only when `on_icon` or `off_icon` is set) in `span.p-togglebutton-icon` with `p-togglebutton-icon-left` or `p-togglebutton-icon-right` per `icon_pos`, and `span.p-togglebutton-label` with `on_label` when checked and `off_label` when unchecked. An empty label SHALL be displayed as a single space.

#### Scenario: Checked
- **WHEN** `checked = true`, `on_label = "On"`
- **THEN** the root has `p-togglebutton-checked`, the label is "On"

#### Scenario: Default labels
- **WHEN** no labels are set
- **THEN** the unchecked button shows "No", the checked one "Yes"

#### Scenario: Icon and position
- **WHEN** `on_icon` is set and `icon_pos = Right`
- **THEN** the icon in the checked state is in `span.p-togglebutton-icon.p-togglebutton-icon-right`

#### Scenario: Custom content
- **WHEN** `content_template` is set
- **THEN** its content receives the current `checked`, and the default icon and label are not rendered

### Requirement: ToggleButton attributes
The root SHALL have `role="button"`, `aria-pressed="true|false"`, `tabindex` (`0` by default, `-1` with `disabled` if `tabindex` is not set explicitly), `aria-label`, `aria-labelledby` (only those set), `data-pc-name="togglebutton"`, `data-p-checked`, `data-p-disabled` and `data-p`, which holds, space-separated, `checked`, `invalid` and the size (`small` or `large`).

#### Scenario: Accessibility
- **WHEN** `aria_label = "Confirm"` is set and the button is checked
- **THEN** the root has `aria-pressed="true"`, `aria-label="Confirm"`, `role="button"`, `data-p="checked"`

#### Scenario: Disabled
- **WHEN** `disabled` is set
- **THEN** `tabindex="-1"`, `data-p-disabled="true"`, class `p-disabled`

### Requirement: ToggleButton properties and pass-through
The properties `on_label`, `off_label`, `on_icon`, `off_icon`, `icon_pos`, `size`, `disabled`, `invalid`, `allow_empty`, `fluid`, `tabindex`, `aria_label`, `aria_labelledby`, `style`, `style_class` SHALL accept a value or a signal and update the markup when the signal changes. `pt` SHALL accept the parts `root`, `content`, `icon`, `label`, adding classes, style and attributes without replacing its own.

#### Scenario: Reactive label
- **WHEN** `on_label` is passed as a signal and changes on a checked button
- **THEN** the label text updates

#### Scenario: Class via pt
- **WHEN** the class `custom` is passed in `pt.label`
- **THEN** the label has `custom` and `p-togglebutton-label`

### Requirement: ToggleButton rendering cost
The number of allocations per instance in SSR SHALL fit the `togglebutton-1000` budget in `benches/alloc-budgets.toml`, and the wasm contribution the `togglebutton` budget in `benches/budgets.toml`.

#### Scenario: Budgets
- **WHEN** `cargo xtask alloc-budget` and `cargo xtask wasm-size` are run
- **THEN** the ToggleButton values do not exceed the budgets
