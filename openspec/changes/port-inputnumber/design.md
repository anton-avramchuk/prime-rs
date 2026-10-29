# Design

## Context

Требования — в `specs/inputnumber`. Исходники PrimeNG 21.1.10: `packages/primeng/src/inputnumber/inputnumber.ts` (1493 строки), `inputnumber/style/inputnumberstyle.ts`, демонстрации `apps/showcase/doc/inputnumber`. Образцы — `crates/prime-inputtext` (классы и `data-p` внутреннего поля, `Variant`/`Size`, конфиг), `crates/prime-knob` и `crates/prime-slider` (слушатели и таймеры из обработчиков, `*_untracked`, правила без пересоздания элементов), `crates/prime-selectbutton` (Effect только под `cfg(wasm32)`). Решение пользователя: форматирование — собственное, на малой таблице локалей и валют.

Наблюдения по исходнику, влияющие на решения:
- Весь формат строится на `Intl.NumberFormat` и регулярных выражениях из его вывода; вывод браузера на сервере недоступен, поэтому форматтер и разборщик нужны на Rust.
- Ввод реализован «хирургией» над отформатированной строкой в `keypress`/`keydown` с отдельными ветками для курсора (`initCursor`, `updateInput`, `insert`, `deleteRange`, `concatValues`); `keypress` устарел и не работает на мобильных клавиатурах.
- `onInputKeyDown` для `Home`/`End` проверяет `if (this.min())`, то есть пропускает `min = 0`.
- Классы `p-disabled` у кнопок зависят от `maxlength()` вместо значения, то есть фактически никогда не включаются.
- Форма значения — `number | null`; `'-'` при разборе — служебная метка «введён только минус».
- `formattedValue()` и поле связаны по `[value]`, а в процессе набора DOM правится напрямую (текст вроде «1.» не равен `format(1)`).

## Decisions

### D1. Значение и отображаемый текст
`value: Option<RwSignal<Option<f64>>>`, без него внутренний сигнал с `default_value`. Отдельно хранится `display: RwSignal<String>` — то, что показано в поле; она привязана к `prop:value` и является единственным источником текста. Правки при наборе записывают текст напрямую в DOM (вместе с позицией курсора) и затем в `display`, а внешнее изменение `value` (сигнал меняют снаружи) переформатирует `display` через `Effect` под `cfg(target_arch = "wasm32")`: если разбор `display` уже равен значению, текст не трогается (набираемая «1.» остаётся). При SSR текст — `format(value)` один раз.

### D2. Форматтер, локали, валюты (`locale.rs`, `format.rs`)
`LocaleData { decimal, group, minus, grouping: Standard | Indian, currency_position, currency_gap }` для `en-US`, `en-GB`, `en-IN`, `de-DE`, `fr-FR` (группа — узкий неразрывный пробел `U+202F`), `ja-JP`, `ru-RU` (группа — `U+00A0`); `resolve_locale(tag)` — точное совпадение, затем по языку, иначе `en-US`; пустая строка и несуществующие теги вроде `jp-JP` дают `en-US`, как у Intl при отсутствии поддержки локали. `CurrencyData { code, symbol, digits }` для USD, EUR, GBP, JPY, INR, RUB; иные валюты — код и 2 знака. `format_number(value, &FormatOptions) -> String`: округление по десятичному представлению `f64` (кратчайшее представление из `Display`) по правилу «половина от нуля»; число знаков: `min..max` (decimal: 0..3, currency: цифры валюты); группировка (3 или 3-2-2); знак минуса локали перед символом валюты; префикс/суффикс валюты по таблице; `prefix`/`suffix` пользователя добавляются снаружи. Всё без аллокаций сверх результирующих строк и без зависимостей.

### D3. Разбор (`parse_text`)
По шагам PrimeNG: убрать суффикс и префикс, пробелы, символ валюты, разделители групп, заменить знак минуса на `-`, десятичный разделитель на `.`, разобрать `f64`; пустое → `None`, одиночный `-` → отдельное состояние `Minus` (значение `None` в модели, но в поле остаётся «-»). Цифры — только ASCII (нелатинские системы счисления не поддерживаются).

### D4. Правка текста (`edit.rs`, чистые функции)
`Edit { text, caret }`. Операции: `insert(text, sel, char, ctx)`, `delete_back`, `delete_forward`, `delete_range`, `paste`, `commit(text) -> (Option<f64>, text)`, `clamp_caret` (по `initCursor`: курсор не заходит в префикс и суффикс). Правила — из PrimeNG: минус только в начале и если нет `min >= 0`, десятичный разделитель один и только при `max_fraction_digits > 0`, число знаков дроби не больше максимума, удаление цифры дроби — нулём (если знаков не больше `min_fraction_digits`), удаление единственной целой цифры даёт «0» или пустоту, удаление через разделитель группы удаляет соседнюю цифру. После операции текст получается так: разобрать новое сырое значение, отформатировать, сохранить набранный «хвост» дроби (`concatValues`: «1.», «1.50», «0.0») и суффикс/валюту. Курсор — «после того же числа значимых символов» (цифры, десятичный разделитель, минус) слева от каретки до правки; в сравнении с ветвями `updateInput` PrimeNG это даёт тот же результат в обычных случаях и проще проверяется. Всё покрыто таблицами тестов.

### D5. События ввода
Вместо `keypress` — `beforeinput`: `insertText` (по `data`), `deleteContentBackward`/`Forward`, `insertFromPaste`/`insertFromDrop` (по `data` или `dataTransfer`) отменяются (`preventDefault`) и обрабатываются операциями D4; остальные типы (`historyUndo` и т. п.) отменяются. Такой путь работает и с мобильными клавиатурами. `keydown` обрабатывает стрелки вверх и вниз (шаг), `Enter`/`Tab` (фиксация), `Home`/`End` (границы), `ArrowLeft`/`ArrowRight` не трогаем (курсор перемещает браузер, а `click` приводит его к числовой части — `clamp_caret`). Клавиши с `Shift` и `Alt` пропускаются браузеру, как в PrimeNG. `Home`/`End` работают при любом заданном `min`/`max`, включая 0. `blur` фиксирует значение (`commit`), `focus` ставит признак фокуса для `p-inputwrapper-focus` и `data-p`.

### D6. Шаги и кнопки
`spin(dir)`: `current = parse(поле).unwrap_or(0)`, `new = clamp(current + step·dir)`, отказ при превышении `max_length`, запись текста и значения, событие `on_input`. Кнопки — `mousedown` (первый шаг и таймер 500 мс, затем каждые 40 мс), `mouseup`/`mouseleave` (остановка), `keydown` Enter/Space (шаг) и `keyup` (остановка); таймер — `set_timeout_with_handle`, остановка также в `on_cleanup`. `readonly`/`disabled` блокируют всё. Кнопки не получают фокус (`tabindex=-1`), `mousedown` отменяет действие по умолчанию, чтобы поле не теряло фокус. Класс `p-disabled` у кнопок не выводится (как фактически и в PrimeNG).

### D7. Разметка
Корень — `<span>`: `p-inputnumber p-component p-inputwrapper`, `p-inputwrapper-filled` (`display` не пуста или `allow_empty = false`), `p-inputwrapper-focus`, раскладка, `p-inputnumber-fluid`, `p-invalid`, `style_class`, `pt.root`; `data-p` — `invalid disabled focus fluid filled empty <size> <layout>`. Внутреннее поле — `prime_inputtext` (`InputTextState`, `write_classes`, `data_p` для `p-inputtext` классов), плюс `p-inputnumber-input` и `input_style_class`. Значок очистки — `TimesIcon` (или `ViewFn`), кнопки — `AngleUpIcon`/`AngleDownIcon` (или `ViewFn`). Для `stacked` кнопки в `span.p-inputnumber-button-group`, иначе после поля; вертикальная раскладка без значка очистки. Структура строится один раз по `show_buttons`/`button_layout`/`show_clear`; всё остальное — замыкания.

### D8. Атрибуты и значения по умолчанию
Поле: `role=spinbutton`, `inputmode=decimal`, `aria-valuemin/max/now`, `min`, `max`, `step` (`step ?? 1`) и необязательные атрибуты — как в шаблоне PrimeNG. `aria-valuenow` — значение (после каждой правки). `locale` по умолчанию — `en-US` (у PrimeNG — язык браузера; на сервере его нет).

### D9. CSS
`inputnumber` в `theme.components` (`upstream.toml`), `cargo xtask gen-theme`; блок `/* For PrimeNG */` содержит только правила `.ng-invalid.ng-dirty` Angular Forms, поэтому `primeng_extra` не меняется. CSS `inputtext` на странице уже подключён.

### D10. Демонстрации
`locale`, `currency`, `numerals`, `prefixsuffix`, `buttons`, `vertical`, `clearicon`, `sizes`, `filled`, `invalid`, `disabled`, `fluid` (двенадцать; `floatlabel`, `iftalabel` и формы пропускаются). В `currency` «jp-JP» из исходника превращается в корректный `ja-JP` с пометкой. В `buttons` и `vertical` значки плюс и минус — `PlusIcon`/`MinusIcon` через `ViewFn`. Для e2e у части демонстраций выводится `echo` значения.

### D11. Тесты
Чистые: форматтер (локали, валюты, округление, дробь, группы), разборщик (туда-обратно: `parse(format(x)) == x` для таблицы значений и локалей), операции правки (таблицы: набор, минус, разделитель, ограничение дроби, Backspace/Delete над разделителями, диапазоны, вставка, курсор). SSR: структура, классы и `data-p`, форматы, кнопки по раскладкам, значок очистки, атрибуты, `pt`. Браузерные (`tests/web.rs`): набор через `beforeinput` и положение курсора, Backspace/Delete, вставка, фиксация по blur/Enter/Tab, границы, стрелки, кнопки и повтор (по таймеру), `readonly`/`disabled`, очистка, внешний сигнал (в том числе при смене `locale`), «1.» не затирается при наборе. e2e: набор с реальной клавиатуры (`type`, `Backspace`), стрелки, кнопки, валюты и локали.

## Risks / Trade-offs

- [Малая таблица локалей и валют отличается от Intl: другие локали и валюты, нелатинские цифры, компактные и процентные режимы] → фиксируется в спеке и `docs/inputnumber.md`; таблица расширяема данными.
- [Поведение курсора и набора не совпадает с PrimeNG символ в символ: `beforeinput` вместо `keypress`, курсор по числу значимых символов] → чистые функции с таблицами; отличия перечислены в документации; e2e с реальной клавиатурой.
- [Большой объём: форматтер, разборщик, правка, события, кнопки, раскладки] → задачи разбиты по слоям, чистая логика проверяется до подключения к DOM.
- [Стоимость отрисовки выше, чем у InputText (две-три кнопки, форматирование)] → бюджет по измерению.

## Migration Plan

Миграции нет: новый крейт. Откат — `git revert`.

## Open Questions

Нет.
