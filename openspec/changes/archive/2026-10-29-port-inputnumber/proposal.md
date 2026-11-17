# Proposal

## Why

InputNumber is a numeric input field with formatting (group separators, fraction part, currency, prefix and suffix), bounds, a step and increment/decrement buttons. It is the most complex of the remaining simple fields and a basis for forms; the port follows `port-knob` and uses `prime-inputtext` for the inner field.

## What Changes

- A new crate `prime-inputnumber` with the `InputNumber` component: a two-way `value` (`RwSignal<Option<f64>>`) or internal state with `default_value`; the `Decimal` and `Currency` modes (`currency`, `currency_display` symbol or code), `locale`, `use_grouping`, `min_fraction_digits`, `max_fraction_digits`, `format`, `prefix`, `suffix`, `min`, `max`, `step` (1), `allow_empty`, `max_length`; `show_buttons` buttons with the `Stacked`, `Horizontal`, `Vertical` layouts; `show_clear`; `readonly`, `disabled`, `invalid`, `required`, `variant`, `size`, `fluid`; `placeholder`, `input_id`, `name`, `tabindex`, `title`, `autocomplete`, `aria_label`, `aria_labelledby`, `aria_describedby`, `aria_required`, `input_style`, `input_style_class`, `style`, `style_class`, `increment_button_class`, `decrement_button_class`; button and clear icons are `ViewFn`; the events `on_input` (value and formatted text), `on_focus`, `on_blur`, `on_key_down`, `on_clear`; `pt`.
- Number formatting and parsing are our own, on a small table of locales (`en-US`, `en-GB`, `en-IN` with 3-2-2 groups, `de-DE`, `fr-FR`, `ja-JP`, `ru-RU`; an unknown locale is `en-US`) and currencies (USD, EUR, GBP, JPY, INR, RUB; others by code). The same text in SSR and in the browser, no new dependencies; half-away-from-zero rounding on the shortest decimal representation, like ICU.
- Markup as in PrimeNG 21.1.10 (`inputnumber.ts`, `inputnumberstyle.ts`): root `<span class="p-inputnumber p-component p-inputwrapper">` with `data-p`, inside it `<input role="spinbutton" class="p-inputtext p-component p-inputnumber-input" inputmode="decimal">`, a clear icon, a button group `p-inputnumber-button-group` (stacked) or separate `p-inputnumber-button` buttons.
- Input as in PrimeNG: only digits, the minus sign and the decimal separator are allowed; the text is reformatted on the fly preserving the caret position; Backspace and Delete respect separators and the fraction part; paste from the clipboard; up and down arrows, buttons (with repeat while held), `Enter`/`Tab`/focus loss commit the value clamped to `min..max`; `Home`/`End` set the bounds.
- The `/inputnumber` page (`locale`, `currency`, `numerals`, `prefixsuffix`, `buttons`, `vertical`, `clearicon`, `sizes`, `filled`, `invalid`, `disabled`, `fluid`), e2e, browser tests, benchmarks, budgets, `docs/inputnumber.md`, the `inputnumber` theme CSS.
- **Out of scope:** Angular Forms (`ngModel`, reactive forms, `updateOn: blur`); `autofocus`; full Intl (the remaining locales and currencies, `localeMatcher`, non-Latin digits, compact and percent modes, `minlength`); the `reactiveforms`, `templatedrivenforms`, `floatlabel`, `iftalabel` demos (FloatLabel and IftaLabel are not ported); the `/* For PrimeNG */` block (the `.ng-invalid.ng-dirty` rules).

## Capabilities

### New Capabilities
- `inputnumber`: the InputNumber component: value, formatting and parsing, keyboard and clipboard input, buttons and repeat, bounds, structure and accessibility, `pt`, performance.

### Modified Capabilities
<!-- None. -->

## Impact

- A new crate `crates/prime-inputnumber` (`publish = false`); dependencies `prime-rs-core`, `prime-inputtext` (classes of the inner field), `prime-icons` (`angleup`, `angledown`, `times`).
- Changed: workspace `Cargo.toml`, `upstream.toml` (`inputnumber` in `theme.components`), `showcase`, `benches/*`, `e2e/tests`, `ci.yml` (browser job for `tests/web.rs`).
