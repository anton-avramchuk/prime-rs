# buttongroup Specification

## Purpose
ButtonGroup объединяет кнопки в одну слитную группу с общими скруглениями по краям.

## Requirements

### Requirement: Разметка ButtonGroup
Компонент `ButtonGroup` крейта `prime-buttongroup` SHALL выводить `<span class="p-buttongroup p-component" role="group">` с дочерним содержимым внутри. `style_class` SHALL добавляться к классам корня после собственных, `style` — задавать встроенный стиль, `pt.root` — добавлять классы, стиль и атрибуты без замены собственных. Слитный вид кнопок внутри группы (общая граница, скругления только у крайних) SHALL обеспечиваться CSS темы `buttongroup`.

#### Scenario: Корень
- **WHEN** в группу вложены три кнопки
- **THEN** выводится один `span.p-buttongroup.p-component[role=group]`, внутри которого три `button.p-button` в исходном порядке

#### Scenario: Классы и стиль
- **WHEN** заданы `style_class = "mine"` и `style = "width: 20rem"`
- **THEN** класс корня `p-buttongroup p-component mine`, атрибут `style="width: 20rem;"`

#### Scenario: Слитные кнопки
- **WHEN** страница с подключённым CSS темы показывает три кнопки в группе
- **THEN** у средней кнопки нет скругления, у первой скруглены только левые углы, у последней только правые

#### Scenario: pt
- **WHEN** `pt.root` содержит класс `custom` и атрибут `data-x="1"`
- **THEN** корень имеет `p-buttongroup p-component custom` и `data-x="1"`

### Requirement: Стоимость отрисовки ButtonGroup
Число аллокаций на экземпляр при SSR SHALL укладываться в бюджет `buttongroup-1000` в `benches/alloc-budgets.toml`, вклад в wasm — в бюджет `buttongroup` в `benches/budgets.toml`.

#### Scenario: Бюджеты
- **WHEN** выполняются `cargo xtask alloc-budget` и `cargo xtask wasm-size`
- **THEN** значения ButtonGroup не превышают бюджеты
