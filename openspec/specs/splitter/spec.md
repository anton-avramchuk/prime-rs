# splitter Specification

## Purpose
Splitter divides an area into panels whose sizes are changed by gutters.

## Requirements

### Requirement: Splitter markup
The `Splitter` component of the `prime-splitter` crate SHALL render a root `div.p-splitter.p-component.p-splitter-<layout>` with the style `display: flex; flex-wrap: nowrap` (and `flex-direction: column` with `Vertical`) and `data-p` with the layout (and `nested` with `nested`). For each panel in `panels` a `div.p-splitterpanel` SHALL be rendered (with `p-splitterpanel-nested` with `nested`, `tabindex="-1"`, `data-pc-section="panel"`), then (except after the last panel) `div.p-splitter-gutter[role=separator][tabindex="-1"][data-p-gutter-resizing="false"]` with a nested `div.p-splitter-gutter-handle[tabindex="0"]`, whose `aria-orientation` equals the layout, `aria-valuenow` the size of the previous panel with four decimals, and whose size along the layout axis is `gutter_size` pixels. `style`, `style_class`, `panel_style`, `panel_style_class`, `pt` SHALL apply to the root and panels.

#### Scenario: Two panels
- **WHEN** a `Splitter` with two panels is rendered
- **THEN** there are two `.p-splitterpanel` and one `.p-splitter-gutter` between them, the handle has `tabindex="0"`

#### Scenario: Vertical layout
- **WHEN** `layout = Vertical`
- **THEN** the root has `p-splitter-vertical` and `flex-direction: column`, the handle has `aria-orientation="vertical"` and a height of `gutter_size`

### Requirement: Initial sizes
The size of each panel SHALL be `panel_sizes[i]`, and if there is no value or it is zero, `100 / n` percent. A panel SHALL get `flex-basis: calc(<size>% - <(n-1)*gutter_size>px)`.

#### Scenario: Equal
- **WHEN** `panel_sizes` is not set and there are two panels
- **THEN** both have `flex-basis: calc(50% - 4px)`

#### Scenario: Given
- **WHEN** `panel_sizes = [20, 80]`
- **THEN** the first panel is `calc(20% - 4px)`, the second `calc(80% - 4px)`

### Requirement: Dragging
Pressing the mouse on a gutter SHALL start a resize, call `on_resize_start` with the current sizes and add `p-splitter-resizing` to the root and `p-splitter-gutter-resizing` and `data-p-gutter-resizing="true"` to the gutter. Mouse movement over the document SHALL transfer a share of a percent between the previous and next panels in proportion to the offset relative to the root size along the layout axis (in `rtl` the horizontal offset is inverted). Release SHALL finish the resize, call `on_resize_end` and remove the classes and subscriptions. Touch SHALL work the same way.

#### Scenario: Offset
- **WHEN** the root is 400 px wide, two panels of 50 %, the mouse is moved 40 px to the right
- **THEN** the sizes are 60 % and 40 %

#### Scenario: Finish
- **WHEN** the mouse is released
- **THEN** `on_resize_end` is called with the final sizes, the resize classes are removed

### Requirement: Minimum sizes
A change SHALL be rejected entirely if the new size of the previous panel is below `min_sizes[i]` or of the next one below `min_sizes[i+1]` (a zero or missing value does not constrain). `aria-valuenow` SHALL be updated even on a rejected change.

#### Scenario: Constraint
- **WHEN** `min_sizes = [10, 0]` and dragging makes the first panel 5 %
- **THEN** the sizes do not change

### Requirement: Keyboard
`keydown` on the handle SHALL shift the boundary by `step` pixels: for `Horizontal` `ArrowRight` enlarges the previous panel and `ArrowLeft` shrinks it; for `Vertical` `ArrowUp` enlarges and `ArrowDown` shrinks; other directions and keys SHALL be ignored. The default action of arrows SHALL always be cancelled. `keyup` SHALL finish the resize: save the state and call `on_resize_end`.

#### Scenario: Right arrow
- **WHEN** `ArrowRight` is pressed on the handle, the root is 400 px wide, `step = 5`
- **THEN** the previous panel grows by 1.25 percentage points, the next shrinks by the same amount

### Requirement: State persistence
With `state_key` set, sizes SHALL be saved when a resize finishes into `localStorage` (`state_storage = Local`) or `sessionStorage` (`Session`) as a JSON array and restored on mount if there is a value.

#### Scenario: Restore
- **WHEN** storage holds `[30,70]` under the key and the component is mounted
- **THEN** the first panel is `calc(30% - 4px)`
