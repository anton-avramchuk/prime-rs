# InputText

Port of PrimeNG 21.1.10 `pInputText` (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-inputtext::InputText` | `packages/primeng/src/inputtext/inputtext.ts`, `inputtext/style/inputtextstyle.ts` |
| CSS | `@primeuix/styles/inputtext` and the Aura preset (`cargo xtask gen-theme`) |
| Demos | `apps/showcase/doc/inputtext/*-doc.ts` (`basic`, `filled`, `sizes`, `invalid`, `disabled`, `fluid`, `floatlabel`, `iftalabel`) |

Spec: `openspec/specs/inputtext`.

## What matches PrimeNG

- Classes `p-inputtext p-component`, `p-filled`, `p-inputtext-sm`/`-lg`, `p-invalid`, `p-variant-filled`, `p-inputtext-fluid` in the order of `inputtextstyle.ts`.
- `data-p`: `invalid`, `fluid`, `filled` (with the `filled` variant), size.
- Variant: the `variant` property, otherwise `PrimeConfig.input_variant`.

## Differences

| Difference | Reason |
|---|---|
| An `<InputText />` component, not a directive on `<input>` | Leptos has no directives on an arbitrary element. The more commonly needed `<input>` attributes are explicit properties. |
| `value: RwSignal<String>` instead of `ngModel` | Two-way binding without form infrastructure. |
| Without `value` the field is uncontrolled, `p-filled` is not tracked | A static field does not need a signal; in PrimeNG `p-filled` affects only floating labels. |
| No form integration and `KeyFilter` | Out of the change's scope. |
| The `fluid` property is `OptProp<bool>`: without a value it is taken from the `Fluid` context | Like `hasFluid = fluid() ?? !!pcFluid` in PrimeNG; see `docs/field-wrappers.md`. |

## Value model (a reference for input fields)

- `value` is an optional `RwSignal<String>`; the field reads it (`prop:value` plus the `value` attribute for SSR) and writes to it on `input`, then calls `on_input`.
- `on_input`/`on_change` receive the original `web_sys::Event`, `on_focus`/`on_blur` — `FocusEvent`.
- The static branch (`all_static!` and no `value`) creates no closures or value handlers.

## Performance

SSR: 13 allocations per instance (budget 16); contribution to raw wasm +175 KB (budget 225 KB) — most of it is `web-sys` event handlers.
The template is the same as for Button (`docs/button.md`).
