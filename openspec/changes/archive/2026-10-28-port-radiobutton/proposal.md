# Proposal

## Why

RadioButton is the next component of wave 1 after Checkbox. It reuses the Checkbox model (the root + `input` + `box`, two data classes, the value signal), adding the selection of one value from a group through a shared signal.

## What Changes

- A new crate `prime-radiobutton` with the `RadioButton` component: the `value` of the checkbox and the two-way `model: RwSignal<Option<String>>` of the group; `default_checked` for the uncontrolled case; `disabled`, `required`, `invalid`, `size`, `variant`, `input_id`, `name`, `tabindex`, `aria_label`, `aria_labelledby`, `style`, `style_class`, `pt` (`root`, `input`, `box`, `icon`).
- The events `on_click` (the value and the original event, like `onClick` in PrimeNG), `on_focus`, `on_blur`.
- Markup and classes as in PrimeNG 21.1.10: `p-radiobutton p-component` and the modifiers `p-radiobutton-checked`, `p-disabled`, `p-invalid`, `p-variant-filled`, `p-radiobutton-sm p-inputfield-sm`, `p-radiobutton-lg p-inputfield-lg`; `<input type="radio" class="p-radiobutton-input">` with `aria-checked`, `<div class="p-radiobutton-box">` with `<div class="p-radiobutton-icon">`; the attributes `data-p`, `data-p-checked`, `data-p-disabled`.
- The `/radiobutton` page (`group`, `filled`, `sizes`, `invalid`, `disabled`), e2e, benchmarks, budgets.
- **Not included:** form integration and the `RadioControlRegistry` registry (the group is defined by a shared signal), `binary`, `autofocus`.

## Capabilities

### New Capabilities
- `radiobutton`: the RadioButton component — state, events, attributes, classes, `pt`, performance.

### Modified Capabilities
<!-- None. -->

## Impact

- A new crate `crates/prime-radiobutton` (`publish = false`); dependencies `prime-core`, `web-sys`.
- Changes to `showcase`, `benches/render`, `benches/wasm-size`, budgets, `e2e/tests`, the CI browser job.
