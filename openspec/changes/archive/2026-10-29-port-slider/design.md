# Design

## Context

Requirements are in `specs/slider`. PrimeNG 21.1.10 sources: `packages/primeng/src/slider/slider.ts`, `slider/style/sliderstyle.ts`, demos `apps/showcase/doc/slider`. Models — `crates/prime-scrollpanel` (dragging, document listeners with unsubscription, `cfg(target_arch = "wasm32")` branching), `crates/prime-selectbutton` and `crates/prime-rating` (tracking rules in closures: events and focus without recreating elements; only `*_untracked` in handlers).

Observations from the source that affect decisions:
- The handle position is always derived from the value (`updateHandleValue` at the end of `updateValue`), so the position is a pure function of the value, and the raw drag percentage is not stored.
- With a given `step`, the result is not rounding to the nearest step but `handleStepChange`: a shift from the previous value by a whole number of steps toward the pointer (`floor` to the right, `ceil` to the left). This is carried over as is and covered by a table-driven test.
- Without `step` the value is `Math.floor` (to an integer), and with a fractional `step` — rounding to its number of decimals.
- In a range the value is written to `values[handleIndex]` clamped to `min..max`, and `[minVal, maxVal]` goes out; handles may cross, the active index does not change. If the starting handle sits at `max`, a press selects it (`handleIndex = 0`).
- A click on the track moves the `handleIndex` handle (the last active, the first by default); a click on a handle is not counted as a track click (`sliderHandleClick`).
- After every value change focus returns to the active handle.
- Touch: `floor(Δ·100 / size) + starting position` in percent, 1 % step; touch events arrive on the handle, no document listeners needed.

## Decisions

### D1. Value
`value: Option<RwSignal<f64>>` and `values: Option<RwSignal<[f64; 2]>>`; without a signal an internal one is created with `default_value` (`f64`, 0) or `default_values` (`Option<[f64; 2]>`, default `[0, 0]`). The one matching `range: Prop<bool>` is active; the inactive one is not read. Numbers are `f64` (a fractional `step` is allowed in PrimeNG).

### D2. Pure logic (`state.rs`)
DOM-free functions, each with a table-driven test: `percent(value, min, max)` (clamped to 0–100 and guarded against `min == max`), `value_from_percent`, `decimals(step)`, `normalize(value, step)`, `step_change(new, old, step)`, `key_target(code, current, step, range) -> Option<KeyAction>` (arrows, `PageUp/Down`, `Home/End`), `clamp`, `bar_percent(pointer, start, size, vertical, rtl)`, `touch_percent(pointer, start, size, vertical, start_percent)`, root classes and `data-p`. PrimeNG's non-standard values (step 1 for keys, 10 for `Page*`, in a range `Page*` equal 1) are fixed here.

### D3. Interaction state
`active: RwSignal<usize>` (active handle index), `dragging: RwSignal<bool>` (for `data-p-sliding` and removing `p-slider-animate`), a `handle_pressed` flag (a plain `Cell`/`AtomicBool`, like `sliderHandleClick`): a press on a handle sets it, a click on the host resets it and skips the track handling. Geometry (`getBoundingClientRect`, `offsetWidth/Height`) is read at the time of the event via the root's `NodeRef`; direction (`getComputedStyle(root).direction`) is determined there too for RTL. Client coordinates and `rect.left/top` are used, which is equivalent to PrimeNG's `pageX − (rect.left + scrollX)`.

### D4. Dragging and listeners
A press on a handle (`mousedown`) sets `dragging`, selects the handle, focuses it, cancels the default action and subscribes `mousemove` and `mouseup` on `window` (`leptos::prelude::window_event_listener`; the handler lives exactly until `mouseup` or unmount — `on_cleanup`). `mousemove` computes the value from the pointer, `mouseup` removes the listeners, resets `dragging` and calls `on_slide_end`. The subscription is created only from an event handler, i.e. in the browser, so SSR is unaffected and a separate `cfg` branch is not needed. Touch — `touchstart/touchmove/touchend` on the handle per the `onDragStart/onDrag/onDragEnd` algorithm.

### D5. Value update
A single function `update_value(raw, event, index)`: clamping to `min..=max`, normalization, writing to the signal (in a range — to `values[index]`, an ordered pair goes out), calling `on_change` and focusing the active handle. `on_change` is called on every applied action, even if the value stayed the same (like `updateValue` in PrimeNG). `set_value_from_handle` with `step` goes through `step_change`.

### D6. Markup and reactivity
The root is a `<div>`; the position of handles and fill — closures over the value and bounds (`inset-inline-start`/`bottom`, `width`/`height`), the own `position: absolute` styles are static. Handles and fill are built once by `range` (the structure depends only on `range`; when `range` changes the block is rebuilt, so the rebuild closure reads only `range`, and position and values are read inside the handles). Classes: `p-slider p-component`, `p-disabled`, `p-invalid`, `p-slider-horizontal|vertical`, `p-slider-animate` (with `animate` and not `dragging`), `style_class`, `pt.root`; `data-p` — orientation. Handle: `role=slider`, `aria-valuemin/now/max`, `aria-orientation`, `data-pc-section`, `tabindex` (absent when `disabled`), `p-slider-handle-active` on the active one in a range.

### D7. CSS
`slider` in `theme.components` (`upstream.toml`), `cargo xtask gen-theme`; `sliderstyle.ts` has no `/* For PrimeNG */` block, `primeng_extra` is unchanged.

### D8. Demos
`basic`, `range`, `step`, `vertical`, `filter`, `input`. In `filter`, instead of an external image a gradient block with CSS filters `contrast`, `brightness`, `sepia` toggled by `SelectButton`; three sliders are created on selection. In `input` the value is linked to `InputText` via two `Effect`s (string ↔ number). Width and height are set via `style`. In `basic` — an `echo` of the value for e2e.

### D9. Tests
SSR: structure, position and `aria-*` for single mode and range, orientations, classes and `data-p`, `disabled`, `pt`, parity of static and reactive properties. Browser (`tests/web.rs`): keys (step, pages, `Home/End`, bounds, focus stays on the handle), click on the track, dragging (synthetic `mousedown/mousemove/mouseup`: value, `on_slide_end`, no listeners after release — verified by `mousemove` after `mouseup` not changing the value), `disabled`, range (order, active handle), external signal and reactive bounds, handle and focus preservation after a change. e2e: dragging with a real mouse and keys. Touch is covered by pure functions and manual verification (synthetic `TouchEvent`s are inconvenient in `wasm-bindgen-test`).

## Risks / Trade-offs

- [Many behavior branches (range, RTL, vertical, step), easy to diverge from PrimeNG] → pure functions with table-driven tests and checking against the source; differences (`D1`, no raw percentage) are recorded in `docs/slider.md`.
- [`window_event_listener` in a handler: listener leak on unmount mid-drag] → removal in `on_cleanup` and on `mouseup`; a test for no reaction after `mouseup`.
- [Touch is verified less than mouse] → the pure function `touch_percent` and a manual browser run.

## Migration Plan

No migration: a new crate. Rollback — `git revert`.

## Open Questions

None.
