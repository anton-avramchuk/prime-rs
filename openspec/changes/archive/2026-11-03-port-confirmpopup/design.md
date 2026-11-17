# Design

## Context

The requirements are in `specs/confirmation` and `specs/confirmpopup`. PrimeNG 21.1.10 sources: `confirmpopup/confirmpopup.ts` (about 530 lines), `confirmpopup/style/confirmpopupstyle.ts`, `api/confirmation.ts`, `api/confirmationservice.ts`; styles `@primeuix/styles/confirmpopup`; demos `apps/showcase/doc/confirmpopup/{basic,template,headless}-doc.ts`. Templates in the project: `prime-popover` (the panel at the target: `align_overlay`, the arrow, flipping, window listeners, retargeting), `prime-toast` (`MessageService` as a `Copy` handle in the context with subscribers), `prime-button`.

Observations on the source:
- The component subscribes to `requireConfirmation$`: `null` — hide; a confirmation with a foreign `key` is ignored (but if the panel is already shown, it is re-aligned on the next frame). Its own confirmation fields are copied onto the component, and `accept` and `reject` are wrapped in emitters.
- The panel is in the DOM while `render`; `onBeforeEnter` sets autofocus, moves it into `body`, aligns it (`absolutePosition`), the arrow, the `overlay` layer and attaches the listeners (in a `setTimeout`, so as not to catch the click that called `confirm`).
- Arrow: `--p-confirmpopup-arrow-left` — the difference of the left edges of the target and the panel, if the panel is to the left of the target; `p-confirm-popup-flipped` and `data-p-confirmpopup-flipped`, if the panel is above the target.
- Closing: `Escape` (if `closeOnEscape !== false`) = "No"; a click outside the panel and outside the target (`dismissableMask !== false`) = simply hide; `resize` of the window (not on touch devices); scrolling of an ancestor of the target. `accept`/`reject` hide and return the focus to the target.
- Focus on show: `confirmation.defaultFocus ?? defaultFocus`: `accept` | `reject` | `none`.
- Buttons: `size` by default `small`; `rejectButtonProps` by default without a style (in the demos `severity=secondary`, `outlined`); `acceptVisible`/`rejectVisible`.

## Goals / Non-Goals

**Goals:** the same DOM, classes, roles and behaviour of the panel; a shared service and `Confirmation`, usable for ConfirmDialog without changes.

**Non-Goals:** `appendTo`, `motionOptions`, `Bind`, `pTemplate`, `ConfirmationService.onAccept`.

## Decisions

### D1. Crate `prime-confirmation`
`Confirmation` is a builder with public fields; `Callback` instead of emitters; icons are `ViewFn`. The target element `Option<web_sys::HtmlElement>` is not `Send`, so `Confirmation` is not `Send`: the service keeps the subscribers in `StoredValue<_, LocalStorage>`, as `PopoverRef` keeps the target. `ConfirmationService` is a `Copy` handle: `confirm(Confirmation)`, `close()`, `subscribe(Callback<ConfirmEvent>) -> usize`, `unsubscribe(id)`; `ConfirmEvent { Confirm(Confirmation), Close }`. `provide_confirmation_service()` puts the service into the context, `use_confirmation_service()` retrieves it (a panic with a clear text if it was not provided — as with `MessageService`).

### D2. Crate `prime-confirmpopup`: the panel in `body`
The panel is rendered into a `Portal` (in `body`) and aligned in the `Absolute` mode, as `appendTo = body` in PrimeNG. The listener logic follows the sample of `Popover`, but the component itself is not wrapped: a different root (`role=alertdialog`, the classes `p-confirmpopup`), different CSS variables of the arrow and a subscription to the service. The pure logic of the arrow and flipping (`arrow_left`, `is_flipped`, `outside_click`) already exists in `prime-popover::state`; a dependency on `prime-popover` for three functions is not needed — they are copied into `state.rs` together with table tests (3–5 lines each).

### D3. Pure logic (`state.rs`)
`root_classes(flipped, extra)`, `arrow_left`, `is_flipped`, `outside_click`, `focus_target(per_confirmation, component) -> ConfirmFocus`, `accepts_key(component_key, confirmation_key)` (comparison of `Option<&str>`: `None == None`), `button_label(custom, default)`. Table tests.

### D4. State and showing
`Core`: `confirmation: RwSignal<Option<Confirmation>>` in `LocalStorage`, `visible: RwSignal<bool>`, `target` is taken from the confirmation. The subscription to the service is in the component body (`subscribe`, `on_cleanup` → `unsubscribe`). `confirm` with the same `key`: store the confirmation, `visible = true`; if the panel is already shown — re-align it on the next tick (`set_timeout(0)`) and rebuild the content. `close` — hide. The panel is built while `visible` and removed after the exit (the classes `p-anchored-overlay-enter-active`/`-leave-active` on the root, a fallback timer, as in `Popover`).

### D5. Listeners
As in `Popover`: click, `Escape`, `resize`, `scroll` with capture; the subscription in `set_timeout(0)`; removal in `closed()` and `on_cleanup`. The difference: `Escape` calls `reject` (if `close_on_escape != false`), a click outside only hides (if `dismissable_mask != false`).

### D6. Buttons
`Button` from `prime-button` with `size=Small`; the properties from `ConfirmButtonProps` are applied over the default values; `autofocus` by `default_focus`. The focus on show — as in `Popover.focus_on_show`: the button is found by the class `p-confirmpopup-accept-button`/`-reject-button` and focused in `set_timeout(5)`. After `accept`/`reject` the focus returns to the target.

### D7. Controlled showing
`visible` (an optional `Prop<bool>`): if set, it determines the showing (`computedVisible` in PrimeNG); the internal signal is needed when `visible` is not set. Closing by the user changes only the internal state, while the showing stays with the owner of `visible`.

### D8. Headless and content
`headless` (`Callback<HeadlessContext, AnyView>`): `HeadlessContext { confirmation, accept: Callback<()>, reject: Callback<()> }` — replaces all the content of the panel; `content` (`Callback<Confirmation, AnyView>`) — only the block `p-confirmpopup-content`. In PrimeNG `#cp` gives `cp.onAccept()`; here a context is used instead.

### D9. CSS
`confirmpopup` is added to `theme.components`. If the styles have no `p-confirmpopup-reject-button`/`accept-button` — only the classes (as in PrimeNG).

### D10. Tests
`prime-confirmation`: the service (subscribers, `confirm`, `close`, `unsubscribe`, the context) — native. `prime-confirmpopup`: SSR (there is no panel until it is shown; on the server the service has no calls), a browser `tests/web.rs` as one sequential test (the shared window listeners and the `z-index` stack): showing by `confirm`, the markup, the default and custom labels, `accept`/`reject` and the callbacks, returning the focus to the target, `Escape`, a click outside, `dismissable_mask=false`, `close_on_escape=false`, a foreign `key`, a repeated `confirm` with another target, `close()`, `default_focus`, `accept_visible`/`reject_visible`, `content` and `headless`, `visible`. e2e: the position under the target, the arrow, flipping at the bottom edge, the three demos.

### D11. Demos and benchmarks
`basic` (Save/Delete with a Toast), `template` (content), `headless`. Bench `confirmpopup-1`/`-1000` (on the server there is no panel — only the root marker is measured) and `case-confirmpopup`.

## Risks / Trade-offs

- [A shared service in a separate crate] An extra crate, but ConfirmDialog does not depend on ConfirmPopup and vice versa.
- [A copy of the arrow functions] The three small functions are duplicated; moving them into `prime-dom` is a separate improvement, needed when a third consumer appears.
- [`Confirmation` is not `Send`] Because of `HtmlElement`; the service and the component use `LocalStorage`.

## Open Questions

- `ConfirmationService.onAccept` is needed only by PrimeNG's ConfirmDialog — we will come back to it in `port-confirmdialog`.

## Migration Plan

No migration. Rollback — `git revert`.
