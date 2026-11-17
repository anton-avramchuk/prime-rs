# Design

## Context

Requirements are in `specs/message`. PrimeNG 21.1.10 sources: `packages/primeng/src/message/message.ts`, `message/style/messagestyle.ts`, demos `apps/showcase/doc/message`. Component template: `docs/toggleswitch.md`, `docs/checkbox.md`.

## Goals / Non-Goals

**Goals:** markup, classes and `data-p` as in PrimeNG; closing and `life`; slots; a static branch for a non-interactive message.
**Non-Goals:** `escape=false`, `@primeuix/motion` animations, ripple on the close button, `MessageService`/`Toast`.

## Decisions

### D1. The root is a `<div>`
Instead of the `<p-message>` host; `role="alert"`, `aria-live="polite"`, `data-p` are on the root.

### D2. Text and content
As in PrimeNG: with a non-empty `text`, a `span.p-message-text` with the text is rendered, then always a second `span.p-message-text` with the child content (empty if there are no children). `container_template` replaces both and receives the close callback.

### D3. Closing and `life`
The `visible` state is created only if the message is closable or has `life`. `close` hides the message and calls `on_close`. Hiding is the `p-message-leave-active` class and `display: none` (PrimeNG removes the element after the animation). `life` starts a `set_timeout` timer in the browser only.

### D4. Icons
`icon` is a CSS class (`<i class="p-message-icon …">`); `icon_template` is rendered without a wrapper (as in PrimeNG); by default the close button contains a `TimesIcon` with the `p-message-close-icon` class; `aria-label` comes from the locale (`aria.close`).

### D5. Branches
Static when `all_static!`, `!closable` and no `life`; otherwise reactive. There is one markup, in a `macro_rules!`; the parts (icon, text, button) are built by functions over the properties.

### D6. Tests
SSR: structure, classes, `data-p`, role, text and children, slots, `pt`, parity. Browser: close on click, `on_close`, auto-hide by `life`.

## Risks / Trade-offs

- [Without animation hiding is instant] → recorded in `docs/message.md`.

## Migration Plan

No migration: a new crate. Rollback is `git revert`.

## Open Questions

None.
