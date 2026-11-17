# Proposal

## Why

ConfirmDialog is the seventh component of wave 4: a modal confirmation window ("Delete the record?") on top of the already finished `Dialog` and `ConfirmationService`. It is the most direct use of the service extracted into `prime-confirmation` during ConfirmPopup, and completes the "popup / modal confirmation" pair.

## What Changes

- New crate `prime-confirmdialog` (port of PrimeNG 21.1.10 `p-confirmdialog`) on top of `prime-dialog::Dialog`.
- `prime-confirmation`: `ConfirmEventType` is added (`Accept`, `Reject`, `Cancel`, like PrimeNG's `ConfirmEventType`), other types do not change.
- `ConfirmDialog` listens to `ConfirmationService`; a confirmation with a matching `key` shows a `Dialog` with the `alertdialog` role, the root class `p-confirmdialog` (added to `p-dialog`) and the mask class `mask_style_class`. The body has the `p-confirmdialog-icon` icon and the `p-confirmdialog-message` text; the footer has the reject button `p-confirmdialog-reject-button` and the accept button `p-confirmdialog-accept-button` (default labels are the `reject`/`accept` translations; `*_button_props` are applied to the buttons).
- Behavior: `accept`/`reject` call the callback and hide the window; closing with the close icon, `Escape` or a click on the mask (`dismissable_mask`) counts as a rejection (`reject` with the `Cancel` type); service `Close` hides the window without callbacks; the `on_hide` event receives `ConfirmEventType`; focus on show — by `default_focus` (`accept` by default, `reject`, `close`, `none`).
- Props (defaults, overridden by `Confirmation`): `key`, `header`, `message`, `icon`, `accept_label`, `reject_label`, `accept_icon`, `reject_icon`, `accept_visible`, `reject_visible`, `accept_button_style_class`, `reject_button_style_class`, `close_on_escape`, `dismissable_mask`, `block_scroll`, `closable`, `modal`, `position` (`DialogPosition`), `default_focus`, `draggable`, `focus_trap`, `auto_z_index`, `base_z_index`, `breakpoints`, `close_aria_label`, `accept_aria_label`, `reject_aria_label`, `style`, `style_class`, `mask_style_class`, `pt`; views `header_content`, `footer`, `message_content`, `icon_content`, `headless`, `accept_icon_content`, `reject_icon_content`; event `on_hide`.
- Showcase page `/confirmdialog` with demos `basic`, `position`, `template` (message), `headless`; e2e; benchmarks and budgets; `docs/confirmdialog.md`.
- Styles: `confirmdialog` is added to `theme.components` (the `dialog` and `button` styles already exist).
- **Out of scope:** `appendTo` (the window is always in `body`, like `Dialog`), `transitionOptions` (animation — theme classes), `rtl`, `Bind`, `pTemplate`, HTML in `message` (PrimeNG inserts it via `innerHTML`; here text only), `ConfirmationService.onAccept`, `closeButtonProps`.

## Capabilities

### New Capabilities
- `confirmdialog`: a modal confirmation window on top of Dialog and ConfirmationService.

### Modified Capabilities
- `confirmation`: `ConfirmEventType` is added.

## Impact

- New crate `crates/prime-confirmdialog` (`publish = false`); dependencies: `prime-core`, `prime-dialog`, `prime-button`, `prime-confirmation`, `web-sys`.
- Changed: `prime-confirmation` (event type), workspace `Cargo.toml`, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml` (browser job, three places: `prime-confirmdialog`).
