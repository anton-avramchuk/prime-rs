# InputNumber

Port of PrimeNG 21.1.10's `p-inputnumber` (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-inputnumber::InputNumber` | `packages/primeng/src/inputnumber/inputnumber.ts`, `inputnumber/style/inputnumberstyle.ts` |
| CSS | `@primeuix/styles/inputnumber` and the Aura preset (`cargo xtask gen-theme`); the field uses the `inputtext` CSS |
| Demos | `apps/showcase/doc/inputnumber/*-doc.ts` (`locale`, `currency`, `numerals`, `prefixsuffix`, `buttons`, `vertical`, `clearicon`, `sizes`, `filled`, `invalid`, `disabled`, `fluid`, `floatlabel`, `iftalabel`) |

Spec: `openspec/specs/inputnumber`.

## What matches PrimeNG

- Structure: root `span.p-inputnumber p-component p-inputwrapper`, inside it `input[role=spinbutton]` with the `InputText` and `p-inputnumber-input` classes, the clear icon `p-inputnumber-clear-icon`, the `p-inputnumber-button` buttons (`stacked` in `p-inputnumber-button-group`, `horizontal` and `vertical` separately); `tabindex=-1` and `aria-hidden` on the buttons.
- Root classes and `data-p` (`invalid`, `disabled`, `focus`, `fluid`, `filled`, `empty`, size, layout), `p-inputwrapper-filled` when `allow_empty = false`.
- Field attributes: `role=spinbutton`, `inputmode=decimal`, `aria-valuemin/max/now`, `min`, `max`, `step` (1 by default) and the optional ones.
- Typing rules: only digits, a minus (at the start of the number and when `min` is below zero) and the locale's decimal separator; the number of fraction digits is limited by the maximum; in the fraction part a digit replaces the next one; paste is parsed as a number.
- As in PrimeNG, a decimal separator can be typed into a number with no fraction part only when `max_fraction_digits` is set explicitly: by default (the "Integer Only" demo) the field accepts integers, and a fraction appears from the value, `min_fraction_digits` or paste.
- Backspace and Delete: skip the group separator, do not touch the prefix and suffix, replace a fraction digit with zero in `Currency` mode, remove it in `Decimal` when the digit count exceeds `min_fraction_digits`, and replace a lone integer digit with zero (a result not greater than zero becomes empty).
- While a number is typed without a decimal separator, `min_fraction_digits` pads the fraction with zeros (`1` → `1.00`); the typed fraction "tail" is kept (`1.`, `1.50`).
- Commit (`Enter`, `Tab`, focus loss) parses the text, clamps to `min..max` and formats again; an empty field is `None` (`0` when `allow_empty = false`).
- Steps: arrows and buttons; holding a button repeats the step after 500 ms every 40 ms; `maxlength` blocks a step that would make the text longer.
- Keys with `Shift` and `Alt` do not interfere with the browser and are not passed to `on_key_down`.

## Differences

| Difference | Reason |
|---|---|
| The root is a `<span>`, not the `<p-inputnumber>` host | The host in Angular is a framework detail. |
| `value: RwSignal<Option<f64>>` instead of `ngModel`; without it, internal state (`default_value`) | Two-way binding without forms infrastructure. |
| Own formatter on a table of seven locales (`en-US`, `en-GB`, `en-IN`, `de-DE`, `fr-FR`, `ja-JP`, `ru-RU`) and six currencies (USD, EUR, GBP, JPY, INR, RUB) instead of `Intl.NumberFormat`; an unknown locale is `en-US`, an unknown currency is its code | Rust has no Intl, and SSR and the browser must produce the same text (user decision); ICU4X would noticeably grow the wasm. Currency symbols and their positions in the table approximate CLDR. |
| `locale` defaults to `en-US`, not the browser language | There is no browser language on the server. |
| "jp-JP" from the `currency` demo is replaced with `ja-JP` | `jp-JP` is not an existing locale; Intl silently falls back to the default locale. |
| Rounding half away from zero on the shortest decimal representation (`1.005` → `1.01`) | ICU and V8 do this. |
| Input via `beforeinput` instead of `keypress` and string parsing by `updateInput` branches | `keypress` is deprecated and does not work on mobile keyboards. |
| After an edit the caret is placed "after the same number of significant characters" (digits, separator) | Easier to verify; in ordinary cases it matches PrimeNG, in rare ones (paste in the middle with groups) it may differ by one character. |
| A minus is also accepted as ASCII `-` in locales with a different sign (`ru-RU`: `U+2212`) | In PrimeNG one had to type the locale's sign. |
| The decimal separator is only the locale's sign (`,` in `de-DE`, `.` is not accepted) | The dot in `de-DE` is the group separator; PrimeNG only adds the numeric `NumpadDecimal` key. |
| `Home`/`End` also work when `min`/`max` equals zero | In PrimeNG the `if (this.min())` check skips zero. |
| A step rounds binary arithmetic noise to 12 digits (`0.1 + 0.2` → `0.3`) | Otherwise the value gets the tail `0.30000000000000004`. |
| `on_input` receives the text after the edit | In PrimeNG `formattedValue` is the text before the edit. |
| The `p-disabled` class on buttons is not emitted | In PrimeNG it depends on `maxlength()` instead of the value and is effectively never enabled. |
| ASCII digits only; no compact, percent or unit modes, `minlength`, `localeMatcher`, `autofocus`, Angular Forms | Out of scope of the change. |
| The `reactiveforms` and `templatedrivenforms` demos are not ported | Forms are out of scope. |
| The `fluid` property is `OptProp<bool>`: without a value it comes from the `Fluid` context | Like `hasFluid = fluid() ?? !!pcFluid` in PrimeNG; see `docs/field-wrappers.md`. |
| `readonly` and `disabled` equally block input, steps and paste | A disabled field gets no events from the browser; the block is needed for synthetic events and buttons. |

Prefixes and suffixes with digits (for example, `prefix="3D "`) are not supported: digits in the surrounding text are taken as part of the number when finding the bounds of the numeric part.

## Performance

SSR: 119 allocations per instance with a currency and buttons (budget 130), HTML ≈ 2.3 KB (two button icons); contribution to raw wasm +437 KB (budget 480 KB) — the formatter, parsing and text editing.
