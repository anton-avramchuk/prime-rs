# skeleton Specification

## Purpose
TBD - created by archiving change port-skeleton. Update Purpose after archive.

## Requirements

### Requirement: Разметка и классы Skeleton
Компонент `Skeleton` крейта `prime-skeleton` SHALL отрисовывать `<div aria-hidden="true">` с классами `p-skeleton p-component`, `p-skeleton-circle` (при `shape = circle`), `p-skeleton-animation-none` (при `animation = none`) и `data-p`, равным имени формы (`rectangle` по умолчанию, `circle`). `style_class` SHALL добавляться без потери собственных классов.

#### Scenario: Значения по умолчанию
- **WHEN** свойства не заданы
- **THEN** корень имеет класс `p-skeleton p-component` и `data-p="rectangle"`, без `p-skeleton-circle` и `p-skeleton-animation-none`

#### Scenario: Круг без анимации
- **WHEN** заданы `shape = circle` и `animation = none`
- **THEN** есть классы `p-skeleton-circle` и `p-skeleton-animation-none`, а `data-p` равен `circle`

### Requirement: Размеры Skeleton
Встроенный стиль SHALL начинаться с `position: relative;`, затем `width` (по умолчанию `100%`) и `height` (по умолчанию `1rem`); при заданном `size` ширина и высота SHALL равняться `size`; при заданном `border_radius` SHALL добавляться `border-radius`.

#### Scenario: Ширина и высота
- **WHEN** заданы `width = "10rem"` и `height = "4rem"`
- **THEN** стиль равен `position: relative; width: 10rem; height: 4rem;`

#### Scenario: Размер
- **WHEN** задан `size = "3rem"` вместе с `width`
- **THEN** ширина и высота равны `3rem`, `width` игнорируется

#### Scenario: Скругление
- **WHEN** задан `border_radius = "16px"`
- **THEN** стиль заканчивается на `border-radius: 16px;`

### Requirement: Pass-through Skeleton
`pt` SHALL принимать `root`, добавляя классы, стиль и атрибуты без замены собственных.

#### Scenario: Класс через pt
- **WHEN** в `pt.root` передан класс `custom`
- **THEN** у корня есть и `custom`, и `p-skeleton`

### Requirement: Стоимость отрисовки Skeleton
Skeleton со статическими свойствами SHALL использовать статическую ветвь; обе ветви SHALL давать одинаковую разметку. Число аллокаций на экземпляр при SSR SHALL укладываться в бюджет `skeleton` в `benches/alloc-budgets.toml`, вклад в wasm — в бюджет `skeleton` в `benches/budgets.toml`.

#### Scenario: Паритет ветвей
- **WHEN** один набор значений передан статически и сигналами
- **THEN** HTML совпадает

#### Scenario: Бюджеты
- **WHEN** выполняются `cargo xtask alloc-budget` и `cargo xtask wasm-size`
- **THEN** значения Skeleton не превышают бюджеты
