# divider Specification

## Purpose
Компонент Divider (порт p-divider из PrimeNG): направление, тип линии, выравнивание содержимого, role и aria-orientation, pass-through, статическая и реактивная ветви отрисовки.

## Requirements

### Requirement: Свойства Divider
Компонент `Divider` крейта `prime-divider` SHALL принимать `layout` (`horizontal`, `vertical`; по умолчанию `horizontal`), `divider_type` (`solid`, `dashed`, `dotted`; по умолчанию `solid`), `align` (`left`, `center`, `right`, `top`, `bottom`), `style`, `style_class`, `children` и `pt`. Все свойства необязательны; изменяемые во время работы SHALL принимать значение или сигнал.

#### Scenario: Значения по умолчанию
- **WHEN** `Divider` отрисован без свойств
- **THEN** у корня `role="separator"`, `aria-orientation="horizontal"`, классы `p-divider p-component p-divider-horizontal p-divider-solid p-divider-left`, а `data-p` равен `horizontal solid`

### Requirement: Классы корня Divider
Корень SHALL иметь классы `p-divider p-component`, `p-divider-<layout>`, `p-divider-<type>` и класс выравнивания: для горизонтального `p-divider-left` (при `align` не задан или `left`), `p-divider-center` (`center`), `p-divider-right` (`right`); для вертикального `p-divider-top` (`top`), `p-divider-center` (не задан или `center`), `p-divider-bottom` (`bottom`). Для недопустимого сочетания (например, `left` у вертикального) класса выравнивания SHALL NOT быть.

#### Scenario: Горизонтальный по центру
- **WHEN** `layout = horizontal`, `align = center`, `divider_type = dotted`
- **THEN** классы `p-divider-horizontal p-divider-dotted p-divider-center`

#### Scenario: Вертикальный по умолчанию
- **WHEN** `layout = vertical`
- **THEN** классы `p-divider-vertical p-divider-center`

#### Scenario: Недопустимое выравнивание
- **WHEN** `layout = vertical`, `align = left`
- **THEN** класса `p-divider-left` нет и класса выравнивания нет вовсе

### Requirement: Атрибуты и стиль Divider
Корень SHALL иметь `role="separator"` и `aria-orientation`, равный `layout`; `data-p` SHALL содержать через пробел `align` (если задан), `layout` и тип. Inline-стиль SHALL содержать `justify-content` (горизонтальный: `center` для центра и для незаданного, `flex-start` для `left`, `flex-end` для `right`) либо `align-items` (вертикальный: `center` для центра и для незаданного, `flex-start` для `top`, `flex-end` для `bottom`); `style` и стиль из `pt` добавляются к нему.

#### Scenario: Стиль выравнивания
- **WHEN** `layout = horizontal`, `align = right`
- **THEN** в стиле корня есть `justify-content: flex-end`

#### Scenario: Пользовательский стиль
- **WHEN** задан `style = "margin: 4px"`
- **THEN** стиль содержит и `margin: 4px`, и стиль выравнивания

### Requirement: Содержимое Divider
Содержимое (`children`) SHALL выводиться внутри `<div class="p-divider-content">`, который присутствует всегда, даже без содержимого.

#### Scenario: Содержимое
- **WHEN** передано содержимое `Text`
- **THEN** внутри корня есть `<div class="p-divider-content">Text</div>`

#### Scenario: Без содержимого
- **WHEN** содержимого нет
- **THEN** `p-divider-content` присутствует и пуст

### Requirement: Pass-through Divider
`Divider` SHALL принимать `pt` для корня и блока содержимого; классы и атрибуты добавляются к собственным, не заменяя их.

#### Scenario: pt на содержимом
- **WHEN** передан `pt` с классом `extra` для содержимого
- **THEN** у `p-divider-content` есть и `p-divider-content`, и `extra`

### Requirement: Стоимость отрисовки Divider
`Divider` SHALL использовать статическую ветвь отрисовки, если все свойства статические, и реактивную иначе; обе ветви SHALL давать одинаковую разметку. Число аллокаций на экземпляр при SSR SHALL укладываться в бюджет `divider` в `benches/alloc-budgets.toml`, вклад в wasm — в бюджет `divider` в `benches/budgets.toml`.

#### Scenario: Паритет ветвей
- **WHEN** один и тот же набор значений передан статически и сигналами
- **THEN** HTML совпадает

#### Scenario: Бюджеты
- **WHEN** выполняются `cargo xtask alloc-budget` и `cargo xtask wasm-size`
- **THEN** значения Divider не превышают бюджеты
