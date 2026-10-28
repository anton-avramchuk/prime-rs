## ADDED Requirements

### Requirement: Выбор и события RadioButton
Компонент `RadioButton` крейта `prime-radiobutton` SHALL принимать `value` и двусторонний `model` (`RwSignal<Option<String>>`), считать себя отмеченным при `model == Some(value)`, при выборе пользователем записывать `Some(value)` в `model` и вызывать `on_click` со значением и исходным событием; вызывать `on_focus` и `on_blur`. При `disabled` выбор SHALL игнорироваться.

#### Scenario: Выбор в группе
- **WHEN** две кнопки делят один `model` и пользователь выбирает вторую
- **THEN** `model` равен значению второй, первая перестаёт быть отмеченной, вызван `on_click`

#### Scenario: Disabled
- **WHEN** `disabled` включён и пользователь кликает
- **THEN** `model` и `on_click` не затронуты

### Requirement: Разметка и классы RadioButton
Корень SHALL быть `<div>` с классами `p-radiobutton p-component` и модификаторами `p-radiobutton-checked`, `p-disabled`, `p-invalid`, `p-variant-filled` (вариант `filled` или `PrimeConfig.input_variant = filled`), `p-radiobutton-sm p-inputfield-sm`, `p-radiobutton-lg p-inputfield-lg`; внутри SHALL идти `<input type="radio" class="p-radiobutton-input">` с `aria-checked`, затем `<div class="p-radiobutton-box">` с `<div class="p-radiobutton-icon">`. Корень SHALL иметь `data-p-checked`, `data-p-disabled` и `data-p` (через пробел `invalid`, `checked`, `disabled`, `filled`, размер).

#### Scenario: Отмечена
- **WHEN** `model == Some(value)`
- **THEN** есть класс `p-radiobutton-checked`, `aria-checked="true"`, `data-p-checked="true"`, у `<input>` атрибут `checked`

#### Scenario: Модификаторы
- **WHEN** заданы `disabled`, `invalid`, `size = small`, `variant = filled`
- **THEN** есть соответствующие классы, а `data-p` равен `invalid disabled filled small`

### Requirement: Атрибуты и pass-through RadioButton
`<input>` SHALL получать `id` (`input_id`), `name`, `value`, `tabindex`, `required`, `disabled`, `aria-label`, `aria-labelledby`; незаданные атрибуты SHALL отсутствовать. `pt` SHALL принимать `root`, `input`, `box`, `icon`, добавляя классы и атрибуты без замены собственных.

#### Scenario: Атрибуты
- **WHEN** заданы `input_id`, `name`, `value`
- **THEN** у `<input>` есть `id`, `name`, `value`, а `required` и `tabindex` нет

### Requirement: Стоимость отрисовки RadioButton
RadioButton со статическими свойствами и без `model` SHALL использовать статическую ветвь; обе ветви SHALL давать одинаковую разметку. Число аллокаций на экземпляр при SSR SHALL укладываться в бюджет `radiobutton` в `benches/alloc-budgets.toml`, вклад в wasm — в бюджет `radiobutton` в `benches/budgets.toml`.

#### Scenario: Паритет ветвей
- **WHEN** один набор значений передан статически и сигналами
- **THEN** HTML совпадает

#### Scenario: Бюджеты
- **WHEN** выполняются `cargo xtask alloc-budget` и `cargo xtask wasm-size`
- **THEN** значения RadioButton не превышают бюджеты
