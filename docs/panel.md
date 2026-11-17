# Panel

Port of `p-panel` from PrimeNG 21.1.10 (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-panel::Panel` | `packages/primeng/src/panel/panel.ts`, `panel/style/panelstyle.ts` |
| Collapsing logic | `prime-rs-core::collapsible` (shared with Fieldset) |
| CSS | `@primeuix/styles/panel`, the base `p-collapsible-*` and the Aura preset (`cargo xtask gen-theme`) |
| Demos | `apps/showcase/doc/panel/*-doc.ts` (`basic`, `toggleable`, `template`); `header-toggler` — from the `toggleable-doc` text |

Spec: `openspec/specs/panel`, `openspec/specs/collapsible`.

## What matches PrimeNG

- Classes `p-panel p-component`, `p-panel-toggleable`, `p-panel-expanded`/`p-panel-collapsed`, `p-panel-header`, `p-panel-title`, `p-panel-header-actions` (+ `p-panel-icons-start|end|center`), `p-panel-toggle-button`, `p-panel-content-container`, `p-panel-content-wrapper`, `p-panel-content`, `p-panel-footer`; `data-p="toggleable"`.
- The toggle button: `p-button` classes as for `<p-button severity="secondary" text rounded>` with an icon, the "minus" icon (expanded) and the "plus" icon (collapsed).
- ARIA: the button has `aria-label` (the header text), `aria-controls`, `aria-expanded`; the region has `role="region"`, `aria-labelledby`, `aria-hidden`, `tabindex="-1"` when collapsed; focusable descendants of a collapsed region get `tabindex="-1"`.
- Animation: classes `p-collapsible-enter-active`/`p-collapsible-leave-active`, as for `pMotion="p-collapsible"`; `on_after_toggle` — when the animation ends.
- `toggler = header` toggles on a click on the header; `icon_pos`, `show_header`, `collapsed` as a two-way binding.

## Differences

| Difference | Reason |
|---|---|
| The root is a `<div>`, not the host `<p-panel>` | The Angular host is a framework detail. |
| `collapsed` is `RwSignal<bool>`; the initial value without a signal is `default_collapsed` | A two-way binding in Leptos is done with a signal (like `value` of InputText); one property cannot be both a signal and a literal. |
| Slots are `ViewFn` (`header_template`, `icons_template`, `footer_template`); `header_icons_template` receives `collapsed` | Instead of `ng-template` and the `<p-header>`/`<p-footer>` facets. |
| The button is drawn by Panel itself, not by the Button component | `aria-expanded` changes reactively, and the `pt` of Button accepts only static attributes. The classes and `data-p` match. |
| The identifier `{id}_header` is on the title, and on the button if there is no title | In PrimeNG it is on both the title and the button (a duplicate `id`); `aria-labelledby` points to a single element. |
| The collapsed region is hidden with an inline `display: none` (right away on the first render); the animation runs only on a change | No animation during SSR and hydration; without `animationend` (disabled animations, a hidden tab) completion happens by the fallback timer of 400 ms. |
| No keydown handler for Enter/Space is needed | The native `<button>` turns them into `click`. |
| No `BlockableUI`, `toggleButtonProps`, `motionOptions`, `transitionOptions` | Outside the scope of the change; the animation is configured by the theme CSS. |
| The gear menu of the `template` demo is not ported | Menu is not ported yet; the icons are SVG from `prime-icons` instead of the PrimeIcons font. |

## Performance

SSR: 63 allocations per collapsible panel (budget 70), 44 per simple one (budget 48); contribution to raw wasm +234 KB (budget 270 KB) — most of it is the `web-sys` bindings and event handlers.
Template — as for Card (`docs/card.md`) and ToggleSwitch (`docs/toggleswitch.md`).
