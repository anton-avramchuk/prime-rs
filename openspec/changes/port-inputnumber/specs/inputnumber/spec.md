## Purpose
InputNumber — поле ввода числа: показывает значение в локальном формате (группы разрядов, дробная часть, валюта, префикс и суффикс), принимает только числовой ввод, ограничивает значение границами и позволяет менять его шагами.

## ADDED Requirements

### Requirement: Значение и форматирование
Компонент `InputNumber` крейта `prime-inputnumber` SHALL принимать двусторонний `value` (`RwSignal<Option<f64>>`); без него SHALL вести внутреннее состояние, начинающееся с `default_value`. Поле SHALL показывать значение, отформатированное по `mode`, `locale`, `currency`, `currency_display`, `use_grouping`, `min_fraction_digits`, `max_fraction_digits`, `prefix` и `suffix`; пустое значение — пустую строку, а при `allow_empty = false` — `0`. Режим `Decimal` без явных границ дроби SHALL показывать от 0 до 3 знаков после запятой, режим `Currency` — число знаков валюты (2; для JPY 0). При `format = false` значение SHALL показываться как число без форматирования. Лишние знаки SHALL округляться «от нуля» по кратчайшему десятичному представлению числа. При внешнем изменении сигнала текст SHALL обновляться.

#### Scenario: Группы и дробь
- **WHEN** `locale = "en-US"`, `min_fraction_digits = 2`, значение `1234.5`
- **THEN** текст «1,234.50»

#### Scenario: Округление
- **WHEN** `max_fraction_digits = 2`, значение `1.005`
- **THEN** текст «1.01»

#### Scenario: Без групп
- **WHEN** `use_grouping = false`, значение `1234567`
- **THEN** текст «1234567»

#### Scenario: Префикс и суффикс
- **WHEN** `prefix = "Expires in "`, `suffix = " days"`, значение `7`
- **THEN** текст «Expires in 7 days»

#### Scenario: Пустое значение
- **WHEN** значение `None`
- **THEN** поле пусто, а при `allow_empty = false` показывает «0»

### Requirement: Локали и валюты
Форматирование SHALL поддерживать локали `en-US`, `en-GB`, `en-IN` (группы 3-2-2), `de-DE`, `fr-FR`, `ja-JP` и `ru-RU` со своими десятичным разделителем, разделителем групп и знаком минуса; для языка без региона (`de`) SHALL выбираться основная локаль языка, а неизвестная или пустая локаль — `en-US`. В режиме `Currency` SHALL поддерживаться валюты USD, EUR, GBP, JPY, INR и RUB с символом и его положением по локали (`$1,234.50`, `1.234,50 €`) и отображением кодом (`currency_display = Code`, «INR 1,234.00»); для остальных валют SHALL выводиться код. Результат SHALL быть одинаковым при SSR и в браузере.

#### Scenario: Немецкая локаль
- **WHEN** `locale = "de-DE"`, `min_fraction_digits = 2`, значение `1234.5`
- **THEN** текст «1.234,50»

#### Scenario: Индийская группировка
- **WHEN** `locale = "en-IN"`, значение `1234567.891`
- **THEN** текст «12,34,567.891»

#### Scenario: Валюта
- **WHEN** `mode = Currency`, `currency = "USD"`, `locale = "en-US"`, значение `-1234.5`
- **THEN** текст «-$1,234.50»

#### Scenario: Валюта в суффиксе
- **WHEN** `mode = Currency`, `currency = "EUR"`, `locale = "de-DE"`, значение `1234.5`
- **THEN** текст «1.234,50 €» (с неразрывным пробелом перед символом)

#### Scenario: Неизвестная локаль
- **WHEN** `locale = "xx-YY"`
- **THEN** используется формат `en-US`

### Requirement: Ввод текста
Поле SHALL принимать только цифры, знак минуса (в начале числа, один раз, если ему не мешает `min >= 0`) и десятичный разделитель локали (один раз и только если допускает формат); остальные символы SHALL игнорироваться. После каждого изменения текст SHALL переформатироваться (группы, префикс, суффикс, валюта) с сохранением набранной дробной части (например «1.», «1.50»), а курсор SHALL оставаться после того же числа значимых символов (цифр, разделителя, минуса). Число знаков после запятой SHALL ограничиваться максимумом формата. Backspace и Delete SHALL удалять цифру, пропуская разделитель групп, не удалять префикс и суффикс, при удалении выделения — весь диапазон; удаление цифры дробной части SHALL заменять её нулём, если число знаков не превышает `min_fraction_digits`, иначе убирать. Вставка из буфера SHALL разбирать вставленный текст как число и подставлять его значение. Каждое изменение значения SHALL записываться в сигнал и вызывать `on_input` со значением и текстом.

#### Scenario: Набор цифр
- **WHEN** в пустое поле вводятся «1», «2», «3», «4»
- **THEN** текст «1,234», значение `1234`

#### Scenario: Игнорируемый символ
- **WHEN** вводится буква «a»
- **THEN** текст и значение не меняются

#### Scenario: Десятичный разделитель
- **WHEN** в «1» вводится «.», затем «5»
- **THEN** сначала текст «1.» и значение `1`, затем «1.5» и значение `1.5`

#### Scenario: Ограничение дроби
- **WHEN** `max_fraction_digits = 2`, в «1.25» вводится ещё цифра в конец
- **THEN** текст остаётся «1.25»

#### Scenario: Backspace над разделителем
- **WHEN** текст «1,234», курсор сразу после запятой, нажат Backspace
- **THEN** удаляется «1», текст «234»

#### Scenario: Вставка
- **WHEN** в пустое поле вставляется «1,234.5» при `locale = "en-US"`
- **THEN** значение `1234.5`

### Requirement: Фиксация и границы
При потере фокуса, `Enter` и `Tab` текст SHALL разбираться в число, ограничиваться `min` и `max`, форматироваться заново, значение записываться в сигнал; пустой или состоящий из одного «-» текст SHALL давать `None` (а при `allow_empty = false` — `0`). `Home` и `End` при заданных `min` и `max` SHALL устанавливать границу и отменять действие по умолчанию. Значение за границей, введённое набором, SHALL ограничиваться при фиксации.

#### Scenario: Потеря фокуса
- **WHEN** `max = 100`, набрано «250» и фокус потерян
- **THEN** значение `100`, текст «100»

#### Scenario: Пустое поле
- **WHEN** текст стёрт и фокус потерян
- **THEN** значение `None`, а при `allow_empty = false` — `0`

#### Scenario: Границы клавишами
- **WHEN** `min = 0`, `max = 100`, нажата `End`
- **THEN** значение `100`

### Requirement: Шаги и кнопки
Клавиши стрелка вверх и вниз SHALL увеличивать и уменьшать текущее значение (разобранное из поля, иначе 0) на `step` (по умолчанию 1), ограничивать `min..=max`, отменять действие по умолчанию и записывать значение. При `show_buttons` SHALL выводиться кнопки увеличения и уменьшения, меняющие значение так же: нажатие мыши сразу делает шаг, а удержание через 500 мс повторяет шаг каждые 40 мс до отпускания или ухода указателя; клавиши Enter и Space на кнопке делают шаг. При `readonly` и `disabled` стрелки, кнопки, набор и вставка SHALL не менять значение. Если после шага длина текста превысила бы `max_length`, шаг SHALL не выполняться.

#### Scenario: Стрелка
- **WHEN** значение `5`, нажата стрелка вверх
- **THEN** значение `6`, текст «6»

#### Scenario: Шаг и граница
- **WHEN** `step = 0.25`, `max = 1`, значение `0.9`, нажата стрелка вверх
- **THEN** значение `1`

#### Scenario: Удержание кнопки
- **WHEN** кнопка увеличения удерживается дольше 500 мс
- **THEN** значение растёт на шаг каждые 40 мс, а после отпускания рост прекращается

#### Scenario: Только чтение
- **WHEN** задан `readonly` и нажата стрелка вверх
- **THEN** значение не меняется

### Requirement: Структура и классы
Корень SHALL быть `<span class="p-inputnumber p-component p-inputwrapper">` с модификаторами `p-inputwrapper-filled` (поле не пусто или `allow_empty = false`), `p-inputwrapper-focus`, `p-inputnumber-stacked`, `p-inputnumber-horizontal`, `p-inputnumber-vertical` (при `show_buttons` по `button_layout`), `p-inputnumber-fluid`, `p-invalid`, `style_class` и атрибутом `data-p` (признаки `invalid`, `disabled`, `focus`, `fluid`, `filled`, `empty`, размер, раскладка кнопок). Внутри SHALL идти `input.p-inputtext.p-component.p-inputnumber-input` (с классами размера, варианта, `p-invalid`, `p-filled`), затем значок очистки `p-inputnumber-clear-icon` (при `show_clear`, непустом значении и не вертикальной раскладке) и кнопки: при `stacked` в `span.p-inputnumber-button-group`, иначе по отдельности; кнопки `p-inputnumber-button p-inputnumber-increment-button` и `p-inputnumber-decrement-button` с `tabindex="-1"`, `aria-hidden` и `disabled` при `disabled`. Клик по значку очистки SHALL сбрасывать значение в `None` и вызывать `on_clear`.

#### Scenario: Стек кнопок
- **WHEN** `show_buttons`, `button_layout = Stacked`
- **THEN** у корня `p-inputnumber-stacked`, кнопки внутри `p-inputnumber-button-group`

#### Scenario: Горизонтальная раскладка
- **WHEN** `show_buttons`, `button_layout = Horizontal`
- **THEN** у корня `p-inputnumber-horizontal`, кнопки вне группы

#### Scenario: Очистка
- **WHEN** `show_clear`, значение `5` и клик по значку
- **THEN** значение `None`, `on_clear` вызван, поле пусто

### Requirement: Атрибуты поля
Поле SHALL иметь `role="spinbutton"`, `inputmode="decimal"`, `aria-valuemin`, `aria-valuemax`, `aria-valuenow` (текущее значение), `min`, `max`, `step` (по умолчанию 1), а `id`, `placeholder`, `title`, `name`, `autocomplete`, `tabindex`, `maxlength`, `aria-label`, `aria-labelledby`, `aria-describedby`, `aria-required`, `required`, `readonly`, `disabled` — только если заданы.

#### Scenario: Доступность
- **WHEN** `min = 0`, `max = 100`, значение `30`, `aria_label = "Amount"`
- **THEN** у поля `role="spinbutton"`, `aria-valuemin="0"`, `aria-valuemax="100"`, `aria-valuenow="30"`, `aria-label="Amount"`

#### Scenario: Недоступно
- **WHEN** задан `disabled`
- **THEN** у поля и кнопок `disabled`, у `data-p` есть `disabled`

### Requirement: Свойства и pass-through
Свойства формата, границ, шага, режимов, раскладки, состояний и подписей SHALL принимать значение или сигнал и обновлять разметку и текст при смене сигнала. `pt` SHALL принимать части `root`, `input`, `clear_icon`, `button_group`, `increment_button`, `increment_button_icon`, `decrement_button`, `decrement_button_icon`, добавляя классы, стиль и атрибуты без замены собственных.

#### Scenario: Реактивная локаль
- **WHEN** `locale` передан сигналом и меняется с `en-US` на `de-DE`
- **THEN** текст перерисовывается с новыми разделителями

#### Scenario: Класс через pt
- **WHEN** в `pt.input` передан класс `custom`
- **THEN** у поля есть `custom` и `p-inputnumber-input`

### Requirement: Стоимость отрисовки InputNumber
Число аллокаций на экземпляр при SSR SHALL укладываться в бюджет `inputnumber-1000` в `benches/alloc-budgets.toml`, вклад в wasm — в бюджет `inputnumber` в `benches/budgets.toml`.

#### Scenario: Бюджеты
- **WHEN** выполняются `cargo xtask alloc-budget` и `cargo xtask wasm-size`
- **THEN** значения InputNumber не превышают бюджеты
