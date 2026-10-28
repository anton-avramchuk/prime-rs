## ADDED Requirements

### Requirement: Свойства Card
Компонент `Card` крейта `prime-card` SHALL принимать `header` и `subheader` (текст заголовка и подзаголовка), слоты `header_template`, `title_template`, `subtitle_template`, `footer_template` (содержимое Leptos), `style`, `style_class`, `children` (содержимое) и `pt`. Все свойства необязательны; изменяемые во время работы SHALL принимать значение или сигнал.

#### Scenario: Пустая карточка
- **WHEN** `Card` отрисован без свойств
- **THEN** есть корень `p-card p-component`, блок `p-card-body` и пустой `p-card-content`, а блоков `p-card-header`, `p-card-title`, `p-card-subtitle`, `p-card-footer` нет

### Requirement: Структура Card
Корень SHALL быть элементом с классами `p-card p-component`. Внутри SHALL идти: блок `div.p-card-header`, если задан `header_template`; затем всегда `div.p-card-body`, содержащий по порядку `div.p-card-title` (если задан `header` или `title_template`), `div.p-card-subtitle` (если задан `subheader` или `subtitle_template`), `div.p-card-content` (всегда, с `children`) и `div.p-card-footer` (если задан `footer_template`). `style_class` и `style` SHALL добавляться без потери собственных классов.

#### Scenario: Простая карточка
- **WHEN** заданы `header = "Simple Card"` и содержимое
- **THEN** внутри `p-card-body` есть `p-card-title` с текстом `Simple Card` и `p-card-content` с содержимым

#### Scenario: Расширенная карточка
- **WHEN** заданы `header_template`, `title_template`, `subtitle_template`, `footer_template` и содержимое
- **THEN** порядок блоков: `p-card-header`, затем `p-card-body` с `p-card-title`, `p-card-subtitle`, `p-card-content`, `p-card-footer`

### Requirement: Приоритет слотов Card
Если заданы и `header`, и `title_template`, SHALL выводиться только `title_template`; то же для `subheader` и `subtitle_template`.

#### Scenario: Слот вместо текста
- **WHEN** заданы `header = "Text"` и `title_template` с содержимым `Slot`
- **THEN** в `p-card-title` только `Slot`

### Requirement: Pass-through Card
`Card` SHALL принимать `pt` для корня, шапки, тела, заголовка, подзаголовка, содержимого и подвала; классы и атрибуты добавляются к собственным, не заменяя их.

#### Scenario: pt на теле
- **WHEN** передан `pt` с классом `extra` для тела
- **THEN** у `p-card-body` есть и `p-card-body`, и `extra`

### Requirement: Стоимость отрисовки Card
`Card` SHALL использовать статическую ветвь отрисовки, если все свойства статические, и реактивную иначе; обе ветви SHALL давать одинаковую разметку. Число аллокаций на экземпляр при SSR SHALL укладываться в бюджет `card` в `benches/alloc-budgets.toml`, вклад в wasm — в бюджет `card` в `benches/budgets.toml`.

#### Scenario: Паритет ветвей
- **WHEN** один и тот же набор значений передан статически и сигналами
- **THEN** HTML совпадает

#### Scenario: Бюджеты
- **WHEN** выполняются `cargo xtask alloc-budget` и `cargo xtask wasm-size`
- **THEN** значения Card не превышают бюджеты
