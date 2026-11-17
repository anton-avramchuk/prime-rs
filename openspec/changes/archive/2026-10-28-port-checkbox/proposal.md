# Proposal

## Why

Checkbox is the next component of wave 1 and the first field with a boolean or multiple value. The value model of InputText (a two-way signal, events with the original `web_sys::Event`) is extended here with the `binary` mode and a set of values, as in PrimeNG; the decisions will become the example for RadioButton and ToggleSwitch.

## What Changes

- A new crate `prime-checkbox` with the `Checkbox` component: the `binary` mode (value `RwSignal<bool>`) and the set mode (`value` + `RwSignal<Vec<String>>`); `indeterminate`, `disabled`, `readonly`, `required`, `invalid`, `size`, `variant`, `input_id`, `name`, `tabindex`, `aria_label`, `aria_labelledby`, `input_class`, `input_style`, `style_class`, `style`, the `icon_template` slot, `pt` (`root`, `input`, `box`, `icon`).
- The events `on_change` (the new `checked` value and the original event), `on_focus`, `on_blur`.
- Markup and classes as in PrimeNG 21.1.10: `p-checkbox p-component` and the modifiers `p-checkbox-checked p-highlight`, `p-disabled`, `p-invalid`, `p-variant-filled`, `p-checkbox-sm p-inputfield-sm`, `p-checkbox-lg p-inputfield-lg`; `<input type="checkbox" class="p-checkbox-input">`, `<div class="p-checkbox-box">` with the `check` or `minus` icon; the attributes `data-p`, `data-p-checked`, `data-p-highlight`, `data-p-disabled`.
- The `/checkbox` page (`basic`, `multiple`, `filled`, `sizes`, `indeterminate`, `invalid`, `disabled`), e2e, benchmarks, budgets.
- **Not included:** form integration, arbitrary types for `trueValue`/`falseValue` (only `bool`), `checkboxIcon` as a CSS class (there is `icon_template`).

## Capabilities

### New Capabilities
- `checkbox`: the Checkbox component — state, events, attributes, classes, `pt`, performance.

### Modified Capabilities
<!-- None. -->

## Impact

- A new crate `crates/prime-checkbox` (`publish = false`); dependencies `prime-rs-core`, `prime-icons` (`check`, `minus`), `web-sys`.
- Changes to `showcase`, `benches/render`, `benches/wasm-size`, budgets, `e2e/tests`, the CI browser job.
