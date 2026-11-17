# Design

## Context

Requirements are in `specs/colorpicker`. PrimeNG 21.1.10 sources: `colorpicker/colorpicker.ts` (`pickColor`, `pickHue`, `updateUI`, `HSBtoRGB`, `RGBtoHSB`, `writeControlValue`), `style/colorpickerstyle.ts`, `@primeuix/styles/colorpicker`. Reference implementations in the project: `prime-popover` (panel inside the root, `align_overlay`, outside click, `watch_scroll`), `prime-slider`/`prime-knob` (gesture via window listeners, `StoredValue<_, LocalStorage>`, removal in `on_cleanup`), `prime-select` (swatch + panel).

## Goals / Non-Goals

**Goals:** the same DOM, the same formulas and values as in PrimeNG.

**Non-Goals:** Angular forms, `appendTo`, `overlayOptions`, `motionOptions`, `Bind`.

## Decisions

### D1. Pure color module (`color.rs`)
`Hsb {h,s,b: f64}`, `Rgb {r,g,b: u8-like f64/i32}`; functions `hsb_to_rgb`, `rgb_to_hsb`, `hex_to_rgb` (string with or without `#`; garbage → black), `rgb_to_hex`, `hsb_to_hex`, `validate_hsb`, `pick_field(x, y) -> (s, b)`, `pick_hue(y) -> h`. All without the DOM, table-driven tests on the TS formulas (including `h == 360`, grays, boundary cases).

### D2. Value
`enum ColorValue { Hex(String), Rgb(Rgb), Hsb(Hsb) }` and `value: RwSignal<Option<ColorValue>>`; the internal `hsb` signal is the source of truth for display. An effect (under `cfg(wasm32)`) converts the external value to `hsb` by `format`; `None` — `default_color`. A write from a gesture updates `hsb` and `value` in one action, without an effect loop (comparison with the last written value).

### D3. Overlay
The panel is inside the root, like Select/Popover: `prime_dom::align_overlay(Relative)`, `ZGroup::Overlay`, outside click and resize on `window` (listeners are attached in `set_timeout(0)` after showing), `prime_dom::watch_scroll` instead of its own scroll listener. The panel is hidden until the position is computed. `inline` — without a swatch and overlay.

### D4. Gestures
The area size is the constant 150. Coordinates: `clientX/Y` minus the area's `getBoundingClientRect` (as in Slider/Knob, not `pageX/Y` — the result is the same but does not depend on scroll). Mouse: `mousedown` on the area → window `mousemove`/`mouseup` listeners for the duration of the gesture, `StoredValue<_, LocalStorage>`, removal in `on_cleanup`. Touch: `touchstart` on the area, `touchmove`/`touchend` on it with `preventDefault`. Touch is NOT verified on a real device — record this honestly in the docs.

### D5. Pointer positions
The pointer and field background styles are computed from `hsb` reactively (not imperatively as in TS): `style:left/top`, `background-color`. Swatch: `style:background-color`.

### D6. SSR
The initial `hsb` from `value`/`default_color` without the DOM; the swatch with a background and the closed panel (or the embedded one) are in the markup.

### D7. Tests
Colors — tables. SSR: markup of closed/embedded, classes, format, disabled state. Browser `tests/web.rs` as one sequential test: opening with `Space`/click, `Escape`, outside click, click on the field and scale (values), dragging until leaving the bounds, `format` rgb/hsb, external value, `disabled`. e2e: real clicks on the field and scale, dragging, `Space`.

## Risks / Trade-offs

- [Rounding] `floor` and `round` from TS are reproduced exactly; comparison with PrimeNG reference values in the tables.
- [Touch] Event emulation only.
- [Panel position] The swatch is an `input`, the panel is positioned relative to the root.

## Open Questions

- None.

## Migration Plan

No migration. Rollback — `git revert`.
