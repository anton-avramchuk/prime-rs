# Proposal

## Why

InputGroup groups an input with text, an icon or buttons next to it (prefix and suffix). It is almost static markup over the already finished InputText and Button: no shared logic or client code is needed, the risk is minimal, and the component is needed for typical forms.

## What Changes

- New crate `prime-inputgroup` with two components: `InputGroup` and `InputGroupAddon`.
- `InputGroup`: root `<div class="p-inputgroup">` with projected content, `style_class`, `style`, `pt`.
- `InputGroupAddon`: `<div class="p-inputgroupaddon">` with projected content, `style_class`, `style`, `pt`.
- Markup as in PrimeNG 21.1.10 (`inputgroup.ts`, `inputgroupaddon.ts`, `inputgroupstyle.ts`, `inputgroupaddonstyle.ts`): the host has no `role` and no `data-p`.
- Page `/inputgroup` (`basic`, `button`, `checkbox`, `multiple`), e2e, benchmarks, budgets, `docs/inputgroup.md`, `inputgroup` theme CSS.
- **Out of scope:** the `floatlabel` and `iftalabel` demos (FloatLabel and IftaLabel are not in the port yet); the `/*For PrimeNG*/` block from `inputgroupstyle.ts` (rules for the Angular wrappers `p-button`, `p-inputwrapper`, `p-inputmask`, which do not exist in Leptos); the `fluid` property.

## Capabilities

### New Capabilities
- `inputgroup`: the InputGroup and InputGroupAddon components - structure, classes, projected content, `style`, `pt`, performance.

### Modified Capabilities
<!-- None. -->

## Impact

- New crate `crates/prime-inputgroup` (`publish = false`); dependencies `prime-core`.
- Changed: workspace `Cargo.toml`, `upstream.toml` (`inputgroup` in `theme.components`), `showcase`, `benches/*`, `e2e/tests`.
