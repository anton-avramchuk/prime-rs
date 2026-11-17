# Design

## Context

Requirements are in `specs/knob`. PrimeNG 21.1.10 sources: `packages/primeng/src/knob/knob.ts`, `knob/style/knobstyle.ts`, demos `apps/showcase/doc/knob`. Reference implementation - `crates/prime-slider` (value `RwSignal<f64>`, `window_event_listener` from a handler with the handle stored in `StoredValue<_, LocalStorage>`, removal in `on_cleanup`, rules without reading the value in list closures; `docs/slider.md`).

Observations about the source that affect decisions:
- Geometry: center (50, 50), radius 40, `minRadians = 4π/3` (bottom left), `maxRadians = -π/3` (bottom right), a gap at the bottom between -120 deg and -60 deg (in radians `-2π/3 ... -π/3`). The angle of a point is computed from `atan2(size/2 - y, x - size/2)`, the value from `mapRange` by angle; when the point falls in the gap the value does not change.
- The value arc goes from the scale zero: `zeroRadians` is the position of 0 in `min..max` (with `min > 0 && max > 0` the position of `min`); the `largeArc` and `sweep` flags are chosen from the angles.
- `valueTemplate.replace('{value}', ...)` in JS replaces only the first occurrence.
- Pointer coordinates in PrimeNG are the event's `offsetX/offsetY`. For a `mousemove` event on `window` they are relative to the current target (an SVG child or another element altogether), so when the pointer leaves the scale the value is computed incorrectly; for a click on `path` the offset is also relative to the descendant.
- `onMouseUp` removes the `mousemove` listener, but by assigning `null` it loses the reference to the `mouseup` listener and does not remove it - the listener stays on `window`.
- `mousedown` calls `preventDefault`, so the SVG does not get focus from a click and keys work only after Tab.
- Step rounding `Math.round((mapped - min) / step) · step + min` is not clamped to the range (when `max - min` is not a multiple of `step`, exceeding `max` is possible). Keys step by 1 and 10 and ignore `step`.
- A `mouseup` after dragging over the SVG is accompanied by a `click`, which reapplies the same value and calls `onChange` again.

## Decisions

### D1. Value
`value: Option<RwSignal<f64>>`; without it an internal one is created with `default_value` (`OptProp<f64>`, read once), and if that is absent with `min`. The number is `f64`.

### D2. Pure logic (`state.rs`)
Functions without DOM, each with a table-driven test: `map_range`, `value_from_angle(angle, min, max, step) -> Option<f64>` (the gap is `None`; rounding by step; clamping `min..=max`), `zero_radians`, `value_radians`, `arc_point(radians)`, `range_path()`, `value_path(value, min, max)` (with `large-arc` and `sweep` flags), `key_target(code, current, min, max) -> Option<f64>`, `display_text(template, value)` (first occurrence), root classes. Coordinates in paths are rounded to three decimals: the result does not depend on `f64` noise, and the HTML is shorter.

### D3. Pointer
Coordinates are client ones: `x = clientX - rect.left`, `y = clientY - rect.top` relative to `<svg>` (`getBoundingClientRect` at the moment of the event), the center is `rect.width / 2`, `rect.height / 2`. This removes the dependence on the event target; when `size` differs from the rendered size, the scale is taken from `rect`. A click, `mousemove` on `window` and `touchmove` on `window` (a single touch) go through one path, `update_from_pointer`. The `window` subscription is created from `mousedown`/`touchstart` and removed on release, on touch end and on unmount (`on_cleanup`), both listeners without losing the reference. `mousedown` focuses the SVG and calls `preventDefault` (so that text is not selected). A click right after dragging, in which the value has already changed, is skipped (the `moved` flag) so as not to repeat `on_change`.

### D4. Keys and update
`update_value(raw, check_step)` clamps the value to the range, writes the signal and calls `on_change` on every application (like PrimeNG, even if the value did not change). Keys step by 1 and 10 regardless of `step` and are clamped to the range; `readonly` and `disabled` block everything.

### D5. Markup and reactivity
The root is a `<div>`; the `<svg>` is built once; everything that depends on the value and properties (the arcs' `d`, the text, `aria-valuenow`) is closures. SVG width and height are `style="width: {size}px; height: {size}px"`; `viewBox="0 0 100 100"`. Default colors are `var(--p-knob-range-background)`, `var(--p-knob-value-background)`, `var(--p-knob-text-color)` (what `$dt(...).variable` turns into in PrimeNG). `tabindex`: `-1` with `readonly` or `disabled`, otherwise the given one or 0. The `name` attribute on `<text>` is rendered if set (as in PrimeNG). A value outside the range is drawn clamped to the edge, while the text and `aria-valuenow` show the original.

### D6. CSS
`knob` in `theme.components` (`upstream.toml`), `cargo xtask gen-theme`; `knobstyle.ts` has no `/* For PrimeNG */` block, `primeng_extra` does not change.

### D7. Demos
`basic`, `minmax`, `reactive`, `size`, `step`, `stroke`, `template`, `color`, `readonly`, `disabled`. In `reactive` the plus and minus buttons are `Button` with `PlusIcon`/`MinusIcon`. In `basic` there is an `echo` of the value for e2e.

### D8. Tests
SSR: structure, `aria-*`, SVG size and styles, paths (key points), template and colors, `show_value`, `disabled`/`readonly`/`tabindex`, `pt`, parity of static and reactive properties. Browser (`tests/web.rs`): keys (bounds, `preventDefault`, focus), click at characteristic points (top, left, right, gap at the bottom), step, dragging with the pointer leaving the SVG, no reaction after `mouseup`, skipping `click` after dragging, `readonly`/`disabled`, external signal, reactive bounds. e2e: dragging with a real mouse and keys. Touch only via the pure function, with no check on a touch device, as with Slider.

## Risks / Trade-offs

- [Differences from PrimeNG in coordinates, listener removal, focus, range clamping and suppression of a repeated `click`] -> each is a fix of an evident defect or an accessibility improvement, recorded in `docs/knob.md`.
- [Rounding path coordinates to 3 decimals changes the text of the `d` attribute compared to PrimeNG] -> visually indistinguishable (1/1000 of a `viewBox` unit), tests check the geometry.
- [Touch is verified less than mouse] -> a pure function; there is no touch-device check, recorded in `docs/knob.md`.

## Migration Plan

No migration: a new crate. Rollback is `git revert`.

## Open Questions

None.
