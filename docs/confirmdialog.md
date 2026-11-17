# ConfirmDialog

Port of PrimeNG 21.1.10 `p-confirmdialog` (commit `fe82b291d6`, see `upstream.toml`). The shared service and the confirmation description are in `docs/confirmation.md`, the window — `docs/dialog.md`.

| Our component | PrimeNG sources |
|---|---|
| `prime-confirmdialog::ConfirmDialog`, `ConfirmDialogPt`, `ConfirmDialogHeadless` | `packages/primeng/src/confirmdialog/confirmdialog.ts`, `confirmdialog/style/confirmdialogstyle.ts` |
| CSS | `@primeuix/styles/confirmdialog` and the Aura preset — `cargo xtask gen-theme` (there is no `/* For PrimeNG */` block); the window and buttons take the `dialog` and `button` styles |
| Demos | `apps/showcase/doc/confirmdialog/*-doc.ts` (`basic`, `position`, `template`, `headless`) |

Spec: `openspec/specs/confirmdialog`.

```rust
let confirm = provide_confirmation_service();
view! {
    <ConfirmDialog />
    <Button label="Delete" on_click=move |_| confirm.confirm(
        Confirmation::new().header("Confirmation").message("Delete this record?").accept(Callback::new(|()| { /* ... */ })),
    ) />
}
```

## What matches PrimeNG

- `ConfirmDialog` is a wrapper over `Dialog` with the `alertdialog` role and the `p-confirmdialog` class on the window. Body: the `p-confirmdialog-icon` icon and the `p-confirmdialog-message` text; footer: the `p-confirmdialog-reject-button` and `p-confirmdialog-accept-button` buttons (reject first). Labels: `*_label`, then `*_button_props.label`, then the `reject` and `accept` translations.
- Every value is taken from `Confirmation` first, then from the component prop (`option()` in PrimeNG): header, text, icon, button labels and icons, button visibility, button classes, `modal`, `position`, `closable`, `block_scroll`, `close_on_escape`, `dismissable_mask`, `default_focus`. A confirmation that arrives while the window is open replaces the shown one in place.
- Closing: the buttons call `accept` / `reject`; the close icon, `Escape` and a click on the mask — `reject` (`ConfirmEventType::Cancel` arrives in `on_hide`); service `Close` hides the window without callbacks. `on_hide` receives the close type.
- `header_content`, `footer`, `message_content`, `icon_content`, `headless`, `accept_icon_content` icons via `accept_icon`/`reject_icon`; `breakpoints`, `draggable`, `focus_trap`, `auto_z_index`, `base_z_index`, `style`, `style_class`, `mask_style_class`.

## Differences

| Difference | Reason |
|---|---|
| Focus on show works by `default_focus` (`Accept` by default), including `Close`; buttons are found by the real `p-confirmdialog-*-button` and `p-dialog-close-button` classes | In PrimeNG the `.p-confirm-dialog-accept`/`-reject` selectors do not match the button classes, so there `Dialog` itself decides the focus. |
| `message` is rendered as text, not HTML | PrimeNG inserts it via `innerHTML`; text is safer and does not require `inner_html`. |
| `reject` does not receive the event type; the type is only in `on_hide` | `Callback<()>` is shared by ConfirmPopup and ConfirmDialog; `ConfirmEventType` is needed only by the window. |
| `resizable` and `maximizable` are explicitly off | In PrimeNG the window is not resizable; our `Dialog` has `resizable` on by default. |
| No `appendTo`: the window is always in `body` (like `Dialog`) | The other overlays of the project have none either. |
| No `transitionOptions` and `rtl` | The animation is set by theme classes; right-to-left layout was not verified. |
| The confirmation position is a string (`"topleft"`, …), an unknown value gives `center` | `Confirmation` does not depend on the `Dialog` crate. |
| PrimeIcons icons in the demos are replaced with `prime-icons` icons; `closeButtonProps` and `ConfirmationService.onAccept` are not ported | No PrimeIcons; these fields are not used by the demos. |
| The `Dialog` header became reactive (`prime-dialog`) | So that replacing a confirmation in place updates the header. |

## Not verified

- Right-to-left layout and touch input.
- Screen reading: `aria-*` and the role were checked against PrimeNG, screen reader behavior was not.
- Several `ConfirmDialog`s with different keys open at once — only checked that a foreign key is ignored.

## Performance

See `benches/` (`confirmdialog-1`, `confirmdialog-1000`, `case-confirmdialog`) and `benches/*budgets.toml`. There is no window on the server, the measurement is the cost of the service subscription.
