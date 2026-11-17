# Proposal

## Why

Slider — choosing a number or a range by dragging a handle; one of the basic form components, noticeably more complex than those already ported: mouse and touch dragging, keyboard, vertical orientation, range mode. It closes the next step after Rating and sets the model for Knob.

## What Changes

- New crate `prime-slider` with the `Slider` component: a single value via a two-way `value` (`RwSignal<f64>`) or a range (`range`) via `values` (`RwSignal<[f64; 2]>`); without a signal the component keeps internal state (`default_value`, `default_values`). Properties `min` (0), `max` (100), `step`, `orientation` (`Horizontal`/`Vertical`), `animate`, `disabled`, `invalid`, `tabindex`, `aria_label`, `aria_labelledby`, `style`, `style_class`; events `on_change` (value and original event) and `on_slide_end`; `pt` (`root`, `range`, `handle`, `start_handler`, `end_handler`).
- Markup as in PrimeNG 21.1.10 (`slider.ts`, `sliderstyle.ts`): root `<div class="p-slider p-component">` with `data-pc-name`, `data-pc-section`, `data-p`, `data-p-sliding`; inside `span.p-slider-range` and one (`role=slider`) or two `span.p-slider-handle` handles with `aria-valuemin/now/max`, `aria-orientation`.
- Interaction as in PrimeNG: dragging a handle with the mouse (document handlers only for the duration of dragging) and touch, a click on the track, arrow keys, `PageUp`/`PageDown`, `Home`/`End`; step and value normalization as in `handleStepChange` and `getNormalizedValue`; RTL support.
- The `/slider` page (`basic`, `range`, `step`, `vertical`, `filter`, `input`), e2e, browser tests, benchmarks, budgets, `docs/slider.md`, the `slider` theme CSS.
- **Out of scope:** Angular Forms (`ngModel`, reactive forms); `autofocus`; the deprecated `styleClass` as separate logic (there is `style_class`); the `reactiveforms` and `templatedrivenforms` demos; an external image in `filter` (a gradient block with CSS filters instead).

## Capabilities

### New Capabilities
- `slider`: the Slider component — value and range, mouse, touch and keyboard input, orientations, step, structure and accessibility, `pt`, performance.

### Modified Capabilities
<!-- None. -->

## Impact

- New crate `crates/prime-slider` (`publish = false`); dependencies `prime-rs-core`.
- Changed: workspace `Cargo.toml`, `upstream.toml` (`slider` in `theme.components`), `showcase`, `benches/*`, `e2e/tests`, `ci.yml` (browser job for `tests/web.rs`).
