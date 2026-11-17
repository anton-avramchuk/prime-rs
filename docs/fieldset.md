# Fieldset

Port of `p-fieldset` from PrimeNG 21.1.10 (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-fieldset::Fieldset` | `packages/primeng/src/fieldset/fieldset.ts`, `fieldset/style/fieldsetstyle.ts` |
| Collapsing logic | `prime-core::collapsible` (shared with Panel, see `docs/panel.md`) |
| CSS | `@primeuix/styles/fieldset`, the base `p-collapsible-*` and the Aura preset (`cargo xtask gen-theme`) |
| Demos | `apps/showcase/doc/fieldset/*-doc.ts` (`basic`, `toggleable`, `template`) |

Spec: `openspec/specs/fieldset`.

## What matches PrimeNG

- Root `<fieldset class="p-fieldset p-component">` with `<legend class="p-fieldset-legend">`; classes `p-fieldset-toggleable`, `p-fieldset-collapsed`, `p-fieldset-legend-label`, `p-fieldset-toggle-button`, `p-fieldset-toggle-icon`, `p-fieldset-content-container`, `p-fieldset-content-wrapper`, `p-fieldset-content`; `data-p="toggleable"` on the root and the legend.
- With `toggleable`, a `<button>` inside the legend with a plus/minus icon, a label and a slot; `aria-controls`, `aria-expanded`, `aria-label` (the legend text).
- The content region: `role="region"`, `aria-labelledby` (the button id), `aria-hidden`, `tabindex="-1"` when collapsed; focusable descendants of a collapsed region get `tabindex="-1"`.
- Animation `p-collapsible-enter-active`/`p-collapsible-leave-active`; `on_after_toggle` — when the animation ends.

## Differences

| Difference | Reason |
|---|---|
| `collapsed` is `RwSignal<bool>`; the initial value without a signal is `default_collapsed` | As for Panel (`docs/panel.md`). |
| Slots are `ViewFn` (`header_template`, `expand_icon_template`, `collapse_icon_template`) instead of `ng-template` and `<p-header>` | Leptos has no Angular templates. |
| Without `toggleable` the region has no `aria-labelledby` attribute | In PrimeNG it points to a missing button. |
| `pt.toggle_icon` applies only the class to the standard SVG icons | Icon components do not accept arbitrary attributes; for a slot the class and the attributes are applied. |
| Enter/Space are handled by the native `<button>` | A separate `keydown` is not needed. |
| No `BlockableUI`, `transitionOptions`, `motionOptions` | Outside the scope of the change; the animation is configured by the theme CSS. |

## Performance

SSR: 57 allocations per collapsible Fieldset (budget 63); contribution to raw wasm +220 KB (budget 255 KB) — most of it is the `web-sys` bindings and event handlers.
Template — as for Panel (`docs/panel.md`).
