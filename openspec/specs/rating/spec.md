# rating Specification

## Purpose
Rating позволяет выбрать оценку от 1 до заданного числа звёзд, показывает её и сообщает об изменении.

## Requirements

### Requirement: Значение и события Rating
Компонент `Rating` крейта `prime-rating` SHALL принимать двусторонний `value` (`RwSignal<Option<u32>>`); без него SHALL вести внутреннее состояние, начинающееся с `default_value`. Выбор звезды SHALL устанавливать её номер (от 1), а повторный выбор звезды с текущим значением SHALL сбрасывать значение в `None`; каждый такой выбор SHALL вызывать `on_rate` один раз с новым значением и исходным событием. При `readonly` или `disabled` значение и `on_rate` SHALL не меняться. `on_focus` и `on_blur` SHALL вызываться при получении и потере фокуса звездой.

#### Scenario: Выбор
- **WHEN** значение `None` и пользователь кликает по третьей звезде
- **THEN** значение становится `Some(3)`, `on_rate` вызван один раз со значением `Some(3)`

#### Scenario: Сброс
- **WHEN** значение `Some(3)` и пользователь кликает по третьей звезде
- **THEN** значение становится `None`, `on_rate` вызван со значением `None`

#### Scenario: Только чтение
- **WHEN** задан `readonly` или `disabled` и пользователь кликает по звезде
- **THEN** значение и `on_rate` не затронуты

### Requirement: Клавиатура
Каждая звезда SHALL содержать нативную радиокнопку одной группы (общее `name`, по умолчанию уникальное), поэтому стрелки SHALL переключать значение между звёздами, а выбор с клавиатуры SHALL работать как выбор мышью. Выбранной (`checked`) SHALL быть только радиокнопка звезды, равной значению.

#### Scenario: Стрелки
- **WHEN** фокус на радиокнопке первой звезды и нажата стрелка вправо
- **THEN** значение становится `Some(2)` и `on_rate` вызван один раз

#### Scenario: Группа
- **WHEN** `name` не задан
- **THEN** у всех радиокнопок компонента одинаковое уникальное `name`, отличное от `name` другого экземпляра

### Requirement: Структура и классы Rating
Корень SHALL быть `<div class="p-rating">` с модификаторами `p-readonly` и `p-disabled`, `data-p` (`readonly`, `disabled` через пробел). Число звёзд SHALL равняться `stars` (по умолчанию 5). Каждая звезда SHALL быть `div.p-rating-option` с `p-rating-option-active`, если её номер не больше значения, и `p-focus-visible`, если на ней клавиатурный фокус; внутри SHALL идти `span.p-hidden-accessible` с `input[type=radio]` и иконка: включённая (номер не больше значения) с классом `p-rating-icon p-rating-on-icon`, иначе `p-rating-icon p-rating-off-icon`, и `p-invalid` у обеих при `invalid`. По умолчанию иконки — заполненная и контурная звезда; `on_icon` и `off_icon` их заменяют, а `on_icon_template` и `off_icon_template` SHALL получать номер звезды и класс иконки.

#### Scenario: Значение
- **WHEN** `stars = 5`, значение `Some(3)`
- **THEN** первые три звезды активны и показывают включённую иконку, остальные две — выключенную

#### Scenario: Число звёзд
- **WHEN** задано `stars = 10`
- **THEN** выводятся десять звёзд

#### Scenario: Недопустимое значение
- **WHEN** задан `invalid`
- **THEN** у каждой иконки есть класс `p-invalid`

### Requirement: Атрибуты Rating
Радиокнопка звезды с номером `n` SHALL иметь `value="n"`, `name`, `required` (только при `required`), `readonly` и `disabled` (только при соответствующих свойствах) и `aria-label` из локали: «1 star» для первой звезды и «{n} stars» для остальных (ключи `aria.star` и `aria.stars`).

#### Scenario: Подписи
- **WHEN** локаль по умолчанию
- **THEN** подписи радиокнопок «1 star», «2 stars», «3 stars» и так далее

#### Scenario: Недоступна
- **WHEN** задан `disabled`
- **THEN** у корня `p-disabled`, у каждой радиокнопки `disabled`

### Requirement: Свойства и pass-through Rating
Свойства `stars`, `readonly`, `disabled`, `invalid`, `required`, `name`, `style`, `style_class` SHALL принимать значение или сигнал и обновлять разметку при смене сигнала. `pt` SHALL принимать части `root`, `option`, `hidden_option_input_container`, `hidden_option_input`, `on_icon`, `off_icon`, добавляя классы, стиль и атрибуты без замены собственных.

#### Scenario: Реактивное число звёзд
- **WHEN** `stars` передан сигналом и меняется с 5 на 7
- **THEN** выводится семь звёзд

#### Scenario: Класс через pt
- **WHEN** в `pt.option` передан класс `custom`
- **THEN** у каждой звезды есть `custom` и `p-rating-option`

### Requirement: Стоимость отрисовки Rating
Число аллокаций на экземпляр при SSR SHALL укладываться в бюджет `rating-1000` в `benches/alloc-budgets.toml`, вклад в wasm — в бюджет `rating` в `benches/budgets.toml`.

#### Scenario: Бюджеты
- **WHEN** выполняются `cargo xtask alloc-budget` и `cargo xtask wasm-size`
- **THEN** значения Rating не превышают бюджеты
