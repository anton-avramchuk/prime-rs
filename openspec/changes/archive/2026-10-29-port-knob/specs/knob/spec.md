## Purpose
Knob lets the user enter a number from `min` to `max` by turning a circular scale with the mouse, touch or keys.

## ADDED Requirements

### Requirement: Value and scale
The `Knob` component of the `prime-knob` crate SHALL accept a two-way `value` (`RwSignal<f64>`); without it SHALL keep internal state starting with `default_value`, or with `min` if it is absent. The scale SHALL be an arc from the bottom left corner to the bottom right through the top (from -120 deg to +120 deg relative to the top, with a gap at the bottom): `path.p-knob-range` draws the whole arc, `path.p-knob-value` the arc from the scale zero (with `min > 0` from `min`) to the value. The position of the end of the value arc SHALL correspond to the share of the value in the range `min..max`; a value outside the range SHALL be displayed clamped to the nearest edge, but shown as is in the text and `aria-valuenow`. With `show_value`, `text.p-knob-text` SHALL be rendered inside with the value substituted into `value_template` in place of the first `{value}`.

#### Scenario: Middle of the scale
- **WHEN** `min = 0`, `max = 100`, value `50`
- **THEN** the end of the value arc lies on the vertical axis above the center

#### Scenario: Template
- **WHEN** `value_template = "{value}%"`, value `60`
- **THEN** the text is "60%"

#### Scenario: External change
- **WHEN** the value signal changes
- **THEN** the value arc, the text and `aria-valuenow` are updated

### Requirement: Keyboard input
The SVG SHALL accept focus (`tabindex`, `0` by default; `-1` with `readonly` and `disabled`) and react to keys: right and up arrows increase the value by 1, left and down decrease by 1, `PageUp` and `PageDown` change it by 10, `Home` and `End` set `min` and `max`; the result SHALL be clamped to the range `min..=max`. Every handled key SHALL cancel the browser default action and call `on_change` once. With `readonly` and `disabled` keys SHALL NOT change the value.

#### Scenario: Arrow
- **WHEN** the value is `50` and the right arrow is pressed
- **THEN** the value is `51`, `on_change` is called once with `51`

#### Scenario: Bounds
- **WHEN** `End` and then the up arrow are pressed
- **THEN** the value equals `max` and does not exceed it

#### Scenario: Read-only
- **WHEN** `readonly` is set and an arrow is pressed
- **THEN** the value and `on_change` are unaffected

### Requirement: Mouse and touch input
A click on the scale SHALL set the value from the angle of the click point relative to the SVG center; pressing a mouse button and moving the pointer SHALL change the value by angle until the button is released, and the move and release handlers SHALL exist only for the duration of the drag (for touch - move and touch end, with a single touch). Points in the gap at the bottom of the scale SHALL be ignored. The angle SHALL map to a value with rounding to the nearest `step` from `min` and clamping to `min..=max`. A press SHALL focus the SVG. With `readonly` and `disabled` the pointer SHALL NOT change the value or call `on_change`.

#### Scenario: Click
- **WHEN** a click on the top point of the scale with `0..100`
- **THEN** the value is about 50

#### Scenario: Gap
- **WHEN** a click on the bottom point of the scale, in the gap
- **THEN** the value does not change

#### Scenario: Dragging
- **WHEN** the scale is pressed, the pointer is moved along the arc, the button is released
- **THEN** the value follows the pointer, and after release pointer movement does not change the value

#### Scenario: Step
- **WHEN** `step = 10` and a click at the angle corresponding to the value 47
- **THEN** the value equals 50

### Requirement: Structure, classes and accessibility
The root SHALL be `<div class="p-knob p-component">` with `p-disabled` with `disabled` and `style_class`. The SVG SHALL have `viewBox="0 0 100 100"`, `role="slider"`, width and height of `size` pixels, `aria-valuemin`, `aria-valuemax`, `aria-valuenow`, `required` (only with `required`), `aria-label` and `aria-labelledby` (only when set). The arcs SHALL have `stroke-width` from `stroke_width` and `stroke` from `range_color` and `value_color`, the text `fill` from `text_color`, `x="50"`, `y="57"`, `text-anchor="middle"` and `name` (if set); default colors are the theme variables `--p-knob-range-background`, `--p-knob-value-background`, `--p-knob-text-color`.

#### Scenario: Root
- **WHEN** `disabled`
- **THEN** the root has `p-knob p-component p-disabled`, the SVG has `tabindex="-1"`

#### Scenario: Size and colors
- **WHEN** `size = 200`, `stroke_width = 5`, `value_color = "SlateGray"`
- **THEN** the SVG is 200 px wide and high, the arcs have `stroke-width="5"`, the value arc has `stroke="SlateGray"`

#### Scenario: Labels
- **WHEN** `aria_label = "Volume"`, `min = 0`, `max = 100`, value `30`
- **THEN** the SVG has `aria-label="Volume"`, `aria-valuemin="0"`, `aria-valuemax="100"`, `aria-valuenow="30"`

### Requirement: Properties and pass-through
The properties `min`, `max`, `step`, `size`, `stroke_width`, `show_value`, `value_template`, colors, `readonly`, `disabled`, `required`, `name`, `tabindex`, `aria_label`, `aria_labelledby`, `style`, `style_class` SHALL accept a value or a signal and update the markup when the signal changes. `pt` SHALL accept the parts `root`, `svg`, `range`, `value`, `text`, adding classes, style and attributes without replacing the component's own.

#### Scenario: Reactive bounds
- **WHEN** `max` is passed as a signal and changes
- **THEN** the value arc is recomputed

#### Scenario: Class via pt
- **WHEN** the class `custom` is passed in `pt.text`
- **THEN** the text has `custom` and `p-knob-text`

### Requirement: Knob rendering cost
The number of allocations per instance in SSR SHALL fit the `knob-1000` budget in `benches/alloc-budgets.toml`, and the wasm contribution the `knob` budget in `benches/budgets.toml`.

#### Scenario: Budgets
- **WHEN** `cargo xtask alloc-budget` and `cargo xtask wasm-size` are run
- **THEN** the Knob values do not exceed the budgets
