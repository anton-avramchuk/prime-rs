# Proposal

## Why

InputNumber — поле числового ввода с форматированием (разделители групп, дробная часть, валюта, префикс и суффикс), границами, шагом и кнопками «больше/меньше». Это самый сложный из оставшихся простых полей и основа для форм; порт следует за `port-knob` и использует `prime-inputtext` для внутреннего поля.

## What Changes

- Новый крейт `prime-inputnumber` с компонентом `InputNumber`: двусторонний `value` (`RwSignal<Option<f64>>`) или внутреннее состояние с `default_value`; режимы `Decimal` и `Currency` (`currency`, `currency_display` символ или код), `locale`, `use_grouping`, `min_fraction_digits`, `max_fraction_digits`, `format`, `prefix`, `suffix`, `min`, `max`, `step` (1), `allow_empty`, `max_length`; кнопки `show_buttons` с раскладками `Stacked`, `Horizontal`, `Vertical`; `show_clear`; `readonly`, `disabled`, `invalid`, `required`, `variant`, `size`, `fluid`; `placeholder`, `input_id`, `name`, `tabindex`, `title`, `autocomplete`, `aria_label`, `aria_labelledby`, `aria_describedby`, `aria_required`, `input_style`, `input_style_class`, `style`, `style_class`, `increment_button_class`, `decrement_button_class`; значки кнопок и очистки — `ViewFn`; события `on_input` (значение и отформатированный текст), `on_focus`, `on_blur`, `on_key_down`, `on_clear`; `pt`.
- Формат и разбор чисел — собственный, на малой таблице локалей (`en-US`, `en-GB`, `en-IN` с группами 3-2-2, `de-DE`, `fr-FR`, `ja-JP`, `ru-RU`; неизвестная локаль — `en-US`) и валют (USD, EUR, GBP, JPY, INR, RUB; иные — по коду). Один и тот же текст при SSR и в браузере, без новых зависимостей; округление «половина от нуля» по кратчайшему десятичному представлению, как ICU.
- Разметка как в PrimeNG 21.1.10 (`inputnumber.ts`, `inputnumberstyle.ts`): корень `<span class="p-inputnumber p-component p-inputwrapper">` с `data-p`, внутри `<input role="spinbutton" class="p-inputtext p-component p-inputnumber-input" inputmode="decimal">`, значок очистки, группа кнопок `p-inputnumber-button-group` (stacked) или отдельные кнопки `p-inputnumber-button`.
- Ввод как в PrimeNG: допускаются только цифры, знак минуса и десятичный разделитель; текст переформатируется на лету с сохранением позиции курсора; Backspace и Delete с учётом разделителей и дробной части; вставка из буфера; стрелки вверх и вниз, кнопки (с повтором при удержании), `Enter`/`Tab`/потеря фокуса фиксируют значение с ограничением `min..max`; `Home`/`End` ставят границы.
- Страница `/inputnumber` (`locale`, `currency`, `numerals`, `prefixsuffix`, `buttons`, `vertical`, `clearicon`, `sizes`, `filled`, `invalid`, `disabled`, `fluid`), e2e, браузерные тесты, бенчмарки, бюджеты, `docs/inputnumber.md`, CSS темы `inputnumber`.
- **Не входит:** Angular Forms (`ngModel`, реактивные формы, `updateOn: blur`); `autofocus`; полный Intl (остальные локали и валюты, `localeMatcher`, нелатинские цифры, компактные и процентные режимы, `minlength`); демонстрации `reactiveforms`, `templatedrivenforms`, `floatlabel`, `iftalabel` (FloatLabel и IftaLabel не портированы); блок `/* For PrimeNG */` (правила `.ng-invalid.ng-dirty`).

## Capabilities

### New Capabilities
- `inputnumber`: компонент InputNumber — значение, форматирование и разбор, ввод с клавиатуры и из буфера, кнопки и повтор, границы, структура и доступность, `pt`, производительность.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-inputnumber` (`publish = false`); зависимости `prime-core`, `prime-inputtext` (классы внутреннего поля), `prime-icons` (`angleup`, `angledown`, `times`).
- Изменяются `Cargo.toml` workspace, `upstream.toml` (`inputnumber` в `theme.components`), `showcase`, `benches/*`, `e2e/tests`, `ci.yml` (браузерный job для `tests/web.rs`).
