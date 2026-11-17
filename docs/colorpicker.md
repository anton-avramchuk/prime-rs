# ColorPicker

Port of `p-colorpicker` from PrimeNG 21.1.10 (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-colorpicker::ColorPicker`, `ColorPickerPt`, `ColorPickerRef`, `ColorValue`, `ColorFormat` | `packages/primeng/src/colorpicker/colorpicker.ts`, `colorpicker/style/colorpickerstyle.ts` |
| HSB / RGB / HEX conversions, point calculation on the field and scale | `prime-colorpicker::color` — a pure port of `HSBtoRGB`, `RGBtoHSB`, `HEXtoRGB`, `pickColor`, `pickHue`, `updateUI`, table-driven tests |
| CSS | `@primeuix/styles/colorpicker` (`colorpicker` in `theme.components`) |
| Demos | `apps/showcase/doc/colorpicker/*-doc.ts` (`basic`, `inline`, `format`, `disabled`) |

Spec: `openspec/specs/colorpicker`. The value is `RwSignal<Option<ColorValue>>` (`Hex("#rrggbb")`, `Rgb`, `Hsb`).

```rust
let value = RwSignal::new(None);
view! { <ColorPicker value=value format=ColorFormat::Rgb on_change=Callback::new(|c| log(c)) /> }
```

## What matches PrimeNG

- Formulas and rounding use the same `f64`, so results are bit-identical, including `h = 300` for gray colors (that is what `RGBtoHSB` gives at zero saturation).
- Field and scale of 150 px: `s = floor(100·x/150)`, `b = floor(100·(150−y)/150)`, `h = floor(360·(150−y)/150)` with the coordinates clamped to the area.
- Swatch (a `readonly` field with a background color), opening on click and `Space`, closing on `Escape`/`Tab`, outside click and resize; `inline` without a swatch.
- The mouse gesture continues outside the area until `mouseup`; during the gesture the root has `p-colorpicker-dragging`.
- Markup and classes: `p-colorpicker`, `-preview`, `-panel`, `-content`, `-color-selector`, `-color-background`, `-color-handle`, `-hue`, `-hue-handle`.

## Differences

| Difference | Reason |
|---|---|
| The panel is rendered inside the root (`align_overlay(Relative)`), not in `p-overlay`; no `appendTo`, `overlayOptions`, `motionOptions`, `showTransitionOptions` | The same approach as Select and Popover. |
| Gesture coordinates are `clientX/Y` minus the area's `getBoundingClientRect`, not `pageX/Y` corrected for scroll | The result is the same but does not depend on scroll (as in Slider and Knob). |
| Pointer positions and the field background are computed reactively from the current HSB, not imperatively in `updateUI` | Natural for Leptos; the values are the same. |
| The swatch `aria-label` is the constant "Select Color" | The project has no translations (`TranslationKeys.ARIA`). |
| No Angular forms, `Bind` and `pt` via the host | Out of the port's scope. |

## Not verified

- Touch: the `touchstart`/`touchmove`/`touchend` events were verified only by emulation, not on a real device.
- Screen reading: `aria-*` match PrimeNG, screen reader behavior was not checked.
- Right-to-left layout.

## Performance

See `benches/` (`colorpicker-1`, `colorpicker-1000`, `case-colorpicker`) and `benches/*budgets.toml`.
