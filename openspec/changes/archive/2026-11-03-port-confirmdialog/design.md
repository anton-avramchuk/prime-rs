# Design

## Context

Requirements are in `specs/confirmdialog` and `specs/confirmation`. PrimeNG 21.1.10 sources: `confirmdialog/confirmdialog.ts` (about 600 lines), `confirmdialog/style/confirmdialogstyle.ts`, `api/confirmationservice.ts`, `api/confirmeventtype.ts`; styles `@primeuix/styles/confirmdialog`; demos `apps/showcase/doc/confirmdialog/{basic,position,template,headless}-doc.ts`. Models in the project: `prime-dialog` (window, mask, layer, scroll, focus, `Escape`), `prime-confirmpopup` (service subscription, `key`, retargeting, buttons), `prime-confirmation`.

Observations on the source:
- The component is a thin wrapper over `p-dialog`: it passes `role="alertdialog"`, `closable`, `modal`, `header`, `closeOnEscape`, `blockScroll`, `position`, `dismissableMask`, `draggable`, `baseZIndex`, `autoZIndex`, root and mask classes; values are taken by the `option(name)` function — first the confirmation's properties (its keys are copied onto the component), then the component's properties.
- Body: an icon (`i.p-confirmdialog-icon` with the class from `icon`) and `span.p-confirmdialog-message` (the `message` content via `innerHTML`); templates `message`, `icon`, `header`, `footer`, `headless`.
- Footer: the reject button (if `rejectVisible`), then accept (if `acceptVisible`); labels `acceptLabel || acceptButtonProps.label || translation`.
- Closing: `onVisibleChange(false)` (close icon, `Escape`, mask) → `close()` = `reject` with `ConfirmEventType.CANCEL` + hide; buttons → `accept` / `reject` with the `ACCEPT`/`REJECT` types; service `Close` → `hide()` without callbacks. `onHide` passes the type.
- After hiding (`onDialogHide`, i.e. after the exit animation) the confirmation is forgotten. Focus: `getElementToFocus` looks for `.p-confirm-dialog-accept`/`-reject` (in PrimeNG the buttons have no such classes — the real classes are `p-confirmdialog-*-button`), so by default focus there lands where `Dialog` decides.

## Goals / Non-Goals

**Goals:** the same DOM, classes, role and closing behavior; using `Dialog` and the shared service without changing their API.

**Non-Goals:** `appendTo`, `transitionOptions`, `rtl`, HTML in the message, `Bind`, `pTemplate`.

## Decisions

### D1. Wrapper over `Dialog`
`ConfirmDialog` creates a `RwSignal<bool>` for showing and passes it to `Dialog` (`visible`), and builds the content, footer and header with closures. The other `Dialog` props (`modal`, `position`, `closable`, `close_on_escape`, `dismissable_mask`, `block_scroll`, `draggable`, `focus_trap`, `auto_z_index`, `base_z_index`, `breakpoints`, `style`, `style_class`, `mask_style_class`, `close_aria_label`) are passed via `Signal::derive` from the confirmation or component value, so a confirmation change while the window is open applies in place. `role = "alertdialog"`, `resizable = false`, `maximizable = false` (PrimeNG does not set them; the PrimeNG `Dialog` default is `resizable = false`; our `Dialog` has `resizable = true` by default, so we turn it off explicitly).

### D2. Value priority
`option(name)`: the `Confirmation` field if set, otherwise the component prop. The pure function `pick(from_confirmation, from_component)` and the derived ones (`accept_label`, `position_of(&str) -> DialogPosition`, `default_focus`) are in `state.rs` with table tests. The confirmation position is a string (`"topleft"`, …) parsed into `DialogPosition`; an unknown value is `Center`.

### D3. Closing and event flow
The service subscription is like in ConfirmPopup. `answered: StoredValue<bool>` marks that the window is closed programmatically (buttons, `Close`); an `Effect` (wasm only) watches `visible`: if it became `false` and there was no programmatic close, this is the close icon, `Escape` or the mask → `reject` + `on_hide(Some(Cancel))`. After a programmatic close the flag is reset. `Dialog.on_hide` (after the animation) forgets the confirmation. A repeated `confirm` while the window is open replaces the confirmation and increments `generation` (the content closures watch it).

### D4. Footer buttons
The same `Button` and props as in ConfirmPopup, but the default size is regular (PrimeNG does not set `size`) and the classes are `p-confirmdialog-reject-button`/`-accept-button`. The label helper (`button_label`) repeats the order "label → `*_button_props.label` → translation"; PrimeNG has the same order. We will not extract the shared button logic into `prime-confirmation`: the two components have their own styling, about ten lines are duplicated.

### D5. Focus
`Dialog.focus_on_show = false`; in `Dialog.on_show` we find the button by class (`accept`, `reject`) or `p-dialog-close-button` (`close`) and focus it (`set_timeout(5)`), `none` — nothing. This is a difference from PrimeNG, where the selectors do not match the classes (see Context), and is documented.

### D6. Templates
`header_content` → `Dialog.header_content`; `footer` replaces the buttons; `message_content: Callback<Confirmation, AnyView>` replaces the icon and text; `icon_content: ViewFn` replaces only the icon; `headless: Callback<ConfirmDialogHeadless, AnyView>` (context — the confirmation, `accept`, `reject`) → `Dialog.headless`.

### D7. Message
The `message` text is rendered as text, not HTML: safer and does not require `inner_html`. The confirmation icon is a `ViewFn` (`Confirmation.icon`) in `span.p-confirmdialog-icon`.

### D8. CSS
`confirmdialog` is added to `theme.components`.

### D9. Tests
`prime-confirmation`: `ConfirmEventType` (`as_str`). `prime-confirmdialog`: SSR (no window before a confirmation), browser `tests/web.rs` as one sequential test (shared listeners and the `z-index` stack, the scroll-lock counter): showing on `confirm`, markup, role, classes, default and custom labels, accept/reject and the types in `on_hide`, close icon and `Escape` = `Cancel` rejection, a click on the mask with `dismissable_mask`, `close()` without callbacks, a foreign `key`, confirmation replacement while the window is open, `default_focus`, `accept_visible`/`reject_visible`, `position`, templates, scroll lock released after closing. e2e: theme styles (mask, centered window, footer buttons), positions, accept/reject with Toast, templates.

### D10. Demos and benchmarks
`basic` (Delete/Save with Toast), `position` (nine positions), `template` (custom message), `headless`. Bench `confirmdialog-1`/`-1000` and `case-confirmdialog`.

## Risks / Trade-offs

- [Double close source] The "programmatic" vs "by the user" distinction rests on the `answered` flag; it is checked by the browser tests.
- [Effect only in the browser] There is no window on the server, so there are no SSR composition checks.
- [`Dialog` differences] If `Dialog` changes its defaults, ConfirmDialog inherits them; so the important ones (`resizable`, `maximizable`) are set explicitly.

## Open Questions

- Whether a shared confirmation button helper is needed: we will wait for SpeedDial/other consumers; the duplication is small for now.

## Migration Plan

No migration. Rollback — `git revert`.
