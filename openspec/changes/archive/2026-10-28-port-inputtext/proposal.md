# Proposal

## Why

InputText is the base of all text fields and the first component that accepts input. In PrimeNG it is the `pInputText` directive on a native `<input>`, tied to a form via `ngModel`; Leptos needs a component with two-way value binding. The decisions made here (value model, input events, `invalid`, `variant`, `fluid`, size) will become the reference for Checkbox, RadioButton, ToggleSwitch and the other input fields.

## What Changes

- New crate `prime-inputtext` with the `InputText` component: two-way `value` binding (`RwSignal<String>`), events `on_input`, `on_change`, `on_focus`, `on_blur`, attributes `input_type`, `id`, `name`, `placeholder`, `disabled`, `readonly`, `required`, `maxlength`, `autocomplete`, `aria_label`, `aria_describedby`; styling `size` (`small`, `large`), `variant` (`filled`, `outlined`, the default from `PrimeConfig.input_variant`), `invalid`, `fluid`, `style_class`, `style`, `pt`.
- Classes `p-inputtext p-component` and the modifiers `p-filled`, `p-inputtext-sm`, `p-inputtext-lg`, `p-invalid`, `p-variant-filled`, `p-inputtext-fluid`, as well as `data-p` as in PrimeNG 21.1.10.
- The `/inputtext` page in the showcase (`basic`, `filled`, `sizes`, `invalid`, `disabled`, `fluid`), e2e, benchmarks, budgets.
- **Out of scope:** form integration (validators, dirty/touched), `KeyFilter`, `FloatLabel`/`IftaLabel`/`IconField`/`InputGroup` (separate components), the `Fluid` context (there is a `fluid` property).

## Capabilities

### New Capabilities
- `inputtext`: the InputText component — value and events, attributes, size, variant, `invalid`, `fluid`, `pt`, performance.

### Modified Capabilities
<!-- None. -->

## Impact

- New crate `crates/prime-inputtext` (`publish = false`); dependencies `prime-rs-core`, `web-sys`.
- Changed: `showcase`, `benches/render`, `benches/wasm-size`, `benches/budgets.toml`, `benches/alloc-budgets.toml`, `e2e/tests`.
