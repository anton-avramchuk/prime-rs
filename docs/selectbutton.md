# SelectButton

Port of `p-selectbutton` from PrimeNG 21.1.10 (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-selectbutton::SelectButton` | `packages/primeng/src/selectbutton/selectbutton.ts`, `selectbutton/style/selectbuttonstyle.ts` |
| CSS | `@primeuix/styles/selectbutton` and the Aura preset (`cargo xtask gen-theme`); button styles — the `togglebutton` CSS |
| Demos | `apps/showcase/doc/selectbutton/*-doc.ts` (`basic`, `disabled`, `fluid`, `invalid`, `multiple`, `sizes`, `template`) |

Spec: `openspec/specs/selectbutton`.

## What matches PrimeNG

- Structure: root `div.p-selectbutton[role=group]`, inside a `ToggleButton` per option with `on_label = off_label = label` (in PrimeNG — `p-togglebutton`).
- Root classes and their order: `p-selectbutton p-component`, `p-invalid`, `p-selectbutton-fluid`; `data-p` — `invalid`.
- `size`, `fluid` and `style_class` are passed to each button (`style_class` — not to the root); `invalid` is not passed to the buttons.
- Selection as in `onOptionSelect`: single — a repeated click clears the value, `allow_empty = false` forbids this; multiple — a value is appended to the end of the list or removed, `allow_empty = false` does not allow removing the last one (`getAllowEmpty`).
- Events: `on_change` (value and original event), then `on_option_click` (option, index, original event).
- `item_template` receives the option and index and replaces the button content (the nested `ToggleButton`'s `content_template`).

## Differences

| Difference | Reason |
|---|---|
| The root is a `<div>`, not the `<p-selectbutton>` host | The host in Angular is a framework detail. |
| Options are `SelectButtonOption { label, value, disabled }`, values are strings; no `optionLabel`, `optionValue`, `optionDisabled`, `dataKey` | Arbitrary object fields are replaced in Rust by a typed option; generalizing to `T` is possible in a separate change. |
| Single mode is `value: RwSignal<Option<String>>`, multiple (`multiple`) is `values: RwSignal<Vec<String>>`; without a signal, internal state (`default_value`, `default_values`) | Two-way binding without `ngModel`; two types read more easily than a one-element list. |
| No `unselectable`, `autofocus`, `tabindex`, form integration, `/* For PrimeNG */` block | `unselectable` is replaced by `allow_empty`; the rest is out of the change's scope or not used by the PrimeNG template. |
| Each button's signal is aligned with the value explicitly (in the browser also by an `Effect`) | `ToggleButton` toggles its `checked` before calling `on_change`; without alignment a rejected click would leave the button on. |
| In the `template` demo the label is in `<strong>` instead of `pi-align-*` icons | `prime-icons` has no such icons. |

## Performance

SSR: 77 allocations per instance with two options (budget 86); contribution to raw wasm +282 KB (budget 330 KB) — together with `ToggleButton`.
