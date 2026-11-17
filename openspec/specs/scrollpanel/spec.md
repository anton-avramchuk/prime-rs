# scrollpanel Specification

## Purpose
TBD - created by archiving change port-scrollpanel. Update Purpose after archive.

## Requirements

### Requirement: ScrollPanel structure and classes
The `ScrollPanel` component of the `prime-scrollpanel` crate SHALL render `<div class="p-scrollpanel p-component">`, inside which come, in order, `div.p-scrollpanel-content-container` with a nested `div.p-scrollpanel-content` (holding the content), then the bar `div.p-scrollpanel-bar.p-scrollpanel-bar-x` and the bar `div.p-scrollpanel-bar.p-scrollpanel-bar-y`. The content SHALL be set by `children` or the `content_template` slot; when the slot is set, `children` are not rendered. `style_class` and `style` SHALL be appended without losing the component's own classes.

#### Scenario: Element order
- **WHEN** a ScrollPanel with `children` is rendered
- **THEN** the HTML contains the content container with `p-scrollpanel-content` inside, then `p-scrollpanel-bar-x`, then `p-scrollpanel-bar-y`

#### Scenario: Slot instead of children
- **WHEN** `content_template` and `children` are both set
- **THEN** only the slot content is rendered inside `p-scrollpanel-content`

### Requirement: ScrollPanel bar accessibility
Each bar SHALL have `role="scrollbar"`, `tabindex="0"`, `aria-orientation` (`horizontal` for X, `vertical` for Y), `aria-valuenow` (the last known `scrollLeft`/`scrollTop`, initially 0), `aria-controls` referencing the content `id` and `data-pc-group-section="bar"`. The `p-scrollpanel-content` content SHALL have this `id`, unique on the page and identical in SSR and hydration.

#### Scenario: Bar attributes
- **WHEN** a ScrollPanel is rendered
- **THEN** `bar-x` has `aria-orientation="horizontal"`, `bar-y` has `aria-orientation="vertical"`, both have `role="scrollbar"` and `aria-controls` equal to the content `id`

### Requirement: ScrollPanel properties and pass-through
The properties `style`, `style_class` and `step` SHALL accept a value or a signal; `style` and `style_class` update the markup when the signal changes, `step` is read when a key is pressed. `pt` SHALL accept the parts `root`, `content_container`, `content`, `bar_x`, `bar_y`, adding classes, style and attributes without replacing the component's own.

#### Scenario: Reactive class
- **WHEN** `style_class` is passed as a signal and changes
- **THEN** the root class is updated, the component's own classes are kept

#### Scenario: Class via pt
- **WHEN** the class `custom` is passed in `pt.bar_y`
- **THEN** the vertical bar has `custom` and `p-scrollpanel-bar-y`

### Requirement: Bar calculation and hiding
In the browser ScrollPanel SHALL recalculate the bars on `scroll` and `mouseenter` on the content, on window `resize`, after mounting and on a `refresh()` call. For an axis where `clientSize / scrollSize >= 1`, the bar SHALL get `data-p-scrollpanel-hidden="true"` and the class `p-scrollpanel-hidden`; otherwise `data-p-scrollpanel-hidden="false"`, the class is removed, and the bar size and position are set in `style` by the PrimeNG formulas (length is `max(ratio × 100, 10)` percent, offset proportional to scroll).

#### Scenario: Content fits
- **WHEN** the content does not exceed the visible area vertically
- **THEN** the vertical bar is hidden (`p-scrollpanel-hidden`)

#### Scenario: Content taller than the area
- **WHEN** the content is taller than the area and scrolled half of the available travel
- **THEN** the vertical bar is visible, its `height` is at least 10% and `top` reflects the scroll position

#### Scenario: Maximum height
- **WHEN** the root has `max-height` set and its own height is zero
- **THEN** after mounting the root height equals `min(max-height, content height with paddings and borders)`

### Requirement: Bar dragging
On `mousedown` on a bar ScrollPanel SHALL focus it, set `data-p-scrollpanel-grabbed="true"` and the class `p-scrollpanel-grabbed` on the bar and on `body`, subscribe to document `mousemove` and `mouseup` and cancel the default action. On `mousemove` the content SHALL scroll by `delta / ratio` of the corresponding axis. On `mouseup` the attributes and classes are removed and the document listeners are unsubscribed. On unmount all listeners SHALL be removed.

#### Scenario: Dragging
- **WHEN** the user presses the vertical bar and moves the mouse down
- **THEN** the content `scrollTop` grows, and on `mouseup` the class `p-scrollpanel-grabbed` is removed

### Requirement: ScrollPanel keyboard
When a bar is focused ScrollPanel SHALL determine the active axis (focus on X is horizontal, on Y is vertical; wheel/gesture scrolling switches the axis by the changed `scrollLeft`/`scrollTop`). For the vertical axis `ArrowDown`/`ArrowUp` scroll by `±step` pixels 40 ms after the press (repeating until `keyup`), `ArrowLeft`/`ArrowRight` only cancel the default action; for the horizontal axis symmetrically. `keyup` SHALL reset the timer. After `blur` from the horizontal bar the axis SHALL return to vertical.

#### Scenario: Arrow down
- **WHEN** focus is on the vertical bar and `ArrowDown` is pressed
- **THEN** after 40 ms the content `scrollTop` has grown by `step`, the event is cancelled

#### Scenario: Key release
- **WHEN** `keyup` arrives before the timer
- **THEN** no scrolling is performed

### Requirement: ScrollPanel programmatic control
The property `panel_ref: Option<ScrollPanelRef>` SHALL provide a handle with the methods `refresh()` (bar recalculation) and `scroll_top(value)` (sets the content `scrollTop`, the value clamped to the range `0..=scrollHeight − clientHeight`). Outside the browser the methods do nothing.

#### Scenario: scroll_top clamping
- **WHEN** `scroll_top(100000)` is called for content 500 tall with an area of 150
- **THEN** `scrollTop` equals 350

### Requirement: ScrollPanel rendering cost
A ScrollPanel with static properties SHALL use the static branch; both branches SHALL produce the same markup. SSR does not touch the DOM. The number of allocations per instance on SSR SHALL fit within the `scrollpanel-1000` budget in `benches/alloc-budgets.toml`, the wasm contribution within the `scrollpanel` budget in `benches/budgets.toml`.

#### Scenario: Branch parity
- **WHEN** one set of values is passed statically and as signals
- **THEN** the HTML matches

#### Scenario: Budgets
- **WHEN** `cargo xtask alloc-budget` and `cargo xtask wasm-size` are run
- **THEN** values of ScrollPanel do not exceed the budgets
