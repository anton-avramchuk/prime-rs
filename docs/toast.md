# Toast

Port of `p-toast` and `MessageService` from PrimeNG 21.1.10 (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-toast::Toast`, `ToastPt`, `ToastMessage`, `ToastSeverity`, `ToastPosition`, `MessageService` | `toast/toast.ts`, `toast/style/*`, `api/messageservice.ts`, `api/toastmessage.ts` |
| CSS | `@primeuix/styles/toast` — `cargo xtask gen-theme` (no `For PrimeNG` block; the container position is inline styles from `toaststyle.ts`, written by the component itself) |
| Demos | `apps/showcase/doc/toast/{basic,severity,multiple,position,sticky,life,clear,target,responsive,template,headless}-doc.ts` |

Spec: `openspec/specs/toast`.

## Usage

The service is put into context once (at the app root or above `Toast`); messages are sent from any handler:

```rust
let service = provide_message_service();
view! {
    <Toast />
    <Button label="Show" on_click=Callback::new(move |_| {
        service.add(ToastMessage::new(ToastSeverity::Success, "Success", "Message Content"));
    }) />
}
```

In components lower in the tree the service is obtained with `use_message_service()`. Addressing is by `key`: `Toast key="a"` shows only `ToastMessage::…key("a")`; `service.clear()` closes everything, `service.clear_key("a")` only the toasts with that key.

## What matches PrimeNG

- Markup and classes: `div.p-toast.p-component.p-toast-<position>` > `div.p-toast-message.p-toast-message-<severity>[role=alert]` > `p-toast-message-content` (severity icon — `Check`, `InfoCircle`, `TimesCircle`, `ExclamationTriangle`; `p-toast-message-text` with `p-toast-summary` and `p-toast-detail`; `button.p-toast-close-button`); `data-p`, `aria-live`, `aria-atomic`.
- Seven positions and the container inline styles (`position: fixed`, 20px at the edges, 50% at the center).
- Keys (both key-less — a match), `preventOpenDuplicates` and `preventDuplicates` with the same comparison (`summary`, `detail`, severity).
- `life` (messages and `Toast`, 3000 ms by default), `sticky`, timer pause on hover and restart when the cursor leaves, closing via the button and `clear`, `on_close` after the exit animation.
- Animations `p-toast-message-enter-active` (300 ms) and `p-toast-message-leave-active` (250 ms), modal group `z-index` (`auto_z_index`, `base_z_index`) while there are messages, `breakpoints` via `<style>` in `head`.

## Differences

| Difference | Reason |
|---|---|
| `MessageService` is a `Copy` handle in the Leptos context (`provide_message_service` / `use_message_service`), subscribers are called synchronously | Leptos has no dependency injection; the event bus is replaced with a subscriber list. |
| `Toast` outside `provide_message_service()` panics with a clear message | `Accordion` parts outside the root behave the same way. |
| No `@primeuix/motion`, `motionOptions`, `show/hideTransformOptions`, `show/hideTransitionOptions` | Animations are PrimeNG CSS classes; a fallback timer (600 ms) removes the message if `animationend` never fires. |
| Custom view is `message` and `headless` (`Callback`), not `pTemplate`/`ng-template`; `headless` receives the close function | Analogue of PrimeNG `closeFn`. |
| `ToastMessage` has no `data` and `text`; `severity` is `ToastSeverity` (with `Custom(String)` for `severity: 'custom'`) | Typed analogue. |
| The close button has no `autofocus` attribute | PrimeNG puts `autofocus` on every message button; in Leptos it does not need to steal focus. |
| `breakpoints` are "width — CSS properties" pairs (`Vec<(String, Vec<(String, String)>)>`) | Typed analogue of the object. |
| Demo `multiple` without `Ripple`, `headless` simplified (a card with two buttons, no file upload or gradient), `template` with initials instead of a picture | `Ripple` is not ported, no file upload. |
| No `Bind`/`pt` with context | `pt` is simple additions of classes, style and attributes. |

## Cost

31 allocations per instance in SSR (empty `Toast`); wasm contribution ≈ 330 KB (budget 370 KB in `benches/budgets.toml`).

## Not verified

Touch, `rtl`, screen readers, behavior when the page opened on the server already has messages (the server provides no messages). Verified in Chrome (browser tests and e2e with a real mouse): adding, markup and all severities, icons, closing via the button and the exit animation, `life`/`sticky`/hover, keys, `clear`/`clear_key`, both duplicate modes, `add_all`, layer, `message`/`headless`, `breakpoints`, container positions and unsubscribe on unmount. Screenshot comparison with PrimeNG (`pages.json`) was not run.
