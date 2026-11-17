# Knob

Port of `p-knob` from PrimeNG 21.1.10 (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-knob::Knob` | `packages/primeng/src/knob/knob.ts`, `knob/style/knobstyle.ts` |
| CSS | `@primeuix/styles/knob` and the Aura preset (`cargo xtask gen-theme`) |
| Demos | `apps/showcase/doc/knob/*-doc.ts` (`basic`, `minmax`, `reactive`, `size`, `step`, `stroke`, `template`, `color`, `readonly`, `disabled`) |

Spec: `openspec/specs/knob`.

## What matches PrimeNG

- Structure: root `div.p-knob`, inside `<svg viewBox="0 0 100 100" role="slider">` with `path.p-knob-range`, `path.p-knob-value` and `text.p-knob-text` (`x=50`, `y=57`, `text-anchor=middle`); root classes `p-knob p-component`, `p-disabled`.
- Geometry: center (50, 50), radius 40, scale from -120 deg to +120 deg relative to the top (angles `4π/3` and `-π/3`), gap at the bottom; the value arc goes from the scale zero (with `min > 0` from `min`) with `large-arc` and `sweep` flags as in `valuePath`.
- Value from angle: `mapRange`, rounding `Math.round((v - min) / step) · step + min`; points in the gap at the bottom are ignored.
- Keys: arrows +-1 (`step` is not taken into account), `PageUp`/`PageDown` +-10, `Home`/`End`; the result is clamped to `min..max`.
- `tabindex` is `-1` with `readonly` and `disabled`; `aria-valuemin/max/now`, `required`, `aria-label`, `aria-labelledby`; `name` on `<text>`.
- Text template: only the first `{value}` is replaced; with no value it shows `min`.
- Default colors are the theme variables `--p-knob-range-background`, `--p-knob-value-background`, `--p-knob-text-color` (what `$dt(...).variable` turns into in PrimeNG).

## Differences

| Difference | Reason |
|---|---|
| The root is a `<div>`, not the `<p-knob>` host | The host in Angular is a framework detail. |
| `value: RwSignal<f64>` instead of `ngModel`; without it internal state (`default_value`, `min` by default) | Two-way binding without form infrastructure. |
| Pointer coordinates are `clientX/Y` minus the SVG's `getBoundingClientRect()` | PrimeNG uses `offsetX/offsetY`, which are relative to the current event target: for `mousemove` on `window` and a click on `path` the value is computed incorrectly, especially when the pointer leaves the scale. |
| `mouseup` and `touchend` remove both `window` listeners | In PrimeNG `onMouseUp` assigns `null` to the `mouseup` listener reference without removing it, and it stays on `window`. |
| Mouse press focuses the SVG | In PrimeNG `mousedown` with `preventDefault` prevents the SVG from getting focus, and keys work only after Tab. |
| The result of step rounding is clamped to `min..max` | In PrimeNG when `max - min` is not a multiple of `step`, the value can exceed `max`. |
| A `click` right after dragging is skipped | In PrimeNG it reapplies the same value and calls `onChange` again. |
| A value outside `min..max` is drawn clamped to the scale edge, the text and `aria-valuenow` show the original | In PrimeNG the arc goes past the end of the scale. |
| Coordinates in `d` are rounded to three decimals | Shorter HTML, independent of `f64` noise; 1/1000 of a `viewBox` unit is invisible to the eye. |
| No separate `styleClass` logic, no Angular Forms integration | `style_class` exists; forms are out of the change's scope. |

Touch is covered only by pure functions (`value_from_angle`) and code reading; it was not tested on a touch device or with synthetic `TouchEvent`s (they are inconvenient in `wasm-bindgen-test`).

## Performance

SSR: 54 allocations per instance (budget 60); contribution to raw wasm +265 KB (budget 305 KB).
