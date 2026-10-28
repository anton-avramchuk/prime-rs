# toolbar Specification

## Purpose
TBD - created by archiving change port-toolbar. Update Purpose after archive.

## Requirements

### Requirement: Структура и классы Toolbar
Компонент `Toolbar` крейта `prime-toolbar` SHALL отрисовывать `<div class="p-toolbar p-component" role="toolbar">`, внутри которого сначала идёт проецируемое содержимое (`children`), затем области `div.p-toolbar-start`, `div.p-toolbar-center` и `div.p-toolbar-end` в этом порядке. Каждая область SHALL выводиться только если задан соответствующий слот (`start_template`, `center_template`, `end_template`). `style_class` и `style` SHALL добавляться без потери собственных классов.

#### Scenario: Три области
- **WHEN** заданы все три слота
- **THEN** HTML содержит `p-toolbar-start`, `p-toolbar-center`, `p-toolbar-end` именно в таком порядке, а у корня `role="toolbar"`

#### Scenario: Пустые области не выводятся
- **WHEN** задан только `end_template`
- **THEN** есть `p-toolbar-end`, а `p-toolbar-start` и `p-toolbar-center` отсутствуют

#### Scenario: Проецируемое содержимое
- **WHEN** передан `children` и слот `start_template`
- **THEN** содержимое `children` стоит перед `p-toolbar-start`

### Requirement: Доступность Toolbar
Toolbar SHALL выводить `aria-labelledby` с переданным значением и не выводить атрибут, если значение не задано.

#### Scenario: Подпись
- **WHEN** задан `aria_labelledby = "title"`
- **THEN** у корня `aria-labelledby="title"`

### Requirement: Свойства и pass-through Toolbar
Свойства `aria_labelledby`, `style` и `style_class` SHALL принимать значение или сигнал и обновлять разметку при смене сигнала. `pt` SHALL принимать части `root`, `start`, `center`, `end`, добавляя классы, стиль и атрибуты без замены собственных.

#### Scenario: Реактивный класс
- **WHEN** `style_class` передан сигналом и меняется
- **THEN** класс корня обновляется, собственные классы сохраняются

#### Scenario: Класс через pt
- **WHEN** в `pt.start` передан класс `custom`
- **THEN** у области начала есть `custom` и `p-toolbar-start`

### Requirement: Стоимость отрисовки Toolbar
Toolbar со статическими свойствами SHALL использовать статическую ветвь; обе ветви SHALL давать одинаковую разметку. Число аллокаций на экземпляр при SSR SHALL укладываться в бюджет `toolbar-1000` в `benches/alloc-budgets.toml`, вклад в wasm — в бюджет `toolbar` в `benches/budgets.toml`.

#### Scenario: Паритет ветвей
- **WHEN** один набор значений передан статически и сигналами
- **THEN** HTML совпадает

#### Scenario: Бюджеты
- **WHEN** выполняются `cargo xtask alloc-budget` и `cargo xtask wasm-size`
- **THEN** значения Toolbar не превышают бюджеты
