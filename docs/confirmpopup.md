# ConfirmPopup

A port of `p-confirmpopup` from PrimeNG 21.1.10 (commit `fe82b291d6`, see `upstream.toml`). The shared service and the confirmation description are in `docs/confirmation.md`.

| Our component | PrimeNG sources |
|---|---|
| `prime-confirmpopup::ConfirmPopup`, `ConfirmPopupPt`, `ConfirmPopupHeadless` | `packages/primeng/src/confirmpopup/confirmpopup.ts`, `confirmpopup/style/confirmpopupstyle.ts` |
| CSS | `@primeuix/styles/confirmpopup` and the Aura preset: `cargo xtask gen-theme` (there is no `/* For PrimeNG */` block); the buttons take the `button` styles |
| Demos | `apps/showcase/doc/confirmpopup/*-doc.ts` (`basic`, `template`, `headless`) |

Spec: `openspec/specs/confirmpopup`.

```rust
let confirm = provide_confirmation_service();
view! {
    <ConfirmPopup />
    <Button label="Delete" on_click=move |e| confirm.confirm(
        Confirmation::new().target_event(&e).message("Delete this record?").accept(Callback::new(|()| { /* ... */ })),
    ) />
}
```

## What matches PrimeNG

- Markup and classes: `div.p-confirmpopup.p-component[role=alertdialog]` > `div.p-confirmpopup-content` (icon `p-confirmpopup-icon`, text `p-confirmpopup-message`) and `div.p-confirmpopup-footer` with the `p-confirmpopup-reject-button` and `p-confirmpopup-accept-button` buttons, both of size `small`; default labels are the `reject` and `accept` translations.
- Shown only for confirmations with its own `key`; the panel is aligned to `target` in `body` (`AlignMode::Absolute`), overlay group layer, the `--p-confirmpopup-arrow-left` arrow, flip above the target (`data-p-confirmpopup-flipped`, `p-confirm-popup-flipped`).
- Focus on show: the confirmation `default_focus`, otherwise the component property (`Accept`); `accept` and `reject` call the callback, hide the panel and return focus to the target. `Escape` = reject (unless `close_on_escape` is `false`); a click outside the panel and target, window `resize` (not on touch devices) and scrolling of a target ancestor hide the panel (the click only if `dismissable_mask` is not `false`). Listeners are attached only while shown, the click on the next tick.
- `content` replaces the content block, `headless` the whole panel, `accept_icon`/`reject_icon` the button icons; `visible` controls the show from outside.

## Differences

| Difference | Reason |
|---|---|
| The `p-confirmpopup-flipped` class is added to the flip class as well | In PrimeNG the class is called `p-confirm-popup-flipped`, while the theme rules are `.p-confirmpopup-flipped`, so in PrimeNG the flipped arrow and offset are not styled. We kept both names: the original and the one that has rules. |
| No `appendTo`: the panel is always in `body` (via `Portal`) | This is how the project other overlays are rendered. The `Portal` container is a service `div` in `body`. |
| No `showTransitionOptions`, `hideTransitionOptions`, `motionOptions` and leave animation: the panel is removed immediately on hide | The theme defines no separate leave classes; enter has no animation (the `p-anchored-overlay-*` rule is not attached). |
| `Confirmation` is a builder with `Callback` instead of the `acceptEvent`/`rejectEvent` emitters; icons are `ViewFn`, not `pi pi-…` classes | No `EventEmitter` and PrimeIcons. |
| A repeated `confirm` while the panel is visible hides it and shows it again at the new target, so `on_hide` and `on_show` are called | In Angular the panel is realigned in place; in Leptos recreating the content is simpler and more reliable. |
| No button `pt` (`pcAcceptButton`, `pcRejectButton`) and `Bind`; button style classes go through `accept_button_style_class`/`reject_button_style_class` | `pt` is simple additions of classes, style and attributes. |
| `#cp` with `cp.onAccept()`/`cp.onReject()` calls in `headless` is replaced with a context with the `accept` and `reject` callbacks | Leptos has no template references to the component. |
| PrimeIcons icons (`pi-exclamation-triangle`, `pi-info-circle`) in the demos are replaced with `prime-icons` icons | No PrimeIcons. |

## Not verified

- Right-to-left layout and touch input.
- Screen reading: `aria-*` were checked against PrimeNG (`role=alertdialog`), screen reader behavior was not.
- Position with several panels with different keys at the same time: only verified that a foreign key is ignored.

## Performance

See `benches/` (`confirmpopup-1`, `confirmpopup-1000`, `case-confirmpopup`) and `benches/*budgets.toml`. There is no panel on the server; the measurement is the cost of the service subscription.
