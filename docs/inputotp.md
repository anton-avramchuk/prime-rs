# InputOtp

Port of `p-inputotp` from PrimeNG 21.1.10 (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-inputotp::InputOtp` | `packages/primeng/src/inputotp/inputotp.ts`, `inputotp/style/inputotpstyle.ts` |
| CSS | `@primeuix/styles/inputotp` and the Aura preset (`cargo xtask gen-theme`) |
| Demos | `apps/showcase/doc/inputotp/*-doc.ts` (`basic`, `mask`, `integeronly`, `sizes`, `template`) |

Spec: `openspec/specs/inputotp`.

## What matches PrimeNG

- Structure: root `div.p-inputotp.p-component`, containing `length` (default 4) fields `input.p-inputtext.p-component.p-inputotp-input`; classes `p-inputtext-sm|lg`, `p-invalid`, `p-variant-filled` and `data-p` come from `prime-inputtext` (`write_classes`, `data_p`).
- `maxlength`: `length` on the first cell (it accepts paste and autofill), `1` on the others; `inputmode="numeric"` with `integer_only`, `type="password"` with `mask`.
- Key logic (`key_decision`): `ArrowLeft`/`ArrowRight` move to the neighboring cell, `ArrowUp`/`ArrowDown` are cancelled, `Backspace` in an empty cell goes back; with `integer_only` non-digits are cancelled; with `length` characters filled and no selection typing is cancelled (except `Delete`); combinations with `Alt`/`Ctrl`/`Meta` are not handled.
- Auto-advance by `inputType` (`input_move`): `deleteContentBackward` goes back, `insertText` and `deleteContentForward` go forward.
- On focus the cell content is selected. Paste distributes up to `length` characters starting from the first cell; several characters in the first cell (autofill) are handled as a paste.
- The `input` template receives the token, index and event handlers (`InputOtpEvents`); focus between cells moves across neighboring `<input>` elements.

## Differences

| Difference | Reason |
|---|---|
| The root is a `<div>`, not a `<p-inputotp>` host | The host in Angular is a framework detail. |
| `value: RwSignal<String>` instead of `ngModel`; without it the tokens are stored inside, the result arrives via `on_change` | Two-way binding without forms infrastructure. |
| Empty cells in the middle of the code are kept in local tokens, but `value` gets the join without them (`1_3` → `13`) | `tokens.join('')` joins the same way; but on an external change of `value` the cells are re-laid out. |
| Paste is cut strictly to `length`; with `integer_only` a non-digit paste is ignored entirely | PrimeNG takes `length + 1` characters but then trims by value, so the result is the same. |
| Focus between cells is searched among `<input>` neighbors, foreign template elements (separators) are skipped | PrimeNG walks `nextElementSibling` and loses focus on a separator. |
| Control keys (`Tab`, `Enter`, `Home`…) are not restricted with `integer_only` and a filled code | In PrimeNG the `default` branch cancels them too. |
| No `autofocus` via a directive, Angular Forms integration, `/* For PrimeNG */` block; the `reactiveforms`, `templatedrivenforms`, `sample` demos are not ported | Out of scope for the change; forms are Angular-specific. |
| In `template` a dash instead of the `pi-minus` icon | `prime-icons` has no such icon. |

The cell list is built by a closure over `length` and does not read the tokens: otherwise typing would recreate the fields and drop focus.

## What was not verified

SMS autofill, on-screen keyboards and IME input differ across browsers and devices; keys, paste (`ClipboardEvent` with `DataTransfer`) and `InputEvent` input were verified in Chromium. Behavior on real mobile devices was not verified.

## Performance

SSR: 209 allocations per instance with six cells (budget 215); contribution to raw wasm +280 KB (budget 283 KB).
