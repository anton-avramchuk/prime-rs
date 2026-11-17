# Proposal

## Why

Textarea is a multi-line input field, the second component of wave 2. It reuses the value model of InputText (a two-way `RwSignal<String>`, events, the variant from `PrimeConfig`) and adds client behaviour — automatic height adjustment (`autoResize`) with the `onResize` event.

## What Changes

- A new crate `prime-textarea` with the `Textarea` component (`<textarea>`): a two-way `value`, `rows`, `cols`, `id`, `name`, `placeholder`, `disabled`, `readonly`, `required`, `maxlength`, `aria_label`, `aria_describedby`, `auto_resize`, `size`, `variant`, `invalid`, `fluid`, `style`, `style_class`, the events `on_input`, `on_change`, `on_focus`, `on_blur`, `on_resize`, `pt`.
- Classes as in PrimeNG 21.1.10: `p-textarea p-component` and `p-filled`, `p-textarea-resizable`, `p-variant-filled`, `p-textarea-fluid`, `p-inputfield-sm p-textarea-sm`, `p-textarea-lg p-inputfield-lg`, `p-invalid` (the PrimeNG directive has no `data-p`).
- `auto_resize`: on mounting and on each change of the value the height is set by `scrollHeight` (taking `max-height` into account), only in the browser.
- The `/textarea` page (`basic`, `autoresize`, `filled`, `sizes`, `invalid`, `disabled`, `fluid`), e2e, benchmarks, budgets.
- **Not included:** form integration, `KeyFilter`, `FloatLabel`/`IftaLabel`, the `Fluid` context.

## Capabilities

### New Capabilities
- `textarea`: the Textarea component — value and events, attributes, classes, height auto-resize, `pt`, performance.

### Modified Capabilities
<!-- None. -->

## Impact

- A new crate `crates/prime-textarea` (`publish = false`); dependencies `prime-rs-core`, `web-sys`.
- Changes to `showcase`, `benches/*`, `e2e/tests`, the CI browser job.
