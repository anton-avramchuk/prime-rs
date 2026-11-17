# inputnumber Specification

## Purpose
InputNumber is a number input field: it shows the value in a locale format (digit groups, fraction part, currency, prefix and suffix), accepts only numeric input, clamps the value to bounds and lets it be changed in steps.

## Requirements

### Requirement: Value and formatting
The `InputNumber` component of the `prime-inputnumber` crate SHALL accept a two-way `value` (`RwSignal<Option<f64>>`); without it, it SHALL keep internal state starting from `default_value`. The field SHALL show the value formatted per `mode`, `locale`, `currency`, `currency_display`, `use_grouping`, `min_fraction_digits`, `max_fraction_digits`, `prefix` and `suffix`; an empty value SHALL be an empty string, and with `allow_empty = false`, `0`. The `Decimal` mode without explicit fraction bounds SHALL show 0 to 3 digits after the decimal point, the `Currency` mode the currency's digit count (2; 0 for JPY). With `format = false` the value SHALL be shown as a number without formatting. Excess digits SHALL be rounded half away from zero on the shortest decimal representation of the number. On an external change of the signal the text SHALL be updated.

#### Scenario: Groups and fraction
- **WHEN** `locale = "en-US"`, `min_fraction_digits = 2`, value `1234.5`
- **THEN** the text is "1,234.50"

#### Scenario: Rounding
- **WHEN** `max_fraction_digits = 2`, value `1.005`
- **THEN** the text is "1.01"

#### Scenario: No groups
- **WHEN** `use_grouping = false`, value `1234567`
- **THEN** the text is "1234567"

#### Scenario: Prefix and suffix
- **WHEN** `prefix = "Expires in "`, `suffix = " days"`, value `7`
- **THEN** the text is "Expires in 7 days"

#### Scenario: Empty value
- **WHEN** the value is `None`
- **THEN** the field is empty, and with `allow_empty = false` it shows "0"

### Requirement: Locales and currencies
Formatting SHALL support the locales `en-US`, `en-GB`, `en-IN` (3-2-2 groups), `de-DE`, `fr-FR`, `ja-JP` and `ru-RU` with their own decimal separator, group separator and minus sign; for a language without a region (`de`) the language's main locale SHALL be chosen, and an unknown or empty locale SHALL be `en-US`. In the `Currency` mode the currencies USD, EUR, GBP, JPY, INR and RUB SHALL be supported with the symbol and its position per locale (`$1,234.50`, `1.234,50 €`) and display by code (`currency_display = Code`, "INR 1,234.00"); for other currencies the code SHALL be output. The result SHALL be identical in SSR and in the browser.

#### Scenario: German locale
- **WHEN** `locale = "de-DE"`, `min_fraction_digits = 2`, value `1234.5`
- **THEN** the text is "1.234,50"

#### Scenario: Indian grouping
- **WHEN** `locale = "en-IN"`, value `1234567.891`
- **THEN** the text is "12,34,567.891"

#### Scenario: Currency
- **WHEN** `mode = Currency`, `currency = "USD"`, `locale = "en-US"`, value `-1234.5`
- **THEN** the text is "-$1,234.50"

#### Scenario: Currency as a suffix
- **WHEN** `mode = Currency`, `currency = "EUR"`, `locale = "de-DE"`, value `1234.5`
- **THEN** the text is "1.234,50 €" (with a no-break space before the symbol)

#### Scenario: Unknown locale
- **WHEN** `locale = "xx-YY"`
- **THEN** the `en-US` format is used

### Requirement: Text input
The field SHALL accept only digits, the minus sign (at the start of the number, once, unless `min >= 0` prevents it) and the locale's decimal separator (once and only if the format allows it); other characters SHALL be ignored. After each change the text SHALL be reformatted (groups, prefix, suffix, currency) while keeping the typed fraction part (for example "1.", "1.50"), and the caret SHALL stay after the same number of significant characters (digits, separator, minus). The number of digits after the decimal point SHALL be limited by the format maximum. Backspace and Delete SHALL delete a digit, skipping the group separator, SHALL NOT delete the prefix and suffix, and when deleting a selection the whole range; deleting a fraction digit SHALL replace it with zero if the digit count does not exceed `min_fraction_digits`, otherwise remove it. Paste from the clipboard SHALL parse the pasted text as a number and substitute its value. Every change of the value SHALL be written to the signal and call `on_input` with the value and the text.

#### Scenario: Typing digits
- **WHEN** "1", "2", "3", "4" are typed into an empty field
- **THEN** the text is "1,234", the value `1234`

#### Scenario: Ignored character
- **WHEN** the letter "a" is typed
- **THEN** the text and value do not change

#### Scenario: Decimal separator
- **WHEN** "." is typed after "1", then "5"
- **THEN** first the text is "1." and the value `1`, then "1.5" and the value `1.5`

#### Scenario: Fraction limit
- **WHEN** `max_fraction_digits = 2`, another digit is typed at the end of "1.25"
- **THEN** the text remains "1.25"

#### Scenario: Backspace over a separator
- **WHEN** the text is "1,234", the caret is right after the comma, Backspace is pressed
- **THEN** "1" is deleted, the text is "234"

#### Scenario: Paste
- **WHEN** "1,234.5" is pasted into an empty field with `locale = "en-US"`
- **THEN** the value is `1234.5`

### Requirement: Commit and bounds
On focus loss, `Enter` and `Tab` the text SHALL be parsed into a number, clamped to `min` and `max`, formatted again, and the value written to the signal; empty text or text consisting of a lone "-" SHALL give `None` (and with `allow_empty = false`, `0`). `Home` and `End` with `min` and `max` set SHALL set the bound and cancel the default action. A value beyond a bound entered by typing SHALL be clamped on commit.

#### Scenario: Focus loss
- **WHEN** `max = 100`, "250" is typed and focus is lost
- **THEN** the value is `100`, the text "100"

#### Scenario: Empty field
- **WHEN** the text is erased and focus is lost
- **THEN** the value is `None`, and with `allow_empty = false` it is `0`

#### Scenario: Bounds by keys
- **WHEN** `min = 0`, `max = 100`, `End` is pressed
- **THEN** the value is `100`

### Requirement: Steps and buttons
The up and down arrow keys SHALL increase and decrease the current value (parsed from the field, otherwise 0) by `step` (1 by default), clamp it to `min..=max`, cancel the default action and write the value. With `show_buttons`, increment and decrement buttons SHALL be output that change the value the same way: a mouse press makes a step immediately, and holding for 500 ms repeats the step every 40 ms until release or the pointer leaving; the Enter and Space keys on a button make a step. With `readonly` and `disabled` the arrows, buttons, typing and paste SHALL NOT change the value. If after a step the text length would exceed `max_length`, the step SHALL NOT be performed.

#### Scenario: Arrow
- **WHEN** the value is `5`, the up arrow is pressed
- **THEN** the value is `6`, the text "6"

#### Scenario: Step and bound
- **WHEN** `step = 0.25`, `max = 1`, value `0.9`, the up arrow is pressed
- **THEN** the value is `1`

#### Scenario: Button hold
- **WHEN** the increment button is held for longer than 500 ms
- **THEN** the value grows by a step every 40 ms, and after release the growth stops

#### Scenario: Read-only
- **WHEN** `readonly` is set and the up arrow is pressed
- **THEN** the value does not change

### Requirement: Structure and classes
The root SHALL be `<span class="p-inputnumber p-component p-inputwrapper">` with the modifiers `p-inputwrapper-filled` (the field is not empty or `allow_empty = false`), `p-inputwrapper-focus`, `p-inputnumber-stacked`, `p-inputnumber-horizontal`, `p-inputnumber-vertical` (with `show_buttons`, per `button_layout`), `p-inputnumber-fluid`, `p-invalid`, `style_class` and the `data-p` attribute (flags `invalid`, `disabled`, `focus`, `fluid`, `filled`, `empty`, size, button layout). Inside there SHALL be `input.p-inputtext.p-component.p-inputnumber-input` (with size, variant, `p-invalid`, `p-filled` classes), then the clear icon `p-inputnumber-clear-icon` (with `show_clear`, a non-empty value and a non-vertical layout) and the buttons: with `stacked` in `span.p-inputnumber-button-group`, otherwise separately; the buttons `p-inputnumber-button p-inputnumber-increment-button` and `p-inputnumber-decrement-button` with `tabindex="-1"`, `aria-hidden` and `disabled` when `disabled`. A click on the clear icon SHALL reset the value to `None` and call `on_clear`.

#### Scenario: Button stack
- **WHEN** `show_buttons`, `button_layout = Stacked`
- **THEN** the root has `p-inputnumber-stacked`, the buttons are inside `p-inputnumber-button-group`

#### Scenario: Horizontal layout
- **WHEN** `show_buttons`, `button_layout = Horizontal`
- **THEN** the root has `p-inputnumber-horizontal`, the buttons are outside the group

#### Scenario: Clearing
- **WHEN** `show_clear`, value `5` and a click on the icon
- **THEN** the value is `None`, `on_clear` was called, the field is empty

### Requirement: Field attributes
The field SHALL have `role="spinbutton"`, `inputmode="decimal"`, `aria-valuemin`, `aria-valuemax`, `aria-valuenow` (the current value), `min`, `max`, `step` (1 by default), and `id`, `placeholder`, `title`, `name`, `autocomplete`, `tabindex`, `maxlength`, `aria-label`, `aria-labelledby`, `aria-describedby`, `aria-required`, `required`, `readonly`, `disabled` only if set.

#### Scenario: Accessibility
- **WHEN** `min = 0`, `max = 100`, value `30`, `aria_label = "Amount"`
- **THEN** the field has `role="spinbutton"`, `aria-valuemin="0"`, `aria-valuemax="100"`, `aria-valuenow="30"`, `aria-label="Amount"`

#### Scenario: Disabled
- **WHEN** `disabled` is set
- **THEN** the field and buttons have `disabled`, and `data-p` includes `disabled`

### Requirement: Properties and pass-through
The properties of format, bounds, step, modes, layout, states and labels SHALL accept a value or a signal and update the markup and text when the signal changes. `pt` SHALL accept the parts `root`, `input`, `clear_icon`, `button_group`, `increment_button`, `increment_button_icon`, `decrement_button`, `decrement_button_icon`, adding classes, style and attributes without replacing the component's own.

#### Scenario: Reactive locale
- **WHEN** `locale` is passed as a signal and changes from `en-US` to `de-DE`
- **THEN** the text is redrawn with the new separators

#### Scenario: Class via pt
- **WHEN** the class `custom` is passed in `pt.input`
- **THEN** the field has `custom` and `p-inputnumber-input`

### Requirement: InputNumber render cost
The number of allocations per instance in SSR SHALL fit the `inputnumber-1000` budget in `benches/alloc-budgets.toml`, the wasm contribution the `inputnumber` budget in `benches/budgets.toml`.

#### Scenario: Budgets
- **WHEN** `cargo xtask alloc-budget` and `cargo xtask wasm-size` are run
- **THEN** the InputNumber values do not exceed the budgets
