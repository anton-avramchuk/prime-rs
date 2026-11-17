# Message

Port of PrimeNG 21.1.10 `p-message` (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-message::Message` | `packages/primeng/src/message/message.ts`, `message/style/messagestyle.ts` |
| CSS | `@primeuix/styles/message` and the Aura preset (`cargo xtask gen-theme`) |
| Demos | `apps/showcase/doc/message/*-doc.ts` (`basic`, `closable`, `severity`, `outlined`, `simple`, `sizes`, `life`) |

Spec: `openspec/specs/message`.

## What matches PrimeNG

- Root with `role="alert"` and `aria-live="polite"`, classes `p-message p-component p-message-<severity>`, `p-message-<variant>`, `p-message-sm`/`-lg`.
- Structure `p-message-content-wrapper` > `p-message-content` with an icon, text and close button; `data-p` (`outlined`, `simple`, severity, size) on the root and all parts.
- When `text` is set, a `span.p-message-text` with the text is rendered, followed by a second `span.p-message-text` with the child content.
- Close button `p-message-close-button` with an `aria-label` from the locale (`aria.close`) and a `p-message-close-icon` icon; `on_close` with the original event; `life` in milliseconds.
- Slots `icon_template`, `close_icon_template`, `container_template` (receives the close callback).

## Differences

| Difference | Reason |
|---|---|
| The root is a `<div>`, not a `<p-message>` host | The host in Angular is a framework detail. |
| A closed message stays in the DOM with `p-message-leave-active` and `display: none`, without animation | `@primeuix/motion` animations are not ported; PrimeNG removes the element after the animation. |
| No `escape=false`, `showTransitionOptions`, `hideTransitionOptions`, `motionOptions` | HTML in text and animations are outside the change scope; use child elements for HTML. |
| No ripple on the close button | The button does not use the `pRipple` directive. |
| `icon`/`close_icon` are CSS classes, as in PrimeNG, but the PrimeIcons font is not shipped | Use `icon_template` with `prime-icons` SVG icons. |
| The static branch is only for non-interactive messages | With `closable` or `life` a visibility signal is needed, so the reactive branch is chosen. |
| `pt.close_icon` for the built-in icon sets only the class | The icon is a separate `prime-icons` component. |

## Performance

SSR: 36 allocations per instance with text (budget 44); contribution to raw wasm +218 KB (budget 270 KB).
The template is the same as for ToggleSwitch (`docs/toggleswitch.md`).
