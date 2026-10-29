## Purpose
Fluid растягивает на всю ширину контейнера вложенные поля и кнопки, не требуя задавать `fluid` у каждого.

## ADDED Requirements

### Requirement: Разметка Fluid
Компонент `Fluid` крейта `prime-fluid` SHALL выводить `<div class="p-fluid">` с дочерним содержимым. `style_class` SHALL добавляться к классам корня после собственных, `style` — задавать встроенный стиль, `pt.root` — добавлять классы, стиль и атрибуты без замены собственных.

#### Scenario: Корень
- **WHEN** внутрь `Fluid` вложены два поля
- **THEN** выводится один `div.p-fluid`, содержащий оба поля

#### Scenario: Классы и стиль
- **WHEN** заданы `style_class = "card"` и `style = "gap: 1rem"`
- **THEN** класс корня `p-fluid card`, атрибут `style="gap: 1rem;"`

### Requirement: Наследование fluid
`Fluid` SHALL предоставлять потомкам контекст, по которому `Button`, `InputText`, `Textarea`, `InputNumber` и `Select` определяют свою ширину: если у компонента свойство `fluid` не задано, он SHALL вести себя как при `fluid = true`, когда у него есть предок `Fluid` (на любой глубине), и как при `fluid = false` без такого предка. Явно заданное `fluid` (в том числе `false`) SHALL иметь приоритет над контекстом. Вне `Fluid` поведение компонентов SHALL оставаться прежним.

#### Scenario: Наследование
- **WHEN** `InputText` без `fluid` лежит внутри `Fluid`
- **THEN** у поля есть класс `p-inputtext-fluid` и `data-p` содержит `fluid`

#### Scenario: Приоритет явного значения
- **WHEN** внутри `Fluid` стоит `InputText` с `fluid = false`
- **THEN** класса `p-inputtext-fluid` у него нет

#### Scenario: Вложенность
- **WHEN** `Select` лежит в `div` внутри `Fluid` внутри другого контейнера
- **THEN** у `Select` есть класс `p-select-fluid`

#### Scenario: Вне Fluid
- **WHEN** `Button`, `Textarea` и `InputNumber` стоят без `Fluid` и без `fluid`
- **THEN** ни у одного из них нет класса растягивания

#### Scenario: Все потребители
- **WHEN** в `Fluid` вложены `Button`, `InputText`, `Textarea`, `InputNumber`, `Select`
- **THEN** у каждого есть свой класс растягивания (`p-button-fluid`, `p-inputtext-fluid`, `p-textarea-fluid`, `p-inputnumber-fluid`, `p-select-fluid`)

#### Scenario: Реактивное fluid
- **WHEN** у потомка `fluid` передан сигналом, внутри `Fluid`, и принимает значение `false`
- **THEN** класс растягивания снимается

### Requirement: Стоимость отрисовки Fluid
Число аллокаций на экземпляр при SSR SHALL укладываться в бюджет `fluid-1000` в `benches/alloc-budgets.toml`, вклад в wasm — в бюджет `fluid` в `benches/budgets.toml`.

#### Scenario: Бюджеты
- **WHEN** выполняются `cargo xtask alloc-budget` и `cargo xtask wasm-size`
- **THEN** значения Fluid не превышают бюджеты
