# Proposal

## Why

ToggleSwitch is the next component of wave 1 after RadioButton: a switch of a boolean value. It repeats the Checkbox model (the `checked` signal, the uncontrolled mode, the static and reactive branches), but has a different markup (`slider` with `handle`) and fires on a click on the whole root, as in PrimeNG.

## What Changes

- A new crate `prime-toggleswitch` with the `ToggleSwitch` component: a two-way `checked: RwSignal<bool>`, `default_checked` for the uncontrolled case; `disabled`, `readonly`, `required`, `invalid`, `input_id`, `name`, `tabindex`, `aria_label`, `aria_labelledby`, `style`, `style_class`, the slot `handle_template` (receives `checked`), `pt` (`root`, `input`, `slider`, `handle`).
- The events `on_change` (the new value and the original event, like `onChange` in PrimeNG), `on_focus`, `on_blur`.
- Markup and classes as in PrimeNG 21.1.10: the root `p-toggleswitch p-component` with `style="position: relative"` and the modifiers `p-toggleswitch-checked`, `p-disabled`, `p-invalid`; `<input type="checkbox" role="switch" class="p-toggleswitch-input">` with `aria-checked`; `<div class="p-toggleswitch-slider">` with `<div class="p-toggleswitch-handle">`; the attributes `data-p`, `data-p-checked`, `data-p-disabled`.
- The `/toggleswitch` page (`basic`, `preselection`, `template`, `invalid`, `disabled`), e2e, benchmarks, budgets.
- **Not included:** form integration, arbitrary types for `trueValue`/`falseValue` (only `bool`), `autofocus`, `size` (in PrimeNG it does not affect the classes).

## Capabilities

### New Capabilities
- `toggleswitch`: the ToggleSwitch component — state, events, attributes, classes, `pt`, performance.

### Modified Capabilities
<!-- None. -->

## Impact

- A new crate `crates/prime-toggleswitch` (`publish = false`); dependencies `prime-rs-core`, `web-sys`.
- Changes to `showcase`, `benches/render`, `benches/wasm-size`, budgets, `e2e/tests`, the CI browser job.
