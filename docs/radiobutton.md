# RadioButton

Port of `p-radiobutton` from PrimeNG 21.1.10 (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-radiobutton::RadioButton` | `packages/primeng/src/radiobutton/radiobutton.ts`, `radiobutton/style/radiobuttonstyle.ts` |
| CSS | `@primeuix/styles/radiobutton` and the Aura preset (`cargo xtask gen-theme`) |
| Demos | `apps/showcase/doc/radiobutton/*-doc.ts` (`group`, `filled`, `sizes`, `invalid`, `disabled`) |

Spec: `openspec/specs/radiobutton`.

## What matches PrimeNG

- Structure: root, inside it `input.p-radiobutton-input` (`aria-checked`), then `div.p-radiobutton-box` with `div.p-radiobutton-icon`.
- Root classes and their order: `p-radiobutton p-component`, `p-radiobutton-checked`, `p-disabled`, `p-invalid`, `p-variant-filled`, `p-radiobutton-sm p-inputfield-sm`, `p-radiobutton-lg p-inputfield-lg`.
- Attributes `data-p-disabled`, `data-p-checked`, `data-p` on the root.
- The `on_click` event with the value and the original event (`onClick` in PrimeNG).

## Differences

| Difference | Reason |
|---|---|
| The root is a `<div>`, not the host `<p-radiobutton>` | The Angular host is a framework detail. |
| The group is a shared `model: RwSignal<Option<String>>`, without `RadioControlRegistry` | The registry is needed by Angular forms; a shared signal updates the whole group reactively. |
| The `value` attribute of `<input>` is the button's own value | In PrimeNG it is `modelValue()` (a boolean state), which breaks native form submission. |
| No `binary`, `autofocus`, form integration | Outside the scope of the change. |
| Without `model` the button is uncontrolled (`default_checked`), root classes are not tracked | The static branch creates no signals. |

## Performance

SSR: 23 allocations per instance (budget 28); contribution to raw wasm +168 KB (budget 210 KB).
Template — as for Checkbox (`docs/checkbox.md`).
