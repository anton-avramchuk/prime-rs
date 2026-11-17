# Proposal

## Why

`InputOtp` is the next wave 2 component: entering a one-time code with a cell per character, with auto-advance, pasting of the whole code and a mask. It builds on the existing `InputText` (classes and variant) and needs no overlays, so it fits as the first interactive batch after the display components.

## What Changes

- New crate `prime-inputotp` with the `InputOtp` component:
  - Root `<div class="p-inputotp p-component">` (instead of the `<p-inputotp>` host) and `length` cells `<input class="p-inputtext p-component p-inputotp-input …">` (`InputText` classes: `p-filled`, `p-inputtext-sm|lg`, `p-invalid`, `p-variant-filled`).
  - The value is an optional `RwSignal<String>`; the cell characters are its characters in order.
  - Properties: `value`, `length` (default 4), `mask` (`type="password"`), `integer_only` (`inputmode="numeric"`, digits only), `size`, `variant`, `invalid`, `disabled`, `readonly`, `autofocus` (first cell), `tabindex`, `name`, `style_class`, `style`, `pt` (`root`, `input`); events `on_change` (new value), `on_focus`, `on_blur`; the cell template `input` is a callback with the token, index and event handlers.
  - Behavior as in PrimeNG: typing a character moves to the next cell, `Backspace` on an empty cell and `ArrowLeft` move to the previous one, `ArrowRight` to the next, `ArrowUp`/`ArrowDown` are cancelled, paste is distributed across the cells (trimmed to `length`), content is selected on focus, typing is limited to digits with `integer_only`, fields accept no input with `readonly` and `disabled`.
  - Pure logic (tokens, key and paste decisions) is extracted to `state.rs` and verified by table-driven tests.
- Showcase page `/inputotp` with the PrimeNG demos `basic`, `mask`, `integeronly`, `sizes`, `template`; e2e with real input, paste and keys; benchmarks and budgets; `docs/inputotp.md`.
- Styles: `inputotp` is added to `theme.components`; `inputtext` is already enabled.
- **Out of scope:** Angular Forms integration (`ngModel`, `ReactiveForms`); the `sample` demo (it relies on buttons and a resend counter, a composite page); on-screen keyboard input and SMS code autofill (`autocomplete="one-time-code"` is set, but browser behavior was not verified).

## Capabilities

### New Capabilities
- `inputotp`: one-time code entry by characters in separate cells.

### Modified Capabilities
<!-- None. -->

## Impact

- New crate `crates/prime-inputotp` (`publish = false`); dependencies: `prime-core`, `prime-inputtext` (cell classes), `web-sys` (`HtmlInputElement`, `KeyboardEvent`, `ClipboardEvent`, `DataTransfer`, `InputEvent`, `FocusEvent`, `Element`).
- Changed: workspace `Cargo.toml`, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, and with browser tests `tests/web.rs` a job in `ci.yml` (three places).
