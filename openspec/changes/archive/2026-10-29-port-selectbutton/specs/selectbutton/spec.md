## Purpose
SelectButton selects one or several values from a list of options shown as buttons and reports changes of the selection.

## ADDED Requirements

### Requirement: Options and display
The `SelectButton` component of the `prime-selectbutton` crate SHALL render one `ToggleButton` button per option from `options` in the given order; the button label SHALL be the option's `label` in both states. Selected options SHALL be on (`aria-pressed="true"`), the rest off. An option with `disabled`, and also all options when the component is `disabled`, SHALL be unavailable. When `options` and the value change, the markup SHALL update.

#### Scenario: Buttons per option
- **WHEN** three options are passed
- **THEN** three buttons with their labels are rendered in the same order

#### Scenario: Selected option
- **WHEN** the value equals the value of the second option
- **THEN** the second button is on, the rest are off

#### Scenario: Unavailable option
- **WHEN** an option has `disabled = true`
- **THEN** its button has `p-disabled`, clicking it does not change the value

### Requirement: Single selection
Without `multiple` the component SHALL accept a two-way `value` (`RwSignal<Option<String>>`); without it SHALL keep internal state starting with `default_value`. A click on a button, as well as Enter and Space on it, SHALL select its value, deselecting the others, and call `on_change` once with the new value and the original event, and then `on_option_click` with the option, its index and the original event. A repeated click on the selected button SHALL reset the value to `None` unless `allow_empty` is `false`; with `allow_empty = false` the value SHALL remain, and `on_change` and `on_option_click` SHALL NOT be called.

#### Scenario: Selection
- **WHEN** the option "One-Way" is selected and the user clicks "Return"
- **THEN** the value becomes the value of "Return", "One-Way" turns off, `on_change` is called once

#### Scenario: Deselection
- **WHEN** the selected button is clicked again and `allow_empty` is not set
- **THEN** the value becomes `None`

#### Scenario: Cannot deselect
- **WHEN** `allow_empty = false` and the selected button is clicked again
- **THEN** the button stays on, `on_change` is not called

### Requirement: Multiple selection
With `multiple` the component SHALL accept a two-way `values` (`RwSignal<Vec<String>>`); without it SHALL keep internal state starting with `default_values`. A click SHALL append the option's value to the end of the list or remove it if it is already selected. With `allow_empty = false` the only selected value SHALL NOT be removable.

#### Scenario: Adding and removing
- **WHEN** `[1]` is selected and the user clicks options 2 and then 1
- **THEN** the values are successively `[1, 2]` and `[2]`

#### Scenario: Last value
- **WHEN** `allow_empty = false` and one value is selected, and it is clicked
- **THEN** the list does not change, `on_change` is not called

### Requirement: SelectButton structure and attributes
The root SHALL be `<div class="p-selectbutton p-component">` with the modifiers `p-invalid` and `p-selectbutton-fluid`, `role="group"`, `aria-labelledby` (only if set) and `data-p` equal to `invalid` when `invalid`. `size` and `fluid` SHALL be passed to each button; `style_class` SHALL be added to each button, not the root (as in PrimeNG). `item_template`, if set, SHALL replace the button content, receiving the option and its index. `pt` SHALL accept the parts `root` and `toggle_button` (passed to each nested button) and add classes, style and attributes without replacing the component's own.

#### Scenario: Root
- **WHEN** `invalid = true`, `fluid = true`, `aria_labelledby = "lbl"`
- **THEN** the root has `p-selectbutton p-component p-invalid p-selectbutton-fluid`, `role="group"`, `aria-labelledby="lbl"`, `data-p="invalid"`

#### Scenario: Size
- **WHEN** `size = Small` is set
- **THEN** each button has `p-togglebutton-sm`

#### Scenario: Button class
- **WHEN** `style_class = "mine"` is set
- **THEN** the class `mine` is on each button and not on the root

#### Scenario: Custom content
- **WHEN** `item_template` is set
- **THEN** the buttons contain the template's result, the default label is not rendered

### Requirement: SelectButton render cost
The allocation count per instance in SSR SHALL fit within the `selectbutton-1000` budget in `benches/alloc-budgets.toml`, the wasm contribution within the `selectbutton` budget in `benches/budgets.toml`.

#### Scenario: Budgets
- **WHEN** `cargo xtask alloc-budget` and `cargo xtask wasm-size` are run
- **THEN** the SelectButton values do not exceed the budgets
