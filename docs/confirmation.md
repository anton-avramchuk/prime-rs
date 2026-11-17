# Confirmation and ConfirmationService

Port of `api/confirmation.ts` and `api/confirmationservice.ts` from PrimeNG 21.1.10 (commit `fe82b291d6`, see `upstream.toml`). The `prime-confirmation` crate has no components or styles; `ConfirmPopup` (`docs/confirmpopup.md`) and later `ConfirmDialog` use it.

| Our type | PrimeNG sources |
|---|---|
| `Confirmation`, `ConfirmButtonProps`, `ConfirmFocus` | `packages/primeng/src/api/confirmation.ts` |
| `ConfirmEventType` | `packages/primeng/src/api/confirmeventtype.ts` |
| `ConfirmationService`, `ConfirmEvent`, `provide_confirmation_service`, `use_confirmation_service` | `packages/primeng/src/api/confirmationservice.ts` |

Spec: `openspec/specs/confirmation`.

The service is put into the context once (at the application root or above the component), and confirmations are sent from any handler:

```rust
let service = provide_confirmation_service();
service.confirm(Confirmation::new().key("delete").message("Sure?").accept(Callback::new(|()| { /* ... */ })));
service.close();
```

## What matches PrimeNG

- All fields of `Confirmation`: `key`, `message`, `header`, `icon`, `target`, `accept`, `reject`, button labels and icons, `accept_visible`/`reject_visible`, button classes and properties, `default_focus`, `close_on_escape`, `dismissable_mask`, `block_scroll`, `closable`, `position`, `modal`. Unset fields are `None`; the default values (buttons visible, closing on `Escape` and on a click) are applied by the component, as in PrimeNG (`!== false`).
- `confirm` and `close` reach all subscribers in subscription order; a confirmation is shown by the component with the same `key`.

## Differences

| Difference | Reason |
|---|---|
| `ConfirmationService` is a `Copy` handle in the Leptos context (`provide_confirmation_service` / `use_confirmation_service`), subscribers are called synchronously | Leptos has no dependency injection; the `Subject` is replaced by a list of subscribers (as `MessageService`). |
| `accept` and `reject` are `Callback<()>`, not a `Function` with the `acceptEvent`/`rejectEvent` emitters | There is no `EventEmitter`. |
| `acceptButtonProps` and `rejectButtonProps` are the `ConfirmButtonProps` struct (`label`, `severity`, `outlined`, `text`, `size`), not an arbitrary object | A typed subset of `Button` properties that the components need. |
| No `closeButtonProps` and no `ConfirmationService.onAccept` | They are not used by the `ConfirmDialog` and `ConfirmPopup` demos. |
| Icons are `ViewFn` | There is no PrimeIcons font. |
| `target` is an `HtmlElement` (from an event — `Confirmation::target_event`) | `EventTarget` in PrimeNG is cast to an element. |

## Not verified

- Several services working at the same time in different subtrees of the context.
