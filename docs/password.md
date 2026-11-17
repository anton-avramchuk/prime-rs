# Password

Port of `p-password` from PrimeNG 21.1.10 (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-password::Password`, `PasswordStrength`, `PasswordPt` | `password/password.ts`, `password/style/passwordstyle.ts` |
| CSS | `@primeuix/styles/password` and the `/* For PrimeNG */` block (animations, `min-width`) — `cargo xtask gen-theme` |
| Demos | `apps/showcase/doc/password/*-doc.ts` |

Spec: `openspec/specs/password`.

## What matches PrimeNG

- Markup and classes: `span.p-password.p-component.p-inputwrapper` (`p-inputwrapper-filled`, `-focus`, `p-variant-filled`, `p-password-fluid`) > `InputText` field with the class `p-password-input`, icons `p-password-clear-icon`, `p-password-unmask-icon` / `p-password-mask-icon`, panel `p-password-overlay` > `p-password-content` > `p-password-meter` > `p-password-meter-label` (with `p-password-meter-weak|medium|strong` and width 33.33 / 66.66 / 100 %) and `p-password-meter-text`.
- The default estimate equals the PrimeNG regular expressions `mediumRegex` and `strongRegex`: medium — 6+ characters and two kinds of lowercase, uppercase Latin letters and digits; strong — 8+ characters and all three; otherwise a non-empty string is weak.
- The panel opens on focus and input, closes on `Escape`, focus loss and window `resize`; it is aligned below the field, z-index from the `Overlay` group; appearing and disappearing use the classes `p-password-enter` / `p-password-leave` (300 ms).
- Labels come from the locale (`passwordPrompt`, `weak`, `medium`, `strong`) unless set by properties.

## Differences

| Difference | Reason |
|---|---|
| The root is a `<span>` without the `<p-password>` host; the `pPassword` directive is not ported | The host in Angular is a framework detail; for a field without a wrapper there is `InputText`. |
| `strength: Callback<String, Option<PasswordStrength>>` instead of `mediumRegex` / `strongRegex` | `regex` does not support lookahead, needed by the default expressions; the built-in estimate is equivalent to them. |
| The clear icon is shown for a non-empty string | In PrimeNG — for any value except `null`, including an empty string. |
| Only `appendTo = self`; no `overlayOptions`, `motionOptions`, `showTransitionOptions` / `hideTransitionOptions` | Tied to the Angular `Overlay`; the panel is always inside the root. |
| No `tabindex`, `aria-labelledby`, `autofocus` of the field | `InputText` does not have them. |
| Templates `header`, `footer`, `content`, `show_icon`, `hide_icon`, `clear_icon` are `ViewFn`, without the `{ class }` context | Instead of `ng-template`. |
| Scrolling ancestors does not close the panel | In PrimeNG `ConnectedOverlayScrollHandler` closes it; we only have window `resize`. |
| No Angular Forms bindings, `onClear` is `on_clear` | The value model is `RwSignal<String>`. |
| The `reactiveforms`, `templatedrivenforms` demos are not ported; `sizes` has no `type="text"` | Tied to Angular Forms. |

## Cost

103 allocations per instance in SSR (with `toggle_mask`, panel closed); contribution to wasm ≈ 483 KB (budget 540 KB in `benches/budgets.toml`; mostly the shared `InputText`, icons and `align_overlay`).

## Not verified

Touch and screen readers: markup, levels, the `Escape` key, show password, clear and panel position were verified in Chrome (e2e and browser tests). `aria-live` was not added to the hint — PrimeNG does not have it either.
