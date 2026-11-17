# slider Specification

## Purpose
Slider lets you choose a number or a range of two numbers between `min` and `max` by moving a handle with the mouse, touch or keyboard.

## Requirements

### Requirement: Value and range
The `Slider` component of the `prime-slider` crate SHALL accept a two-way `value` (`RwSignal<f64>`), and with `range` — `values` (`RwSignal<[f64; 2]>`); without a signal it SHALL keep internal state starting from `default_value` (default `0`) or `default_values` (default `[0, 0]`). The value SHALL be clamped to the `min..=max` interval (default `0..=100`). The handle position and the width or height of the filled part SHALL correspond to the value's fraction within the interval; in a range the part between the handles is filled regardless of their order, and the event and the written value SHALL contain the smaller value first.

#### Scenario: Position
- **WHEN** `min = 0`, `max = 200`, value `50`
- **THEN** the handle is at 25 % of the track, 25 % is filled

#### Scenario: Range
- **WHEN** `range`, values `[20, 80]`
- **THEN** two handles at 20 % and 80 %, the part between them is filled

#### Scenario: External change
- **WHEN** the value signal changes
- **THEN** the handle position and the fill are updated

### Requirement: Keyboard input
A handle SHALL accept focus (`tabindex`, default `0`; absent when `disabled`) and react to keys: right and up arrows increase the value, left and down decrease it by `step` or, without it, by 1; `PageUp` and `PageDown` change the value by `step` or, without it, by 10 (in a range by 1); `Home` and `End` set `min` and `max`. Each handled key SHALL cancel the browser's default action and call `on_change` once; the result SHALL be clamped to `min..=max`. In a range the key SHALL change the value of the focused handle.

#### Scenario: Arrow
- **WHEN** value `50` and the right arrow is pressed
- **THEN** value `51`, `on_change` called once

#### Scenario: Step
- **WHEN** `step = 20`, value `20` and the right arrow is pressed
- **THEN** value `40`

#### Scenario: Bounds
- **WHEN** `End` is pressed, then the right arrow
- **THEN** the value equals `max` and does not exceed it

### Requirement: Mouse and touch input
Pressing a handle and moving the pointer SHALL change the value by the pointer position along the track (with `orientation = Vertical` from bottom to top; with RTL from right to left) until the button is released, and the release SHALL call `on_slide_end`; the move and release handlers SHALL exist only for the duration of dragging. A click on the track SHALL move the handle that was last active (in a range; the first by default) to the indicated place and call `on_slide_end`; a click on a handle SHALL NOT count as a track click. Touching a handle SHALL move it by the fraction of the finger offset along the track in 1 % steps (PrimeNG's `onDrag`). With `disabled`, all these actions SHALL NOT change the value or fire events. With a given `step`, the resulting value SHALL be obtained, as in PrimeNG, from `handleStepChange`: a shift from the previous value by a whole number of steps toward the pointer; without `step` the value is rounded down to an integer or to the number of decimals as in `step`.

#### Scenario: Click on the track
- **WHEN** value `0` and a click in the middle of a horizontal track 200 pixels wide
- **THEN** value `50`, `on_change` and `on_slide_end` called

#### Scenario: Dragging
- **WHEN** a handle is pressed and the pointer is moved a quarter of the track to the right, then the button is released
- **THEN** the value grew by 25, `on_slide_end` called once, no listeners are left on the document

#### Scenario: Disabled
- **WHEN** `disabled` is set
- **THEN** click, dragging and keys do not change the value or fire events

### Requirement: Structure, classes and accessibility
The root SHALL be `<div class="p-slider p-component">` with the modifiers `p-disabled`, `p-invalid`, `p-slider-horizontal` or `p-slider-vertical`, `p-slider-animate` (with `animate`, except during dragging), `style_class`; attributes `data-pc-name="slider"`, `data-pc-section="root"`, `data-p` (orientation) and `data-p-sliding` (`true` during dragging). Inside SHALL come `span.p-slider-range` and `span.p-slider-handle` handles with `position: absolute`: one with `data-pc-section="handle"`, and in a range two — `startHandler` and `endHandler`, the active one with `p-slider-handle-active`. Each handle SHALL have `role="slider"`, `aria-valuemin`, `aria-valuenow`, `aria-valuemax`, `aria-orientation`, and `aria-label` and `aria-labelledby` — only if set.

#### Scenario: Root
- **WHEN** `orientation = Vertical`, `disabled`, `invalid`
- **THEN** the root has `p-slider p-component p-disabled p-invalid p-slider-vertical`, `data-p="vertical"`

#### Scenario: Handle
- **WHEN** value `30`, `min = 0`, `max = 100`
- **THEN** the handle has `role="slider"`, `aria-valuemin="0"`, `aria-valuenow="30"`, `aria-valuemax="100"`, `aria-orientation="horizontal"`

#### Scenario: Disabled handle
- **WHEN** `disabled` is set
- **THEN** the handle has no `tabindex`

### Requirement: Properties and pass-through
The properties `min`, `max`, `step`, `orientation`, `animate`, `range`, `disabled`, `invalid`, `tabindex`, `aria_label`, `aria_labelledby`, `style`, `style_class` SHALL accept a value or a signal and update the markup when the signal changes. `pt` SHALL accept the parts `root`, `range`, `handle`, `start_handler`, `end_handler`, adding classes, style and attributes without replacing the component's own.

#### Scenario: Reactive bounds
- **WHEN** `max` is passed as a signal and changes
- **THEN** the handle position is recomputed

#### Scenario: Class via pt
- **WHEN** the class `custom` is passed in `pt.handle`
- **THEN** the handle has `custom` and `p-slider-handle`

### Requirement: Slider rendering cost
The number of allocations per instance in SSR SHALL fit within the `slider-1000` budget in `benches/alloc-budgets.toml`, the wasm contribution — within the `slider` budget in `benches/budgets.toml`.

#### Scenario: Budgets
- **WHEN** `cargo xtask alloc-budget` and `cargo xtask wasm-size` are run
- **THEN** Slider values do not exceed the budgets
