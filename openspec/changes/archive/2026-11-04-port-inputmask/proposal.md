# Proposal

## Why

InputMask — an input field with a mask (phone, date, serial number): the user types only allowed characters, and the constant characters of the mask are inserted automatically. This is the last component of wave 5; the mask engine is a self-contained pure logic (the buffer, the checking of characters, the shifts on input and deletion), and the shell is an ordinary `InputText` field with an optional clear icon.

## What Changes

- A new crate `prime-inputmask` (a port of `p-inputmask` from PrimeNG 21.1.10).
- Mask: `9` — a digit, `a` — a letter (`character_pattern`, by default ASCII letters), `*` — a letter or a digit, `?` — the start of an optional part; the other characters are constant. Examples: `(999) 999-9999`, `99-aaa?9`.
- `InputMask`: `span.p-inputmask.p-component.p-inputwrapper` (`p-variant-filled`) > `input.p-inputtext.p-component` (`type`, `placeholder`, `readonly`, `disabled`, `aria-*`, `title`, `autocomplete`) and the clear icon `p-inputmask-clear-icon` (with `show_clear`, a non-empty value and an available field). The value is `RwSignal<String>`: the masked text or only the entered characters when `unmask`.
- Behaviour: on focus the field shows the mask buffer with the placeholder characters `slot_char` (`_`), the caret goes to the first free position (if the mask is completely filled, everything is selected); printable characters are checked against the mask, inserted with a shift to the right and skip the constant characters; `Backspace`/`Delete` delete a character or the selection with a shift to the left; `Enter` completes the input; `Escape` returns the text as of the moment of focus; paste and input through `input` redistribute the characters over the mask; on losing focus an incomplete value is cleared (`auto_clear`) or kept (`keep_buffer` — the buffer is preserved); when the required part is filled, `on_complete` is called.
- Properties: `mask`, `value`, `slot_char`, `auto_clear`, `show_clear`, `unmask`, `character_pattern`, `keep_buffer`, `input_type`, `placeholder`, `readonly`, `disabled`, `invalid`, `variant`, `size`, `fluid`, `input_id`, `tabindex`, `title`, `aria_label`, `aria_labelledby`, `aria_required`, `autocomplete`, `id`, `style`, `style_class`; the events `on_complete`, `on_focus`, `on_blur`, `on_input`, `on_clear`, `on_unmasked_change`.
- The page `/inputmask` (`basic`, `mask`, `optional`, `slotchar`, `unmask`, `clearicon`, `disabled`, `invalid`), e2e, benchmarks and budgets, `docs/inputmask.md`, `inputmask` in `theme.components` and `primeng_extra`; `xtask gen-theme` supports components without a module in `@primeuix/styles` (only the `For PrimeNG` block).
- **Not included:** input on Android Chrome through the `input` event (`handleAndroidInput`) — the common parsing of `input` is used instead; `keypress` (`keydown` is used); Angular forms; `Bind`, `pTemplate`, the template of the clear icon; the `iPhone` Backspace branch.

## Capabilities

### New Capabilities
- `inputmask`: an input field with a mask.

### Modified Capabilities
<!-- None. -->

## Impact

- A new crate `crates/prime-inputmask` (`publish = false`); dependencies: `prime-core`, `prime-dom`, `prime-inputtext` (the classes), `prime-icons` (times).
- Changes to `Cargo.toml` of the workspace, `upstream.toml`, `xtask/theme/extract.mjs` (a component without a styles module), `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml`.
