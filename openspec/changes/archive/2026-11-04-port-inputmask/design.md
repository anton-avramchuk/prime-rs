# Design

## Context

The requirements are in `specs/inputmask`. PrimeNG 21.1.10 sources: `inputmask/inputmask.ts` (`initMask`, `checkVal`, `shiftL`/`shiftR`, `onKeyPress`, `onInputKeydown`, `onInputFocus`/`Blur`), `style/inputmaskstyle.ts` (the `@primeuix/styles` package has no `inputmask` module — only the `For PrimeNG` block). Templates — `prime-inputnumber` (a field on the `InputText` classes, input via `keydown`), `prime-password`.

## Goals / Non-Goals

**Goals:** the same behaviour of the mask engine and the same DOM.

**Non-Goals:** the Android Chrome branch (`handleAndroidInput`), the iPhone Backspace, `keypress`, Angular forms.

## Decisions

### D1. Pure engine (`mask.rs`)
`Mask` stores `tests: Vec<Option<Kind>>` (`Digit`, `Alpha`, `AlphaNum`), `len`, `partial_position`, `first_non_mask`, `last_required`, a `Vec<char>` buffer, `default_buffer`, `slot`, `auto_clear`, `keep_buffer` and the letter predicate. The methods without the DOM, each returns the new text and the caret: `check_val(text, allow) -> (text, pos)`, `key_press(ch, caret) -> Option<Typed { text, caret, completed }>`, `delete(caret, forward) -> (text, caret)`, `on_focus(text) -> (text, caret_begin, caret_end)`, `on_blur(text) -> text`, `unmasked()`, `is_completed()`, `seek_next/prev`, `shift_l/shift_r` — table tests on the PrimeNG formulas (`(999) 999-9999`, `99-aaa?9`, `*`, insertion with a shift, deletion, the optional part, `keep_buffer`).

### D2. Shell
The field is uncontrolled: the text is written by the engine (`input.set_value`), `value: RwSignal<String>` is the result (`updateModel`). The caret (`set_selection_range`) — only in the browser. Input — `keydown` for printable characters (with `preventDefault`), `Backspace`/`Delete`/`Enter`/`Escape` as in the source, `input` and `paste` — through `check_val(true)` in `set_timeout(0)`. An external change of `value` (an effect under `cfg(wasm32)`) recomputes the text through `check_val`.

### D3. SSR
The initial text is `check_val(value)` without the DOM; in the `value` attribute.

### D4. Theme
`xtask/theme/extract.mjs` skips a missing module `@primeuix/styles/<name>` if the component has a `For PrimeNG` block.

### D5. Tests
The engine — tables. SSR: markup, the initial value, clearing, classes. Browser `tests/web.rs` as one sequential test: typing digits and unsuitable characters, `Backspace`/`Delete` with a shift, paste, `on_complete`, losing focus (clearing/`keep_buffer`/`auto_clear = false`), `unmask`, `readonly`, the external value, clearing.

## Risks / Trade-offs

- [The caret in the browser] The caret position is checked through `selectionStart`; a real focus in headless Chrome is needed.
- [Input via IME and Android] Covered by the common parsing of `input`, but not checked on devices.

## Open Questions

- Whether `keypress` is needed for compatibility with old keyboards — no, `keydown` with `key` covers it.

## Migration Plan

No migration. Rollback — `git revert`.
