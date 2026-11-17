# Accordion

Port of PrimeNG 21.1.10 `p-accordion` (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-accordion::{Accordion, AccordionPanel, AccordionHeader, AccordionContent}` | `packages/primeng/src/accordion/accordion.ts`, `accordion/style/accordionstyle.ts` |
| Animation | `prime-rs-core::collapsible::bind_region_with` (shared with Panel and Fieldset) |
| CSS | `@primeuix/styles/accordion`, the `/* For PrimeNG */` block from `accordionstyle.ts` (`primeng_extra` in `upstream.toml`), base `p-collapsible-*` |
| Demos | `apps/showcase/doc/accordion/*-doc.ts` (`basic`, `multiple`, `disabled`, `controlled`, `dynamic`, `template`) |

Spec: `openspec/specs/accordion`, `openspec/specs/collapsible`.

## What matches PrimeNG

- Classes and attributes: `p-accordion p-component`, `p-accordionpanel` (+ `p-accordionpanel-active`, `p-disabled`), `p-accordionheader`, `p-accordionheader-toggle-icon`, `p-accordioncontent`, `p-accordioncontent-wrapper`, `p-accordioncontent-content`, `data-p-active`, `data-p-disabled`, `data-p="active"`.
- Header: `role="button"`, `aria-expanded`, `aria-controls`, `aria-disabled`, `tabindex` `0`/`-1`; content: `role="region"`, `aria-labelledby`; identifiers `{id}_accordionheader_{value}` and `{id}_accordioncontent_{value}`.
- Keyboard: Enter/Space/NumpadEnter toggle, ArrowDown/ArrowUp/Home/End move focus skipping disabled panels and wrapping around; `select_on_focus`.
- The `on_open` and `on_close` events fire only on a header click (as in PrimeNG: toggling from the keyboard does not call them) and only for the panel that was clicked.

## Differences

| Difference | Reason |
|---|---|
| `value` is `RwSignal<Vec<String>>` for both modes; the initial value without a signal is `default_value` | In PrimeNG `value` is `string \| number \| array`; a single type is simpler for Rust. Numbers are converted via `to_string`. |
| The four components are linked by the Leptos context (`provide_context`/`use_context`), not by Angular DI | An analogue of `inject(forwardRef(() => Accordion))`. Parts outside `Accordion`/`AccordionPanel` panic with a clear message. |
| The "down" chevron also gets the class `p-accordionheader-toggle-icon` | In PrimeNG only the "up" chevron and icons from CSS classes have it. |
| The `toggle_icon_template` slot instead of `expandIcon`/`collapseIcon` (CSS icon classes) | The port has no icon fonts, the slot accepts any SVG. |
| Inactive content is hidden with `display: none`, not `visibility` | As in Panel and Fieldset; the shared `collapsible` mechanism. |
| The animation root is `div.p-motion` inside `p-accordioncontent` | Repeats PrimeNG's `p-motion` placement so that the `.p-accordioncontent .p-motion` rule from its CSS works. |
| Without ripple, `BlockableUI`, `transitionOptions`, `motionOptions` | Out of the change's scope; animation is configured by the theme CSS. |
| No `accessibility` demo | It is PrimeNG's text reference without an interactive example. |

## Performance

SSR: 235 allocations for an accordion of three panels (budget 260), about 78 per panel; contribution to raw wasm +231 KB (budget 265 KB) — most of it is `web-sys` bindings, event handlers and two chevron icons.
There is no static branch: the activity of each panel depends on the shared signal.
