# InputGroup

Port of `p-inputgroup` and `p-inputgroup-addon` from PrimeNG 21.1.10 (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-inputgroup::InputGroup` | `packages/primeng/src/inputgroup/inputgroup.ts`, `inputgroup/style/inputgroupstyle.ts` |
| `prime-inputgroup::InputGroupAddon` | `packages/primeng/src/inputgroupaddon/inputgroupaddon.ts`, `inputgroupaddon/style/inputgroupaddonstyle.ts` |
| CSS | `@primeuix/styles/inputgroup` and the Aura preset (`cargo xtask gen-theme`) |
| Demos | `apps/showcase/doc/inputgroup/*-doc.ts` (`basic`, `button`, `checkbox`, `multiple`) |

Spec: `openspec/specs/inputgroup`.

## What matches PrimeNG

- Root `div.p-inputgroup`, addon `div.p-inputgroupaddon`; projected content inside without extra wrappers.
- The hosts have no `role` and no `data-p`.

## Differences

| Difference | Reason |
|---|---|
| The roots are `<div>`, not the `<p-inputgroup>` and `<p-inputgroup-addon>` hosts | The host in Angular is a framework detail. |
| `InputGroup` has `style` | PrimeNG does not have it on `InputGroup` (only on the addon); for consistency with the other components. |
| No `/*For PrimeNG*/` block from `inputgroupstyle.ts` | Rules for the Angular wrappers `p-button`, `p-inputwrapper`, `p-inputmask`, which do not exist in Leptos. |
| No `fluid` | In PrimeNG 21.1.10 `InputGroup` has no such input: `instance.fluid` is always `undefined`, the `p-inputgroup-fluid` class is not rendered. |
| No `floatlabel` and `iftalabel` demos | FloatLabel and IftaLabel are not ported yet. |
| In `basic` and `button`, `InputText` and plain buttons instead of InputNumber, Select and Menu; SVG icons from `prime-icons` (`search`, `star-fill`, `check`, `times`) instead of the PrimeIcons font (`user`, `map`, `clock`) | These components are not in the port yet, and the icons `user`, `map`, `clock` are not in the set. |

## Performance

SSR: 19 allocations per instance with two addons and an input (budget 21); contribution to raw wasm +93 KB (budget 107 KB).
The template is like Toolbar's (`docs/toolbar.md`).
