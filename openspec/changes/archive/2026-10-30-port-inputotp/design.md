# Design

## Context

Requirements are in `specs/inputotp`. PrimeNG 21.1.10 sources: `inputotp/inputotp.ts`, `inputotp/style/inputotpstyle.ts`; demos `apps/showcase/doc/inputotp/{basic,mask,integeronly,sizes,template}-doc.ts`. Reference implementations in the project: `crates/prime-inputtext` (value model `RwSignal<String>`, `write_classes`/`InputTextState`), `crates/prime-rating` and `prime-slider` (focus and keys, `Effect` only under `cfg(wasm32)`), `prime_rs_core::Pt`.

Observations about the source:
- State is an array of tokens; `updateModel` joins it into a string. A single cell reads its token by index.
- `onInput(event, index)`: if a string longer than one character lands in the first cell, it is a paste/autofill (`handleOnPaste`); otherwise write the token and by `inputType` move back (`deleteContentBackward`) or forward (`insertText`, `deleteContentForward`).
- `onKeyDown`: combinations with Alt/Ctrl/Meta are skipped; `ArrowLeft/Right` move to the neighboring cell, `ArrowUp/Down` cancel; `Backspace` in an empty cell goes back; otherwise: with `integerOnly` a non-digit cancels, with `length` characters filled and no selection it cancels (except `Delete`).
- `handleOnPaste`: takes `length + 1` characters (in PrimeNG this has no effect, since the tokens are later trimmed by value), with `integerOnly` requires numbers. We trim strictly to `length`.
- Style is only two classes: `p-inputotp p-component` and `p-inputotp-input`; there is no `/* For PrimeNG */` block.

## Decisions

### D1. Crate and dependencies
`prime-inputotp` with `InputOtp`, `InputOtpPt { root, input }`, `InputOtpSlot { token, index, events }`, `InputOtpEvents`. Depends on `prime-rs-core`, `prime-inputtext` (for `write_classes`/`InputTextState`) and `web-sys`. The cells themselves are drawn as `<input>` with InputText classes plus `p-inputotp-input`, not as a nested `InputText`: the cell shares a value, and the handlers it needs are different.

### D2. Pure logic (`state.rs`)
- `tokens_of(value: &str, length) -> Vec<String>`: characters of the value in order, trimmed to `length`.
- `join(tokens) -> String`.
- `key_decision(key, mods, integer_only, has_selection, filled, length, cell_empty) -> KeyAction` (`Allow`, `Prevent`, `Prev`, `Next`) after `onKeyDown`.
- `paste_value(text, length, integer_only) -> Option<String>`: trimmed to `length`; `None` if `integer_only` and the text is not only digits.
- `input_move(input_type) -> Move { Prev, Next, None }`.
All with table-driven tests.

### D3. Value
`value: Option<RwSignal<String>>`. Tokens are computed from the signal (with `Some`) or live in a local `RwSignal<Vec<String>>` (without `value`, uncontrolled mode). The write to `value` is the joined string of tokens; empty cells in the middle of the code are kept as empty tokens in the calculation, but the join skips them (like `tokens.join('')` in PrimeNG).

### D4. Cells and events
A cell is an `<input>` with attributes per the spec and `prop:value` from the token. Handlers (`on:input`, `on:keydown`, `on:focus`, `on:blur`, `on:paste`) are created once per cell; focus on a neighboring cell goes through `Element::next_element_sibling`/`previous_element_sibling` and `HtmlInputElement::focus`/`select`. Everything that touches the DOM is under `cfg(target_arch = "wasm32")` or in handlers (which are not called on SSR). `autofocus` is an attribute of the first cell. For the `input` template the handlers are wrapped in `InputOtpEvents` (`Callback<web_sys::Event>` and so on).

### D5. CSS
`inputotp` is added to `theme.components`; `cargo xtask gen-theme`, check that the CSS of the others did not change.

### D6. Tests
SSR for each markup and value scenario; table-driven `state.rs` tests; browser tests `tests/web.rs` (wasm-bindgen-test) for input, auto-advance, `Backspace`, arrows, paste, `integer_only`; e2e with real keys and `clipboard` paste. If the browser tests require a new job, add the crate in three places of `ci.yml`.

### D7. Demos
`/inputotp`: `basic`, `mask`, `integeronly`, `sizes`, `template` (custom cell). The form demos (`reactiveforms`, `templatedrivenforms`) and `sample` are not ported.

### D8. Benchmarks
Scenarios `inputotp-1` and `-1000`, `case-inputotp`; first `cargo bench -p render-bench --bench alloc`, then the budgets.

## Risks / Trade-offs

- [SMS autofill and on-screen keyboard] → behave differently across browsers; only paste and keys are verified, the rest is honestly recorded in `docs/inputotp.md`.
- [Focus via DOM neighbors] → as in PrimeNG; the `input` template must output `<input>` as neighboring elements.
- [Uncontrolled mode] → tokens in a local signal; without `value` the result is read only via `on_change`.

## Migration Plan

No migration. Rollback is `git revert`.

## Open Questions

None.
