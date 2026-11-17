# Design

## Context

Requirements are in `specs/inputnumber`. PrimeNG 21.1.10 sources: `packages/primeng/src/inputnumber/inputnumber.ts` (1493 lines), `inputnumber/style/inputnumberstyle.ts`, the demos in `apps/showcase/doc/inputnumber`. Models: `crates/prime-inputtext` (classes and `data-p` of the inner field, `Variant`/`Size`, config), `crates/prime-knob` and `crates/prime-slider` (listeners and timers from handlers, `*_untracked`, rules without recreating elements), `crates/prime-selectbutton` (Effect only under `cfg(wasm32)`). User decision: formatting is our own, on a small table of locales and currencies.

Observations about the source that affect the decisions:
- The whole format is built on `Intl.NumberFormat` and regular expressions over its output; the browser's output is unavailable on the server, so the formatter and parser are needed in Rust.
- Input is implemented as "surgery" on the formatted string in `keypress`/`keydown` with separate branches for the caret (`initCursor`, `updateInput`, `insert`, `deleteRange`, `concatValues`); `keypress` is deprecated and does not work on mobile keyboards.
- `onInputKeyDown` for `Home`/`End` checks `if (this.min())`, i.e. it skips `min = 0`.
- The `p-disabled` classes on the buttons depend on `maxlength()` instead of the value, i.e. are effectively never enabled.
- The value shape is `number | null`; `'-'` during parsing is a service marker meaning "only a minus was entered".
- `formattedValue()` and the field are linked via `[value]`, but while typing the DOM is edited directly (text like "1." is not equal to `format(1)`).

## Decisions

### D1. Value and displayed text
`value: Option<RwSignal<Option<f64>>>`, without it an internal signal with `default_value`. Separately there is `display: RwSignal<String>`, what is shown in the field; it is bound to `prop:value` and is the single source of the text. Edits while typing write the text directly to the DOM (together with the caret position) and then to `display`, while an external change of `value` (the signal is changed from outside) reformats `display` via an `Effect` under `cfg(target_arch = "wasm32")`: if the parse of `display` already equals the value, the text is left alone (a typed "1." stays). In SSR the text is `format(value)` once.

### D2. Formatter, locales, currencies (`locale.rs`, `format.rs`)
`LocaleData { decimal, group, minus, grouping: Standard | Indian, currency_position, currency_gap }` for `en-US`, `en-GB`, `en-IN`, `de-DE`, `fr-FR` (group is the narrow no-break space `U+202F`), `ja-JP`, `ru-RU` (group is `U+00A0`); `resolve_locale(tag)` is an exact match, then by language, otherwise `en-US`; an empty string and non-existent tags like `jp-JP` give `en-US`, as Intl does when a locale is unsupported. `CurrencyData { code, symbol, digits }` for USD, EUR, GBP, JPY, INR, RUB; other currencies get the code and 2 digits. `format_number(value, &FormatOptions) -> String`: rounding on the decimal representation of `f64` (the shortest representation from `Display`) by the half-away-from-zero rule; digit count: `min..max` (decimal: 0..3, currency: the currency's digits); grouping (3 or 3-2-2); the locale's minus sign before the currency symbol; currency prefix/suffix from the table; the user's `prefix`/`suffix` are added outside. All without allocations beyond the resulting strings and without dependencies.

### D3. Parsing (`parse_text`)
Following PrimeNG's steps: remove the suffix and prefix, spaces, the currency symbol, group separators, replace the minus sign with `-`, the decimal separator with `.`, parse `f64`; empty gives `None`, a lone `-` gives a separate `Minus` state (value `None` in the model, but "-" stays in the field). Digits are ASCII only (non-Latin numeral systems are not supported).

### D4. Text editing (`edit.rs`, pure functions)
`Edit { text, caret }`. Operations: `insert(text, sel, char, ctx)`, `delete_back`, `delete_forward`, `delete_range`, `paste`, `commit(text) -> (Option<f64>, text)`, `clamp_caret` (after `initCursor`: the caret does not enter the prefix and suffix). Rules are from PrimeNG: a minus only at the start and if there is no `min >= 0`, one decimal separator and only when `max_fraction_digits > 0`, fraction digits not above the maximum, deleting a fraction digit with a zero (if the digit count is not above `min_fraction_digits`), deleting the only integer digit gives "0" or empty, deleting across a group separator deletes the neighbouring digit. After an operation the text is obtained as follows: parse the new raw value, format it, keep the typed fraction "tail" (`concatValues`: "1.", "1.50", "0.0") and the suffix/currency. The caret is "after the same number of significant characters" (digits, decimal separator, minus) to the left of the caret before the edit; compared with PrimeNG's `updateInput` branches this gives the same result in ordinary cases and is easier to verify. Everything is covered by test tables.

### D5. Input events
Instead of `keypress`, `beforeinput`: `insertText` (by `data`), `deleteContentBackward`/`Forward`, `insertFromPaste`/`insertFromDrop` (by `data` or `dataTransfer`) are cancelled (`preventDefault`) and handled by the D4 operations; other types (`historyUndo` etc.) are cancelled. This path also works with mobile keyboards. `keydown` handles up and down arrows (step), `Enter`/`Tab` (commit), `Home`/`End` (bounds); `ArrowLeft`/`ArrowRight` are left alone (the browser moves the caret, and `click` brings it to the numeric part, `clamp_caret`). Keys with `Shift` and `Alt` are passed to the browser, as in PrimeNG. `Home`/`End` work with any set `min`/`max`, including 0. `blur` commits the value (`commit`), `focus` sets the focus flag for `p-inputwrapper-focus` and `data-p`.

### D6. Steps and buttons
`spin(dir)`: `current = parse(field).unwrap_or(0)`, `new = clamp(current + step·dir)`, refused when `max_length` is exceeded, write the text and value, the `on_input` event. Buttons: `mousedown` (first step and a 500 ms timer, then every 40 ms), `mouseup`/`mouseleave` (stop), `keydown` Enter/Space (step) and `keyup` (stop); the timer is `set_timeout_with_handle`, also stopped in `on_cleanup`. `readonly`/`disabled` block everything. The buttons do not get focus (`tabindex=-1`), `mousedown` cancels the default action so the field does not lose focus. The `p-disabled` class on the buttons is not emitted (as is effectively the case in PrimeNG).

### D7. Markup
The root is a `<span>`: `p-inputnumber p-component p-inputwrapper`, `p-inputwrapper-filled` (`display` is not empty or `allow_empty = false`), `p-inputwrapper-focus`, the layout, `p-inputnumber-fluid`, `p-invalid`, `style_class`, `pt.root`; `data-p` is `invalid disabled focus fluid filled empty <size> <layout>`. The inner field is `prime_inputtext` (`InputTextState`, `write_classes`, `data_p` for the `p-inputtext` classes), plus `p-inputnumber-input` and `input_style_class`. The clear icon is `TimesIcon` (or a `ViewFn`), the buttons are `AngleUpIcon`/`AngleDownIcon` (or a `ViewFn`). For `stacked` the buttons are in `span.p-inputnumber-button-group`, otherwise after the field; the vertical layout has no clear icon. The structure is built once by `show_buttons`/`button_layout`/`show_clear`; everything else is closures.

### D8. Attributes and defaults
The field: `role=spinbutton`, `inputmode=decimal`, `aria-valuemin/max/now`, `min`, `max`, `step` (`step ?? 1`) and the optional attributes, as in PrimeNG's template. `aria-valuenow` is the value (after each edit). `locale` defaults to `en-US` (PrimeNG uses the browser language; there is none on the server).

### D9. CSS
`inputnumber` in `theme.components` (`upstream.toml`), `cargo xtask gen-theme`; the `/* For PrimeNG */` block contains only Angular Forms' `.ng-invalid.ng-dirty` rules, so `primeng_extra` does not change. The `inputtext` CSS is already linked on the page.

### D10. Demos
`locale`, `currency`, `numerals`, `prefixsuffix`, `buttons`, `vertical`, `clearicon`, `sizes`, `filled`, `invalid`, `disabled`, `fluid` (twelve; `floatlabel`, `iftalabel` and forms are skipped). In `currency` "jp-JP" from the source becomes the correct `ja-JP` with a note. In `buttons` and `vertical` the plus and minus icons are `PlusIcon`/`MinusIcon` via `ViewFn`. For e2e some demos output an `echo` of the value.

### D11. Tests
Pure: the formatter (locales, currencies, rounding, fraction, groups), the parser (round trip: `parse(format(x)) == x` for a table of values and locales), edit operations (tables: typing, minus, separator, fraction limit, Backspace/Delete over separators, ranges, paste, caret). SSR: structure, classes and `data-p`, formats, buttons by layout, clear icon, attributes, `pt`. Browser (`tests/web.rs`): typing via `beforeinput` and caret position, Backspace/Delete, paste, commit on blur/Enter/Tab, bounds, arrows, buttons and repeat (by timer), `readonly`/`disabled`, clearing, external signal (including on `locale` change), "1." is not overwritten while typing. e2e: typing from a real keyboard (`type`, `Backspace`), arrows, buttons, currencies and locales.

## Risks / Trade-offs

- [The small table of locales and currencies differs from Intl: other locales and currencies, non-Latin digits, compact and percent modes] → recorded in the spec and `docs/inputnumber.md`; the table is extensible with data.
- [Caret and typing behaviour does not match PrimeNG character for character: `beforeinput` instead of `keypress`, caret by the number of significant characters] → pure functions with tables; differences are listed in the docs; e2e with a real keyboard.
- [Large scope: formatter, parser, editing, events, buttons, layouts] → tasks are split by layer, pure logic is verified before wiring to the DOM.
- [Render cost higher than InputText (two or three buttons, formatting)] → budget by measurement.

## Migration Plan

No migration: a new crate. Rollback is `git revert`.

## Open Questions

None.
