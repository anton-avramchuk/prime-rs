# Textarea

Port of `pTextarea` from PrimeNG 21.1.10 (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-textarea::Textarea` | `packages/primeng/src/textarea/textarea.ts`, `textarea/style/textareastyle.ts` |
| CSS | `@primeuix/styles/textarea` and the Aura preset (`cargo xtask gen-theme`) |
| Demos | `apps/showcase/doc/textarea/*-doc.ts` (`basic`, `autoresize`, `filled`, `sizes`, `invalid`, `disabled`, `fluid`, `floatlabel`, `iftalabel`) |

Spec: `openspec/specs/textarea`.

## What matches PrimeNG

- Classes `p-textarea p-component`, `p-filled`, `p-textarea-resizable`, `p-variant-filled`, `p-textarea-fluid`, `p-inputfield-sm p-textarea-sm`, `p-textarea-lg p-inputfield-lg`, `p-invalid` in the order of `textareastyle.ts` (the directive has no `data-p`).
- Variant: the `variant` property, otherwise `PrimeConfig.input_variant`.
- `auto_resize`: `height = auto`, then `scrollHeight`; when the inline `max-height` is reached — `overflow-y: scroll`, otherwise `overflow: hidden`; `on_resize` after the adjustment.

## Differences

| Difference | Reason |
|---|---|
| A `<Textarea />` component, not a directive on `<textarea>` | Leptos has no directives on arbitrary elements. |
| `value: RwSignal<String>` instead of `ngModel`; without `value` the field is uncontrolled and `p-filled` is not tracked | As for InputText (`docs/inputtext.md`). |
| Height adjustment runs on signal changes and on input, not on every change check | Leptos has no analog of `ngAfterViewChecked`. |
| The initial SSR text is escaped manually, and a leading line break is duplicated | Leptos renders the children of `<textarea>` without escaping (like `<script>`): otherwise a value such as `</textarea><script>` would break the markup, and the HTML parser drops the first line break. |
| No `KeyFilter` and no form integration; the `keyfilter` demo is not ported | Outside the scope of the change. |
| The `fluid` property is `OptProp<bool>`: without a value it is taken from the `Fluid` context | As `hasFluid = fluid() ?? !!pcFluid` in PrimeNG; see `docs/field-wrappers.md`. |

## Performance

SSR: 10 allocations per instance (budget 12); contribution to raw wasm +255 KB (budget 300 KB) — most of it is the `web-sys` bindings for height adjustment and events.
Template — as for InputText (`docs/inputtext.md`).
