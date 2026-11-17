# Proposal

## Why

Message is a component for displaying messages (information, success, warning, error) with closing and auto-hide. After the input fields this is the first wave 1 component with a lifetime timer and content slots.

## What Changes

- New crate `prime-message` with the `Message` component: `severity` (`success`, `info` by default, `warn`, `error`, `secondary`, `contrast`), `variant` (`outlined`, `text`, `simple`), `size` (`small`, `large`), `text`, child content, `icon` (CSS class), slots `icon_template`, `close_icon_template`, `container_template` (receives the close callback), `closable`, `close_icon` (CSS class), `life` (ms), `style`, `style_class`, `pt` (`root`, `content_wrapper`, `content`, `icon`, `text`, `close_button`, `close_icon`), the `on_close` event.
- Markup and classes as in PrimeNG 21.1.10: root `role="alert" aria-live="polite"` with classes `p-message p-component p-message-<severity>`, `p-message-<variant>`, `p-message-sm`/`-lg`; inside `p-message-content-wrapper` > `p-message-content` with an icon, text (`p-message-text`), a close button (`p-message-close-button` with `p-message-close-icon`, `aria-label` from the locale); `data-p` on the root and all parts.
- A closed Message is hidden (the `p-message-leave-active` class and `display: none`).
- The `/message` page (`basic`, `closable`, `severity`, `outlined`, `simple`, `sizes`, `icon`, `life`), e2e, benchmarks, budgets.
- **Out of scope:** `escape=false` (HTML in `text`), `@primeuix/motion` animations (`showTransitionOptions`, `hideTransitionOptions`, `motionOptions`), ripple on the close button, integration with `MessageService`/`Toast`.

## Capabilities

### New Capabilities
- `message`: the Message component: markup, classes, closing, `life`, slots, `pt`, performance.

### Modified Capabilities
<!-- None. -->

## Impact

- New crate `crates/prime-message` (`publish = false`); dependencies `prime-rs-core`, `prime-icons` (`times`), `web-sys`.
- Changed: `showcase`, `benches/render`, `benches/wasm-size`, budgets, `e2e/tests`; the CI browser job is unchanged (browser tests for `life` and closing are added to it).
