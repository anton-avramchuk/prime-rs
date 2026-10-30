## Purpose
Timeline показывает последовательность событий вдоль вертикальной или горизонтальной линии с маркерами, соединителями и подписями по обе стороны линии.

## ADDED Requirements

### Requirement: Корень Timeline
Компонент `Timeline` крейта `prime-timeline` SHALL выводить корень с классами `p-timeline p-component p-timeline-<align> p-timeline-<layout>`, где `align` — `left` (по умолчанию), `right`, `top`, `bottom` или `alternate`, а `layout` — `vertical` (по умолчанию) или `horizontal`. Атрибут `data-p` SHALL содержать через пробел `layout` и `align`. `style_class` SHALL добавляться после собственных классов, `style` — задавать встроенный стиль, `pt.root` — добавлять классы, стиль и атрибуты.

#### Scenario: Значения по умолчанию
- **WHEN** `align` и `layout` не заданы
- **THEN** классы корня `p-timeline p-component p-timeline-left p-timeline-vertical`, `data-p="vertical left"`

#### Scenario: Горизонтальная раскладка
- **WHEN** `layout = Horizontal`, `align = Top`
- **THEN** классы корня содержат `p-timeline-top` и `p-timeline-horizontal`

#### Scenario: Реактивные свойства
- **WHEN** `align` передан сигналом и меняется с `Left` на `Right`
- **THEN** класс корня заменяется без пересоздания событий

### Requirement: События Timeline
Для каждого элемента `value` в исходном порядке SHALL выводиться `div.p-timeline-event` с тремя частями: `p-timeline-event-opposite`, `p-timeline-event-separator` и `p-timeline-event-content`. В разделителе SHALL быть маркер `p-timeline-event-marker` и, у всех событий кроме последнего, соединитель `p-timeline-event-connector`. Каждая часть SHALL иметь `data-p` корня и принимать `pt`.

#### Scenario: Структура
- **WHEN** `value` содержит три события
- **THEN** выводятся три `p-timeline-event`, в каждом три части, всего три маркера и два соединителя

#### Scenario: Пустой список
- **WHEN** `value` пуст
- **THEN** корень выводится без событий

#### Scenario: pt частей
- **WHEN** `pt.event_marker` содержит класс `custom`
- **THEN** каждый маркер имеет класс `custom`

### Requirement: Шаблоны Timeline
Шаблоны `content`, `opposite` и `marker` SHALL получать событие и выводиться в соответствующих частях. Без шаблона `content` и `opposite` части SHALL быть пустыми; шаблон `marker` заменяет стандартный маркер целиком, соединитель при этом сохраняется.

#### Scenario: Содержимое
- **WHEN** задан шаблон `content`, выводящий название события
- **THEN** в `p-timeline-event-content` каждого события находится его название

#### Scenario: Свой маркер
- **WHEN** задан шаблон `marker`
- **THEN** вместо `p-timeline-event-marker` выводится результат шаблона, а соединитель остаётся

### Requirement: Стоимость отрисовки Timeline
Число аллокаций на экземпляр при SSR SHALL укладываться в бюджет `timeline-1000` в `benches/alloc-budgets.toml`, вклад в wasm — в бюджет `timeline` в `benches/budgets.toml`.

#### Scenario: Бюджеты
- **WHEN** выполняются `cargo xtask alloc-budget` и `cargo xtask wasm-size`
- **THEN** значения Timeline не превышают бюджеты
