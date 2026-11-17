# Proposal

## Why

ConfirmPopup is the sixth component of wave 4: a popup confirmation at the element that triggered the action ("Delete the record?"). It is built on the shared confirmation service `ConfirmationService`, which ConfirmDialog will use afterwards, so the service and the confirmation description are moved into a separate crate without an interface.

## What Changes

- A new crate `prime-confirmation` (a port of `api/confirmation.ts` and `api/confirmationservice.ts` from PrimeNG 21.1.10), without components and styles:
  - `Confirmation` — the description of a confirmation (a builder): `key`, `message`, `header`, `icon` (`ViewFn`), `target` (the target element for ConfirmPopup), `accept`, `reject` (`Callback<()>`), the labels and icons of the buttons, `accept_visible`, `reject_visible`, `accept_button_style_class`, `reject_button_style_class`, `accept_button_props`, `reject_button_props` (`ConfirmButtonProps`: `label`, `severity`, `outlined`, `text`, `size`), `default_focus` (`ConfirmFocus`), `close_on_escape`, `dismissable_mask`, `block_scroll`, `closable`, `position`, `modal` — all the fields that `Confirmation` in PrimeNG has, so that ConfirmDialog does not change the description.
  - `ConfirmationService` (a `Copy` handle in the Leptos context): `confirm`, `close`, `subscribe`/`unsubscribe`; `provide_confirmation_service` and `use_confirmation_service`.
- A new crate `prime-confirmpopup` (a port of `p-confirmpopup`).
- `ConfirmPopup`: listens to the service; a confirmation with a key matching the `key` of the component shows the panel `div.p-confirmpopup.p-component[role=alertdialog]` at `target` > `div.p-confirmpopup-content` (the icon `p-confirmpopup-icon`, the text `p-confirmpopup-message`) and `div.p-confirmpopup-footer` with the buttons "No" (`p-confirmpopup-reject-button`) and "Yes" (`p-confirmpopup-accept-button`), both of size `small`; the default labels are the translations `reject` and `accept`.
- Behaviour: alignment to the target with flipping above the target and a shift of the arrow (`--p-confirmpopup-arrow-left`, `p-confirm-popup-flipped`, `data-p-confirmpopup-flipped`); the layer of the overlay group; focus on the button by `default_focus` (by default "Yes"); `accept` and `reject` call the callback, hide the panel and return the focus to the target; closing by `Escape` (as "No"), by a click outside the panel and the target (`dismissable_mask`), by `resize` of the window (not on touch devices), by scrolling of an ancestor of the target; a repeated `confirm` retargets the panel.
- Properties: `key`, `default_focus` (`accept`), `auto_z_index`, `base_z_index`, `visible` (a controlled showing), `style`, `style_class`, `pt`; the views `content`, `headless`, `accept_icon`, `reject_icon`; the events `on_show`, `on_hide`.
- The showcase page `/confirmpopup` with the demos `basic`, `template` (content), `headless`; e2e; benchmarks and budgets; `docs/confirmpopup.md`, `docs/confirmation.md`.
- Styles: `confirmpopup` is added to `theme.components`.
- **Not included:** `appendTo` (the panel is always in `body` through `Portal`), `showTransitionOptions`/`hideTransitionOptions`/`motionOptions` (the animation is the theme classes), `Bind`, `pTemplate`, the icon classes `pi pi-…` (the icons are `ViewFn`), `ConfirmationService.onAccept` (only PrimeNG's ConfirmDialog listens to it; it will be added when needed, together with it).

## Capabilities

### New Capabilities
- `confirmation`: the description of a confirmation and the shared confirmation service.
- `confirmpopup`: a popup confirmation at the target element.

### Modified Capabilities
<!-- None. -->

## Impact

- New crates `crates/prime-confirmation` (native tests only) and `crates/prime-confirmpopup` (`publish = false`); dependencies: `prime-core`, `prime-dom`, `prime-button`, `prime-confirmation`, `web-sys`.
- Changes to `Cargo.toml` of the workspace, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml` (the browser job, three places: `prime-confirmpopup`).
