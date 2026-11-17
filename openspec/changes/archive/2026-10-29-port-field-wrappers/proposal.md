# Proposal

## Why

Wave 2 starts with a set of tiny form wrappers — `ButtonGroup`, `Fluid`, `IconField` with `InputIcon`, `FloatLabel`, `IftaLabel`. Each takes 30–60 lines in PrimeNG (a container with classes and a slot), but without them a typical form cannot be assembled: an icon in a field, a floating label, a button group, "stretch everything". The demos that were skipped in InputText, InputNumber and Select (`floatlabel`, `iftalabel`) also depend on them, and Select already uses the `p-iconfield` markup for the search field.

## What Changes

- Five new crates, one per wrapper: `prime-buttongroup` (`ButtonGroup`), `prime-fluid` (`Fluid`), `prime-iconfield` (`IconField` and `InputIcon`, like `InputGroup` and `InputGroupAddon` in one crate), `prime-floatlabel` (`FloatLabel`), `prime-iftalabel` (`IftaLabel`). All have: child content, `style`, `style_class`, `pt` with a `root` part.
  - `ButtonGroup` — `<span class="p-buttongroup p-component" role="group">`.
    - `IconField` — `<span class="p-iconfield p-iconfield-left|right">` (`icon_position`, left by default), `InputIcon` — `<span class="p-inputicon">` with an icon inside.
    - `FloatLabel` — `<span class="p-floatlabel p-floatlabel-over|on|in">` (`variant`, `Over` by default), `IftaLabel` — `<span class="p-iftalabel">`.
    - `Fluid` — `<div class="p-fluid">`, which sets the "stretch nested fields" context.
- `fluid` inheritance via context: `prime-core` gets `FluidContext` (`provide_fluid`, `in_fluid`, `resolve_fluid`); `Button`, `InputText`, `Textarea`, `InputNumber` and `Select` become consumers — their `fluid` property changes from `Prop<bool>` to `OptProp<bool>`: an explicit value (including `false`) takes precedence over the context, and without it a component inside `Fluid` stretches, like `hasFluid = fluid() ?? !!pcFluid` in PrimeNG. Calls like `fluid=true` stay the same.
- Theme CSS `buttongroup`, `floatlabel`, `iftalabel` (`iconfield` is already attached); `fluid` has no styles in PrimeNG.
- Showcase pages `/buttongroup` (demos from `button/buttongroup-doc.ts`), `/fluid`, `/iconfield` (`basic`, `sizes`, `template`, `floatlabel`, `iftalabel`), `/floatlabel` (`basic`, `variants`, `invalid`), `/iftalabel` (`basic`, `invalid`); the `floatlabel` and `iftalabel` demos skipped earlier are added to the InputText, InputNumber, Select and Textarea pages; e2e, benchmarks and budgets for the five crates, `docs/field-wrappers.md`.
- **Out of scope:** Angular Forms integration (`ng-invalid`, `:has(.ng-invalid.ng-dirty)` rules in the `/* For PrimeNG */` blocks — they do not apply, `primeng_extra` does not change); the `.p-buttongroup p-button …` rules from the ButtonGroup `/* For PrimeNG */` block (in PrimeNG they are needed by the `<p-button>` host element, our `Button` renders `<button class="p-button">` itself, and the base theme CSS already covers it); `hostName`; `fluid` support in `ToggleButton` and `SelectButton` (in PrimeNG they do not inherit from `Fluid`).

## Capabilities

### New Capabilities
- `buttongroup`: a group of joined buttons.
- `fluid`: a context for stretching nested fields and buttons.
- `iconfield`: a field with an icon inside (`IconField` and `InputIcon`).
- `floatlabel`: a floating label of three variants.
- `iftalabel`: a label above the entered value.

### Modified Capabilities
<!-- The behavior of existing components is extended by the requirement from `fluid` (inheritance from the context); their own requirements remain in force, so no deltas are needed. -->

## Impact

- New crates `crates/prime-buttongroup`, `prime-fluid`, `prime-iconfield`, `prime-floatlabel`, `prime-iftalabel` (`publish = false`); dependencies `prime-core`.
- Changed: `prime-core` (`fluid`), `prime-button`, `prime-inputtext`, `prime-textarea`, `prime-inputnumber`, `prime-select` (the type and resolution of `fluid`), workspace `Cargo.toml`, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`.
