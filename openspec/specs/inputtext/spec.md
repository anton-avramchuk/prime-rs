# inputtext Specification

## Purpose
TBD - created by archiving change port-inputtext. Update Purpose after archive.

## Requirements

### Requirement: Значение и события InputText
Компонент `InputText` крейта `prime-inputtext` SHALL отрисовывать нативный `<input>` и принимать необязательное двустороннее `value` (`RwSignal<String>`): значение сигнала отображается в поле, ввод пользователя записывается обратно в сигнал. Он SHALL вызывать `on_input`, `on_change`, `on_focus`, `on_blur` с исходным событием.

#### Scenario: Сигнал в поле
- **WHEN** `value` содержит `abc`
- **THEN** у `<input>` значение `abc`; при изменении сигнала поле обновляется

#### Scenario: Ввод в сигнал
- **WHEN** пользователь вводит текст в поле
- **THEN** сигнал `value` получает введённый текст и вызывается `on_input`

### Requirement: Атрибуты InputText
Компонент SHALL передавать в `<input>` атрибуты `type` (`input_type`, по умолчанию `text`), `id`, `name`, `placeholder`, `disabled`, `readonly`, `required`, `maxlength`, `autocomplete`, `aria-label`, `aria-describedby`; незаданные атрибуты SHALL отсутствовать.

#### Scenario: Атрибуты
- **WHEN** заданы `placeholder = "Name"`, `name = "n"`, `disabled = true`
- **THEN** у `<input>` есть `placeholder="Name"`, `name="n"` и `disabled`, а `readonly` и `required` нет

#### Scenario: Тип по умолчанию
- **WHEN** `input_type` не задан
- **THEN** у `<input>` атрибут `type="text"`

### Requirement: Классы и атрибуты InputText
Корень SHALL иметь классы `p-inputtext p-component` и модификаторы: `p-filled` (значение `value` не пусто), `p-inputtext-sm` и `p-inputtext-lg` (по `size`), `p-invalid` (`invalid`), `p-variant-filled` (`variant = filled`, а при незаданном `variant` — `PrimeConfig.input_variant = filled`), `p-inputtext-fluid` (`fluid`). Атрибут `data-p` SHALL содержать через пробел `invalid`, `fluid`, `filled` (при варианте `filled`) и размер (`small` или `large`) в этом порядке. `style_class` и `style` SHALL добавляться без потери собственных классов.

#### Scenario: Размер, вариант, invalid, fluid
- **WHEN** заданы `size = small`, `variant = filled`, `invalid`, `fluid`
- **THEN** классы `p-inputtext-sm`, `p-variant-filled`, `p-invalid`, `p-inputtext-fluid`, а `data-p` равен `invalid fluid filled small`

#### Scenario: Заполненное поле
- **WHEN** `value` не пусто
- **THEN** у `<input>` есть `p-filled`; при очистке значения класс пропадает

#### Scenario: Вариант из конфигурации
- **WHEN** `variant` не задан, а `PrimeConfig.input_variant = filled`
- **THEN** у `<input>` есть `p-variant-filled`

### Requirement: Pass-through InputText
`InputText` SHALL принимать `pt` для корневого элемента; классы и атрибуты добавляются к собственным, не заменяя их.

#### Scenario: Класс через pt
- **WHEN** передан `pt` с классом `custom`
- **THEN** у `<input>` есть и `custom`, и собственные классы

### Requirement: Стоимость отрисовки InputText
`InputText` без привязанного `value` и со статическими свойствами SHALL использовать статическую ветвь отрисовки; при `value` или сигнальных свойствах — реактивную; обе ветви SHALL давать одинаковую разметку. Число аллокаций на экземпляр при SSR SHALL укладываться в бюджет `inputtext` в `benches/alloc-budgets.toml`, вклад в wasm — в бюджет `inputtext` в `benches/budgets.toml`.

#### Scenario: Паритет ветвей
- **WHEN** один и тот же набор значений передан статически и сигналами
- **THEN** HTML совпадает

#### Scenario: Бюджеты
- **WHEN** выполняются `cargo xtask alloc-budget` и `cargo xtask wasm-size`
- **THEN** значения InputText не превышают бюджеты
