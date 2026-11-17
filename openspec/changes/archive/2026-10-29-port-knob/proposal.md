# Proposal

## Why

Knob - numeric input as a circular scale dragged by mouse and touch; by design it is the closest relative of Slider (value, bounds, step, dragging with a `window` subscription, keys), only with an SVG arc instead of a bar. The port follows `port-slider` and reuses its techniques.

## What Changes

- New crate `prime-knob` with the `Knob` component: two-way `value` (`RwSignal<f64>`) or internal state with `default_value` (`min` is shown without it), properties `min` (0), `max` (100), `step` (1), `size` (100 px), `stroke_width` (14), `show_value` (`true`), `value_template` (`{value}`), `value_color`, `range_color`, `text_color`, `readonly`, `disabled`, `required`, `name`, `tabindex`, `aria_label`, `aria_labelledby`, `style`, `style_class`; the `on_change` event (new value) and `pt` (`root`, `svg`, `range`, `value`, `text`).
- Markup as in PrimeNG 21.1.10 (`knob.ts`, `knobstyle.ts`): root `<div class="p-knob p-component">` (`p-disabled`), inside `<svg viewBox="0 0 100 100" role="slider">` with `path.p-knob-range`, `path.p-knob-value` and `text.p-knob-text`; an arc from -120 deg to +120 deg (with a gap at the bottom), the value is drawn from the scale zero.
- Interaction as in PrimeNG: click on the scale, dragging with mouse and touch (`window` listeners only while pressed), arrow keys (+-1), `PageUp`/`PageDown` (+-10), `Home`/`End`.
- Page `/knob` (`basic`, `minmax`, `reactive`, `size`, `step`, `stroke`, `template`, `color`, `readonly`, `disabled`), e2e, browser tests, benchmarks, budgets, `docs/knob.md`, `knob` theme CSS.
- **Out of scope:** Angular Forms (`ngModel`, reactive forms); `styleClass` as separate logic (`style_class` exists); the `reactiveforms` and `templatedrivenforms` demos.

## Capabilities

### New Capabilities
- `knob`: the Knob component - value and scale, mouse, touch and keyboard input, structure and accessibility, `pt`, performance.

### Modified Capabilities
<!-- None. -->

## Impact

- New crate `crates/prime-knob` (`publish = false`); dependencies `prime-rs-core`.
- Changed: workspace `Cargo.toml`, `upstream.toml` (`knob` in `theme.components`), `showcase`, `benches/*`, `e2e/tests`, `ci.yml` (browser job for `tests/web.rs`).
