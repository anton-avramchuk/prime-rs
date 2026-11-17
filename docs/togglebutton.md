# ToggleButton

A port of `p-togglebutton` from PrimeNG 21.1.10 (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-togglebutton::ToggleButton` | `packages/primeng/src/togglebutton/togglebutton.ts`, `togglebutton/style/togglebuttonstyle.ts` |
| CSS | `@primeuix/styles/togglebutton`, the Aura preset and the `/* For PrimeNG (iconPos) */` block (`cargo xtask gen-theme`, `primeng_extra`) |
| Demos | `apps/showcase/doc/togglebutton/*-doc.ts` (`basic`, `customized`, `disabled`, `fluid`, `invalid`, `sizes`) |

Spec: `openspec/specs/togglebutton`.

## What matches PrimeNG

- Structure: root `div.p-togglebutton[role=button]`, inside it `span.p-togglebutton-content` with an icon (`span.p-togglebutton-icon` and `-left`/`-right`) and `span.p-togglebutton-label`.
- Root classes and their order: `p-togglebutton p-component`, `p-togglebutton-checked`, `p-invalid`, `p-disabled`, `p-togglebutton-sm p-inputfield-sm` / `p-togglebutton-lg p-inputfield-lg`, `p-togglebutton-fluid`.
- Attributes `aria-pressed`, `tabindex` (`0`, `-1` when `disabled`), `data-pc-name`, `data-p-checked`, `data-p-disabled`, `data-p` (`checked invalid <size>`) on the root and `data-p` on the content.
- `toggle` as in PrimeNG: click and the Enter and Space keys (by `event.code`, with `preventDefault`); `disabled` and `allow_empty = false` on a checked button block toggling.
- Default labels are "Yes" and "No", an empty label is rendered as a space; the `icon_template` and `content_template` slots receive the current `checked`.

## Differences

| Difference | Reason |
|---|---|
| The root is a `<div>`, not the `<p-togglebutton>` host | In Angular the host is a framework detail. |
| `checked: RwSignal<bool>` instead of `ngModel`; without it, an internal signal with `default_checked` | Two-way binding without forms infrastructure; in PrimeNG the component also holds the state itself. |
| `on_icon`/`off_icon` are a `ViewFn` in `span.p-togglebutton-icon`, not a PrimeIcons class | `prime-icons` has no font; the wrapper is needed for CSS (`order` on `-right`). |
| `icon_pos` is its own `IconPos` (`Left`, `Right`) | A dependency on `prime-button` just for an enum is not needed. |
| `icon_template`, `content_template`: `Callback<bool, AnyView>` | Leptos slots. |
| No `autofocus`, ripple, Angular Forms integration | Outside the change scope; the `.ng-invalid.ng-dirty` rule in CSS is harmless. |
| In the `customized` demo `CheckIcon` and `TimesIcon` instead of `pi-lock` and `pi-lock-open` | `prime-icons` has no locks. |
| No static branch: the markup is always reactive | Any press changes `class`, `aria-pressed`, `data-*` and the label. |

`xtask gen-theme` accepts the `/* For PrimeNG */` marker with an optional note in parentheses (in `togglebuttonstyle.ts` it is `(iconPos)`).

## Performance

SSR: 23 allocations per instance (budget 26); contribution to raw wasm +169 KB (budget 200 KB).
