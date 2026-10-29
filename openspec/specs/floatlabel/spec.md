# floatlabel Specification

## Purpose
FloatLabel показывает метку поверх пустого поля и поднимает её при фокусе или вводе, в трёх вариантах расположения.

## Requirements

### Requirement: Разметка FloatLabel
Компонент `FloatLabel` крейта `prime-floatlabel` SHALL выводить `<span class="p-floatlabel p-floatlabel-over">` с дочерним содержимым (поле и его `<label>`); `variant = On` SHALL давать `p-floatlabel-on`, `variant = In` — `p-floatlabel-in`; по умолчанию `Over`. В классах корня SHALL быть ровно один из трёх вариантов. `style_class` SHALL добавляться после собственных классов, `style` — задавать встроенный стиль, `pt.root` — добавлять классы, стиль и атрибуты без замены собственных. Положение, подъём и размер метки SHALL обеспечиваться CSS темы `floatlabel` по состоянию вложенного поля (`:focus`, заполненность `p-filled`, `p-inputwrapper-filled`, `p-inputwrapper-focus`).

#### Scenario: Вариант по умолчанию
- **WHEN** `variant` не задан
- **THEN** класс корня `p-floatlabel p-floatlabel-over`

#### Scenario: Варианты
- **WHEN** `variant = In`, затем `On`
- **THEN** классы корня `p-floatlabel p-floatlabel-in` и `p-floatlabel p-floatlabel-on`

#### Scenario: Реактивный вариант
- **WHEN** `variant` передан сигналом и меняется с `Over` на `On`
- **THEN** класс корня заменяется без пересоздания дочерних элементов

#### Scenario: Подъём метки
- **WHEN** вложенное поле получает фокус или значение
- **THEN** метка поднимается над полем (в варианте `Over`) или уменьшается внутри поля (`In`), или садится на верхнюю границу (`On`)

#### Scenario: Недопустимое значение
- **WHEN** вложенное поле имеет класс `p-invalid`
- **THEN** метка получает цвет недопустимого значения из темы

### Requirement: Стоимость отрисовки FloatLabel
Число аллокаций на экземпляр при SSR SHALL укладываться в бюджет `floatlabel-1000` в `benches/alloc-budgets.toml`, вклад в wasm — в бюджет `floatlabel` в `benches/budgets.toml`.

#### Scenario: Бюджеты
- **WHEN** выполняются `cargo xtask alloc-budget` и `cargo xtask wasm-size`
- **THEN** значения FloatLabel не превышают бюджеты
