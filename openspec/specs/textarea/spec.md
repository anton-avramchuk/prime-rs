# textarea Specification

## Purpose
TBD - created by archiving change port-textarea. Update Purpose after archive.

## Requirements

### Requirement: Значение и события Textarea
Компонент `Textarea` крейта `prime-textarea` SHALL отрисовывать нативный `<textarea>` и принимать необязательное двустороннее `value` (`RwSignal<String>`): значение сигнала выводится в поле (при SSR — дочерним текстом), ввод пользователя записывается обратно. Он SHALL вызывать `on_input`, `on_change`, `on_focus`, `on_blur` с исходным событием.

#### Scenario: Сигнал в поле
- **WHEN** `value` содержит `abc`
- **THEN** HTML содержит `<textarea …>abc</textarea>`, а изменение сигнала на клиенте обновляет поле

#### Scenario: Ввод в сигнал
- **WHEN** пользователь вводит текст
- **THEN** сигнал получает текст и вызывается `on_input`

### Requirement: Атрибуты и классы Textarea
`<textarea>` SHALL получать `id`, `name`, `placeholder`, `rows`, `cols`, `disabled`, `readonly`, `required`, `maxlength`, `aria-label`, `aria-describedby`; незаданные атрибуты SHALL отсутствовать. Классы SHALL быть `p-textarea p-component` и модификаторы в порядке: `p-filled` (значение не пусто), `p-textarea-resizable` (`auto_resize`), `p-variant-filled` (`variant = filled` или `PrimeConfig.input_variant = filled`), `p-textarea-fluid`, `p-inputfield-sm p-textarea-sm` (small), `p-textarea-lg p-inputfield-lg` (large), `p-invalid`. `style_class` и `style` SHALL добавляться без потери собственных классов.

#### Scenario: Модификаторы
- **WHEN** заданы `size = small`, `variant = filled`, `invalid`, `fluid`, `auto_resize`
- **THEN** есть классы `p-textarea-resizable`, `p-variant-filled`, `p-textarea-fluid`, `p-inputfield-sm p-textarea-sm`, `p-invalid`

#### Scenario: Атрибуты
- **WHEN** заданы `rows = 5`, `cols = 30`, `placeholder = "Hi"`
- **THEN** у `<textarea>` есть `rows="5"`, `cols="30"`, `placeholder="Hi"`, а `disabled` и `readonly` нет

### Requirement: Автоподгонка высоты Textarea
При `auto_resize` Textarea SHALL в браузере при монтировании и при каждом изменении значения устанавливать высоту по `scrollHeight`; при достижении `max-height` SHALL включать `overflow-y: scroll` и ограничивать высоту, иначе `overflow: hidden`; после каждой подгонки SHALL вызывать `on_resize`.

#### Scenario: Рост текста
- **WHEN** в поле с `auto_resize` вводится многострочный текст
- **THEN** высота поля увеличивается и вызывается `on_resize`

#### Scenario: Предел высоты
- **WHEN** содержимое выше `max-height`
- **THEN** высота равна `max-height` и включена вертикальная прокрутка

### Requirement: Pass-through Textarea
`pt` SHALL принимать корень `<textarea>`, добавляя классы, стиль и атрибуты без замены собственных.

#### Scenario: Класс через pt
- **WHEN** передан `pt` с классом `custom`
- **THEN** у `<textarea>` есть и `custom`, и собственные классы

### Requirement: Стоимость отрисовки Textarea
Textarea без `value` и со статическими свойствами SHALL использовать статическую ветвь; обе ветви SHALL давать одинаковую разметку. Число аллокаций на экземпляр при SSR SHALL укладываться в бюджет `textarea` в `benches/alloc-budgets.toml`, вклад в wasm — в бюджет `textarea` в `benches/budgets.toml`.

#### Scenario: Паритет ветвей
- **WHEN** один набор значений передан статически и сигналами
- **THEN** HTML совпадает

#### Scenario: Бюджеты
- **WHEN** выполняются `cargo xtask alloc-budget` и `cargo xtask wasm-size`
- **THEN** значения Textarea не превышают бюджеты
