# ToggleSwitch

Port of `p-toggleswitch` from PrimeNG 21.1.10 (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-toggleswitch::ToggleSwitch` | `packages/primeng/src/toggleswitch/toggleswitch.ts`, `toggleswitch/style/toggleswitchstyle.ts` |
| CSS | `@primeuix/styles/toggleswitch` and the Aura preset (`cargo xtask gen-theme`) |
| Demos | `apps/showcase/doc/toggleswitch/*-doc.ts` (`basic`, `preselection`, `template`, `invalid`, `disabled`) |

Spec: `openspec/specs/toggleswitch`.

## What matches PrimeNG

- Structure: root with `style="position: relative"`, inside it `input.p-toggleswitch-input[role=switch]` (`aria-checked`), then `div.p-toggleswitch-slider` with `div.p-toggleswitch-handle`.
- Root classes and their order: `p-toggleswitch p-component`, `p-toggleswitch-checked`, `p-disabled`, `p-invalid`.
- Attributes `data-p-checked`, `data-p-disabled` on the root and `data-p` on the root, the track and the handle.
- The click is handled on the root (like `@HostListener('click')`), then the focus returns to `<input>`; `readonly` and `disabled` block toggling.
- The handle slot receives the current `checked`.

## Differences

| Difference | Reason |
|---|---|
| The root is a `<div>`, not the host `<p-toggleswitch>` | The Angular host is a framework detail. |
| `checked: RwSignal<bool>` instead of `ngModel` | Two-way binding without form infrastructure. |
| No `trueValue`/`falseValue`, `autofocus`, `size`, form integration | In PrimeNG `size` does not affect the classes; the rest is outside the scope of the change. |
| `handle_template: Callback<bool, AnyView>` instead of the `#handle` template | Leptos slots. |
| Without `checked` the switch is uncontrolled (`default_checked`), root classes are not tracked | The static branch creates no signals. |
| The native state of `<input>` is explicitly aligned after a click | A click on the track does not toggle `<input>` natively; a click on `<input>` toggles it before the root handler. |

## Performance

SSR: 33 allocations per instance (budget 40); contribution to raw wasm +167 KB (budget 210 KB).
Template — as for Checkbox (`docs/checkbox.md`).
