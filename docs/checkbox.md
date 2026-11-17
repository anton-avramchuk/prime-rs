# Checkbox

Port of `p-checkbox` from PrimeNG 21.1.10 (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-checkbox::Checkbox` | `packages/primeng/src/checkbox/checkbox.ts`, `checkbox/style/checkboxstyle.ts` |
| CSS | `@primeuix/styles/checkbox` and the Aura preset (`cargo xtask gen-theme`) |
| Demos | `apps/showcase/doc/checkbox/*-doc.ts` (`basic`, `multiple`, `filled`, `sizes`, `indeterminate`, `invalid`, `disabled`) |

Spec: `openspec/specs/checkbox`.

## What matches PrimeNG

- Structure: root, inside it `input.p-checkbox-input`, then `div.p-checkbox-box` with a `check` or `minus` icon (`p-checkbox-icon`).
- Root classes and their order: `p-checkbox p-component`, `p-checkbox-checked p-highlight`, `p-disabled`, `p-invalid`, `p-variant-filled`, `p-checkbox-sm p-inputfield-sm`, `p-checkbox-lg p-inputfield-lg`.
- Attributes `data-p-highlight`, `data-p-checked`, `data-p-disabled` on the root and `data-p` on the root and the box.
- The `binary` and set-of-values modes; `indeterminate` does not count as checked and is reset after a change.

## Differences

| Difference | Reason |
|---|---|
| The root is a `<div>`, not the host `<p-checkbox>` | The Angular host is a framework detail. |
| `checked: RwSignal<bool>` (binary) and `model: RwSignal<Vec<String>>` (set) instead of `ngModel` | Two-way binding without form infrastructure; set values are strings. |
| No arbitrary `trueValue`/`falseValue` types | Only `bool`. |
| `icon_template: ViewFn` instead of `checkboxIcon` (a class) and the `#icon` template | Leptos slots. |
| `pt.icon` sets only the class, the icon has no `data-p` | The icon is a separate `prime-icons` component and does not accept arbitrary attributes. |
| Without signals the checkbox is uncontrolled (`default_checked`), root classes are not tracked | The static branch creates no signals. |
| No form integration | Outside the scope of the change. |

## Performance

SSR: 34 allocations per instance with an icon (budget 42); contribution to raw wasm +201 KB (budget 250 KB).
Template — as for InputText (`docs/inputtext.md`) and Button (`docs/button.md`); the closures of the reactive branch are assembled via `rx(props, pt, fn)` over shared `Arc`s.
