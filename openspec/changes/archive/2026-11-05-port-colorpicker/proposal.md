# Proposal

## Why

ColorPicker — color selection: the user moves a pointer over the saturation/brightness field and the hue scale, and the result goes into the value in `hex`, `rgb` or `hsb` format. The component is self-contained: pure color math (HSB/RGB/HEX) plus a panel with two draggable areas. It opens as an overlay (a swatch field and a panel) or is embedded (`inline`). The first component of wave 6.

## What Changes

- New crate `prime-colorpicker` (port of `p-colorpicker` from PrimeNG 21.1.10).
- Pure module `color.rs`: `Hsb`, `Rgb`, `hsb_to_rgb`, `rgb_to_hsb`, `hex_to_rgb`, `hsb_to_hex`, value clamping; table-driven tests on the PrimeNG formulas (including rounding).
- `ColorPicker`: root `div.p-colorpicker.p-component` (`p-colorpicker-overlay` without `inline`, `p-colorpicker-dragging` during a gesture); `input.p-colorpicker-preview` (`readonly`, `p-disabled`, background — the current color, `aria-label` "Select Color") and panel `div.p-colorpicker-panel` > `.p-colorpicker-content` > `.p-colorpicker-color-selector` (> `.p-colorpicker-color-background` > `.p-colorpicker-color-handle`) and `.p-colorpicker-hue` (> `.p-colorpicker-hue-handle`). The area size is 150 px.
- The value is `RwSignal<ColorValue>` (`Hex(String)`, `Rgb(Rgb)`, `Hsb(Hsb)`) by `format`; an empty value shows `default_color` (`ff0000`).
- Behavior: click/`Space` on the swatch toggles the panel; `Escape`/`Tab` close; press and drag with mouse and touch on the field and scale change the color (window `mousemove`/`mouseup` listeners for the duration of the gesture); outside click and resize close the panel; `disabled` blocks gestures.
- Properties: `value`, `format`, `inline`, `default_color`, `disabled`, `tabindex`, `input_id`, `autofocus`, `auto_z_index`, `id`, `style`, `style_class`; events `on_change`, `on_show`, `on_hide`; `ColorPickerRef` (`show`, `hide`).
- Page `/colorpicker` (`basic`, `inline`, `format`, `disabled`), e2e, benchmarks and budgets, `docs/colorpicker.md`, `colorpicker` in `theme.components`.
- **Out of scope:** Angular forms, `Bind`, `pt` via the host, `overlayOptions`/`appendTo`/`motionOptions` (the panel is inside the root, like Select/Popover), `showTransitionOptions`/`hideTransitionOptions`.

## Capabilities

### New Capabilities
- `colorpicker`: color selection on a panel with a brightness/saturation field and a hue scale.

### Modified Capabilities
<!-- None. -->

## Impact

- New crate `crates/prime-colorpicker` (`publish = false`); dependencies: `prime-core`, `prime-dom`.
- Changed: workspace `Cargo.toml`, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml`.
