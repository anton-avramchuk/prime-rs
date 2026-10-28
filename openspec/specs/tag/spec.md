# tag Specification

## Purpose
Компонент Tag (порт p-tag из PrimeNG): значение, серьёзность, скругление, иконка, содержимое, pass-through, статическая и реактивная ветви отрисовки.

## Requirements

### Requirement: Свойства Tag
Компонент `Tag` крейта `prime-tag` SHALL принимать `value` (текст метки), `severity` (`secondary`, `info`, `success`, `warn`, `danger`, `contrast`), `rounded`, `icon` (содержимое Leptos), `style`, `style_class`, `children` и `pt`. Все свойства необязательны; изменяемые во время работы SHALL принимать значение или сигнал.

#### Scenario: Значение
- **WHEN** `Tag` отрисован со значением `New`
- **THEN** внутри есть `<span class="p-tag-label">New</span>`

#### Scenario: Пустой Tag
- **WHEN** `Tag` отрисован без свойств
- **THEN** есть корневой элемент с классами `p-tag p-component` и пустая метка `p-tag-label`, как в PrimeNG

### Requirement: Классы и атрибуты Tag
Корневой элемент SHALL иметь классы `p-tag p-component` и модификаторы: `p-tag-<severity>` для серьёзности (`info`, `success`, `warn`, `danger`, `secondary`, `contrast`) и `p-tag-rounded` при `rounded`. Атрибут `data-p` SHALL содержать через пробел `rounded` и серьёзность, как в PrimeNG. `style_class` и `style` SHALL добавляться без потери собственных классов.

#### Scenario: Серьёзность и скругление
- **WHEN** заданы `severity = success` и `rounded = true`
- **THEN** у корня классы `p-tag-success` и `p-tag-rounded`, а `data-p` равен `rounded success`

#### Scenario: Пользовательский класс и стиль
- **WHEN** заданы `style_class = "custom"` и `style = "margin: 4px"`
- **THEN** у корня есть `custom`, собственные классы остались, и стиль равен `margin: 4px`

### Requirement: Содержимое и иконка Tag
`children`, если заданы, SHALL выводиться первыми, как проецируемое содержимое в PrimeNG; затем иконка в `<span class="p-tag-icon">`, затем метка `p-tag-label`. Иконка SHALL выводиться только если задана.

#### Scenario: Порядок содержимого
- **WHEN** заданы `children`, `icon` и `value`
- **THEN** порядок дочерних узлов: `children`, `span.p-tag-icon`, `span.p-tag-label`

#### Scenario: Без иконки
- **WHEN** `icon` не задана
- **THEN** элемента `p-tag-icon` нет

### Requirement: Pass-through Tag
`Tag` SHALL принимать `pt` для корня, иконки и метки; классы и атрибуты из `pt` добавляются к собственным, не заменяя их.

#### Scenario: pt на метке
- **WHEN** передан `pt` с классом `extra` для метки
- **THEN** у `p-tag-label` есть и `p-tag-label`, и `extra`

### Requirement: Стоимость отрисовки Tag
`Tag` SHALL использовать статическую ветвь отрисовки, если все свойства статические, и реактивную иначе; обе ветви SHALL давать одинаковую разметку. Число аллокаций на экземпляр при SSR SHALL укладываться в бюджет `tag` в `benches/alloc-budgets.toml`, вклад в wasm — в бюджет `tag` в `benches/budgets.toml`.

#### Scenario: Паритет ветвей
- **WHEN** один и тот же набор значений передан статически и сигналами
- **THEN** HTML совпадает

#### Scenario: Бюджеты
- **WHEN** выполняются `cargo xtask alloc-budget` и `cargo xtask wasm-size`
- **THEN** значения Tag не превышают бюджеты
