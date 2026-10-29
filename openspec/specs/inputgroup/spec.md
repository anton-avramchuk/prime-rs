# inputgroup Specification

## Purpose
TBD - created by archiving change port-inputgroup. Update Purpose after archive.

## Requirements

### Requirement: Структура и классы InputGroup
Компонент `InputGroup` крейта `prime-inputgroup` SHALL отрисовывать `<div class="p-inputgroup">` с проецируемым содержимым (`children`) без дополнительных обёрток и без `role`. `style_class` и `style` SHALL добавляться без потери собственного класса.

#### Scenario: Содержимое
- **WHEN** в `InputGroup` переданы `InputGroupAddon`, `InputText` и `Button`
- **THEN** HTML содержит `div.p-inputgroup`, внутри которого они стоят в том же порядке

#### Scenario: Свой класс
- **WHEN** задан `style_class = "custom"`
- **THEN** у корня есть и `p-inputgroup`, и `custom`

### Requirement: Структура и классы InputGroupAddon
Компонент `InputGroupAddon` SHALL отрисовывать `<div class="p-inputgroupaddon">` с проецируемым содержимым. `style_class` и `style` SHALL добавляться без потери собственного класса.

#### Scenario: Текст и иконка
- **WHEN** в `InputGroupAddon` передан текст или иконка
- **THEN** они находятся внутри `div.p-inputgroupaddon`

#### Scenario: Инлайн-стиль
- **WHEN** задан `style = "width: 3rem"`
- **THEN** у корня есть атрибут `style` с этим значением

### Requirement: Свойства и pass-through
Свойства `style_class` и `style` обоих компонентов SHALL принимать значение или сигнал и обновлять разметку при смене сигнала. `pt` SHALL принимать часть `root`, добавляя классы, стиль и атрибуты без замены собственных.

#### Scenario: Реактивный класс
- **WHEN** `style_class` передан сигналом и меняется
- **THEN** класс корня обновляется, собственный класс сохраняется

#### Scenario: Класс через pt
- **WHEN** в `pt.root` передан класс `custom`
- **THEN** у корня есть `custom` и собственный класс компонента

### Requirement: Стоимость отрисовки InputGroup
Компоненты со статическими свойствами SHALL использовать статическую ветвь; обе ветви SHALL давать одинаковую разметку. Число аллокаций на экземпляр при SSR SHALL укладываться в бюджет `inputgroup-1000` в `benches/alloc-budgets.toml`, вклад в wasm — в бюджет `inputgroup` в `benches/budgets.toml`.

#### Scenario: Паритет ветвей
- **WHEN** один набор значений передан статически и сигналами
- **THEN** HTML совпадает

#### Scenario: Бюджеты
- **WHEN** выполняются `cargo xtask alloc-budget` и `cargo xtask wasm-size`
- **THEN** значения InputGroup не превышают бюджеты
