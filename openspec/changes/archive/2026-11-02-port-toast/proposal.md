# Proposal

## Why

Toast is the eighth component of wave 3: pop-up notifications in a screen corner. In PrimeNG Toast does not live on its own but receives messages from the global `MessageService` (`messageService.add({...})` from anywhere in the app); so besides the component a service is needed too. This is the first component in the project with a shared event bus between independent parts of the app (later it will be needed by `ConfirmDialog` and `ConfirmPopup`).

## What Changes

- New crate `prime-toast` (port of `p-toast` and `MessageService` from PrimeNG 21.1.10).
- `MessageService` — a `Copy` handle put into context (`provide_message_service()`) and taken from there (`use_message_service()`): `add(ToastMessage)`, `add_all(Vec<ToastMessage>)`, `clear()` (all toasts), `clear_key(key)` (toasts with a key). Subscribers (`Toast` components) receive events synchronously; the subscription is dropped on unmount.
- `ToastMessage` (builder): `severity` (`ToastSeverity`: `Success`, `Info`, `Warn`, `Error`, `Secondary`, `Contrast`, `Custom(String)`; `Info` by default), `summary`, `detail`, `key`, `id`, `life`, `sticky`, `closable` (yes), `icon`, `close_icon`, `style_class`, `content_style_class`.
- `Toast` — root `div.p-toast.p-component.p-toast-<position>` (position: `TopRight` by default, `TopLeft`, `TopCenter`, `BottomLeft`, `BottomCenter`, `BottomRight`, `Center`; `position: fixed` positioning via inline styles, as in PrimeNG) with a list of messages `div.p-toast-message.p-toast-message-<severity>[role=alert]` > `div.p-toast-message-content` (icon by severity or `icon`, `div.p-toast-message-text` with `div.p-toast-summary` and `div.p-toast-detail`, button `button.p-toast-close-button`).
- `Toast` properties: `key`, `position`, `life` (3000 ms), `auto_z_index` (yes), `base_z_index`, `prevent_open_duplicates`, `prevent_duplicates`, `breakpoints` (styles by screen width), `style_class`, `pt`; `message` (message view instead of the content), `headless` (instead of the whole message, receives the close function); `on_close`.
- Behavior: a message goes to the `Toast` with the same `key` (both key-less is also a match); it is shown with an enter animation; closes on its own after `life` (of the message and of `Toast`), except `sticky`; hovering stops the timer, leaving restarts it; the close button and `clear()` start the exit animation, after which the message is removed and `on_close` is called; the modal group `z-index` is assigned while there are messages.
- Showcase page `/toast` with demos `basic`, `severity`, `multiple`, `position`, `sticky`, `life`, `clear`, `target`, `responsive`, `template`, `headless` (simplified); e2e; benchmarks and budgets; `docs/toast.md`.
- Styles: `toast` is added to `theme.components`.
- **Out of scope:** `@primeuix/motion`, `motionOptions` and `show/hideTransformOptions`/`TransitionOptions` (animations are CSS classes), `Ripple` in the `multiple` demo, `ToastMessageOptions.data` and `text`, `pTemplate`, `pt` with context, `Bind`.

## Capabilities

### New Capabilities
- `toast`: notifications in a screen corner received from a shared message service.

### Modified Capabilities
<!-- None. -->

## Impact

- New crate `crates/prime-toast` (`publish = false`); dependencies: `prime-core`, `prime-icons` (`check`, `infocircle`, `timescircle`, `exclamationtriangle`, `times`), `prime-dom`, `web-sys`.
- Changed: workspace `Cargo.toml`, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml` (browser job, three places).
