## ADDED Requirements

### Requirement: Структура и классы ProgressSpinner
Компонент `ProgressSpinner` крейта `prime-progressspinner` SHALL отрисовывать `<div class="p-progressspinner">` с `role="progressbar"` и `aria-busy="true"`, внутри которого `svg.p-progressspinner-spin` с `viewBox="25 25 50 50"` и вложенным `circle.p-progressspinner-circle` (`cx="50"`, `cy="50"`, `r="20"`, `stroke-miterlimit="10"`). `style_class` и `style` SHALL добавляться к корню без потери собственных классов.

#### Scenario: Разметка по умолчанию
- **WHEN** отрисован `ProgressSpinner` без свойств
- **THEN** HTML содержит `p-progressspinner`, `role="progressbar"`, `aria-busy="true"`, `p-progressspinner-spin`, `p-progressspinner-circle`, `fill="none"`, `stroke-width="2"` и `animation-duration: 2s`

#### Scenario: Дополнительный класс
- **WHEN** задан `style_class = "mine"`
- **THEN** у корня есть `p-progressspinner` и `mine`

### Requirement: Свойства окружности и анимации
`stroke_width`, `fill` и `animation_duration` SHALL принимать значение или сигнал и выводиться как `stroke-width` и `fill` у `circle` и `animation-duration` в `style` у `svg`. При смене сигнала разметка SHALL обновляться.

#### Scenario: Свои значения
- **WHEN** заданы `stroke_width = "8"`, `fill = "transparent"`, `animation_duration = ".5s"`
- **THEN** у окружности `stroke-width="8"` и `fill="transparent"`, у `svg` `animation-duration: .5s`

#### Scenario: Реактивное значение
- **WHEN** `stroke_width` передан сигналом и меняется
- **THEN** новая отрисовка содержит новое значение

### Requirement: Доступность ProgressSpinner
Свойство `aria_label` SHALL выводиться как `aria-label` корня и не выводиться, если не задано.

#### Scenario: Подпись
- **WHEN** задан `aria_label = "loading"`
- **THEN** у корня `aria-label="loading"`

#### Scenario: Без подписи
- **WHEN** `aria_label` не задан
- **THEN** атрибута `aria-label` в HTML нет

### Requirement: Pass-through ProgressSpinner
`pt` SHALL принимать части `root`, `spin`, `circle`, добавляя классы, стиль и атрибуты без замены собственных.

#### Scenario: Класс через pt
- **WHEN** в `pt.circle` передан класс `custom`
- **THEN** у окружности есть `custom` и `p-progressspinner-circle`

### Requirement: Стоимость отрисовки ProgressSpinner
ProgressSpinner со статическими свойствами SHALL использовать статическую ветвь; обе ветви SHALL давать одинаковую разметку. Число аллокаций на экземпляр при SSR SHALL укладываться в бюджет `progressspinner-1000` в `benches/alloc-budgets.toml`, вклад в wasm — в бюджет `progressspinner` в `benches/budgets.toml`.

#### Scenario: Паритет ветвей
- **WHEN** один набор значений передан статически и сигналами
- **THEN** HTML совпадает

#### Scenario: Бюджеты
- **WHEN** выполняются `cargo xtask alloc-budget` и `cargo xtask wasm-size`
- **THEN** значения ProgressSpinner не превышают бюджеты
