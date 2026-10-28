## ADDED Requirements

### Requirement: Разметка и ARIA ProgressBar
Компонент `ProgressBar` крейта `prime-progressbar` SHALL отрисовывать корень `<div role="progressbar">` с `aria-valuemin="0"`, `aria-valuemax="100"`, классами `p-progressbar p-component p-progressbar-determinate` (по умолчанию) или `p-progressbar-indeterminate` и `data-p` (`determinate` или `indeterminate`). При заданном `value` корень SHALL иметь `aria-valuenow` со значением и `aria-level` со значением и единицей; без значения эти атрибуты SHALL отсутствовать.

#### Scenario: Определённый режим
- **WHEN** `value = 50`
- **THEN** корень имеет классы `p-progressbar p-component p-progressbar-determinate`, `aria-valuenow="50"`, `aria-level="50%"`, `data-p="determinate"`

#### Scenario: Без значения
- **WHEN** `value` не задан
- **THEN** у корня нет `aria-valuenow` и `aria-level`

### Requirement: Определённый режим ProgressBar
В режиме `determinate` ProgressBar SHALL выводить `div.p-progressbar-value` со стилем `width: <value>%; display: flex` (и `background: <color>` при `color`) и `value_style_class`, а внутри — `div.p-progressbar-label` с `div` значения и единицы (`unit`, по умолчанию `%`), который скрыт (`display: none`) при отсутствии значения или нуле. При `show_value = false` значение не выводится; при `content_template` вместо значения выводится шаблон, получающий значение.

#### Scenario: Ширина и подпись
- **WHEN** `value = 40`
- **THEN** у `p-progressbar-value` стиль содержит `width: 40%`, а подпись содержит `40%`

#### Scenario: Нулевое значение
- **WHEN** `value = 0`
- **THEN** подпись присутствует, но её `div` имеет `display: none`

#### Scenario: Шаблон
- **WHEN** задан `content_template`
- **THEN** вместо значения выводится результат шаблона, получившего значение

### Requirement: Неопределённый режим ProgressBar
В режиме `indeterminate` ProgressBar SHALL выводить только `div.p-progressbar-value` (с `background: <color>` при `color` и `value_style_class`), без ширины и подписи.

#### Scenario: Indeterminate
- **WHEN** `mode = indeterminate`
- **THEN** есть `p-progressbar-indeterminate`, `p-progressbar-value` без `width` и нет `p-progressbar-label`

### Requirement: Pass-through ProgressBar
`pt` SHALL принимать `root`, `value`, `label`, добавляя классы и атрибуты без замены собственных.

#### Scenario: Класс через pt
- **WHEN** в `pt.value` передан класс `custom`
- **THEN** у `p-progressbar-value` есть и `custom`, и собственный класс

### Requirement: Стоимость отрисовки ProgressBar
ProgressBar со статическими свойствами SHALL использовать статическую ветвь; обе ветви SHALL давать одинаковую разметку. Число аллокаций на экземпляр при SSR SHALL укладываться в бюджет `progressbar` в `benches/alloc-budgets.toml`, вклад в wasm — в бюджет `progressbar` в `benches/budgets.toml`.

#### Scenario: Паритет ветвей
- **WHEN** один набор значений передан статически и сигналами
- **THEN** HTML совпадает

#### Scenario: Бюджеты
- **WHEN** выполняются `cargo xtask alloc-budget` и `cargo xtask wasm-size`
- **THEN** значения ProgressBar не превышают бюджеты
