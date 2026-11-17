# Slider

Port of `p-slider` from PrimeNG 21.1.10 (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-slider::Slider` | `packages/primeng/src/slider/slider.ts`, `slider/style/sliderstyle.ts` |
| CSS | `@primeuix/styles/slider` and the Aura preset (`cargo xtask gen-theme`) |
| Demos | `apps/showcase/doc/slider/*-doc.ts` (`basic`, `range`, `step`, `vertical`, `filter`, `input`) |

Spec: `openspec/specs/slider`.

## What matches PrimeNG

- Structure: root `div.p-slider`, inside it `span.p-slider-range` and `span.p-slider-handle` handles (`role=slider`, `aria-valuemin/now/max`, `aria-orientation`); in a range, two handles with `data-pc-section` `startHandler` and `endHandler`.
- Root classes and their order: `p-slider p-component`, `p-disabled`, `p-invalid`, `p-slider-horizontal|vertical`, `p-slider-animate`; root attributes `data-pc-name`, `data-pc-section`, `data-p` (orientation), `data-p-sliding`.
- Keys: arrows by `step` or 1, `PageUp`/`PageDown` by `step` or 10 (in a range 1), `Home`/`End`; after each change focus is on the active handle.
- Step: `handleStepChange` — shift from the previous value by a whole number of steps toward the pointer (not rounding to the nearest step); without `step` the value is rounded down to an integer, with a fractional `step` — to its number of decimals.
- A click on the track moves the active handle (the first by default) and calls `on_slide_end`; a click that ends a handle drag is not counted as a track click (`sliderHandleClick`).
- Touch: finger offset in percent rounded down plus the handle's starting position (`onDrag`).
- RTL and vertical orientation support (the value grows from bottom to top).

## Differences

| Difference | Reason |
|---|---|
| The root is a `<div>`, not a `<p-slider>` host | The host in Angular is a framework detail. |
| `value: RwSignal<f64>` and, with `range`, `values: RwSignal<[f64; 2]>` instead of `ngModel`; without a signal, internal state | Two-way binding without form infrastructure. |
| The range pair is always ordered; when handles cross, the other one becomes active, and focus follows the value | In PrimeNG the internal array stays unordered and an ordered copy goes out; the resulting value and position are the same, but we have no divergence between the signal and the internal state. |
| The handle position is always computed from the value | Same result in PrimeNG, but via a separate `handleValue` field. |
| `mousemove`/`mouseup` listeners are attached to `window` for the duration of dragging | PrimeNG uses `document` via `Renderer2` and `NgZone`. Removed on release and in `on_cleanup`. |
| Geometry is read on every event, client coordinates and `getBoundingClientRect` | Equivalent to PrimeNG's `pageX − (rect.left + scrollX)`, but does not go stale on scrolling during dragging. |
| The "handle pressed" flag is also reset by a timer after `mouseup` | In PrimeNG, if the button is released outside the track, the flag stays and swallows the next track click. |
| No `autofocus`, no Angular Forms integration | Out of scope of the change. |
| In the `filter` demo a gradient block instead of an external image, `SelectButton` with `allow_empty=false` | No CDN dependency; the PrimeNG variant allows clearing the selection and ending up with no filter. |

Touch is covered only by pure functions (`touch_percent`) and code reading; it was not verified on a touch device or with synthetic `TouchEvent`s (they are inconvenient in `wasm-bindgen-test`).

## Performance

SSR: 64 allocations per instance (budget 72); contribution to raw wasm +291 KB (budget 335 KB).
