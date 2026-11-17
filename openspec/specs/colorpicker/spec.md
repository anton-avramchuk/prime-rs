# colorpicker Specification

## Purpose
ColorPicker — color selection on a panel with a saturation/brightness field and a hue scale.

## Requirements

### Requirement: Color conversions
The module SHALL convert between HSB, RGB and HEX using the PrimeNG formulas: `h` 0–360, `s`, `b` 0–100, `r`, `g`, `b` 0–255 (rounded to an integer); HEX shorter than six digits SHALL be left-padded with zeros; an unparsable string SHALL yield black.

#### Scenario: HSB to HEX
- **WHEN** HSB `{h: 0, s: 100, b: 100}`
- **THEN** HEX `ff0000`

#### Scenario: HEX to HSB
- **WHEN** HEX `#00ff00`
- **THEN** HSB `{h: 120, s: 100, b: 100}`

#### Scenario: Clamping
- **WHEN** HSB `{h: 400, s: -5, b: 120}`
- **THEN** the clamping result is `{h: 360, s: 0, b: 100}`

### Requirement: Markup
`ColorPicker` SHALL render `div.p-colorpicker.p-component` (`p-colorpicker-overlay` without `inline`) with `input.p-colorpicker-preview` (`readonly`, `type=text`, `tabindex`, `id`, `aria-label`, `disabled` and `p-disabled` when disabled; background color — the current color) and a panel `div.p-colorpicker-panel` (`p-colorpicker-panel-inline` with `inline`, `p-disabled` when disabled) > `.p-colorpicker-content` > `.p-colorpicker-color-selector` (> `.p-colorpicker-color-background` > `.p-colorpicker-color-handle`) and `.p-colorpicker-hue` (> `.p-colorpicker-hue-handle`). With `inline` there is no swatch and the panel is always visible; otherwise the panel is rendered only when open.

#### Scenario: Closed
- **WHEN** `inline = false`, the panel is closed
- **THEN** the DOM has the swatch and no panel

#### Scenario: Embedded
- **WHEN** `inline = true`
- **THEN** there is no swatch, the panel is present with `p-colorpicker-panel-inline`

### Requirement: Value and format
The value SHALL be a `ColorValue` by `format`: `hex` — the string `#rrggbb`, `rgb` — `{r, g, b}`, `hsb` — `{h, s, b}`. An empty value SHALL show `default_color` (`ff0000` by default). An external change of `value` SHALL update the swatch and the pointer positions; the pointers are at `left = floor(150·s/100)` px, `top = floor(150·(100−b)/100)` px (field) and `top = floor(150 − 150·h/360)` px (scale); the field background is the color with `s = 100, b = 100` of the current hue.

#### Scenario: rgb format
- **WHEN** `format = rgb`, the user picks pure red
- **THEN** the value is `Rgb {r: 255, g: 0, b: 0}`

#### Scenario: Pointer positions
- **WHEN** the value is HSB `{h: 0, s: 100, b: 100}`
- **THEN** the field pointer is at `left: 150px; top: 0px`, the scale pointer at `top: 0px`

### Requirement: Opening and closing
Clicking the swatch and the `Space` key SHALL toggle the panel (`Space` with `preventDefault`); `Escape` and `Tab` on the swatch SHALL close it; a click outside the root and a window resize SHALL close it; `ColorPickerRef::show`/`hide` — programmatically. `on_show` and `on_hide` SHALL be called on an actual state change; with `disabled` the swatch does not open the panel.

#### Scenario: Space
- **WHEN** focus is on the swatch, `Space` is pressed
- **THEN** the panel is open; pressing again closes it

#### Scenario: Outside click
- **WHEN** the panel is open, a click outside the root
- **THEN** the panel is closed, `on_hide` is called

### Requirement: Color selection
`mousedown` and `touchstart` on the field SHALL set `s = floor(100·clamp(x, 0, 150)/150)` and `b = floor(100·(150 − clamp(y, 0, 150))/150)`; on the scale — `h = floor(360·(150 − clamp(y, 0, 150))/150)` (x, y — from the top-left corner of the area). A gesture SHALL continue as the mouse moves over the window until `mouseup` (for touch — `touchmove`/`touchend` of the area, with `preventDefault`). Every change SHALL update the value and call `on_change`; during a gesture the root SHALL have `p-colorpicker-dragging`. With `disabled` gestures are ignored.

#### Scenario: Click on the field
- **WHEN** a click in the top-right corner of the field
- **THEN** `s = 100`, `b = 100`

#### Scenario: Dragging
- **WHEN** `mousedown` on the scale and mouse movement below its lower bound
- **THEN** the hue is clamped to `0`; after `mouseup` movement does not change the value

#### Scenario: Disabled
- **WHEN** `disabled`, `mousedown` on the field
- **THEN** the value and `on_change` are unaffected
